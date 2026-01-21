use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper};

/// Value noise generator.
/// Smooth gradient noise from N dimensional grid.
#[derive(Clone, Debug)]
pub struct Value {
    /// Feature Scale (effectively 1/frequency). Default: 100.0
    pub feature_scale: f32,
    /// Offset applied to the seed. Default: 0
    pub seed_offset: i32,
    /// Minimum output value. Default: -1.0
    pub output_min: f32,
    /// Maximum output value. Default: 1.0
    pub output_max: f32,
}

impl Default for Value {
    fn default() -> Self {
        Self {
            feature_scale: 100.0,
            seed_offset: 0,
            output_min: -1.0,
            output_max: 1.0,
        }
    }
}

impl Generator for Value {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Value").unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        node.set("OutputMin", self.output_min).unwrap();
        node.set("OutputMax", self.output_max).unwrap();
        SafeNode(node.into()).into()
    }
}

/// Creates a Value noise generator with default parameters.
pub fn value() -> GeneratorWrapper<Value> {
    Value::default().into()
}

impl GeneratorWrapper<Value> {
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
    fn test_value() {
        let node = value().build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_value_builder_methods() {
        let node = value()
            .with_feature_scale(50.0)
            .with_seed_offset(42)
            .with_output_range(0.0, 1.0)
            .build();
        test_generator_produces_output(node.0);
    }
}
