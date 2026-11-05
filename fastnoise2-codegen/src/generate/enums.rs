//! Generates `enums.rs`: the enum member types.
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    Generator,
    tokens::{doc_lines, node_links},
};

impl Generator<'_> {
    pub(super) fn enums_file(&self) -> TokenStream {
        let enums = self.enums.values().map(|enum_type| {
            let name = format_ident!("{}", enum_type.name);
            let variants = enum_type
                .values
                .iter()
                .map(|value| format_ident!("{}", value.to_upper_camel_case()))
                .collect::<Vec<_>>();
            let values = &enum_type.values;

            let mut doc = doc_lines(&enum_type.description);
            if !doc.is_empty() {
                doc.push(String::new());
            }
            doc.push(format!(" Used by {}.", node_links(&enum_type.nodes)));

            quote! {
                #(#[doc = #doc])*
                #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
                pub enum #name {
                    #(#variants,)*
                }

                impl #name {
                    /// The FastNoise2 name of the value.
                    pub(crate) fn name(self) -> &'static str {
                        match self {
                            #(Self::#variants => #values,)*
                        }
                    }
                }
            }
        });

        quote! {
            #![doc = " Enum member types."]

            #(#enums)*
        }
    }
}
