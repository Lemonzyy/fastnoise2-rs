use std::time::Instant;

use fastnoise2::{FastNoise, FastNoiseError};
use image::{GrayImage, Luma};

const X_SIZE: i32 = 1024;
const Y_SIZE: i32 = 1024;

fn create_node() -> Result<FastNoise, FastNoiseError> {
    let mut cellular = FastNoise::from_name("CellularDistance")?;
    cellular.set("ReturnType", "Index0Add1")?;
    cellular.set("DistanceIndex0", 2)?;

    let mut fractal = FastNoise::from_name("FractalFBm")?;
    let simplex = FastNoise::from_name("Simplex")?;
    fractal.set("Source", &simplex)?;
    fractal.set("Gain", 3.0)?;
    fractal.set("Lacunarity", 0.6)?;

    let mut add_dim = FastNoise::from_name("AddDimension")?;
    add_dim.set("Source", &cellular)?;
    add_dim.set("NewDimensionPosition", 0.5)?;

    let mut max_smooth = FastNoise::from_name("MaxSmooth")?;
    max_smooth.set("LHS", &fractal)?;
    max_smooth.set("RHS", &add_dim)?;

    Ok(max_smooth)
}

fn main() {
    let noise = create_node().unwrap();
    println!("SIMD level: {}", noise.get_simd_level());

    let mut noise_out = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let start = Instant::now();
    // SAFETY:
    // Even though `noise_out` has sufficient capacity and the `noise` node seems correctly configured
    // (with valid node names and parameter types), undefined behavior may still occur during noise generation.
    // Potential issues include:
    // - Incorrect or incomplete parameter setup (e.g., missing source nodes or invalid values).
    // - Internal errors or limitations in the FastNoise2 library that may not be evident from Rust's type safety
    //   or runtime checks.
    // Verify that all node configurations and parameters are correct.
    let min_max = unsafe {
        noise.gen_uniform_grid_2d_unchecked(
            &mut noise_out,
            -X_SIZE / 2,
            -Y_SIZE / 2,
            X_SIZE,
            Y_SIZE,
            0.02,
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

    save(img, "manual.png");
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
