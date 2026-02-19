// <FILE>tui-vfx-core/src/schema/fnc_variant_to_tagged_schema.rs</FILE> - <DESC>Convert SchemaVariant to tagged JSON Schema</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OFPF refactoring: extract variant handlers from fnc_to_json_schema.rs</WCTX>
// <CLOG>Initial extraction of variant_to_schema for tagged enums</CLOG>

use serde_json::{Map, Value, json};

use super::fnc_node_to_json_schema::node_to_schema;
use super::types::SchemaVariant;

/// Convert a variant to a tagged schema.
///
/// Creates a JSON Schema object where the tag field identifies the variant type.
pub fn variant_to_tagged_schema(
    variant: &SchemaVariant,
    tag: &str,
    definitions: &mut Map<String, Value>,
) -> Value {
    match variant {
        SchemaVariant::Unit {
            name,
            description,
            json_value,
        } => {
            let value = json_value.as_ref().unwrap_or(name);
            let mut schema = json!({
                "type": "object",
                "properties": {
                    tag: { "const": value }
                },
                "required": [tag]
            });
            if let Some(desc) = description {
                schema["description"] = json!(desc);
            }
            schema
        }
        SchemaVariant::Struct {
            name,
            description,
            json_value,
            fields,
        } => {
            let value = json_value.as_ref().unwrap_or(name);
            let mut properties = Map::new();
            let mut required = vec![tag.to_string()];

            properties.insert(tag.to_string(), json!({ "const": value }));

            for field in fields {
                let field_name = field.json_key.as_ref().unwrap_or(&field.name);
                let mut field_schema = node_to_schema(&field.schema, definitions);

                if let Some(desc) = &field.meta.description {
                    if let Value::Object(ref mut obj) = field_schema {
                        obj.insert("description".to_string(), json!(desc));
                    }
                }

                properties.insert(field_name.clone(), field_schema);
                if !field.meta.optional {
                    required.push(field_name.clone());
                }
            }

            let mut schema = json!({
                "type": "object",
                "properties": properties,
                "required": required
            });
            if let Some(desc) = description {
                schema["description"] = json!(desc);
            }
            schema
        }
        SchemaVariant::Tuple {
            name,
            description,
            json_value,
            items,
        } => {
            let value = json_value.as_ref().unwrap_or(name);
            let items_schema: Vec<Value> = items
                .iter()
                .map(|f| node_to_schema(&f.schema, definitions))
                .collect();

            let mut schema = json!({
                "type": "object",
                "properties": {
                    tag: { "const": value },
                    "value": {
                        "type": "array",
                        "items": items_schema,
                        "minItems": items.len(),
                        "maxItems": items.len()
                    }
                },
                "required": [tag, "value"]
            });
            if let Some(desc) = description {
                schema["description"] = json!(desc);
            }
            schema
        }
    }
}

// <FILE>tui-vfx-core/src/schema/fnc_variant_to_tagged_schema.rs</FILE> - <DESC>Convert SchemaVariant to tagged JSON Schema</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
