//! Built nodes, generators and the dynamic node builder.
//!
//! A [`Node`] is a built FastNoise2 node: immutable, cheap to clone (cloning shares the same
//! C++ node) and safe to generate noise with. A [`Generator`] is anything that can be built
//! into a [`Node`]. A [`NodeBuilder`] creates nodes from FastNoise2 metadata names and checks
//! that every input is set before building.
use std::{
    ffi::{CString, c_void},
    fmt,
    ptr::NonNull,
    sync::Arc,
};

use fastnoise2_sys::*;

use crate::{
    FastNoiseError, FeatureSet, MemberType, OutputMinMax,
    feature_set::max_feature_set_bits,
    metadata::{METADATA_NAME_LOOKUP, Member, Metadata, NODE_METADATA, format_lookup},
};

/// Owner of a FastNoise2 node reference, released on drop.
pub(crate) struct NodeHandle {
    ptr: NonNull<c_void>,
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
    fn new(metadata: &Metadata) -> Self {
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
    unsafe fn new(ptr: *mut c_void) -> Option<Self> {
        let ptr = NonNull::new(ptr)?;

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
    pub fn from_encoded_node_tree(encoded_node_tree: &str) -> Result<Self, FastNoiseError> {
        #[cfg(feature = "encode")]
        let description = Description::Encoded(encoded_node_tree.to_string());
        let encoded_node_tree = CString::new(encoded_node_tree)?;

        let ptr =
            unsafe { fnNewFromEncodedNodeTree(encoded_node_tree.as_ptr(), max_feature_set_bits()) };

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
        let bits = unsafe { fnGetActiveFeatureSet(self.as_ptr()) };
        FeatureSet::from_bits(bits).expect("FastNoise2 nodes have a known feature set")
    }

    fn as_ptr(&self) -> *mut c_void {
        self.0.handle.ptr.as_ptr()
    }

    /// # Panics
    /// Panics if a count is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_count * y_count`.
    pub fn gen_uniform_grid_2d(
        &self,
        noise_out: &mut [f32],
        x_offset: f32,
        y_offset: f32,
        x_count: i32,
        y_count: i32,
        x_step_size: f32,
        y_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= grid_len(&[x_count, y_count]));

        let mut min_max = [0.0; 2];

        unsafe {
            fnGenUniformGrid2D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                x_offset,
                y_offset,
                x_count,
                y_count,
                x_step_size,
                y_step_size,
                seed,
                min_max.as_mut_ptr(),
            )
        };

        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if a count is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_count * y_count * z_count`.
    pub fn gen_uniform_grid_3d(
        &self,
        noise_out: &mut [f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        x_count: i32,
        y_count: i32,
        z_count: i32,
        x_step_size: f32,
        y_step_size: f32,
        z_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= grid_len(&[x_count, y_count, z_count]));

        let mut min_max = [0.0; 2];

        unsafe {
            fnGenUniformGrid3D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                x_offset,
                y_offset,
                z_offset,
                x_count,
                y_count,
                z_count,
                x_step_size,
                y_step_size,
                z_step_size,
                seed,
                min_max.as_mut_ptr(),
            )
        };

        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if a count is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_count * y_count * z_count * w_count`.
    pub fn gen_uniform_grid_4d(
        &self,
        noise_out: &mut [f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        w_offset: f32,
        x_count: i32,
        y_count: i32,
        z_count: i32,
        w_count: i32,
        x_step_size: f32,
        y_step_size: f32,
        z_step_size: f32,
        w_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= grid_len(&[x_count, y_count, z_count, w_count]));

        let mut min_max = [0.0; 2];

        unsafe {
            fnGenUniformGrid4D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                x_offset,
                y_offset,
                z_offset,
                w_offset,
                x_count,
                y_count,
                z_count,
                w_count,
                x_step_size,
                y_step_size,
                z_step_size,
                w_step_size,
                seed,
                min_max.as_mut_ptr(),
            )
        };

        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, and `y_pos_array` are empty or do not have the same length.
    pub fn gen_position_array_2d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        check_position_arrays(noise_out, &[x_pos_array, y_pos_array]);

        let mut min_max = [0.0; 2];

        unsafe {
            fnGenPositionArray2D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                noise_out.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                x_offset,
                y_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        };

        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, `y_pos_array`, and `z_pos_array` are empty or do not have the same length.
    pub fn gen_position_array_3d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        z_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        check_position_arrays(noise_out, &[x_pos_array, y_pos_array, z_pos_array]);

        let mut min_max = [0.0; 2];

        unsafe {
            fnGenPositionArray3D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                noise_out.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                z_pos_array.as_ptr(),
                x_offset,
                y_offset,
                z_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        };

        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if `noise_out`, `x_pos_array`, `y_pos_array`, `z_pos_array` and `w_pos_array` are empty or do not have the same length.
    pub fn gen_position_array_4d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        z_pos_array: &[f32],
        w_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        w_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        check_position_arrays(
            noise_out,
            &[x_pos_array, y_pos_array, z_pos_array, w_pos_array],
        );

        let mut min_max = [0.0; 2];

        unsafe {
            fnGenPositionArray4D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                noise_out.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                z_pos_array.as_ptr(),
                w_pos_array.as_ptr(),
                x_offset,
                y_offset,
                z_offset,
                w_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        };

        OutputMinMax::new(min_max)
    }

    /// # Panics
    /// Panics if a size is not positive, if the grid size overflows an `i32`, or if `noise_out.len() < x_size * y_size`.
    pub fn gen_tileable_2d(
        &self,
        noise_out: &mut [f32],
        x_size: i32,
        y_size: i32,
        x_step_size: f32,
        y_step_size: f32,
        seed: i32,
    ) -> OutputMinMax {
        assert!(noise_out.len() >= grid_len(&[x_size, y_size]));

        let mut min_max = [0.0; 2];

        unsafe {
            fnGenTileable2D(
                self.as_ptr(),
                noise_out.as_mut_ptr(),
                x_size,
                y_size,
                x_step_size,
                y_step_size,
                seed,
                min_max.as_mut_ptr(),
            )
        };

        OutputMinMax::new(min_max)
    }

    #[inline]
    pub fn gen_single_2d(&self, x: f32, y: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle2D(self.as_ptr(), x, y, seed) }
    }

    #[inline]
    pub fn gen_single_3d(&self, x: f32, y: f32, z: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle3D(self.as_ptr(), x, y, z, seed) }
    }

    #[inline]
    pub fn gen_single_4d(&self, x: f32, y: f32, z: f32, w: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle4D(self.as_ptr(), x, y, z, w, seed) }
    }
}

/// Anything that can be built into a [`Node`].
pub trait Generator {
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

/// Value of a hybrid member: a constant or a node.
#[derive(Clone, Debug)]
pub enum Hybrid {
    Value(f32),
    Node(Node),
}

impl From<f32> for Hybrid {
    fn from(value: f32) -> Self {
        Self::Value(value)
    }
}

impl<G: Generator> From<G> for Hybrid {
    fn from(generator: G) -> Self {
        Self::Node(generator.build())
    }
}

/// Value of any member, set by name with [`NodeBuilder::set`].
#[derive(Clone, Debug)]
pub enum MemberValue {
    Float(f32),
    Int(i32),
    /// Enum value name, compared ignoring case and spaces.
    Enum(String),
    Node(Node),
}

impl MemberValue {
    fn member_type(&self) -> MemberType {
        match self {
            Self::Float(_) => MemberType::Float,
            Self::Int(_) => MemberType::Int,
            Self::Enum(_) => MemberType::Enum,
            Self::Node(_) => MemberType::NodeLookup,
        }
    }
}

impl From<f32> for MemberValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i32> for MemberValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<&str> for MemberValue {
    fn from(value: &str) -> Self {
        Self::Enum(value.to_string())
    }
}

impl From<String> for MemberValue {
    fn from(value: String) -> Self {
        Self::Enum(value)
    }
}

impl From<Hybrid> for MemberValue {
    fn from(hybrid: Hybrid) -> Self {
        match hybrid {
            Hybrid::Value(value) => Self::Float(value),
            Hybrid::Node(node) => Self::Node(node),
        }
    }
}

impl From<&Hybrid> for MemberValue {
    fn from(hybrid: &Hybrid) -> Self {
        match hybrid {
            Hybrid::Value(value) => Self::Float(*value),
            Hybrid::Node(node) => Self::Node(node.clone()),
        }
    }
}

impl<G: Generator> From<G> for MemberValue {
    fn from(generator: G) -> Self {
        Self::Node(generator.build())
    }
}

/// Builds a [`Node`] from its FastNoise2 metadata name, setting members by name.
///
/// ```rust
/// use fastnoise2::NodeBuilder;
///
/// let perlin = NodeBuilder::new("Perlin")?.set("Feature Scale", 50.0)?.build()?;
/// let fbm = NodeBuilder::new("FractalFBm")?
///     .set("Source", &perlin)?
///     .set("Octaves", 5)?
///     .set("Gain", &perlin)?
///     .build()?;
/// let value = fbm.gen_single_2d(0.0, 0.0, 1337);
/// # Ok::<(), fastnoise2::FastNoiseError>(())
/// ```
pub struct NodeBuilder {
    handle: NodeHandle,
    /// Whether each node lookup member is set, by node lookup index.
    inputs_set: Vec<bool>,
    #[cfg(feature = "encode")]
    data: NodeData,
}

impl NodeBuilder {
    /// # Errors
    /// Returns an error if `node_name` is not a FastNoise2 node name.
    pub fn new(node_name: &str) -> Result<Self, FastNoiseError> {
        let metadata_id = *METADATA_NAME_LOOKUP
            .get(&format_lookup(node_name))
            .ok_or_else(|| FastNoiseError::MetadataNameNotFound {
                expected: NODE_METADATA.iter().map(|m| m.name.clone()).collect(),
                found: node_name.to_string(),
            })?;

        let ptr = unsafe { fnNewFromMetadata(metadata_id, max_feature_set_bits()) };
        let handle = unsafe { NodeHandle::new(ptr) }.ok_or(FastNoiseError::NodeCreationFailed)?;

        let input_count = handle
            .metadata()
            .members
            .iter()
            .filter(|member| matches!(member.member_type, MemberType::NodeLookup))
            .count();

        Ok(Self {
            inputs_set: vec![false; input_count],
            #[cfg(feature = "encode")]
            data: NodeData::new(handle.metadata()),
            handle,
        })
    }

    /// Sets a member by name, ignoring case and spaces (e.g. "Feature Scale" or "featurescale").
    ///
    /// # Errors
    /// Returns an error if the member doesn't exist, if the value type doesn't match the member
    /// type, or if an enum value doesn't exist.
    pub fn set(
        mut self,
        member_name: &str,
        value: impl Into<MemberValue>,
    ) -> Result<Self, FastNoiseError> {
        let metadata = self.handle.metadata();
        let member =
            metadata
                .member(member_name)
                .ok_or_else(|| FastNoiseError::MemberNameNotFound {
                    node: metadata.name.clone(),
                    expected: metadata.members.iter().map(|m| m.name.clone()).collect(),
                    found: member_name.to_string(),
                })?;

        let ptr = self.handle.ptr.as_ptr();
        let index = member.index as usize;
        #[cfg(feature = "encode")]
        let data = &mut self.data;

        let is_set = match (member.member_type, value.into()) {
            (MemberType::Float, MemberValue::Float(value)) => {
                #[cfg(feature = "encode")]
                {
                    data.variables[index] = Some(value.to_bits() as i32);
                }
                unsafe { fnSetVariableFloat(ptr, member.index, value) }
            }
            (MemberType::Int, MemberValue::Int(value)) => {
                #[cfg(feature = "encode")]
                {
                    data.variables[index] = Some(value);
                }
                unsafe { fnSetVariableIntEnum(ptr, member.index, value) }
            }
            (MemberType::Enum, MemberValue::Enum(value)) => {
                let enum_index =
                    member
                        .enum_index(&value)
                        .ok_or_else(|| FastNoiseError::EnumValueNotFound {
                            node: metadata.name.clone(),
                            member: member.name.clone(),
                            expected: member.enum_values.clone(),
                            found: value,
                        })?;

                #[cfg(feature = "encode")]
                {
                    data.variables[index] = Some(enum_index);
                }
                unsafe { fnSetVariableIntEnum(ptr, member.index, enum_index) }
            }
            (MemberType::NodeLookup, MemberValue::Node(node)) => {
                if !unsafe { fnSetNodeLookup(ptr, member.index, node.as_ptr()) } {
                    return Err(input_not_accepted(metadata, member, &node));
                }

                self.inputs_set[index] = true;
                #[cfg(feature = "encode")]
                {
                    data.inputs[index] = Some(node);
                }
                true
            }
            (MemberType::Hybrid, MemberValue::Float(value)) => {
                #[cfg(feature = "encode")]
                {
                    data.hybrids[index] = Some(Hybrid::Value(value));
                }
                unsafe { fnSetHybridFloat(ptr, member.index, value) }
            }
            (MemberType::Hybrid, MemberValue::Node(node)) => {
                if !unsafe { fnSetHybridNodeLookup(ptr, member.index, node.as_ptr()) } {
                    return Err(input_not_accepted(metadata, member, &node));
                }

                #[cfg(feature = "encode")]
                {
                    data.hybrids[index] = Some(Hybrid::Node(node));
                }
                true
            }
            (_, value) => return Err(invalid_member_type(metadata, member, &value)),
        };

        if !is_set {
            return Err(FastNoiseError::SetMemberFailed {
                node: metadata.name.clone(),
                member: member.name.clone(),
            });
        }

        Ok(self)
    }

    /// # Errors
    /// Returns an error if an input (node lookup member) is not set, generating noise without
    /// it would crash FastNoise2.
    pub fn build(self) -> Result<Node, FastNoiseError> {
        let metadata = self.handle.metadata();
        let missing_input = metadata
            .members
            .iter()
            .filter(|member| matches!(member.member_type, MemberType::NodeLookup))
            .find(|member| !self.inputs_set[member.index as usize]);

        if let Some(member) = missing_input {
            return Err(FastNoiseError::MissingInput {
                node: metadata.name.clone(),
                member: member.name.clone(),
            });
        }

        Ok(Node(Arc::new(NodeInner {
            handle: self.handle,
            #[cfg(feature = "encode")]
            description: Description::Built(self.data),
        })))
    }
}

/// Returns the number of values in a grid, as computed by FastNoise2 (an `i32` product).
#[inline]
pub(crate) fn grid_len(counts: &[i32]) -> usize {
    assert!(
        counts.iter().all(|&count| count > 0),
        "grid counts must be positive"
    );
    counts
        .iter()
        .try_fold(1i32, |len, &count| len.checked_mul(count))
        .expect("grid size must fit in an i32") as usize
}

#[inline]
pub(crate) fn check_position_arrays(noise_out: &[f32], pos_arrays: &[&[f32]]) {
    let len = noise_out.len();
    assert!(len > 0, "position arrays must not be empty");
    assert!(len <= i32::MAX as usize, "position arrays are too long");
    assert!(
        pos_arrays.iter().all(|pos_array| pos_array.len() == len),
        "noise_out and position arrays must have the same length"
    );
}

#[cold]
fn input_not_accepted(metadata: &Metadata, member: &Member, input: &Node) -> FastNoiseError {
    FastNoiseError::InputNotAccepted {
        node: metadata.name.clone(),
        member: member.name.clone(),
        input: input.name().to_string(),
    }
}

#[cold]
fn invalid_member_type(
    metadata: &Metadata,
    member: &Member,
    value: &MemberValue,
) -> FastNoiseError {
    FastNoiseError::InvalidMemberType {
        node: metadata.name.clone(),
        member: member.name.clone(),
        description: member.description.clone(),
        expected: member.member_type,
        found: value.member_type(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENCODED_NODE_TREE: &str = "E@BBZEG@BD8JFgIECArXIzwECiQIw/UoPwkuAAE@BJDQAH@BC@AIEAJBw@ABZEED0KV78YZmZmPwQDmpkZPwsAAIA/HAMAAHBCBA==";

    fn perlin() -> Node {
        NodeBuilder::new("Perlin").unwrap().build().unwrap()
    }

    fn grid(node: &Node) -> Vec<f32> {
        let mut output = vec![0.0; 64];
        node.gen_uniform_grid_2d(&mut output, -40.0, -40.0, 8, 8, 10.0, 10.0, 1337);
        output
    }

    #[test]
    fn test_node_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Node>();
    }

    #[test]
    fn test_build_sets_members() {
        let default = perlin();
        let scaled = NodeBuilder::new("Perlin")
            .unwrap()
            .set("Feature Scale", 10.0)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(scaled.name(), "Perlin");
        assert_ne!(grid(&default), grid(&scaled));
    }

    #[test]
    fn test_inputs_and_hybrids_accept_nodes_by_reference() {
        let source = perlin();
        let fbm = NodeBuilder::new("FractalFBm")
            .unwrap()
            .set("Source", &source)
            .unwrap()
            .set("Gain", &source)
            .unwrap()
            .set("Octaves", 4)
            .unwrap()
            .build()
            .unwrap();
        assert!(grid(&fbm).iter().all(|value| value.is_finite()));
        // `source` is still usable, and building a node only shares it
        assert!(Arc::ptr_eq(&source.build().0, &source.0));
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
    #[should_panic(expected = "grid counts must be positive")]
    fn test_gen_uniform_grid_2d_zero_count() {
        perlin().gen_uniform_grid_2d(&mut [], 0.0, 0.0, 0, 4, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "grid counts must be positive")]
    fn test_gen_tileable_2d_zero_size() {
        perlin().gen_tileable_2d(&mut [], 0, 0, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "grid size must fit in an i32")]
    fn test_gen_uniform_grid_2d_overflow() {
        let mut output = vec![0.0; 65536];
        perlin().gen_uniform_grid_2d(&mut output, 0.0, 0.0, 65536, 65537, 1.0, 1.0, 1337);
    }

    #[test]
    #[should_panic(expected = "position arrays must not be empty")]
    fn test_gen_position_array_2d_empty() {
        perlin().gen_position_array_2d(&mut [], &[], &[], 0.0, 0.0, 1337);
    }

    #[test]
    #[should_panic(expected = "noise_out and position arrays must have the same length")]
    fn test_gen_position_array_4d_short_w() {
        let positions = [0.0; 4];
        perlin().gen_position_array_4d(
            &mut [0.0; 4],
            &positions,
            &positions,
            &positions,
            &positions[..1],
            0.0,
            0.0,
            0.0,
            0.0,
            1337,
        );
    }

    #[test]
    fn test_member_value_from_hybrid_reference() {
        let hybrid = Hybrid::from(perlin());
        let MemberValue::Node(node) = MemberValue::from(&hybrid) else {
            panic!("expected a node");
        };
        let Hybrid::Node(hybrid_node) = &hybrid else {
            panic!("expected a node");
        };
        assert!(Arc::ptr_eq(&node.0, &hybrid_node.0));
        assert!(
            matches!(MemberValue::from(&Hybrid::from(0.5)), MemberValue::Float(value) if value == 0.5)
        );
    }

    #[test]
    fn test_missing_input() {
        let error = NodeBuilder::new("FractalFBm").unwrap().build().unwrap_err();
        assert_eq!(
            error.to_string(),
            "missing input 'Source' of node 'FractalFBm'"
        );
    }

    #[test]
    fn test_input_not_accepted() {
        let Err(error) = NodeBuilder::new("DomainWarpFractalProgressive")
            .unwrap()
            .set("Domain Warp Source", perlin())
        else {
            panic!("expected an error");
        };
        assert_eq!(
            error.to_string(),
            "node 'Perlin' is not accepted by input 'Domain Warp Source' of node \
             'DomainWarpFractalProgressive'"
        );
    }

    #[test]
    fn test_invalid_member_type() {
        let Err(error) = NodeBuilder::new("Perlin").unwrap().set("Seed Offset", 1.5) else {
            panic!("expected an error");
        };
        assert!(error.to_string().starts_with(
            "invalid type for member 'Seed Offset' of node 'Perlin' (expected i32, found f32)"
        ));
    }

    #[test]
    fn test_enum_member() {
        let builder = NodeBuilder::new("CellularValue").unwrap();
        let builder = builder.set("Distance Function", "Manhattan").unwrap();
        let Err(error) = builder.set("Distance Function", "Euclidian") else {
            panic!("expected an error");
        };
        assert!(matches!(error, FastNoiseError::EnumValueNotFound { .. }));
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
