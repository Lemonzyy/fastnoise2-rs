//! FastNoise2 WASM Demo - Simple noise visualization

use fastnoise2::generator::prelude::*;
use fastnoise2::SafeNode;

/// Generate 2D noise and write grayscale values (0-255) to output buffer
#[no_mangle]
pub extern "C" fn generate_noise(output: *mut u8, width: i32, height: i32) {
    let size = (width * height) as usize;
    let node: GeneratorWrapper<SafeNode> = simplex().fbm(0.5, 0.0, 4, 2.0).build();

    let mut float_output = vec![0.0f32; size];
    node.gen_uniform_grid_2d(
        &mut float_output,
        0.0, 0.0,
        width, height,
        0.01, 0.01,
        1337,
    );

    let output_slice = unsafe { std::slice::from_raw_parts_mut(output, size) };
    for (i, &v) in float_output.iter().enumerate() {
        output_slice[i] = ((v + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
    }
}

fn main() {}
