# Changelog

All notable changes to fastnoise2-rs will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Updated FastNoise2 C++ submodule from `3728fde` to `8176c3f` (v1.1.1):
- C API: `fnGetSIMDLevel` renamed to `fnGetActiveFeatureSet`
- C API: new metadata introspection functions (descriptions, defaults, min/max, groups)
- C API: fixed `fnSetNodeLookup` and `fnSetHybridNodeLookup`, which now take the node pointer directly
- No new nodes, members or default values

### Added

- Typed node API generated from FastNoise2 metadata, in `fastnoise2::nodes` and `fastnoise2::prelude`:
  - A type per FastNoise2 node, with documented `with_*` builder methods and FastNoise2 default values
  - Nodes without inputs are created by functions (`perlin()`), nodes with inputs are chained from their first input (`perlin().fractal_f_bm()`, `GeneratorExt`)
  - Inputs only accepting domain warp nodes are checked at compile time (`DomainWarpSource`)
  - Operators `+`, `-`, `*`, `/`, `%` and negation, with constants on either side (`0.5 - &node`)
  - Every FastNoise2 member is available, including the ones missing before (e.g. cellular feature scale, domain warp seed offset and amplitude scaling), and hybrid members accept nodes (e.g. `Remap` bounds, `Gradient` offsets)
- `Node`: a built node, `Send + Sync`, cloning shares the same FastNoise2 node
- `Metadata` and `Member`: FastNoise2 metadata of every node type (`Metadata::all`, `Metadata::by_name`, `Node::metadata`), with names, descriptions, groups, member types, enum values, default values and the ranges the Node Editor clamps members to (`MemberRange`, not enforced by FastNoise2)
- `FeatureSet` and `with_max_feature_set`: creates nodes with at most a SIMD feature set on the current thread (lowered to `FeatureSet::detected`), returning `FastNoiseError::FeatureSetNotAvailable` for a feature set FastNoise2 is not compiled for instead of crashing
- `FastNoiseError::FeatureSetMismatch`: an input with another SIMD feature set than its node is rejected, FastNoise2 only asserts it and generating noise crashed
- `Generator` trait, implemented by `Node`, typed nodes and references to them
- `Hybrid` and `MemberValue` value types
- `NodeBuilder`: nodes by FastNoise2 name, rejecting a missing input (`FastNoiseError::MissingInput`) or an input not accepting a node (`FastNoiseError::InputNotAccepted`)
- `encode` feature, disabled by default: `Node::encode` encodes a node tree in the format of the FastNoise2 Node Editor, a node shared in the tree is encoded once
- `fastnoise2-codegen` crate (not published), generating `fastnoise2-rs/src/nodes`, with `--check` to verify it is up to date
- `nodes`, `node_builder`, `encode` and `metadata` examples
- `rust-version = "1.85"` and edition 2024
- fastnoise2-sys: FastSIMD is bundled as a Git submodule, building no longer needs network access
- fastnoise2-sys: a precompiled library in `FASTNOISE2_LIB_DIR` is rejected if it misses functions of the C header

### Changed

- **Breaking**: `Node` is now the safe, built node. Nodes by name use `NodeBuilder` instead of `Node::from_name` and `Node::set`
- **Breaking**: names are generated from FastNoise2 names (e.g. `fbm()` is `fractal_f_bm()`, `supersimplex()` is `super_simplex()`), and constructors with positional arguments are replaced by builder methods
- **Breaking**: default values come from FastNoise2 metadata, `DistanceToPoint` distance function defaults to `Euclidean` like in the Node Editor
- **Breaking**: `get_simd_level` renamed to `get_active_feature_set`, which returns a `FeatureSet` instead of a `u32`
- **Breaking**: `FastNoiseError` member errors name the node and member, `Set*Failed` variants are merged into `SetMemberFailed`
- Errors use FastNoise2 display names in metadata order (e.g. `'Feature Scale'` instead of `'featurescale'`), and invalid type errors include the member description from FastNoise2
- Generation functions panic on non-positive counts, grid sizes overflowing an `i32` and empty position arrays
- `safe_simple_terrain` example renamed `simple_terrain`
- fastnoise2-sys: cached bindings in `FASTNOISE2_BINDINGS_DIR` are stored per crate version and only reused with an identical C header
- fastnoise2-sys: WASM builds only need Emscripten in `PATH`, `EMSDK` is no longer required
- fastnoise2-sys: `FASTNOISE2_SOURCE_DIR` is used to generate bindings when `FASTNOISE2_LIB_DIR` is set
- fastnoise2-sys: bindgen 0.73, bindings are generated for edition 2024 (`unsafe extern "C"`)
- Example images are no longer included in the package

### Removed

- **Breaking**: `SafeNode`, `GeneratorWrapper`, the `generator` module and its types, replaced by `Node` and the generated `nodes` types
- `safe` and `manual` examples, replaced by `nodes` and `node_builder`

### Fixed

- Node lookup and hybrid node lookup members are set with the node handle, matching the fixed C API
- `gen_position_array_4d` did not check the `w_pos_array` length (out-of-bounds read)
- Generation functions crashed or hung on zero counts or empty arrays
- fastnoise2-sys: build script is rerun when FastNoise2 sources or the precompiled library change
- fastnoise2-sys: C++ standard library is linked on Emscripten, Android, iOS, tvOS, watchOS, visionOS, OpenBSD, NetBSD and windows-gnullvm
- Examples use feature scales and step sizes suited to the feature scale API

### Migration

```rust
// Old
let node: GeneratorWrapper<SafeNode> = perlin().fbm(0.5, 0.0, 3, 2.0).domain_scale(0.66).build();
let node = SafeNode::from_encoded_node_tree(encoded)?;
let mut node = Node::from_name("Perlin")?;
node.set("FeatureScale", 50.0)?;

// New
let node: Node = perlin().fractal_f_bm().with_octaves(3).domain_scale().with_scaling(0.66).build();
let node = Node::from_encoded_node_tree(encoded)?;
let node = NodeBuilder::new("Perlin")?.set("Feature Scale", 50.0)?.build()?;
```

## [0.4.0] - 2026-01-21

Updated FastNoise2 C++ submodule from `f8facba` to `3728fde`:
- NewFastSIMD integration with native WASM SIMD128 support
- Cellular lookup metadata fixes
- Optional clamping for Remap node output

### Added

- New generator types:
  - `PingPong<S, P>` - Standalone modifier that bounces values between -1 and 1
    - Creates flowing contour-like patterns
    - `P` is hybrid: can be constant or generator-driven strength
  - `Abs<S>` - Returns absolute value of source output
    - Useful for creating ridge-like effects from bipolar noise
  - `SignedSquareRoot<S>` - Square root that preserves sign
    - Compresses high values more than low values
  - `DomainRotatePlane<S>` - Optimized rotation for reducing axis-aligned artifacts
    - Faster than general `DomainRotate` for common use cases
  - `DomainWarpSimplex<S, A>` - Simplex-based domain warping
    - Smoother than gradient-based warping
  - `DomainWarpSuperSimplex<S, A>` - Higher quality simplex domain warping
    - Better quality but slower than `DomainWarpSimplex`
  - `Modulus<Lhs, Rhs>` - Floating-point modulo operator
    - Both sides are hybrid: can be constants or generators

- New enums:
  - `PlaneRotationType`
    - `ImproveXYPlanes` - optimizes for top-down 2D views
    - `ImproveXZPlanes` - optimizes for side-view terrain
  - `VectorizationScheme`
    - Controls how simplex domain warp calculates displacement vectors
    - `OrthogonalGradientMatrix` (default) vs `GradientOuterProduct`
  - `DistanceFunction::Minkowski`
    - Generalized distance metric controlled by `minkowski_p` parameter
    - p=1 gives Manhattan, p=2 gives Euclidean, fractional values give interesting shapes

- Generator parameter enhancements (all via builder methods for backward compatibility):
  - `Perlin` & `Value`:
    - Added `feature_scale`, `seed_offset`, `output_min`, `output_max` fields
    - Feature scale is effectively 1/frequency - higher values produce larger features
    - Builder methods: `.with_feature_scale()`, `.with_seed_offset()`, `.with_output_range()`
    - Defaults: feature_scale=100.0, seed_offset=0, output_range=(-1.0, 1.0)
  - `Simplex` & `SuperSimplex`:
    - Added `seed_offset`, `output_min`, `output_max` fields
    - Builder methods: `.with_feature_scale()`, `.with_seed_offset()`, `.with_output_range()`
    - Defaults: feature_scale=100.0, seed_offset=0, output_range=(-1.0, 1.0)
  - `White`:
    - Added `seed_offset`, `output_min`, `output_max` fields
    - Builder methods: `.with_seed_offset()`, `.with_output_range()`
  - `Checkerboard` & `SineWave`:
    - Added `output_min`, `output_max` fields
    - Builder methods: `.with_feature_scale()`, `.with_output_range()`
  - `DistanceToPoint`:
    - Made point coordinates Hybrid (can be f32 or Generator)
    - Added `minkowski_p` as Hybrid field (default 1.5)
    - Builder methods: `.with_point_x/y/z/w()`, `.with_minkowski_p()`
    - Added `.with_distance_function()` and `.with_point()` builder methods
  - `Gradient`:
    - Added builder methods for multipliers and offsets:
      - `.with_multiplier_x/y/z/w()`, `.with_multipliers()`
      - `.with_offset_x/y/z/w()`, `.with_offsets()`
  - `Remap`:
    - Added `clamp_output: bool` field
    - New constructor: `remap_clamped(from_min, from_max, to_min, to_max, clamp_output)`
    - Existing `remap()` defaults to no clamping (backwards compatible)
  - `Cellular` types (`CellularValue`, `CellularDistance`, `CellularLookup`):
    - Added `minkowski_p` and `size_jitter` hybrid parameters
    - `minkowski_p` - controls Minkowski distance function shape (1=Manhattan, 2=Euclidean)
    - `size_jitter` - randomizes cell sizes for more organic look

- Native WASM SIMD128 support via NewFastSIMD integration

### Changed

- Renamed types:
  - `OpenSimplex2` → `SuperSimplex`
    - K.jpg's improved simplex variant
    - The "Open" prefix was dropped upstream for clarity
  - `opensimplex2()` → `supersimplex()`
    - Function rename to match type
  - `PositionOutput` → `Gradient`
    - Better describes what it does: outputs linear gradient based on position
  - `SquareRoot` → `SignedSquareRoot`
    - Clarifies that it preserves sign (sqrt of abs value, then restores sign)

- Renamed fields (aligning with upstream C++ API):
  - `DomainScale`: `scale` → `scaling`
  - `DomainAxisScale`: `scale_x/y/z/w` → `scaling_x/y/z/w`
  - `Terrace`: `multiplier` → `step_count`
    - Clarifies meaning: higher value = more steps = smaller terraces
  - `CellularValue/Distance/Lookup`: `jitter_modifier` → `grid_jitter`
    - More descriptive: controls how much cells deviate from grid
  - `DomainWarpGradient`: `warp_frequency` → `feature_scale`
    - Consistent with other noise types

- Struct signature changes:
  - `Simplex`:
    - Was unit struct, now has `feature_scale`, `seed_offset`, `output_min`, `output_max`
    - Feature scale is effectively 1/frequency - higher = larger features
  - `SuperSimplex`:
    - Now has `feature_scale`, `seed_offset`, `output_min`, `output_max` fields
  - `Terrace<S>` → `Terrace<S, Sm>`:
    - Smoothness is now Hybrid (can be f32 or Generator)
    - Enables spatially-varying smoothness: sharp terraces in some areas, smooth in others
    - Backwards compatible: `terrace(4.0, 0.5)` still works
  - `CellularValue<J>` → `CellularValue<J, M, S>`
  - `CellularDistance<J>` → `CellularDistance<J, M, S>`
  - `CellularLookup<L, J>` → `CellularLookup<L, J, M, S>`

### Removed

- `FractalPingPong`
  - Ping-pong is now a standalone modifier `PingPong<S, P>` instead of a fractal type
  - More flexible: can be applied to any generator, not just fractals
- `OpenSimplex2S`
  - Was the "smooth" variant, now consolidated into `SuperSimplex`

### Fixed

- Cellular lookup metadata parent class inheritance

---

## Migration Guide

### Type Renames

```rust
// Old
OpenSimplex2
opensimplex2()
PositionOutput { ... }
SquareRoot { source }

// New
SuperSimplex { feature_scale: 1.0, ..Default::default() }
supersimplex()
Gradient { ... }
SignedSquareRoot { source }
```

### Field Renames

```rust
// Old
DomainScale { source, scale: 2.0 }
Terrace { source, multiplier: 4.0, smoothness: 0.5 }
CellularValue { jitter_modifier: 1.0, ... }

// New
DomainScale { source, scaling: 2.0 }
Terrace { source, step_count: 4.0, smoothness: 0.5 }
CellularValue { grid_jitter: 1.0, minkowski_p: 2.0, size_jitter: 0.0, ... }
```

### Struct Initialization

```rust
// Old - unit struct
Simplex

// New - use Default for optional fields
Simplex::default()
simplex()  // helper function still works
```

### Constructor Changes

```rust
// Old
gradient([0.0, 3.0, 0.0, 0.0], [0.0; 4])
distance_to_point(DistanceFunction::Euclidean, [0.0; 4])

// New
gradient()
    .with_multipliers([0.0, 3.0, 0.0, 0.0])
    .with_offsets([0.0; 4])

distance_to_point()
    .with_distance_function(DistanceFunction::Euclidean)
    .with_point([0.0; 4])
```

### Builder Pattern Usage

```rust
// Simple (backward compatible)
perlin()
simplex()
white()

// With customization
perlin()
    .with_feature_scale(50.0)
    .with_seed_offset(42)
    .with_output_range(0.0, 1.0)

simplex()
    .with_feature_scale(0.5)
    .with_output_range(-0.5, 0.5)

// Generator-driven parameters
distance_to_point()
    .with_distance_function(DistanceFunction::Euclidean)
    .with_point([0.0, 0.0, 0.0, 0.0])
    .with_point_x(simplex())  // moving point!
    .with_minkowski_p(2.0)

gradient()
    .with_multipliers([0.0, 3.0, 0.0, 0.0])
    .with_offsets([0.0; 4])

terrace(4.0, simplex())  // spatially-varying smoothness
```

### FractalPingPong Migration

```rust
// Old - dedicated fractal type
FractalPingPong { source, octaves: 4, ... }

// New - composable modifier
fractal_fbm(perlin(), 4)
    .ping_pong(2.0)  // apply ping-pong as modifier
```

### Noise Generation Functions

Grids now take float offsets and per-axis step sizes instead of integer starts and a frequency.
Frequency moved to the nodes as `feature_scale` (default 100.0), so sampled coordinates are divided by it.

```rust
// Old
node.gen_uniform_grid_2d(&mut out, x_start, y_start, x_size, y_size, frequency, seed);
node.gen_tileable_2d(&mut out, x_size, y_size, frequency, seed);

// New, same sampled coordinates with the default feature scale of 100.0
let step = frequency * 100.0;
node.gen_uniform_grid_2d(&mut out, x_start as f32 * step, y_start as f32 * step, x_size, y_size, step, step, seed);
node.gen_tileable_2d(&mut out, x_size, y_size, step, step, seed);
```

The same applies to `gen_uniform_grid_3d` and `gen_uniform_grid_4d`.

### fastnoise2-sys

- `fnGenUniformGrid2D/3D/4D` and `fnGenTileable2D` signatures changed the same way
- `fnNewFromMetadata` and `fnNewFromEncodedNodeTree` take a max `FastSIMD::FeatureSet`, use `~0u` (`u32::MAX`) for auto-detection, `0` is no longer auto and fails

---

## Upstream Reference

FastNoise2 C++ version: `8176c3f9d8b631e6c998ed98a40bfbb1d0338a73`
