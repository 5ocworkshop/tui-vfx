// <FILE>crates/tui-vfx-content/src/mechanical/mod.rs</FILE> - <DESC>Private mechanical display helper module</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: register cycle-resolution and route-building helpers.</WCTX>
// <CLOG>0.3.0: register enum_cycle_error, cls_resolved_cycle, fnc_expand_cycle_preset, fnc_normalize_cycle_face, fnc_weighted_cycle_order, fnc_resolve_mechanical_cycle, fnc_route_between.</CLOG>

//! Mechanism-specific helpers for fixed-window mechanical display effects.
//!
//! This module is private to `tui-vfx-content`. It samples paired character-cell
//! grids for mechanisms such as Odometer; the public transformer boundary remains
//! `TextTransformer -> Cow<str>`. `mechanical` is mechanism-specific;
//! `cell_motion` is general per-cell source remapping.
//!
//! Phase 2 adds cycle-resolution helpers that translate
//! `MechanicalCycleConfig` into routes through ordered/circular face
//! supplies. The route helpers are independent of any specific
//! mechanism (Odometer, SplitFlap) and are exercised by their own
//! inline tests.

mod cls_resolved_cycle;
mod enum_cycle_error;
mod fnc_expand_cycle_preset;
mod fnc_grid_text;
mod fnc_normalize_cycle_face;
mod fnc_resolve_mechanical_cycle;
mod fnc_roll_grid_window;
mod fnc_route_between;
mod fnc_split_flap_tile_frame;
mod fnc_weighted_cycle_order;
mod types;

pub(crate) use cls_resolved_cycle::{
    MechanicalCycleRoute, NumericRouteHint, ResolvedMechanicalCycle, ResolvedMechanicalFace,
};
pub(crate) use enum_cycle_error::MechanicalCycleError;
pub(crate) use fnc_expand_cycle_preset::expand_cycle_preset;
pub(crate) use fnc_grid_text::{grid_to_text, paired_grids};
pub(crate) use fnc_normalize_cycle_face::normalize_cycle_face;
pub(crate) use fnc_resolve_mechanical_cycle::resolve_mechanical_cycle;
pub(crate) use fnc_roll_grid_window::roll_grid_window;
pub(crate) use fnc_route_between::route_between;
pub(crate) use fnc_split_flap_tile_frame::split_flap_tile_frame;
pub(crate) use fnc_weighted_cycle_order::{shuffle_in_place, weighted_cycle_order};
pub(crate) use types::{
    MechanicalSizing, MechanicalSource, MechanicalTile, validate_split_flap_tile,
};

// <FILE>crates/tui-vfx-content/src/mechanical/mod.rs</FILE> - <DESC>Private mechanical display helper module</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
