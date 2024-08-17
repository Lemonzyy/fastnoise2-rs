use crate::{typed::TypedFastNoise, FastNoise};

use super::{Generator, Hybrid, Node};

#[derive(Clone, Copy, Debug)]
pub struct DomainWarpGradient<Source: Node, WarpAmplitude: Hybrid> {
    source: Source,
    warp_amplitude: WarpAmplitude,
    warp_frequency: f32,
}

impl<Source: Node, WarpAmplitude: Hybrid> Node for DomainWarpGradient<Source, WarpAmplitude> {
    fn build_node(&self) -> TypedFastNoise {
        let mut node = FastNoise::from_name("DomainWarpGradient").unwrap();
        node.set("Source", self.source).unwrap();
        node.set("WarpAmplitude", self.warp_amplitude).unwrap();
        node.set("WarpFrequency", self.warp_frequency).unwrap();
        TypedFastNoise(node)
    }
}

pub fn gradient<Source: Node, WarpAmplitude: Hybrid>(
    source: Source,
    warp_amplitude: WarpAmplitude,
    warp_frequency: f32,
) -> Generator<DomainWarpGradient<Source, WarpAmplitude>> {
    Generator(DomainWarpGradient {
        source,
        warp_amplitude,
        warp_frequency,
    })
}
