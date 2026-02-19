// <FILE>tui-vfx-core/src/schema/fnc_to_markdown.rs</FILE> - <DESC>Convert SchemaNode to Markdown documentation</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Schema Reference Auto-Generation</WCTX>
// <CLOG>Initial Markdown generator</CLOG>

use super::types::{ScalarValue, SchemaNode, SchemaVariant};
use std::collections::HashSet;

/// Convert a SchemaNode tree to Markdown documentation
pub fn to_markdown(root: &SchemaNode, title: &str) -> String {
    let mut output = String::new();
    let mut visited = HashSet::new();

    // Header
    output.push_str(&format!("# {}\n\n", title));

    // Generate table of contents
    output.push_str("## Table of Contents\n\n");
    collect_toc(root, &mut output, &mut HashSet::new());
    output.push_str("\n---\n\n");

    // Generate type documentation
    generate_type_docs(root, &mut output, &mut visited);

    output
}

fn collect_toc(node: &SchemaNode, output: &mut String, visited: &mut HashSet<String>) {
    match node {
        SchemaNode::Struct { name, fields, .. } => {
            if visited.insert(name.clone()) {
                output.push_str(&format!("- [{}](#{})\n", name, name.to_lowercase()));
                for field in fields {
                    collect_toc(&field.schema, output, visited);
                }
            }
        }
        SchemaNode::Enum { name, variants, .. } => {
            if visited.insert(name.clone()) {
                output.push_str(&format!("- [{}](#{})\n", name, name.to_lowercase()));
                for variant in variants {
                    match variant {
                        SchemaVariant::Struct { fields, .. }
                        | SchemaVariant::Tuple { items: fields, .. } => {
                            for field in fields {
                                collect_toc(&field.schema, output, visited);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        SchemaNode::Option { inner } | SchemaNode::Box { inner } => {
            collect_toc(inner, output, visited);
        }
        SchemaNode::Vec { item } => {
            collect_toc(item, output, visited);
        }
        _ => {}
    }
}

fn generate_type_docs(node: &SchemaNode, output: &mut String, visited: &mut HashSet<String>) {
    match node {
        SchemaNode::Struct {
            name,
            description,
            json_name,
            fields,
        } => {
            if !visited.insert(name.clone()) {
                return;
            }

            output.push_str(&format!("## {}\n\n", name));

            if let Some(desc) = description {
                output.push_str(&format!("{}\n\n", desc));
            }

            if let Some(jn) = json_name {
                if jn != name {
                    output.push_str(&format!("**JSON name:** `{}`\n\n", jn));
                }
            }

            if !fields.is_empty() {
                output.push_str("### Fields\n\n");
                output.push_str("| Field | Type | Required | Default | Description |\n");
                output.push_str("|-------|------|----------|---------|-------------|\n");

                for field in fields {
                    let field_name = field.json_key.as_ref().unwrap_or(&field.name);
                    let type_name = get_type_name(&field.schema);
                    let required = if field.meta.optional { "No" } else { "Yes" };
                    let default = field
                        .meta
                        .default
                        .as_ref()
                        .map(|d| format!("`{}`", scalar_to_string(d)))
                        .unwrap_or_else(|| "-".to_string());
                    let desc = field
                        .meta
                        .description
                        .as_ref()
                        .map(|d| d.replace('\n', " "))
                        .unwrap_or_else(|| "-".to_string());

                    output.push_str(&format!(
                        "| `{}` | {} | {} | {} | {} |\n",
                        field_name, type_name, required, default, desc
                    ));
                }
                output.push('\n');
            }

            output.push_str("---\n\n");

            // Recursively document nested types
            for field in fields {
                generate_type_docs(&field.schema, output, visited);
            }
        }

        SchemaNode::Enum {
            name,
            description,
            json_name,
            tag_field,
            variants,
        } => {
            if !visited.insert(name.clone()) {
                return;
            }

            output.push_str(&format!("## {}\n\n", name));

            if let Some(desc) = description {
                output.push_str(&format!("{}\n\n", desc));
            }

            if let Some(jn) = json_name {
                if jn != name {
                    output.push_str(&format!("**JSON name:** `{}`\n\n", jn));
                }
            }

            if let Some(tag) = tag_field {
                output.push_str(&format!("**Discriminator field:** `{}`\n\n", tag));
            }

            // Check if simple unit enum
            let all_unit = variants
                .iter()
                .all(|v| matches!(v, SchemaVariant::Unit { .. }));

            if all_unit {
                output.push_str("### Values\n\n");
                output.push_str("| Value | Description |\n");
                output.push_str("|-------|-------------|\n");

                for variant in variants {
                    if let SchemaVariant::Unit {
                        name,
                        description,
                        json_value,
                    } = variant
                    {
                        let value = json_value.as_ref().unwrap_or(name);
                        let desc = description
                            .as_ref()
                            .map(|d| d.replace('\n', " "))
                            .unwrap_or_else(|| "-".to_string());
                        output.push_str(&format!("| `{}` | {} |\n", value, desc));
                    }
                }
            } else {
                output.push_str("### Variants\n\n");

                for variant in variants {
                    match variant {
                        SchemaVariant::Unit {
                            name,
                            description,
                            json_value,
                        } => {
                            let value = json_value.as_ref().unwrap_or(name);
                            output.push_str(&format!("#### `{}`\n\n", value));
                            if let Some(desc) = description {
                                output.push_str(&format!("{}\n\n", desc));
                            }
                        }
                        SchemaVariant::Struct {
                            name,
                            description,
                            json_value,
                            fields,
                        } => {
                            let value = json_value.as_ref().unwrap_or(name);
                            output.push_str(&format!("#### `{}`\n\n", value));
                            if let Some(desc) = description {
                                output.push_str(&format!("{}\n\n", desc));
                            }
                            if !fields.is_empty() {
                                output.push_str("| Field | Type | Required | Description |\n");
                                output.push_str("|-------|------|----------|-------------|\n");
                                for field in fields {
                                    let field_name = field.json_key.as_ref().unwrap_or(&field.name);
                                    let type_name = get_type_name(&field.schema);
                                    let required = if field.meta.optional { "No" } else { "Yes" };
                                    let desc = field
                                        .meta
                                        .description
                                        .as_ref()
                                        .map(|d| d.replace('\n', " "))
                                        .unwrap_or_else(|| "-".to_string());
                                    output.push_str(&format!(
                                        "| `{}` | {} | {} | {} |\n",
                                        field_name, type_name, required, desc
                                    ));
                                }
                                output.push('\n');
                            }
                        }
                        SchemaVariant::Tuple {
                            name,
                            description,
                            json_value,
                            items,
                        } => {
                            let value = json_value.as_ref().unwrap_or(name);
                            output.push_str(&format!("#### `{}`\n\n", value));
                            if let Some(desc) = description {
                                output.push_str(&format!("{}\n\n", desc));
                            }
                            if !items.is_empty() {
                                output.push_str("**Tuple fields:**\n\n");
                                for (i, item) in items.iter().enumerate() {
                                    output.push_str(&format!(
                                        "{}. {}\n",
                                        i,
                                        get_type_name(&item.schema)
                                    ));
                                }
                                output.push('\n');
                            }
                        }
                    }
                }
            }

            output.push_str("---\n\n");

            // Recursively document nested types
            for variant in variants {
                match variant {
                    SchemaVariant::Struct { fields, .. }
                    | SchemaVariant::Tuple { items: fields, .. } => {
                        for field in fields {
                            generate_type_docs(&field.schema, output, visited);
                        }
                    }
                    _ => {}
                }
            }
        }

        SchemaNode::Option { inner } | SchemaNode::Box { inner } => {
            generate_type_docs(inner, output, visited);
        }
        SchemaNode::Vec { item } => {
            generate_type_docs(item, output, visited);
        }
        _ => {}
    }
}

fn get_type_name(node: &SchemaNode) -> String {
    match node {
        SchemaNode::Primitive { type_name, .. } => format!("`{}`", type_name),
        SchemaNode::Option { inner } => format!("{}?", get_type_name(inner)),
        SchemaNode::Vec { item } => format!("[{}]", get_type_name(item)),
        SchemaNode::Box { inner } => get_type_name(inner),
        SchemaNode::Struct { name, .. } | SchemaNode::Enum { name, .. } => {
            format!("[`{}`](#{})", name, name.to_lowercase())
        }
        SchemaNode::Opaque { type_name } => format!("`{}`", type_name),
    }
}

fn scalar_to_string(scalar: &ScalarValue) -> String {
    match scalar {
        ScalarValue::Bool(b) => b.to_string(),
        ScalarValue::Number(n) => n.clone(),
        ScalarValue::String(s) => format!("\"{}\"", s),
        ScalarValue::Char(c) => format!("'{}'", c),
    }
}

// <FILE>tui-vfx-core/src/schema/fnc_to_markdown.rs</FILE> - <DESC>Convert SchemaNode to Markdown documentation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
