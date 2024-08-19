// // This example illustrates the use of different types of writing with generators to build safe trees.
// use std::time::Instant;

// use fastnoise2::{
//     generator::{
//         basic::{Constant, SineWave},
//         blend::{Add, Fade},
//         prelude::*,
//         simplex::Simplex,
//         Generator,
//     },
//     SafeNode,
// };
// use image::{GrayImage, Luma};

// const X_SIZE: i32 = 1024;
// const Y_SIZE: i32 = 1024;

// fn create_node() -> SafeNode {
//     // All these writings produce the same result.
//     // Please note that using "0.5" (a float) instead of "Constant { value : 0.5 }" (a node) is faster
//     // because the right-hand member of the "Add" node is a hybrid member (either a node or a float)
//     // that FastNoise2 handles differently. It is also easier to write "0.5".
//     let _n = Add {
//         lhs: Fade {
//             a: SineWave { scale: 0.1 },
//             b: SineWave { scale: -0.2 },
//             fade: Simplex,
//         },
//         rhs: Constant { value: 0.5 }, // this uses a Constant node as entry to a hybrid member
//     };

//     let _n = Add {
//         lhs: Fade {
//             a: SineWave { scale: 0.1 },
//             b: SineWave { scale: -0.2 },
//             fade: Simplex,
//         },
//         rhs: 0.5, // and this uses a float directly
//     };

//     // You can, and I would recommend this, use functions to instance different types of nodes wrapped in a Generator, necessary for operators such as + - * /
//     let _n = sinewave(0.1).fade(sinewave(-0.2), simplex()) + constant(0.5); // this uses a Constant node as entry to a hybrid member
//     let _n = sinewave(0.1).fade(sinewave(-0.2), simplex()) + 0.5; // and this uses a float directly

//     // You can also mix the two writings. Note the use of the Generator wrapper type to enable use of the operator
//     let _n = Generator(Fade {
//         a: SineWave { scale: 0.1 },
//         b: sinewave(-0.2),
//         fade: Simplex,
//     }) + 0.5;

//     // Qualifying the "fade" method can also lead to better syntax, although this is subjective.
//     let _n = Generator::fade(sinewave(0.1), sinewave(-0.2), simplex()) + 0.5;

//     // simplex() takes two unnecessary parentheses, so you can create the Simplex structure directly, since the Generator wrapper is not needed here.
//     let _n = Generator::fade(sinewave(0.1), sinewave(-0.2), Simplex) + 0.5;

//     // In the end, this is the most idiomatic writing, and it's easier to import functions by using "use fastnoise2::generator::prelude::*;"
//     let n = sinewave(0.1).fade(sinewave(-0.2), simplex()) + 0.5;

//     n.build_node()
// }

// fn main() {
//     let node = create_node();
//     println!("SIMD level: {}", node.get_simd_level());

//     let mut noise = vec![0.0; (X_SIZE * Y_SIZE) as usize];

//     let start = Instant::now();
//     let min_max = node.gen_uniform_grid_2d(
//         &mut noise,
//         -X_SIZE / 2,
//         -Y_SIZE / 2,
//         X_SIZE,
//         Y_SIZE,
//         0.02,
//         1337,
//     );
//     let elapsed = start.elapsed();

//     println!(
//         "Took {elapsed:?} to generate {} values ({}/s): {min_max:?}",
//         noise.len(),
//         noise.len() as f32 / elapsed.as_secs_f32()
//     );

//     // Do whatever you want with `noise`! In this case, generate an image with it.

//     let mut img = GrayImage::new(X_SIZE as u32, Y_SIZE as u32);

//     for x in 0..X_SIZE {
//         for y in 0..Y_SIZE {
//             let index = ((Y_SIZE - 1 - y) * X_SIZE + x) as usize;
//             let value = noise[index];
//             let pixel_value = (255.0 * ((value + 1.0) / 2.0)) as u8;
//             img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
//         }
//     }

//     save(img, "safe.png");
// }

// fn save(img: GrayImage, filename: &str) {
//     let output_dir =
//         std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default())
//             .join("examples_output");
//     std::fs::create_dir_all(&output_dir).expect("Failed to create directories");
//     let output_path = output_dir.join(filename);
//     img.save(&output_path).expect("Failed to save image");
//     println!("Image successfully saved as {}", output_path.display());
// }
// fn main() {
//     use fastnoise2::r#gen::{Add, Checkerboard, FractalFBm, NodeBuilder, SineWave};
//     let checkerboard = Checkerboard { size: 5.0 }.build();

//     let node = FractalFBm {
//         source: &Add {
//             lhs: &checkerboard.build(),
//             rhs: &0.5,
//         },
//         gain: &0.65,
//         weighted_strength: &checkerboard.build(),
//         octaves: 4,
//         lacunarity: 0.5,
//     };
//     println!("{:#?}", node);

//     let node = node.build();
//     // println!("{:#?}", node);

//     // let noise = node.gen_single_2d(250.0, 0.0, 123);
//     // println!("{noise}");
// }

// This example illustrates the use of different types of writing with generators to build safe trees.
use std::time::Instant;

use fastnoise2::{
    gen::{checkerboard, FractalFBm, Generator, GeneratorWrapper, GeneratorWrapperExt, Perlin},
    SafeNode,
};
use image::{GrayImage, Luma};

const X_SIZE: i32 = 2048;
const Y_SIZE: i32 = 2048;

fn create_node() -> GeneratorWrapper<SafeNode> {
    // Add {
    //     lhs: Checkerboard { size: 1.0 },
    //     rhs: SineWave { scale: 2.0 },
    // }
    // .build()
    let checkerboard = checkerboard(5.0).build();
    FractalFBm {
        source: Perlin.wrap() - checkerboard.clone(),
        gain: 0.65,
        weighted_strength: checkerboard + 0.5,
        octaves: 4,
        lacunarity: 0.5,
    }
    .wrap()
    .build()
}

fn main() {
    #[cfg(feature = "trace")]
    {
        use tracing_subscriber::layer::SubscriberExt;
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let node = create_node();

    println!("SIMD level: {}", node.get_simd_level());

    let mut noise = vec![0.0; (X_SIZE * Y_SIZE) as usize];

    let start = Instant::now();
    let min_max = node.gen_uniform_grid_2d(
        &mut noise,
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

    save(img, "safe.png");
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
