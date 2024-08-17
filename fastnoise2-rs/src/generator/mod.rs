use std::fmt::Display;

use crate::{metadata::MemberValue, typed::TypedFastNoise, FastNoise, MemberType};

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

pub trait Node: Copy {
    fn build_node(&self) -> TypedFastNoise;
}

impl<N: Node> MemberValue for N {
    const TYPE: MemberType = MemberType::NodeLookup;

    fn apply(
        self,
        node: &mut FastNoise,
        member: &crate::metadata::Member,
    ) -> Result<(), crate::FastNoiseError> {
        node.set(&member.name, &self.build_node().0)
    }
}

pub trait Hybrid: MemberValue {}

impl Hybrid for f32 {}

impl<N: Node> Hybrid for N {}

#[derive(Clone, Copy, Debug)]
pub struct Generator<T: Hybrid>(pub T);

impl<T: Node> Node for Generator<T> {
    fn build_node(&self) -> TypedFastNoise {
        self.0.build_node()
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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
