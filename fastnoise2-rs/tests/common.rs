use fastnoise2::SafeNode;

/// Helper to test a generator produces valid noise output
pub fn test_generator_produces_output(node: SafeNode) {
    let mut output = [0.0f32; 16];
    let min_max = node.gen_uniform_grid_2d(&mut output, 0.0, 0.0, 4, 4, 0.1, 0.1, 1337);
    // Check that we got valid output
    assert!(min_max.min.is_finite());
    assert!(min_max.max.is_finite());
}
