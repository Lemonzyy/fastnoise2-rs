use super::{DistanceFunction, Generator, GeneratorWrapper};
use crate::{safe::SafeNode, Node};

#[derive(Clone, Debug)]
pub struct Constant {
    pub value: f32,
}

#[derive(Clone, Debug)]
pub struct White;

#[derive(Clone, Debug)]
pub struct Checkerboard {
    pub size: f32,
}

#[derive(Clone, Debug)]
pub struct SineWave {
    pub feature_scale: f32,
}

/// Gradient generator (formerly PositionOutput in older FastNoise2 versions).
/// Outputs a linear gradient based on position coordinates.
#[derive(Clone, Debug)]
pub struct Gradient {
    pub multiplier_x: f32,
    pub multiplier_y: f32,
    pub multiplier_z: f32,
    pub multiplier_w: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub offset_w: f32,
}

#[derive(Clone, Debug)]
pub struct DistanceToPoint {
    pub distance_function: DistanceFunction,
    pub x_point: f32,
    pub y_point: f32,
    pub z_point: f32,
    pub w_point: f32,
}

impl Generator for Constant {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Constant").unwrap();
        node.set("Value", self.value).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for White {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        SafeNode(Node::from_name("White").unwrap().into()).into()
    }
}

impl Generator for Checkerboard {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Checkerboard").unwrap();
        node.set("FeatureScale", self.size).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for SineWave {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("SineWave").unwrap();
        node.set("FeatureScale", self.feature_scale).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for Gradient {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Gradient").unwrap();
        node.set("MultiplierX", self.multiplier_x).unwrap();
        node.set("MultiplierY", self.multiplier_y).unwrap();
        node.set("MultiplierZ", self.multiplier_z).unwrap();
        node.set("MultiplierW", self.multiplier_w).unwrap();
        node.set("OffsetX", self.offset_x).unwrap();
        node.set("OffsetY", self.offset_y).unwrap();
        node.set("OffsetZ", self.offset_z).unwrap();
        node.set("OffsetW", self.offset_w).unwrap();
        SafeNode(node.into()).into()
    }
}

impl Generator for DistanceToPoint {
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DistanceToPoint").unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string()).unwrap();
        node.set("PointX", self.x_point).unwrap();
        node.set("PointY", self.y_point).unwrap();
        node.set("PointZ", self.z_point).unwrap();
        node.set("PointW", self.w_point).unwrap();
        SafeNode(node.into()).into()
    }
}

pub fn constant(value: f32) -> GeneratorWrapper<Constant> {
    Constant { value }.into()
}

pub fn white() -> GeneratorWrapper<White> {
    White.into()
}

pub fn checkerboard(size: f32) -> GeneratorWrapper<Checkerboard> {
    Checkerboard { size }.into()
}
pub fn sinewave(feature_scale: f32) -> GeneratorWrapper<SineWave> {
    SineWave { feature_scale }.into()
}

/// Creates a Gradient generator with the given multipliers and offsets.
pub fn gradient(multiplier: [f32; 4], offset: [f32; 4]) -> GeneratorWrapper<Gradient> {
    let [multiplier_x, multiplier_y, multiplier_z, multiplier_w] = multiplier;
    let [offset_x, offset_y, offset_z, offset_w] = offset;
    Gradient {
        multiplier_x,
        multiplier_y,
        multiplier_z,
        multiplier_w,
        offset_x,
        offset_y,
        offset_z,
        offset_w,
    }
    .into()
}

pub fn distance_to_point(distance_function: DistanceFunction, point: [f32; 4]) -> GeneratorWrapper<DistanceToPoint> {
    let [x_point, y_point, z_point, w_point] = point;
    DistanceToPoint {
        distance_function,
        x_point,
        y_point,
        z_point,
        w_point,
    }
    .into()
}
