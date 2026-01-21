use std::sync::Arc;

use crate::{FastNoiseError, Node, OutputMinMax};

/// Unlike [`Node`], this structure is safe to use because it is built from typed nodes
/// that implement the [`Generator`][`crate::generator::Generator`] trait, or built by an encoded node tree produced by the [Node Editor](https://github.com/Auburn/FastNoise2?tab=readme-ov-file#node-editor).
///
/// You can see how to use it in the [`generator`][`crate::generator`] module.
#[derive(Debug, Clone)]
pub struct SafeNode(pub(crate) Arc<Node>);

unsafe impl Send for SafeNode {}
unsafe impl Sync for SafeNode {}

impl SafeNode {
    /// Creates a [`SafeNode`] instance from an encoded node tree.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree is invalid or if creation fails.
    pub fn from_encoded_node_tree(encoded_node_tree: &str) -> Result<Self, FastNoiseError> {
        Node::from_encoded_node_tree(encoded_node_tree)
            .map(Arc::new)
            .map(Self)
    }

    pub fn get_simd_level(&self) -> u32 {
        self.0.get_simd_level()
    }

    /// # Panics
    /// Panics if `noise_out.len() < x_count * y_count`.
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
        assert!(noise_out.len() >= (x_count * y_count) as usize);

        unsafe {
            self.0.gen_uniform_grid_2d_unchecked(
                noise_out,
                x_offset,
                y_offset,
                x_count,
                y_count,
                x_step_size,
                y_step_size,
                seed,
            )
        }
    }

    /// # Panics
    /// Panics if `noise_out.len() < x_count * y_count * z_count`.
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
        assert!(noise_out.len() >= (x_count * y_count * z_count) as usize);

        unsafe {
            self.0.gen_uniform_grid_3d_unchecked(
                noise_out,
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
            )
        }
    }

    /// # Panics
    /// Panics if `noise_out.len() < x_count * y_count * z_count * w_count`.
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
        assert!(noise_out.len() >= (x_count * y_count * z_count * w_count) as usize);

        unsafe {
            self.0.gen_uniform_grid_4d_unchecked(
                noise_out,
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
            )
        }
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, and `y_pos_array` do not have the same length.
    pub fn gen_position_array_2d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() == x_pos_array.len() && x_pos_array.len() == y_pos_array.len());

        unsafe {
            self.0.gen_position_array_2d_unchecked(
                noise_out,
                x_pos_array,
                y_pos_array,
                x_offset,
                y_offset,
                seed,
            )
        }
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, `y_pos_array`, and `z_pos_array` do not have the same length.
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
        assert!(
            noise_out.len() == x_pos_array.len()
                && x_pos_array.len() == y_pos_array.len()
                && y_pos_array.len() == z_pos_array.len()
        );

        unsafe {
            self.0.gen_position_array_3d_unchecked(
                noise_out,
                x_pos_array,
                y_pos_array,
                z_pos_array,
                x_offset,
                y_offset,
                z_offset,
                seed,
            )
        }
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, `y_pos_array`, `z_pos_array` and `w_pos_array` do not have the same length.
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
        assert!(
            noise_out.len() == x_pos_array.len()
                && x_pos_array.len() == y_pos_array.len()
                && y_pos_array.len() == z_pos_array.len()
        );

        unsafe {
            self.0.gen_position_array_4d_unchecked(
                noise_out,
                x_pos_array,
                y_pos_array,
                z_pos_array,
                w_pos_array,
                x_offset,
                y_offset,
                z_offset,
                w_offset,
                seed,
            )
        }
    }

    /// # Panics
    /// Panics if `noise_out.len() < x_size * y_size`.
    pub fn gen_tileable_2d(
        &self,
        noise_out: &mut [f32],
        x_size: i32,
        y_size: i32,
        x_step_size: f32,
        y_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= (x_size * y_size) as usize);

        unsafe {
            self.0.gen_tileable_2d_unchecked(
                noise_out,
                x_size,
                y_size,
                x_step_size,
                y_step_size,
                seed,
            )
        }
    }

    pub fn gen_single_2d(&self, x: f32, y: f32, seed: i32) -> f32 {
        unsafe { self.0.gen_single_2d_unchecked(x, y, seed) }
    }

    pub fn gen_single_3d(&self, x: f32, y: f32, z: f32, seed: i32) -> f32 {
        unsafe { self.0.gen_single_3d_unchecked(x, y, z, seed) }
    }

    pub fn gen_single_4d(&self, x: f32, y: f32, z: f32, w: f32, seed: i32) -> f32 {
        unsafe { self.0.gen_single_4d_unchecked(x, y, z, w, seed) }
    }
}
