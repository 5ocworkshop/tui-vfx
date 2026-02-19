// <FILE>crates/tui-vfx-shadow/src/renderers/mod.rs</FILE> - <DESC>Shadow renderer implementations</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New crate for theme-aware shadow rendering with multiple styles</WCTX>
// <CLOG>Initial creation - re-export all renderer types</CLOG>

//! Shadow renderer implementations.
//!
//! This module contains the various shadow rendering strategies:
//!
//! - [`HalfBlockRenderer`] - Sub-cell precision using Unicode half-block characters
//! - [`BrailleRenderer`] - Dithered shadows using braille patterns
//! - [`SolidRenderer`] - Simple solid-color background fills
//! - [`GradientRenderer`] - Multi-layer gradient shadows

mod cls_braille;
mod cls_gradient;
mod cls_half_block;
mod cls_solid;

pub use cls_braille::BrailleRenderer;
pub use cls_gradient::GradientRenderer;
pub use cls_half_block::HalfBlockRenderer;
pub use cls_solid::SolidRenderer;

// <FILE>crates/tui-vfx-shadow/src/renderers/mod.rs</FILE> - <DESC>Shadow renderer implementations</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
