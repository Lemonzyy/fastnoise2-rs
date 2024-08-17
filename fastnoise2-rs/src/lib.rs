//! # fastnoise2
//!
//! [![Crates.io License](https://img.shields.io/crates/l/fastnoise2)](https://github.com/Lemonzyy/fastnoise2-rs/blob/main/LICENSE)2)
//! [![Crates.io Version](https://img.shields.io/crates/v/fastnoise2)](https://crates.io/crates/fastnoise2)2)
//! [![docs.rs](https://docs.rs/fastnoise2/badge.svg)](https://docs.rs/fastnoise2/latest/fastnoise2/)
//!
//! fastnoise2 provides an easy-to-use and mostly safe interface for the [FastNoise2](https://github.com/Auburn/FastNoise2) C++ library, which provides modular node graph-based noise generation using SIMD.
//!
//! ![NoiseTool Node Tree](https://raw.githubusercontent.com/Lemonzyy/fastnoise2-rs/main/fastnoise2-rs/examples/noisetool.png)
//!
//! This crate acts as a wrapper around [fastnoise2-sys](https://crates.io/crates/fastnoise2-sys), the unsafe bindings for FastNoise2.
//!
//! ## Examples
//!
//! Here is an example of a encoded node tree, exported by FastNoise2's NoiseTool.
//!
//! ```rust
//! use fastnoise2::FastNoise;
//!
//! let (x_size, y_size) = (1000, 1000);
//! let encoded_node_tree = "EQACAAAAAAAgQBAAAAAAQBkAEwDD9Sg/DQAEAAAAAAAgQAkAAGZmJj8AAAAAPwEEAAAAAAAAAEBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM3MTD4AMzMzPwAAAAA/";
//! let node = FastNoise::from_encoded_node_tree(encoded_node_tree).unwrap();
//!
//! // Allocate a buffer of enough size to hold all output data.
//! let mut noise_out = vec![0.0; (x_size * y_size) as usize];
//!
//! // SAFETY: for now, it has to be unsafe, see the examples for more details.
//! let min_max = unsafe {
//!     node.gen_uniform_grid_2d_unchecked(
//!         &mut noise_out,
//!         -x_size / 2, // x offset
//!         -y_size / 2, // y offset
//!         x_size,
//!         y_size,
//!         0.01, // frequency
//!         1337, // seed
//!     )
//! };
//!
//! // use `noise_out`!
//! ```
//!
//! You can also manually code a node tree using the metadata system of FastNoise2. See `examples` for more information.
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
#![allow(clippy::too_many_arguments)]
mod error;
pub mod generator;
mod metadata;
mod typed;

pub use error::FastNoiseError;
pub use metadata::MemberType;
use metadata::{format_lookup, MemberValue, METADATA_NAME_LOOKUP, NODE_METADATA};
pub use typed::TypedFastNoise;

use fastnoise2_sys::*;
use std::ffi::CString;

/// Represents a node in the FastNoise2 C++ library.
///
/// This struct interfaces with the library, which uses metadata to dynamically manage node names and parameters.
/// For details on available metadata, see the [library documentation](https://github.com/Auburn/FastNoise2/wiki).
///
/// # Safety
/// Generating noise is unsafe. Refer to the specific method documentation for safety details.
#[derive(Debug)]
pub struct FastNoise {
    handle: *mut core::ffi::c_void,
    metadata_id: i32,
}

impl FastNoise {
    /// Creates a [`FastNoise`] instance using a metadata name.
    ///
    /// # Errors
    /// Returns an error if the metadata name is not found in the FastNoise2 metadata system.
    pub fn from_name(metadata_name: &str) -> Result<Self, FastNoiseError> {
        let metadata_name = format_lookup(metadata_name);
        let metadata_id = *METADATA_NAME_LOOKUP.get(&metadata_name).ok_or_else(|| {
            FastNoiseError::MetadataNameNotFound {
                expected: METADATA_NAME_LOOKUP.keys().cloned().collect(),
                found: metadata_name,
            }
        })?;
        let handle = unsafe { fnNewFromMetadata(metadata_id, 0) };
        Ok(Self {
            handle,
            metadata_id,
        })
    }

    /// Creates a `FastNoise` instance from an encoded node tree.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree is invalid or if creation fails.
    pub fn from_encoded_node_tree(encoded_node_tree: &str) -> Result<Self, FastNoiseError> {
        let cstring =
            CString::new(encoded_node_tree).map_err(FastNoiseError::CStringCreationFailed)?;
        let node_ptr = unsafe { fnNewFromEncodedNodeTree(cstring.as_ptr(), 0) };
        if node_ptr.is_null() {
            Err(FastNoiseError::NodeCreationFailed)
        } else {
            Ok(Self {
                handle: node_ptr,
                metadata_id: unsafe { fnGetMetadataID(node_ptr) },
            })
        }
    }

    pub fn get_simd_level(&self) -> u32 {
        unsafe { fnGetSIMDLevel(self.handle) }
    }

    /// Sets a value for a member.
    ///
    /// The `member_name` is looked up in the metadata, and the `value` is applied based on its type.
    /// The type of `value` must match the member's expected type as defined in the metadata.
    ///
    /// # Errors
    /// Returns an error if the member name is not found which includes a list of valid member names.
    /// Also returns an error if `value`'s type does not match the expected type for the member. The error provides the expected and actual types to assist in debugging.
    #[allow(private_bounds)]
    pub fn set(
        &mut self,
        member_name: &str,
        value: impl MemberValue,
    ) -> Result<(), FastNoiseError> {
        let metadata = &NODE_METADATA[self.metadata_id as usize];
        let member_name = format_lookup(member_name);
        let member = metadata.members.get(&member_name).ok_or_else(|| {
            FastNoiseError::MemberNameNotFound {
                expected: metadata.members.values().map(|m| m.name.clone()).collect(),
                found: member_name,
            }
        })?;

        value.apply(self, member)
    }

    /// # Safety
    /// - The caller must ensure that `noise_out` has enough space to hold `x_size * y_size` values.
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_uniform_grid_2d_unchecked(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        x_size: i32,
        y_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];

        fnGenUniformGrid2D(
            self.handle,
            noise_out.as_mut_ptr(),
            x_start,
            y_start,
            x_size,
            y_size,
            frequency,
            seed,
            min_max.as_mut_ptr(),
        );

        OutputMinMax::new(min_max)
    }

    /// # Safety
    /// - The caller must ensure that `noise_out` has enough space to hold `x_size * y_size * z_size` values.
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_uniform_grid_3d_unchecked(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        z_start: i32,
        x_size: i32,
        y_size: i32,
        z_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];

        fnGenUniformGrid3D(
            self.handle,
            noise_out.as_mut_ptr(),
            x_start,
            y_start,
            z_start,
            x_size,
            y_size,
            z_size,
            frequency,
            seed,
            min_max.as_mut_ptr(),
        );

        OutputMinMax::new(min_max)
    }

    /// # Safety
    /// - The caller must ensure that `noise_out` has enough space to hold `x_size * y_size * z_size * w_size` values.
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_uniform_grid_4d_unchecked(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        z_start: i32,
        w_start: i32,
        x_size: i32,
        y_size: i32,
        z_size: i32,
        w_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];

        fnGenUniformGrid4D(
            self.handle,
            noise_out.as_mut_ptr(),
            x_start,
            y_start,
            z_start,
            w_start,
            x_size,
            y_size,
            z_size,
            w_size,
            frequency,
            seed,
            min_max.as_mut_ptr(),
        );

        OutputMinMax::new(min_max)
    }

    /// # Safety
    /// - The caller must ensure that `noise_out`, `x_pos_array`, and `y_pos_array` all have the same length.
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_position_array_2d_unchecked(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];

        fnGenPositionArray2D(
            self.handle,
            noise_out.as_mut_ptr(),
            x_pos_array.len() as i32,
            x_pos_array.as_ptr(),
            y_pos_array.as_ptr(),
            x_offset,
            y_offset,
            seed,
            min_max.as_mut_ptr(),
        );

        OutputMinMax::new(min_max)
    }

    /// # Safety
    /// - The caller must ensure that `noise_out`, `x_pos_array`, `y_pos_array`, and `z_pos_array` all have the same length.
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_position_array_3d_unchecked(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        z_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];

        fnGenPositionArray3D(
            self.handle,
            noise_out.as_mut_ptr(),
            x_pos_array.len() as i32,
            x_pos_array.as_ptr(),
            y_pos_array.as_ptr(),
            z_pos_array.as_ptr(),
            x_offset,
            y_offset,
            z_offset,
            seed,
            min_max.as_mut_ptr(),
        );

        OutputMinMax::new(min_max)
    }

    /// # Safety
    /// - The caller must ensure that `noise_out`, `x_pos_array`, `y_pos_array`, `z_pos_array`, and `w_pos_array` all have the same length.
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_position_array_4d_unchecked(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        z_pos_array: &[f32],
        w_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        w_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];

        fnGenPositionArray4D(
            self.handle,
            noise_out.as_mut_ptr(),
            x_pos_array.len() as i32,
            x_pos_array.as_ptr(),
            y_pos_array.as_ptr(),
            z_pos_array.as_ptr(),
            w_pos_array.as_ptr(),
            x_offset,
            y_offset,
            z_offset,
            w_offset,
            seed,
            min_max.as_mut_ptr(),
        );

        OutputMinMax::new(min_max)
    }

    /// # Safety
    /// - The caller must ensure that `noise_out` has enough space to hold `x_size * y_size` values.
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_tileable_2d_unchecked(
        &self,
        noise_out: &mut [f32],
        x_size: i32,
        y_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];

        fnGenTileable2D(
            self.handle,
            noise_out.as_mut_ptr(),
            x_size,
            y_size,
            frequency,
            seed,
            min_max.as_mut_ptr(),
        );

        OutputMinMax::new(min_max)
    }

    /// # Safety
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_single_2d_unchecked(&self, x: f32, y: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle2D(self.handle, x, y, seed) }
    }

    /// # Safety
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_single_3d_unchecked(&self, x: f32, y: f32, z: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle3D(self.handle, x, y, z, seed) }
    }

    /// # Safety
    /// - The internal state of the noise generator must be correctly configured before calling this method.
    pub unsafe fn gen_single_4d_unchecked(&self, x: f32, y: f32, z: f32, w: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle4D(self.handle, x, y, z, w, seed) }
    }
}

impl Drop for FastNoise {
    fn drop(&mut self) {
        unsafe { fnDeleteNodeRef(self.handle) };
    }
}

/// Holds the minimum and maximum values from noise generation.
///
/// Used to represent the range of values produced by noise functions.
#[derive(Debug)]
pub struct OutputMinMax {
    pub min: f32,
    pub max: f32,
}

impl OutputMinMax {
    fn new([min, max]: [f32; 2]) -> Self {
        Self { min, max }
    }
}
