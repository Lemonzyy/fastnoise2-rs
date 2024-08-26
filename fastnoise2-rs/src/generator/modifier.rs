use crate::{safe::SafeNode, Node};

use super::{Dimension, Generator, GeneratorWrapper, Hybrid};

#[derive(Clone, Debug)]
pub struct DomainScale<Source: Generator> {
    pub source: Source,
    pub scale: f32,
}

#[derive(Clone, Debug)]
pub struct DomainOffset<
    Source: Generator,
    XOffset: Hybrid,
    YOffset: Hybrid,
    ZOffset: Hybrid,
    WOffset: Hybrid,
> {
    pub source: Source,
    pub x_offset: XOffset,
    pub y_offset: YOffset,
    pub z_offset: ZOffset,
    pub w_offset: WOffset,
}

#[derive(Clone, Debug)]
pub struct DomainRotate<Source: Generator> {
    pub source: Source,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Clone, Debug)]
pub struct SeedOffset<Source: Generator> {
    pub source: Source,
    pub seed_offset: i32,
}

#[derive(Clone, Debug)]
pub struct Remap<Source: Generator> {
    pub source: Source,
    pub from_min: f32,
    pub from_max: f32,
    pub to_min: f32,
    pub to_max: f32,
}

#[derive(Clone, Debug)]
pub struct ConvertRgba8<Source: Generator> {
    pub source: Source,
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Debug)]
pub struct Terrace<Source: Generator> {
    pub source: Source,
    pub multiplier: f32,
    pub smoothness: f32,
}

#[derive(Clone, Debug)]
pub struct DomainAxisScale<Source: Generator> {
    pub source: Source,
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
    pub w_scale: f32,
}

#[derive(Clone, Debug)]
pub struct AddDimension<Source: Generator, NewDimensionPosition> {
    pub source: Source,
    pub new_dimension_position: NewDimensionPosition,
}

#[derive(Clone, Debug)]
pub struct RemoveDimension<Source: Generator> {
    pub source: Source,
    pub remove_dimension: Dimension,
}

#[derive(Clone, Debug)]
pub struct GeneratorCache<Source: Generator> {
    pub source: Source,
}

impl<Source: Generator> Generator for DomainScale<Source> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainScale").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Scale", self.scale).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator, XOffset: Hybrid, YOffset: Hybrid, ZOffset: Hybrid, WOffset: Hybrid>
    Generator for DomainOffset<Source, XOffset, YOffset, ZOffset, WOffset>
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

impl<Source: Generator> Generator for DomainRotate<Source> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("DomainRotate").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Yaw", self.yaw).unwrap();
        node.set("Pitch", self.pitch).unwrap();
        node.set("Roll", self.roll).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator> Generator for SeedOffset<Source> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("SeedOffset").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("SeedOffset", self.seed_offset).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator> Generator for Remap<Source> {
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

impl<Source: Generator> Generator for ConvertRgba8<Source> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("ConvertRgba8").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Min", self.min).unwrap();
        node.set("Max", self.max).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator> Generator for Terrace<Source> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Terrace").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("Multiplier", self.multiplier).unwrap();
        node.set("Smoothness", self.smoothness).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator> Generator for DomainAxisScale<Source> {
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

impl<Source: Generator, NewDimensionPosition: Hybrid> Generator
    for AddDimension<Source, NewDimensionPosition>
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("AddDimension").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("NewDimensionPosition", self.new_dimension_position.clone())
            .unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator> Generator for RemoveDimension<Source> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("RemoveDimension").unwrap();
        node.set("Source", &self.source).unwrap();
        node.set("RemoveDimension", &*self.remove_dimension.to_string())
            .unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator> Generator for GeneratorCache<Source> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("GeneratorCache").unwrap();
        node.set("Source", &self.source).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Source: Generator> GeneratorWrapper<Source> {
    pub fn scale(self, scale: f32) -> GeneratorWrapper<DomainScale<Source>> {
        DomainScale {
            source: self.0,
            scale,
        }
        .into()
    }

    pub fn offset<XOffset: Hybrid, YOffset: Hybrid, ZOffset: Hybrid, WOffset: Hybrid>(
        self,
        x_offset: XOffset,
        y_offset: YOffset,
        z_offset: ZOffset,
        w_offset: WOffset,
    ) -> GeneratorWrapper<DomainOffset<Source, XOffset, YOffset, ZOffset, WOffset>> {
        DomainOffset {
            source: self.0,
            x_offset,
            y_offset,
            z_offset,
            w_offset,
        }
        .into()
    }

    pub fn rotate(self, yaw: f32, pitch: f32, roll: f32) -> GeneratorWrapper<DomainRotate<Source>> {
        DomainRotate {
            source: self.0,
            yaw,
            pitch,
            roll,
        }
        .into()
    }

    pub fn seed_offset(self, seed_offset: i32) -> GeneratorWrapper<SeedOffset<Source>> {
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
    ) -> GeneratorWrapper<Remap<Source>> {
        Remap {
            source: self.0,
            from_min,
            from_max,
            to_min,
            to_max,
        }
        .into()
    }

    pub fn convert_rgba8(self, min: f32, max: f32) -> GeneratorWrapper<ConvertRgba8<Source>> {
        ConvertRgba8 {
            source: self.0,
            min,
            max,
        }
        .into()
    }

    pub fn terrace(self, multiplier: f32, smoothness: f32) -> GeneratorWrapper<Terrace<Source>> {
        Terrace {
            source: self.0,
            multiplier,
            smoothness,
        }
        .into()
    }

    pub fn axis_scale(self, scale: [f32; 4]) -> GeneratorWrapper<DomainAxisScale<Source>> {
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

    pub fn add_dimension<NewDimensionPosition: Hybrid>(
        self,
        new_dimension_position: NewDimensionPosition,
    ) -> GeneratorWrapper<AddDimension<Source, NewDimensionPosition>> {
        AddDimension {
            source: self.0,
            new_dimension_position,
        }
        .into()
    }

    pub fn remove_dimension(
        self,
        remove_dimension: Dimension,
    ) -> GeneratorWrapper<RemoveDimension<Source>> {
        RemoveDimension {
            source: self.0,
            remove_dimension,
        }
        .into()
    }

    pub fn cache(self) -> GeneratorWrapper<GeneratorCache<Source>> {
        GeneratorCache { source: self.0 }.into()
    }
}
