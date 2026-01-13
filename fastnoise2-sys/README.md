# fastnoise2-sys

Low-level Rust FFI bindings for [FastNoise2](https://github.com/Auburn/FastNoise2), a high-performance noise generation library with SIMD support.

## Building

### Native Platforms (Linux, Windows, macOS)

Native builds use cmake and work out of the box:

```bash
cargo build
```

### WebAssembly (WASM)

WASM builds require the [Emscripten SDK](https://emscripten.org/):

#### 1. Install Emscripten

```bash
# Clone the emsdk repository
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk

# Install and activate Emscripten 3.1.68 or later
./emsdk install 3.1.68
./emsdk activate 3.1.68

# Add Emscripten to your environment
source ./emsdk_env.sh
```

**Note**: You need to run `source ./emsdk_env.sh` in each new terminal session before building for WASM.

#### 2. Build for WASM

```bash
cargo build --target wasm32-unknown-emscripten
```

The build script will automatically detect the Emscripten target and use Emscripten to compile FastNoise2.

#### Custom Emscripten Flags

You can pass custom flags to emcc via the `EMCC_CFLAGS` environment variable:

```bash
export EMCC_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0"
cargo build --target wasm32-unknown-emscripten
```

## Environment Variables

| Variable | Purpose | Required |
|----------|---------|----------|
| `EMSDK` | Path to Emscripten SDK | Yes (WASM only) |
| `EMCC_CFLAGS` | Custom flags for emcc | No |
| `FASTNOISE2_SOURCE_DIR` | Override FastNoise2 source path | No |
| `FASTNOISE2_LIB_DIR` | Use precompiled library | No |
| `FASTNOISE2_BINDINGS_DIR` | Cache directory for bindings | No |

## Troubleshooting

### Error: "EMSDK environment variable required for WASM builds"

You need to install and activate the Emscripten SDK (see instructions above), then source the environment:

```bash
source /path/to/emsdk/emsdk_env.sh
```

### Slow bindgen compilation

Set `FASTNOISE2_BINDINGS_DIR` to cache generated bindings:

```bash
export FASTNOISE2_BINDINGS_DIR=~/.cache/fastnoise2-bindings
cargo build --target wasm32-unknown-emscripten
```

## License

This crate provides bindings to FastNoise2, which is licensed under the MIT License.
