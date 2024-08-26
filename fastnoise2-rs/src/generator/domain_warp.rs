use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper, Hybrid};

pub trait DomainWarpNode: Generator {}

#[derive(Clone, Debug)]
pub struct DomainWarpGradient<Source: Generator, WarpAmplitude: Hybrid> {
    pub source: Source,
    pub warp_amplitude: WarpAmplitude,
    pub warp_frequency: f32,
}

impl<Source: Generator, WarpAmplitude: Hybrid> Generator
    for DomainWarpGradient<Source, WarpAmplitude>
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainWarpGradient").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("WarpAmplitude", self.warp_amplitude.clone())
            .unwrap();
        node.set("WarpFrequency", self.warp_frequency).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator, WarpAmplitude: Hybrid> DomainWarpNode
    for DomainWarpGradient<Source, WarpAmplitude>
{
}

impl<Source: Generator> GeneratorWrapper<Source> {
    pub fn warp_gradient<WarpAmplitude: Hybrid>(
        self,
        warp_amplitude: WarpAmplitude,
        warp_frequency: f32,
    ) -> GeneratorWrapper<DomainWarpGradient<Source, WarpAmplitude>> {
        DomainWarpGradient {
            source: self.0,
            warp_amplitude,
            warp_frequency,
        }
        .into()
    }
}
