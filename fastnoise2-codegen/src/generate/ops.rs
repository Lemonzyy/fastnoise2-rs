//! Generates `ops.rs`: the arithmetic operators.
use heck::ToSnakeCase;
use quote::{format_ident, quote};

use super::{
    Generator,
    render::{HEADER, render_items},
};
use crate::metadata::MemberKind;

impl Generator<'_> {
    /// Arithmetic operators on every generator, creating the matching operator nodes.
    ///
    /// Nodes whose `LHS` is an input (e.g. Add) are created from the left operand, and swap the
    /// operands when the constant is on the left. Nodes whose `LHS` is a hybrid (e.g. Subtract)
    /// set both sides.
    pub(super) fn ops_file(&self) -> String {
        const OPERATORS: [(&str, &str, &str, bool); 5] = [
            ("Add", "add", "Add", true),
            ("Sub", "sub", "Subtract", false),
            ("Mul", "mul", "Multiply", true),
            ("Div", "div", "Divide", false),
            ("Rem", "rem", "Modulus", false),
        ];

        let impls = OPERATORS
            .iter()
            .map(|&(op_trait, op_method, node_name, commutative)| {
                let node = self
                    .nodes
                    .iter()
                    .find(|node| node.name == node_name)
                    .unwrap_or_else(|| panic!("missing {node_name} node"));
                let member = |name: &str| {
                    node.members
                        .iter()
                        .find(|member| member.name == name)
                        .unwrap_or_else(|| panic!("missing {name} member of {node_name}"))
                };
                let (lhs, rhs) = (member("LHS"), member("RHS"));

                let op_trait = format_ident!("{op_trait}");
                let op_method = format_ident!("{op_method}");
                let output = format_ident!("{node_name}");
                let with_rhs = format_ident!("with_{}", rhs.name.to_snake_case());

                // (generator on the left, constant on the left)
                let (generator_lhs, constant_lhs) = match lhs.kind {
                    MemberKind::Input(_) => {
                        assert!(commutative, "{node_name} needs its LHS input on the left");
                        (
                            quote! { #output::new(self.build()).#with_rhs(rhs) },
                            quote! { #output::new(rhs.build()).#with_rhs(self) },
                        )
                    }
                    _ => {
                        let constructor = format_ident!("{}", node_name.to_snake_case());
                        let with_lhs = format_ident!("with_{}", lhs.name.to_snake_case());
                        let body = quote! { #constructor().#with_lhs(self).#with_rhs(rhs) };
                        (body.clone(), body)
                    }
                };

                quote! {
                    impl<R: Into<Hybrid>> ops::#op_trait<R> for __Generator {
                        type Output = #output;

                        fn #op_method(self, rhs: R) -> #output {
                            #generator_lhs
                        }
                    }

                    impl<R: Into<Hybrid>> ops::#op_trait<R> for &__Generator {
                        type Output = #output;

                        fn #op_method(self, rhs: R) -> #output {
                            #generator_lhs
                        }
                    }

                    impl ops::#op_trait<__Generator> for f32 {
                        type Output = #output;

                        fn #op_method(self, rhs: __Generator) -> #output {
                            #constant_lhs
                        }
                    }

                    impl ops::#op_trait<&__Generator> for f32 {
                        type Output = #output;

                        fn #op_method(self, rhs: &__Generator) -> #output {
                            #constant_lhs
                        }
                    }
                }
            });

        let generators = self.nodes.iter().map(|node| format_ident!("{}", node.name));

        let impls = render_items(quote! {
            #(#impls)*

            /// Negation, multiplying by -1.
            impl ops::Neg for __Generator {
                type Output = Multiply;

                fn neg(self) -> Multiply {
                    self * -1.0
                }
            }

            /// Negation, multiplying by -1.
            impl ops::Neg for &__Generator {
                type Output = Multiply;

                fn neg(self) -> Multiply {
                    self * -1.0
                }
            }
        });
        let macro_body = impls
            .replace("__Generator", "$generator")
            .lines()
            .map(|line| match line {
                "" => String::new(),
                line => format!("        {line}"),
            })
            .collect::<Vec<_>>()
            .join("\n");

        let uses = render_items(quote! {
            #![doc = " Arithmetic operators on generators, with constants on either side."]

            use std::ops;

            use super::*;
            use crate::node::{Generator, Hybrid, Node};
        });
        let invocation = render_items(quote! {
            operators!(Node, #(#generators),*);
        });

        format!(
            "{HEADER}{uses}\nmacro_rules! operators {{\n    ($($generator:ty),*) => {{$(\n{macro_body}\n    )*}};\n}}\n\n{invocation}"
        )
    }
}
