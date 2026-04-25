// <FILE>crates/tui-vfx-content/src/mechanical/mod.rs</FILE> - <DESC>Private mechanical display helper module</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 mechanical helpers for grid-first odometer tile roll.</WCTX>
// <CLOG>Add private text-grid and roll-window helpers for Odometer.</CLOG>

//! Mechanism-specific helpers for fixed-window mechanical display effects.
//!
//! This module is private to `tui-vfx-content`. It samples paired character-cell
//! grids for mechanisms such as Odometer; the public transformer boundary remains
//! `TextTransformer -> Cow<str>`. `mechanical` is mechanism-specific;
//! `cell_motion` is general per-cell source remapping.

mod fnc_grid_text;
mod fnc_roll_grid_window;
mod types;

pub(crate) use fnc_grid_text::{grid_to_text, paired_grids};
pub(crate) use fnc_roll_grid_window::roll_grid_window;
pub(crate) use types::{MechanicalSizing, MechanicalSource, MechanicalTile};

// <FILE>crates/tui-vfx-content/src/mechanical/mod.rs</FILE> - <DESC>Private mechanical display helper module</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
