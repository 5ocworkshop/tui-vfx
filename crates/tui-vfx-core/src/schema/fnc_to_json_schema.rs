// <FILE>tui-vfx-core/src/schema/fnc_to_json_schema.rs</FILE> - <DESC>Convert SchemaNode to JSON Schema draft-07</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>OFPF refactoring: extract variant handlers to separate files</WCTX>
// <CLOG>Extract node_to_schema, variant handlers to separate fnc_ files</CLOG>

use serde_json::{Map, Value, json};

use super::fnc_node_to_json_schema::collect_definitions;
use super::types::SchemaNode;

/// Convert a SchemaNode tree to JSON Schema draft-07 format.
///
/// # Arguments
/// * `root` - The root SchemaNode to convert
/// * `root_type_name` - The name to use as the schema title
///
/// # Returns
/// A JSON value conforming to JSON Schema draft-07 specification.
pub fn to_json_schema(root: &SchemaNode, root_type_name: &str) -> Value {
    let mut definitions = Map::new();
    collect_definitions(root, &mut definitions);

    json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": root_type_name,
        "type": "object",
        "$ref": format!("#/definitions/{}", root_type_name),
        "definitions": definitions
    })
}

// <FILE>tui-vfx-core/src/schema/fnc_to_json_schema.rs</FILE> - <DESC>Convert SchemaNode to JSON Schema draft-07</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
