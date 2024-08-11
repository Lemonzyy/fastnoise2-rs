#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

#[cfg(feature = "build-from-source")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "build-from-source"))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs"));
