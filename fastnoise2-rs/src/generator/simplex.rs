use super::{Generator, GeneratorWrapper};
use crate::{safe::SafeNode, Node};

#[derive(Clone, Debug)]
pub struct Simplex {
    /// Feature Scale (effectively 1/frequency). Default: 1.0
    pub feature_scale: f32,
}

#[derive(Clone, Debug)]
pub struct SuperSimplex {
    /// Feature Scale (effectively 1/frequency). Default: 1.0
    pub feature_scale: f32,
}

impl Default for Simplex {
    fn default() -> Self {
        Self { feature_scale: 1.0 }
    }
}

impl Default for SuperSimplex {
    fn default() -> Self {
        Self { feature_scale: 1.0 }
    }
}

impl Generator for Simplex {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Simplex").unwrap();
        node.set("Feature Scale", self.feature_scale).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for SuperSimplex {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("SuperSimplex").unwrap();
        node.set("Feature Scale", self.feature_scale).unwrap();
        SafeNode(node.into()).into()
    }
}

/// Creates a Simplex noise generator with default Feature Scale of 1.0
pub fn simplex() -> GeneratorWrapper<Simplex> {
    Simplex::default().into()
}

/// Creates a SuperSimplex noise generator with default Feature Scale of 1.0
pub fn supersimplex() -> GeneratorWrapper<SuperSimplex> {
    SuperSimplex::default().into()
}

/// Creates a SuperSimplex noise generator with custom Feature Scale
pub fn supersimplex_scaled(feature_scale: f32) -> GeneratorWrapper<SuperSimplex> {
    SuperSimplex { feature_scale }.into()
}
