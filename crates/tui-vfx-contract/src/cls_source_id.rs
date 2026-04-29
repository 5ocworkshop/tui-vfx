// <FILE>crates/tui-vfx-contract/src/cls_source_id.rs</FILE> - <DESC>Stable surface source descriptor id newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: identify stable source descriptors.</WCTX>
// <CLOG>0.1.0: INIT — add dotted source id DTO and validation helper.</CLOG>

/// Stable canonical source identifier for a surface-producing descriptor.
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
pub struct SourceId(
    /// Canonical source id, commonly namespaced such as `source.text`.
    #[schemars(regex(pattern = "^[A-Za-z][A-Za-z0-9_-]*(\\.[A-Za-z][A-Za-z0-9_-]*)*$"))]
    pub String,
);

impl SourceId {
    /// Build a source id from a string-like value.
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

pub(crate) fn is_identifier_segment(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    first.is_ascii_alphabetic()
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

// <FILE>crates/tui-vfx-contract/src/cls_source_id.rs</FILE> - <DESC>Stable surface source descriptor id newtype</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
