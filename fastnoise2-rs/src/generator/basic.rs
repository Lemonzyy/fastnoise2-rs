use crate::{safe::SafeNode, Node};

use super::{DistanceFunction, Generator, TypedNode};

#[derive(Debug)]
pub struct Constant {
    pub value: f32,
}

#[derive(Debug)]
pub struct White;

#[derive(Debug)]
pub struct Checkerboard {
    pub size: f32,
}

#[derive(Debug)]
pub struct SineWave {
    pub scale: f32,
}

#[derive(Debug)]
pub struct PositionOutput {
    pub x_multiplier: f32,
    pub y_multiplier: f32,
    pub z_multiplier: f32,
    pub w_multiplier: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub z_offset: f32,
    pub w_offset: f32,
}

#[derive(Debug)]
pub struct DistanceToPoint {
    pub distance_function: DistanceFunction,
    pub x_point: f32,
    pub y_point: f32,
    pub z_point: f32,
    pub w_point: f32,
}

impl TypedNode for Constant {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Constant").unwrap();
        node.set("Value", self.value).unwrap();
        SafeNode(node)
    }
}

impl TypedNode for White {
    fn build_node(&self) -> SafeNode {
        SafeNode(Node::from_name("White").unwrap())
    }
}

impl TypedNode for Checkerboard {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Checkerboard").unwrap();
        node.set("Size", self.size).unwrap();
        SafeNode(node)
    }
}

impl TypedNode for SineWave {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("SineWave").unwrap();
        node.set("Scale", self.scale).unwrap();
        SafeNode(node)
    }
}

impl TypedNode for PositionOutput {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("PositionOutput").unwrap();
        node.set("MultiplierX", self.x_multiplier).unwrap();
        node.set("MultiplierY", self.y_multiplier).unwrap();
        node.set("MultiplierZ", self.z_multiplier).unwrap();
        node.set("MultiplierW", self.w_multiplier).unwrap();
        node.set("OffsetX", self.x_offset).unwrap();
        node.set("OffsetY", self.y_offset).unwrap();
        node.set("OffsetZ", self.z_offset).unwrap();
        node.set("OffsetW", self.w_offset).unwrap();
        SafeNode(node)
    }
}

impl TypedNode for DistanceToPoint {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("DistanceToPoint").unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("PointX", self.x_point).unwrap();
        node.set("PointY", self.y_point).unwrap();
        node.set("PointZ", self.z_point).unwrap();
        node.set("PointW", self.w_point).unwrap();
        SafeNode(node)
    }
}

pub fn constant(value: f32) -> Generator<Constant> {
    Generator(Constant { value })
}

pub fn white() -> Generator<White> {
    Generator(White)
}

pub fn checkerboard(size: f32) -> Generator<Checkerboard> {
    Generator(Checkerboard { size })
}
pub fn sinewave(scale: f32) -> Generator<SineWave> {
    Generator(SineWave { scale })
}

pub fn position_output(multiplier: [f32; 4], offset: [f32; 4]) -> Generator<PositionOutput> {
    let [x_multiplier, y_multiplier, z_multiplier, w_multiplier] = multiplier;
    let [x_offset, y_offset, z_offset, w_offset] = offset;
    Generator(PositionOutput {
        x_multiplier,
        y_multiplier,
        z_multiplier,
        w_multiplier,
        x_offset,
        y_offset,
        z_offset,
        w_offset,
    })
}

pub fn distance_to_point(
    distance_function: DistanceFunction,
    point: [f32; 4],
) -> Generator<DistanceToPoint> {
    let [x_point, y_point, z_point, w_point] = point;
    Generator(DistanceToPoint {
        distance_function,
        x_point,
        y_point,
        z_point,
        w_point,
    })
}
