use std::rc::Rc;

use crate::{FastNoiseError, Node, OutputMinMax};

/// Unlike [`Node`], this structure is safe to use because it is built from typed nodes
/// that implement the [`TypedNode`][`crate::generator::TypedNode`] trait, or built by an encoded node tree produced by [NoiseTool](https://github.com/Auburn/FastNoise2?tab=readme-ov-file#noise-tool).
///
/// You can see how to use it in the [`generator`][`crate::generator`] module.
#[derive(Debug, Clone)]
pub struct SafeNode(pub(crate) Rc<Node>);

impl SafeNode {
    /// Creates a [`SafeNode`] instance from an encoded node tree.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree is invalid or if creation fails.
    pub fn from_encoded_node_tree(encoded_node_tree: &str) -> Result<Self, FastNoiseError> {
        Node::from_encoded_node_tree(encoded_node_tree)
            .map(Rc::new)
            .map(Self)
    }

    pub fn get_simd_level(&self) -> u32 {
        self.0.get_simd_level()
    }

    /// # Panics
    /// Panics if `noise_out.len() < x_size * y_size`.
    pub fn gen_uniform_grid_2d(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        x_size: i32,
        y_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= (x_size * y_size) as usize);

        unsafe {
            self.0.gen_uniform_grid_2d_unchecked(
                noise_out, x_start, y_start, x_size, y_size, frequency, seed,
            )
        }
    }

    /// # Panics
    /// Panics if `noise_out.len() < x_size * y_size * z_size`.
    pub fn gen_uniform_grid_3d(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        z_start: i32,
        x_size: i32,
        y_size: i32,
        z_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= (x_size * y_size * z_size) as usize);

        unsafe {
            self.0.gen_uniform_grid_3d_unchecked(
                noise_out, x_start, y_start, z_start, x_size, y_size, z_size, frequency, seed,
            )
        }
    }

    /// # Panics
    /// Panics if `noise_out.len() < x_size * y_size * z_size * w_size`.
    pub fn gen_uniform_grid_4d(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        z_start: i32,
        w_start: i32,
        x_size: i32,
        y_size: i32,
        z_size: i32,
        w_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= (x_size * y_size * z_size * w_size) as usize);

        unsafe {
            self.0.gen_uniform_grid_4d_unchecked(
                noise_out, x_start, y_start, z_start, w_start, x_size, y_size, z_size, w_size,
                frequency, seed,
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
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= (x_size * y_size) as usize);

        unsafe {
            self.0
                .gen_tileable_2d_unchecked(noise_out, x_size, y_size, frequency, seed)
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
