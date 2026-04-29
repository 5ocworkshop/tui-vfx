// <FILE>crates/tui-vfx-contract/src/cls_source_input_id.rs</FILE> - <DESC>Stable source input id newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: identify descriptor-local source inputs.</WCTX>
// <CLOG>0.1.0: INIT — add dotted source input id DTO and validation helper.</CLOG>

use crate::cls_source_id::is_identifier_segment;

/// Stable descriptor-local source input identifier.
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
pub struct SourceInputId(
    /// Source input id, allowing dotted paths such as `wave.speed`.
    #[schemars(regex(pattern = "^[A-Za-z][A-Za-z0-9_-]*(\\.[A-Za-z][A-Za-z0-9_-]*)*$"))]
    pub String,
);

impl SourceInputId {
    /// Build a source input id from a string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return true when the id is made of non-empty dotted ASCII id segments.
    pub fn is_valid(&self) -> bool {
        self.0.split('.').all(is_identifier_segment)
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_source_input_id.rs</FILE> - <DESC>Stable source input id newtype</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
