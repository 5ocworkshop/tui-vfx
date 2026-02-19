// <FILE>tui-vfx-core/src/schema/fnc_json_write_schema_node.rs</FILE> - <DESC>Write SchemaNode JSON representation</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Add description, json_name, and tag_field to JSON output</CLOG>

use super::cls_json_writer::JsonWriter;
use super::fnc_json_write_range::json_write_range;
use super::fnc_json_write_schema_field::json_write_schema_field;
use super::fnc_json_write_schema_variant::json_write_schema_variant;
use super::types::SchemaNode;
pub(super) fn json_write_schema_node(writer: &mut JsonWriter<'_>, node: &SchemaNode, level: usize) {
    match node {
        SchemaNode::Struct {
            name,
            description,
            json_name,
            fields,
        } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("struct");
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("name");
            writer.out.push_str(": ");
            writer.value_string(name);
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("description");
            writer.out.push_str(": ");
            match description {
                Some(d) => writer.value_string(d),
                None => writer.out.push_str("null"),
            }
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("json_name");
            writer.out.push_str(": ");
            match json_name {
                Some(n) => writer.value_string(n),
                None => writer.out.push_str("null"),
            }
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("fields");
            writer.out.push_str(": [\n");
            for (idx, field) in fields.iter().enumerate() {
                writer.indent(level + 2);
                json_write_schema_field(writer, field, level + 2);
                if idx + 1 != fields.len() {
                    writer.out.push(',');
                }
                writer.out.push('\n');
            }
            writer.indent(level + 1);
            writer.out.push_str("]\n");
            writer.indent(level);
            writer.out.push('}');
        }
        SchemaNode::Enum {
            name,
            description,
            json_name,
            tag_field,
            variants,
        } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("enum");
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("name");
            writer.out.push_str(": ");
            writer.value_string(name);
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("description");
            writer.out.push_str(": ");
            match description {
                Some(d) => writer.value_string(d),
                None => writer.out.push_str("null"),
            }
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("json_name");
            writer.out.push_str(": ");
            match json_name {
                Some(n) => writer.value_string(n),
                None => writer.out.push_str("null"),
            }
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("tag_field");
            writer.out.push_str(": ");
            match tag_field {
                Some(t) => writer.value_string(t),
                None => writer.out.push_str("null"),
            }
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("variants");
            writer.out.push_str(": [\n");
            for (idx, variant) in variants.iter().enumerate() {
                writer.indent(level + 2);
                json_write_schema_variant(writer, variant, level + 2);
                if idx + 1 != variants.len() {
                    writer.out.push(',');
                }
                writer.out.push('\n');
            }
            writer.indent(level + 1);
            writer.out.push_str("]\n");
            writer.indent(level);
            writer.out.push('}');
        }
        SchemaNode::Primitive { type_name, range } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("primitive");
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("type");
            writer.out.push_str(": ");
            writer.value_string(type_name);
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("range");
            writer.out.push_str(": ");
            match range {
                Some(r) => json_write_range(writer, r, level + 1),
                None => writer.out.push_str("null"),
            }
            writer.out.push('\n');
            writer.indent(level);
            writer.out.push('}');
        }
        SchemaNode::Option { inner } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("option");
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("inner");
            writer.out.push_str(": ");
            json_write_schema_node(writer, inner, level + 1);
            writer.out.push('\n');
            writer.indent(level);
            writer.out.push('}');
        }
        SchemaNode::Vec { item } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("vec");
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("item");
            writer.out.push_str(": ");
            json_write_schema_node(writer, item, level + 1);
            writer.out.push('\n');
            writer.indent(level);
            writer.out.push('}');
        }
        SchemaNode::Box { inner } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("box");
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("inner");
            writer.out.push_str(": ");
            json_write_schema_node(writer, inner, level + 1);
            writer.out.push('\n');
            writer.indent(level);
            writer.out.push('}');
        }
        SchemaNode::Opaque { type_name } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("opaque");
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("type");
            writer.out.push_str(": ");
            writer.value_string(type_name);
            writer.out.push('\n');
            writer.indent(level);
            writer.out.push('}');
        }
    }
}

// <FILE>tui-vfx-core/src/schema/fnc_json_write_schema_node.rs</FILE> - <DESC>Write SchemaNode JSON representation</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
