//! Encoding of node trees in the format of the FastNoise2 Node Editor.
//!
//! This is a port of `Metadata::SerialiseNodeData` and `Base64::Encode` of FastNoise2, which
//! only work on the Node Editor's node data: FastNoise2 nodes can't be read back, so the values
//! are taken from the [`Description`] recorded when building the node.
use std::{collections::HashMap, sync::Arc};

use base64::{Engine, alphabet, engine::general_purpose::STANDARD};

use crate::{
    FastNoiseError, Hybrid, MemberType,
    node::{Description, Node, NodeInner},
};

/// Written in place of a node id for a node already encoded, followed by its reference id.
const REFERENCE: u8 = u8::MAX;

/// Number of consecutive node ends stored in a single node end lookup.
const MAX_NODE_ENDS: u8 = 32;

/// Type of a member lookup byte: 3 bits of type, then 5 bits of index.
#[derive(Clone, Copy)]
enum Lookup {
    Variable = 0,
    Inputs = 1,
    HybridNode = 2,
    HybridValue = 3,
    NodeEnd = 4,
}

#[derive(Default)]
struct DataStream {
    bytes: Vec<u8>,
    pending_node_ends: u8,
}

impl DataStream {
    fn push(&mut self, bytes: &[u8]) {
        self.flush_node_ends();
        self.bytes.extend_from_slice(bytes);
    }

    fn push_lookup(&mut self, lookup: Lookup, index: usize) {
        assert!(
            index < 32,
            "member lookup index {index} doesn't fit in 5 bits"
        );

        self.push(&[lookup as u8 | (index as u8) << 3]);
    }

    /// Node ends are only written before the next value, grouped by up to 32.
    fn node_end(&mut self) {
        self.pending_node_ends += 1;

        if self.pending_node_ends == MAX_NODE_ENDS {
            self.flush_node_ends();
        }
    }

    fn flush_node_ends(&mut self) {
        if self.pending_node_ends > 0 {
            let count = self.pending_node_ends;
            self.pending_node_ends = 0;

            self.bytes.push(Lookup::NodeEnd as u8 | (count - 1) << 3);
        }
    }
}

/// Encodes a node tree, see [`Node::encode`].
pub(crate) fn encode(node: &Node) -> Result<String, FastNoiseError> {
    if let Description::Encoded(encoded) = &node.0.description {
        return Ok(encoded.clone());
    }

    let mut stream = DataStream::default();
    encode_node(node, &mut stream, &mut HashMap::new())?;
    stream.flush_node_ends();

    Ok(compress_as(&STANDARD.encode(&stream.bytes)))
}

/// Encodes a node and its inputs, nodes already encoded are written as references.
fn encode_node(
    node: &Node,
    stream: &mut DataStream,
    references: &mut HashMap<*const NodeInner, usize>,
) -> Result<(), FastNoiseError> {
    let key = Arc::as_ptr(&node.0);
    if let Some(&reference) = references.get(&key) {
        // FastNoise2 truncates the reference id, which would reference another node
        let reference = u16::try_from(reference).map_err(|_| FastNoiseError::TooManyNodes)?;

        stream.push(&[REFERENCE]);
        stream.push(&reference.to_le_bytes());

        return Ok(());
    }

    let metadata = node.0.handle.metadata();
    let Description::Built(data) = &node.0.description else {
        return Err(FastNoiseError::NotEncodable {
            node: metadata.name.clone(),
        });
    };

    let node_id =
        u8::try_from(node.0.handle.metadata_id()).expect("FastNoise2 node ids fit in a byte");
    stream.push(&[node_id]);

    // Members are in FastNoise2 order: variables, node lookups, then hybrids. Only values
    // different from the default are written.
    for member in &metadata.members {
        let index = member.index as usize;
        match member.member_type {
            MemberType::Float | MemberType::Int | MemberType::Enum => {
                let changed = data.variables[index].filter(|&value| value != member.default_bits);
                if let Some(value) = changed {
                    stream.push_lookup(Lookup::Variable, index);
                    stream.push(&value.to_le_bytes());
                }
            }
            MemberType::NodeLookup => {
                if index == 0 {
                    stream.push_lookup(Lookup::Inputs, data.inputs.len());
                }

                let input = data.inputs[index]
                    .as_ref()
                    .expect("a built node has every input set");
                encode_node(input, stream, references)?;
            }
            MemberType::Hybrid => match &data.hybrids[index] {
                None => {}
                // Compared as floats, like FastNoise2
                Some(Hybrid::Value(value)) => {
                    if *value != f32::from_bits(member.default_bits as u32) {
                        stream.push_lookup(Lookup::HybridValue, index);
                        stream.push(&value.to_bits().to_le_bytes());
                    }
                }
                Some(Hybrid::Node(input)) => {
                    stream.push_lookup(Lookup::HybridNode, index);
                    encode_node(input, stream, references)?;
                }
            },
        }
    }

    stream.node_end();

    references.insert(key, references.len());

    Ok(())
}

/// Compresses runs of 'A' (zero bits) of a base64 string like FastNoise2: 3 or more are written
/// as '@' followed by the run length minus 3, as a base64 digit.
///
/// Like FastNoise2, a run reaching 66 'A' is written as '@/' (66 'A') followed by an extra 'A',
/// which decodes to one 'A' too many. It can't happen with a node tree: the longest run of zero
/// bytes is a reference to node 0 followed by a node with id 0 setting its first variable to 0
/// (8 bytes, about 11 'A'), every other byte of the format is not zero.
fn compress_as(base64: &str) -> String {
    let alphabet = alphabet::STANDARD;
    let digits = alphabet.as_str().as_bytes();

    let mut out = String::with_capacity(base64.len());
    let mut consecutive_as = 0;

    let end_run = |out: &mut String, consecutive_as: usize| {
        out.truncate(out.len() - 2);
        out.push('@');
        out.push(digits[consecutive_as - 3] as char);
    };

    // FastNoise2 flushes a trailing run with a null character when there is no padding
    let flush = (!base64.ends_with('=')).then_some(None);
    for c in base64.chars().map(Some).chain(flush) {
        if c == Some('A') {
            consecutive_as += 1;

            if consecutive_as <= 2 {
                out.push('A');
            } else if consecutive_as >= digits.len() + 2 {
                end_run(&mut out, consecutive_as);
                out.push('A');
                consecutive_as = 1;
            }
        } else {
            if consecutive_as >= 3 {
                end_run(&mut out, consecutive_as);
            }
            if let Some(c) = c {
                out.push(c);
            }

            consecutive_as = 0;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MemberValue, NodeBuilder,
        metadata::{Metadata, NODE_METADATA},
    };

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

    fn perlin() -> Node {
        build("Perlin", &[])
    }

    fn grid(node: &Node) -> Vec<f32> {
        let mut output = vec![0.0; 64];
        node.gen_uniform_grid_2d(&mut output, -40.0, -40.0, 8, 8, 10.0, 10.0, 1337);
        output
    }

    /// Encodes the node, decodes it with FastNoise2, and checks both generate the same noise.
    fn assert_round_trip(node: &Node) -> String {
        let encoded = node.encode().unwrap();
        let decoded = Node::from_encoded_node_tree(&encoded)
            .unwrap_or_else(|error| panic!("{encoded} doesn't decode: {error}"));

        let same = grid(node)
            .iter()
            .zip(grid(&decoded))
            .all(|(node, decoded)| *node == decoded || (node.is_nan() && decoded.is_nan()));
        assert!(
            same,
            "{node:?} encoded as {encoded} decodes to different noise"
        );

        encoded
    }

    /// Only the inputs of a node, every other member is left at its default.
    fn inputs<'a>(
        metadata: &'a Metadata,
        source: &Node,
        domain_warp: &Node,
    ) -> Vec<(&'a str, MemberValue)> {
        metadata
            .members
            .iter()
            .filter(|member| member.member_type == MemberType::NodeLookup)
            .map(|member| {
                let input = if member.name == "Domain Warp Source" {
                    domain_warp
                } else {
                    source
                };
                (member.name.as_str(), MemberValue::from(input))
            })
            .collect::<Vec<_>>()
    }

    /// Every node, with every input set and every other member set to a non default value.
    #[test]
    fn test_every_node_round_trips() {
        let source = perlin();
        let domain_warp = build(
            "DomainWarpGradient",
            &[("Source", MemberValue::from(&source))],
        );
        let hybrid_node = build("SineWave", &[]);

        for metadata in NODE_METADATA.iter() {
            for hybrids_as_nodes in [false, true] {
                let members = metadata
                    .members
                    .iter()
                    .map(|member| {
                        let value = match member.member_type {
                            MemberType::Float => {
                                MemberValue::Float(f32::from_bits(member.default_bits as u32) + 1.5)
                            }
                            MemberType::Int => MemberValue::Int(member.default_bits + 1),
                            MemberType::Enum => {
                                let index =
                                    (member.default_bits as usize + 1) % member.enum_values.len();
                                MemberValue::from(member.enum_values[index].as_str())
                            }
                            MemberType::NodeLookup if member.name == "Domain Warp Source" => {
                                MemberValue::from(&domain_warp)
                            }
                            MemberType::NodeLookup => MemberValue::from(&source),
                            MemberType::Hybrid if hybrids_as_nodes => {
                                MemberValue::from(&hybrid_node)
                            }
                            MemberType::Hybrid => {
                                MemberValue::Float(f32::from_bits(member.default_bits as u32) + 1.5)
                            }
                        };
                        (member.name.as_str(), value)
                    })
                    .collect::<Vec<_>>();

                assert_round_trip(&build(&metadata.name, &members));
            }

            // Every member left at its default
            assert_round_trip(&build(
                &metadata.name,
                &inputs(metadata, &source, &domain_warp),
            ));
        }
    }

    #[test]
    fn test_shared_node_is_encoded_as_a_reference() {
        // A reference takes 3 bytes, more than a node left at its defaults
        let fractal = || {
            build(
                "FractalFBm",
                &[
                    ("Source", MemberValue::from(perlin())),
                    ("Octaves", MemberValue::Int(5)),
                    ("Gain", MemberValue::Float(0.25)),
                ],
            )
        };

        let shared = fractal();
        let shared_twice = build(
            "Fade",
            &[
                ("A", MemberValue::from(&shared)),
                ("B", MemberValue::from(&shared)),
            ],
        );
        let distinct = build(
            "Fade",
            &[
                ("A", MemberValue::from(fractal())),
                ("B", MemberValue::from(fractal())),
            ],
        );

        let shared_twice = assert_round_trip(&shared_twice);
        let distinct = assert_round_trip(&distinct);
        assert!(
            shared_twice.len() < distinct.len(),
            "{shared_twice} vs {distinct}"
        );
    }

    /// More than 32 nested nodes, so the node ends are written in several groups.
    #[test]
    fn test_deep_tree_round_trips() {
        let node = (0..40).fold(perlin(), |node, _| {
            build(
                "DomainOffset",
                &[
                    ("Source", MemberValue::from(node)),
                    ("Offset X", MemberValue::Float(1.0)),
                ],
            )
        });

        assert_round_trip(&node);
    }

    /// A reference to a node encoded after 65536 others doesn't fit in the format.
    #[test]
    fn test_too_many_nodes() {
        let add = |lhs: &Node, rhs: &Node| {
            build(
                "Add",
                &[
                    ("LHS", MemberValue::from(lhs)),
                    ("RHS", MemberValue::from(rhs)),
                ],
            )
        };

        // A balanced tree of 2^17 - 1 distinct nodes, so the recursion stays shallow
        let mut level = (0..1 << 16).map(|_| perlin()).collect::<Vec<_>>();
        while level.len() > 1 {
            level = level
                .chunks(2)
                .map(|pair| add(&pair[0], &pair[1]))
                .collect::<Vec<_>>();
        }
        let large = &level[0];

        let late = perlin();
        assert!(add(large, &add(&late, &perlin())).encode().is_ok());
        assert!(matches!(
            add(large, &add(&late, &late)).encode(),
            Err(FastNoiseError::TooManyNodes)
        ));
    }

    #[test]
    fn test_encoded_node_tree() {
        let encoded = perlin().encode().unwrap();
        let decoded = Node::from_encoded_node_tree(&encoded).unwrap();
        assert_eq!(decoded.encode().unwrap(), encoded);

        let parent = build("Abs", &[("Source", MemberValue::from(&decoded))]);
        assert!(matches!(
            parent.encode(),
            Err(FastNoiseError::NotEncodable { node }) if node == "Perlin"
        ));
    }

    /// Same string as FastNoise2's encoder: variables are written in index order, whatever their
    /// type (Seed Offset is an int between two floats).
    #[test]
    fn test_encode_matches_fast_noise() {
        let node = build(
            "Perlin",
            &[
                ("Output Min", MemberValue::Float(-0.5)),
                ("Seed Offset", MemberValue::Int(3)),
            ],
        );

        assert_eq!(node.encode().unwrap(), "CAgD@BE@BL8E");
    }

    #[test]
    fn test_compress_as() {
        assert_eq!(compress_as("QUFB"), "QUFB");
        assert_eq!(compress_as("AAQ="), "AAQ=");
        assert_eq!(compress_as("AAAQ"), "@AQ");
        assert_eq!(compress_as("QAAAA"), "Q@B");
        assert_eq!(compress_as("AAAAAAAA"), "@F");
    }

    /// Same output as FastNoise2's `Base64::Encode` for 0x01, zero bytes, then 0x01, including
    /// the extra 'A' after a run of 66.
    #[test]
    fn test_compress_as_matches_fast_noise() {
        let cases = [
            (47, "AQ@8Q=="),
            (48, "AQ@9E="),
            (49, "AQ@+B"),
            (50, "AQ@/AAQ=="),
            (96, "AQ@/@8E="),
            (97, "AQ@/@9B"),
            (100, "AQ@/@/@AB"),
        ];

        for (zeros, expected) in cases {
            let mut bytes = vec![0; zeros + 2];
            bytes[0] = 1;
            bytes[zeros + 1] = 1;

            assert_eq!(
                compress_as(&STANDARD.encode(&bytes)),
                expected,
                "{zeros} zeros"
            );
        }
    }
}
