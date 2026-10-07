//! Generates `ext.rs`: the chaining methods.
use std::collections::BTreeMap;

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{Generator, tokens::field_ident};
use crate::metadata::{Input, Member};

impl Generator<'_> {
    /// Chaining methods: one per node with inputs, called on its first input.
    ///
    /// Inputs accepting only some node types get a marker trait implemented by those types,
    /// and the nodes whose first input is restricted get their own extension trait.
    pub(super) fn ext_file(&self) -> TokenStream {
        let mut restricted = BTreeMap::<String, (&Member, &Vec<String>)>::new();
        for (member, input) in self.nodes.iter().flat_map(|node| node.inputs()) {
            if let Some(accepted) = &input.accepted {
                restricted
                    .entry(member.name.to_upper_camel_case())
                    .or_insert((member, accepted));
            }
        }

        let mut methods = BTreeMap::<Option<String>, Vec<TokenStream>>::new();
        for node in self.nodes {
            let inputs = node.inputs().collect::<Vec<_>>();
            let Some(((first, first_input), others)) = inputs.split_first() else {
                continue;
            };

            let name = format_ident!("{}", node.name);
            let method = format_ident!("{}", node.name.to_snake_case());
            let parameters = others
                .iter()
                .map(|(member, input)| {
                    let parameter = field_ident(member);
                    let bound = input_bound(input, member);
                    quote! { #parameter: impl #bound }
                })
                .collect::<Vec<_>>();
            let arguments = others.iter().map(|(member, _)| {
                let parameter = field_ident(member);
                quote! { #parameter.build() }
            });
            let doc = format!(
                " Creates a [`{}`] node with this generator as its `{}` input.",
                node.name, first.name
            );

            let trait_name = first_input
                .accepted
                .as_ref()
                .map(|_| format!("{}Ext", first.name.to_upper_camel_case()));
            methods.entry(trait_name).or_default().push(quote! {
                #[doc = #doc]
                fn #method(&self, #(#parameters),*) -> #name {
                    #name::new(self.build(), #(#arguments),*)
                }
            });
        }

        let marker_traits = restricted.iter().map(|(marker, (member, accepted))| {
            let marker = format_ident!("{marker}");
            let doc = format!(" Generators accepted by `{}` inputs.", member.name);
            let accepted = accepted.iter().map(|node| format_ident!("{node}"));
            quote! {
                #[doc = #doc]
                pub trait #marker: Generator {}

                #(impl #marker for #accepted {})*

                impl<G: #marker + ?Sized> #marker for &G {}
            }
        });

        let ext_traits = methods
            .iter()
            .map(|(trait_name, methods)| match trait_name {
                None => quote! {
                    /// Chaining methods available on every generator.
                    pub trait GeneratorExt: Generator {
                        #(#methods)*
                    }

                    impl<G: Generator + ?Sized> GeneratorExt for G {}
                },
                Some(trait_name) => {
                    let ext = format_ident!("{trait_name}");
                    let marker = format_ident!("{}", trait_name.trim_end_matches("Ext"));
                    let doc = format!(" Chaining methods available on [`{marker}`] generators.");
                    quote! {
                        #[doc = #doc]
                        pub trait #ext: #marker {
                            #(#methods)*
                        }

                        impl<G: #marker + ?Sized> #ext for G {}
                    }
                }
            });

        quote! {
            #![doc = " Chaining methods, creating a node from the generator used as its first input."]

            use super::*;
            use crate::node::Generator;

            #(#marker_traits)*
            #(#ext_traits)*
        }
    }
}

/// Trait bound of an input parameter: any generator, or the marker trait of the accepted nodes.
fn input_bound(input: &Input, member: &Member) -> TokenStream {
    match input.accepted {
        Some(_) => {
            let marker = format_ident!("{}", member.name.to_upper_camel_case());
            quote! { #marker }
        }
        None => quote! { Generator },
    }
}
