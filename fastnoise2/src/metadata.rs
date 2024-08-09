use std::{any::type_name, collections::HashMap, ffi::CStr, sync::LazyLock};

use fastnoise2_sys::*;

use crate::FastNoise;

#[derive(Debug)]
pub(crate) struct Metadata {
    pub id: i32,
    pub name: String,
    pub members: HashMap<String, Member>,
}

#[derive(Debug, Clone)]
pub(crate) struct Member {
    pub name: String,
    pub member_type: MemberType,
    pub index: i32,
    pub enum_names: HashMap<String, i32>,
}

#[derive(Clone, Copy, Debug)]
pub enum MemberType {
    Float,
    Int,
    Enum,
    NodeLookup,
    Hybrid,
}

impl std::fmt::Display for MemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => f.write_str(type_name::<f32>()),
            Self::Int => f.write_str(type_name::<i32>()),
            Self::Enum => f.write_str(type_name::<&str>()),
            Self::NodeLookup => f.write_str(type_name::<&FastNoise>()),
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
