use fastnoise2::{FastNoiseError, prelude::*};

/// Asserts that a generator produces finite, varying noise.
fn assert_produces_noise(generator: impl Generator) {
    let mut output = [0.0f32; 64];
    let min_max =
        generator
            .build()
            .gen_uniform_grid_2d(&mut output, 0.0, 0.0, 8, 8, 10.0, 10.0, 1337);

    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
    assert!(min_max.min < min_max.max);
}

#[test]
fn test_complex_terrain() {
    let base = perlin().fractal_f_bm().with_octaves(6);
    let detail = simplex()
        .fractal_f_bm()
        .with_gain(0.6)
        .with_octaves(4)
        .with_lacunarity(2.5);
    let warped = base.domain_warp_gradient().with_warp_amplitude(20.0);
    let combined = warped.min_smooth().with_rhs(detail).with_smoothness(0.2);

    assert_produces_noise(
        combined
            .remap()
            .with_from_min(-1.0)
            .with_from_max(1.0)
            .with_to_min(0.0)
            .with_to_max(1.0),
    );
}

#[test]
fn test_ridged_mountains() {
    let mountains = perlin()
        .fractal_ridged()
        .with_weighted_strength(0.5)
        .with_octaves(5)
        .abs()
        .terrace()
        .with_step_count(8.0)
        .with_smoothness(0.3);

    assert_produces_noise(mountains);
}

#[test]
fn test_cellular_with_domain_warp() {
    let cells = cellular_value()
        .with_distance_function(DistanceFunction::Euclidean)
        .domain_warp_simplex()
        .with_warp_amplitude(30.0);

    assert_produces_noise(cells);
}

#[test]
fn test_all_distance_functions() {
    for distance_function in [
        DistanceFunction::Euclidean,
        DistanceFunction::EuclideanSquared,
        DistanceFunction::Manhattan,
        DistanceFunction::Hybrid,
        DistanceFunction::MaxAxis,
        DistanceFunction::Minkowski,
    ] {
        assert_produces_noise(cellular_value().with_distance_function(distance_function));
    }
}

#[test]
fn test_shared_node_and_operators() {
    let shared = perlin().build();
    let blend = (0.5 + &shared) * (1.0 - &shared) + shared.fractal_f_bm().with_gain(&shared);

    assert_produces_noise(blend);
}

/// A built domain warp node is accepted by the inputs taking only domain warp nodes, from a
/// typed domain warp node or once checked.
#[test]
fn test_domain_warp_source_node() {
    // Shared by both fractals, checked at compile time
    let warp = DomainWarpSourceNode::from(perlin().domain_warp_gradient());
    let progressive = warp.domain_warp_fractal_progressive().build();
    let independent = warp.domain_warp_fractal_independent().build();
    assert!(progressive.gen_single_2d(1.0, 2.0, 1337).is_finite());
    assert!(independent.gen_single_2d(1.0, 2.0, 1337).is_finite());

    let warp = perlin().domain_warp_gradient().build();
    let node = DomainWarpSourceNode::try_from(warp)
        .unwrap()
        .domain_warp_fractal_progressive()
        .build();
    assert_eq!(node.name(), "DomainWarpFractalProgressive");
    assert!(node.gen_single_2d(1.0, 2.0, 1337).is_finite());

    let error = DomainWarpSourceNode::try_from(perlin().build()).unwrap_err();
    assert!(matches!(
        error,
        FastNoiseError::NodeNotAccepted { ref node, ref expected, .. }
            if node == "Perlin" && expected.iter().any(|name| name == "DomainWarpGradient")
    ));
}
