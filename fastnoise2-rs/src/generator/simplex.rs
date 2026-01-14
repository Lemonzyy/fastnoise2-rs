use super::{Generator, GeneratorWrapper};
use crate::{safe::SafeNode, Node};

/// Simplex noise generator.
#[derive(Clone, Debug)]
pub struct Simplex {
    /// Feature Scale (effectively 1/frequency). Default: 1.0
    pub feature_scale: f32,
    /// Offset applied to the seed. Default: 0
    pub seed_offset: i32,
    /// Minimum output value. Default: -1.0
    pub output_min: f32,
    /// Maximum output value. Default: 1.0
    pub output_max: f32,
}

/// SuperSimplex noise generator (K.jpg's improved simplex variant).
#[derive(Clone, Debug)]
pub struct SuperSimplex {
    /// Feature Scale (effectively 1/frequency). Default: 1.0
    pub feature_scale: f32,
    /// Offset applied to the seed. Default: 0
    pub seed_offset: i32,
    /// Minimum output value. Default: -1.0
    pub output_min: f32,
    /// Maximum output value. Default: 1.0
    pub output_max: f32,
}

impl Default for Simplex {
    fn default() -> Self {
        Self {
            feature_scale: 1.0,
            seed_offset: 0,
            output_min: -1.0,
            output_max: 1.0,
        }
    }
}

impl Default for SuperSimplex {
    fn default() -> Self {
        Self {
            feature_scale: 1.0,
            seed_offset: 0,
            output_min: -1.0,
            output_max: 1.0,
        }
    }
}

impl Generator for Simplex {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Simplex").unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        node.set("OutputMin", self.output_min).unwrap();
        node.set("OutputMax", self.output_max).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for SuperSimplex {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("SuperSimplex").unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        node.set("OutputMin", self.output_min).unwrap();
        node.set("OutputMax", self.output_max).unwrap();
        SafeNode(node.into()).into()
    }
}

/// Creates a Simplex noise generator with default Feature Scale of 1.0
pub fn simplex() -> GeneratorWrapper<Simplex> {
    Simplex::default().into()
}

/// Creates a Simplex noise generator with custom Feature Scale
pub fn simplex_scaled(feature_scale: f32) -> GeneratorWrapper<Simplex> {
    Simplex { feature_scale, ..Default::default() }.into()
}

/// Creates a SuperSimplex noise generator with default Feature Scale of 1.0
pub fn supersimplex() -> GeneratorWrapper<SuperSimplex> {
    SuperSimplex::default().into()
}

/// Creates a SuperSimplex noise generator with custom Feature Scale
pub fn supersimplex_scaled(feature_scale: f32) -> GeneratorWrapper<SuperSimplex> {
    SuperSimplex { feature_scale, ..Default::default() }.into()
}

impl GeneratorWrapper<Simplex> {
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

impl GeneratorWrapper<SuperSimplex> {
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
