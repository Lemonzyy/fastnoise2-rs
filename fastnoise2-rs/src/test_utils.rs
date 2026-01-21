use crate::safe::SafeNode;

/// Helper to test a generator produces valid noise output
pub fn test_generator_produces_output(node: SafeNode) {
    let mut output = [0.0f32; 16];
    let min_max = node.gen_uniform_grid_2d(&mut output, 0.0, 0.0, 4, 4, 0.1, 0.1, 1337);
    // Check that we got valid output
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
}

/// Helper to generate output at a fixed position for comparison
pub fn generate_output(node: &SafeNode) -> [f32; 64] {
    let mut output = [0.0f32; 64];
    node.gen_uniform_grid_2d(&mut output, 0.0, 0.0, 8, 8, 0.05, 0.05, 1337);
    output
}

/// Helper to generate 3D output for parameters that only affect 3D
pub fn generate_output_3d(node: &SafeNode) -> [f32; 64] {
    let mut output = [0.0f32; 64];
    node.gen_uniform_grid_3d(&mut output, 0.0, 0.0, 0.0, 4, 4, 4, 0.1, 0.1, 0.1, 1337);
    output
}

/// Helper to assert two outputs are different
pub fn assert_outputs_differ(output1: &[f32], output2: &[f32], param_name: &str) {
    let differs = output1
        .iter()
        .zip(output2.iter())
        .any(|(a, b)| (a - b).abs() > 1e-6);
    assert!(
        differs,
        "Parameter '{}' did not affect output - check test conditions",
        param_name
    );
}
