// <FILE>tui-vfx-core/src/schema/fnc_json_write_range.rs</FILE>
// <DESC>Write Range JSON representation</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Stable schema JSON</WCTX>
// <CLOG>Extracted Range JSON writer</CLOG>

use super::cls_json_writer::JsonWriter;
use super::types::Range;

pub(super) fn json_write_range(writer: &mut JsonWriter<'_>, range: &Range, level: usize) {
    writer.out.push_str("{\n");

    writer.indent(level + 1);
    writer.value_string("min");
    writer.out.push_str(": ");
    match &range.min {
        Some(v) => writer.value_scalar(v),
        None => writer.out.push_str("null"),
    }
    writer.out.push_str(",\n");

    writer.indent(level + 1);
    writer.value_string("max");
    writer.out.push_str(": ");
    match &range.max {
        Some(v) => writer.value_scalar(v),
        None => writer.out.push_str("null"),
    }
    writer.out.push('\n');
    writer.indent(level);
    writer.out.push('}');
}

// <FILE>tui-vfx-core/src/schema/fnc_json_write_range.rs</FILE>
// <DESC>Write Range JSON representation</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
