//! # fastnoise2-sys
//!
//! fastnoise2-sys provides unsafe Rust bindings for [FastNoise2](https://github.com/Auburn/FastNoise2), a C++ library for modular node graph-based noise generation with SIMD.
//! For a higher-level, mostly safe API, consider using [fastnoise2](https://crates.io/crates/fastnoise2).
//!
//! These bindings are automatically generated using [bindgen](https://crates.io/crates/bindgen).
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
//! - The `FASTNOISE2_SOURCE_DIR` environment variable is generally not needed as fastnoise2-sys includes the FastNoise2 source code as a Git submodule. If you need to use a different source directory, set `FASTNOISE2_SOURCE_DIR` to point to the root of the FastNoise2 source code.
//!
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
