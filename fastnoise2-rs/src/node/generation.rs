//! Noise generation from built nodes.
use fastnoise2_sys::*;

use super::Node;
use crate::OutputMinMax;

impl Node {
    /// # Panics
    /// Panics if a count is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_count * y_count`.
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.name())))]
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

        // SAFETY: the node is valid, `noise_out` holds the whole grid (asserted above) and
        // `min_max` the two values FastNoise2 writes
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
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.name())))]
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

        // SAFETY: the node is valid, `noise_out` holds the whole grid (asserted above) and
        // `min_max` the two values FastNoise2 writes
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
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.name())))]
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

        // SAFETY: the node is valid, `noise_out` holds the whole grid (asserted above) and
        // `min_max` the two values FastNoise2 writes
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
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.name())))]
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

        // SAFETY: the node is valid, `noise_out` and the position arrays hold `noise_out.len()`
        // values (checked above) and `min_max` the two values FastNoise2 writes
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
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.name())))]
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

        // SAFETY: the node is valid, `noise_out` and the position arrays hold `noise_out.len()`
        // values (checked above) and `min_max` the two values FastNoise2 writes
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
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.name())))]
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

        // SAFETY: the node is valid, `noise_out` and the position arrays hold `noise_out.len()`
        // values (checked above) and `min_max` the two values FastNoise2 writes
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
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.name())))]
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

        // SAFETY: the node is valid, `noise_out` holds the whole grid (asserted above) and
        // `min_max` the two values FastNoise2 writes
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

    #[inline]
    pub fn gen_single_2d(&self, x: f32, y: f32, seed: i32) -> f32 {
        // SAFETY: the node is valid
        unsafe { fnGenSingle2D(self.as_ptr(), x, y, seed) }
    }

    #[inline]
    pub fn gen_single_3d(&self, x: f32, y: f32, z: f32, seed: i32) -> f32 {
        // SAFETY: the node is valid
        unsafe { fnGenSingle3D(self.as_ptr(), x, y, z, seed) }
    }

    #[inline]
    pub fn gen_single_4d(&self, x: f32, y: f32, z: f32, w: f32, seed: i32) -> f32 {
        // SAFETY: the node is valid
        unsafe { fnGenSingle4D(self.as_ptr(), x, y, z, w, seed) }
    }
}

/// Returns the number of values in a grid, as computed by FastNoise2 (an `i32` product).
#[inline]
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

#[inline]
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
    use crate::NodeBuilder;

    fn perlin() -> Node {
        NodeBuilder::new("Perlin").unwrap().build().unwrap()
    }

    #[test]
    #[should_panic(expected = "grid counts must be positive")]
    fn test_gen_uniform_grid_2d_zero_count() {
        perlin().gen_uniform_grid_2d(&mut [], 0.0, 0.0, 0, 4, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "grid counts must be positive")]
    fn test_gen_tileable_2d_zero_size() {
        perlin().gen_tileable_2d(&mut [], 0, 0, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "grid size must fit in an i32")]
    fn test_gen_uniform_grid_2d_overflow() {
        let mut output = vec![0.0; 65536];
        perlin().gen_uniform_grid_2d(&mut output, 0.0, 0.0, 65536, 65537, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "position arrays must not be empty")]
    fn test_gen_position_array_2d_empty() {
        perlin().gen_position_array_2d(&mut [], &[], &[], 0.0, 0.0, 1337);
    }

    #[test]
    #[should_panic(expected = "noise_out and position arrays must have the same length")]
    fn test_gen_position_array_4d_short_w() {
        let positions = [0.0; 4];
        perlin().gen_position_array_4d(
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
