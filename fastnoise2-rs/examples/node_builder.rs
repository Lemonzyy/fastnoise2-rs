// This example illustrates the dynamic API: nodes are created from FastNoise2 metadata names,
// members are set by name, and `build` checks that every input is set.
use std::{env, fs, path::PathBuf};

use fastnoise2::node::{Hybrid, Node, NodeBuilder};
use image::{GrayImage, Luma};

const SIZE: i32 = 512;

fn create_node() -> Result<Node, fastnoise2::FastNoiseError> {
    // Built once and used twice below: both uses share the same C++ node
    let perlin = NodeBuilder::new("Perlin")?
        .set("Feature Scale", 150.0)?
        .build()?;

    let fbm = NodeBuilder::new("FractalFBm")?
        .set("Source", &perlin)?
        .set("Octaves", 5)?
        .set("Gain", 0.5)?
        .build()?;

    // A hybrid member driven by a node: the warp amplitude varies with `perlin`
    let warp = NodeBuilder::new("DomainWarpGradient")?
        .set("Source", &fbm)?
        .set("Warp Amplitude", Hybrid::from(&perlin))?
        .set("Feature Scale", 80.0)?
        .build()?;

    NodeBuilder::new("Remap")?
        .set("Source", &warp)?
        .set("From Min", -1.0)?
        .set("From Max", 1.0)?
        .set("To Min", 0.0)?
        .set("To Max", 1.0)?
        .build()
}

fn main() {
    let node = create_node().unwrap();
    println!(
        "{node:?}, active feature set: {}",
        node.get_active_feature_set()
    );

    let mut noise = vec![0.0; (SIZE * SIZE) as usize];
    let min_max = node.gen_uniform_grid_2d(&mut noise, 0.0, 0.0, SIZE, SIZE, 1.0, 1.0, 1337);
    println!("{min_max:?}");

    // Errors carry the node, the member and what FastNoise2 expects
    for error in [
        NodeBuilder::new("Perln").err(),
        NodeBuilder::new("FractalFBm").unwrap().build().err(),
        NodeBuilder::new("Perlin")
            .unwrap()
            .set("Seed Offset", 1.5)
            .err(),
        NodeBuilder::new("DomainWarpFractalProgressive")
            .unwrap()
            .set("Domain Warp Source", &node)
            .err(),
    ]
    .into_iter()
    .flatten()
    {
        println!("error: {error}");
    }

    let image = GrayImage::from_fn(SIZE as u32, SIZE as u32, |x, y| {
        let value = noise[(y * SIZE as u32 + x) as usize];
        Luma([(value.clamp(0.0, 1.0) * 255.0) as u8])
    });
    let output_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join("examples_output");
    fs::create_dir_all(&output_dir).expect("Failed to create directories");
    let output_path = output_dir.join("node_builder.png");
    image.save(&output_path).expect("Failed to save image");
    println!("Image successfully saved as {}", output_path.display());
}
