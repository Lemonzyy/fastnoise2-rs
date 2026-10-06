//! Generates the group files: the node types.
use std::collections::BTreeSet;

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    Generator,
    tokens::{doc_lines, field_ident},
    types::{FieldType, range_doc},
};
use crate::metadata::{Member, MemberKind, Node};

impl Generator<'_> {
    pub(super) fn group_file(&self, group: &str, nodes: &[&Node]) -> TokenStream {
        let module_doc = format!(" {group} nodes.");

        let enum_names = nodes
            .iter()
            .flat_map(|node| &node.members)
            .filter_map(|member| match self.field_type(member) {
                FieldType::Enum(enum_type) => Some(enum_type.name.as_str()),
                _ => None,
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|name| format_ident!("{name}"))
            .collect::<Vec<_>>();
        let enum_import = (!enum_names.is_empty()).then(|| {
            quote! { use super::enums::{#(#enum_names),*}; }
        });

        let has_hybrid = nodes
            .iter()
            .flat_map(|node| &node.members)
            .any(|member| matches!(member.kind, MemberKind::Hybrid { .. }));
        let core_import = if has_hybrid {
            quote! { use crate::node::{Generator, Hybrid, Node, NodeBuilder}; }
        } else {
            quote! { use crate::node::{Generator, Node, NodeBuilder}; }
        };

        let nodes = nodes.iter().map(|node| self.node(node));

        quote! {
            #![doc = #module_doc]

            #enum_import
            #core_import

            #(#nodes)*
        }
    }

    fn node(&self, node: &Node) -> TokenStream {
        let name = format_ident!("{}", node.name);
        let node_name = &node.name;
        let doc = if node.description.is_empty() {
            vec![format!(" {} node.", node.name)]
        } else {
            doc_lines(&node.description)
        };

        let fields = node.members.iter().map(|member| {
            let field = field_ident(member);
            let rust_type = self.rust_type(member);
            quote! { #field: #rust_type }
        });

        // Every member set to its default value, inputs set from the constructor parameters
        let initializers = node
            .members
            .iter()
            .map(|member| {
                let field = field_ident(member);
                match self.default_value(member) {
                    Some(default) => quote! { #field: #default },
                    None => quote! { #field },
                }
            })
            .collect::<Vec<_>>();

        let inputs = node
            .inputs()
            .map(|(member, _)| field_ident(member))
            .collect::<Vec<_>>();
        let (default, constructor) = if inputs.is_empty() {
            let function = format_ident!("{}", node.name.to_snake_case());
            let function_doc =
                format!(" Creates a [`{}`] node with its default values.", node.name);
            let default = quote! {
                impl Default for #name {
                    fn default() -> Self {
                        Self { #(#initializers,)* }
                    }
                }

                #[doc = #function_doc]
                pub fn #function() -> #name {
                    #name::default()
                }
            };
            (Some(default), None)
        } else {
            let constructor = quote! {
                pub(crate) fn new(#(#inputs: Node),*) -> Self {
                    Self { #(#initializers,)* }
                }
            };
            (None, Some(constructor))
        };

        let builders = node
            .members
            .iter()
            .filter(|member| !matches!(member.kind, MemberKind::Input(_)))
            .map(|member| self.builder(member));

        let sets = node.members.iter().map(|member| {
            let member_name = &member.name;
            let field = field_ident(member);
            let value = match self.field_type(member) {
                FieldType::Float | FieldType::Int => quote! { self.#field },
                FieldType::Bool => quote! { if self.#field { "True" } else { "False" } },
                FieldType::Enum(_) => quote! { self.#field.name() },
                FieldType::Input | FieldType::Hybrid => quote! { &self.#field },
            };
            quote! { .and_then(|builder| builder.set(#member_name, #value)) }
        });

        quote! {
            #(#[doc = #doc])*
            #[derive(Clone, Debug)]
            pub struct #name {
                #(#fields,)*
            }

            #default

            impl #name {
                #constructor
                #(#builders)*
            }

            impl Generator for #name {
                fn build(&self) -> Node {
                    NodeBuilder::new(#node_name)
                        #(#sets)*
                        .and_then(NodeBuilder::build)
                        .unwrap_or_else(|error| panic!("{error}"))
                }
            }
        }
    }

    fn builder(&self, member: &Member) -> TokenStream {
        let field = field_ident(member);
        let parameter = format_ident!("{}", member.name.to_snake_case());
        let method = format_ident!("with_{}", member.name.to_snake_case());

        let mut doc = doc_lines(&member.description);
        if !doc.is_empty() {
            doc.push(String::new());
        }
        doc.push(format!(" Default: `{}`", self.default_doc(member)));
        if let Some(range) = range_doc(member) {
            doc.push(String::new());
            doc.push(format!(
                " Node Editor range: `{range}`, not enforced by FastNoise2"
            ));
        }

        match self.field_type(member) {
            FieldType::Hybrid => quote! {
                #(#[doc = #doc])*
                pub fn #method(mut self, #parameter: impl Into<Hybrid>) -> Self {
                    self.#field = #parameter.into();
                    self
                }
            },
            _ => {
                let rust_type = self.rust_type(member);
                quote! {
                    #(#[doc = #doc])*
                    pub fn #method(mut self, #parameter: #rust_type) -> Self {
                        self.#field = #parameter;
                        self
                    }
                }
            }
        }
    }
}
