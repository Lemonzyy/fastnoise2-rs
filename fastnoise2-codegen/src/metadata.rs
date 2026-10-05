//! Loads FastNoise2 metadata through the C API.
use std::ffi::{c_char, CStr};

use fastnoise2_sys::*;

pub struct Node {
    pub name: String,
    pub description: String,
    pub group: String,
    pub members: Vec<Member>,
}

impl Node {
    pub fn inputs(&self) -> impl Iterator<Item = &Member> {
        self.members
            .iter()
            .filter(|member| matches!(member.kind, MemberKind::Input))
    }
}

pub struct Member {
    /// Name as displayed by FastNoise2, with the dimension (e.g. "Feature Scale", "Offset X").
    pub name: String,
    pub description: String,
    pub kind: MemberKind,
}

pub enum MemberKind {
    Float { default: f32 },
    Int { default: i32 },
    Enum { values: Vec<String>, default: usize },
    Input,
    Hybrid { default: f32 },
}

pub fn load() -> Vec<Node> {
    let count = unsafe { fnGetMetadataCount() };
    let names = (0..count)
        .map(|id| to_string(unsafe { fnGetMetadataName(id) }))
        .collect::<Vec<_>>();

    (0..count).map(|id| load_node(id, &names)).collect()
}

fn load_node(id: i32, names: &[String]) -> Node {
    let mut members = Vec::new();

    for index in 0..unsafe { fnGetMetadataVariableCount(id) } {
        let kind = match unsafe { fnGetMetadataVariableType(id, index) } {
            0 => MemberKind::Float {
                default: unsafe { fnGetMetadataVariableDefaultFloat(id, index) },
            },
            1 => MemberKind::Int {
                default: unsafe { fnGetMetadataVariableDefaultIntEnum(id, index) },
            },
            2 => MemberKind::Enum {
                values: (0..unsafe { fnGetMetadataEnumCount(id, index) })
                    .map(|value| to_string(unsafe { fnGetMetadataEnumName(id, index, value) }))
                    .collect(),
                default: unsafe { fnGetMetadataVariableDefaultIntEnum(id, index) } as usize,
            },
            variable_type => panic!("unknown variable type {variable_type}"),
        };

        members.push(Member {
            name: dimension_name(
                to_string(unsafe { fnGetMetadataVariableName(id, index) }),
                unsafe { fnGetMetadataVariableDimensionIdx(id, index) },
            ),
            description: to_string(unsafe { fnGetMetadataVariableDescription(id, index) }),
            kind,
        });
    }

    for index in 0..unsafe { fnGetMetadataNodeLookupCount(id) } {
        members.push(Member {
            name: dimension_name(
                to_string(unsafe { fnGetMetadataNodeLookupName(id, index) }),
                unsafe { fnGetMetadataNodeLookupDimensionIdx(id, index) },
            ),
            description: to_string(unsafe { fnGetMetadataNodeLookupDescription(id, index) }),
            kind: MemberKind::Input,
        });
    }

    for index in 0..unsafe { fnGetMetadataHybridCount(id) } {
        members.push(Member {
            name: dimension_name(
                to_string(unsafe { fnGetMetadataHybridName(id, index) }),
                unsafe { fnGetMetadataHybridDimensionIdx(id, index) },
            ),
            description: to_string(unsafe { fnGetMetadataHybridDescription(id, index) }),
            kind: MemberKind::Hybrid {
                default: unsafe { fnGetMetadataHybridDefault(id, index) },
            },
        });
    }

    Node {
        name: names[id as usize].clone(),
        description: to_string(unsafe { fnGetMetadataDescription(id) }),
        group: to_string(unsafe { fnGetMetadataGroupName(id, 0) }),
        members,
    }
}

fn dimension_name(name: String, dimension: i32) -> String {
    match dimension {
        0..=3 => format!("{name} {}", ['X', 'Y', 'Z', 'W'][dimension as usize]),
        _ => name,
    }
}

fn to_string(c_str: *const c_char) -> String {
    unsafe { CStr::from_ptr(c_str) }
        .to_string_lossy()
        .into_owned()
}
