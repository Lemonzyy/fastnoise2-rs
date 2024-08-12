# fastnoise2

![Crates.io License](https://img.shields.io/crates/l/fastnoise2)
![Crates.io Version](https://img.shields.io/crates/v/fastnoise2)
![docs.rs](https://docs.rs/fastnoise2/badge.svg)

`fastnoise2` provides an easy-to-use and mostly safe interface for the [FastNoise2](https://github.com/Auburn/FastNoise2) C++ library, which provides modular node graph-based noise generation using SIMD.

![NoiseTool Node Tree](https://raw.githubusercontent.com/Lemonzyy/fastnoise2-rs/main/examples/noisetool.png)

This crate acts as a wrapper around [fastnoise2-sys](https://crates.io/crates/fastnoise2-sys), the unsafe bindings for FastNoise2.

## Examples

Here is an example of a encoded node tree, exported by FastNoise2's NoiseTool.

```rust
use fastnoise2::FastNoise;

let (x_size, y_size) = (1000, 1000);
let encoded_node_tree = "EQACAAAAAAAgQBAAAAAAQBkAEwDD9Sg/DQAEAAAAAAAgQAkAAGZmJj8AAAAAPwEEAAAAAAAAAEBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM3MTD4AMzMzPwAAAAA/";
let node = FastNoise::from_encoded_node_tree(encoded_node_tree).unwrap();

// Allocate a buffer of enough size to hold all output data.
let mut noise_out = vec![0.0; (x_size * y_size) as usize];

// SAFETY: for now, it has to be unsafe, see the examples for more details.
let min_max = unsafe {
    node.gen_uniform_grid_2d_unchecked(
        &mut noise_out,
        -x_size / 2, // x offset
        -y_size / 2, // y offset
        x_size,
        y_size,
        0.01, // frequency
        1337, // seed
    )
};

// use `noise_out`!
```

You can also manually code a node tree using the metadata system of FastNoise2. See `examples` for more information.
