// <FILE>tui-vfx-core/src/schema/cls_schema_registry.rs</FILE> - <DESC>Schema registry for collecting types</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Initial implementation of SchemaRegistry</CLOG>

use std::collections::HashMap;
use std::sync::OnceLock;

use super::types::SchemaNode;

/// Global registry of all schema types
pub struct SchemaRegistry {
    types: HashMap<String, SchemaNode>,
}

impl SchemaRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    /// Register a schema type
    pub fn register(&mut self, type_name: &str, schema: SchemaNode) {
        self.types.insert(type_name.to_string(), schema);
    }

    /// Get schema for a specific type
    pub fn get(&self, type_name: &str) -> Option<&SchemaNode> {
        self.types.get(type_name)
    }

    /// Iterate all registered types
    pub fn iter(&self) -> impl Iterator<Item = (&String, &SchemaNode)> {
        self.types.iter()
    }

    /// Get all type names
    pub fn type_names(&self) -> impl Iterator<Item = &String> {
        self.types.keys()
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Global singleton for schema registry
static REGISTRY: OnceLock<SchemaRegistry> = OnceLock::new();

/// Get or initialize the global schema registry
pub fn global_registry() -> &'static SchemaRegistry {
    REGISTRY.get_or_init(SchemaRegistry::new)
}

// <FILE>tui-vfx-core/src/schema/cls_schema_registry.rs</FILE> - <DESC>Schema registry for collecting types</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-31T00:00:00Z</VERS>
