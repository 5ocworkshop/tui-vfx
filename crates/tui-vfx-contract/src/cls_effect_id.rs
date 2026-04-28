// <FILE>crates/tui-vfx-contract/src/cls_effect_id.rs</FILE> - <DESC>Stable effect descriptor id newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: add stable descriptor identity vocabulary.</WCTX>
// <CLOG>0.1.0: INIT — add transparent effect id DTO for descriptor schemas.</CLOG>

/// Stable canonical effect identifier.
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
pub struct EffectId(
    /// Canonical effect identifier string.
    pub String,
);

impl EffectId {
    /// Build an effect id from a string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_effect_id.rs</FILE> - <DESC>Stable effect descriptor id newtype</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
