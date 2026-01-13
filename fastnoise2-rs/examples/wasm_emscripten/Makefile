# FastNoise2 WASM Demo - Emscripten Build
#
# Requires:
#   - Emscripten SDK (source ~/emsdk/emsdk_env.sh)
#   - rustup target add wasm32-unknown-emscripten
#
# Usage:
#   make build   - Build the WASM module
#   make serve   - Start a local web server
#   make clean   - Clean build artifacts

.PHONY: build serve clean

# Emscripten compiler flags for linking
EMCC_CFLAGS = \
	-s MODULARIZE=1 \
	-s EXPORT_NAME=createFastNoise2Module \
	-s EXPORTED_FUNCTIONS=_generate_noise,_malloc,_free,_main \
	-s EXPORTED_RUNTIME_METHODS=HEAPU8 \
	-s ALLOW_MEMORY_GROWTH=1 \
	-s ERROR_ON_UNDEFINED_SYMBOLS=0 \
	-O3

build:
	@echo "Building FastNoise2 WASM with Emscripten target..."
	@echo "Make sure you have sourced emsdk_env.sh first!"
	EMCC_CFLAGS="$(EMCC_CFLAGS)" cargo build --release --target wasm32-unknown-emscripten
	@echo ""
	@echo "Copying output files to ./dist/"
	@mkdir -p dist
	@cp target/wasm32-unknown-emscripten/release/fastnoise2_wasm.js dist/
	@cp target/wasm32-unknown-emscripten/release/fastnoise2_wasm.wasm dist/
	@echo "Build complete!"
	@echo ""
	@ls -lh dist/

serve:
	@echo "Starting web server at http://localhost:8080"
	@echo "Open index.html in your browser"
	python3 -m http.server 8080

clean:
	cargo clean
	rm -rf dist/
