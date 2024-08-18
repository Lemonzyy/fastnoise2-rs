use std::{any::type_name, collections::HashMap, ffi::CStr, sync::LazyLock};

use fastnoise2_sys::*;

use crate::{FastNoiseError, Node};

#[derive(Debug)]
pub(crate) struct Metadata {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub name: String,
    pub members: HashMap<String, Member>,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub member_type: MemberType,
    pub index: i32,
    pub enum_names: HashMap<String, i32>,
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
    /// A reference to a [`Node`] instance.
    NodeLookup,
    /// A member that can be either a floating-point value ([`f32`]) or a [`Node`] reference.
    Hybrid,
}

impl std::fmt::Display for MemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => f.write_str(type_name::<f32>()),
            Self::Int => f.write_str(type_name::<i32>()),
            Self::Enum => f.write_str(type_name::<&str>()),
            Self::NodeLookup => f.write_str(type_name::<&Node>()),
            Self::Hybrid => f.write_fmt(format_args!("{} or {}", Self::Float, Self::NodeLookup)),
        }
    }
}

pub(crate) static METADATA_NAME_LOOKUP: LazyLock<HashMap<String, i32>> = LazyLock::new(|| {
    let metadata_count = unsafe { fnGetMetadataCount() };
    let mut lookup = HashMap::new();

    for id in 0..metadata_count {
        let name =
            format_lookup(&unsafe { CStr::from_ptr(fnGetMetadataName(id)) }.to_string_lossy());
        lookup.insert(name, id);
    }
    lookup
});

pub(crate) static NODE_METADATA: LazyLock<Vec<Metadata>> = LazyLock::new(|| {
    let metadata_count = unsafe { fnGetMetadataCount() };
    let mut metadata_vec = Vec::with_capacity(metadata_count as usize);
    for id in 0..metadata_count {
        let name =
            format_lookup(&unsafe { CStr::from_ptr(fnGetMetadataName(id)) }.to_string_lossy());
        let mut members = HashMap::new();

        let variable_count = unsafe { fnGetMetadataVariableCount(id) };
        let node_lookup_count = unsafe { fnGetMetadataNodeLookupCount(id) };
        let hybrid_count = unsafe { fnGetMetadataHybridCount(id) };

        for variable_idx in 0..variable_count {
            let member_type = match unsafe { fnGetMetadataVariableType(id, variable_idx) } {
                0 => MemberType::Float,
                1 => MemberType::Int,
                2 => MemberType::Enum,
                _ => MemberType::Hybrid,
            };
            let dimension_idx = unsafe { fnGetMetadataVariableDimensionIdx(id, variable_idx) };
            let name = format_dimension_member(
                &format_lookup(
                    &unsafe { CStr::from_ptr(fnGetMetadataVariableName(id, variable_idx)) }
                        .to_string_lossy(),
                ),
                dimension_idx,
            );
            let mut enum_names = HashMap::new();
            if let MemberType::Enum = member_type {
                let enum_count = unsafe { fnGetMetadataEnumCount(id, variable_idx) };
                for enum_idx in 0..enum_count {
                    let enum_name = format_lookup(
                        &unsafe {
                            CStr::from_ptr(fnGetMetadataEnumName(id, variable_idx, enum_idx))
                        }
                        .to_string_lossy(),
                    );
                    enum_names.insert(enum_name, enum_idx);
                }
            }
            members.insert(
                name.clone(),
                Member {
                    name,
                    member_type,
                    index: variable_idx,
                    enum_names,
                },
            );
        }

        for node_lookup_idx in 0..node_lookup_count {
            let dimension_idx = unsafe { fnGetMetadataNodeLookupDimensionIdx(id, node_lookup_idx) };
            let name = format_dimension_member(
                &format_lookup(
                    &unsafe { CStr::from_ptr(fnGetMetadataNodeLookupName(id, node_lookup_idx)) }
                        .to_string_lossy(),
                ),
                dimension_idx,
            );
            members.insert(
                name.clone(),
                Member {
                    name,
                    member_type: MemberType::NodeLookup,
                    index: node_lookup_idx,
                    enum_names: HashMap::new(),
                },
            );
        }

        for hybrid_idx in 0..hybrid_count {
            let dimension_idx = unsafe { fnGetMetadataHybridDimensionIdx(id, hybrid_idx) };
            let name = format_dimension_member(
                &format_lookup(
                    &unsafe { CStr::from_ptr(fnGetMetadataHybridName(id, hybrid_idx)) }
                        .to_string_lossy(),
                ),
                dimension_idx,
            );
            members.insert(
                name.clone(),
                Member {
                    name,
                    member_type: MemberType::Hybrid,
                    index: hybrid_idx,
                    enum_names: HashMap::new(),
                },
            );
        }

        metadata_vec.push(Metadata { id, name, members });
    }
    metadata_vec
});

pub(crate) fn format_lookup(name: &str) -> String {
    name.replace(" ", "").to_lowercase()
}

fn format_dimension_member(name: &str, dim_idx: i32) -> String {
    if dim_idx >= 0 {
        let dim_suffix = ['x', 'y', 'z', 'w'];
        let suffix = dim_suffix[dim_idx as usize];
        format!("{name}{suffix}")
    } else {
        name.to_string()
    }
}

pub trait MemberValue: Copy {
    const TYPE: MemberType;

    fn apply(self, node: &mut Node, member: &Member) -> Result<(), FastNoiseError>;

    fn invalid_member_type_error(member: &Member) -> FastNoiseError {
        FastNoiseError::InvalidMemberType {
            member_name: member.name.clone(),
            expected: member.member_type,
            found: Self::TYPE,
        }
    }
}

impl MemberValue for f32 {
    const TYPE: MemberType = MemberType::Float;

    fn apply(self, node: &mut Node, member: &Member) -> Result<(), FastNoiseError> {
        match member.member_type {
            MemberType::Float => {
                if !unsafe { fnSetVariableFloat(node.handle, member.index, self) } {
                    return Err(FastNoiseError::SetFloatFailed);
                }
            }
            MemberType::Hybrid => {
                if !unsafe { fnSetHybridFloat(node.handle, member.index, self) } {
                    return Err(FastNoiseError::SetHybridFloatFailed);
                }
            }
            _ => return Err(Self::invalid_member_type_error(member)),
        }
        Ok(())
    }
}

impl MemberValue for i32 {
    const TYPE: MemberType = MemberType::Int;

    fn apply(self, node: &mut Node, member: &Member) -> Result<(), FastNoiseError> {
        match member.member_type {
            MemberType::Int => {
                if !unsafe { fnSetVariableIntEnum(node.handle, member.index, self) } {
                    return Err(FastNoiseError::SetIntFailed);
                }
            }
            _ => return Err(Self::invalid_member_type_error(member)),
        }
        Ok(())
    }
}

impl MemberValue for &str {
    const TYPE: MemberType = MemberType::Enum;

    fn apply(self, node: &mut Node, member: &Member) -> Result<(), FastNoiseError> {
        match member.member_type {
            MemberType::Enum => {
                let enum_idx = member.enum_names.get(&format_lookup(self)).ok_or_else(|| {
                    FastNoiseError::EnumValueNotFound {
                        expected: member.enum_names.keys().cloned().collect(),
                        found: self.to_string(),
                    }
                })?;
                if !unsafe { fnSetVariableIntEnum(node.handle, member.index, *enum_idx) } {
                    return Err(FastNoiseError::SetEnumFailed);
                }
            }
            _ => return Err(Self::invalid_member_type_error(member)),
        }
        Ok(())
    }
}

impl MemberValue for &Node {
    const TYPE: MemberType = MemberType::NodeLookup;

    fn apply(self, node: &mut Node, member: &Member) -> Result<(), FastNoiseError> {
        match member.member_type {
            MemberType::NodeLookup => {
                if !unsafe { fnSetNodeLookup(node.handle, member.index, self.handle) } {
                    return Err(FastNoiseError::SetNodeLookupFailed);
                }
            }
            MemberType::Hybrid => {
                if !unsafe { fnSetHybridNodeLookup(node.handle, member.index, self.handle) } {
                    return Err(FastNoiseError::SetHybridNodeLookupFailed);
                }
            }
            _ => return Err(Self::invalid_member_type_error(member)),
        }
        Ok(())
    }
}
