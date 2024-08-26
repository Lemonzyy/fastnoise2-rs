use std::{fmt::Debug, ops::Deref};

use crate::{metadata::MemberValue, MemberType, Node, SafeNode};

pub trait Generator: Clone {
    fn build(&self) -> GeneratorWrapper<SafeNode>;
}

impl Generator for SafeNode {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        self.clone().into()
    }
}

pub trait Hybrid: MemberValue + Clone {}

impl Hybrid for f32 {}

impl Hybrid for GeneratorWrapper<f32> {}

impl MemberValue for GeneratorWrapper<f32> {
    const TYPE: MemberType = MemberType::Float;

    fn apply(
        &self,
        node: &mut Node,
        member: &crate::metadata::Member,
    ) -> Result<(), crate::FastNoiseError> {
        self.0.apply(node, member)
    }
}

impl<T: Generator + Debug> Hybrid for T {}

impl<T: Generator + Debug> Generator for &T {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        (*self).build()
    }
}

impl<T: Generator + Debug> MemberValue for T {
    const TYPE: MemberType = MemberType::NodeLookup;

    fn apply(
        &self,
        node: &mut Node,
        member: &crate::metadata::Member,
    ) -> Result<(), crate::FastNoiseError> {
        node.set(&member.name, self.build().0 .0.as_ref())
    }
}

#[derive(Clone, Debug)]
pub struct GeneratorWrapper<T>(pub T);

impl<T: Hybrid> From<T> for GeneratorWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for GeneratorWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Generator + Debug> Generator for GeneratorWrapper<T> {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        self.0.build()
    }
}

#[derive(Clone, Debug)]
pub struct Checkerboard {
    pub size: f32,
}

impl Generator for Checkerboard {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Checkerboard").unwrap();
        node.set("Size", self.size).unwrap();
        SafeNode(node.into()).into()
    }
}

pub fn checkerboard(size: f32) -> GeneratorWrapper<Checkerboard> {
    Checkerboard { size }.into()
}

#[derive(Clone, Debug)]
pub struct SineWave {
    pub scale: f32,
}

impl Generator for SineWave {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("SineWave").unwrap();
        node.set("Scale", self.scale).unwrap();
        SafeNode(node.into()).into()
    }
}

#[derive(Clone, Debug)]
pub struct Add<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

impl<Lhs, Rhs> Generator for Add<Lhs, Rhs>
where
    Lhs: Generator + Debug,
    Rhs: Hybrid + Debug,
{
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Add").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> std::ops::Add<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Generator + Debug,
    Rhs: Hybrid + Debug,
{
    type Output = GeneratorWrapper<Add<Lhs, Rhs>>;

    fn add(self, rhs: Rhs) -> Self::Output {
        Add { lhs: self.0, rhs }.into()
    }
}

#[derive(Clone, Debug)]
pub struct Subtract<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

impl<Lhs, Rhs> Generator for Subtract<Lhs, Rhs>
where
    Lhs: Hybrid + Debug,
    Rhs: Hybrid + Debug,
{
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Subtract").unwrap();
        node.set("LHS", self.lhs.clone()).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> std::ops::Sub<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Hybrid + Debug,
    Rhs: Hybrid + Debug,
{
    type Output = GeneratorWrapper<Subtract<Lhs, Rhs>>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        Subtract { lhs: self.0, rhs }.into()
    }
}

#[derive(Clone, Debug)]
pub struct FractalFBm<S, G, W>
where
    S: Generator,
    G: Hybrid,
    W: Hybrid,
{
    pub source: S,
    pub gain: G,
    pub weighted_strength: W,
    pub octaves: i32,
    pub lacunarity: f32,
}

impl<S, G, W> Generator for FractalFBm<S, G, W>
where
    S: Generator + Debug,
    G: Hybrid + Debug,
    W: Hybrid + Debug,
{
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("FractalFBm").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

#[derive(Clone, Debug)]
pub struct Perlin;

impl Generator for Perlin {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        SafeNode(Node::from_name("Perlin").unwrap().into()).into()
    }
}
