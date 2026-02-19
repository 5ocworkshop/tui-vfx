// Test to verify manual schema construction works

use tui_vfx_core::schema::{FieldMeta, SchemaNode, SchemaVariant};

#[test]
fn test_manual_construction() {
    let _ = SchemaNode::Enum {
        name: "Test".to_string(),
        description: None,
        json_name: None,
        tag_field: None,
        variants: vec![SchemaVariant::Unit {
            name: "UnitVariant".to_string(),
            description: None,
            json_value: None,
        }],
    };

    let _ = FieldMeta {
        help: None,
        description: None,
        default: None,
        range: None,
        json_key: None,
        optional: false,
    };
}
