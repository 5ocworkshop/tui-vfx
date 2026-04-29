// <FILE>crates/tui-vfx-contract/src/cls_descriptor_pack_id.rs</FILE> - <DESC>Stable descriptor pack id newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: identify shared descriptor packs for canonical recipe validation.</WCTX>
// <CLOG>0.1.0: INIT — add dotted descriptor pack id DTO and validation helper.</CLOG>

/// Stable canonical descriptor pack identifier.
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
pub struct DescriptorPackId(
    /// Canonical descriptor pack id, commonly namespaced such as `v3.1.primitive`.
    #[schemars(regex(pattern = "^[A-Za-z][A-Za-z0-9_-]*(\\.[A-Za-z0-9][A-Za-z0-9_-]*)*$"))]
    pub String,
);

impl DescriptorPackId {
    /// Build a descriptor pack id from a string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return true when the id is made of non-empty dotted ASCII id segments.
    pub fn is_valid(&self) -> bool {
        let mut segments = self.0.split('.');
        let Some(first) = segments.next() else {
            return false;
        };
        is_alpha_segment(first) && segments.all(is_alphanumeric_segment)
    }
}

fn is_alpha_segment(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    first.is_ascii_alphabetic()
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

fn is_alphanumeric_segment(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    first.is_ascii_alphanumeric()
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

// <FILE>crates/tui-vfx-contract/src/cls_descriptor_pack_id.rs</FILE> - <DESC>Stable descriptor pack id newtype</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
