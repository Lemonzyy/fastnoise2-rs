use std::{
    collections::HashMap,
    ffi::{c_char, CStr},
    fmt,
    sync::LazyLock,
};

use fastnoise2_sys::*;

#[derive(Debug)]
pub(crate) struct Metadata {
    /// Node name, as displayed by FastNoise2 (e.g. "Perlin").
    pub name: String,
    /// Members in FastNoise2 metadata order: variables, node lookups, then hybrids.
    pub members: Vec<Member>,
    /// Index into `members` by formatted lookup name.
    member_lookup: HashMap<String, usize>,
}

impl Metadata {
    pub fn member(&self, name: &str) -> Option<&Member> {
        self.member_lookup
            .get(&format_lookup(name))
            .map(|&index| &self.members[index])
    }
}

#[derive(Debug, Clone)]
pub struct Member {
    /// Member name, as displayed by FastNoise2 (e.g. "Feature Scale", "Multiplier X").
    pub name: String,
    /// Member description from FastNoise2, may be empty.
    pub description: String,
    pub member_type: MemberType,
    pub index: i32,
    /// Enum values in FastNoise2 order, the position is the enum index.
    pub enum_values: Vec<String>,
}

impl Member {
    pub fn enum_index(&self, value: &str) -> Option<i32> {
        let value = format_lookup(value);
        self.enum_values
            .iter()
            .position(|enum_value| format_lookup(enum_value) == value)
            .map(|index| index as i32)
    }
}

/// Defines the type of value or reference a node can handle.
#[derive(Clone, Copy, Debug)]
pub enum MemberType {
    /// A floating-point number ([`f32`]).
    Float,
    /// An integer ([`i32`]).
    Int,
    /// An enumerated value represented as a string ([`&str`]).
    Enum,
    /// A reference to a [`Node`](crate::Node) instance.
    NodeLookup,
    /// A member that can be either a floating-point value ([`f32`]) or a [`Node`](crate::Node) reference.
    Hybrid,
}

impl fmt::Display for MemberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float => f.write_str("f32"),
            Self::Int => f.write_str("i32"),
            Self::Enum => f.write_str("&str"),
            Self::NodeLookup => f.write_str("Node"),
            Self::Hybrid => f.write_str("f32 or Node"),
        }
    }
}

pub(crate) static METADATA_NAME_LOOKUP: LazyLock<HashMap<String, i32>> = LazyLock::new(|| {
    NODE_METADATA
        .iter()
        .enumerate()
        .map(|(id, metadata)| (format_lookup(&metadata.name), id as i32))
        .collect()
});

pub(crate) static NODE_METADATA: LazyLock<Vec<Metadata>> = LazyLock::new(|| {
    let metadata_count = unsafe { fnGetMetadataCount() };
    (0..metadata_count).map(load_metadata).collect()
});

fn load_metadata(id: i32) -> Metadata {
    let mut members = Vec::new();

    for variable_idx in 0..unsafe { fnGetMetadataVariableCount(id) } {
        let member_type = match unsafe { fnGetMetadataVariableType(id, variable_idx) } {
            0 => MemberType::Float,
            1 => MemberType::Int,
            2 => MemberType::Enum,
            _ => MemberType::Hybrid,
        };

        let enum_values = match member_type {
            MemberType::Enum => (0..unsafe { fnGetMetadataEnumCount(id, variable_idx) })
                .map(|enum_idx| {
                    to_string(unsafe { fnGetMetadataEnumName(id, variable_idx, enum_idx) })
                })
                .collect(),
            _ => Vec::new(),
        };

        members.push(Member {
            name: dimension_member_name(
                to_string(unsafe { fnGetMetadataVariableName(id, variable_idx) }),
                unsafe { fnGetMetadataVariableDimensionIdx(id, variable_idx) },
            ),
            description: to_string(unsafe { fnGetMetadataVariableDescription(id, variable_idx) }),
            member_type,
            index: variable_idx,
            enum_values,
        });
    }

    for node_lookup_idx in 0..unsafe { fnGetMetadataNodeLookupCount(id) } {
        members.push(Member {
            name: dimension_member_name(
                to_string(unsafe { fnGetMetadataNodeLookupName(id, node_lookup_idx) }),
                unsafe { fnGetMetadataNodeLookupDimensionIdx(id, node_lookup_idx) },
            ),
            description: to_string(unsafe {
                fnGetMetadataNodeLookupDescription(id, node_lookup_idx)
            }),
            member_type: MemberType::NodeLookup,
            index: node_lookup_idx,
            enum_values: Vec::new(),
        });
    }

    for hybrid_idx in 0..unsafe { fnGetMetadataHybridCount(id) } {
        members.push(Member {
            name: dimension_member_name(
                to_string(unsafe { fnGetMetadataHybridName(id, hybrid_idx) }),
                unsafe { fnGetMetadataHybridDimensionIdx(id, hybrid_idx) },
            ),
            description: to_string(unsafe { fnGetMetadataHybridDescription(id, hybrid_idx) }),
            member_type: MemberType::Hybrid,
            index: hybrid_idx,
            enum_values: Vec::new(),
        });
    }

    let member_lookup = members
        .iter()
        .enumerate()
        .map(|(index, member)| (format_lookup(&member.name), index))
        .collect();

    Metadata {
        name: to_string(unsafe { fnGetMetadataName(id) }),
        members,
        member_lookup,
    }
}

fn to_string(c_str: *const c_char) -> String {
    unsafe { CStr::from_ptr(c_str) }
        .to_string_lossy()
        .into_owned()
}

pub(crate) fn format_lookup(name: &str) -> String {
    name.replace(" ", "").to_lowercase()
}

fn dimension_member_name(name: String, dim_idx: i32) -> String {
    match dim_idx {
        0..=3 => format!("{name} {}", ['X', 'Y', 'Z', 'W'][dim_idx as usize]),
        _ => name,
    }
}

#[cfg(test)]
mod tests {
    use crate::{node::NodeBuilder, FastNoiseError};

    #[test]
    fn test_member_name_not_found_lists_display_names_in_order() {
        let error = NodeBuilder::new("Perlin")
            .unwrap()
            .set("FeatureScal", 10.0)
            .err()
            .unwrap();
        assert_eq!(
            error.to_string(),
            "unknown member 'FeatureScal' on node 'Perlin' (expected one of \
             'Feature Scale', 'Seed Offset', 'Output Min', 'Output Max')"
        );
    }

    #[test]
    fn test_enum_value_not_found_lists_display_names_in_order() {
        let error = NodeBuilder::new("CellularValue")
            .unwrap()
            .set("DistanceFunction", "Euclidian")
            .err()
            .unwrap();
        assert_eq!(
            error.to_string(),
            "unknown value 'Euclidian' for member 'Distance Function' of node \
             'CellularValue' (expected one of 'Euclidean', 'Euclidean Squared', 'Manhattan', \
             'Hybrid', 'Max Axis', 'Minkowski')"
        );
    }

    #[test]
    fn test_invalid_member_type_names_node_and_member() {
        let error = NodeBuilder::new("Perlin")
            .unwrap()
            .set("SeedOffset", 1.5)
            .err()
            .unwrap();
        assert_eq!(
            error.to_string(),
            "invalid type for member 'Seed Offset' of node 'Perlin' (expected i32, found f32)\n\
             This value is added to the seed before generation\n\
             Doesn't affect the seed passed to child nodes\n\
             Useful if you have multiple nodes of the same type and want them to give different outputs"
        );
    }

    #[test]
    fn test_invalid_member_type_without_description() {
        let error = NodeBuilder::new("DistanceToPoint")
            .unwrap()
            .set("DistanceFunction", 1.0)
            .err()
            .unwrap();
        assert_eq!(
            error.to_string(),
            "invalid type for member 'Distance Function' of node 'DistanceToPoint' \
             (expected &str, found f32)"
        );
    }

    #[test]
    fn test_metadata_name_not_found_keeps_input_and_display_names() {
        let Err(FastNoiseError::MetadataNameNotFound { expected, found }) =
            NodeBuilder::new("Perln")
        else {
            panic!("expected MetadataNameNotFound");
        };
        assert_eq!(found, "Perln");
        assert!(expected.iter().any(|name| name == "Perlin"));
    }

    #[test]
    fn test_member_type_names() {
        let error = NodeBuilder::new("Perlin")
            .unwrap()
            .set(
                "Seed Offset",
                NodeBuilder::new("Perlin").unwrap().build().unwrap(),
            )
            .err()
            .unwrap();
        assert!(error.to_string().starts_with(
            "invalid type for member 'Seed Offset' of node 'Perlin' (expected i32, found Node)"
        ));
    }

    #[test]
    fn test_dimension_member_names() {
        let node = NodeBuilder::new("Gradient").unwrap();
        let node = node.set("Multiplier X", 1.0).unwrap();
        assert!(node.set("offsetw", 1.0).is_ok());
    }
}
