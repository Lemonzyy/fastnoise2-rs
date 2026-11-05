//! Generates the Rust source files of the typed node API.
use std::collections::{BTreeMap, BTreeSet};

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::metadata::{MemberKind, Node};

mod enums;
mod ext;
mod node_tests;
mod nodes;
mod ops;
mod render;
mod tokens;
mod types;

use render::render;
use types::is_bool;

/// An enum member type, shared by every node using the same name and values.
struct EnumType {
    name: String,
    values: Vec<String>,
    description: String,
    nodes: Vec<String>,
}

pub struct Generator<'a> {
    nodes: &'a [Node],
    enums: BTreeMap<String, EnumType>,
}

impl<'a> Generator<'a> {
    pub fn new(nodes: &'a [Node]) -> Self {
        let node_names = nodes
            .iter()
            .map(|node| node.name.as_str())
            .collect::<BTreeSet<_>>();
        let mut enums = BTreeMap::<String, EnumType>::new();

        for node in nodes {
            for member in &node.members {
                let MemberKind::Enum { values, .. } = &member.kind else {
                    continue;
                };
                if is_bool(values) {
                    continue;
                }

                let mut name = member.name.to_upper_camel_case();
                if node_names.contains(name.as_str()) {
                    name.push_str("Type");
                }

                let enum_type = enums.entry(name.clone()).or_insert_with(|| EnumType {
                    name,
                    values: values.clone(),
                    description: member.description.clone(),
                    nodes: Vec::new(),
                });
                assert_eq!(
                    &enum_type.values, values,
                    "enum {} has different values",
                    enum_type.name
                );
                enum_type.nodes.push(node.name.clone());
            }
        }

        Self { nodes, enums }
    }

    /// Returns the generated files, as (file name, content).
    pub fn files(&self) -> Vec<(String, String)> {
        let mut groups = BTreeMap::<String, Vec<&Node>>::new();
        for node in self.nodes {
            groups
                .entry(node.group.to_snake_case())
                .or_default()
                .push(node);
        }

        let mut files = vec![
            ("mod.rs".to_string(), render(self.module(&groups))),
            ("enums.rs".to_string(), render(self.enums_file())),
            ("ext.rs".to_string(), render(self.ext_file())),
            ("ops.rs".to_string(), self.ops_file()),
            ("tests.rs".to_string(), render(self.tests_file())),
        ];
        for (module, nodes) in &groups {
            let content = render(self.group_file(&nodes[0].group, nodes));
            files.push((format!("{module}.rs"), content));
        }

        files
    }

    fn module(&self, groups: &BTreeMap<String, Vec<&Node>>) -> TokenStream {
        let modules = groups
            .keys()
            .map(|module| format_ident!("{module}"))
            .collect::<Vec<_>>();

        quote! {
            #![doc = " Typed FastNoise2 nodes, generated from FastNoise2 metadata."]

            pub mod enums;
            pub mod ext;
            mod ops;
            #(pub mod #modules;)*

            pub use enums::*;
            pub use ext::*;
            #(pub use #modules::*;)*

            #[cfg(test)]
            mod tests;
        }
    }
}
