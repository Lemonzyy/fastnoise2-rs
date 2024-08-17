use std::time::Instant;

use fastnoise2::{
    generator::{basic::position_output, simplex::opensimplex2, Node},
    TypedFastNoise,
};
use image::{GrayImage, Luma};

const X_SIZE: i32 = 1024;
const Y_SIZE: i32 = 1024;

fn create_node() -> TypedFastNoise {
    (opensimplex2().fbm(0.65, 0.5, 4, 2.5).scale(0.66)
        + position_output([0.0, 3.0, 0.0, 0.0], [0.0; 4]))
    .warp_gradient(0.2, 2.0)
    .warp_progressive(0.7, 0.5, 2, 2.5)
    .build_node()
}

fn main() {
    let noise = create_node();
    println!("SIMD level: {}", noise.get_simd_level());

    let mut noise_out = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let start = Instant::now();
    let min_max = noise.gen_uniform_grid_2d(
        &mut noise_out,
        -X_SIZE / 2,
        -Y_SIZE / 2,
        X_SIZE,
        Y_SIZE,
        0.02,
        1337,
    );
    let elapsed = start.elapsed();

    println!(
        "Took {elapsed:?} to generate {} values ({}/s): {min_max:?}",
        noise_out.len(),
        noise_out.len() as f32 / elapsed.as_secs_f32()
    );

    // Do whatever you want with `noise_out`! In this case, generate an image with it.

    let mut img = GrayImage::new(X_SIZE as u32, Y_SIZE as u32);

    for x in 0..X_SIZE {
        for y in 0..Y_SIZE {
            let index = ((Y_SIZE - 1 - y) * X_SIZE + x) as usize;
            let value = noise_out[index];
            let pixel_value = (255.0 * ((value + 1.0) / 2.0)) as u8;
            img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    save(img, "typed_simple_terrain.png");
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
