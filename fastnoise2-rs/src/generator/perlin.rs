use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper};

/// Perlin gradient noise.
/// Smooth gradient noise from N dimensional grid, developed by Ken Perlin in 1983.
#[derive(Clone, Debug)]
pub struct Perlin {
    /// Feature Scale (effectively 1/frequency). Default: 100.0
    pub feature_scale: f32,
    /// Offset applied to the seed. Default: 0
    pub seed_offset: i32,
    /// Minimum output value. Default: -1.0
    pub output_min: f32,
    /// Maximum output value. Default: 1.0
    pub output_max: f32,
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            feature_scale: 100.0,
            seed_offset: 0,
            output_min: -1.0,
            output_max: 1.0,
        }
    }
}

impl Generator for Perlin {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Perlin").unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        node.set("OutputMin", self.output_min).unwrap();
        node.set("OutputMax", self.output_max).unwrap();
        SafeNode(node.into()).into()
    }
}

/// Creates a Perlin noise generator with default parameters.
pub fn perlin() -> GeneratorWrapper<Perlin> {
    Perlin::default().into()
}

impl GeneratorWrapper<Perlin> {
    /// Sets the feature scale (effectively 1/frequency).
    pub fn with_feature_scale(mut self, scale: f32) -> Self {
        self.0.feature_scale = scale;
        self
    }

    /// Sets the seed offset for variation.
    pub fn with_seed_offset(mut self, offset: i32) -> Self {
        self.0.seed_offset = offset;
        self
    }

    /// Sets the output range.
    pub fn with_output_range(mut self, min: f32, max: f32) -> Self {
        self.0.output_min = min;
        self.0.output_max = max;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_perlin() {
        let node = perlin().build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_perlin_builder_methods() {
        // Test all builder methods chain correctly
        let node = perlin()
            .with_feature_scale(50.0)
            .with_seed_offset(42)
            .with_output_range(0.0, 1.0)
            .build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_param_perlin_feature_scale() {
        let node1 = perlin().with_feature_scale(10.0).build();
        let node2 = perlin().with_feature_scale(100.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "Perlin.FeatureScale");
    }

    #[test]
    fn test_param_perlin_seed_offset() {
        let node1 = perlin().with_seed_offset(0).build();
        let node2 = perlin().with_seed_offset(42).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "Perlin.SeedOffset");
    }

    #[test]
    fn test_param_perlin_output_range() {
        let node1 = perlin().with_output_range(-1.0, 1.0).build();
        let node2 = perlin().with_output_range(0.0, 100.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "Perlin.OutputMin/Max");
    }
}
