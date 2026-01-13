use super::{Generator, GeneratorWrapper, Hybrid};
use crate::{safe::SafeNode, Node};

#[derive(Clone, Debug)]
pub struct FractalFBm<S, G, W>
where
    S: Generator,
    G: Hybrid,
    W: Hybrid,
{
    pub source: S,
    pub gain: G,
    pub weighted_strength: W,
    pub octaves: i32,
    pub lacunarity: f32,
}

#[derive(Clone, Debug)]
pub struct FractalRidged<S, G, W>
where
    S: Generator,
    G: Hybrid,
    W: Hybrid,
{
    pub source: S,
    pub gain: G,
    pub weighted_strength: W,
    pub octaves: i32,
    pub lacunarity: f32,
}

impl<S, G, W> Generator for FractalFBm<S, G, W>
where
    S: Generator,
    G: Hybrid,
    W: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("FractalFBm").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone()).unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, G, W> Generator for FractalRidged<S, G, W>
where
    S: Generator,
    G: Hybrid,
    W: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("FractalRidged").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone()).unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> GeneratorWrapper<S>
where
    S: Generator,
{
    pub fn fbm<G, W>(self, gain: G, weighted_strength: W, octaves: i32, lacunarity: f32) -> GeneratorWrapper<FractalFBm<S, G, W>>
    where
        G: Hybrid,
        W: Hybrid,
    {
        FractalFBm {
            source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        }
        .into()
    }

    pub fn ridged<G, W>(self, gain: G, weighted_strength: W, octaves: i32, lacunarity: f32) -> GeneratorWrapper<FractalRidged<S, G, W>>
    where
        G: Hybrid,
        W: Hybrid,
    {
        FractalRidged {
            source: self.0,
            gain,
            weighted_strength,
            octaves,
            lacunarity,
        }
        .into()
    }
}
