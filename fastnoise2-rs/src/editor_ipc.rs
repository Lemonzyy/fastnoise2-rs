//! Communication with the FastNoise2 Node Editor through shared memory.
//!
//! This implements the protocol of FastNoise2's `NodeEditorIpc` library: a shared memory region
//! named `/FastNoise2NodeEditor` of 64 KiB, holding a message counter (byte 0), a message type
//! (byte 1) and a null terminated encoded node tree. Unlike `NodeEditorIpc`, closing it doesn't
//! remove the region, which would disconnect the processes opening it afterwards from a running
//! Node Editor, and polling can't lose a message.
use std::{
    ffi::{CStr, OsStr},
    io,
    process::Command,
    ptr::{self, NonNull},
    sync::atomic::{AtomicU8, Ordering},
};

const SHARED_MEMORY_NAME: &CStr = c"/FastNoise2NodeEditor";
const SHARED_MEMORY_SIZE: usize = 64 * 1024;

/// Bytes before the encoded node tree: the message counter and type.
const HEADER_SIZE: usize = 2;

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
    pub fn open() -> io::Result<Self> {
        Self::open_named(SHARED_MEMORY_NAME)
    }

    fn open_named(name: &CStr) -> io::Result<Self> {
        let shared_memory = SharedMemory::open(name)?;
        let last_counter = shared_memory.counter().load(Ordering::Acquire);

        Ok(Self {
            shared_memory,
            last_counter,
        })
    }

    /// Returns the next message, `None` if there is no new message. Only the last message is
    /// kept, a message sent before the previous one is read is lost.
    pub fn poll(&mut self) -> Option<EditorMessage> {
        let counter = self.shared_memory.counter().load(Ordering::Acquire);
        if counter == self.last_counter {
            return None;
        }

        self.last_counter = counter;
        self.read_message()
    }

    /// The encoded node tree currently selected in the Node Editor, if the last message is a
    /// selected node.
    pub fn selected_node(&self) -> Option<String> {
        match self.read_message()? {
            EditorMessage::SelectedNode(encoded) => Some(encoded),
            EditorMessage::ImportRequest(_) => None,
        }
    }

    /// Sends an encoded node tree to import in the Node Editor.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree contains a null character or doesn't fit in the
    /// shared memory.
    pub fn send_import_request(&mut self, encoded_node_tree: &str) -> io::Result<()> {
        self.send(MESSAGE_IMPORT_REQUEST, encoded_node_tree)
    }

    /// Sends an encoded node tree as the selected node, like the Node Editor does.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree contains a null character or doesn't fit in the
    /// shared memory.
    pub fn send_selected_node(&mut self, encoded_node_tree: &str) -> io::Result<()> {
        self.send(MESSAGE_SELECTED_NODE, encoded_node_tree)
    }

    fn read_message(&self) -> Option<EditorMessage> {
        let message_type = self.shared_memory.message_type().load(Ordering::Relaxed);
        if message_type != MESSAGE_SELECTED_NODE && message_type != MESSAGE_IMPORT_REQUEST {
            return None;
        }

        // Copied before looking for the end, the other process may write it meanwhile
        let mut payload = vec![0; SHARED_MEMORY_SIZE - HEADER_SIZE];
        unsafe {
            ptr::copy_nonoverlapping(
                self.shared_memory.payload(),
                payload.as_mut_ptr(),
                payload.len(),
            );
        }

        let end = payload.iter().position(|&byte| byte == 0)?;
        payload.truncate(end);
        let encoded = String::from_utf8(payload).ok()?;

        Some(match message_type {
            MESSAGE_SELECTED_NODE => EditorMessage::SelectedNode(encoded),
            _ => EditorMessage::ImportRequest(encoded),
        })
    }

    fn send(&mut self, message_type: u8, encoded_node_tree: &str) -> io::Result<()> {
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
    #[cfg(windows)]
    handle: windows_sys::Win32::Foundation::HANDLE,
}

impl SharedMemory {
    #[cfg(unix)]
    fn open(name: &CStr) -> io::Result<Self> {
        use rustix::{
            fs::ftruncate,
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
        })
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

    #[test]
    fn test_longer_then_shorter_message() {
        let region = TestRegion::new("lengths");
        let mut editor = region.open();
        let mut game = region.open();

        editor.send_selected_node(&"A".repeat(1000)).unwrap();
        game.poll();
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
