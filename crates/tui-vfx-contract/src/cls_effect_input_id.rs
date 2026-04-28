// <FILE>crates/tui-vfx-contract/src/cls_effect_input_id.rs</FILE> - <DESC>Stable effect input id newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F1: identify effect descriptor inputs by stable map keys.</WCTX>
// <CLOG>0.1.0: INIT — add transparent input id DTO and local validation helper.</CLOG>

/// Stable descriptor-local effect input identifier.
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
pub struct EffectInputId(
    /// Descriptor-local input identifier string.
    pub String,
);

impl EffectInputId {
    /// Build an input id from a string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return true when the id is non-empty and ASCII identifier-like.
    pub fn is_valid(&self) -> bool {
        let mut chars = self.0.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        first.is_ascii_alphabetic()
            && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_effect_input_id.rs</FILE> - <DESC>Stable effect input id newtype</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
