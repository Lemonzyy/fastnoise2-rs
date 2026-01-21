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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        generator::{perlin::perlin, simplex::simplex},
        test_utils::*,
    };

    #[test]
    fn test_fractal_fbm() {
        let node = perlin().fbm(0.5, 0.0, 4, 2.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_fractal_ridged() {
        let node = perlin().ridged(0.5, 0.0, 4, 2.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_param_fractal_gain() {
        let node1 = perlin().fbm(0.3, 0.0, 4, 2.0).build();
        let node2 = perlin().fbm(0.7, 0.0, 4, 2.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "FractalFBm.Gain");
    }

    #[test]
    fn test_param_fractal_weighted_strength() {
        let node1 = perlin().fbm(0.5, 0.0, 4, 2.0).build();
        let node2 = perlin().fbm(0.5, 0.5, 4, 2.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "FractalFBm.Weighted Strength");
    }

    #[test]
    fn test_param_fractal_octaves() {
        let node1 = perlin().fbm(0.5, 0.0, 2, 2.0).build();
        let node2 = perlin().fbm(0.5, 0.0, 6, 2.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "FractalFBm.Octaves");
    }

    #[test]
    fn test_param_fractal_lacunarity() {
        let node1 = perlin().fbm(0.5, 0.0, 4, 1.5).build();
        let node2 = perlin().fbm(0.5, 0.0, 4, 3.0).build();
        let output1 = generate_output(&node1.0);
        let output2 = generate_output(&node2.0);
        assert_outputs_differ(&output1, &output2, "FractalFBm.Lacunarity");
    }

    #[test]
    fn test_hybrid_gain_fbm() {
        // Using a generator as the gain parameter (hybrid)
        let gain_node = simplex().domain_scale(0.1);
        let node = perlin().fbm(gain_node, 0.0, 4, 2.0).build();
        test_generator_produces_output(node.0);
    }

    #[test]
    fn test_hybrid_weighted_strength_ridged() {
        // Using a generator as weighted_strength parameter
        let strength_node = simplex().domain_scale(0.1);
        let node = perlin().ridged(0.5, strength_node, 4, 2.0).build();
        test_generator_produces_output(node.0);
    }
}
