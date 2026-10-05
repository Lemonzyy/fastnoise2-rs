//! # fastnoise2
//!
//! [![Crates.io License](https://img.shields.io/crates/l/fastnoise2)](https://github.com/Lemonzyy/fastnoise2-rs/blob/main/LICENSE)
//! [![Crates.io Version](https://img.shields.io/crates/v/fastnoise2)](https://crates.io/crates/fastnoise2)
//! [![docs.rs](https://docs.rs/fastnoise2/badge.svg)](https://docs.rs/fastnoise2/latest/fastnoise2/)
//!
//! fastnoise2 provides an easy-to-use and mostly safe interface for the [FastNoise2](https://github.com/Auburn/FastNoise2) C++ library, which provides modular node graph-based noise generation using SIMD.
//!
//! ![Node Editor Node Tree](https://raw.githubusercontent.com/Lemonzyy/fastnoise2-rs/main/fastnoise2-rs/examples/nodeeditor.png)
//! ![Node Editor Node Tree Output](https://raw.githubusercontent.com/Lemonzyy/fastnoise2-rs/main/fastnoise2-rs/examples/nodeeditor_output.bmp)
//!
//! This crate acts as a wrapper around [fastnoise2-sys](https://crates.io/crates/fastnoise2-sys), the unsafe bindings for FastNoise2.
//!
//! ## Examples
//!
//! Here is an example of a encoded node tree, exported by FastNoise2's Node Editor.
//!
//! ```rust
//! use fastnoise2::node::Node;
//!
//! let (x_count, y_count) = (1000, 1000);
//! let step_size = 3.0;
//! let encoded_node_tree = "E@BBZEG@BD8JFgIECArXIzwECiQIw/UoPwkuAAE@BJDQAH@BC@AIEAJBw@ABZEED0KV78YZmZmPwQDmpkZPwsAAIA/HAMAAHBCBA==";
//! let node = Node::from_encoded_node_tree(encoded_node_tree).unwrap();
//!
//! // Allocate a buffer of enough size to hold all output data.
//! let mut noise_out = vec![0.0; (x_count * y_count) as usize];
//!
//! let min_max = node.gen_uniform_grid_2d(
//!     &mut noise_out,
//!     -x_count as f32 / 2.0 * step_size, // x_offset
//!     -y_count as f32 / 2.0 * step_size, // y_offset
//!     x_count,                            // x_count
//!     y_count,                            // y_count
//!     step_size,                          // x_step_size
//!     step_size,                          // y_step_size
//!     1337,                               // seed
//! );
//!
//! // use `noise_out`!
//! ```
//!
//! You can also build node trees with the typed nodes of [`nodes`], or by name with [`node::NodeBuilder`].
//!
//! Take a look at [examples](https://github.com/Lemonzyy/fastnoise2-rs/tree/main/fastnoise2-rs/examples) to find out more.
//!
//! ## Setup
//!
//! fastnoise2-sys, the underlying bindings for fastnoise2, uses a build script that follows a specific order of preference for compiling and/or linking the FastNoise2 library:
//!
//! 1. Building from source, if the `build-from-source` feature is enabled.
//! 2. If the `FASTNOISE2_LIB_DIR` environment variable is set to `/path/to/lib/`, that path will be searched for static `FastNoise` library.
//! 3. If not set, it falls back to building from source.
//!
//! ## Building from Source
//!
//! To build FastNoise2 from source using fastnoise2-sys, ensure you have:
//!
//! - [CMake](https://cmake.org/)
//! - a C++17 compiler
//!
//! ## Notes
//!
//! - If you prefer not to build from source, precompiled binaries are available for download from the [FastNoise2 Releases](https://github.com/Auburn/FastNoise2/releases).
//! - For a web-based Node Editor experience, check out the [official Web WASM Node Editor](https://auburn.github.io/fastnoise2nodeeditor/).
//! - For desktop platforms, you can download compiled Node Editor binaries from the [FastNoise2 Releases](https://github.com/Auburn/FastNoise2/releases/latest).
//! - The `FASTNOISE2_SOURCE_DIR` environment variable is generally not needed as fastnoise2-sys includes the FastNoise2 source code as a Git submodule. If you need to use a different source directory, set `FASTNOISE2_SOURCE_DIR` to point to the root of the FastNoise2 source code. FastSIMD, the FastNoise2 dependency, is also included as a Git submodule and used when `FASTNOISE2_SOURCE_DIR` is not set, so building does not need network access.
//!
#![allow(clippy::too_many_arguments)]
mod error;
mod metadata;
pub mod node;
#[rustfmt::skip]
pub mod nodes;

pub use error::FastNoiseError;
pub use metadata::MemberType;

/// Holds the minimum and maximum values from noise generation.
///
/// Used to represent the range of values produced by noise functions.
#[derive(Debug)]
pub struct OutputMinMax {
    pub min: f32,
    pub max: f32,
}

impl OutputMinMax {
    pub(crate) fn new([min, max]: [f32; 2]) -> Self {
        Self { min, max }
    }
}
