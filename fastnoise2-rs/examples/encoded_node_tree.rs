// This example illustrates the use of "Node::from_encoded_node_tree" to build a node tree exported by the Node Editor.
use std::{env, fs, path::PathBuf, time::Instant};

use fastnoise2::Node;
use image::{GrayImage, Luma};

// "Mountain Terrain" tree integrated into FastNoise2 Node Editor.
const DEFAULT_ENCODED_NODE_TREE: &str =
    "E@BBZEG@BD8JFgIECArXIzwECiQIw/UoPwkuAAE@BJDQAH@BC@AIEAJBw@ABZEED0KV78YZmZmPwQDmpkZPwsAAIA/HAMAAHBCBA==";
const X_SIZE: i32 = 1024;
const Y_SIZE: i32 = 1024;

fn main() {
    let encoded_node_tree = env::args().nth(1).unwrap_or_else(|| {
        println!(
            "Invalid or unspecified encoded node tree, defaulting to '{DEFAULT_ENCODED_NODE_TREE}'"
        );
        DEFAULT_ENCODED_NODE_TREE.to_string()
    });

    let node = Node::from_encoded_node_tree(&encoded_node_tree).unwrap();

    let mut noise = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let start = Instant::now();
    let step_size = 3.0;
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

    save(img, "encoded_node_tree.png");
}

fn save(img: GrayImage, filename: &str) {
    let output_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join("examples_output");
    fs::create_dir_all(&output_dir).expect("Failed to create directories");
    let output_path = output_dir.join(filename);
    img.save(&output_path).expect("Failed to save image");
    println!("Image successfully saved as {}", output_path.display());
}
