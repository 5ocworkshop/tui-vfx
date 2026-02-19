// <FILE>tui-vfx-core/tests/test_fnc_to_markdown.rs</FILE> - <DESC>Tests for Markdown documentation generator</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Schema Reference Auto-Generation</WCTX>
// <CLOG>Initial test suite for to_markdown function</CLOG>

use tui_vfx_core::schema::{FieldMeta, ScalarValue, SchemaField, SchemaNode, SchemaVariant};

#[test]
fn test_simple_struct_generates_header_and_fields_table() {
    let schema = SchemaNode::Struct {
        name: "Config".to_string(),
        description: Some("Application configuration".to_string()),
        json_name: None,
        fields: vec![
            SchemaField::new(
                "width",
                SchemaNode::Primitive {
                    type_name: "u32".to_string(),
                    range: None,
                },
                FieldMeta {
                    description: Some("Window width".to_string()),
                    optional: false,
                    default: Some(ScalarValue::number("800")),
                    ..Default::default()
                },
            ),
            SchemaField::new(
                "title",
                SchemaNode::Primitive {
                    type_name: "String".to_string(),
                    range: None,
                },
                FieldMeta {
                    description: Some("Window title".to_string()),
                    optional: true,
                    ..Default::default()
                },
            ),
        ],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&schema, "Test Config");

    assert!(markdown.contains("# Test Config"));
    assert!(markdown.contains("## Config"));
    assert!(markdown.contains("Application configuration"));
    assert!(markdown.contains("### Fields"));
    assert!(markdown.contains("| Field | Type | Required | Default | Description |"));
    assert!(markdown.contains("| `width` | `u32` | Yes | `800` | Window width |"));
    assert!(markdown.contains("| `title` | `String` | No | - | Window title |"));
}

#[test]
fn test_enum_with_unit_variants_generates_values_table() {
    let schema = SchemaNode::Enum {
        name: "Color".to_string(),
        description: Some("Color options".to_string()),
        json_name: None,
        tag_field: None,
        variants: vec![
            SchemaVariant::Unit {
                name: "Red".to_string(),
                description: Some("Red color".to_string()),
                json_value: Some("red".to_string()),
            },
            SchemaVariant::Unit {
                name: "Blue".to_string(),
                description: Some("Blue color".to_string()),
                json_value: Some("blue".to_string()),
            },
        ],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&schema, "Colors");

    assert!(markdown.contains("## Color"));
    assert!(markdown.contains("Color options"));
    assert!(markdown.contains("### Values"));
    assert!(markdown.contains("| Value | Description |"));
    assert!(markdown.contains("| `red` | Red color |"));
    assert!(markdown.contains("| `blue` | Blue color |"));
}

#[test]
fn test_tagged_enum_generates_variants_with_discriminator() {
    let schema = SchemaNode::Enum {
        name: "Shape".to_string(),
        description: Some("Geometric shapes".to_string()),
        json_name: None,
        tag_field: Some("type".to_string()),
        variants: vec![
            SchemaVariant::Unit {
                name: "Circle".to_string(),
                description: Some("A circle".to_string()),
                json_value: Some("circle".to_string()),
            },
            SchemaVariant::Struct {
                name: "Rectangle".to_string(),
                description: Some("A rectangle".to_string()),
                json_value: Some("rectangle".to_string()),
                fields: vec![SchemaField::new(
                    "width",
                    SchemaNode::Primitive {
                        type_name: "f64".to_string(),
                        range: None,
                    },
                    FieldMeta {
                        description: Some("Width of rectangle".to_string()),
                        optional: false,
                        ..Default::default()
                    },
                )],
            },
        ],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&schema, "Shapes");

    assert!(markdown.contains("## Shape"));
    assert!(markdown.contains("**Discriminator field:** `type`"));
    assert!(markdown.contains("### Variants"));
    assert!(markdown.contains("#### `circle`"));
    assert!(markdown.contains("A circle"));
    assert!(markdown.contains("#### `rectangle`"));
    assert!(markdown.contains("A rectangle"));
    assert!(markdown.contains("| Field | Type | Required | Description |"));
    assert!(markdown.contains("| `width` | `f64` | Yes | Width of rectangle |"));
}

#[test]
fn test_nested_types_generate_cross_reference_links() {
    let inner_struct = SchemaNode::Struct {
        name: "Point".to_string(),
        description: None,
        json_name: None,
        fields: vec![
            SchemaField::new(
                "x",
                SchemaNode::Primitive {
                    type_name: "f64".to_string(),
                    range: None,
                },
                FieldMeta::default(),
            ),
            SchemaField::new(
                "y",
                SchemaNode::Primitive {
                    type_name: "f64".to_string(),
                    range: None,
                },
                FieldMeta::default(),
            ),
        ],
    };

    let outer_struct = SchemaNode::Struct {
        name: "Line".to_string(),
        description: None,
        json_name: None,
        fields: vec![
            SchemaField::new("start", inner_struct.clone(), FieldMeta::default()),
            SchemaField::new("end", inner_struct, FieldMeta::default()),
        ],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&outer_struct, "Geometry");

    // Check that Line references Point with anchor link
    assert!(markdown.contains("[`Point`](#point)"));

    // Check that both types are documented
    assert!(markdown.contains("## Line"));
    assert!(markdown.contains("## Point"));

    // Check table of contents
    assert!(markdown.contains("## Table of Contents"));
    assert!(markdown.contains("- [Line](#line)"));
    assert!(markdown.contains("- [Point](#point)"));
}

#[test]
fn test_optional_fields_show_as_no_in_required_column() {
    let schema = SchemaNode::Struct {
        name: "Settings".to_string(),
        description: None,
        json_name: None,
        fields: vec![
            SchemaField::new(
                "required_field",
                SchemaNode::Primitive {
                    type_name: "bool".to_string(),
                    range: None,
                },
                FieldMeta {
                    optional: false,
                    ..Default::default()
                },
            ),
            SchemaField::new(
                "optional_field",
                SchemaNode::Primitive {
                    type_name: "bool".to_string(),
                    range: None,
                },
                FieldMeta {
                    optional: true,
                    ..Default::default()
                },
            ),
        ],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&schema, "Test");

    assert!(markdown.contains("| `required_field` | `bool` | Yes |"));
    assert!(markdown.contains("| `optional_field` | `bool` | No |"));
}

#[test]
fn test_default_values_are_displayed() {
    let schema = SchemaNode::Struct {
        name: "Config".to_string(),
        description: None,
        json_name: None,
        fields: vec![
            SchemaField::new(
                "flag",
                SchemaNode::Primitive {
                    type_name: "bool".to_string(),
                    range: None,
                },
                FieldMeta {
                    default: Some(ScalarValue::Bool(true)),
                    ..Default::default()
                },
            ),
            SchemaField::new(
                "count",
                SchemaNode::Primitive {
                    type_name: "i32".to_string(),
                    range: None,
                },
                FieldMeta {
                    default: Some(ScalarValue::number("42")),
                    ..Default::default()
                },
            ),
            SchemaField::new(
                "name",
                SchemaNode::Primitive {
                    type_name: "String".to_string(),
                    range: None,
                },
                FieldMeta {
                    default: Some(ScalarValue::String("default".to_string())),
                    ..Default::default()
                },
            ),
            SchemaField::new(
                "char",
                SchemaNode::Primitive {
                    type_name: "char".to_string(),
                    range: None,
                },
                FieldMeta {
                    default: Some(ScalarValue::Char('x')),
                    ..Default::default()
                },
            ),
        ],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&schema, "Test");

    assert!(markdown.contains("| `flag` | `bool` | Yes | `true` |"));
    assert!(markdown.contains("| `count` | `i32` | Yes | `42` |"));
    assert!(markdown.contains("| `name` | `String` | Yes | `\"default\"` |"));
    assert!(markdown.contains("| `char` | `char` | Yes | `'x'` |"));
}

#[test]
fn test_table_of_contents_includes_all_types() {
    let point = SchemaNode::Struct {
        name: "Point".to_string(),
        description: None,
        json_name: None,
        fields: vec![],
    };

    let color = SchemaNode::Enum {
        name: "Color".to_string(),
        description: None,
        json_name: None,
        tag_field: None,
        variants: vec![SchemaVariant::Unit {
            name: "Red".to_string(),
            description: None,
            json_value: None,
        }],
    };

    let shape = SchemaNode::Struct {
        name: "Shape".to_string(),
        description: None,
        json_name: None,
        fields: vec![
            SchemaField::new("position", point, FieldMeta::default()),
            SchemaField::new("color", color, FieldMeta::default()),
        ],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&shape, "Test");

    assert!(markdown.contains("## Table of Contents"));
    assert!(markdown.contains("- [Shape](#shape)"));
    assert!(markdown.contains("- [Point](#point)"));
    assert!(markdown.contains("- [Color](#color)"));
}

#[test]
fn test_option_and_vec_types_format_correctly() {
    let schema = SchemaNode::Struct {
        name: "Container".to_string(),
        description: None,
        json_name: None,
        fields: vec![
            SchemaField::new(
                "optional_value",
                SchemaNode::Option {
                    inner: Box::new(SchemaNode::Primitive {
                        type_name: "i32".to_string(),
                        range: None,
                    }),
                },
                FieldMeta::default(),
            ),
            SchemaField::new(
                "list",
                SchemaNode::Vec {
                    item: Box::new(SchemaNode::Primitive {
                        type_name: "String".to_string(),
                        range: None,
                    }),
                },
                FieldMeta::default(),
            ),
        ],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&schema, "Test");

    assert!(markdown.contains("| `optional_value` | `i32`? |"));
    assert!(markdown.contains("| `list` | [`String`] |"));
}

#[test]
fn test_tuple_variant_displays_tuple_fields() {
    let schema = SchemaNode::Enum {
        name: "Value".to_string(),
        description: None,
        json_name: None,
        tag_field: Some("kind".to_string()),
        variants: vec![SchemaVariant::Tuple {
            name: "Pair".to_string(),
            description: Some("A pair of values".to_string()),
            json_value: Some("pair".to_string()),
            items: vec![
                SchemaField::new(
                    "0",
                    SchemaNode::Primitive {
                        type_name: "i32".to_string(),
                        range: None,
                    },
                    FieldMeta::default(),
                ),
                SchemaField::new(
                    "1",
                    SchemaNode::Primitive {
                        type_name: "String".to_string(),
                        range: None,
                    },
                    FieldMeta::default(),
                ),
            ],
        }],
    };

    let markdown = tui_vfx_core::schema::to_markdown(&schema, "Test");

    assert!(markdown.contains("#### `pair`"));
    assert!(markdown.contains("A pair of values"));
    assert!(markdown.contains("**Tuple fields:**"));
    assert!(markdown.contains("0. `i32`"));
    assert!(markdown.contains("1. `String`"));
}

// <FILE>tui-vfx-core/tests/test_fnc_to_markdown.rs</FILE> - <DESC>Tests for Markdown documentation generator</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
