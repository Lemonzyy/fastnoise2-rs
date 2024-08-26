use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper, Hybrid};

#[derive(Clone, Debug)]
pub struct Add<LHS: Generator, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Debug)]
pub struct Subtract<LHS: Hybrid, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Debug)]
pub struct Multiply<LHS: Generator, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Debug)]
pub struct Divide<LHS: Hybrid, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Debug)]
pub struct Min<LHS: Generator, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Debug)]
pub struct Max<LHS: Generator, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Debug)]
pub struct MinSmooth<LHS: Generator, RHS: Hybrid, SMOOTHNESS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
    pub smoothness: SMOOTHNESS,
}

#[derive(Clone, Debug)]
pub struct MaxSmooth<LHS: Generator, RHS: Hybrid, SMOOTHNESS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
    pub smoothness: SMOOTHNESS,
}

#[derive(Clone, Debug)]
pub struct Fade<A: Generator, B: Generator, FADE: Hybrid> {
    pub a: A,
    pub b: B,
    pub fade: FADE,
}

#[derive(Clone, Debug)]
pub struct PowFloat<VALUE: Hybrid, POW: Hybrid> {
    pub value: VALUE,
    pub pow: POW,
}

#[derive(Clone, Debug)]
pub struct PowInt<VALUE: Generator> {
    pub value: VALUE,
    pub pow: i32,
}

impl<LHS: Generator, RHS: Hybrid> Generator for Add<LHS, RHS> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Add").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<LHS: Hybrid, RHS: Hybrid> Generator for Subtract<LHS, RHS> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Subtract").unwrap();
        node.set("LHS", self.lhs.clone()).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<LHS: Generator, RHS: Hybrid> Generator for Multiply<LHS, RHS> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Multiply").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<LHS: Hybrid, RHS: Hybrid> Generator for Divide<LHS, RHS> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Divide").unwrap();
        node.set("LHS", self.lhs.clone()).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<LHS: Generator, RHS: Hybrid> Generator for Min<LHS, RHS> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Min").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<LHS: Generator, RHS: Hybrid> Generator for Max<LHS, RHS> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Max").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<LHS: Generator, RHS: Hybrid, SMOOTHNESS: Hybrid> Generator
    for MinSmooth<LHS, RHS, SMOOTHNESS>
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("MinSmooth").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        node.set("Smoothness", self.smoothness.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<LHS: Generator, RHS: Hybrid, SMOOTHNESS: Hybrid> Generator
    for MaxSmooth<LHS, RHS, SMOOTHNESS>
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("MaxSmooth").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        node.set("Smoothness", self.smoothness.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<A: Generator, B: Generator, FADE: Hybrid> Generator for Fade<A, B, FADE> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Fade").unwrap();
        node.set("A", &self.a).unwrap();
        node.set("B", &self.b).unwrap();
        node.set("Fade", self.fade.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<VALUE: Hybrid, POW: Hybrid> Generator for PowFloat<VALUE, POW> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("PowFloat").unwrap();
        node.set("Value", self.value.clone()).unwrap();
        node.set("Pow", self.pow.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<VALUE: Generator> Generator for PowInt<VALUE> {
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("PowInt").unwrap();
        node.set("Value", &self.value).unwrap();
        node.set("Pow", self.pow).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs: Generator, Rhs: Hybrid> std::ops::Add<Rhs> for GeneratorWrapper<Lhs> {
    type Output = GeneratorWrapper<Add<Lhs, Rhs>>;

    fn add(self, rhs: Rhs) -> Self::Output {
        Add { lhs: self.0, rhs }.into()
    }
}

impl<Lhs: Hybrid, Rhs: Hybrid> std::ops::Sub<Rhs> for GeneratorWrapper<Lhs> {
    type Output = GeneratorWrapper<Subtract<Lhs, Rhs>>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        Subtract { lhs: self.0, rhs }.into()
    }
}

impl<Lhs: Generator, Rhs: Hybrid> std::ops::Mul<Rhs> for GeneratorWrapper<Lhs> {
    type Output = GeneratorWrapper<Multiply<Lhs, Rhs>>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Multiply { lhs: self.0, rhs }.into()
    }
}

impl<Lhs: Generator> std::ops::Neg for GeneratorWrapper<Lhs> {
    type Output = GeneratorWrapper<Multiply<Lhs, f32>>;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<Lhs: Hybrid, Rhs: Hybrid> std::ops::Div<Rhs> for GeneratorWrapper<Lhs> {
    type Output = GeneratorWrapper<Divide<Lhs, Rhs>>;

    fn div(self, rhs: Rhs) -> Self::Output {
        Divide { lhs: self.0, rhs }.into()
    }
}

impl<Lhs: Generator> GeneratorWrapper<Lhs> {
    pub fn min<Rhs: Hybrid>(self, rhs: Rhs) -> GeneratorWrapper<Min<Lhs, Rhs>> {
        Min { lhs: self.0, rhs }.into()
    }

    pub fn max<Rhs: Hybrid>(self, rhs: Rhs) -> GeneratorWrapper<Max<Lhs, Rhs>> {
        Max { lhs: self.0, rhs }.into()
    }

    pub fn min_smooth<Rhs: Hybrid, Smoothness: Hybrid>(
        self,
        rhs: Rhs,
        smoothness: Smoothness,
    ) -> GeneratorWrapper<MinSmooth<Lhs, Rhs, Smoothness>> {
        MinSmooth {
            lhs: self.0,
            rhs,
            smoothness,
        }
        .into()
    }

    pub fn max_smooth<Rhs: Hybrid, Smoothness: Hybrid>(
        self,
        rhs: Rhs,
        smoothness: Smoothness,
    ) -> GeneratorWrapper<MaxSmooth<Lhs, Rhs, Smoothness>> {
        MaxSmooth {
            lhs: self.0,
            rhs,
            smoothness,
        }
        .into()
    }
}

impl<A: Generator> GeneratorWrapper<A> {
    pub fn fade<B: Generator, FADE: Hybrid>(
        self,
        b: B,
        fade: FADE,
    ) -> GeneratorWrapper<Fade<A, B, FADE>> {
        Fade { a: self.0, b, fade }.into()
    }
}

impl<Value: Hybrid> GeneratorWrapper<Value> {
    pub fn powf<Pow: Hybrid>(self, pow: Pow) -> GeneratorWrapper<PowFloat<Value, Pow>> {
        PowFloat { value: self.0, pow }.into()
    }
}

impl<Value: Generator> GeneratorWrapper<Value> {
    pub fn powi(self, pow: i32) -> GeneratorWrapper<PowInt<Value>> {
        PowInt { value: self.0, pow }.into()
    }
}

impl<T: Hybrid> GeneratorWrapper<T> {
    pub fn recip(self) -> GeneratorWrapper<Divide<f32, T>> {
        Divide {
            lhs: 1.0,
            rhs: self.0,
        }
        .into()
    }
}
