// <FILE>tui-vfx-core/src/schema/fnc_schema_node_to_json_pretty.rs</FILE>
// <DESC>Serialize a SchemaNode into stable pretty JSON</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Stable output for tests + tooling</WCTX>
// <CLOG>Extracted SchemaNode to_json_pretty implementation</CLOG>

use super::cls_json_writer::JsonWriter;
use super::fnc_json_write_schema_node::json_write_schema_node;
use super::types::SchemaNode;

pub(super) fn schema_node_to_json_pretty(node: &SchemaNode) -> String {
    let mut out = String::new();
    let mut writer = JsonWriter::new(&mut out);
    json_write_schema_node(&mut writer, node, 0);
    out
}

// <FILE>tui-vfx-core/src/schema/fnc_schema_node_to_json_pretty.rs</FILE>
// <DESC>Serialize a SchemaNode into stable pretty JSON</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
