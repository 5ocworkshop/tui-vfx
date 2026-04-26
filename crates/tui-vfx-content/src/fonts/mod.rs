// <FILE>crates/tui-vfx-content/src/fonts/mod.rs</FILE> - <DESC>Embedded fonts module hosting the project-default Line 3x3 glyph table (Intention 36) plus FontGlyphTable / FontRegistry abstractions</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Slice 6.2 of mechanical circular content cycles plan: add FontGlyphTable enum and FontRegistry with default-font sentinel routing so content effects can resolve runtime-bindable font names.</WCTX>
// <CLOG>0.2.0: add cls_font_glyph_table (FontGlyphTable enum with Line3x3 variant, lookup_glyph_rows / render_text / render_glyph) and cls_font_registry (FontRegistry with insert / resolve / default + DEFAULT_FONT_SENTINEL routing). 0.1.0: introduce fonts module with col_line_3x3_heavy_glyphs and fnc_lookup_line_3x3_glyph.</CLOG>

//! Embedded fonts.
//!
//! Currently the canonical Line 3x3 heavy glyph table — the project's
//! default and runtime-fallback font per Intention 36 in
//! `steering/INTENTIONS.md`. Other faces will land here as they earn
//! their place; the table is the source of truth and other crates
//! mirror from this home rather than carrying their own copies.
//!
//! Slice 6.2 of the mechanical circular content cycles plan adds
//! [`FontGlyphTable`] (the pluggable per-character glyph contract) and
//! [`FontRegistry`] (the name → glyph-table registry with default-
//! sentinel routing). Recipes that bind a font through the
//! `requires_assets` declaration consume the registry's `resolve` to
//! turn an authored font name into a glyph table.

mod cls_font_glyph_table;
mod cls_font_registry;
mod col_line_3x3_heavy_glyphs;
mod fnc_lookup_line_3x3_glyph;

pub use cls_font_glyph_table::FontGlyphTable;
pub use cls_font_registry::{FontRegistry, DEFAULT_FONT_SENTINEL};
pub use col_line_3x3_heavy_glyphs::line_3x3_heavy_glyphs;
pub use fnc_lookup_line_3x3_glyph::{lookup_line_3x3_glyph, render_line_3x3_text};

// <FILE>crates/tui-vfx-content/src/fonts/mod.rs</FILE> - <DESC>Embedded fonts module</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
