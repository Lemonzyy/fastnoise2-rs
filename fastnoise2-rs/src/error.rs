use thiserror::Error;

use crate::metadata::MemberType;

/// Errors that can occur when interacting with [`Node`][`crate::Node`].
///
/// This enum covers various failure scenarios including metadata issues, value setting problems, and node creation errors.
#[derive(Error, Debug)]
pub enum FastNoiseError {
    /// Indicates that the provided metadata name was not found.
    ///
    /// FastNoise2 uses metadata to manage node names and parameters. This error occurs if the given metadata name is not recognized.
    #[error(
        "metadata name not found (expected one of {}, found '{found}')",
        format_slice(expected)
    )]
    MetadataNameNotFound {
        /// A list of valid metadata names.
        expected: Vec<String>,
        /// The metadata name that was not found.
        found: String,
    },

    /// Indicates a failure to create a [`CString`][`std::ffi::CString`] from the provided encoded node tree string.
    #[error("failed to create CString from encoded node tree")]
    CStringCreationFailed(#[from] std::ffi::NulError),

    /// Indicates a failure to create a node from the encoded node tree.
    #[error("failed to create noise node from the encoded node tree")]
    NodeCreationFailed,

    /// Indicates that the provided member name was not found.
    ///
    /// This error occurs if the member name specified is not available for the node.
    #[error(
        "member name not found (expected one of {}, found '{found}')",
        format_slice(expected)
    )]
    MemberNameNotFound {
        /// A list of valid member names.
        expected: Vec<String>,
        /// The member name that was not found.
        found: String,
    },

    /// Indicates that the member type does not match the expected type.
    ///
    /// This error occurs when there is a mismatch between the expected member type and the provided value type.
    #[error("invalid member type for '{member_name}' (expected {expected}, found {found})")]
    InvalidMemberType {
        /// The name of the member with the type mismatch.
        member_name: String,
        /// The expected member type.
        expected: MemberType,
        /// The actual member type found.
        found: MemberType,
    },

    /// Indicates a failure to set a float value for a member.
    #[error("failed to set float value")]
    SetFloatFailed,

    /// Indicates a failure to set a hybrid float value for a member.
    #[error("failed to set hybrid float value")]
    SetHybridFloatFailed,

    /// Indicates a failure to set an integer value for a member.
    #[error("failed to set integer value")]
    SetIntFailed,

    /// Indicates that the specified enum value was not found.
    ///
    /// This error occurs if the provided enum value does not match any of the expected enum values.
    #[error(
        "enum value not found (expected one of {}, found '{found}')",
        format_slice(expected)
    )]
    EnumValueNotFound {
        /// A list of valid enum values.
        expected: Vec<String>,
        /// The enum value that was not found.
        found: String,
    },

    /// Indicates a failure to set an enum value for a member.
    #[error("failed to set enum value")]
    SetEnumFailed,

    /// Indicates a failure to set a node lookup for a member.
    #[error("failed to set node lookup")]
    SetNodeLookupFailed,

    /// Indicates a failure to set a hybrid node lookup for a member.
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
