// <FILE>crates/tui-vfx-contract/src/cls_scene_id.rs</FILE> - <DESC>Stable canonical recipe scene id</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H1: identify canonical recipe scene roots.</WCTX>
// <CLOG>0.1.0: INIT — add scene id DTO and validation helper.</CLOG>

use crate::cls_source_id::is_identifier_segment;

/// Stable canonical recipe scene identifier.
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
pub struct SceneId(
    /// Canonical scene identifier string.
    #[schemars(regex(pattern = "^[A-Za-z][A-Za-z0-9_-]*$"))]
    pub String,
);

impl SceneId {
    /// Build a scene id from a string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return true when the id is a non-empty ASCII identifier segment.
    pub fn is_valid(&self) -> bool {
        is_identifier_segment(&self.0)
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_scene_id.rs</FILE> - <DESC>Stable canonical recipe scene id</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
