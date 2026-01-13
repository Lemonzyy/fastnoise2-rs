// This example illustrates the use of the metadata-based API to build trees.
use std::time::Instant;

use fastnoise2::{FastNoiseError, Node};
use image::{GrayImage, Luma};

const X_SIZE: i32 = 1024;
const Y_SIZE: i32 = 1024;

fn create_node() -> Result<Node, FastNoiseError> {
    let mut cellular = Node::from_name("CellularDistance")?;
    cellular.set("ReturnType", "Index0Add1")?;
    cellular.set("DistanceIndex0", 2)?;

    let mut fractal = Node::from_name("FractalFBm")?;
    let simplex = Node::from_name("Simplex")?;
    fractal.set("Source", &simplex)?;
    fractal.set("Gain", 3.0)?;
    fractal.set("Lacunarity", 0.6)?;

    let mut add_dim = Node::from_name("AddDimension")?;
    add_dim.set("Source", &cellular)?;
    add_dim.set("NewDimensionPosition", 0.5)?;

    let mut max_smooth = Node::from_name("MaxSmooth")?;
    max_smooth.set("LHS", &fractal)?;
    max_smooth.set("RHS", &add_dim)?;

    Ok(max_smooth)
}

fn main() {
    let node = create_node().unwrap();
    println!("SIMD level: {}", node.get_simd_level());

    let mut noise = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let start = Instant::now();
    // SAFETY:
    // Even though `noise` has sufficient capacity and the node seems correctly configured
    // (with valid node names and parameter types), undefined behavior may still occur during noise generation.
    // Potential issues include:
    // - Incorrect or incomplete parameter setup (e.g., missing source nodes or invalid values).
    // - Internal errors or limitations in the FastNoise2 library that may not be evident from Rust's type safety
    //   or runtime checks.
    // Verify that all node configurations and parameters are correct.
    let step_size = 0.02;
    let min_max = unsafe {
        node.gen_uniform_grid_2d_unchecked(
            &mut noise,
            -X_SIZE as f32 / 2.0 * step_size, // x_offset
            -Y_SIZE as f32 / 2.0 * step_size, // y_offset
            X_SIZE,                           // x_count
            Y_SIZE,                           // y_count
            step_size,                        // x_step_size
            step_size,                        // y_step_size
            1337,
        )
    };
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

    save(img, "manual.png");
}

fn save(img: GrayImage, filename: &str) {
    let output_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join("examples_output");
    std::fs::create_dir_all(&output_dir).expect("Failed to create directories");
    let output_path = output_dir.join(filename);
    img.save(&output_path).expect("Failed to save image");
    println!("Image successfully saved as {}", output_path.display());
}
