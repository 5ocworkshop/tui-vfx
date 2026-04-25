// <FILE>tui-vfx-geometry/src/wipe/mod.rs</FILE> - <DESC>Wipe-direction vocabulary and shared geometry helpers (visibility math) consumed by both the Wipe mask and the RevealWipe shader</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Capability audit recommendation 1.2 + 1.3 — single-source-of-truth wipe geometry so authors get the same direction vocabulary at both the mask and shader layers, and so we can carry corner-out/corner-in directions in one place.</WCTX>
// <CLOG>1.0.0: introduce shared wipe module hosting WipeDirection (re-exported from types::cls_wipe_direction) and the wipe_progress visibility helper.</CLOG>

//! # Wipe geometry
//!
//! Shared wipe-progress math used by both the `Wipe` mask (in
//! `tui-vfx-compositor`) and the `RevealWipe` shader (in `tui-vfx-style`).
//! Hosting the math here lets both consumers stay in sync as the
//! direction vocabulary evolves and avoids the previous duplication that
//! caused the shader's direction set to drift behind the mask's.
//!
//! See [`fnc_wipe_progress`] for the per-cell visibility predicate.

pub mod fnc_wipe_progress;

pub use fnc_wipe_progress::{wipe_progress, wipe_visible_at};

// Re-export the canonical direction enum at the wipe module level so
// callers can `use tui_vfx_geometry::wipe::WipeDirection;` without
// reaching into `types::`.
pub use crate::types::cls_wipe_direction::WipeDirection;

// <FILE>tui-vfx-geometry/src/wipe/mod.rs</FILE> - <DESC>Wipe module root</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
