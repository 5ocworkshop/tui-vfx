// <FILE>tui-vfx-core/src/schema/fnc_json_write_schema_field.rs</FILE>
// <DESC>Write SchemaField JSON representation</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Add json_key field to JSON output</CLOG>

use super::cls_json_writer::JsonWriter;
use super::fnc_json_write_field_meta::json_write_field_meta;
use super::fnc_json_write_schema_node::json_write_schema_node;
use super::types::SchemaField;

pub(super) fn json_write_schema_field(
    writer: &mut JsonWriter<'_>,
    field: &SchemaField,
    level: usize,
) {
    writer.out.push_str("{\n");
    writer.indent(level + 1);
    writer.value_string("name");
    writer.out.push_str(": ");
    writer.value_string(&field.name);
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("json_key");
    writer.out.push_str(": ");
    match &field.json_key {
        Some(k) => writer.value_string(k),
        None => writer.out.push_str("null"),
    }
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("meta");
    writer.out.push_str(": ");
    json_write_field_meta(writer, &field.meta, level + 1);
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("schema");
    writer.out.push_str(": ");
    json_write_schema_node(writer, &field.schema, level + 1);
    writer.out.push('\n');
    writer.indent(level);
    writer.out.push('}');
}

// <FILE>tui-vfx-core/src/schema/fnc_json_write_schema_field.rs</FILE>
// <DESC>Write SchemaField JSON representation</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
