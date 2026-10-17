//! Communication with the FastNoise2 Node Editor through shared memory.
//!
//! This implements the protocol of FastNoise2's `NodeEditorIpc` library: a shared memory region
//! named `/FastNoise2NodeEditor` of 64 KiB, holding a message counter (byte 0), a message type
//! (byte 1) and a null terminated encoded node tree. Unlike `NodeEditorIpc`:
//! - closing it doesn't remove the region, which would disconnect the processes opening it
//!   afterwards from a running Node Editor, and it reconnects when the Node Editor restarts;
//! - polling doesn't drop a message for a too small buffer, and reads a message again if it
//!   changes while being read.
//!
//! The region holds a single message: only the last message sent before a poll is received.
use std::{
    ffi::{CStr, CString, OsStr},
    io,
    process::Command,
    ptr::{self, NonNull},
    sync::atomic::{self, AtomicU8, Ordering},
};

const SHARED_MEMORY_NAME: &CStr = c"/FastNoise2NodeEditor";
const SHARED_MEMORY_SIZE: usize = 64 * 1024;

/// Bytes before the encoded node tree: the message counter and type.
const HEADER_SIZE: usize = 2;

/// Times a message is read again when it changes while being read.
const READ_ATTEMPTS: usize = 4;

const MESSAGE_SELECTED_NODE: u8 = 1;
const MESSAGE_IMPORT_REQUEST: u8 = 2;

/// A message from the shared memory, sent by the Node Editor or another process.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorMessage {
    /// The encoded node tree selected in the Node Editor, sent when it changes.
    SelectedNode(String),
    /// An encoded node tree to import in the Node Editor, sent by
    /// [`NodeEditorIpc::send_import_request`].
    ImportRequest(String),
}

/// Connection to the FastNoise2 Node Editor, to receive the node tree selected in it and to send
/// it node trees to import.
///
/// ```rust,no_run
/// use fastnoise2::{EditorMessage, Node, NodeEditorIpc};
///
/// let mut ipc = NodeEditorIpc::open()?;
/// loop {
///     // Every frame: rebuild the node when the selection changes in the Node Editor
///     if let Some(EditorMessage::SelectedNode(encoded)) = ipc.poll() {
///         let node = Node::from_encoded_node_tree(&encoded);
///     }
/// #   break;
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
pub struct NodeEditorIpc {
    name: CString,
    shared_memory: SharedMemory,
    /// Counter of the last message read or sent.
    last_counter: u8,
}

// SAFETY: the shared memory is only accessed through atomics and copies, from any thread.
unsafe impl Send for NodeEditorIpc {}

impl NodeEditorIpc {
    /// Opens the shared memory, creating it if the Node Editor isn't running. Only messages sent
    /// afterwards are polled, see [`NodeEditorIpc::selected_node`] for the current selection.
    ///
    /// # Errors
    /// Returns an error if the shared memory can't be created or mapped.
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(level = "trace", err(level = "debug"))
    )]
    pub fn open() -> io::Result<Self> {
        Self::open_named(SHARED_MEMORY_NAME)
    }

    fn open_named(name: &CStr) -> io::Result<Self> {
        let shared_memory = SharedMemory::open(name)?;
        let last_counter = shared_memory.counter().load(Ordering::Acquire);

        Ok(Self {
            name: name.to_owned(),
            shared_memory,
            last_counter,
        })
    }

    /// Reopens the shared memory if its name refers to another region or to none: the Node
    /// Editor removes the name when it exits, and the next one creates a new region.
    fn reconnect(&mut self) {
        if self.shared_memory.is_current(&self.name) {
            return;
        }

        match SharedMemory::open(&self.name) {
            Ok(shared_memory) => {
                #[cfg(feature = "trace")]
                tracing::debug!("reopened the Node Editor shared memory");

                // The current message is polled, e.g. the selection of a restarted Node Editor
                let counter = shared_memory.counter().load(Ordering::Acquire);
                self.last_counter = counter.wrapping_sub(1);
                self.shared_memory = shared_memory;
            }
            Err(_error) => {
                #[cfg(feature = "trace")]
                tracing::debug!(error = %_error, "failed to reopen the Node Editor shared memory");
            }
        }
    }

    /// Returns the next message, `None` if there is no new message. Only the last message is
    /// kept, a message sent before the previous one is read is lost, and the counter wraps after
    /// 256 messages, so exactly 256 messages sent between two polls are all missed.
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, ret))]
    pub fn poll(&mut self) -> Option<EditorMessage> {
        // A message written before the name was removed is still read from the old region
        if let Some(message) = self.next_message() {
            return Some(message);
        }

        self.reconnect();
        self.next_message()
    }

    fn next_message(&mut self) -> Option<EditorMessage> {
        if self.shared_memory.counter().load(Ordering::Acquire) == self.last_counter {
            return None;
        }

        let (counter, message) = self.read_stable_message();
        self.last_counter = counter;
        message
    }

    /// Reads the current message and the counter it was written with. The protocol has no write
    /// lock, so a message is read again if the counter changes while it is read. A write still
    /// in progress when the counter is checked isn't detected, as the counter is only increased
    /// after the message is written.
    fn read_stable_message(&self) -> (u8, Option<EditorMessage>) {
        let counter = self.shared_memory.counter();

        let mut before = counter.load(Ordering::Acquire);
        for _ in 1..READ_ATTEMPTS {
            let message = self.read_message();

            // The message is read before checking the counter again
            atomic::fence(Ordering::Acquire);
            let after = counter.load(Ordering::Relaxed);
            if after == before {
                return (before, message);
            }
            before = after;
        }

        (before, self.read_message())
    }

    /// The encoded node tree currently selected in the Node Editor, if the last message is a
    /// selected node.
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, ret))]
    pub fn selected_node(&mut self) -> Option<String> {
        self.reconnect();

        match self.read_stable_message().1? {
            EditorMessage::SelectedNode(encoded) => Some(encoded),
            EditorMessage::ImportRequest(_) => None,
        }
    }

    /// Sends an encoded node tree to import in the Node Editor.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree contains a null character or doesn't fit in the
    /// shared memory.
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(len = encoded_node_tree.len()), err(level = "debug")))]
    pub fn send_import_request(&mut self, encoded_node_tree: &str) -> io::Result<()> {
        self.send(MESSAGE_IMPORT_REQUEST, encoded_node_tree)
    }

    /// Sends an encoded node tree as the selected node, like the Node Editor does.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree contains a null character or doesn't fit in the
    /// shared memory.
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(len = encoded_node_tree.len()), err(level = "debug")))]
    pub fn send_selected_node(&mut self, encoded_node_tree: &str) -> io::Result<()> {
        self.send(MESSAGE_SELECTED_NODE, encoded_node_tree)
    }

    fn read_message(&self) -> Option<EditorMessage> {
        let message_type = self.shared_memory.message_type().load(Ordering::Relaxed);
        if message_type != MESSAGE_SELECTED_NODE && message_type != MESSAGE_IMPORT_REQUEST {
            return None;
        }

        let encoded = String::from_utf8(self.copy_payload()?).ok()?;

        Some(match message_type {
            MESSAGE_SELECTED_NODE => EditorMessage::SelectedNode(encoded),
            _ => EditorMessage::ImportRequest(encoded),
        })
    }

    /// Copies the payload up to its null terminator, `None` if it has none. It is copied by
    /// chunks before looking for the end, the other process may write it meanwhile.
    fn copy_payload(&self) -> Option<Vec<u8>> {
        const CHUNK_SIZE: usize = 1024;
        const PAYLOAD_SIZE: usize = SHARED_MEMORY_SIZE - HEADER_SIZE;

        let payload = self.shared_memory.payload();
        let mut bytes = Vec::<u8>::new();
        while bytes.len() < PAYLOAD_SIZE {
            let start = bytes.len();
            let len = CHUNK_SIZE.min(PAYLOAD_SIZE - start);

            bytes.reserve(len);
            unsafe {
                ptr::copy_nonoverlapping(payload.add(start), bytes.as_mut_ptr().add(start), len);
                bytes.set_len(start + len);
            }

            if let Some(end) = bytes[start..].iter().position(|&byte| byte == 0) {
                bytes.truncate(start + end);
                return Some(bytes);
            }
        }

        None
    }

    fn send(&mut self, message_type: u8, encoded_node_tree: &str) -> io::Result<()> {
        self.reconnect();

        let bytes = encoded_node_tree.as_bytes();
        if bytes.contains(&0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "encoded node tree contains a null character",
            ));
        }
        // Same limit as `NodeEditorIpc`, for the header and the null terminator
        if bytes.len() + HEADER_SIZE + 1 >= SHARED_MEMORY_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "encoded node tree doesn't fit in the Node Editor shared memory",
            ));
        }

        unsafe {
            let payload = self.shared_memory.payload();
            ptr::copy_nonoverlapping(bytes.as_ptr(), payload, bytes.len());
            payload.add(bytes.len()).write(0);
        }
        self.shared_memory
            .message_type()
            .store(message_type, Ordering::Relaxed);

        // Published by the counter, so the other process reads the payload once it changes
        let counter = self.shared_memory.counter();
        let new_counter = counter.load(Ordering::Relaxed).wrapping_add(1);
        counter.store(new_counter, Ordering::Release);

        // Not polled back
        self.last_counter = new_counter;

        Ok(())
    }
}

/// Command starting the Node Editor executable at `path`, as FastNoise2's `NodeEditorIpc` does.
/// `encoded_node_tree` is imported at startup, and the `detached` Node Editor only shows the node
/// graph (no preview).
///
/// The Node Editor keeps running when the returned [`Child`](std::process::Child) is dropped,
/// [`kill`](std::process::Child::kill) it to close it.
pub fn node_editor_command(
    path: impl AsRef<OsStr>,
    encoded_node_tree: Option<&str>,
    detached: bool,
) -> Command {
    let mut command = Command::new(path);
    if detached {
        command.arg("--detached");
    }
    if let Some(encoded_node_tree) = encoded_node_tree.filter(|encoded| !encoded.is_empty()) {
        command.args(["--import-ent", encoded_node_tree]);
    }
    command
}

/// The mapped shared memory region, unmapped on drop.
struct SharedMemory {
    ptr: NonNull<u8>,
    /// Identifies the region, its name may later refer to another one.
    #[cfg(unix)]
    inode: u64,
    #[cfg(windows)]
    handle: windows_sys::Win32::Foundation::HANDLE,
}

impl SharedMemory {
    #[cfg(unix)]
    fn open(name: &CStr) -> io::Result<Self> {
        use rustix::{
            fs::{fstat, ftruncate},
            io::Errno,
            mm::{MapFlags, ProtFlags, mmap},
            shm::{self, Mode, OFlags},
        };

        let fd = shm::open(
            name,
            OFlags::CREATE | OFlags::RDWR,
            Mode::from_raw_mode(0o666),
        )?;

        // Fails with EINVAL if the region already has this size on some systems (e.g. macOS)
        match ftruncate(&fd, SHARED_MEMORY_SIZE as u64) {
            Ok(()) | Err(Errno::INVAL) => {}
            Err(error) => return Err(error.into()),
        }
        let inode = fstat(&fd)?.st_ino;

        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                SHARED_MEMORY_SIZE,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                &fd,
                0,
            )?
        };

        Ok(Self {
            ptr: NonNull::new(ptr.cast()).expect("mmap doesn't return null on success"),
            inode,
        })
    }

    /// Whether `name` still refers to this region.
    #[cfg(unix)]
    fn is_current(&self, name: &CStr) -> bool {
        use rustix::{
            fs::fstat,
            io::Errno,
            shm::{self, Mode, OFlags},
        };

        match shm::open(name, OFlags::RDWR, Mode::empty()) {
            Ok(fd) => match fstat(&fd) {
                Ok(stat) if self.inode != 0 => stat.st_ino == self.inode,
                // macOS gives no inode number to shared memory (`st_ino` is 0)
                Ok(_) => self.is_mapped_by(&fd),
                Err(_) => true,
            },
            Err(Errno::NOENT) => false,
            // Keeps using the region if the name can't be checked
            Err(_) => true,
        }
    }

    /// Whether `fd` refers to this region: its last byte, never written by the protocol (a
    /// message ends at byte 65534 at most), is changed twice and read through a new mapping of
    /// `fd`, so a value matching by chance is ruled out.
    #[cfg(unix)]
    fn is_mapped_by(&self, fd: &rustix::fd::OwnedFd) -> bool {
        use rustix::mm::{MapFlags, ProtFlags, mmap, munmap};

        let Ok(other) = (unsafe {
            mmap(
                ptr::null_mut(),
                SHARED_MEMORY_SIZE,
                ProtFlags::READ,
                MapFlags::SHARED,
                fd,
                0,
            )
        }) else {
            return true;
        };

        let probe = self.probe();
        let other_probe =
            unsafe { AtomicU8::from_ptr(other.cast::<u8>().add(SHARED_MEMORY_SIZE - 1)) };
        let is_same = (0..2).all(|_| {
            let value = probe.load(Ordering::Relaxed).wrapping_add(1);
            probe.store(value, Ordering::SeqCst);
            other_probe.load(Ordering::SeqCst) == value
        });

        // Can't fail with a region mapped above
        let _ = unsafe { munmap(other, SHARED_MEMORY_SIZE) };
        is_same
    }

    /// A named file mapping exists as long as a process has it open, Node Editors share it.
    #[cfg(windows)]
    fn is_current(&self, _name: &CStr) -> bool {
        true
    }

    #[cfg(windows)]
    fn open(name: &CStr) -> io::Result<Self> {
        use windows_sys::Win32::{
            Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
            System::Memory::{
                CreateFileMappingA, FILE_MAP_ALL_ACCESS, MapViewOfFile, PAGE_READWRITE,
            },
        };

        let handle = unsafe {
            CreateFileMappingA(
                INVALID_HANDLE_VALUE,
                ptr::null(),
                PAGE_READWRITE,
                0,
                SHARED_MEMORY_SIZE as u32,
                name.as_ptr().cast(),
            )
        };
        if handle.is_null() {
            return Err(io::Error::last_os_error());
        }

        let view = unsafe { MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, SHARED_MEMORY_SIZE) };
        let Some(ptr) = NonNull::new(view.Value.cast()) else {
            let error = io::Error::last_os_error();
            unsafe { CloseHandle(handle) };
            return Err(error);
        };

        Ok(Self { ptr, handle })
    }

    #[inline]
    fn counter(&self) -> &AtomicU8 {
        unsafe { AtomicU8::from_ptr(self.ptr.as_ptr()) }
    }

    #[inline]
    fn message_type(&self) -> &AtomicU8 {
        unsafe { AtomicU8::from_ptr(self.ptr.as_ptr().add(1)) }
    }

    /// The last byte, never written by the protocol, see [`SharedMemory::is_mapped_by`].
    #[cfg(unix)]
    #[inline]
    fn probe(&self) -> &AtomicU8 {
        unsafe { AtomicU8::from_ptr(self.ptr.as_ptr().add(SHARED_MEMORY_SIZE - 1)) }
    }

    #[inline]
    fn payload(&self) -> *mut u8 {
        unsafe { self.ptr.as_ptr().add(HEADER_SIZE) }
    }
}

impl Drop for SharedMemory {
    /// Unmaps the region without removing it, the Node Editor and other processes keep using it.
    #[cfg(unix)]
    fn drop(&mut self) {
        // Can't fail with a region mapped by `open`
        let _ = unsafe { rustix::mm::munmap(self.ptr.as_ptr().cast(), SHARED_MEMORY_SIZE) };
    }

    #[cfg(windows)]
    fn drop(&mut self) {
        use windows_sys::Win32::{
            Foundation::CloseHandle,
            System::Memory::{MEMORY_MAPPED_VIEW_ADDRESS, UnmapViewOfFile},
        };

        unsafe {
            UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS {
                Value: self.ptr.as_ptr().cast(),
            });
            CloseHandle(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    /// Shared memory with a name of its own, removed on drop, so tests don't talk to a running
    /// Node Editor or to each other.
    struct TestRegion(CString);

    impl TestRegion {
        fn new(test: &str) -> Self {
            Self(CString::new(format!("/fn2rs-{}-{test}", std::process::id())).unwrap())
        }

        fn open(&self) -> NodeEditorIpc {
            NodeEditorIpc::open_named(&self.0).unwrap()
        }
    }

    impl Drop for TestRegion {
        fn drop(&mut self) {
            #[cfg(unix)]
            let _ = rustix::shm::unlink(self.0.as_c_str());
        }
    }

    #[test]
    fn test_messages_between_connections() {
        let region = TestRegion::new("messages");
        let mut editor = region.open();
        let mut game = region.open();

        assert_eq!(game.poll(), None);

        editor.send_selected_node("DQkGDA==").unwrap();
        assert_eq!(editor.poll(), None, "a sent message isn't polled back");
        assert_eq!(
            game.poll(),
            Some(EditorMessage::SelectedNode("DQkGDA==".to_string()))
        );
        assert_eq!(game.poll(), None);

        game.send_import_request("GQ==").unwrap();
        assert_eq!(
            editor.poll(),
            Some(EditorMessage::ImportRequest("GQ==".to_string()))
        );
    }

    #[test]
    fn test_selected_node() {
        let region = TestRegion::new("selected");
        let mut editor = region.open();
        editor.send_selected_node("DQkGDA==").unwrap();

        // Opened after the message: not polled, but readable as the current selection
        let mut game = region.open();
        assert_eq!(game.poll(), None);
        assert_eq!(game.selected_node().as_deref(), Some("DQkGDA=="));

        editor.send_import_request("GQ==").unwrap();
        assert_eq!(game.selected_node(), None);
    }

    /// The Node Editor removes the name when it exits, the next one creates a new region.
    #[cfg(unix)]
    #[test]
    fn test_reconnects_to_a_new_region() {
        let region = TestRegion::new("reconnect");
        let mut game = region.open();

        // A message sent just before the Node Editor exits is still read
        let mut editor = region.open();
        editor.send_selected_node("GQ==").unwrap();
        drop(editor);
        rustix::shm::unlink(region.0.as_c_str()).unwrap();
        assert_eq!(
            game.poll(),
            Some(EditorMessage::SelectedNode("GQ==".to_string()))
        );
        let mut editor = region.open();
        editor.send_selected_node("DQkGDA==").unwrap();

        assert_eq!(
            game.poll(),
            Some(EditorMessage::SelectedNode("DQkGDA==".to_string()))
        );
        assert_eq!(game.poll(), None);

        // Sending after the name is removed creates a region the next Node Editor opens
        rustix::shm::unlink(region.0.as_c_str()).unwrap();
        game.send_import_request("GQ==").unwrap();
        let mut next_editor = region.open();
        game.send_import_request("GQ==").unwrap();
        assert_eq!(
            next_editor.poll(),
            Some(EditorMessage::ImportRequest("GQ==".to_string()))
        );
    }

    /// The region identity check used without inode numbers (macOS), run on every Unix.
    #[cfg(unix)]
    #[test]
    fn test_is_mapped_by() {
        use rustix::shm::{self, Mode, OFlags};

        let region = TestRegion::new("probe");
        let game = region.open();
        let open = || shm::open(region.0.as_c_str(), OFlags::RDWR, Mode::empty()).unwrap();

        assert!(game.shared_memory.is_mapped_by(&open()));
        assert!(game.shared_memory.is_mapped_by(&open()));

        shm::unlink(region.0.as_c_str()).unwrap();
        let _editor = region.open();
        assert!(!game.shared_memory.is_mapped_by(&open()));
    }

    #[test]
    fn test_longer_then_shorter_message() {
        let region = TestRegion::new("lengths");
        let mut editor = region.open();
        let mut game = region.open();

        // Longer than a chunk of `copy_payload`, and the largest message
        for len in [5000, SHARED_MEMORY_SIZE - HEADER_SIZE - 2] {
            let long = "A".repeat(len);
            editor.send_selected_node(&long).unwrap();
            assert_eq!(game.poll(), Some(EditorMessage::SelectedNode(long)));
        }

        editor.send_selected_node("DQkGDA==").unwrap();
        assert_eq!(
            game.poll(),
            Some(EditorMessage::SelectedNode("DQkGDA==".to_string()))
        );
    }

    #[test]
    fn test_invalid_messages() {
        let region = TestRegion::new("invalid");
        let mut ipc = region.open();

        assert!(ipc.send_selected_node("DQ\0kGDA==").is_err());

        let largest = SHARED_MEMORY_SIZE - HEADER_SIZE - 2;
        assert!(ipc.send_selected_node(&"A".repeat(largest)).is_ok());
        assert!(ipc.send_selected_node(&"A".repeat(largest + 1)).is_err());
    }

    #[test]
    fn test_node_editor_command() {
        let command = node_editor_command("NodeEditor", Some("DQkGDA=="), true);
        assert_eq!(command.get_program(), "NodeEditor");
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            ["--detached", "--import-ent", "DQkGDA=="]
        );

        let command = node_editor_command("NodeEditor", Some(""), false);
        assert_eq!(command.get_args().count(), 0);
    }
}
