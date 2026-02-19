// <FILE>tui-vfx-core/tests/test_cls_schema_registry.rs</FILE> - <DESC>Tests for SchemaRegistry</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Initial test implementation for SchemaRegistry</CLOG>

use tui_vfx_core::schema::{SchemaNode, SchemaRegistry};

#[test]
fn test_registry_new() {
    let registry = SchemaRegistry::new();
    assert_eq!(registry.type_names().count(), 0);
}

#[test]
fn test_registry_default() {
    let registry = SchemaRegistry::default();
    assert_eq!(registry.type_names().count(), 0);
}

#[test]
fn test_registry_register_and_get() {
    let mut registry = SchemaRegistry::new();

    let schema = SchemaNode::Primitive {
        type_name: "i32".to_string(),
        range: None,
    };

    registry.register("TestType", schema.clone());

    let retrieved = registry.get("TestType");
    assert!(retrieved.is_some());
    assert_eq!(*retrieved.unwrap(), schema);
}

#[test]
fn test_registry_get_nonexistent() {
    let registry = SchemaRegistry::new();
    assert!(registry.get("NonExistent").is_none());
}

#[test]
fn test_registry_iter() {
    let mut registry = SchemaRegistry::new();

    let schema1 = SchemaNode::Primitive {
        type_name: "i32".to_string(),
        range: None,
    };

    let schema2 = SchemaNode::Primitive {
        type_name: "String".to_string(),
        range: None,
    };

    registry.register("Type1", schema1);
    registry.register("Type2", schema2);

    let count = registry.iter().count();
    assert_eq!(count, 2);
}

#[test]
fn test_registry_type_names() {
    let mut registry = SchemaRegistry::new();

    let schema = SchemaNode::Primitive {
        type_name: "i32".to_string(),
        range: None,
    };

    registry.register("TypeA", schema.clone());
    registry.register("TypeB", schema.clone());
    registry.register("TypeC", schema);

    let mut names: Vec<_> = registry.type_names().cloned().collect();
    names.sort();

    assert_eq!(names, vec!["TypeA", "TypeB", "TypeC"]);
}

#[test]
fn test_registry_overwrite() {
    let mut registry = SchemaRegistry::new();

    let schema1 = SchemaNode::Primitive {
        type_name: "i32".to_string(),
        range: None,
    };

    let schema2 = SchemaNode::Primitive {
        type_name: "String".to_string(),
        range: None,
    };

    registry.register("Test", schema1);
    registry.register("Test", schema2.clone());

    let retrieved = registry.get("Test");
    assert!(retrieved.is_some());
    assert_eq!(*retrieved.unwrap(), schema2);
}

#[test]
fn test_schema_node_with_new_fields() {
    let schema = SchemaNode::Struct {
        name: "TestStruct".to_string(),
        description: Some("A test structure".to_string()),
        json_name: Some("test_struct".to_string()),
        fields: vec![],
    };

    match schema {
        SchemaNode::Struct {
            name,
            description,
            json_name,
            ..
        } => {
            assert_eq!(name, "TestStruct");
            assert_eq!(description, Some("A test structure".to_string()));
            assert_eq!(json_name, Some("test_struct".to_string()));
        }
        _ => panic!("Expected Struct variant"),
    }
}

#[test]
fn test_schema_enum_with_new_fields() {
    let schema = SchemaNode::Enum {
        name: "TestEnum".to_string(),
        description: Some("A test enum".to_string()),
        json_name: Some("test_enum".to_string()),
        tag_field: Some("type".to_string()),
        variants: vec![],
    };

    match schema {
        SchemaNode::Enum {
            name,
            description,
            json_name,
            tag_field,
            ..
        } => {
            assert_eq!(name, "TestEnum");
            assert_eq!(description, Some("A test enum".to_string()));
            assert_eq!(json_name, Some("test_enum".to_string()));
            assert_eq!(tag_field, Some("type".to_string()));
        }
        _ => panic!("Expected Enum variant"),
    }
}

// <FILE>tui-vfx-core/tests/test_cls_schema_registry.rs</FILE> - <DESC>Tests for SchemaRegistry</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-31T00:00:00Z</VERS>
