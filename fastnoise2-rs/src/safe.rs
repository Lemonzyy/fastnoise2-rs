use std::sync::Arc;

use crate::{FastNoiseError, Node, OutputMinMax};

/// Unlike [`Node`], this structure is safe to use because it is built from typed nodes
/// that implement the [`Generator`][`crate::generator::Generator`] trait, or built by an encoded node tree produced by the [Node Editor](https://github.com/Auburn/FastNoise2?tab=readme-ov-file#node-editor).
///
/// You can create and test node trees using the [Web WASM Node Editor](https://auburn.github.io/fastnoise2nodeeditor/) or download desktop binaries from [FastNoise2 Releases](https://github.com/Auburn/FastNoise2/releases/latest).
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

    pub fn get_active_feature_set(&self) -> u32 {
        self.0.get_active_feature_set()
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

/// Returns the number of values in a grid, as computed by FastNoise2 (an `i32` product).
pub(crate) fn grid_len(counts: &[i32]) -> usize {
    assert!(
        counts.iter().all(|&count| count > 0),
        "grid counts must be positive"
    );
    counts
        .iter()
        .try_fold(1i32, |len, &count| len.checked_mul(count))
        .expect("grid size must fit in an i32") as usize
}

pub(crate) fn check_position_arrays(noise_out: &[f32], pos_arrays: &[&[f32]]) {
    let len = noise_out.len();
    assert!(len > 0, "position arrays must not be empty");
    assert!(len <= i32::MAX as usize, "position arrays are too long");
    assert!(
        pos_arrays.iter().all(|pos_array| pos_array.len() == len),
        "noise_out and position arrays must have the same length"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        generator::{perlin::perlin, Generator},
        test_utils::*,
    };

    #[test]
    fn test_encoded_node_tree() {
        // "Mountain Terrain" example from the Node Editor
        let encoded = "E@BBZEG@BD8JFgIECArXIzwECiQIw/UoPwkuAAE@BJDQAH@BC@AIEAJBw@ABZEED0KV78YZmZmPwQDmpkZPwsAAIA/HAMAAHBCBA==";
        let node = SafeNode::from_encoded_node_tree(encoded).unwrap();
        test_generator_produces_output(node);
    }

    #[test]
    fn test_gen_single_2d() {
        let node = perlin().build();
        let value = node.0.gen_single_2d(0.5, 0.5, 1337);
        assert!(value.is_finite());
        assert!((-1.5..=1.5).contains(&value)); // Perlin should be roughly -1 to 1
    }

    #[test]
    fn test_gen_single_3d() {
        let node = perlin().build();
        let value = node.0.gen_single_3d(0.5, 0.5, 0.5, 1337);
        assert!(value.is_finite());
    }

    #[test]
    fn test_gen_uniform_grid_3d() {
        let node = perlin().build();
        let mut output = [0.0f32; 64]; // 4x4x4
        let min_max =
            node.0
                .gen_uniform_grid_3d(&mut output, 0.0, 0.0, 0.0, 4, 4, 4, 0.1, 0.1, 0.1, 1337);
        assert!(min_max.min.is_finite());
        assert!(min_max.max.is_finite());
        assert!(output.iter().any(|&v| v != output[0]));
    }

    #[test]
    fn test_gen_uniform_grid_4d() {
        let node = perlin().build();
        let mut output = [0.0f32; 16]; // 2x2x2x2
        let min_max = node.0.gen_uniform_grid_4d(
            &mut output,
            0.0,
            0.0,
            0.0,
            0.0,
            2,
            2,
            2,
            2,
            0.1,
            0.1,
            0.1,
            0.1,
            1337,
        );
        assert!(min_max.min.is_finite());
        assert!(min_max.max.is_finite());
        assert!(output.iter().any(|&v| v != output[0]));
    }

    #[test]
    fn test_gen_position_array_2d() {
        let node = perlin().build();
        let x_pos = [0.0, 0.1, 0.2, 0.3];
        let y_pos = [0.0, 0.1, 0.2, 0.3];
        let mut output = [0.0f32; 4];
        let min_max = node
            .0
            .gen_position_array_2d(&mut output, &x_pos, &y_pos, 0.0, 0.0, 1337);
        assert!(min_max.min.is_finite());
        assert!(min_max.max.is_finite());
        assert!(output.iter().all(|&v| v.is_finite()));
    }

    #[test]
    fn test_gen_position_array_3d() {
        let node = perlin().build();
        let x_pos = [0.0, 0.1, 0.2, 0.3];
        let y_pos = [0.0, 0.1, 0.2, 0.3];
        let z_pos = [0.0, 0.1, 0.2, 0.3];
        let mut output = [0.0f32; 4];
        let min_max =
            node.0
                .gen_position_array_3d(&mut output, &x_pos, &y_pos, &z_pos, 0.0, 0.0, 0.0, 1337);
        assert!(min_max.min.is_finite());
        assert!(min_max.max.is_finite());
        assert!(output.iter().all(|&v| v.is_finite()));
    }

    #[test]
    fn test_gen_position_array_4d() {
        let node = perlin().build();
        let x_pos = [0.0, 0.1, 0.2, 0.3];
        let y_pos = [0.0, 0.1, 0.2, 0.3];
        let z_pos = [0.0, 0.1, 0.2, 0.3];
        let w_pos = [0.0, 0.1, 0.2, 0.3];
        let mut output = [0.0f32; 4];
        let min_max = node.0.gen_position_array_4d(
            &mut output,
            &x_pos,
            &y_pos,
            &z_pos,
            &w_pos,
            0.0,
            0.0,
            0.0,
            0.0,
            1337,
        );
        assert!(min_max.min.is_finite());
        assert!(min_max.max.is_finite());
        assert!(output.iter().all(|&v| v.is_finite()));
    }

    #[test]
    fn test_gen_tileable_2d() {
        let node = perlin().build();
        let mut output = [0.0f32; 16]; // 4x4
        let min_max = node.0.gen_tileable_2d(&mut output, 4, 4, 0.1, 0.1, 1337);
        assert!(min_max.min.is_finite());
        assert!(min_max.max.is_finite());
        assert!(output.iter().any(|&v| v != output[0]));
    }

    #[test]
    fn test_gen_single_4d() {
        let node = perlin().build();
        let value = node.0.gen_single_4d(0.5, 0.5, 0.5, 0.5, 1337);
        assert!(value.is_finite());
    }

    #[test]
    fn test_get_active_feature_set() {
        let node = perlin().build();
        let feature_set = node.0.get_active_feature_set();
        // FastSIMD::FeatureSet flags, the actual value depends on the CPU.
        // 0 is FeatureSet::Invalid and u32::MAX is FeatureSet::Max.
        assert_ne!(feature_set, 0);
        assert_ne!(feature_set, u32::MAX);
    }

    #[test]
    #[should_panic(expected = "grid counts must be positive")]
    fn test_gen_uniform_grid_2d_zero_count() {
        perlin()
            .build()
            .gen_uniform_grid_2d(&mut [], 0.0, 0.0, 0, 4, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "grid counts must be positive")]
    fn test_gen_tileable_2d_zero_size() {
        perlin()
            .build()
            .gen_tileable_2d(&mut [], 0, 0, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "grid size must fit in an i32")]
    fn test_gen_uniform_grid_2d_overflow() {
        let mut output = vec![0.0; 65536];
        perlin()
            .build()
            .gen_uniform_grid_2d(&mut output, 0.0, 0.0, 65536, 65537, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "position arrays must not be empty")]
    fn test_gen_position_array_2d_empty() {
        perlin()
            .build()
            .gen_position_array_2d(&mut [], &[], &[], 0.0, 0.0, 1337);
    }

    #[test]
    #[should_panic(expected = "noise_out and position arrays must have the same length")]
    fn test_gen_position_array_4d_short_w() {
        let positions = [0.0; 4];
        perlin().build().gen_position_array_4d(
            &mut [0.0; 4],
            &positions,
            &positions,
            &positions,
            &positions[..1],
            0.0,
            0.0,
            0.0,
            0.0,
            1337,
        );
    }
}
