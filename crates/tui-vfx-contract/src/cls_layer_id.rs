// <FILE>crates/tui-vfx-contract/src/cls_layer_id.rs</FILE> - <DESC>Optional scene layer identifier</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase D1: provide lightweight layer grouping without building a full layer graph.</WCTX>
// <CLOG>0.1.0: ADD — introduce schema-visible layer ids as optional scene element metadata.</CLOG>

/// Optional scene layer identity used for grouping elements without a full layer graph.
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
pub struct LayerId(String);

impl LayerId {
    /// Create a layer id from a stable string label.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Borrow the layer id as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_layer_id.rs</FILE> - <DESC>Optional scene layer identifier</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
