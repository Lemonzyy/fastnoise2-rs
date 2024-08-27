use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper, Hybrid};

pub trait DomainWarpNode: Generator {}

#[derive(Clone, Debug)]
pub struct DomainWarpGradient<S, A>
where
    S: Generator,
    A: Hybrid,
{
    pub source: S,
    pub warp_amplitude: A,
    pub warp_frequency: f32,
}

impl<S, A> Generator for DomainWarpGradient<S, A>
where
    S: Generator,
    A: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainWarpGradient").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("WarpAmplitude", self.warp_amplitude.clone())
            .unwrap();
        node.set("WarpFrequency", self.warp_frequency).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, A> DomainWarpNode for DomainWarpGradient<S, A>
where
    S: Generator,
    A: Hybrid,
{
}

impl<S> GeneratorWrapper<S>
where
    S: Generator,
{
    pub fn domain_warp_gradient<A>(
        self,
        warp_amplitude: A,
        warp_frequency: f32,
    ) -> GeneratorWrapper<DomainWarpGradient<S, A>>
    where
        A: Hybrid,
    {
        DomainWarpGradient {
            source: self.0,
            warp_amplitude,
            warp_frequency,
        }
        .into()
    }
}
