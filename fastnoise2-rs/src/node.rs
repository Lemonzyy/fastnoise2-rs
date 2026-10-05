//! Built nodes and generators.
//!
//! A [`Node`] is a built FastNoise2 node: immutable, cheap to clone (cloning shares the same
//! C++ node) and safe to generate noise with. A [`Generator`] is anything that can be built
//! into a [`Node`].
use std::{
    ffi::{c_void, CString},
    fmt,
    ptr::NonNull,
    sync::Arc,
};

use fastnoise2_sys::*;

use crate::{
    metadata::{Metadata, NODE_METADATA},
    safe::{check_position_arrays, grid_len},
    FastNoiseError, OutputMinMax,
};

/// Owner of a FastNoise2 node reference, released on drop.
struct NodeHandle {
    ptr: NonNull<c_void>,
    metadata_id: i32,
}

// SAFETY: FastNoise2 node reference counts are atomic, noise generation only reads the node,
// and `GeneratorCache` keeps its cache in thread local storage.
unsafe impl Send for NodeHandle {}
unsafe impl Sync for NodeHandle {}

impl NodeHandle {
    /// # Safety
    /// `ptr` must be a node reference returned by FastNoise2 and owned by the caller.
    unsafe fn new(ptr: *mut c_void) -> Option<Self> {
        let ptr = NonNull::new(ptr)?;
        let metadata_id = unsafe { fnGetMetadataID(ptr.as_ptr()) };
        Some(Self { ptr, metadata_id })
    }

    fn metadata(&self) -> &'static Metadata {
        &NODE_METADATA[self.metadata_id as usize]
    }
}

impl Drop for NodeHandle {
    fn drop(&mut self) {
        unsafe { fnDeleteNodeRef(self.ptr.as_ptr()) };
    }
}

/// A built FastNoise2 node.
///
/// Cloning a node shares the same C++ node, so a node used in several places of a tree is
/// evaluated by the same C++ node (which is what makes `GeneratorCache` effective).
#[derive(Clone)]
pub struct Node(Arc<NodeHandle>);

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Node")
            .field(&self.0.metadata().name)
            .finish()
    }
}

impl Node {
    /// Creates a node from an encoded node tree, as exported by the FastNoise2 Node Editor.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree is invalid.
    pub fn from_encoded_node_tree(encoded_node_tree: &str) -> Result<Self, FastNoiseError> {
        let encoded_node_tree = CString::new(encoded_node_tree)?;
        // u32::MAX (~0u in C++) auto-detects the feature set
        let ptr = unsafe { fnNewFromEncodedNodeTree(encoded_node_tree.as_ptr(), u32::MAX) };
        unsafe { NodeHandle::new(ptr) }
            .map(|handle| Self(Arc::new(handle)))
            .ok_or(FastNoiseError::NodeCreationFailed)
    }

    /// The FastNoise2 node name (e.g. "Perlin").
    pub fn name(&self) -> &'static str {
        &self.0.metadata().name
    }

    /// The `FastSIMD::FeatureSet` used by this node.
    pub fn get_active_feature_set(&self) -> u32 {
        unsafe { fnGetActiveFeatureSet(self.as_ptr()) }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.0.ptr.as_ptr()
    }

    /// # Panics
    /// Panics if a count is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_count * y_count`.
    pub fn gen_uniform_grid_2d(
        &self,
        noise_out: &mut [f32],
        x_offset: f32,
        y_offset: f32,
        x_count: i32,
        y_count: i32,
        x_step_size: f32,
        y_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= grid_len(&[x_count, y_count]));
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenUniformGrid2D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                x_offset,
                y_offset,
                x_count,
                y_count,
                x_step_size,
                y_step_size,
                seed,
                min_max.as_mut_ptr(),
            )
        };
        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if a count is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_count * y_count * z_count`.
    pub fn gen_uniform_grid_3d(
        &self,
        noise_out: &mut [f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        x_count: i32,
        y_count: i32,
        z_count: i32,
        x_step_size: f32,
        y_step_size: f32,
        z_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= grid_len(&[x_count, y_count, z_count]));
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenUniformGrid3D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                x_offset,
                y_offset,
                z_offset,
                x_count,
                y_count,
                z_count,
                x_step_size,
                y_step_size,
                z_step_size,
                seed,
                min_max.as_mut_ptr(),
            )
        };
        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if a count is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_count * y_count * z_count * w_count`.
    pub fn gen_uniform_grid_4d(
        &self,
        noise_out: &mut [f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        w_offset: f32,
        x_count: i32,
        y_count: i32,
        z_count: i32,
        w_count: i32,
        x_step_size: f32,
        y_step_size: f32,
        z_step_size: f32,
        w_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= grid_len(&[x_count, y_count, z_count, w_count]));
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenUniformGrid4D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                x_offset,
                y_offset,
                z_offset,
                w_offset,
                x_count,
                y_count,
                z_count,
                w_count,
                x_step_size,
                y_step_size,
                z_step_size,
                w_step_size,
                seed,
                min_max.as_mut_ptr(),
            )
        };
        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, and `y_pos_array` are empty or do not have the same length.
    pub fn gen_position_array_2d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        check_position_arrays(noise_out, &[x_pos_array, y_pos_array]);
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenPositionArray2D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                noise_out.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                x_offset,
                y_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        };
        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, `y_pos_array`, and `z_pos_array` are empty or do not have the same length.
    pub fn gen_position_array_3d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        z_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        check_position_arrays(noise_out, &[x_pos_array, y_pos_array, z_pos_array]);
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenPositionArray3D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                noise_out.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                z_pos_array.as_ptr(),
                x_offset,
                y_offset,
                z_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        };
        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, `y_pos_array`, `z_pos_array` and `w_pos_array` are empty or do not have the same length.
    pub fn gen_position_array_4d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        z_pos_array: &[f32],
        w_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        w_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        check_position_arrays(
            noise_out,
            &[x_pos_array, y_pos_array, z_pos_array, w_pos_array],
        );
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenPositionArray4D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                noise_out.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                z_pos_array.as_ptr(),
                w_pos_array.as_ptr(),
                x_offset,
                y_offset,
                z_offset,
                w_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        };
        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if a size is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_size * y_size`.
    pub fn gen_tileable_2d(
        &self,
        noise_out: &mut [f32],
        x_size: i32,
        y_size: i32,
        x_step_size: f32,
        y_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= grid_len(&[x_size, y_size]));
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenTileable2D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                x_size,
                y_size,
                x_step_size,
                y_step_size,
                seed,
                min_max.as_mut_ptr(),
            )
        };
        OutputMinMax::new(min_max)
    }

    pub fn gen_single_2d(&self, x: f32, y: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle2D(self.as_ptr(), x, y, seed) }
    }

    pub fn gen_single_3d(&self, x: f32, y: f32, z: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle3D(self.as_ptr(), x, y, z, seed) }
    }

    pub fn gen_single_4d(&self, x: f32, y: f32, z: f32, w: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle4D(self.as_ptr(), x, y, z, w, seed) }
    }
}

/// Anything that can be built into a [`Node`].
pub trait Generator {
    fn build(&self) -> Node;
}

impl Generator for Node {
    /// Shares the node, no C++ node is created.
    fn build(&self) -> Node {
        self.clone()
    }
}

impl<G: Generator + ?Sized> Generator for &G {
    fn build(&self) -> Node {
        (**self).build()
    }
}

/// Value of a hybrid member: a constant or a node.
#[derive(Clone, Debug)]
pub enum Hybrid {
    Value(f32),
    Node(Node),
}

impl From<f32> for Hybrid {
    fn from(value: f32) -> Self {
        Self::Value(value)
    }
}

impl<G: Generator> From<G> for Hybrid {
    fn from(generator: G) -> Self {
        Self::Node(generator.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENCODED_NODE_TREE: &str =
        "E@BBZEG@BD8JFgIECArXIzwECiQIw/UoPwkuAAE@BJDQAH@BC@AIEAJBw@ABZEED0KV78YZmZmPwQDmpkZPwsAAIA/HAMAAHBCBA==";

    fn grid(node: &Node) -> Vec<f32> {
        let mut output = vec![0.0; 64];
        node.gen_uniform_grid_2d(&mut output, -40.0, -40.0, 8, 8, 10.0, 10.0, 1337);
        output
    }

    #[test]
    fn test_node_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Node>();
    }

    #[test]
    fn test_encoded_node_tree() {
        let node = Node::from_encoded_node_tree(ENCODED_NODE_TREE).unwrap();
        assert!(grid(&node).iter().any(|value| *value != 0.0));
        assert!(matches!(
            Node::from_encoded_node_tree("not a tree"),
            Err(FastNoiseError::NodeCreationFailed)
        ));
    }

    #[test]
    fn test_build_and_hybrid_share_the_node() {
        let node = Node::from_encoded_node_tree(ENCODED_NODE_TREE).unwrap();
        assert!(Arc::ptr_eq(&node.build().0, &node.0));
        assert!(Arc::ptr_eq(&<&Node as Generator>::build(&&node).0, &node.0));
        let Hybrid::Node(hybrid) = Hybrid::from(&node) else {
            panic!("expected a node");
        };
        assert!(Arc::ptr_eq(&hybrid.0, &node.0));
        assert!(matches!(Hybrid::from(0.5), Hybrid::Value(value) if value == 0.5));
    }
}
