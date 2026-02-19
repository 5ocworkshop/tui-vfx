// <FILE>tui-vfx-core/tests/test_fnc_to_json_schema.rs</FILE> - <DESC>Test JSON Schema generation from SchemaNode</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Schema Reference Auto-Generation - Phase 3</WCTX>
// <CLOG>Initial test suite for to_json_schema function</CLOG>

use serde_json::Value;
use tui_vfx_core::schema::{
    FieldMeta, Range, ScalarValue, SchemaField, SchemaNode, SchemaVariant, to_json_schema,
};

#[test]
fn test_simple_struct_with_primitives() {
    let schema = SchemaNode::Struct {
        name: "Person".to_string(),
        description: Some("A person record".to_string()),
        json_name: None,
        fields: vec![
            SchemaField {
                name: "name".to_string(),
                json_key: None,
                schema: Box::new(SchemaNode::Primitive {
                    type_name: "String".to_string(),
                    range: None,
                }),
                meta: FieldMeta {
                    help: None,
                    description: Some("Person's name".to_string()),
                    default: None,
                    range: None,
                    json_key: None,
                    optional: false,
                },
            },
            SchemaField {
                name: "age".to_string(),
                json_key: None,
                schema: Box::new(SchemaNode::Primitive {
                    type_name: "u32".to_string(),
                    range: Some(Range::new(
                        Some(ScalarValue::number("0")),
                        Some(ScalarValue::number("4294967295")),
                    )),
                }),
                meta: FieldMeta {
                    help: None,
                    description: Some("Person's age".to_string()),
                    default: None,
                    range: None,
                    json_key: None,
                    optional: false,
                },
            },
        ],
    };

    let json_schema = to_json_schema(&schema, "Person");

    // Verify it's valid JSON
    assert!(json_schema.is_object());

    // Verify schema version
    assert_eq!(
        json_schema["$schema"],
        "http://json-schema.org/draft-07/schema#"
    );
    assert_eq!(json_schema["title"], "Person");

    // Verify definitions exist
    let defs = &json_schema["definitions"];
    assert!(defs.is_object());
    assert!(defs["Person"].is_object());

    let person_def = &defs["Person"];
    assert_eq!(person_def["type"], "object");
    assert_eq!(person_def["description"], "A person record");

    // Verify properties
    let props = &person_def["properties"];
    assert!(props["name"].is_object());
    assert_eq!(props["name"]["type"], "string");
    assert_eq!(props["name"]["description"], "Person's name");

    assert!(props["age"].is_object());
    assert_eq!(props["age"]["type"], "integer");
    assert_eq!(props["age"]["description"], "Person's age");

    // Verify required fields
    let required = &person_def["required"];
    assert!(required.is_array());
    let required_arr = required.as_array().unwrap();
    assert!(required_arr.contains(&Value::String("name".to_string())));
    assert!(required_arr.contains(&Value::String("age".to_string())));
}

#[test]
fn test_struct_with_optional_fields() {
    let schema = SchemaNode::Struct {
        name: "Config".to_string(),
        description: None,
        json_name: None,
        fields: vec![
            SchemaField {
                name: "required_field".to_string(),
                json_key: None,
                schema: Box::new(SchemaNode::Primitive {
                    type_name: "String".to_string(),
                    range: None,
                }),
                meta: FieldMeta {
                    help: None,
                    description: None,
                    default: None,
                    range: None,
                    json_key: None,
                    optional: false,
                },
            },
            SchemaField {
                name: "optional_field".to_string(),
                json_key: None,
                schema: Box::new(SchemaNode::Option {
                    inner: Box::new(SchemaNode::Primitive {
                        type_name: "i32".to_string(),
                        range: None,
                    }),
                }),
                meta: FieldMeta {
                    help: None,
                    description: None,
                    default: None,
                    range: None,
                    json_key: None,
                    optional: true,
                },
            },
        ],
    };

    let json_schema = to_json_schema(&schema, "Config");
    let config_def = &json_schema["definitions"]["Config"];

    // Verify required only includes non-optional fields
    let required = config_def["required"].as_array().unwrap();
    assert_eq!(required.len(), 1);
    assert!(required.contains(&Value::String("required_field".to_string())));
    assert!(!required.contains(&Value::String("optional_field".to_string())));

    // Verify optional field is anyOf with null
    let opt_field = &config_def["properties"]["optional_field"];
    assert!(opt_field["anyOf"].is_array());
}

#[test]
fn test_simple_unit_enum() {
    let schema = SchemaNode::Enum {
        name: "Status".to_string(),
        description: Some("Status enum".to_string()),
        json_name: None,
        tag_field: None,
        variants: vec![
            SchemaVariant::Unit {
                name: "Active".to_string(),
                description: None,
                json_value: Some("active".to_string()),
            },
            SchemaVariant::Unit {
                name: "Inactive".to_string(),
                description: None,
                json_value: Some("inactive".to_string()),
            },
            SchemaVariant::Unit {
                name: "Pending".to_string(),
                description: None,
                json_value: None,
            },
        ],
    };

    let json_schema = to_json_schema(&schema, "Status");
    let status_def = &json_schema["definitions"]["Status"];

    // Verify it's a string enum
    assert_eq!(status_def["type"], "string");
    assert_eq!(status_def["description"], "Status enum");

    // Verify enum values
    let enum_values = status_def["enum"].as_array().unwrap();
    assert_eq!(enum_values.len(), 3);
    assert!(enum_values.contains(&Value::String("active".to_string())));
    assert!(enum_values.contains(&Value::String("inactive".to_string())));
    assert!(enum_values.contains(&Value::String("Pending".to_string())));
}

#[test]
fn test_tagged_enum_with_struct_variants() {
    let schema = SchemaNode::Enum {
        name: "Shape".to_string(),
        description: Some("Geometric shape".to_string()),
        json_name: None,
        tag_field: Some("type".to_string()),
        variants: vec![
            SchemaVariant::Unit {
                name: "Point".to_string(),
                description: Some("A point".to_string()),
                json_value: Some("point".to_string()),
            },
            SchemaVariant::Struct {
                name: "Circle".to_string(),
                description: Some("A circle".to_string()),
                json_value: Some("circle".to_string()),
                fields: vec![SchemaField {
                    name: "radius".to_string(),
                    json_key: None,
                    schema: Box::new(SchemaNode::Primitive {
                        type_name: "f64".to_string(),
                        range: None,
                    }),
                    meta: FieldMeta {
                        help: None,
                        description: Some("Circle radius".to_string()),
                        default: None,
                        range: None,
                        json_key: None,
                        optional: false,
                    },
                }],
            },
        ],
    };

    let json_schema = to_json_schema(&schema, "Shape");
    let shape_def = &json_schema["definitions"]["Shape"];

    // Verify it's a oneOf with discriminator
    assert!(shape_def["oneOf"].is_array());
    assert_eq!(shape_def["description"], "Geometric shape");

    let variants = shape_def["oneOf"].as_array().unwrap();
    assert_eq!(variants.len(), 2);

    // Check Point variant
    let point = &variants[0];
    assert_eq!(point["type"], "object");
    assert_eq!(point["description"], "A point");
    assert_eq!(point["properties"]["type"]["const"], "point");

    // Check Circle variant
    let circle = &variants[1];
    assert_eq!(circle["type"], "object");
    assert_eq!(circle["description"], "A circle");
    assert_eq!(circle["properties"]["type"]["const"], "circle");
    assert_eq!(circle["properties"]["radius"]["type"], "number");
    assert_eq!(
        circle["properties"]["radius"]["description"],
        "Circle radius"
    );
}

#[test]
fn test_nested_types() {
    let inner_struct = SchemaNode::Struct {
        name: "Address".to_string(),
        description: None,
        json_name: None,
        fields: vec![SchemaField {
            name: "city".to_string(),
            json_key: None,
            schema: Box::new(SchemaNode::Primitive {
                type_name: "String".to_string(),
                range: None,
            }),
            meta: FieldMeta {
                help: None,
                description: None,
                default: None,
                range: None,
                json_key: None,
                optional: false,
            },
        }],
    };

    let outer_struct = SchemaNode::Struct {
        name: "Company".to_string(),
        description: None,
        json_name: None,
        fields: vec![SchemaField {
            name: "address".to_string(),
            json_key: None,
            schema: Box::new(inner_struct.clone()),
            meta: FieldMeta {
                help: None,
                description: None,
                default: None,
                range: None,
                json_key: None,
                optional: false,
            },
        }],
    };

    let json_schema = to_json_schema(&outer_struct, "Company");

    // Verify both types are in definitions
    assert!(json_schema["definitions"]["Company"].is_object());
    assert!(json_schema["definitions"]["Address"].is_object());

    // Verify reference
    let company_def = &json_schema["definitions"]["Company"];
    let addr_ref = &company_def["properties"]["address"]["$ref"];
    assert_eq!(addr_ref, "#/definitions/Address");
}

#[test]
fn test_array_types() {
    let schema = SchemaNode::Struct {
        name: "List".to_string(),
        description: None,
        json_name: None,
        fields: vec![SchemaField {
            name: "items".to_string(),
            json_key: None,
            schema: Box::new(SchemaNode::Vec {
                item: Box::new(SchemaNode::Primitive {
                    type_name: "String".to_string(),
                    range: None,
                }),
            }),
            meta: FieldMeta {
                help: None,
                description: None,
                default: None,
                range: None,
                json_key: None,
                optional: false,
            },
        }],
    };

    let json_schema = to_json_schema(&schema, "List");
    let list_def = &json_schema["definitions"]["List"];

    let items_field = &list_def["properties"]["items"];
    assert_eq!(items_field["type"], "array");
    assert_eq!(items_field["items"]["type"], "string");
}

#[test]
fn test_field_with_default_value() {
    let schema = SchemaNode::Struct {
        name: "WithDefault".to_string(),
        description: None,
        json_name: None,
        fields: vec![SchemaField {
            name: "enabled".to_string(),
            json_key: None,
            schema: Box::new(SchemaNode::Primitive {
                type_name: "bool".to_string(),
                range: None,
            }),
            meta: FieldMeta {
                help: None,
                description: None,
                default: Some(ScalarValue::Bool(true)),
                range: None,
                json_key: None,
                optional: false,
            },
        }],
    };

    let json_schema = to_json_schema(&schema, "WithDefault");
    let def = &json_schema["definitions"]["WithDefault"];

    let enabled_field = &def["properties"]["enabled"];
    assert_eq!(enabled_field["type"], "boolean");
    assert_eq!(enabled_field["default"], true);
}

#[test]
fn test_field_with_range_constraints() {
    let schema = SchemaNode::Struct {
        name: "Constrained".to_string(),
        description: None,
        json_name: None,
        fields: vec![SchemaField {
            name: "value".to_string(),
            json_key: None,
            schema: Box::new(SchemaNode::Primitive {
                type_name: "i32".to_string(),
                range: None,
            }),
            meta: FieldMeta {
                help: None,
                description: None,
                default: None,
                range: Some(Range::new(
                    Some(ScalarValue::number("0")),
                    Some(ScalarValue::number("100")),
                )),
                json_key: None,
                optional: false,
            },
        }],
    };

    let json_schema = to_json_schema(&schema, "Constrained");
    let def = &json_schema["definitions"]["Constrained"];

    let value_field = &def["properties"]["value"];
    assert_eq!(value_field["type"], "integer");
    assert_eq!(value_field["minimum"], 0);
    assert_eq!(value_field["maximum"], 100);
}

#[test]
fn test_json_schema_is_parseable() {
    let schema = SchemaNode::Struct {
        name: "Test".to_string(),
        description: None,
        json_name: None,
        fields: vec![],
    };

    let json_schema = to_json_schema(&schema, "Test");

    // Convert to string and parse back
    let json_str = serde_json::to_string(&json_schema).unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();

    // Verify structure is preserved
    assert_eq!(parsed["$schema"], "http://json-schema.org/draft-07/schema#");
    assert!(parsed["definitions"].is_object());
}

// <FILE>tui-vfx-core/tests/test_fnc_to_json_schema.rs</FILE> - <DESC>Test JSON Schema generation from SchemaNode</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
