// <FILE>crates/tui-vfx-content/src/cell_motion/mod.rs</FILE> - <DESC>Pure content per-cell motion module</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: public cell-motion types and deterministic scheduler.</WCTX>
// <CLOG>0.1.0: add pure tui-vfx-content cell-motion substrate and focused tests.</CLOG>

//! Pure content-local per-cell motion scheduler for V3 source-cell remapping.

pub mod cls_cell_actor;
pub(crate) mod cls_cell_motion_candidate;
pub mod cls_cell_motion_spec;
pub mod cls_cell_motion_stats;
pub mod cls_cell_motion_visibility;
pub mod enum_cell_collision_mode;
pub mod enum_cell_placement;
pub mod enum_cell_stagger;
pub mod fnc_apply_cell_motion;
pub(crate) mod fnc_cell_motion_visibility_position;
pub(crate) mod fnc_cell_motion_winner_index;
pub(crate) mod fnc_clip_cell_motion_position;
pub mod fnc_collect_cell_actors;
pub(crate) mod fnc_lower_cell_motion_path;
pub mod fnc_resolve_actor_offset_ms;
pub mod fnc_resolve_cell_placement;
pub(crate) mod fnc_sample_cell_motion_position;
pub(crate) mod fnc_selected_cell_actor_bounds;
pub(crate) mod fnc_update_cell_motion_t_range;

pub use cls_cell_actor::CellActor;
pub use cls_cell_motion_spec::{
    CellMotionAffect, CellMotionCoord, CellMotionError, CellMotionPhaseSpec, CellMotionScope,
    CellMotionSpec,
};
pub use cls_cell_motion_stats::{
    CellMotionOptions, CellMotionPhase, CellMotionResult, CellMotionSample, CellMotionStats,
    CellMotionTiming,
};
pub use cls_cell_motion_visibility::{CellMotionVisibility, CellVisibilityMode};
pub use enum_cell_collision_mode::CellCollisionMode;
pub use enum_cell_placement::{CellPlacement, CellPlacementBasis};
pub use enum_cell_stagger::{CellStagger, CellStaggerAxis, CellStaggerDirection};
pub use fnc_apply_cell_motion::apply_cell_motion;
pub use fnc_collect_cell_actors::collect_cell_actors;
pub use fnc_resolve_actor_offset_ms::resolve_actor_offset_ms;
pub use fnc_resolve_cell_placement::{CellPlacementContext, resolve_cell_placement};

// <FILE>crates/tui-vfx-content/src/cell_motion/mod.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
