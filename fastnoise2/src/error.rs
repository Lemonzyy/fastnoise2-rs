use thiserror::Error;

use crate::metadata::MemberType;

#[derive(Error, Debug)]
pub enum FastNoiseError {
    #[error("Metadata name not found: '{0}'")]
    MetadataNameNotFound(String),

    #[error("Failed to create CString from encoded node tree")]
    CStringCreationFailed(#[from] std::ffi::NulError),

    #[error("Failed to create noise node from the encoded node tree.")]
    NodeCreationFailed,

    #[error("Member name not found: '{0}'")]
    MemberNameNotFound(String),

    #[error("Cannot set '{member_name}' to {given_type}. Expected: {expected_type}")]
    InvalidMemberType {
        member_name: String,
        given_type: String,
        expected_type: MemberType,
    },

    #[error("Failed to set float value.")]
    SetFloatFailed,

    #[error("Failed to set hybrid float value.")]
    SetHybridFloatFailed,

    #[error("Failed to set integer value.")]
    SetIntFailed,

    #[error("Enum value not found: '{0}'")]
    EnumValueNotFound(String),

    #[error("Failed to set enum value.")]
    SetEnumFailed,

    #[error("Failed to set node lookup")]
    SetNodeLookupFailed,

    #[error("Failed to set hybrid node lookup")]
    SetHybridNodeLookupFailed,
}
