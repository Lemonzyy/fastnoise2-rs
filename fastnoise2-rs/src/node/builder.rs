//! Nodes built by FastNoise2 metadata name, setting members by name.
use std::{ffi::c_void, sync::Arc};

use fastnoise2_sys::*;

#[cfg(feature = "encode")]
use super::{Description, Hybrid, NodeData};
use super::{MemberValue, Node, NodeHandle, NodeInner};
use crate::{
    FastNoiseError, FeatureSet, MemberType,
    feature_set::max_feature_set_bits,
    metadata::{METADATA_NAME_LOOKUP, Member, Metadata, NODE_METADATA, format_lookup},
};

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
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(level = "trace", err(level = "debug"))
    )]
    pub fn new(node_name: &str) -> Result<Self, FastNoiseError> {
        let metadata_id = *METADATA_NAME_LOOKUP
            .get(&format_lookup(node_name))
            .ok_or_else(|| FastNoiseError::MetadataNameNotFound {
                expected: NODE_METADATA.iter().map(|m| m.name.clone()).collect(),
                found: node_name.to_string(),
            })?;

        // SAFETY: the metadata id comes from FastNoise2 metadata, and the feature set is one
        // FastNoise2 is compiled for (checked by `with_max_feature_set`)
        let ptr = unsafe { fnNewFromMetadata(metadata_id, max_feature_set_bits()) };
        // SAFETY: FastNoise2 returns a new reference owned by the caller, or null
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
    /// type, if an enum value doesn't exist, or if an input has another SIMD feature set.
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.handle.metadata().name, member = member_name), err(level = "debug")))]
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

        // SAFETY (setters below): `ptr` is the valid node being built, `member.index` comes from
        // its metadata, and FastNoise2 takes its own reference on input nodes
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
                check_feature_set(ptr, metadata, member, &node)?;
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
                check_feature_set(ptr, metadata, member, &node)?;
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
    #[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip_all, fields(node = self.handle.metadata().name), err(level = "debug")))]
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

/// FastNoise2 only asserts that an input has the feature set of its node, generating noise with
/// another one crashes.
#[inline]
fn check_feature_set(
    ptr: *mut c_void,
    metadata: &Metadata,
    member: &Member,
    input: &Node,
) -> Result<(), FastNoiseError> {
    // SAFETY: `ptr` is the valid node being built and `input` a valid node
    let feature_set = unsafe { fnGetActiveFeatureSet(ptr) };
    // SAFETY: as above
    if feature_set == unsafe { fnGetActiveFeatureSet(input.as_ptr()) } {
        return Ok(());
    }

    Err(FastNoiseError::FeatureSetMismatch {
        node: metadata.name.clone(),
        member: member.name.clone(),
        expected: FeatureSet::from_bits(feature_set)
            .expect("FastNoise2 nodes have a known feature set"),
        found: input.get_active_feature_set(),
    })
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
    use super::{
        super::{Generator, test_utils::*},
        *,
    };

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
}
