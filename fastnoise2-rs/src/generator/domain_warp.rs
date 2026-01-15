use std::fmt::Display;

use super::{Generator, GeneratorWrapper, Hybrid};
use crate::{safe::SafeNode, Node};

pub trait DomainWarpNode: Generator {}

/// Vectorization scheme for simplex-based domain warping.
#[derive(Clone, Debug, Default)]
pub enum VectorizationScheme {
    #[default]
    OrthogonalGradientMatrix,
    GradientOuterProduct,
}

impl Display for VectorizationScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorizationScheme::OrthogonalGradientMatrix => {
                f.write_str("Orthogonal Gradient Matrix")
            }
            VectorizationScheme::GradientOuterProduct => f.write_str("Gradient Outer Product"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DomainWarpGradient<S, A>
where
    S: Generator,
    A: Hybrid,
{
    pub source: S,
    pub warp_amplitude: A,
    pub feature_scale: f32,
}

/// Simplex-based domain warping.
#[derive(Clone, Debug)]
pub struct DomainWarpSimplex<S, A>
where
    S: Generator,
    A: Hybrid,
{
    pub source: S,
    pub warp_amplitude: A,
    pub feature_scale: f32,
    pub vectorization_scheme: VectorizationScheme,
}

/// Higher quality simplex-based domain warping.
#[derive(Clone, Debug)]
pub struct DomainWarpSuperSimplex<S, A>
where
    S: Generator,
    A: Hybrid,
{
    pub source: S,
    pub warp_amplitude: A,
    pub feature_scale: f32,
    pub vectorization_scheme: VectorizationScheme,
}

impl<S, A> Generator for DomainWarpGradient<S, A>
where
    S: Generator,
    A: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainWarpGradient").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("WarpAmplitude", self.warp_amplitude.clone())
            .unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, A> DomainWarpNode for DomainWarpGradient<S, A>
where
    S: Generator,
    A: Hybrid,
{
}

impl<S, A> Generator for DomainWarpSimplex<S, A>
where
    S: Generator,
    A: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainWarpSimplex").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("WarpAmplitude", self.warp_amplitude.clone())
            .unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        node.set(
            "VectorizationScheme",
            &*self.vectorization_scheme.to_string(),
        )
        .unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, A> DomainWarpNode for DomainWarpSimplex<S, A>
where
    S: Generator,
    A: Hybrid,
{
}

impl<S, A> Generator for DomainWarpSuperSimplex<S, A>
where
    S: Generator,
    A: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainWarpSuperSimplex").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("WarpAmplitude", self.warp_amplitude.clone())
            .unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        node.set(
            "VectorizationScheme",
            &*self.vectorization_scheme.to_string(),
        )
        .unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, A> DomainWarpNode for DomainWarpSuperSimplex<S, A>
where
    S: Generator,
    A: Hybrid,
{
}

impl<S> GeneratorWrapper<S>
where
    S: Generator,
{
    pub fn domain_warp_gradient<A>(
        self,
        warp_amplitude: A,
        feature_scale: f32,
    ) -> GeneratorWrapper<DomainWarpGradient<S, A>>
    where
        A: Hybrid,
    {
        DomainWarpGradient {
            source: self.0,
            warp_amplitude,
            feature_scale,
        }
        .into()
    }

    pub fn domain_warp_simplex<A>(
        self,
        warp_amplitude: A,
        feature_scale: f32,
    ) -> GeneratorWrapper<DomainWarpSimplex<S, A>>
    where
        A: Hybrid,
    {
        DomainWarpSimplex {
            source: self.0,
            warp_amplitude,
            feature_scale,
            vectorization_scheme: VectorizationScheme::default(),
        }
        .into()
    }

    pub fn domain_warp_simplex_with_scheme<A>(
        self,
        warp_amplitude: A,
        feature_scale: f32,
        vectorization_scheme: VectorizationScheme,
    ) -> GeneratorWrapper<DomainWarpSimplex<S, A>>
    where
        A: Hybrid,
    {
        DomainWarpSimplex {
            source: self.0,
            warp_amplitude,
            feature_scale,
            vectorization_scheme,
        }
        .into()
    }

    pub fn domain_warp_super_simplex<A>(
        self,
        warp_amplitude: A,
        feature_scale: f32,
    ) -> GeneratorWrapper<DomainWarpSuperSimplex<S, A>>
    where
        A: Hybrid,
    {
        DomainWarpSuperSimplex {
            source: self.0,
            warp_amplitude,
            feature_scale,
            vectorization_scheme: VectorizationScheme::default(),
        }
        .into()
    }

    pub fn domain_warp_super_simplex_with_scheme<A>(
        self,
        warp_amplitude: A,
        feature_scale: f32,
        vectorization_scheme: VectorizationScheme,
    ) -> GeneratorWrapper<DomainWarpSuperSimplex<S, A>>
    where
        A: Hybrid,
    {
        DomainWarpSuperSimplex {
            source: self.0,
            warp_amplitude,
            feature_scale,
            vectorization_scheme,
        }
        .into()
    }
}
