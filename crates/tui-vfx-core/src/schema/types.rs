// <FILE>tui-vfx-core/src/schema/types.rs</FILE> - <DESC>Schema types for configuration introspection</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Add documentation and JSON mapping fields to schema types</CLOG>

use std::fmt;

use super::fnc_schema_node_to_json_pretty::schema_node_to_json_pretty;

pub trait ConfigSchema {
    fn schema() -> SchemaNode;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FieldMeta {
    pub help: Option<String>,
    pub description: Option<String>,
    pub default: Option<ScalarValue>,
    pub range: Option<Range>,
    pub json_key: Option<String>,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    pub min: Option<ScalarValue>,
    pub max: Option<ScalarValue>,
}

impl Range {
    pub fn new(min: Option<ScalarValue>, max: Option<ScalarValue>) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarValue {
    Bool(bool),
    Number(String),
    String(String),
    Char(char),
}

impl ScalarValue {
    pub fn number<T: Into<String>>(value: T) -> Self {
        Self::Number(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaField {
    pub name: String,
    pub json_key: Option<String>,
    pub schema: Box<SchemaNode>,
    pub meta: FieldMeta,
}

impl SchemaField {
    pub fn new(name: impl Into<String>, schema: SchemaNode, meta: FieldMeta) -> Self {
        Self {
            name: name.into(),
            json_key: None,
            schema: Box::new(schema),
            meta,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaVariant {
    Unit {
        name: String,
        description: Option<String>,
        json_value: Option<String>,
    },
    Tuple {
        name: String,
        description: Option<String>,
        json_value: Option<String>,
        items: Vec<SchemaField>,
    },
    Struct {
        name: String,
        description: Option<String>,
        json_value: Option<String>,
        fields: Vec<SchemaField>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaNode {
    Struct {
        name: String,
        description: Option<String>,
        json_name: Option<String>,
        fields: Vec<SchemaField>,
    },
    Enum {
        name: String,
        description: Option<String>,
        json_name: Option<String>,
        tag_field: Option<String>,
        variants: Vec<SchemaVariant>,
    },
    Primitive {
        type_name: String,
        range: Option<Range>,
    },
    Option {
        inner: Box<SchemaNode>,
    },
    Vec {
        item: Box<SchemaNode>,
    },
    Box {
        inner: Box<SchemaNode>,
    },
    Opaque {
        type_name: String,
    },
}

impl SchemaNode {
    pub fn to_json_pretty(&self) -> String {
        schema_node_to_json_pretty(self)
    }
}

impl fmt::Display for SchemaNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_json_pretty())
    }
}

// <FILE>tui-vfx-core/src/schema/types.rs</FILE> - <DESC>Schema types for configuration introspection</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
