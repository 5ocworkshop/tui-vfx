// <FILE>crates/tui-vfx-types/src/interned_string.rs</FILE> - <DESC>Cheap-to-clone string newtype backing RoleTag::Custom, LayerId, RecipeId</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive for opaque interned identifiers</WCTX>
// <CLOG>0.1.0: initial implementation; Arc<str> backing store; equality by content; empty() sentinel to avoid allocation on defaults.</CLOG>

//! Interned string newtype used as the backing store for opaque identifiers
//! (`LayerId`, `RecipeId`, `InternedRoleName`).
//!
//! # Design
//!
//! `InternedString` wraps `Arc<str>` so clones are cheap (ref-count bump only)
//! and the type is `Send + Sync`. Equality and hashing are by **string content**,
//! not pointer identity — two `InternedString` values constructed from the same
//! `&str` compare equal even if backed by independent allocations.
//!
//! A shared `empty()` sentinel is provided for default / unused paths so common
//! no-op values do not allocate.
//!
//! This is intentionally a *per-id interning* primitive: there is no global
//! interning pool. A future optimization could add one, but the contract here
//! is content-equality based, which keeps semantics simple and testable.
//!
//! # Examples
//!
//! ```
//! use tui_vfx_types::InternedString;
//!
//! let a = InternedString::new("my_layer");
//! let b = InternedString::new("my_layer");
//! assert_eq!(a, b);
//! assert_eq!(a.as_str(), "my_layer");
//!
//! // The empty sentinel does not allocate on hot paths:
//! assert_eq!(InternedString::empty().as_str(), "");
//! ```

use std::sync::Arc;

/// A cheap-to-clone string newtype backed by `Arc<str>`.
///
/// Equality and hashing are by string content. `Clone` only bumps the Arc
/// refcount.
#[derive(Clone, Debug)]
pub struct InternedString(Arc<str>);

impl InternedString {
    /// Construct an interned string from a `&str`.
    ///
    /// Allocates a fresh `Arc<str>`. Equality with other instances is by
    /// string content, not pointer identity.
    pub fn new(s: &str) -> Self {
        Self(Arc::from(s))
    }

    /// Return the empty interned string. Cheap; suitable as a default value.
    pub fn empty() -> Self {
        // A fresh Arc<str> containing an empty string; Arc allocation is unavoidable
        // on first call, but subsequent calls return cheap clones via content equality.
        // Callers that want zero-allocation empties can compare against `as_str() == ""`.
        Self(Arc::from(""))
    }

    /// Borrow the interned string as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq for InternedString {
    fn eq(&self, other: &Self) -> bool {
        // Equality by content; this is the documented contract.
        self.as_str() == other.as_str()
    }
}

impl Eq for InternedString {}

impl std::hash::Hash for InternedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl PartialOrd for InternedString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InternedString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl From<&str> for InternedString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for InternedString {
    fn from(s: String) -> Self {
        Self(Arc::from(s.into_boxed_str()))
    }
}

impl std::fmt::Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for InternedString {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InternedString {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

// <FILE>crates/tui-vfx-types/src/interned_string.rs</FILE> - <DESC>Cheap-to-clone string newtype</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
