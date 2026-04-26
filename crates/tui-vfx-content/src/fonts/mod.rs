// <FILE>crates/tui-vfx-content/src/fonts/mod.rs</FILE> - <DESC>Embedded fonts module hosting the project-default Line 3x3 glyph table (Intention 36)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 5 of mechanical circular content cycles plan: register the canonical Line 3x3 face used as the default and runtime-fallback font.</WCTX>
// <CLOG>0.1.0: introduce fonts module with col_line_3x3_heavy_glyphs and fnc_lookup_line_3x3_glyph.</CLOG>

//! Embedded fonts.
//!
//! Currently the canonical Line 3x3 heavy glyph table — the project's
//! default and runtime-fallback font per Intention 36 in
//! `steering/INTENTIONS.md`. Other faces will land here as they earn
//! their place; the table is the source of truth and other crates
//! mirror from this home rather than carrying their own copies.

mod col_line_3x3_heavy_glyphs;
mod fnc_lookup_line_3x3_glyph;

pub use col_line_3x3_heavy_glyphs::line_3x3_heavy_glyphs;
pub use fnc_lookup_line_3x3_glyph::{lookup_line_3x3_glyph, render_line_3x3_text};

// <FILE>crates/tui-vfx-content/src/fonts/mod.rs</FILE> - <DESC>Embedded fonts module</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
