// <FILE>tui-vfx-core/src/schema/fnc_variant_to_untagged_schema.rs</FILE> - <DESC>Convert SchemaVariant to untagged JSON Schema</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OFPF refactoring: extract variant handlers from fnc_to_json_schema.rs</WCTX>
// <CLOG>Initial extraction of variant_to_untagged_schema</CLOG>

use serde_json::{Map, Value, json};

use super::fnc_node_to_json_schema::node_to_schema;
use super::types::SchemaVariant;

/// Convert a variant to an untagged schema.
///
/// Creates a JSON Schema without a discriminator tag field.
pub fn variant_to_untagged_schema(
    variant: &SchemaVariant,
    definitions: &mut Map<String, Value>,
) -> Value {
    match variant {
        SchemaVariant::Unit {
            name: _,
            description,
            json_value,
        } => {
            let value = json_value
                .as_ref()
                .map_or_else(|| json!(null), |v| json!(v));
            let mut schema = json!({ "const": value });
            if let Some(desc) = description {
                schema["description"] = json!(desc);
            }
            schema
        }
        SchemaVariant::Struct {
            name: _,
            description,
            json_value: _,
            fields,
        } => {
            let mut properties = Map::new();
            let mut required = Vec::new();

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
                "properties": properties
            });

            if !required.is_empty() {
                schema["required"] = json!(required);
            }

            if let Some(desc) = description {
                schema["description"] = json!(desc);
            }

            schema
        }
        SchemaVariant::Tuple {
            name: _,
            description,
            json_value: _,
            items,
        } => {
            let items_schema: Vec<Value> = items
                .iter()
                .map(|f| node_to_schema(&f.schema, definitions))
                .collect();

            let mut schema = json!({
                "type": "array",
                "items": items_schema,
                "minItems": items.len(),
                "maxItems": items.len()
            });

            if let Some(desc) = description {
                schema["description"] = json!(desc);
            }

            schema
        }
    }
}

// <FILE>tui-vfx-core/src/schema/fnc_variant_to_untagged_schema.rs</FILE> - <DESC>Convert SchemaVariant to untagged JSON Schema</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
