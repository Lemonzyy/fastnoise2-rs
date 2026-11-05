//! Built nodes, generators and the dynamic node builder.
//!
//! A [`Node`] is a built FastNoise2 node: immutable, cheap to clone (cloning shares the same
//! C++ node) and safe to generate noise with. A [`Generator`] is anything that can be built
//! into a [`Node`]. A [`NodeBuilder`] creates nodes from FastNoise2 metadata names and checks
//! that every input is set before building.
mod builder;
mod generation;
#[cfg(test)]
mod test_utils;
mod value;

pub use builder::NodeBuilder;
pub use value::{Hybrid, MemberValue};

use std::{
    ffi::{CString, c_void},
    fmt,
    ptr::NonNull,
    sync::Arc,
};

use fastnoise2_sys::*;

#[cfg(feature = "encode")]
use crate::MemberType;
use crate::{
    FastNoiseError, FeatureSet,
    feature_set::max_feature_set_bits,
    metadata::{Metadata, NODE_METADATA},
};

/// Owner of a FastNoise2 node reference, released on drop.
///
/// `ptr` is a valid node reference owned by the handle from `new` until drop, the invariant every
/// FastNoise2 call on it relies on.
pub(crate) struct NodeHandle {
    pub(super) ptr: NonNull<c_void>,
    metadata_id: i32,
}

/// What is known about a node to encode it: FastNoise2 nodes can't be read back.
#[cfg(feature = "encode")]
pub(crate) enum Description {
    /// Built with a [`NodeBuilder`], from the values it set.
    Built(NodeData),
    /// Created from an encoded node tree.
    Encoded(String),
}

/// Values set on a node, like FastNoise2's `NodeData`, `None` for a member left at its default.
#[cfg(feature = "encode")]
pub(crate) struct NodeData {
    /// By variable index: the bits of a float, an int or an enum index.
    pub(crate) variables: Vec<Option<i32>>,
    /// By node lookup index.
    pub(crate) inputs: Vec<Option<Node>>,
    /// By hybrid index.
    pub(crate) hybrids: Vec<Option<Hybrid>>,
}

#[cfg(feature = "encode")]
impl NodeData {
    pub(super) fn new(metadata: &Metadata) -> Self {
        let count = |member_types: &[MemberType]| {
            metadata
                .members
                .iter()
                .filter(|member| member_types.contains(&member.member_type))
                .count()
        };

        Self {
            variables: vec![None; count(&[MemberType::Float, MemberType::Int, MemberType::Enum])],
            inputs: vec![None; count(&[MemberType::NodeLookup])],
            hybrids: vec![None; count(&[MemberType::Hybrid])],
        }
    }
}

// SAFETY: FastNoise2 node reference counts are atomic, noise generation only reads the node,
// and `GeneratorCache` keeps its cache in thread local storage.
unsafe impl Send for NodeHandle {}
unsafe impl Sync for NodeHandle {}

impl NodeHandle {
    /// # Safety
    /// `ptr` must be a node reference returned by FastNoise2 and owned by the caller.
    pub(super) unsafe fn new(ptr: *mut c_void) -> Option<Self> {
        let ptr = NonNull::new(ptr)?;

        // SAFETY: `ptr` is a valid node reference (contract of `new`)
        let metadata_id = unsafe { fnGetMetadataID(ptr.as_ptr()) };
        Some(Self { ptr, metadata_id })
    }

    #[cfg(feature = "encode")]
    #[inline]
    pub(crate) fn metadata_id(&self) -> i32 {
        self.metadata_id
    }

    #[inline]
    pub(crate) fn metadata(&self) -> &'static Metadata {
        &NODE_METADATA[self.metadata_id as usize]
    }
}

impl Drop for NodeHandle {
    fn drop(&mut self) {
        // SAFETY: the handle owns this reference, released once
        unsafe { fnDeleteNodeRef(self.ptr.as_ptr()) };
    }
}

/// A built FastNoise2 node.
///
/// Cloning a node shares the same C++ node, so a node used in several places of a tree is
/// evaluated by the same C++ node (which is what makes `GeneratorCache` effective).
#[derive(Clone)]
pub struct Node(pub(crate) Arc<NodeInner>);

pub(crate) struct NodeInner {
    pub(crate) handle: NodeHandle,
    #[cfg(feature = "encode")]
    pub(crate) description: Description,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Node")
            .field(&self.0.handle.metadata().name)
            .finish()
    }
}

impl Node {
    /// Creates a node from an encoded node tree, as exported by the FastNoise2 Node Editor.
    ///
    /// # Errors
    /// Returns an error if the encoded node tree is invalid.
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(level = "trace", err(level = "debug"))
    )]
    pub fn from_encoded_node_tree(encoded_node_tree: &str) -> Result<Self, FastNoiseError> {
        #[cfg(feature = "encode")]
        let description = Description::Encoded(encoded_node_tree.to_string());
        let encoded_node_tree = CString::new(encoded_node_tree)?;

        // SAFETY: the string is null terminated, and the feature set is one FastNoise2 is
        // compiled for (checked by `with_max_feature_set`)
        let ptr =
            unsafe { fnNewFromEncodedNodeTree(encoded_node_tree.as_ptr(), max_feature_set_bits()) };

        // SAFETY: FastNoise2 returns a new reference owned by the caller, or null
        unsafe { NodeHandle::new(ptr) }
            .map(|handle| {
                Self(Arc::new(NodeInner {
                    handle,
                    #[cfg(feature = "encode")]
                    description,
                }))
            })
            .ok_or(FastNoiseError::NodeCreationFailed)
    }

    /// Encodes the node tree in the format of the FastNoise2 Node Editor, which can load it or
    /// pass it to [`Node::from_encoded_node_tree`].
    ///
    /// ```rust
    /// use fastnoise2::prelude::*;
    ///
    /// let node = perlin().fractal_f_bm().with_octaves(5).build();
    /// let encoded = node.encode()?;
    /// let decoded = Node::from_encoded_node_tree(&encoded)?;
    /// assert_eq!(decoded.gen_single_2d(1.0, 2.0, 1337), node.gen_single_2d(1.0, 2.0, 1337));
    /// # Ok::<(), fastnoise2::FastNoiseError>(())
    /// ```
    ///
    /// # Errors
    /// Returns an error if the tree contains a node created from an encoded node tree, unless it
    /// is the node itself, or if a node is shared after more than 65536 distinct nodes.
    #[cfg(feature = "encode")]
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.name()), err(level = "debug")))]
    pub fn encode(&self) -> Result<String, FastNoiseError> {
        crate::encode::encode(self)
    }

    /// The FastNoise2 node name (e.g. "Perlin").
    pub fn name(&self) -> &'static str {
        &self.0.handle.metadata().name
    }

    /// The FastNoise2 metadata of the node type.
    pub fn metadata(&self) -> &'static Metadata {
        self.0.handle.metadata()
    }

    /// The SIMD feature set this node generates noise with.
    pub fn get_active_feature_set(&self) -> FeatureSet {
        // SAFETY: the node is valid
        let bits = unsafe { fnGetActiveFeatureSet(self.as_ptr()) };
        FeatureSet::from_bits(bits).expect("FastNoise2 nodes have a known feature set")
    }

    pub(super) fn as_ptr(&self) -> *mut c_void {
        self.0.handle.ptr.as_ptr()
    }
}

/// Anything that can be built into a [`Node`].
pub trait Generator {
    /// # Panics
    /// Typed nodes panic if an input has another SIMD feature set, built outside of the same
    /// [`with_max_feature_set`](crate::with_max_feature_set) scope.
    fn build(&self) -> Node;
}

impl Generator for Node {
    /// Shares the node, no C++ node is created.
    fn build(&self) -> Node {
        self.clone()
    }
}

impl<G: Generator + ?Sized> Generator for &G {
    fn build(&self) -> Node {
        (**self).build()
    }
}

#[cfg(test)]
mod tests {
    use super::{test_utils::*, *};
    use crate::MemberType;

    const ENCODED_NODE_TREE: &str = "E@BBZEG@BD8JFgIECArXIzwECiQIw/UoPwkuAAE@BJDQAH@BC@AIEAJBw@ABZEED0KV78YZmZmPwQDmpkZPwsAAIA/HAMAAHBCBA==";

    #[test]
    fn test_node_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Node>();
    }

    #[test]
    fn test_hybrid_value_or_node() {
        let constant = NodeBuilder::new("Add")
            .unwrap()
            .set("LHS", perlin())
            .unwrap()
            .set("RHS", Hybrid::from(1.0))
            .unwrap()
            .build()
            .unwrap();
        let node = NodeBuilder::new("Add")
            .unwrap()
            .set("LHS", perlin())
            .unwrap()
            .set("RHS", Hybrid::from(perlin()))
            .unwrap()
            .build()
            .unwrap();
        assert_ne!(grid(&constant), grid(&node));
    }

    #[test]
    fn test_build_and_hybrid_share_the_node() {
        let node = Node::from_encoded_node_tree(ENCODED_NODE_TREE).unwrap();
        assert!(Arc::ptr_eq(&node.build().0, &node.0));
        assert!(Arc::ptr_eq(&<&Node as Generator>::build(&&node).0, &node.0));
        let Hybrid::Node(hybrid) = Hybrid::from(&node) else {
            panic!("expected a node");
        };
        assert!(Arc::ptr_eq(&hybrid.0, &node.0));
        assert!(matches!(Hybrid::from(0.5), Hybrid::Value(value) if value == 0.5));
    }

    #[test]
    fn test_encoded_node_tree() {
        let node = Node::from_encoded_node_tree(ENCODED_NODE_TREE).unwrap();
        assert!(grid(&node).iter().any(|value| *value != 0.0));
        assert!(matches!(
            Node::from_encoded_node_tree("not a tree"),
            Err(FastNoiseError::NodeCreationFailed)
        ));
    }

    /// Every node with every input set must generate without crashing, whatever the values.
    #[test]
    fn test_every_node_generates_with_extreme_values() {
        let source = perlin();
        let floats = [
            0.0,
            -1.0,
            1e30,
            -1e30,
            f32::MAX,
            f32::MIN,
            f32::NAN,
            f32::INFINITY,
        ];
        // i32::MAX octaves or pow doesn't crash but takes minutes to generate
        let ints = [0, -1, 1000, i32::MIN];

        // "Domain Warp Source" inputs only accept domain warp nodes
        let domain_warp = NodeBuilder::new("DomainWarpGradient")
            .unwrap()
            .set("Source", &source)
            .unwrap()
            .build()
            .unwrap();

        for metadata in NODE_METADATA.iter() {
            let new_builder = || {
                metadata
                    .members
                    .iter()
                    .filter(|member| matches!(member.member_type, MemberType::NodeLookup))
                    .fold(
                        NodeBuilder::new(&metadata.name).unwrap(),
                        |builder, member| {
                            let input = match member.name.as_str() {
                                "Domain Warp Source" => &domain_warp,
                                _ => &source,
                            };
                            builder.set(&member.name, input).unwrap()
                        },
                    )
            };
            assert!(grid(&new_builder().build().unwrap()).len() == 64);

            for member in &metadata.members {
                let values: Vec<MemberValue> = match member.member_type {
                    MemberType::Float | MemberType::Hybrid => {
                        floats.iter().map(|&value| value.into()).collect()
                    }
                    MemberType::Int => ints.iter().map(|&value| value.into()).collect(),
                    MemberType::Enum => member
                        .enum_values
                        .iter()
                        .map(|value| value.as_str().into())
                        .collect(),
                    MemberType::NodeLookup => continue,
                };
                for value in values {
                    let node = new_builder()
                        .set(&member.name, value)
                        .unwrap()
                        .build()
                        .unwrap();
                    grid(&node);
                }
            }
        }
    }
}
