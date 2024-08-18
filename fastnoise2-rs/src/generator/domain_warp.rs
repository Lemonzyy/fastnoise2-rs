use crate::{safe::SafeNode, Node};

use super::{Generator, Hybrid, TypedNode};

pub trait DomainWarpNode: TypedNode {}

#[derive(Debug)]
pub struct DomainWarpGradient<Source: TypedNode, WarpAmplitude: Hybrid> {
    pub source: Source,
    pub warp_amplitude: WarpAmplitude,
    pub warp_frequency: f32,
}

impl<Source: TypedNode, WarpAmplitude: Hybrid> TypedNode
    for DomainWarpGradient<Source, WarpAmplitude>
{
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("DomainWarpGradient").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("WarpAmplitude", &self.warp_amplitude).unwrap();
        node.set("WarpFrequency", self.warp_frequency).unwrap();
        SafeNode(node)
    }
}

impl<Source: TypedNode, WarpAmplitude: Hybrid> DomainWarpNode
    for DomainWarpGradient<Source, WarpAmplitude>
{
}

impl<Source: TypedNode> Generator<Source> {
    pub fn warp_gradient<WarpAmplitude: Hybrid>(
        self,
        warp_amplitude: WarpAmplitude,
        warp_frequency: f32,
    ) -> Generator<DomainWarpGradient<Source, WarpAmplitude>> {
        Generator(DomainWarpGradient {
            source: self.0,
            warp_amplitude,
            warp_frequency,
        })
    }
}
