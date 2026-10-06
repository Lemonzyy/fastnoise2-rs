use std::{
    collections::HashMap,
    ffi::{CStr, c_char},
    fmt,
    sync::LazyLock,
};

use fastnoise2_sys::*;

use crate::MemberValue;

/// FastNoise2 metadata of a node type: its name, description and members.
///
/// ```rust
/// use fastnoise2::Metadata;
///
/// let fbm = Metadata::by_name("fractal fbm").unwrap();
/// assert_eq!(fbm.name(), "FractalFBm");
///
/// for member in fbm.members() {
///     println!("{} ({}): {:?}", member.name(), member.member_type(), member.default_value());
/// }
/// ```
#[derive(Debug)]
pub struct Metadata {
    /// Node name, as displayed by FastNoise2 (e.g. "Perlin").
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) groups: Vec<String>,
    /// Members in FastNoise2 metadata order: variables, node lookups, then hybrids.
    pub(crate) members: Vec<Member>,
    /// Index into `members` by formatted lookup name.
    member_lookup: HashMap<String, usize>,
}

impl Metadata {
    /// Metadata of every FastNoise2 node type.
    pub fn all() -> &'static [Metadata] {
        &NODE_METADATA
    }

    /// Metadata of a node type by name, ignoring case and spaces (e.g. "FractalFBm" or
    /// "fractal fbm").
    pub fn by_name(name: &str) -> Option<&'static Metadata> {
        METADATA_NAME_LOOKUP
            .get(&format_lookup(name))
            .map(|&id| &NODE_METADATA[id as usize])
    }

    /// Node name, as displayed by FastNoise2 (e.g. "Perlin").
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Node description from FastNoise2, may be empty.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Groups of the node in the FastNoise2 Node Editor (e.g. "Coherent Noise").
    pub fn groups(&self) -> &[String] {
        &self.groups
    }

    /// Members in FastNoise2 order: variables, inputs (node lookups), then hybrids.
    pub fn members(&self) -> &[Member] {
        &self.members
    }

    /// Member by name, ignoring case and spaces (e.g. "Feature Scale" or "featurescale").
    pub fn member(&self, name: &str) -> Option<&Member> {
        self.member_lookup
            .get(&format_lookup(name))
            .map(|&index| &self.members[index])
    }
}

/// FastNoise2 metadata of a node member.
#[derive(Debug, Clone)]
pub struct Member {
    /// Member name, as displayed by FastNoise2 (e.g. "Feature Scale", "Multiplier X").
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) member_type: MemberType,
    /// Index among the members of the same kind (variables, node lookups or hybrids).
    pub(crate) index: i32,
    /// Enum values in FastNoise2 order, the position is the enum index.
    pub(crate) enum_values: Vec<String>,
    /// Default value: the bits of the float for float and hybrid members, the value for int and
    /// enum members, 0 for node lookups.
    pub(crate) default_bits: i32,
}

impl Member {
    /// Member name, as displayed by FastNoise2 (e.g. "Feature Scale", "Multiplier X").
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Member description from FastNoise2, may be empty.
    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn member_type(&self) -> MemberType {
        self.member_type
    }

    /// Enum values in FastNoise2 order, empty if the member is not an enum.
    pub fn enum_values(&self) -> &[String] {
        &self.enum_values
    }

    /// FastNoise2 default value, `None` for inputs (node lookups), which have none. The default
    /// of a hybrid member is a float.
    pub fn default_value(&self) -> Option<MemberValue> {
        match self.member_type {
            MemberType::Float | MemberType::Hybrid => {
                Some(MemberValue::Float(f32::from_bits(self.default_bits as u32)))
            }
            MemberType::Int => Some(MemberValue::Int(self.default_bits)),
            MemberType::Enum => Some(MemberValue::Enum(
                self.enum_values[self.default_bits as usize].clone(),
            )),
            MemberType::NodeLookup => None,
        }
    }

    pub(crate) fn enum_index(&self, value: &str) -> Option<i32> {
        let value = format_lookup(value);
        self.enum_values
            .iter()
            .position(|enum_value| format_lookup(enum_value) == value)
            .map(|index| index as i32)
    }
}

/// Defines the type of value or reference a node can handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
            // The raw bits of the value union, whatever the variable type
            default_bits: unsafe { fnGetMetadataVariableDefaultIntEnum(id, variable_idx) },
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
            default_bits: 0,
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
            default_bits: unsafe { fnGetMetadataHybridDefault(id, hybrid_idx) }.to_bits() as i32,
        });
    }

    let member_lookup = members
        .iter()
        .enumerate()
        .map(|(index, member)| (format_lookup(&member.name), index))
        .collect();

    Metadata {
        name: to_string(unsafe { fnGetMetadataName(id) }),
        description: to_string(unsafe { fnGetMetadataDescription(id) }),
        groups: (0..unsafe { fnGetMetadataGroupCount(id) })
            .map(|group_idx| to_string(unsafe { fnGetMetadataGroupName(id, group_idx) }))
            .collect(),
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
    use super::*;
    use crate::{FastNoiseError, node::NodeBuilder};

    #[test]
    fn test_all_and_by_name() {
        assert_eq!(Metadata::all().len(), NODE_METADATA.len());
        assert!(
            Metadata::all()
                .iter()
                .any(|metadata| metadata.name() == "Perlin")
        );

        assert_eq!(
            Metadata::by_name("fractal fbm").unwrap().name(),
            "FractalFBm"
        );
        assert!(Metadata::by_name("Perln").is_none());
    }

    #[test]
    fn test_descriptions_and_groups() {
        let perlin = Metadata::by_name("Perlin").unwrap();
        assert!(!perlin.description().is_empty());
        assert_eq!(perlin.groups(), ["Coherent Noise"]);

        let progressive = Metadata::by_name("DomainWarpFractalProgressive").unwrap();
        assert_eq!(progressive.groups(), ["Domain Warp", "Fractal"]);
    }

    #[test]
    fn test_member_default_values() {
        let fbm = Metadata::by_name("FractalFBm").unwrap();
        let default = |name: &str| fbm.member(name).unwrap().default_value();

        assert!(matches!(default("Octaves"), Some(MemberValue::Int(3))));
        assert!(matches!(
            default("Lacunarity"),
            Some(MemberValue::Float(2.0))
        ));
        assert!(matches!(default("Gain"), Some(MemberValue::Float(0.5))));
        assert!(default("Source").is_none());

        let cellular = Metadata::by_name("CellularValue").unwrap();
        let distance_function = cellular.member("Distance Function").unwrap();
        assert_eq!(distance_function.member_type(), MemberType::Enum);
        assert_eq!(distance_function.enum_values()[0], "Euclidean");
        assert!(matches!(
            distance_function.default_value(),
            Some(MemberValue::Enum(value)) if value == "Euclidean Squared"
        ));
    }

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
