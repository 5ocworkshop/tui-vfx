// <FILE>tui-vfx-core/tests/test_derive_macro_phase2.rs</FILE> - <DESC>Test Phase 2 derive macro enhancements</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 2</WCTX>
// <CLOG>Test doc comment extraction and serde attribute parsing</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_core::{ConfigSchema, SchemaNode, SchemaVariant};

/// A test struct with documentation
#[derive(ConfigSchema, Serialize, Deserialize)]
#[serde(rename = "test_config")]
struct TestConfig {
    /// The name field with doc comment
    name: String,

    /// An optional value
    value: Option<i32>,

    /// A field with serde rename
    #[serde(rename = "customField")]
    custom_field: bool,
}

/// A test enum with documentation
#[derive(ConfigSchema, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)] // Test enum intentionally uses consistent naming
enum TestEnum {
    /// First variant with doc
    FirstVariant,

    /// Second variant with data
    SecondVariant {
        /// Inner field
        data: String,
    },

    /// Third variant with tuple
    ThirdVariant(
        /// First item
        i32,
        /// Second item
        String,
    ),
}

/// A test enum with tag field
#[derive(ConfigSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
enum TaggedEnum {
    /// Option A
    OptionA { value: i32 },
    /// Option B
    OptionB { name: String },
}

#[test]
fn test_struct_description() {
    let schema = TestConfig::schema();
    if let SchemaNode::Struct {
        description,
        json_name,
        ..
    } = schema
    {
        assert_eq!(
            description,
            Some("A test struct with documentation".to_string())
        );
        assert_eq!(json_name, Some("test_config".to_string()));
    } else {
        panic!("Expected SchemaNode::Struct");
    }
}

#[test]
fn test_field_descriptions() {
    let schema = TestConfig::schema();
    if let SchemaNode::Struct { fields, .. } = schema {
        // Check name field
        let name_field = fields.iter().find(|f| f.name == "name").unwrap();
        assert_eq!(
            name_field.meta.description,
            Some("The name field with doc comment".to_string())
        );
        assert_eq!(name_field.json_key, None);

        // Check value field (optional)
        let value_field = fields.iter().find(|f| f.name == "value").unwrap();
        assert_eq!(
            value_field.meta.description,
            Some("An optional value".to_string())
        );
        assert!(value_field.meta.optional);

        // Check custom_field (renamed)
        let custom_field = fields.iter().find(|f| f.name == "custom_field").unwrap();
        assert_eq!(
            custom_field.meta.description,
            Some("A field with serde rename".to_string())
        );
        assert_eq!(custom_field.json_key, Some("customField".to_string()));
    } else {
        panic!("Expected SchemaNode::Struct");
    }
}

#[test]
fn test_enum_description() {
    let schema = TestEnum::schema();
    if let SchemaNode::Enum {
        description,
        json_name,
        tag_field,
        ..
    } = schema
    {
        assert_eq!(
            description,
            Some("A test enum with documentation".to_string())
        );
        assert_eq!(json_name, None);
        assert_eq!(tag_field, None);
    } else {
        panic!("Expected SchemaNode::Enum");
    }
}

#[test]
fn test_enum_with_tag_field() {
    let schema = TaggedEnum::schema();
    if let SchemaNode::Enum { tag_field, .. } = schema {
        assert_eq!(tag_field, Some("type".to_string()));
    } else {
        panic!("Expected SchemaNode::Enum");
    }
}

#[test]
fn test_variant_descriptions_and_json_values() {
    let schema = TestEnum::schema();
    if let SchemaNode::Enum { variants, .. } = schema {
        // Check FirstVariant
        let first = variants.iter().find(|v| match v {
            SchemaVariant::Unit { name, .. } => name == "FirstVariant",
            _ => false,
        });
        if let Some(SchemaVariant::Unit {
            description,
            json_value,
            ..
        }) = first
        {
            assert_eq!(*description, Some("First variant with doc".to_string()));
            assert_eq!(*json_value, Some("first_variant".to_string()));
        } else {
            panic!("Expected Unit variant FirstVariant");
        }

        // Check SecondVariant
        let second = variants.iter().find(|v| match v {
            SchemaVariant::Struct { name, .. } => name == "SecondVariant",
            _ => false,
        });
        if let Some(SchemaVariant::Struct {
            description,
            json_value,
            fields,
            ..
        }) = second
        {
            assert_eq!(*description, Some("Second variant with data".to_string()));
            assert_eq!(*json_value, Some("second_variant".to_string()));

            let data_field = fields.iter().find(|f| f.name == "data").unwrap();
            assert_eq!(data_field.meta.description, Some("Inner field".to_string()));
            // json_key is None when it matches the field name
            assert_eq!(data_field.json_key, None);
        } else {
            panic!("Expected Struct variant SecondVariant");
        }

        // Check ThirdVariant
        let third = variants.iter().find(|v| match v {
            SchemaVariant::Tuple { name, .. } => name == "ThirdVariant",
            _ => false,
        });
        if let Some(SchemaVariant::Tuple {
            description,
            json_value,
            items,
            ..
        }) = third
        {
            assert_eq!(*description, Some("Third variant with tuple".to_string()));
            assert_eq!(*json_value, Some("third_variant".to_string()));

            assert_eq!(items.len(), 2);
            assert_eq!(items[0].meta.description, Some("First item".to_string()));
            assert_eq!(items[1].meta.description, Some("Second item".to_string()));
        } else {
            panic!("Expected Tuple variant ThirdVariant");
        }
    } else {
        panic!("Expected SchemaNode::Enum");
    }
}

// <FILE>tui-vfx-core/tests/test_derive_macro_phase2.rs</FILE> - <DESC>Test Phase 2 derive macro enhancements</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-31T00:00:00Z</VERS>
