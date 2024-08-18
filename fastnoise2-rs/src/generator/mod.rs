//! FastNoise2 generators as types. Ensures safety at compile time.
//!
//! # Examples
//!
//! ```rust
//! use fastnoise2::generator::prelude::*;
//!
//! let node = perlin().fbm(0.5, 0.0, 3, 2.0).min(sinewave(0.3) + 0.2).build_node();
//! let out = node.gen_single_2d(0.0, 0.0, 123);
//! ```
//!
//! ## See Also
//! - [safe example](https://github.com/Lemonzyy/fastnoise2-rs/blob/main/fastnoise2-rs/examples/safe.rs)
//! - [safe_simple_terrain example](https://github.com/Lemonzyy/fastnoise2-rs/blob/main/fastnoise2-rs/examples/safe_simple_terrain.rs)
use std::fmt::Display;

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
    //! Functions and [`TypedNode`] re-exports
    pub use super::{
        basic::{checkerboard, constant, distance_to_point, position_output, sinewave, white},
        cellular::{
            distance as cellular_distance, lookup as cellular_lookup, value as cellular_value,
        },
        perlin::perlin,
        simplex::{opensimplex2, opensimplex2s, simplex},
        value::value,
        TypedNode,
    };
}

pub trait TypedNode {
    fn build_node(&self) -> SafeNode;
}

impl<N: TypedNode> MemberValue for N {
    const TYPE: MemberType = MemberType::NodeLookup;

    fn apply(
        &self,
        node: &mut Node,
        member: &crate::metadata::Member,
    ) -> Result<(), crate::FastNoiseError> {
        node.set(&member.name, &self.build_node().0)
    }
}

pub trait Hybrid: MemberValue {}

impl Hybrid for f32 {}

impl<N: TypedNode> Hybrid for N {}

// impl Hybrid for &f32 {}

// impl<N: TypedNode + Sized> Hybrid for N {}

#[derive(Debug)]
pub struct Generator<T: Hybrid>(pub T);

impl<T: TypedNode> TypedNode for Generator<T> {
    fn build_node(&self) -> SafeNode {
        self.0.build_node()
    }
}

impl<N: TypedNode> TypedNode for &N {
    fn build_node(&self) -> SafeNode {
        (*self).build_node()
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
