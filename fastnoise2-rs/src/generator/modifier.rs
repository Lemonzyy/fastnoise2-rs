use crate::{safe::SafeNode, Node};

use super::{Dimension, Generator, GeneratorWrapper, Hybrid};

#[derive(Clone, Debug)]
pub struct DomainScale<S>
where
    S: Generator,
{
    pub source: S,
    pub scale: f32,
}

#[derive(Clone, Debug)]
pub struct DomainOffset<S, X, Y, Z, W>
where
    S: Generator,
    X: Hybrid,
    Y: Hybrid,
    Z: Hybrid,
    W: Hybrid,
{
    pub source: S,
    pub x_offset: X,
    pub y_offset: Y,
    pub z_offset: Z,
    pub w_offset: W,
}

#[derive(Clone, Debug)]
pub struct DomainRotate<S>
where
    S: Generator,
{
    pub source: S,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Clone, Debug)]
pub struct SeedOffset<S>
where
    S: Generator,
{
    pub source: S,
    pub seed_offset: i32,
}

#[derive(Clone, Debug)]
pub struct Remap<S>
where
    S: Generator,
{
    pub source: S,
    pub from_min: f32,
    pub from_max: f32,
    pub to_min: f32,
    pub to_max: f32,
}

#[derive(Clone, Debug)]
pub struct ConvertRgba8<S>
where
    S: Generator,
{
    pub source: S,
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Debug)]
pub struct Terrace<S>
where
    S: Generator,
{
    pub source: S,
    pub multiplier: f32,
    pub smoothness: f32,
}

#[derive(Clone, Debug)]
pub struct DomainAxisScale<S>
where
    S: Generator,
{
    pub source: S,
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
    pub w_scale: f32,
}

#[derive(Clone, Debug)]
pub struct AddDimension<S, N>
where
    S: Generator,
    N: Hybrid,
{
    pub source: S,
    pub new_dimension_position: N,
}

#[derive(Clone, Debug)]
pub struct RemoveDimension<S>
where
    S: Generator,
{
    pub source: S,
    pub remove_dimension: Dimension,
}

#[derive(Clone, Debug)]
pub struct GeneratorCache<S>
where
    S: Generator,
{
    pub source: S,
}

impl<S> Generator for DomainScale<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainScale").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Scale", self.scale).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, X, Y, Z, W> Generator for DomainOffset<S, X, Y, Z, W>
where
    S: Generator,
    X: Hybrid,
    Y: Hybrid,
    Z: Hybrid,
    W: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainOffset").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("OffsetX", self.x_offset.clone()).unwrap();
        node.set("OffsetY", self.y_offset.clone()).unwrap();
        node.set("OffsetZ", self.z_offset.clone()).unwrap();
        node.set("OffsetW", self.w_offset.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> Generator for DomainRotate<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainRotate").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Yaw", self.yaw).unwrap();
        node.set("Pitch", self.pitch).unwrap();
        node.set("Roll", self.roll).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> Generator for SeedOffset<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("SeedOffset").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> Generator for Remap<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Remap").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("FromMin", self.from_min).unwrap();
        node.set("FromMax", self.from_max).unwrap();
        node.set("ToMin", self.to_min).unwrap();
        node.set("ToMax", self.to_max).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> Generator for ConvertRgba8<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("ConvertRgba8").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Min", self.min).unwrap();
        node.set("Max", self.max).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> Generator for Terrace<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Terrace").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Multiplier", self.multiplier).unwrap();
        node.set("Smoothness", self.smoothness).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> Generator for DomainAxisScale<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainAxisScale").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("ScaleX", self.x_scale).unwrap();
        node.set("ScaleY", self.y_scale).unwrap();
        node.set("ScaleZ", self.z_scale).unwrap();
        node.set("ScaleW", self.w_scale).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S, N> Generator for AddDimension<S, N>
where
    S: Generator,
    N: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("AddDimension").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("NewDimensionPosition", self.new_dimension_position.clone())
            .unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> Generator for RemoveDimension<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("RemoveDimension").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("RemoveDimension", &*self.remove_dimension.to_string())
            .unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> Generator for GeneratorCache<S>
where
    S: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("GeneratorCache").unwrap();
        node.set("Source", &self.source).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<S> GeneratorWrapper<S>
where
    S: Generator,
{
    pub fn scale(self, scale: f32) -> GeneratorWrapper<DomainScale<S>> {
        DomainScale {
            source: self.0,
            scale,
        }
        .into()
    }

    pub fn offset<X, Y, Z, W>(
        self,
        x_offset: X,
        y_offset: Y,
        z_offset: Z,
        w_offset: W,
    ) -> GeneratorWrapper<DomainOffset<S, X, Y, Z, W>>
    where
        X: Hybrid,
        Y: Hybrid,
        Z: Hybrid,
        W: Hybrid,
    {
        DomainOffset {
            source: self.0,
            x_offset,
            y_offset,
            z_offset,
            w_offset,
        }
        .into()
    }

    pub fn rotate(self, yaw: f32, pitch: f32, roll: f32) -> GeneratorWrapper<DomainRotate<S>> {
        DomainRotate {
            source: self.0,
            yaw,
            pitch,
            roll,
        }
        .into()
    }

    pub fn seed_offset(self, seed_offset: i32) -> GeneratorWrapper<SeedOffset<S>> {
        SeedOffset {
            source: self.0,
            seed_offset,
        }
        .into()
    }

    pub fn remap(
        self,
        from_min: f32,
        from_max: f32,
        to_min: f32,
        to_max: f32,
    ) -> GeneratorWrapper<Remap<S>> {
        Remap {
            source: self.0,
            from_min,
            from_max,
            to_min,
            to_max,
        }
        .into()
    }

    pub fn convert_rgba8(self, min: f32, max: f32) -> GeneratorWrapper<ConvertRgba8<S>> {
        ConvertRgba8 {
            source: self.0,
            min,
            max,
        }
        .into()
    }

    pub fn terrace(self, multiplier: f32, smoothness: f32) -> GeneratorWrapper<Terrace<S>> {
        Terrace {
            source: self.0,
            multiplier,
            smoothness,
        }
        .into()
    }

    pub fn axis_scale(self, scale: [f32; 4]) -> GeneratorWrapper<DomainAxisScale<S>> {
        let [x_scale, y_scale, z_scale, w_scale] = scale;
        DomainAxisScale {
            source: self.0,
            x_scale,
            y_scale,
            z_scale,
            w_scale,
        }
        .into()
    }

    pub fn add_dimension<N>(self, new_dimension_position: N) -> GeneratorWrapper<AddDimension<S, N>>
    where
        N: Hybrid,
    {
        AddDimension {
            source: self.0,
            new_dimension_position,
        }
        .into()
    }

    pub fn remove_dimension(
        self,
        remove_dimension: Dimension,
    ) -> GeneratorWrapper<RemoveDimension<S>> {
        RemoveDimension {
            source: self.0,
            remove_dimension,
        }
        .into()
    }

    pub fn cache(self) -> GeneratorWrapper<GeneratorCache<S>> {
        GeneratorCache { source: self.0 }.into()
    }
}
