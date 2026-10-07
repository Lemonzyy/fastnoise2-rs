//! Generates `tests.rs`: the tests of the typed nodes.
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    Generator,
    tokens::{float_literal, int_literal},
    types::FieldType,
};
use crate::metadata::{Input, Member, MemberKind, Node};

impl Generator<'_> {
    /// Tests comparing every typed node and builder method with the same node built by name.
    ///
    /// Each builder test sets one member to a non default value, on the typed node and on a
    /// [`NodeBuilder`] with every other member at its default, and checks both outputs match.
    pub(super) fn tests_file(&self) -> TokenStream {
        let tests = self.nodes.iter().flat_map(|node| {
            let default_test = format_ident!("{}_default", node.name.to_snake_case());
            let typed = self.typed_node(node);
            let manual = self.manual_node(node, None);
            let default = quote! {
                #[test]
                fn #default_test() {
                    assert_same(#typed, #manual);
                }
            };

            let member_tests = node
                .members
                .iter()
                .filter(|member| !matches!(member.kind, MemberKind::Input(_)))
                .map(move |member| {
                    let test = format_ident!(
                        "{}_{}",
                        node.name.to_snake_case(),
                        member.name.to_snake_case()
                    );
                    let method = format_ident!("with_{}", member.name.to_snake_case());
                    let (typed_value, member_value) = self.test_value(member);
                    let typed = self.typed_node(node);
                    let manual = self.manual_node(node, Some((member, member_value)));
                    quote! {
                        #[test]
                        fn #test() {
                            assert_same(#typed.#method(#typed_value), #manual);
                        }
                    }
                });

            std::iter::once(default).chain(member_tests)
        });

        quote! {
            #![doc = " Tests comparing typed nodes with nodes built by name."]

            use super::*;
            use crate::node::{Generator, MemberValue, Node, NodeBuilder};

            fn source() -> Node {
                perlin().build()
            }

            fn domain_warp() -> DomainWarpGradient {
                perlin().domain_warp_gradient()
            }

            fn hybrid() -> Node {
                sine_wave().build()
            }

            fn assert_same(typed: impl Generator, manual: NodeBuilder) {
                let grid = |node: &Node| {
                    let mut output = vec![0.0; 64];
                    node.gen_uniform_grid_2d(&mut output, -40.0, -40.0, 8, 8, 10.0, 10.0, 1337);
                    output
                };
                let typed = grid(&typed.build());
                let manual = grid(&manual.build().unwrap());
                let same = typed
                    .iter()
                    .zip(&manual)
                    .all(|(typed, manual)| typed == manual || (typed.is_nan() && manual.is_nan()));
                assert!(same, "typed {typed:?} != manual {manual:?}");
            }

            #(#tests)*
        }
    }

    /// Typed node with every member at its default, inputs from the test helpers.
    fn typed_node(&self, node: &Node) -> TokenStream {
        let inputs = node.inputs().collect::<Vec<_>>();
        let Some(((_, first_input), others)) = inputs.split_first() else {
            let function = format_ident!("{}", node.name.to_snake_case());
            return quote! { #function() };
        };

        let method = format_ident!("{}", node.name.to_snake_case());
        let receiver = test_input(first_input);
        let arguments = others.iter().map(|(_, input)| test_input(input));
        quote! { #receiver.#method(#(#arguments),*) }
    }

    /// The same node built by name, every member set to its default, `changed` set last.
    fn manual_node(&self, node: &Node, changed: Option<(&Member, TokenStream)>) -> TokenStream {
        let node_name = &node.name;
        let sets = node.members.iter().map(|member| {
            let member_name = &member.name;
            let value = match &member.kind {
                MemberKind::Input(input) => test_input(input),
                _ => self.member_value(member),
            };
            quote! { .set(#member_name, #value).unwrap() }
        });
        let changed = changed.map(|(member, value)| {
            let member_name = &member.name;
            quote! { .set(#member_name, #value).unwrap() }
        });

        quote! {
            NodeBuilder::new(#node_name).unwrap()
                #(#sets)*
                #changed
        }
    }

    /// `MemberValue` of the default value of a member.
    fn member_value(&self, member: &Member) -> TokenStream {
        match (&member.kind, self.field_type(member)) {
            (MemberKind::Float { default, .. } | MemberKind::Hybrid { default }, _) => {
                let default = float_literal(*default);
                quote! { MemberValue::Float(#default) }
            }
            (MemberKind::Int { default, .. }, _) => {
                let default = int_literal(*default);
                quote! { MemberValue::Int(#default) }
            }
            (MemberKind::Enum { default, .. }, FieldType::Bool) => {
                let value = if *default == 1 { "True" } else { "False" };
                quote! { MemberValue::from(#value) }
            }
            (MemberKind::Enum { values, default }, _) => {
                let value = &values[*default];
                quote! { MemberValue::from(#value) }
            }
            (MemberKind::Input(_), _) => unreachable!("inputs are set from the test helpers"),
        }
    }

    /// A non default test value, as (typed builder argument, `MemberValue`).
    fn test_value(&self, member: &Member) -> (TokenStream, TokenStream) {
        match (&member.kind, self.field_type(member)) {
            (MemberKind::Float { default, .. }, _) => {
                let value = float_literal(default + 1.5);
                (quote! { #value }, quote! { MemberValue::Float(#value) })
            }
            (MemberKind::Int { default, .. }, _) => {
                let value = int_literal(default + 1);
                (quote! { #value }, quote! { MemberValue::Int(#value) })
            }
            (MemberKind::Enum { default, .. }, FieldType::Bool) => {
                let value = *default != 1;
                let name = if value { "True" } else { "False" };
                (quote! { #value }, quote! { MemberValue::from(#name) })
            }
            (MemberKind::Enum { values, default }, FieldType::Enum(enum_type)) => {
                let value = &values[(default + 1) % values.len()];
                let enum_name = format_ident!("{}", enum_type.name);
                let variant = format_ident!("{}", value.to_upper_camel_case());
                (
                    quote! { #enum_name::#variant },
                    quote! { MemberValue::from(#value) },
                )
            }
            (MemberKind::Hybrid { .. }, _) => {
                (quote! { hybrid() }, quote! { MemberValue::from(hybrid()) })
            }
            _ => unreachable!("inputs have no test value"),
        }
    }
}

/// Test helper providing a node for an input.
fn test_input(input: &Input) -> TokenStream {
    match input.accepted {
        Some(_) => quote! { domain_warp() },
        None => quote! { source() },
    }
}
