use std::time::Instant;

use fastnoise2::FastNoise;
use image::{GrayImage, Luma};

const DEFAULT_ENCODED_NODE_TREE: &str = "EQACAAAAAAAgQBAAAAAAQBkAEwDD9Sg/DQAEAAAAAAAgQAkAAGZmJj8AAAAAPwEEAAAAAAAAAEBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM3MTD4AMzMzPwAAAAA/";
const X_SIZE: i32 = 1024;
const Y_SIZE: i32 = 1024;

fn main() {
    let encoded_node_tree = std::env::args().nth(1).unwrap_or_else(|| {
        println!(
            "Invalid or unspecified encoded node tree, defaulting to '{DEFAULT_ENCODED_NODE_TREE}'"
        );
        DEFAULT_ENCODED_NODE_TREE.to_string()
    });

    let node = FastNoise::from_encoded_node_tree(&encoded_node_tree).unwrap();

    let mut noise_out = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let start = Instant::now();
    // SAFETY:
    // Using `FastNoise::from_encoded_node_tree` is generally safer than manually constructing the node tree with
    // `FastNoise::from_name` and `FastNoise::set`, as it ensures the nodes and parameters are correctly set by the C++ library's
    // tools. However, once the node is created, modifying parameters directly using `FastNoise::set` can introduce the same
    // risks as manually building the node tree. Issues might arise due to incorrect parameter types, missing members, or other
    // configuration errors. Ensure that all modifications are valid and consult the FastNoise2 documentation for guidance on
    // parameter types and expected values.
    let min_max = unsafe {
        node.gen_uniform_grid_2d_unchecked(
            &mut noise_out,
            -X_SIZE / 2,
            -Y_SIZE / 2,
            X_SIZE,
            Y_SIZE,
            0.01,
            1337,
        )
    };
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

    save(img, "encoded_node_tree.png");
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