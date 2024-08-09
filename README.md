# fastnoise2-rs

Rust bindings and wrapper for the [FastNoise2](https://github.com/Auburn/FastNoise2) C++ library, which provides modular node graph-based noise generation using SIMD.

- [fastnoise2](./fastnoise2/README.md): A mostly safe wrapper around fastnoise2-sys. This is likely the crate you want to use.
- [fastnoise2-sys](./fastnoise2-sys/README.md): Unsafe Rust bindings for direct interaction with the FastNoise2 C++ library.
