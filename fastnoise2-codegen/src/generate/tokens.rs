//! Helpers building tokens and doc comments.
use heck::ToSnakeCase;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::metadata::Member;

/// Float literal without suffix, always with a decimal point (e.g. `2.0`).
pub(super) fn float_literal(value: f32) -> TokenStream {
    let literal = Literal::f32_unsuffixed(value.abs());
    if value < 0.0 {
        quote! { -#literal }
    } else {
        quote! { #literal }
    }
}

/// Integer literal without suffix.
pub(super) fn int_literal(value: i32) -> TokenStream {
    let literal = Literal::u32_unsuffixed(value.unsigned_abs());
    if value < 0 {
        quote! { -#literal }
    } else {
        quote! { #literal }
    }
}

/// Field identifier of a member, escaped if it is a Rust keyword.
pub(super) fn field_ident(member: &Member) -> Ident {
    let name = member.name.to_snake_case();
    if syn::parse_str::<Ident>(&name).is_ok() {
        Ident::new(&name, Span::call_site())
    } else {
        Ident::new_raw(&name, Span::call_site())
    }
}

/// Doc comment lines, with a leading space and bare URLs turned into links.
pub(super) fn doc_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(|line| {
            let line = line
                .split(' ')
                .map(|word| {
                    if word.starts_with("http://") || word.starts_with("https://") {
                        format!("<{word}>")
                    } else {
                        word.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            format!(" {}", line.trim_end())
        })
        .collect()
}

pub(super) fn node_links(nodes: &[String]) -> String {
    nodes
        .iter()
        .map(|node| format!("[`{node}`](super::{node})"))
        .collect::<Vec<_>>()
        .join(", ")
}
