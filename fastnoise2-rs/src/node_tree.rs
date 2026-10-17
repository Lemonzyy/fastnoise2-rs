//! Node trees as data, decoded from the format of the FastNoise2 Node Editor.
//!
//! This is a port of `Metadata::DeserialiseNodeData` and `Base64::Decode` of FastNoise2: unlike
//! [`Node::from_encoded_node_tree`], it gives the node types and values of the tree, to show or
//! edit them, and [`NodeTree::build`] builds them back into a [`Node`].
use base64::{
    Engine, alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};

use crate::{
    FastNoiseError, MemberType, MemberValue, Node, NodeBuilder,
    metadata::{Member, Metadata, NODE_METADATA},
};

/// Written in place of a node id for a node already decoded, followed by its reference id.
const REFERENCE: u8 = u8::MAX;

/// Like FastNoise2's decoder: padding is optional, and trailing bits are ignored.
const BASE64: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new()
        .with_decode_padding_mode(DecodePaddingMode::Indifferent)
        .with_decode_allow_trailing_bits(true),
);

/// A node tree as data: the node types and member values the Node Editor encodes.
///
/// Nodes are listed inputs first: a node only refers to nodes before it, and the root is the
/// last one. A node used by several others is listed once.
///
/// ```rust
/// use fastnoise2::{NodeTree, TreeValue, prelude::*};
///
/// let encoded = perlin().with_feature_scale(50.0).fractal_f_bm().build().encode()?;
/// let tree = NodeTree::decode(&encoded)?;
///
/// let [perlin, fbm] = tree.nodes() else { unreachable!() };
/// assert_eq!(perlin.metadata().name(), "Perlin");
/// assert_eq!(perlin.value("Feature Scale"), Some(&TreeValue::Float(50.0)));
/// assert_eq!(fbm.value("Source"), Some(&TreeValue::Node(Some(0))));
///
/// let node = tree.build()?;
/// # Ok::<(), fastnoise2::FastNoiseError>(())
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct NodeTree {
    nodes: Vec<TreeNode>,
}

/// A node of a [`NodeTree`]: its type and the value of every member.
#[derive(Clone, Debug)]
pub struct TreeNode {
    metadata: &'static Metadata,
    /// By member, in the order of [`Metadata::members`].
    values: Vec<TreeValue>,
}

/// Value of a member of a [`TreeNode`].
#[derive(Clone, Debug, PartialEq)]
pub enum TreeValue {
    /// A float, or the constant of a hybrid member.
    Float(f32),
    Int(i32),
    /// Index in [`Member::enum_values`].
    Enum(usize),
    /// Index of the input node in [`NodeTree::nodes`], for inputs and hybrid members. `None` for
    /// an input the encoded tree doesn't set, which [`NodeTree::build`] can't build.
    Node(Option<usize>),
}

impl NodeTree {
    /// Decodes an encoded node tree, as exported by the FastNoise2 Node Editor or
    /// [`Node::encode`].
    ///
    /// # Errors
    /// Returns an error if the encoded node tree is invalid.
    pub fn decode(encoded_node_tree: &str) -> Result<Self, FastNoiseError> {
        let bytes = BASE64
            .decode(decompress_as(encoded_node_tree)?)
            .map_err(|_| invalid("not base64"))?;
        let mut stream = Stream {
            bytes: &bytes,
            position: 0,
            pending_node_ends: 0,
        };
        let mut tree = Self { nodes: Vec::new() };
        let root = tree.decode_node(&mut stream)?;
        // The root is the last node decoded, unless it is a reference
        if root + 1 != tree.nodes.len() {
            return Err(invalid("the root is a reference"));
        }

        Ok(tree)
    }

    /// The nodes of the tree, inputs first, the root last.
    pub fn nodes(&self) -> &[TreeNode] {
        &self.nodes
    }

    /// The root node: the one the tree generates noise with.
    pub fn root(&self) -> &TreeNode {
        self.nodes.last().expect("a node tree has a root")
    }

    /// Builds the node tree, each node once even if several others use it.
    ///
    /// # Errors
    /// Returns an error if an input isn't set, or isn't accepted by its node.
    pub fn build(&self) -> Result<Node, FastNoiseError> {
        let mut built: Vec<Node> = Vec::with_capacity(self.nodes.len());
        for node in &self.nodes {
            let mut builder = NodeBuilder::new(node.metadata.name())?;
            for (member, value) in node.metadata.members().iter().zip(&node.values) {
                let value = match value {
                    TreeValue::Float(value) => MemberValue::Float(*value),
                    TreeValue::Int(value) => MemberValue::Int(*value),
                    TreeValue::Enum(index) => {
                        MemberValue::Enum(member.enum_values()[*index].clone())
                    }
                    TreeValue::Node(Some(input)) => MemberValue::Node(built[*input].clone()),
                    // Reported as a missing input when building
                    TreeValue::Node(None) => continue,
                };
                builder = builder.set(member.name(), value)?;
            }
            built.push(builder.build()?);
        }

        Ok(built.pop().expect("a node tree has a root"))
    }

    /// Decodes a node and its inputs, returns its index.
    fn decode_node(&mut self, stream: &mut Stream) -> Result<usize, FastNoiseError> {
        let node_id = stream.byte()?;
        if node_id == REFERENCE {
            let reference = u16::from_le_bytes(stream.array()?) as usize;
            return if reference < self.nodes.len() {
                Ok(reference)
            } else {
                Err(invalid("reference to a node not decoded yet"))
            };
        }

        let metadata = NODE_METADATA
            .get(node_id as usize)
            .ok_or_else(|| invalid("unknown node id"))?;
        let mut values = metadata
            .members()
            .iter()
            .map(default_value)
            .collect::<Vec<_>>();
        let position = |member_types: &[MemberType], index: usize| {
            metadata.members().iter().position(|member| {
                member_types.contains(&member.member_type()) && member.index as usize == index
            })
        };

        let mut lookup = stream.lookup()?;

        while let Lookup::Variable(index) = lookup {
            let bits = i32::from_le_bytes(stream.array()?);
            // Members FastNoise2 no longer has are skipped, like FastNoise2 does
            let variables = [MemberType::Float, MemberType::Int, MemberType::Enum];
            if let Some(position) = position(&variables, index) {
                let member = &metadata.members()[position];
                values[position] = match member.member_type() {
                    MemberType::Float => TreeValue::Float(f32::from_bits(bits as u32)),
                    MemberType::Int => TreeValue::Int(bits),
                    _ => TreeValue::Enum(
                        usize::try_from(bits)
                            .ok()
                            .filter(|&index| index < member.enum_values().len())
                            .ok_or_else(|| invalid("unknown enum value"))?,
                    ),
                };
            }
            lookup = stream.lookup()?;
        }

        if let Lookup::Inputs(count) = lookup {
            for index in 0..count {
                let input = self.decode_node(stream)?;
                if let Some(position) = position(&[MemberType::NodeLookup], index) {
                    values[position] = TreeValue::Node(Some(input));
                }
            }
            lookup = stream.lookup()?;
        }

        loop {
            match lookup {
                Lookup::HybridNode(index) => {
                    let input = self.decode_node(stream)?;
                    if let Some(position) = position(&[MemberType::Hybrid], index) {
                        values[position] = TreeValue::Node(Some(input));
                    }
                }
                Lookup::HybridValue(index) => {
                    let value = f32::from_le_bytes(stream.array()?);
                    if let Some(position) = position(&[MemberType::Hybrid], index) {
                        values[position] = TreeValue::Float(value);
                    }
                }
                Lookup::NodeEnd => break,
                Lookup::Variable(_) | Lookup::Inputs(_) => {
                    return Err(invalid("members out of order"));
                }
            }
            lookup = stream.lookup()?;
        }

        self.nodes.push(TreeNode { metadata, values });
        Ok(self.nodes.len() - 1)
    }
}

impl TreeNode {
    /// The FastNoise2 metadata of the node type.
    pub fn metadata(&self) -> &'static Metadata {
        self.metadata
    }

    /// The value of every member, in the order of [`Metadata::members`].
    pub fn values(&self) -> &[TreeValue] {
        &self.values
    }

    /// The value of a member by name, ignoring case and spaces.
    pub fn value(&self, member_name: &str) -> Option<&TreeValue> {
        let member = self.metadata.member(member_name)?;
        let position = self
            .metadata
            .members()
            .iter()
            .position(|other| std::ptr::eq(other, member))?;
        self.values.get(position)
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        // Metadata is static: one per node type
        std::ptr::eq(self.metadata, other.metadata) && self.values == other.values
    }
}

/// The default value of a member, unset for inputs.
fn default_value(member: &Member) -> TreeValue {
    match member.default_value() {
        Some(MemberValue::Float(value)) => TreeValue::Float(value),
        Some(MemberValue::Int(value)) => TreeValue::Int(value),
        Some(MemberValue::Enum(_)) => TreeValue::Enum(member.default_bits as usize),
        Some(MemberValue::Node(_)) | None => TreeValue::Node(None),
    }
}

fn invalid(reason: &'static str) -> FastNoiseError {
    FastNoiseError::InvalidNodeTree { reason }
}

/// A member lookup byte: 3 bits of type, then 5 bits of index.
enum Lookup {
    Variable(usize),
    /// The number of inputs that follow.
    Inputs(usize),
    HybridNode(usize),
    HybridValue(usize),
    NodeEnd,
}

struct Stream<'a> {
    bytes: &'a [u8],
    position: usize,
    /// Node ends still to read from the last node end lookup, which groups up to 32.
    pending_node_ends: u8,
}

impl Stream<'_> {
    fn byte(&mut self) -> Result<u8, FastNoiseError> {
        Ok(self.array::<1>()?[0])
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], FastNoiseError> {
        // Like FastNoise2, a value can't come before the node ends of a group are all read
        if self.pending_node_ends > 0 {
            return Err(invalid("value inside a group of node ends"));
        }
        let bytes = self
            .bytes
            .get(self.position..self.position + N)
            .ok_or_else(|| invalid("truncated"))?;
        self.position += N;
        Ok(bytes.try_into().expect("N bytes"))
    }

    fn lookup(&mut self) -> Result<Lookup, FastNoiseError> {
        if self.pending_node_ends > 0 {
            self.pending_node_ends -= 1;
            return Ok(Lookup::NodeEnd);
        }
        let byte = self.byte()?;
        let index = (byte >> 3) as usize;
        Ok(match byte & 0b111 {
            0 => Lookup::Variable(index),
            1 => Lookup::Inputs(index),
            2 => Lookup::HybridNode(index),
            3 => Lookup::HybridValue(index),
            4 => {
                self.pending_node_ends = index as u8;
                Lookup::NodeEnd
            }
            _ => return Err(invalid("unknown member lookup")),
        })
    }
}

/// Expands the runs of 'A' compressed by FastNoise2: '@' followed by a base64 digit `d` stands
/// for `d + 3` 'A'.
fn decompress_as(encoded: &str) -> Result<String, FastNoiseError> {
    let digits = alphabet::STANDARD.as_str();
    let mut out = String::with_capacity(encoded.len());
    let mut chars = encoded.chars();
    while let Some(c) = chars.next() {
        if c == '@' {
            let run = chars
                .next()
                .and_then(|digit| digits.find(digit))
                .ok_or_else(|| invalid("'@' not followed by a base64 digit"))?;
            out.extend(std::iter::repeat_n('A', run + 3));
        } else {
            out.push(c);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(node_name: &str, members: &[(&str, MemberValue)]) -> Node {
        members
            .iter()
            .fold(
                NodeBuilder::new(node_name).unwrap(),
                |builder, (name, value)| builder.set(name, value.clone()).unwrap(),
            )
            .build()
            .unwrap()
    }

    fn grid(node: &Node) -> Vec<f32> {
        let mut output = vec![0.0; 64];
        node.gen_uniform_grid_2d(&mut output, -40.0, -40.0, 8, 8, 10.0, 10.0, 1337);
        output
    }

    /// Decodes the encoded tree, builds it and checks it generates the same noise as FastNoise2's
    /// decoding.
    fn assert_same_noise(encoded: &str) -> NodeTree {
        let tree = NodeTree::decode(encoded)
            .unwrap_or_else(|error| panic!("{encoded} doesn't decode: {error}"));
        let built = tree.build().unwrap();
        let reference = Node::from_encoded_node_tree(encoded).unwrap();
        let same = grid(&built)
            .iter()
            .zip(grid(&reference))
            .all(|(a, b)| *a == b || (a.is_nan() && b.is_nan()));
        assert!(same, "{encoded} decodes to {tree:?}, which differs");
        tree
    }

    #[test]
    fn test_node_editor_tree() {
        // From the crate's documentation, exported by the Node Editor
        let encoded = "E@BBZEG@BD8JFgIECArXIzwECiQIw/UoPwkuAAE@BJDQAH@BC@AIEAJBw@ABZEED0KV78YZmZmPwQDmpkZPwsAAIA/HAMAAHBCBA==";
        let tree = assert_same_noise(encoded);
        assert!(tree.nodes().len() > 2);
        // Built back, the tree encodes to the same values
        let encoded_again = tree.build().unwrap().encode().unwrap();
        assert_eq!(NodeTree::decode(&encoded_again).unwrap(), tree);
    }

    #[test]
    fn test_values_and_shared_nodes() {
        let perlin = build("Perlin", &[("Feature Scale", MemberValue::Float(42.0))]);
        let ridged = build(
            "FractalRidged",
            &[
                ("Source", MemberValue::from(&perlin)),
                ("Octaves", MemberValue::Int(6)),
                ("Gain", MemberValue::from(&perlin)),
            ],
        );
        let add = build(
            "Add",
            &[
                ("LHS", MemberValue::from(&perlin)),
                ("RHS", MemberValue::from(&ridged)),
            ],
        );
        let tree = assert_same_noise(&add.encode().unwrap());

        let [perlin, ridged, add] = tree.nodes() else {
            panic!("{tree:?} should have 3 nodes");
        };
        assert_eq!(perlin.value("Feature Scale"), Some(&TreeValue::Float(42.0)));
        assert_eq!(ridged.value("Source"), Some(&TreeValue::Node(Some(0))));
        assert_eq!(ridged.value("Octaves"), Some(&TreeValue::Int(6)));
        assert_eq!(ridged.value("Gain"), Some(&TreeValue::Node(Some(0))));
        // Left at its default
        assert_eq!(ridged.value("Lacunarity"), Some(&TreeValue::Float(2.0)));
        assert_eq!(add.value("LHS"), Some(&TreeValue::Node(Some(0))));
        assert_eq!(add.value("RHS"), Some(&TreeValue::Node(Some(1))));
        assert!(std::ptr::eq(tree.root(), add));
    }

    #[test]
    fn test_enums() {
        let node = build(
            "CellularValue",
            &[("Distance Function", MemberValue::from("Manhattan"))],
        );
        let tree = assert_same_noise(&node.encode().unwrap());
        let member = tree.root().metadata().member("Distance Function").unwrap();
        let Some(&TreeValue::Enum(index)) = tree.root().value("Distance Function") else {
            panic!("{tree:?}");
        };
        assert_eq!(member.enum_values()[index], "Manhattan");
    }

    /// More than 32 nested nodes: node ends come in several groups.
    #[test]
    fn test_deep_tree() {
        let node = (0..40).fold(build("Perlin", &[]), |node, _| {
            build(
                "DomainOffset",
                &[
                    ("Source", MemberValue::from(node)),
                    ("Offset X", MemberValue::Float(1.0)),
                ],
            )
        });
        let tree = assert_same_noise(&node.encode().unwrap());
        assert_eq!(tree.nodes().len(), 41);
    }

    #[test]
    fn test_missing_input_does_not_build() {
        let perlin = build("Perlin", &[]);
        let fade = build(
            "Fade",
            &[
                ("A", MemberValue::from(&perlin)),
                ("B", MemberValue::from(&perlin)),
            ],
        );
        let mut tree = NodeTree::decode(&fade.encode().unwrap()).unwrap();
        let fade = tree.nodes.last_mut().unwrap();
        let a = fade
            .metadata
            .members()
            .iter()
            .position(|m| m.name() == "A")
            .unwrap();
        fade.values[a] = TreeValue::Node(None);
        assert!(matches!(
            tree.build(),
            Err(FastNoiseError::MissingInput { .. })
        ));
    }

    #[test]
    fn test_invalid_trees() {
        for encoded in ["", "@", "!!!!", "AAAA", "/w=="] {
            assert!(
                matches!(
                    NodeTree::decode(encoded),
                    Err(FastNoiseError::InvalidNodeTree { .. })
                ),
                "{encoded} should be invalid"
            );
        }
    }
}
