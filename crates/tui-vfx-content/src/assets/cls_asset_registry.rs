// <FILE>crates/tui-vfx-content/src/assets/cls_asset_registry.rs</FILE> - <DESC>Name-to-bytes asset registry with default-fallback resolution; parallel to FontRegistry for non-font asset kinds (rocketsplash images, future audio cues, etc.)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 7 (breadcrumb) of mechanical circular content cycles plan: pluggable name → bytes mapping consumed by future scene-layer source variants that load rocketsplash .rss images by name. Sentinel-routing semantics match FontRegistry for ergonomic consistency.</WCTX>
// <CLOG>0.1.0: introduce AssetRegistry with insert / resolve / default / set_default; recognize the default_logo sentinel literal as the canonical alias for the registered default. Inline tests for default registration, named lookup, sentinel resolution, missing-name fallback, case-sensitivity contract, set_default rules.</CLOG>

//! Name-to-asset-bytes registry consumed by future scene-layer source
//! variants that load rocketsplash `.rss` images (and other byte-source
//! assets) by name.
//!
//! Phase 7 of the mechanical circular content cycles plan keeps the
//! consuming source surface deferred — adding a `type: "rocketsplash_
//! image"` scene-layer source variant intersects with sibling's V3
//! scene-layer composition work and warrants its own coordinated
//! session. This module is the byte-supplying half: a host populates
//! the registry with named asset bytes, and (future) recipe consumers
//! resolve names to bytes through it.
//!
//! The registry shape mirrors `FontRegistry`: name → bytes map plus a
//! registered default, with the reserved sentinel literal `default_logo`
//! routing to whatever asset is currently the default. Snake_case ASCII
//! per the cycle-plan v0.5.0 casing rule.

use std::collections::BTreeMap;

/// The reserved sentinel literal recipes use to mean "the host's
/// registered default asset" without naming a specific asset. Snake_
/// case ASCII per Intention 37 / cycle plan v0.5.0 — case-sensitive
/// lookups make typos silent failures.
pub const DEFAULT_LOGO_SENTINEL: &str = "default_logo";

/// Pluggable name → asset-bytes registry.
///
/// Constructed empty. Hosts call `insert` to register named assets and
/// `set_default` to bind the default-asset sentinel routing. Until a
/// default is registered, the `default_logo` sentinel resolves to
/// `None` and consumers handle the miss per their own fallback rules
/// (typically: skip the layer, render an empty rect, log a warning).
#[derive(Debug, Clone, Default)]
pub struct AssetRegistry {
    assets: BTreeMap<String, Vec<u8>>,
    default_name: Option<String>,
}

impl AssetRegistry {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert (or replace) named asset bytes. Returns the previous
    /// binding for the name if any. The first asset inserted does not
    /// automatically become the default; callers set the default
    /// explicitly via `set_default` so their intent is clear.
    pub fn insert(&mut self, name: impl Into<String>, bytes: Vec<u8>) -> Option<Vec<u8>> {
        self.assets.insert(name.into(), bytes)
    }

    /// Set the default asset's name. Returns true if the name resolves
    /// to a registered asset (in which case the default is updated);
    /// returns false and leaves the default unchanged otherwise. Refuse
    /// to set the default to the sentinel literal `default_logo` itself
    /// — that would make resolution recursive.
    pub fn set_default(&mut self, name: impl Into<String>) -> bool {
        let name = name.into();
        if name == DEFAULT_LOGO_SENTINEL {
            return false;
        }
        if !self.assets.contains_key(&name) {
            return false;
        }
        self.default_name = Some(name);
        true
    }

    /// The current default asset's registered name, or `None` if no
    /// default has been set.
    pub fn default_name(&self) -> Option<&str> {
        self.default_name.as_deref()
    }

    /// The current default asset's bytes, or `None` if no default has
    /// been set.
    pub fn default_bytes(&self) -> Option<&[u8]> {
        let name = self.default_name.as_deref()?;
        self.assets.get(name).map(|v| v.as_slice())
    }

    /// Resolve a name to asset bytes.
    ///
    /// Lookup precedence:
    /// 1. The reserved sentinel `default_logo` short-circuits to the
    ///    current default bytes regardless of the default's actual
    ///    name. Returns `None` if no default has been set.
    /// 2. Otherwise, looks up `name` in the registered assets.
    /// 3. Returns `None` if unrecognized — callers fall back per their
    ///    own policy or surface the miss to the validator.
    pub fn resolve(&self, name: &str) -> Option<&[u8]> {
        if name == DEFAULT_LOGO_SENTINEL {
            return self.default_bytes();
        }
        self.assets.get(name).map(|v| v.as_slice())
    }

    /// Resolve a name to asset bytes with implicit fallback to the
    /// default. Returns `None` only when both the named asset and the
    /// default are absent. Mirrors `FontRegistry::resolve_or_default`
    /// for ergonomic consistency, but distinct in semantics: missing
    /// fonts can degrade silently to the canonical Line 3x3 (Intention
    /// 36 — there's always a font); missing assets can legitimately
    /// have no fallback (host hasn't registered any), so this method
    /// is `Option`-typed rather than infallible.
    pub fn resolve_or_default(&self, name: &str) -> Option<&[u8]> {
        self.resolve(name).or_else(|| self.default_bytes())
    }

    /// Iterate over (name, bytes) pairs in BTree-key order.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &[u8])> + '_ {
        self.assets
            .iter()
            .map(|(name, bytes)| (name.as_str(), bytes.as_slice()))
    }

    /// Number of registered assets.
    pub fn len(&self) -> usize {
        self.assets.len()
    }

    /// Whether the registry has no assets.
    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_registry_is_empty() {
        let reg = AssetRegistry::new();
        assert!(reg.is_empty());
        assert_eq!(reg.len(), 0);
        assert!(reg.default_name().is_none());
        assert!(reg.default_bytes().is_none());
    }

    #[test]
    fn insert_returns_previous_binding_when_replacing() {
        let mut reg = AssetRegistry::new();
        let prev = reg.insert("logo", vec![1, 2, 3]);
        assert!(prev.is_none());
        let prev = reg.insert("logo", vec![4, 5, 6]);
        assert_eq!(prev, Some(vec![1, 2, 3]));
    }

    #[test]
    fn resolve_returns_registered_bytes() {
        let mut reg = AssetRegistry::new();
        reg.insert("logo", vec![1, 2, 3]);
        assert_eq!(reg.resolve("logo"), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn resolve_unknown_name_returns_none() {
        let reg = AssetRegistry::new();
        assert!(reg.resolve("does-not-exist").is_none());
    }

    #[test]
    fn resolve_default_logo_sentinel_with_no_default_returns_none() {
        let mut reg = AssetRegistry::new();
        reg.insert("logo", vec![1, 2, 3]);
        // Default not yet set — sentinel resolves to None.
        assert!(reg.resolve(DEFAULT_LOGO_SENTINEL).is_none());
    }

    #[test]
    fn resolve_default_logo_sentinel_returns_registered_default_bytes() {
        let mut reg = AssetRegistry::new();
        reg.insert("logo", vec![1, 2, 3]);
        assert!(reg.set_default("logo"));
        assert_eq!(reg.resolve(DEFAULT_LOGO_SENTINEL), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn set_default_succeeds_for_registered_asset() {
        let mut reg = AssetRegistry::new();
        reg.insert("logo", vec![1, 2, 3]);
        assert!(reg.set_default("logo"));
        assert_eq!(reg.default_name(), Some("logo"));
    }

    #[test]
    fn set_default_rejects_unregistered_name() {
        let mut reg = AssetRegistry::new();
        assert!(!reg.set_default("does-not-exist"));
        assert!(reg.default_name().is_none());
    }

    #[test]
    fn set_default_rejects_default_logo_sentinel() {
        // Setting the default name to the sentinel itself would make
        // resolve("default_logo") behavior ambiguous. Guard against it.
        let mut reg = AssetRegistry::new();
        reg.insert(DEFAULT_LOGO_SENTINEL, vec![9, 9, 9]);
        // Even though the sentinel is a registered key, set_default
        // refuses to bind to it.
        assert!(!reg.set_default(DEFAULT_LOGO_SENTINEL));
        assert!(reg.default_name().is_none());
    }

    #[test]
    fn resolve_is_case_sensitive() {
        // Per cycle plan v0.5.0 / Intention 37: case-sensitive lookups
        // make typos silent failures. Validator near-miss check (L2)
        // catches authoring bugs; runtime does not normalize case.
        let mut reg = AssetRegistry::new();
        reg.insert("logo", vec![1, 2, 3]);
        assert!(reg.resolve("LOGO").is_none());
        assert!(reg.resolve("Logo").is_none());
    }

    #[test]
    fn default_logo_sentinel_string_is_snake_case() {
        // Convention check: the reserved sentinel follows the snake_
        // case rule. Renaming it (e.g., to kebab-case) would be a
        // breaking change.
        assert_eq!(DEFAULT_LOGO_SENTINEL, "default_logo");
        assert!(!DEFAULT_LOGO_SENTINEL.contains('-'));
        assert!(!DEFAULT_LOGO_SENTINEL.contains(' '));
    }

    #[test]
    fn resolve_or_default_falls_back_to_default_on_miss() {
        let mut reg = AssetRegistry::new();
        reg.insert("logo", vec![1, 2, 3]);
        reg.set_default("logo");
        assert_eq!(
            reg.resolve_or_default("does-not-exist"),
            Some(&[1, 2, 3][..]),
        );
    }

    #[test]
    fn resolve_or_default_returns_none_when_no_default_and_miss() {
        let reg = AssetRegistry::new();
        assert!(reg.resolve_or_default("does-not-exist").is_none());
    }

    #[test]
    fn default_logo_sentinel_tracks_changes_to_default() {
        let mut reg = AssetRegistry::new();
        reg.insert("logo_a", vec![1]);
        reg.insert("logo_b", vec![2]);
        reg.set_default("logo_a");
        assert_eq!(reg.resolve(DEFAULT_LOGO_SENTINEL), Some(&[1][..]));
        reg.set_default("logo_b");
        assert_eq!(reg.resolve(DEFAULT_LOGO_SENTINEL), Some(&[2][..]));
    }

    #[test]
    fn entries_lists_registered_assets_in_btree_order() {
        let mut reg = AssetRegistry::new();
        reg.insert("zeta", vec![3]);
        reg.insert("alpha", vec![1]);
        reg.insert("middle", vec![2]);
        let names: Vec<&str> = reg.entries().map(|(n, _)| n).collect();
        assert_eq!(names, vec!["alpha", "middle", "zeta"]);
    }

    #[test]
    fn registry_len_and_is_empty_track_inserts() {
        let mut reg = AssetRegistry::new();
        assert_eq!(reg.len(), 0);
        assert!(reg.is_empty());
        reg.insert("a", vec![1]);
        assert_eq!(reg.len(), 1);
        assert!(!reg.is_empty());
        reg.insert("b", vec![2]);
        assert_eq!(reg.len(), 2);
    }
}

// <FILE>crates/tui-vfx-content/src/assets/cls_asset_registry.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
