// <FILE>tui-vfx-core/src/schema/fnc_node_to_json_schema.rs</FILE> - <DESC>Convert SchemaNode to JSON Schema value</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OFPF refactoring: extract node conversion from fnc_to_json_schema.rs</WCTX>
// <CLOG>Initial extraction of node_to_schema and collect_definitions</CLOG>

use serde_json::{Map, Value, json};

use super::fnc_variant_to_tagged_schema::variant_to_tagged_schema;
use super::fnc_variant_to_untagged_schema::variant_to_untagged_schema;
use super::types::{Range, ScalarValue, SchemaNode, SchemaVariant};

/// Recursively collect all type definitions from a SchemaNode tree.
pub fn collect_definitions(node: &SchemaNode, definitions: &mut Map<String, Value>) {
    match node {
        SchemaNode::Struct {
            name,
            description,
            json_name: _,
            fields,
        } => {
            if definitions.contains_key(name) {
                return;
            }

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

                if let Some(default) = &field.meta.default {
                    if let Value::Object(ref mut obj) = field_schema {
                        obj.insert("default".to_string(), scalar_to_json(default));
                    }
                }

                if let Some(range) = &field.meta.range {
                    if let Value::Object(ref mut obj) = field_schema {
                        apply_range_constraints(obj, range);
                    }
                }

                properties.insert(field_name.clone(), field_schema);

                if !field.meta.optional {
                    required.push(field_name.clone());
                }
            }

            let mut schema_def = json!({
                "type": "object",
                "properties": properties
            });

            if !required.is_empty() {
                schema_def["required"] = json!(required);
            }

            if let Some(desc) = description {
                schema_def["description"] = json!(desc);
            }

            definitions.insert(name.clone(), schema_def);
        }

        SchemaNode::Enum {
            name,
            description,
            json_name: _,
            tag_field,
            variants,
        } => {
            if definitions.contains_key(name) {
                return;
            }

            let all_unit = variants
                .iter()
                .all(|v| matches!(v, SchemaVariant::Unit { .. }));

            if all_unit {
                let enum_values: Vec<String> = variants
                    .iter()
                    .filter_map(|v| match v {
                        SchemaVariant::Unit {
                            name,
                            json_value,
                            description: _,
                        } => Some(json_value.as_ref().unwrap_or(name).clone()),
                        _ => None,
                    })
                    .collect();

                let mut schema_def = json!({
                    "type": "string",
                    "enum": enum_values
                });

                if let Some(desc) = description {
                    schema_def["description"] = json!(desc);
                }

                definitions.insert(name.clone(), schema_def);
            } else if let Some(tag) = tag_field {
                let one_of: Vec<Value> = variants
                    .iter()
                    .map(|v| variant_to_tagged_schema(v, tag, definitions))
                    .collect();

                let mut schema_def = json!({ "oneOf": one_of });

                if let Some(desc) = description {
                    schema_def["description"] = json!(desc);
                }

                definitions.insert(name.clone(), schema_def);
            } else {
                let any_of: Vec<Value> = variants
                    .iter()
                    .map(|v| variant_to_untagged_schema(v, definitions))
                    .collect();

                let mut schema_def = json!({ "anyOf": any_of });

                if let Some(desc) = description {
                    schema_def["description"] = json!(desc);
                }

                definitions.insert(name.clone(), schema_def);
            }
        }

        _ => {}
    }
}

/// Convert a SchemaNode to a JSON Schema value (for nested types).
pub fn node_to_schema(node: &SchemaNode, definitions: &mut Map<String, Value>) -> Value {
    match node {
        SchemaNode::Primitive { type_name, range } => {
            let json_type = match type_name.as_str() {
                "bool" => "boolean",
                "String" | "&str" | "char" => "string",
                "f32" | "f64" => "number",
                _ if type_name.starts_with('i') || type_name.starts_with('u') => "integer",
                _ => "string",
            };

            let mut schema = json!({ "type": json_type });

            if let Some(r) = range {
                apply_range_constraints(schema.as_object_mut().unwrap(), r);
            }

            schema
        }
        SchemaNode::Option { inner } => {
            let inner_schema = node_to_schema(inner, definitions);
            json!({ "anyOf": [inner_schema, { "type": "null" }] })
        }
        SchemaNode::Vec { item } => json!({
            "type": "array",
            "items": node_to_schema(item, definitions)
        }),
        SchemaNode::Box { inner } => node_to_schema(inner, definitions),
        SchemaNode::Opaque { type_name } => {
            json!({ "$ref": format!("#/definitions/{}", type_name) })
        }
        SchemaNode::Struct { name, .. } | SchemaNode::Enum { name, .. } => {
            collect_definitions(node, definitions);
            json!({ "$ref": format!("#/definitions/{}", name) })
        }
    }
}

fn apply_range_constraints(obj: &mut Map<String, Value>, range: &Range) {
    if let Some(min) = &range.min {
        obj.insert("minimum".to_string(), scalar_to_json(min));
    }
    if let Some(max) = &range.max {
        obj.insert("maximum".to_string(), scalar_to_json(max));
    }
}

fn scalar_to_json(scalar: &ScalarValue) -> Value {
    match scalar {
        ScalarValue::Bool(b) => json!(b),
        ScalarValue::Number(n) => {
            if let Ok(i) = n.parse::<i64>() {
                json!(i)
            } else if let Ok(f) = n.parse::<f64>() {
                json!(f)
            } else {
                json!(n)
            }
        }
        ScalarValue::String(s) => json!(s),
        ScalarValue::Char(c) => json!(c.to_string()),
    }
}

// <FILE>tui-vfx-core/src/schema/fnc_node_to_json_schema.rs</FILE> - <DESC>Convert SchemaNode to JSON Schema value</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
