use thiserror::Error;

use crate::metadata::MemberType;

#[derive(Error, Debug)]
pub enum FastNoiseError {
    #[error(
        "metadata name not found (expected one of {}, found '{found}')",
        format_slice(expected)
    )]
    MetadataNameNotFound {
        expected: Vec<String>,
        found: String,
    },

    #[error("failed to create CString from encoded node tree")]
    CStringCreationFailed(#[from] std::ffi::NulError),

    #[error("failed to create noise node from the encoded node tree")]
    NodeCreationFailed,

    #[error(
        "member name not found (expected one of {}, found '{found}')",
        format_slice(expected)
    )]
    MemberNameNotFound {
        expected: Vec<String>,
        found: String,
    },

    #[error("invalid member type for '{member_name}' (expected {expected}, found {found})")]
    InvalidMemberType {
        member_name: String,
        expected: MemberType,
        found: MemberType,
    },

    #[error("failed to set float value")]
    SetFloatFailed,

    #[error("failed to set hybrid float value")]
    SetHybridFloatFailed,

    #[error("failed to set integer value")]
    SetIntFailed,

    #[error(
        "enum value not found (expected one of {}, found '{found}')",
        format_slice(expected)
    )]
    EnumValueNotFound {
        expected: Vec<String>,
        found: String,
    },

    #[error("failed to set enum value")]
    SetEnumFailed,

    #[error("failed to set node lookup")]
    SetNodeLookupFailed,

    #[error("failed to set hybrid node lookup")]
    SetHybridNodeLookupFailed,
}

fn format_slice(slice: &[String]) -> String {
    slice
        .iter()
        .map(|s| format!("'{s}'"))
        .collect::<Vec<String>>()
        .join(", ")
}
