mod common;

use common::test_generator_produces_output;
use fastnoise2::generator::{prelude::*, DistanceFunction};

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
    let node = perlin()
        .ridged(0.5, 0.5, 5, 2.0)
        .abs()
        .terrace(8.0, 0.3)
        .build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_cellular_with_domain_warp() {
    let node = cellular_value(1.0, DistanceFunction::Euclidean, 0)
        .domain_warp_simplex(30.0, 100.0)
        .build();
    test_generator_produces_output(node.0);
}

#[test]
fn test_all_distance_functions() {
    for function in [
        DistanceFunction::Euclidean,
        DistanceFunction::EuclideanSquared,
        DistanceFunction::Manhattan,
        DistanceFunction::Hybrid,
        DistanceFunction::MaxAxis,
        DistanceFunction::Minkowski,
    ] {
        let node = cellular_value(1.0, function, 0).build();
        test_generator_produces_output(node.0);
    }
}
