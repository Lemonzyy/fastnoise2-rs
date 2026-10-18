// This example illustrates the FastNoise2 metadata available at runtime: every node type with its
// groups and members, which is what a node editor UI or a dynamic configuration needs.
use fastnoise2::{MemberRange, MemberValue, Metadata};

fn main() {
    for metadata in Metadata::all() {
        println!("{} [{}]", metadata.name(), metadata.groups().join(", "));

        for member in metadata.members() {
            let default = match member.default_value() {
                Some(MemberValue::Float(value)) => format!(" = {value}"),
                Some(MemberValue::Int(value)) => format!(" = {value}"),
                Some(MemberValue::Enum(value)) => {
                    format!(" = {value} (one of {})", member.enum_values().join(", "))
                }
                Some(MemberValue::Node(_)) | None => String::new(),
            };

            let range = match member.range() {
                Some(MemberRange::Float(range)) => format!(", in {range:?}"),
                Some(MemberRange::Int(range)) => format!(", in {range:?}"),
                None => String::new(),
            };

            println!(
                "  {}: {}{default}{range}",
                member.name(),
                member.member_type()
            );
        }
    }
}
