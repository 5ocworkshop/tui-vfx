// <FILE>tui-vfx-core/src/schema/fnc_json_write_schema_variant.rs</FILE> - <DESC>Write SchemaVariant JSON representation</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Add description and json_value fields to JSON output</CLOG>

use super::cls_json_writer::JsonWriter;
use super::fnc_json_write_schema_field::json_write_schema_field;
use super::types::SchemaVariant;
pub(super) fn json_write_schema_variant(
    writer: &mut JsonWriter<'_>,
    variant: &SchemaVariant,
    level: usize,
) {
    match variant {
        SchemaVariant::Unit {
            name,
            description,
            json_value,
        } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("unit");
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
            writer.value_string("json_value");
            writer.out.push_str(": ");
            match json_value {
                Some(v) => writer.value_string(v),
                None => writer.out.push_str("null"),
            }
            writer.out.push('\n');
            writer.indent(level);
            writer.out.push('}');
        }
        SchemaVariant::Tuple {
            name,
            description,
            json_value,
            items,
        } => {
            writer.out.push_str("{\n");
            writer.indent(level + 1);
            writer.value_string("kind");
            writer.out.push_str(": ");
            writer.value_string("tuple");
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
            writer.value_string("json_value");
            writer.out.push_str(": ");
            match json_value {
                Some(v) => writer.value_string(v),
                None => writer.out.push_str("null"),
            }
            writer.out.push_str(",\n");
            writer.indent(level + 1);
            writer.value_string("items");
            writer.out.push_str(": [\n");
            for (idx, item) in items.iter().enumerate() {
                writer.indent(level + 2);
                json_write_schema_field(writer, item, level + 2);
                if idx + 1 != items.len() {
                    writer.out.push(',');
                }
                writer.out.push('\n');
            }
            writer.indent(level + 1);
            writer.out.push_str("]\n");
            writer.indent(level);
            writer.out.push('}');
        }
        SchemaVariant::Struct {
            name,
            description,
            json_value,
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
            writer.value_string("json_value");
            writer.out.push_str(": ");
            match json_value {
                Some(v) => writer.value_string(v),
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
    }
}

// <FILE>tui-vfx-core/src/schema/fnc_json_write_schema_variant.rs</FILE> - <DESC>Write SchemaVariant JSON representation</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
