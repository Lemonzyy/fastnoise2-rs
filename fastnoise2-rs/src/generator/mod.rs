//! FastNoise2 generators as types. Ensures safety at compile time.
//!
//! # Examples
//!
//! ```rust
//! use fastnoise2::generator::prelude::*;
//!
//! let node = perlin().fbm(0.5, 0.0, 3, 2.0).min(sinewave(0.3) + 0.2).build();
//! let out = node.gen_single_2d(0.0, 0.0, 123);
//! ```
//!
//! ## See Also
//! - [safe example](https://github.com/Lemonzyy/fastnoise2-rs/blob/main/fastnoise2-rs/examples/safe.rs)
//! - [safe_simple_terrain example](https://github.com/Lemonzyy/fastnoise2-rs/blob/main/fastnoise2-rs/examples/safe_simple_terrain.rs)
use std::fmt::{Debug, Display};

use crate::{metadata::MemberValue, safe::SafeNode, MemberType, Node};

pub mod basic;
pub mod blend;
pub mod cellular;
pub mod domain_warp;
pub mod domain_warp_fractal;
pub mod fractal;
pub mod modifier;
pub mod perlin;
pub mod simplex;
pub mod value;

pub mod prelude {
    //! Functions and [`Generator`] re-exports
    pub use super::{
        basic::{checkerboard, constant, distance_to_point, position_output, sinewave, white},
        cellular::{cellular_distance, cellular_lookup, cellular_value},
        perlin::perlin,
        simplex::{opensimplex2, opensimplex2s, simplex},
        value::value,
        Generator, GeneratorWrapper,
    };
}

pub trait Generator: Clone + Debug {
    fn build(&self) -> GeneratorWrapper<SafeNode>;
}

impl<T: Generator> Generator for &T {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        (*self).build()
    }
}

impl Generator for SafeNode {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        self.clone().into()
    }
}

pub trait Hybrid: MemberValue + Clone + Debug {}

impl Hybrid for f32 {}

impl<T: Generator> Hybrid for T {}

impl<T: Generator> MemberValue for T {
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

impl<T> std::ops::Deref for GeneratorWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Generator> Generator for GeneratorWrapper<T> {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        self.0.build()
    }
}

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

#[derive(Clone, Debug)]
pub enum DistanceFunction {
    Euclidean,
    EuclideanSquared,
    Manhattan,
    Hybrid,
    MaxAxis,
}

impl Display for DistanceFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistanceFunction::Euclidean => f.write_str("Euclidean"),
            DistanceFunction::EuclideanSquared => f.write_str("EuclideanSquared"),
            DistanceFunction::Manhattan => f.write_str("Manhattan"),
            DistanceFunction::Hybrid => f.write_str("Hybrid"),
            DistanceFunction::MaxAxis => f.write_str("MaxAxis"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Dimension {
    X,
    Y,
    Z,
    W,
}

impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dimension::X => f.write_str("X"),
            Dimension::Y => f.write_str("Y"),
            Dimension::Z => f.write_str("Z"),
            Dimension::W => f.write_str("W"),
        }
    }
}
