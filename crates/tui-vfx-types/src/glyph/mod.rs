// <FILE>crates/tui-vfx-types/src/glyph/mod.rs</FILE> - <DESC>Glyph module root: encoder vocabulary and subcell sampling helpers for field-effect rendering</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 3: glyph module for water/fire/future field-effect glyph encoding</WCTX>
// <CLOG>0.1.0: initial implementation re-exporting GlyphEncoder, sample_eight_subcells, sample_eight_subcells_with_slope, and SUBCELL_OFFSETS</CLOG>

//! Glyph encoding vocabulary for scalar-field-to-terminal-character rendering.
//!
//! This module provides the shared encoding primitives that any 2D scalar field
//! (water, fire, terrain, audio) can drive through the `ScalarFieldGlyphFilter`
//! in `tui-vfx-compositor`.
//!
//! ## Core types
//!
//! - [`GlyphEncoder`] — closed enum with five variants: `BrailleSubcell`,
//!   `BrailleEighths`, `BlockHorizontal`, `BlockVertical`, `Ramp`.
//! - [`sample_eight_subcells`] — sample any `Signal` at the eight braille
//!   subcell positions (eight full evaluations per cell).
//! - [`sample_eight_subcells_with_slope`] — sample any `SignalWithSlope` once
//!   and linearly interpolate subcell values from the slope (one evaluation).
//! - [`SUBCELL_OFFSETS`] — the `(dx, dy)` table for the eight dot positions,
//!   ordered by braille dot number minus one.

pub mod cls_glyph_encoder;
pub mod fnc_sample_eight_subcells;

pub use cls_glyph_encoder::GlyphEncoder;
pub use fnc_sample_eight_subcells::{
    SUBCELL_OFFSETS, sample_eight_subcells, sample_eight_subcells_with_slope,
};

// <FILE>crates/tui-vfx-types/src/glyph/mod.rs</FILE> - <DESC>Glyph module root: encoder vocabulary and subcell sampling helpers for field-effect rendering</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
