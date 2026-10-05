use std::ffi::NulError;

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
    #[error("unknown node '{found}' (expected one of {})", format_slice(expected))]
    MetadataNameNotFound {
        /// A list of valid metadata names.
        expected: Vec<String>,
        /// The metadata name that was not found.
        found: String,
    },

    /// Indicates a failure to create a [`CString`][`std::ffi::CString`] from the provided encoded node tree string.
    #[error("failed to create CString from encoded node tree")]
    CStringCreationFailed(#[from] NulError),

    /// Indicates a failure to create a node from the encoded node tree.
    #[error("failed to create noise node from the encoded node tree")]
    NodeCreationFailed,

    /// Indicates that the provided member name was not found.
    ///
    /// This error occurs if the member name specified is not available for the node.
    #[error(
        "unknown member '{found}' on node '{node}' (expected one of {})",
        format_slice(expected)
    )]
    MemberNameNotFound {
        /// The name of the node.
        node: String,
        /// A list of valid member names.
        expected: Vec<String>,
        /// The member name that was not found.
        found: String,
    },

    /// Indicates that the member type does not match the expected type.
    ///
    /// This error occurs when there is a mismatch between the expected member type and the provided value type.
    #[error(
        "invalid type for member '{member}' of node '{node}' (expected {expected}, found {found}){}",
        format_description(description)
    )]
    InvalidMemberType {
        /// The name of the node.
        node: String,
        /// The name of the member with the type mismatch.
        member: String,
        /// The description of the member from FastNoise2, may be empty.
        description: String,
        /// The expected member type.
        expected: MemberType,
        /// The actual member type found.
        found: MemberType,
    },

    /// Indicates that an input (node lookup member) of a node is not set.
    ///
    /// Generating noise with a missing input would crash FastNoise2.
    #[error("missing input '{member}' of node '{node}'")]
    MissingInput {
        /// The name of the node.
        node: String,
        /// The name of the input member.
        member: String,
    },

    /// Indicates that an input of a node doesn't accept the given node type.
    ///
    /// For example, a "Domain Warp Source" input only accepts domain warp nodes.
    #[error("node '{input}' is not accepted by input '{member}' of node '{node}'")]
    InputNotAccepted {
        /// The name of the node.
        node: String,
        /// The name of the input member.
        member: String,
        /// The name of the rejected node.
        input: String,
    },

    /// Indicates that FastNoise2 failed to set the value of a member.
    #[error("failed to set member '{member}' of node '{node}'")]
    SetMemberFailed {
        /// The name of the node.
        node: String,
        /// The name of the member.
        member: String,
    },

    /// Indicates that the specified enum value was not found.
    ///
    /// This error occurs if the provided enum value does not match any of the expected enum values.
    #[error(
        "unknown value '{found}' for member '{member}' of node '{node}' (expected one of {})",
        format_slice(expected)
    )]
    EnumValueNotFound {
        /// The name of the node.
        node: String,
        /// The name of the enum member.
        member: String,
        /// A list of valid enum values.
        expected: Vec<String>,
        /// The enum value that was not found.
        found: String,
    },
}

fn format_slice(slice: &[String]) -> String {
    slice
        .iter()
        .map(|s| format!("'{s}'"))
        .collect::<Vec<String>>()
        .join(", ")
}

fn format_description(description: &str) -> String {
    if description.is_empty() {
        String::new()
    } else {
        format!("\n{description}")
    }
}
