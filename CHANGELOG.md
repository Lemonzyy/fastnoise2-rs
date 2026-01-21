# Changelog

All notable changes to fastnoise2-rs will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
    - Kept feature_scale default at 1.0 for backward compatibility
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

---

## Upstream Reference

FastNoise2 C++ version: `3728fde069704509fcf2973825b2d385348bf336`
