use std::time::Instant;

use fastnoise2::{FastNoise, MemberValue};
use image::{GrayImage, Luma};

const X_SIZE: i32 = 512;
const Y_SIZE: i32 = 512;

fn main() {
    let cellular = FastNoise::from_name("CellularDistance").unwrap();
    cellular
        .set("ReturnType", MemberValue::Enum("Index0Add1"))
        .unwrap();
    cellular.set("DistanceIndex0", MemberValue::Int(2)).unwrap();

    let fractal = FastNoise::from_name("FractalFBm").unwrap();
    let simplex = FastNoise::from_name("Simplex").unwrap();
    fractal
        .set("Source", MemberValue::NodeLookup(&simplex))
        .unwrap();
    fractal.set("Gain", MemberValue::Float(0.3)).unwrap();
    fractal.set("Lacunarity", MemberValue::Float(0.6)).unwrap();

    let add_dim = FastNoise::from_name("AddDimension").unwrap();
    add_dim
        .set("Source", MemberValue::NodeLookup(&cellular))
        .unwrap();
    add_dim
        .set("NewDimensionPosition", MemberValue::Float(0.5))
        .unwrap();

    let max_smooth = FastNoise::from_name("MaxSmooth").unwrap();
    max_smooth
        .set("LHS", MemberValue::NodeLookup(&fractal))
        .unwrap();
    max_smooth
        .set("RHS", MemberValue::NodeLookup(&add_dim))
        .unwrap();

    println!("SIMD level: {}", max_smooth.get_simd_level());

    let mut noise_out = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let start = Instant::now();
    let min_max = max_smooth.gen_uniform_grid_2d(&mut noise_out, 0, 0, X_SIZE, Y_SIZE, 0.02, 1337);
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
    img.save("examples_output/manual.png")
        .expect("Failed to save image");
    println!("Image saved as examples_output/manual.png");
}
