# FastNoise2 WASM Demo

Interactive browser demo using FastNoise2 compiled to WebAssembly.

## Why Emscripten?

FastNoise2 is a C++ library. `wasm32-unknown-unknown` targets pure Rust only—no C++ compiler, no libc, no stdlib.

Emscripten is required to:
- Compile FastNoise2's C++ code to WASM
- Provide libc++ runtime
- Enable WASM SIMD128 via FastSIMD's native support

## Quick Start

```bash
# Install Emscripten SDK (https://emscripten.org/docs/getting_started)
source ~/emsdk/emsdk_env.sh
rustup target add wasm32-unknown-emscripten

make build
make serve
# Open http://localhost:8080
```

## Files

- `src/main.rs` - Rust code exposing noise generation to JS
- `index.html` - Interactive demo with canvas visualization
- `Makefile` - Build commands
