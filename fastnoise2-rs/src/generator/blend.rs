use crate::{safe::SafeNode, Node};

use super::{Generator, Hybrid, TypedNode};

#[derive(Clone, Copy, Debug)]
pub struct Add<LHS: TypedNode, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy, Debug)]
pub struct Subtract<LHS: Hybrid, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy, Debug)]
pub struct Multiply<LHS: TypedNode, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy, Debug)]
pub struct Divide<LHS: Hybrid, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy, Debug)]
pub struct Min<LHS: TypedNode, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy, Debug)]
pub struct Max<LHS: TypedNode, RHS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
}

#[derive(Clone, Copy, Debug)]
pub struct MinSmooth<LHS: TypedNode, RHS: Hybrid, SMOOTHNESS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
    pub smoothness: SMOOTHNESS,
}

#[derive(Clone, Copy, Debug)]
pub struct MaxSmooth<LHS: TypedNode, RHS: Hybrid, SMOOTHNESS: Hybrid> {
    pub lhs: LHS,
    pub rhs: RHS,
    pub smoothness: SMOOTHNESS,
}

#[derive(Clone, Copy, Debug)]
pub struct Fade<A: TypedNode, B: TypedNode, FADE: Hybrid> {
    pub a: A,
    pub b: B,
    pub fade: FADE,
}

#[derive(Clone, Copy, Debug)]
pub struct PowFloat<VALUE: Hybrid, POW: Hybrid> {
    pub value: VALUE,
    pub pow: POW,
}

#[derive(Clone, Copy, Debug)]
pub struct PowInt<VALUE: TypedNode> {
    pub value: VALUE,
    pub pow: i32,
}

impl<LHS: TypedNode, RHS: Hybrid> TypedNode for Add<LHS, RHS> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Add").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        SafeNode(node)
    }
}

impl<LHS: Hybrid, RHS: Hybrid> TypedNode for Subtract<LHS, RHS> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Subtract").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        SafeNode(node)
    }
}

impl<LHS: TypedNode, RHS: Hybrid> TypedNode for Multiply<LHS, RHS> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Multiply").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        SafeNode(node)
    }
}

impl<LHS: Hybrid, RHS: Hybrid> TypedNode for Divide<LHS, RHS> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Divide").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        SafeNode(node)
    }
}

impl<LHS: TypedNode, RHS: Hybrid> TypedNode for Min<LHS, RHS> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Min").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        SafeNode(node)
    }
}

impl<LHS: TypedNode, RHS: Hybrid> TypedNode for Max<LHS, RHS> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Max").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        SafeNode(node)
    }
}

impl<LHS: TypedNode, RHS: Hybrid, SMOOTHNESS: Hybrid> TypedNode
    for MinSmooth<LHS, RHS, SMOOTHNESS>
{
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("MinSmooth").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node.set("Smoothness", self.smoothness).unwrap();
        SafeNode(node)
    }
}

impl<LHS: TypedNode, RHS: Hybrid, SMOOTHNESS: Hybrid> TypedNode
    for MaxSmooth<LHS, RHS, SMOOTHNESS>
{
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("MaxSmooth").unwrap();
        node.set("LHS", self.lhs).unwrap();
        node.set("RHS", self.rhs).unwrap();
        node.set("Smoothness", self.smoothness).unwrap();
        SafeNode(node)
    }
}

impl<A: TypedNode, B: TypedNode, FADE: Hybrid> TypedNode for Fade<A, B, FADE> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("Fade").unwrap();
        node.set("A", self.a).unwrap();
        node.set("B", self.b).unwrap();
        node.set("Fade", self.fade).unwrap();
        SafeNode(node)
    }
}

impl<VALUE: Hybrid, POW: Hybrid> TypedNode for PowFloat<VALUE, POW> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("PowFloat").unwrap();
        node.set("Value", self.value).unwrap();
        node.set("Pow", self.pow).unwrap();
        SafeNode(node)
    }
}

impl<VALUE: TypedNode> TypedNode for PowInt<VALUE> {
    fn build_node(&self) -> SafeNode {
        let mut node = Node::from_name("PowInt").unwrap();
        node.set("Value", self.value).unwrap();
        node.set("Pow", self.pow).unwrap();
        SafeNode(node)
    }
}

impl<Lhs: TypedNode, Rhs: Hybrid> std::ops::Add<Rhs> for Generator<Lhs> {
    type Output = Generator<Add<Lhs, Rhs>>;

    fn add(self, rhs: Rhs) -> Self::Output {
        Generator(Add { lhs: self.0, rhs })
    }
}

impl<Lhs: Hybrid, Rhs: Hybrid> std::ops::Sub<Rhs> for Generator<Lhs> {
    type Output = Generator<Subtract<Lhs, Rhs>>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        Generator(Subtract { lhs: self.0, rhs })
    }
}

impl<Lhs: TypedNode, Rhs: Hybrid> std::ops::Mul<Rhs> for Generator<Lhs> {
    type Output = Generator<Multiply<Lhs, Rhs>>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Generator(Multiply { lhs: self.0, rhs })
    }
}

impl<Lhs: Hybrid, Rhs: Hybrid> std::ops::Div<Rhs> for Generator<Lhs> {
    type Output = Generator<Divide<Lhs, Rhs>>;

    fn div(self, rhs: Rhs) -> Self::Output {
        Generator(Divide { lhs: self.0, rhs })
    }
}

impl<Lhs: TypedNode> Generator<Lhs> {
    pub fn min<Rhs: Hybrid>(self, rhs: Rhs) -> Generator<Min<Lhs, Rhs>> {
        Generator(Min { lhs: self.0, rhs })
    }

    pub fn max<Rhs: Hybrid>(self, rhs: Rhs) -> Generator<Max<Lhs, Rhs>> {
        Generator(Max { lhs: self.0, rhs })
    }

    pub fn min_smooth<Rhs: Hybrid, Smoothness: Hybrid>(
        self,
        rhs: Rhs,
        smoothness: Smoothness,
    ) -> Generator<MinSmooth<Lhs, Rhs, Smoothness>> {
        Generator(MinSmooth {
            lhs: self.0,
            rhs,
            smoothness,
        })
    }

    pub fn max_smooth<Rhs: Hybrid, Smoothness: Hybrid>(
        self,
        rhs: Rhs,
        smoothness: Smoothness,
    ) -> Generator<MaxSmooth<Lhs, Rhs, Smoothness>> {
        Generator(MaxSmooth {
            lhs: self.0,
            rhs,
            smoothness,
        })
    }
}

impl<A: TypedNode> Generator<A> {
    pub fn fade<B: TypedNode, FADE: Hybrid>(self, b: B, fade: FADE) -> Generator<Fade<A, B, FADE>> {
        Generator(Fade { a: self.0, b, fade })
    }
}

impl<Value: Hybrid> Generator<Value> {
    pub fn powf<Pow: Hybrid>(self, pow: Pow) -> Generator<PowFloat<Value, Pow>> {
        Generator(PowFloat { value: self.0, pow })
    }
}

impl<Value: TypedNode> Generator<Value> {
    pub fn powi(self, pow: i32) -> Generator<PowInt<Value>> {
        Generator(PowInt { value: self.0, pow })
    }
}
