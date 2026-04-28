// <FILE>crates/tui-vfx-next/src/cls_element_id.rs</FILE> - <DESC>Stable scene element identifier</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase D1: distinguish element identity from semantic roles in scene composition.</WCTX>
// <CLOG>0.1.0: ADD — introduce schema-visible element ids for scene diagnostics and future recipe references.</CLOG>

/// Stable scene element identity used by diagnostics and future recipe references.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(transparent)]
pub struct ElementId(String);

impl ElementId {
    /// Create an element id from a stable string label.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Borrow the element id as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// <FILE>crates/tui-vfx-next/src/cls_element_id.rs</FILE> - <DESC>Stable scene element identifier</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
