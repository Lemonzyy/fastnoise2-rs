// This example illustrates the use of the typed API with generators to build safe trees. It builds the "Simple Terrain" example integrated into an old version of the Node Editor.
use std::time::Instant;

use fastnoise2::{generator::prelude::*, SafeNode};
use image::{GrayImage, Luma};

const X_SIZE: i32 = 1024;
const Y_SIZE: i32 = 1024;

fn create_node() -> GeneratorWrapper<SafeNode> {
    (supersimplex().fbm(0.65, 0.5, 4, 2.5).domain_scale(0.66)
        + gradient()
            .with_multipliers([0.0, 3.0, 0.0, 0.0])
            .with_offsets([0.0; 4]))
    .domain_warp_gradient(0.2, 2.0)
    .domain_warp_progressive(0.7, 0.5, 2, 2.5)
    .build()
}

fn main() {
    let node = create_node();
    println!("SIMD level: {}", node.get_simd_level());

    let mut noise = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let step_size = 0.02;
    let start = Instant::now();
    let min_max = node.gen_uniform_grid_2d(
        &mut noise,
        -X_SIZE as f32 / 2.0 * step_size, // x_offset
        -Y_SIZE as f32 / 2.0 * step_size, // y_offset
        X_SIZE,                           // x_count
        Y_SIZE,                           // y_count
        step_size,                        // x_step_size
        step_size,                        // y_step_size
        1337,
    );
    let elapsed = start.elapsed();

    println!(
        "Took {elapsed:?} to generate {} values ({}/s): {min_max:?}",
        noise.len(),
        noise.len() as f32 / elapsed.as_secs_f32()
    );

    // Do whatever you want with `noise`! In this case, generate an image with it.

    let mut img = GrayImage::new(X_SIZE as u32, Y_SIZE as u32);

    for x in 0..X_SIZE {
        for y in 0..Y_SIZE {
            let index = ((Y_SIZE - 1 - y) * X_SIZE + x) as usize;
            let value = noise[index];
            let pixel_value = (255.0 * ((value + 1.0) / 2.0)) as u8;
            img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    save(img, "safe_simple_terrain.png");
}

fn save(img: GrayImage, filename: &str) {
    let output_dir =
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default())
            .join("examples_output");
    std::fs::create_dir_all(&output_dir).expect("Failed to create directories");
    let output_path = output_dir.join(filename);
    img.save(&output_path).expect("Failed to save image");
    println!("Image successfully saved as {}", output_path.display());
}
