// <FILE>crates/tui-vfx-content/src/fonts/cls_font_registry.rs</FILE> - <DESC>Name-to-FontGlyphTable registry with default-fallback resolution per Intention 36</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Slice 6.2 of mechanical circular content cycles plan: pluggable name → glyph-table mapping consumed by Mechanical cycle face expansion. Registers Line 3x3 as the default; future RSF-backed faces register additional entries without changing consumer code.</WCTX>
// <CLOG>0.1.0: introduce FontRegistry with insert / resolve / default / set_default; recognize the default_font sentinel literal as the canonical alias for the registered default (per cycle plan v0.5.0 Phase 6 sub-plan). Inline tests for default registration, named lookup, sentinel resolution, missing-name fallback, and case-sensitivity contract.</CLOG>

//! Name-to-glyph-table registry consumed by content-effect face expansion.
//!
//! The registry is the runtime authority Intention 36 names: it owns the
//! mapping from font name to glyph table, holds one named entry as the
//! default, and resolves the `default_font` sentinel literal to that
//! default. Recipes never carry a real font name when they want
//! "whatever the player chose" — they reference the sentinel and the
//! registry routes.
//!
//! Today the registry self-registers the embedded Line 3x3 table
//! (`FontGlyphTable::Line3x3`) on construction and names it as the
//! default. Host code that wants to add more fonts (e.g. RSF-loaded
//! faces) inserts them and may rebind the default via `set_default`.

use std::collections::BTreeMap;

use super::cls_font_glyph_table::FontGlyphTable;

/// The reserved sentinel literal recipes use to mean "the player's
/// registered default font" without naming a specific font.
///
/// Snake_case ASCII per the cycle-plan v0.5.0 casing rule (case-
/// sensitive lookups make typos silent failures). The sentinel is
/// part of the public schema contract; renaming it is a breaking
/// change.
pub const DEFAULT_FONT_SENTINEL: &str = "default_font";

/// Pluggable name → glyph-table registry.
///
/// Construction registers `FontGlyphTable::Line3x3` under the name
/// `"line-3x3"` and sets it as the default. Host code can register
/// additional named tables and may rebind the default to a different
/// name. The `default_font` sentinel always routes to whatever name
/// is currently the default.
#[derive(Debug, Clone)]
pub struct FontRegistry {
    fonts: BTreeMap<String, FontGlyphTable>,
    default_name: String,
}

impl FontRegistry {
    /// Construct a registry pre-loaded with the embedded Line 3x3
    /// face under the name `"line-3x3"` and set as the default.
    pub fn new() -> Self {
        let mut fonts = BTreeMap::new();
        fonts.insert("line-3x3".to_string(), FontGlyphTable::Line3x3);
        Self {
            fonts,
            default_name: "line-3x3".to_string(),
        }
    }

    /// Insert (or replace) a named font in the registry. Returns the
    /// previous binding for the name if any.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        table: FontGlyphTable,
    ) -> Option<FontGlyphTable> {
        self.fonts.insert(name.into(), table)
    }

    /// Set the default font's name. Returns true if the name resolves
    /// to a registered font (in which case the default is updated);
    /// returns false and leaves the default unchanged otherwise. Refuse
    /// to set the default to the sentinel literal `default_font` itself
    /// — that would make resolution recursive.
    pub fn set_default(&mut self, name: impl Into<String>) -> bool {
        let name = name.into();
        if name == DEFAULT_FONT_SENTINEL {
            return false;
        }
        if !self.fonts.contains_key(&name) {
            return false;
        }
        self.default_name = name;
        true
    }

    /// The current default font's registered name.
    pub fn default_name(&self) -> &str {
        self.default_name.as_str()
    }

    /// The current default font's glyph table.
    pub fn default_table(&self) -> FontGlyphTable {
        // BTreeMap::get always finds default_name because new() and
        // set_default() guarantee invariance: default_name is always a
        // registered key.
        self.fonts
            .get(&self.default_name)
            .copied()
            .expect("default_name must always reference a registered font")
    }

    /// Resolve a name to a glyph table.
    ///
    /// Lookup precedence:
    /// 1. The reserved sentinel `default_font` short-circuits to the
    ///    current default table regardless of the default's actual name.
    /// 2. Otherwise, looks up `name` in the registered fonts.
    /// 3. Returns `None` if unrecognized — callers fall back to the
    ///    default per Intention 36 (`fallback to default on miss with
    ///    a trace warning`) or surface the miss to the validator.
    pub fn resolve(&self, name: &str) -> Option<FontGlyphTable> {
        if name == DEFAULT_FONT_SENTINEL {
            return Some(self.default_table());
        }
        self.fonts.get(name).copied()
    }

    /// Resolve a name to a glyph table with implicit fallback to the
    /// default. This is the Intention-36 fallback path: missing fonts
    /// don't fail rendering, they degrade to the project default with
    /// the expectation that callers emit a trace warning.
    pub fn resolve_or_default(&self, name: &str) -> FontGlyphTable {
        self.resolve(name).unwrap_or_else(|| self.default_table())
    }

    /// Iterate over (name, table) pairs in registration order.
    pub fn entries(&self) -> impl Iterator<Item = (&str, FontGlyphTable)> + '_ {
        self.fonts.iter().map(|(name, table)| (name.as_str(), *table))
    }
}

impl Default for FontRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_registry_has_line_3x3_as_default() {
        let reg = FontRegistry::new();
        assert_eq!(reg.default_name(), "line-3x3");
        assert_eq!(reg.default_table(), FontGlyphTable::Line3x3);
    }

    #[test]
    fn resolve_named_returns_registered_table() {
        let reg = FontRegistry::new();
        assert_eq!(reg.resolve("line-3x3"), Some(FontGlyphTable::Line3x3));
    }

    #[test]
    fn resolve_default_font_sentinel_returns_default_table() {
        let reg = FontRegistry::new();
        assert_eq!(reg.resolve(DEFAULT_FONT_SENTINEL), Some(FontGlyphTable::Line3x3));
    }

    #[test]
    fn resolve_unknown_name_returns_none() {
        let reg = FontRegistry::new();
        assert!(reg.resolve("not-a-real-font").is_none());
    }

    #[test]
    fn resolve_or_default_falls_back_to_default_on_miss() {
        let reg = FontRegistry::new();
        assert_eq!(
            reg.resolve_or_default("not-a-real-font"),
            FontGlyphTable::Line3x3,
        );
    }

    #[test]
    fn resolve_is_case_sensitive() {
        // Per cycle plan v0.5.0 / Intention 37: case-sensitive lookups
        // make typos silent failures. The validator near-miss check
        // (planned at L2) catches authoring bugs; the runtime itself
        // does not normalize case.
        let reg = FontRegistry::new();
        assert!(reg.resolve("LINE-3X3").is_none());
        assert!(reg.resolve("Line-3x3").is_none());
    }

    #[test]
    fn default_font_sentinel_string_is_snake_case() {
        // Convention check: the reserved sentinel follows the snake_case
        // rule from Intention 37 / cycle plan v0.5.0. Renaming it (e.g.,
        // to kebab-case) would be a breaking change.
        assert_eq!(DEFAULT_FONT_SENTINEL, "default_font");
        assert!(!DEFAULT_FONT_SENTINEL.contains('-'));
        assert!(!DEFAULT_FONT_SENTINEL.contains(' '));
    }

    #[test]
    fn insert_returns_previous_binding_when_replacing() {
        let mut reg = FontRegistry::new();
        let prev = reg.insert("line-3x3", FontGlyphTable::Line3x3);
        assert_eq!(prev, Some(FontGlyphTable::Line3x3));
    }

    #[test]
    fn set_default_succeeds_for_registered_font() {
        let mut reg = FontRegistry::new();
        reg.insert("alt", FontGlyphTable::Line3x3);
        assert!(reg.set_default("alt"));
        assert_eq!(reg.default_name(), "alt");
    }

    #[test]
    fn set_default_rejects_unregistered_name() {
        let mut reg = FontRegistry::new();
        assert!(!reg.set_default("does-not-exist"));
        assert_eq!(reg.default_name(), "line-3x3");
    }

    #[test]
    fn set_default_rejects_default_font_sentinel() {
        // Setting the default name to the sentinel itself would make
        // resolve("default_font") infinitely recursive (or at minimum
        // ambiguous). Guard against it.
        let mut reg = FontRegistry::new();
        assert!(!reg.set_default(DEFAULT_FONT_SENTINEL));
        assert_eq!(reg.default_name(), "line-3x3");
    }

    #[test]
    fn default_font_sentinel_tracks_changes_to_default() {
        let mut reg = FontRegistry::new();
        reg.insert("alt", FontGlyphTable::Line3x3);
        reg.set_default("alt");
        // Sentinel resolution still works after default rebinds.
        assert_eq!(reg.resolve(DEFAULT_FONT_SENTINEL), Some(FontGlyphTable::Line3x3));
        assert_eq!(reg.default_name(), "alt");
    }

    #[test]
    fn entries_lists_registered_fonts_in_btree_order() {
        let mut reg = FontRegistry::new();
        reg.insert("alpha", FontGlyphTable::Line3x3);
        reg.insert("zeta", FontGlyphTable::Line3x3);
        let names: Vec<&str> = reg.entries().map(|(n, _)| n).collect();
        // BTreeMap iterates in lexicographic key order.
        assert_eq!(names, vec!["alpha", "line-3x3", "zeta"]);
    }
}

// <FILE>crates/tui-vfx-content/src/fonts/cls_font_registry.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
