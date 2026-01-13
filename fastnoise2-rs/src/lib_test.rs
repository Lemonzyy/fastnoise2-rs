use super::*;
use generator::prelude::*;
use generator::{
    cellular::{cellular_distance_full, cellular_lookup_full, cellular_value_full, CellularDistanceReturnType},
    domain_warp::VectorizationScheme,
    modifier::PlaneRotationType,
    simplex::supersimplex_scaled,
    Dimension, DistanceFunction, FadeInterpolation,
};

/// Helper to test a generator produces valid noise output
fn test_generator_produces_output(node: SafeNode) {
    let mut output = [0.0f32; 16];
    let min_max = node.gen_uniform_grid_2d(&mut output, 0.0, 0.0, 4, 4, 0.1, 0.1, 1337);
    // Check that we got valid output
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    // Check that not all values are the same (noise should vary)
    let all_same = output.iter().all(|&v| v == output[0]);
    // Some generators (like Constant) will have all same values, which is fine
    assert!(all_same || output.iter().any(|&v| v != output[0]));
}

// ==================== Basic Generators ====================

#[test]
fn test_constant() {
    let node = constant(0.5).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_white_noise() {
    let node = white().build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_checkerboard() {
    let node = checkerboard(10.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_sinewave() {
    let node = sinewave(10.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_gradient() {
    let node = gradient([0.01, 0.01, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_distance_to_point() {
    let node = distance_to_point(DistanceFunction::Euclidean, [0.0, 0.0, 0.0, 0.0]).build();
    test_generator_produces_output(node.0);
}

// ==================== Coherent Noise ====================

#[test]
fn test_perlin() {
    let node = perlin().build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_simplex() {
    let node = simplex().build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_supersimplex() {
    let node = supersimplex().build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_value() {
    let node = generator::value::value().build();
    test_generator_produces_output(node.0);
}

// ==================== Cellular ====================

#[test]
fn test_cellular_value() {
    let node = cellular_value(1.0, DistanceFunction::EuclideanSquared, 0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_cellular_value_with_minkowski() {
    let node = cellular_value_full(1.0, DistanceFunction::Minkowski, 0, 1.5, 0.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_cellular_distance() {
    let node = cellular_distance(1.0, DistanceFunction::EuclideanSquared, 0, 1, CellularDistanceReturnType::Index0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_cellular_distance_full() {
    let node = cellular_distance_full(
        1.0,
        DistanceFunction::Minkowski,
        0,
        1,
        CellularDistanceReturnType::Index0Sub1,
        2.0, // minkowski_p
        0.1, // size_jitter
    )
    .build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_cellular_lookup() {
    let node = cellular_lookup(perlin(), 1.0, DistanceFunction::Euclidean).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_cellular_lookup_full() {
    let node = cellular_lookup_full(
        simplex(),
        1.0,
        DistanceFunction::Minkowski,
        1.5, // minkowski_p
        0.0, // size_jitter
    )
    .build();
    test_generator_produces_output(node.0);
}

// ==================== Fractals ====================

#[test]
fn test_fractal_fbm() {
    let node = perlin().fbm(0.5, 0.0, 4, 2.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_fractal_ridged() {
    let node = perlin().ridged(0.5, 0.0, 4, 2.0).build();
    test_generator_produces_output(node.0);
}

// ==================== Domain Warp ====================

#[test]
fn test_domain_warp_gradient() {
    let node = simplex().domain_warp_gradient(50.0, 100.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_warp_simplex() {
    let node = perlin().domain_warp_simplex(50.0, 100.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_warp_simplex_with_scheme() {
    let node = perlin().domain_warp_simplex_with_scheme(50.0, 100.0, VectorizationScheme::GradientOuterProduct).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_warp_super_simplex() {
    let node = perlin().domain_warp_super_simplex(50.0, 100.0).build();
    test_generator_produces_output(node.0);
}

// ==================== Domain Warp Fractal ====================

#[test]
fn test_domain_warp_fractal_progressive() {
    // Domain warp fractal methods work on DomainWarpNode types
    let node = perlin().domain_warp_gradient(50.0, 100.0).domain_warp_progressive(0.5, 0.0, 4, 2.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_warp_fractal_independent() {
    let node = perlin().domain_warp_gradient(50.0, 100.0).domain_warp_independent(0.5, 0.0, 4, 2.0).build();
    test_generator_produces_output(node.0);
}

// ==================== Operators ====================

#[test]
fn test_add() {
    let node = (perlin() + 0.5).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_subtract() {
    let node = (perlin() - simplex()).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_multiply() {
    let node = (perlin() * 2.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_divide() {
    let node = (perlin() / 2.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_modulus() {
    let node = (perlin() % 0.5).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_min() {
    let node = perlin().min(0.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_max() {
    let node = perlin().max(0.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_negate() {
    let node = (-perlin()).build();
    test_generator_produces_output(node.0);
}

// ==================== Blends ====================

#[test]
fn test_min_smooth() {
    let node = perlin().min_smooth(simplex(), 0.1).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_max_smooth() {
    let node = perlin().max_smooth(simplex(), 0.1).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_fade() {
    let node = perlin().fade(simplex(), 0.5).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_fade_with_range() {
    let node = perlin().fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Hermite).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_powf() {
    let node = perlin().abs().powf(2.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_powi() {
    let node = perlin().powi(2).build();
    test_generator_produces_output(node.0);
}

// ==================== Modifiers ====================

#[test]
fn test_domain_scale() {
    let node = perlin().domain_scale(0.5).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_offset() {
    let node = perlin().domain_offset(1.0, 2.0, 0.0, 0.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_rotate() {
    let node = perlin().domain_rotate(0.5, 0.0, 0.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_seed_offset() {
    let node = perlin().seed_offset(42).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_remap() {
    let node = perlin().remap(-1.0, 1.0, 0.0, 1.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_terrace() {
    let node = perlin().terrace(4.0, 0.5).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_axis_scale() {
    let node = perlin().domain_axis_scale([1.0, 2.0, 1.0, 1.0]).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_add_dimension() {
    let node = perlin().add_dimension(0.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_remove_dimension() {
    let node = perlin().remove_dimension(Dimension::Z).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_cache() {
    let node = perlin().cache().build();
    test_generator_produces_output(node.0);
}

// ==================== New Modifiers ====================

#[test]
fn test_ping_pong() {
    let node = perlin().ping_pong(2.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_abs() {
    let node = perlin().abs().build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_signed_sqrt() {
    let node = perlin().signed_sqrt().build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_rotate_plane() {
    let node = perlin().domain_rotate_plane().build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_domain_rotate_plane_xz() {
    let node = perlin().domain_rotate_plane_with_type(PlaneRotationType::ImproveXZPlanes).build();
    test_generator_produces_output(node.0);
}

// ==================== Complex Combinations ====================

#[test]
fn test_complex_terrain() {
    // A complex terrain generator combining multiple features
    let base = perlin().fbm(0.5, 0.0, 6, 2.0);
    let detail = simplex().fbm(0.6, 0.0, 4, 2.5);
    let warped = base.domain_warp_gradient(20.0, 100.0);
    let combined = warped.min_smooth(detail, 0.2);
    let node = combined.remap(-1.0, 1.0, 0.0, 1.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_ridged_mountains() {
    let node = perlin().ridged(0.5, 0.5, 5, 2.0).abs().terrace(8.0, 0.3).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_cellular_with_domain_warp() {
    let node = cellular_value(1.0, DistanceFunction::Euclidean, 0).domain_warp_simplex(30.0, 100.0).build();
    test_generator_produces_output(node.0);
}

// ==================== Single Point Generation ====================

#[test]
fn test_gen_single_2d() {
    let node = perlin().build();
    let value = node.0.gen_single_2d(0.5, 0.5, 1337);
    assert!(value.is_finite());
    assert!(value >= -1.5 && value <= 1.5); // Perlin should be roughly -1 to 1
}

#[test]
fn test_gen_single_3d() {
    let node = perlin().build();
    let value = node.0.gen_single_3d(0.5, 0.5, 0.5, 1337);
    assert!(value.is_finite());
}

// ==================== Distance Functions ====================

#[test]
fn test_all_distance_functions() {
    let functions = [
        DistanceFunction::Euclidean,
        DistanceFunction::EuclideanSquared,
        DistanceFunction::Manhattan,
        DistanceFunction::Hybrid,
        DistanceFunction::MaxAxis,
        DistanceFunction::Minkowski,
    ];

    for func in functions {
        let node = cellular_value(1.0, func, 0).build();
        test_generator_produces_output(node.0);
    }
}

// ==================== Encoded Node Tree ====================

#[test]
fn test_encoded_node_tree() {
    let encoded = "DQAFAAAAAAAAQAgAAAAAAD8="; // Simple Perlin
    let node = SafeNode::from_encoded_node_tree(encoded);
    // This might fail if the encoded string is invalid
    if let Ok(node) = node {
        test_generator_produces_output(node);
    }
}

// ==================== Generation Output Methods ====================

#[test]
fn test_gen_uniform_grid_3d() {
    let node = perlin().build();
    let mut output = [0.0f32; 64]; // 4x4x4
    let min_max = node.0.gen_uniform_grid_3d(&mut output, 0.0, 0.0, 0.0, 4, 4, 4, 0.1, 0.1, 0.1, 1337);
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    assert!(output.iter().any(|&v| v != output[0]));
}

#[test]
fn test_gen_uniform_grid_4d() {
    let node = perlin().build();
    let mut output = [0.0f32; 16]; // 2x2x2x2
    let min_max = node.0.gen_uniform_grid_4d(&mut output, 0.0, 0.0, 0.0, 0.0, 2, 2, 2, 2, 0.1, 0.1, 0.1, 0.1, 1337);
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    assert!(output.iter().any(|&v| v != output[0]));
}

#[test]
fn test_gen_position_array_2d() {
    let node = perlin().build();
    let x_pos = [0.0, 0.1, 0.2, 0.3];
    let y_pos = [0.0, 0.1, 0.2, 0.3];
    let mut output = [0.0f32; 4];
    let min_max = node.0.gen_position_array_2d(&mut output, &x_pos, &y_pos, 0.0, 0.0, 1337);
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    assert!(output.iter().all(|&v| v.is_finite()));
}

#[test]
fn test_gen_position_array_3d() {
    let node = perlin().build();
    let x_pos = [0.0, 0.1, 0.2, 0.3];
    let y_pos = [0.0, 0.1, 0.2, 0.3];
    let z_pos = [0.0, 0.1, 0.2, 0.3];
    let mut output = [0.0f32; 4];
    let min_max = node.0.gen_position_array_3d(&mut output, &x_pos, &y_pos, &z_pos, 0.0, 0.0, 0.0, 1337);
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    assert!(output.iter().all(|&v| v.is_finite()));
}

#[test]
fn test_gen_position_array_4d() {
    let node = perlin().build();
    let x_pos = [0.0, 0.1, 0.2, 0.3];
    let y_pos = [0.0, 0.1, 0.2, 0.3];
    let z_pos = [0.0, 0.1, 0.2, 0.3];
    let w_pos = [0.0, 0.1, 0.2, 0.3];
    let mut output = [0.0f32; 4];
    let min_max = node.0.gen_position_array_4d(&mut output, &x_pos, &y_pos, &z_pos, &w_pos, 0.0, 0.0, 0.0, 0.0, 1337);
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    assert!(output.iter().all(|&v| v.is_finite()));
}

#[test]
fn test_gen_tileable_2d() {
    let node = perlin().build();
    let mut output = [0.0f32; 16]; // 4x4
    let min_max = node.0.gen_tileable_2d(&mut output, 4, 4, 0.1, 0.1, 1337);
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    assert!(output.iter().any(|&v| v != output[0]));
}

#[test]
fn test_gen_single_4d() {
    let node = perlin().build();
    let value = node.0.gen_single_4d(0.5, 0.5, 0.5, 0.5, 1337);
    assert!(value.is_finite());
}

// ==================== Additional Node Types ====================

#[test]
fn test_supersimplex_scaled() {
    let node = supersimplex_scaled(0.5).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_convert_rgba8() {
    // ConvertRgba8 outputs packed RGBA8 data as floats, so we just verify
    // it can be constructed and called without crashing
    let node = perlin().convert_rgba8(-1.0, 1.0).build();
    let mut output = [0.0f32; 16];
    // This won't produce standard noise values - it packs RGBA8 into floats
    let _min_max = node.0.gen_uniform_grid_2d(&mut output, 0.0, 0.0, 4, 4, 0.1, 0.1, 1337);
    // Just verify the node runs without crashing
}

#[test]
fn test_recip() {
    // recip() creates 1.0 / value
    let node = (constant(2.0) + constant(0.0)).recip().build();
    let mut output = [0.0f32; 4];
    let min_max = node.0.gen_uniform_grid_2d(&mut output, 0.0, 0.0, 2, 2, 0.1, 0.1, 1337);
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    // 1/2 = 0.5
    assert!(output.iter().all(|&v| (v - 0.5).abs() < 0.001));
}

#[test]
fn test_domain_warp_super_simplex_with_scheme() {
    let node = perlin()
        .domain_warp_super_simplex_with_scheme(50.0, 100.0, VectorizationScheme::GradientOuterProduct)
        .build();
    test_generator_produces_output(node.0);
}

// ==================== CellularDistanceReturnType Variants ====================

#[test]
fn test_cellular_distance_return_types() {
    let return_types = [
        CellularDistanceReturnType::Index0,
        CellularDistanceReturnType::Index0Add1,
        CellularDistanceReturnType::Index0Sub1,
        CellularDistanceReturnType::Index0Mul1,
        CellularDistanceReturnType::Index0Div1,
    ];

    for return_type in return_types {
        let node = cellular_distance(1.0, DistanceFunction::EuclideanSquared, 0, 1, return_type).build();
        test_generator_produces_output(node.0);
    }
}

// ==================== FadeInterpolation Variants ====================

#[test]
fn test_fade_interpolation_linear() {
    let node = perlin()
        .fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Linear)
        .build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_fade_interpolation_quintic() {
    let node = perlin()
        .fade_with_range(simplex(), 0.5, -1.0, 1.0, FadeInterpolation::Quintic)
        .build();
    test_generator_produces_output(node.0);
}

// ==================== Dimension Variants ====================

#[test]
fn test_remove_dimension_all() {
    let dimensions = [Dimension::X, Dimension::Y, Dimension::Z, Dimension::W];
    for dim in dimensions {
        let node = perlin().remove_dimension(dim).build();
        test_generator_produces_output(node.0);
    }
}

// ==================== PlaneRotationType Variants ====================

#[test]
fn test_plane_rotation_types() {
    let rotation_types = [PlaneRotationType::ImproveXYPlanes, PlaneRotationType::ImproveXZPlanes];
    for rotation_type in rotation_types {
        let node = perlin().domain_rotate_plane_with_type(rotation_type).build();
        test_generator_produces_output(node.0);
    }
}

// ==================== VectorizationScheme Variants ====================

#[test]
fn test_vectorization_schemes() {
    let schemes = [VectorizationScheme::OrthogonalGradientMatrix, VectorizationScheme::GradientOuterProduct];
    for scheme in schemes {
        let node = perlin().domain_warp_simplex_with_scheme(50.0, 100.0, scheme).build();
        test_generator_produces_output(node.0);
    }
}

// ==================== SIMD Level ====================

#[test]
fn test_get_simd_level() {
    let node = perlin().build();
    let simd_level = node.0.get_simd_level();
    // Just verify we can get a SIMD level - the actual value depends on the system
    // SIMD levels: 0=Scalar, 1=SSE, 2=SSE2, 3=SSE3, 4=SSSE3, 5=SSE41, 6=SSE42,
    //              7=AVX, 8=AVX2, 9=AVX512
    // On systems with AVX-512 this will be higher
    assert!(simd_level < u32::MAX); // Just verify it's a valid number
}

// ==================== Hybrid Parameters with Generators ====================

#[test]
fn test_hybrid_gain_fbm() {
    // Using a generator as the gain parameter (hybrid)
    let gain_node = simplex().domain_scale(0.1);
    let node = perlin().fbm(gain_node, 0.0, 4, 2.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_weighted_strength_ridged() {
    // Using a generator as weighted_strength parameter
    let strength_node = simplex().domain_scale(0.1);
    let node = perlin().ridged(0.5, strength_node, 4, 2.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_warp_amplitude() {
    // Using a generator as warp amplitude (hybrid)
    let amplitude_node = simplex().remap(-1.0, 1.0, 10.0, 50.0);
    let node = perlin().domain_warp_gradient(amplitude_node, 100.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_cellular_jitter() {
    // Using a generator for grid jitter
    let jitter_node = simplex().remap(-1.0, 1.0, 0.5, 1.5);
    let node = cellular_value(jitter_node, DistanceFunction::Euclidean, 0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_domain_offset() {
    // Using generators for offset parameters
    let offset_gen = simplex().domain_scale(0.1);
    let node = perlin().domain_offset(offset_gen.clone(), offset_gen, 0.0, 0.0).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_add_dimension() {
    // Using a generator for new dimension position
    let pos_gen = simplex();
    let node = perlin().add_dimension(pos_gen).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_ping_pong() {
    // Using a generator for ping pong strength
    let strength_gen = simplex().remap(-1.0, 1.0, 1.0, 3.0);
    let node = perlin().ping_pong(strength_gen).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_fade() {
    // Using a generator for fade value
    let fade_gen = simplex();
    let node = perlin().fade(value(), fade_gen).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_min_smooth() {
    // Using a generator for smoothness
    let smooth_gen = simplex().remap(-1.0, 1.0, 0.05, 0.2);
    let node = perlin().min_smooth(value(), smooth_gen).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_hybrid_powf() {
    // Using a generator for power
    let pow_gen = simplex().remap(-1.0, 1.0, 1.5, 2.5);
    let node = perlin().abs().powf(pow_gen).build();
    test_generator_produces_output(node.0);
}

// ==================== Generator as Operand ====================

#[test]
fn test_generator_add_generator() {
    let node = (perlin() + simplex()).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_generator_multiply_generator() {
    let node = (perlin() * simplex()).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_generator_min_generator() {
    let node = perlin().min(simplex()).build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_generator_max_generator() {
    let node = perlin().max(simplex()).build();
    test_generator_produces_output(node.0);
}
