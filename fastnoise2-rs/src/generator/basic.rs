use crate::FastNoise;

use super::{DistanceFunction, Node};

#[derive(Clone, Copy)]
pub struct Constant {
    pub value: f32,
}

#[derive(Clone, Copy)]
pub struct White;

#[derive(Clone, Copy)]
pub struct Checkerboard {
    pub size: f32,
}

#[derive(Clone, Copy)]
pub struct SineWave {
    pub scale: f32,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct DistanceToPoint {
    pub distance_function: DistanceFunction,
    pub x_point: f32,
    pub y_point: f32,
    pub z_point: f32,
    pub w_point: f32,
}

impl Node for Constant {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Constant").unwrap();
        node.set("Value", self.value).unwrap();
        node
    }
}

impl Node for White {
    fn build_node(&self) -> FastNoise {
        FastNoise::from_name("White").unwrap()
    }
}

impl Node for Checkerboard {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("Checkerboard").unwrap();
        node.set("Size", self.size).unwrap();
        node
    }
}

impl Node for SineWave {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("SineWave").unwrap();
        node.set("Scale", self.scale).unwrap();
        node
    }
}

impl Node for PositionOutput {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("PositionOutput").unwrap();
        node.set("MultiplierX", self.x_multiplier).unwrap();
        node.set("MultiplierY", self.y_multiplier).unwrap();
        node.set("MultiplierZ", self.z_multiplier).unwrap();
        node.set("MultiplierW", self.w_multiplier).unwrap();
        node.set("OffsetX", self.x_offset).unwrap();
        node.set("OffsetY", self.y_offset).unwrap();
        node.set("OffsetZ", self.z_offset).unwrap();
        node.set("OffsetW", self.w_offset).unwrap();
        node
    }
}

impl Node for DistanceToPoint {
    fn build_node(&self) -> FastNoise {
        let mut node = FastNoise::from_name("DistanceToPoint").unwrap();
        node.set("DistanceFunction", &*self.distance_function.to_string())
            .unwrap();
        node.set("PointX", self.x_point).unwrap();
        node.set("PointY", self.y_point).unwrap();
        node.set("PointZ", self.z_point).unwrap();
        node.set("PointW", self.w_point).unwrap();
        node
    }
}
