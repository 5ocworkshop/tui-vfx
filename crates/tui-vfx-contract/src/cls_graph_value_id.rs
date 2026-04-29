// <FILE>crates/tui-vfx-contract/src/cls_graph_value_id.rs</FILE> - <DESC>Stable graph-local value id newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: identify node-published values on the graph value bus.</WCTX>
// <CLOG>0.1.0: INIT — add transparent graph value id DTO and validation helper.</CLOG>

/// Stable value identifier published into a canonical graph-local value bus.
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
pub struct GraphValueId(
    /// Graph-local value identifier string.
    #[schemars(regex(pattern = "^[A-Za-z][A-Za-z0-9_-]*$"))]
    pub String,
);

impl GraphValueId {
    /// Build a graph value id from a string-like value.
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

// <FILE>crates/tui-vfx-contract/src/cls_graph_value_id.rs</FILE> - <DESC>Stable graph-local value id newtype</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
