// <FILE>crates/tui-vfx-contract/src/cls_structured_value.rs</FILE> - <DESC>Descriptor-owned structured JSON-compatible value</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Vertical recipe parity: carry primitive-specific structured payloads without losing data.</WCTX>
// <CLOG>0.1.0: INIT — add recursive structured value payload for descriptor-owned fields.</CLOG>

use std::collections::BTreeMap;

/// JSON-compatible structured payload owned by a descriptor field.
///
/// Use this only when the primitive owns a nested payload whose stable shape is
/// descriptor-specific, such as path lists, particle options, or glyph-style
/// rule tables. Prefer narrower typed values whenever a field has scalar,
/// color, gradient, duration, scope, or rect semantics.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum StructuredValue {
    /// Explicit null.
    Null,
    /// Boolean leaf value.
    Boolean(bool),
    /// Floating-point numeric leaf value.
    Number(f64),
    /// String leaf value.
    String(String),
    /// Ordered child values.
    Array(Vec<StructuredValue>),
    /// String-keyed object values.
    Object(BTreeMap<String, StructuredValue>),
}

// <FILE>crates/tui-vfx-contract/src/cls_structured_value.rs</FILE> - <DESC>Descriptor-owned structured JSON-compatible value</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
