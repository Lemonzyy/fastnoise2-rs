// This example illustrates the typed API generated from FastNoise2 metadata: every node is a
// type with builder methods, nodes are chained from their first input, and operators work with
// constants on either side.
use std::{env, fs, path::PathBuf};

use fastnoise2::prelude::*;
use image::{GrayImage, Luma};

const SIZE: i32 = 512;

fn create_node() -> Node {
    // Built once and used three times below: every use shares the same C++ node
    let perlin = perlin().with_feature_scale(150.0).build();

    let fbm = perlin
        .fractal_f_bm()
        .with_octaves(5)
        .with_gain(0.5)
        .domain_warp_gradient()
        // A hybrid member driven by a node: the warp amplitude varies with `perlin`
        .with_warp_amplitude(&perlin * 40.0)
        .with_feature_scale(80.0);

    // Constants on either side of operators
    let ridges = 1.0 - perlin.fractal_ridged().with_octaves(3).abs();

    (0.5 * fbm + &ridges * 0.5)
        .remap()
        .with_from_min(-1.0)
        .with_from_max(1.0)
        .with_to_min(0.0)
        .with_to_max(1.0)
        .build()
}

fn main() {
    let node = create_node();
    println!(
        "{node:?}, active feature set: {}",
        node.get_active_feature_set()
    );

    let mut noise = vec![0.0; (SIZE * SIZE) as usize];
    let min_max = node.gen_uniform_grid_2d(&mut noise, 0.0, 0.0, SIZE, SIZE, 1.0, 1.0, 1337);
    println!("{min_max:?}");

    let image = GrayImage::from_fn(SIZE as u32, SIZE as u32, |x, y| {
        let value = noise[(y * SIZE as u32 + x) as usize];
        Luma([(value.clamp(0.0, 1.0) * 255.0) as u8])
    });
    let output_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join("examples_output");
    fs::create_dir_all(&output_dir).expect("Failed to create directories");
    let output_path = output_dir.join("nodes.png");
    image.save(&output_path).expect("Failed to save image");
    println!("Image successfully saved as {}", output_path.display());
}
