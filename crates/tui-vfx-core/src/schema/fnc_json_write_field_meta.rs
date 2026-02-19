// <FILE>tui-vfx-core/src/schema/fnc_json_write_field_meta.rs</FILE>
// <DESC>Write FieldMeta JSON representation</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Add description, json_key, and optional fields to JSON output</CLOG>

use super::cls_json_writer::JsonWriter;
use super::fnc_json_write_range::json_write_range;
use super::types::FieldMeta;

pub(super) fn json_write_field_meta(writer: &mut JsonWriter<'_>, meta: &FieldMeta, level: usize) {
    writer.out.push_str("{\n");

    writer.indent(level + 1);
    writer.value_string("help");
    writer.out.push_str(": ");
    match &meta.help {
        Some(h) => writer.value_string(h),
        None => writer.out.push_str("null"),
    }
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("description");
    writer.out.push_str(": ");
    match &meta.description {
        Some(d) => writer.value_string(d),
        None => writer.out.push_str("null"),
    }
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("default");
    writer.out.push_str(": ");
    match &meta.default {
        Some(d) => writer.value_scalar(d),
        None => writer.out.push_str("null"),
    }
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("range");
    writer.out.push_str(": ");
    match &meta.range {
        Some(r) => json_write_range(writer, r, level + 1),
        None => writer.out.push_str("null"),
    }
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("json_key");
    writer.out.push_str(": ");
    match &meta.json_key {
        Some(k) => writer.value_string(k),
        None => writer.out.push_str("null"),
    }
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("optional");
    writer.out.push_str(": ");
    writer
        .out
        .push_str(if meta.optional { "true" } else { "false" });
    writer.out.push('\n');
    writer.indent(level);
    writer.out.push('}');
}

// <FILE>tui-vfx-core/src/schema/fnc_json_write_field_meta.rs</FILE>
// <DESC>Write FieldMeta JSON representation</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
