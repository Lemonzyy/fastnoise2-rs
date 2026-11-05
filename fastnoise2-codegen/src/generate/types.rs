//! Rust types and default values of members.
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    EnumType, Generator,
    tokens::{float_literal, int_literal},
};
use crate::metadata::{Member, MemberKind};

/// Rust type of a member, as stored in the generated struct.
pub(super) enum FieldType<'a> {
    Float,
    Int,
    Bool,
    Enum(&'a EnumType),
    Input,
    Hybrid,
}

impl Generator<'_> {
    pub(super) fn field_type(&self, member: &Member) -> FieldType<'_> {
        match &member.kind {
            MemberKind::Float { .. } => FieldType::Float,
            MemberKind::Int { .. } => FieldType::Int,
            MemberKind::Enum { values, .. } if is_bool(values) => FieldType::Bool,
            MemberKind::Enum { values, .. } => FieldType::Enum(
                self.enums
                    .values()
                    .find(|enum_type| &enum_type.values == values)
                    .expect("enum type collected"),
            ),
            MemberKind::Input(_) => FieldType::Input,
            MemberKind::Hybrid { .. } => FieldType::Hybrid,
        }
    }

    pub(super) fn rust_type(&self, member: &Member) -> TokenStream {
        match self.field_type(member) {
            FieldType::Float => quote! { f32 },
            FieldType::Int => quote! { i32 },
            FieldType::Bool => quote! { bool },
            FieldType::Enum(enum_type) => {
                let name = format_ident!("{}", enum_type.name);
                quote! { #name }
            }
            FieldType::Input => quote! { Node },
            FieldType::Hybrid => quote! { Hybrid },
        }
    }

    /// Rust expression of the default value, `None` for inputs.
    pub(super) fn default_value(&self, member: &Member) -> Option<TokenStream> {
        Some(match (&member.kind, self.field_type(member)) {
            (MemberKind::Float { default, .. }, _) => float_literal(*default),
            (MemberKind::Int { default, .. }, _) => int_literal(*default),
            (MemberKind::Enum { default, .. }, FieldType::Bool) => {
                let default = *default == 1;
                quote! { #default }
            }
            (MemberKind::Enum { values, default }, FieldType::Enum(enum_type)) => {
                let name = format_ident!("{}", enum_type.name);
                let variant = format_ident!("{}", values[*default].to_upper_camel_case());
                quote! { #name::#variant }
            }
            (MemberKind::Hybrid { default }, _) => {
                let default = float_literal(*default);
                quote! { Hybrid::Value(#default) }
            }
            _ => return None,
        })
    }

    pub(super) fn default_doc(&self, member: &Member) -> String {
        match (&member.kind, self.field_type(member)) {
            (MemberKind::Float { default, .. } | MemberKind::Hybrid { default }, _) => {
                format!("{default:?}")
            }
            (MemberKind::Int { default, .. }, _) => default.to_string(),
            (MemberKind::Enum { default, .. }, FieldType::Bool) => (*default == 1).to_string(),
            (MemberKind::Enum { values, default }, FieldType::Enum(enum_type)) => {
                format!(
                    "{}::{}",
                    enum_type.name,
                    values[*default].to_upper_camel_case()
                )
            }
            _ => unreachable!("inputs have no default"),
        }
    }
}

/// Range the Node Editor clamps the member to, `None` if it has none.
pub(super) fn range_doc(member: &Member) -> Option<String> {
    match &member.kind {
        MemberKind::Float {
            range: Some(range), ..
        } => Some(format!("{range:?}")),
        MemberKind::Int {
            range: Some(range), ..
        } => Some(format!("{range:?}")),
        _ => None,
    }
}

pub(super) fn is_bool(values: &[String]) -> bool {
    values == ["False", "True"]
}
