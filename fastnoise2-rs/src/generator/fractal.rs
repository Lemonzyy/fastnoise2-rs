use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper, Hybrid};

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

#[derive(Clone, Debug)]
pub struct FractalPingPong<S, G, W, P>
where
    S: Generator,
    G: Hybrid,
    W: Hybrid,
    P: Hybrid,
{
    pub source: S,
    pub gain: G,
    pub weighted_strength: W,
    pub ping_pong_strength: P,
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
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
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
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, G, W, P> Generator for FractalPingPong<S, G, W, P>
where
    S: Generator,
    G: Hybrid,
    W: Hybrid,
    P: Hybrid,
{
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("FractalFBm").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Gain", self.gain.clone()).unwrap();
        node.set("WeightedStrength", self.weighted_strength.clone())
            .unwrap();
        node.set("PingPongStrength", self.ping_pong_strength.clone())
            .unwrap();
        node.set("Octaves", self.octaves).unwrap();
        node.set("Lacunarity", self.lacunarity).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> GeneratorWrapper<S>
where
    S: Generator,
{
    pub fn fbm<G, W>(
        self,
        gain: G,
        weighted_strength: W,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<FractalFBm<S, G, W>>
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

    pub fn ridged<G, W>(
        self,
        gain: G,
        weighted_strength: W,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<FractalRidged<S, G, W>>
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

    pub fn ping_pong<G, W, P>(
        self,
        gain: G,
        weighted_strength: W,
        ping_pong_strength: P,
        octaves: i32,
        lacunarity: f32,
    ) -> GeneratorWrapper<FractalPingPong<S, G, W, P>>
    where
        G: Hybrid,
        W: Hybrid,
        P: Hybrid,
    {
        FractalPingPong {
            source: self.0,
            gain,
            weighted_strength,
            ping_pong_strength,
            octaves,
            lacunarity,
        }
        .into()
    }
}
