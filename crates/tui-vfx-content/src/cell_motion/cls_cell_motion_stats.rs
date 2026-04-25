// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_motion_stats.rs</FILE> - <DESC>Cell motion result stats</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: observable scheduler aggregate counts and deterministic samples.</WCTX>
// <CLOG>0.1.0: add stats, samples, options, timing, and result structs.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_geometry::types::Position;
use tui_vfx_types::SemanticScene;

/// Active lifecycle phase sampled by the pure cell-motion scheduler.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum CellMotionPhase {
    Enter,
    Exit,
}

/// Frame timing supplied by the runtime after recipe bindings have resolved.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellMotionTiming {
    /// Active enter/exit phase.
    pub phase: CellMotionPhase,
    /// Monotonic elapsed milliseconds in the active phase; source for stagger math.
    pub phase_elapsed_ms: u64,
    /// Normalized phase progress for diagnostics only, not stagger math.
    pub phase_t: f64,
    /// Absolute runtime clock in milliseconds.
    pub absolute_t_ms: f64,
    /// Host reduced-motion policy flag.
    pub reduced_motion: bool,
    /// Future canonical recipe/layer seed supplied by runtime.
    pub seed: u64,
}

/// Scheduler options that must remain explicit rather than hidden globals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellMotionOptions {
    /// Future recipe/layer seed. Defaults to 0 until V3 wires a seed home.
    pub recipe_or_layer_seed: u64,
    /// Maximum deterministic samples to retain in stats output.
    pub sample_limit: usize,
}

impl Default for CellMotionOptions {
    fn default() -> Self {
        Self {
            recipe_or_layer_seed: 0,
            sample_limit: 8,
        }
    }
}

/// One deterministic actor sample retained for probe/debug output.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellMotionSample {
    /// Actor row-major authored index.
    pub authored_index: u32,
    /// Resolved phase start placement.
    pub from: Position,
    /// Resolved phase end placement.
    pub to: Position,
    /// Rendered snapped position, or `None` when hidden/clipped.
    pub rendered: Option<Position>,
    /// Actor-local normalized progress after stagger/duration/quantization.
    pub local_t: f32,
}

/// Aggregate scheduler observability for one sampled cell-motion frame.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellMotionStats {
    pub selected_actor_count: u32,
    pub moved_actor_count: u32,
    pub clipped_actor_count: u32,
    pub collision_count: u32,
    pub baseline_overwrite_count: u32,
    pub hidden_before_start_count: u32,
    pub hidden_after_complete_count: u32,
    pub max_stagger_offset_ms: u64,
    pub min_local_t: f32,
    pub max_local_t: f32,
    pub samples: Vec<CellMotionSample>,
}

impl Default for CellMotionStats {
    fn default() -> Self {
        Self {
            selected_actor_count: 0,
            moved_actor_count: 0,
            clipped_actor_count: 0,
            collision_count: 0,
            baseline_overwrite_count: 0,
            hidden_before_start_count: 0,
            hidden_after_complete_count: 0,
            max_stagger_offset_ms: 0,
            min_local_t: 0.0,
            max_local_t: 0.0,
            samples: Vec::new(),
        }
    }
}

/// Scene plus stats returned by [`crate::cell_motion::apply_cell_motion`].
#[derive(Clone, Debug)]
pub struct CellMotionResult {
    pub scene: SemanticScene,
    pub stats: CellMotionStats,
}

// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_motion_stats.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
