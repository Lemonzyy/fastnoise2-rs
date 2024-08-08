use std::time::Instant;

use fastnoise2_rs::FastNoise;
use image::{GrayImage, Luma};

const DEFAULT_ENCODED_NODE_TREE: &str = "DQAFAAAAAAAAQAgAAAAAAD8AAAAAAA==";
const X_SIZE: i32 = 4096;
const Y_SIZE: i32 = 4096;

fn main() {
    let encoded_node_tree = std::env::args().nth(1).unwrap_or_else(|| {
        println!(
            "Invalid or unspecified encoded node tree, defaulting to {DEFAULT_ENCODED_NODE_TREE}"
        );
        DEFAULT_ENCODED_NODE_TREE.to_string()
    });

    let Some(node) = FastNoise::from_encoded_node_tree(&encoded_node_tree) else {
        panic!("Failed to create node");
    };

    println!("SIMD level: {}", node.get_simd_level());

    let mut noise_out = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let start = Instant::now();
    let min_max = node.gen_uniform_grid_2d(
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

    let mut img = GrayImage::new(X_SIZE as u32, Y_SIZE as u32);

    for x in 0..X_SIZE {
        for y in 0..Y_SIZE {
            let index = ((Y_SIZE - 1 - y) * X_SIZE + x) as usize;
            let value = noise_out[index];
            let pixel_value = (255.0 * ((value + 1.0) / 2.0)) as u8;
            img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    std::fs::create_dir_all("examples_output").expect("Failed to create directories");
    img.save("examples_output/encoded_node_tree.png")
        .expect("Failed to save image");
    println!("Image saved as examples_output/encoded_node_tree.png");
}
