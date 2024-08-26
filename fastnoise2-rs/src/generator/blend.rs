use crate::{safe::SafeNode, Node};

use super::{Generator, GeneratorWrapper, Hybrid};

#[derive(Clone, Debug)]
pub struct Add<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Subtract<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Multiply<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Divide<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Min<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct Max<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug)]
pub struct MinSmooth<Lhs, Rhs, S>
where
    Lhs: Generator,
    Rhs: Hybrid,
    S: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
    pub smoothness: S,
}

#[derive(Clone, Debug)]
pub struct MaxSmooth<Lhs, Rhs, S>
where
    Lhs: Generator,
    Rhs: Hybrid,
    S: Hybrid,
{
    pub lhs: Lhs,
    pub rhs: Rhs,
    pub smoothness: S,
}

#[derive(Clone, Debug)]
pub struct Fade<A, B, F>
where
    A: Generator,
    B: Generator,
    F: Hybrid,
{
    pub a: A,
    pub b: B,
    pub fade: F,
}

#[derive(Clone, Debug)]
pub struct PowFloat<V, P>
where
    V: Hybrid,
    P: Hybrid,
{
    pub value: V,
    pub pow: P,
}

#[derive(Clone, Debug)]
pub struct PowInt<V>
where
    V: Generator,
{
    pub value: V,
    pub pow: i32,
}

impl<Lhs, Rhs> Generator for Add<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Add").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Subtract<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Subtract").unwrap();
        node.set("LHS", self.lhs.clone()).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Multiply<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Multiply").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Divide<Lhs, Rhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Divide").unwrap();
        node.set("LHS", self.lhs.clone()).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Min<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Min").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> Generator for Max<Lhs, Rhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Max").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs, S> Generator for MinSmooth<Lhs, Rhs, S>
where
    Lhs: Generator,
    Rhs: Hybrid,
    S: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("MinSmooth").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        node.set("Smoothness", self.smoothness.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs, S> Generator for MaxSmooth<Lhs, Rhs, S>
where
    Lhs: Generator,
    Rhs: Hybrid,
    S: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("MaxSmooth").unwrap();
        node.set("LHS", &self.lhs).unwrap();
        node.set("RHS", self.rhs.clone()).unwrap();
        node.set("Smoothness", self.smoothness.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<A, B, F> Generator for Fade<A, B, F>
where
    A: Generator,
    B: Generator,
    F: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("Fade").unwrap();
        node.set("A", &self.a).unwrap();
        node.set("B", &self.b).unwrap();
        node.set("Fade", self.fade.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<V, P> Generator for PowFloat<V, P>
where
    V: Hybrid,
    P: Hybrid,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("PowFloat").unwrap();
        node.set("Value", self.value.clone()).unwrap();
        node.set("Pow", self.pow.clone()).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<V> Generator for PowInt<V>
where
    V: Generator,
{
    fn build(&self) -> GeneratorWrapper<SafeNode> {
        let mut node = Node::from_name("PowInt").unwrap();
        node.set("Value", &self.value).unwrap();
        node.set("Pow", self.pow).unwrap();
        SafeNode(node.into()).into()
    }
}

impl<Lhs, Rhs> std::ops::Add<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Add<Lhs, Rhs>>;

    fn add(self, rhs: Rhs) -> Self::Output {
        Add { lhs: self.0, rhs }.into()
    }
}

impl<Lhs, Rhs> std::ops::Sub<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Subtract<Lhs, Rhs>>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        Subtract { lhs: self.0, rhs }.into()
    }
}

impl<Lhs, Rhs> std::ops::Mul<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Generator,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Multiply<Lhs, Rhs>>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Multiply { lhs: self.0, rhs }.into()
    }
}

impl<Lhs> std::ops::Neg for GeneratorWrapper<Lhs>
where
    Lhs: Generator,
{
    type Output = GeneratorWrapper<Multiply<Lhs, f32>>;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<Lhs, Rhs> std::ops::Div<Rhs> for GeneratorWrapper<Lhs>
where
    Lhs: Hybrid,
    Rhs: Hybrid,
{
    type Output = GeneratorWrapper<Divide<Lhs, Rhs>>;

    fn div(self, rhs: Rhs) -> Self::Output {
        Divide { lhs: self.0, rhs }.into()
    }
}

impl<Lhs> GeneratorWrapper<Lhs>
where
    Lhs: Generator,
{
    pub fn min<Rhs>(self, rhs: Rhs) -> GeneratorWrapper<Min<Lhs, Rhs>>
    where
        Rhs: Hybrid,
    {
        Min { lhs: self.0, rhs }.into()
    }

    pub fn max<Rhs>(self, rhs: Rhs) -> GeneratorWrapper<Max<Lhs, Rhs>>
    where
        Rhs: Hybrid,
    {
        Max { lhs: self.0, rhs }.into()
    }

    pub fn min_smooth<Rhs, S>(
        self,
        rhs: Rhs,
        smoothness: S,
    ) -> GeneratorWrapper<MinSmooth<Lhs, Rhs, S>>
    where
        Rhs: Hybrid,
        S: Hybrid,
    {
        MinSmooth {
            lhs: self.0,
            rhs,
            smoothness,
        }
        .into()
    }

    pub fn max_smooth<Rhs, S>(
        self,
        rhs: Rhs,
        smoothness: S,
    ) -> GeneratorWrapper<MaxSmooth<Lhs, Rhs, S>>
    where
        Rhs: Hybrid,
        S: Hybrid,
    {
        MaxSmooth {
            lhs: self.0,
            rhs,
            smoothness,
        }
        .into()
    }
}

impl<A> GeneratorWrapper<A>
where
    A: Generator,
{
    pub fn fade<B, F>(self, b: B, fade: F) -> GeneratorWrapper<Fade<A, B, F>>
    where
        B: Generator,
        F: Hybrid,
    {
        Fade { a: self.0, b, fade }.into()
    }
}

impl<V> GeneratorWrapper<V>
where
    V: Hybrid,
{
    pub fn powf<P>(self, pow: P) -> GeneratorWrapper<PowFloat<V, P>>
    where
        P: Hybrid,
    {
        PowFloat { value: self.0, pow }.into()
    }
}

impl<V> GeneratorWrapper<V>
where
    V: Generator,
{
    pub fn powi(self, pow: i32) -> GeneratorWrapper<PowInt<V>> {
        PowInt { value: self.0, pow }.into()
    }
}

impl<T> GeneratorWrapper<T>
where
    T: Hybrid,
{
    pub fn recip(self) -> GeneratorWrapper<Divide<f32, T>> {
        Divide {
            lhs: 1.0,
            rhs: self.0,
        }
        .into()
    }
}
