// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_motion_spec.rs</FILE> - <DESC>Cell motion spec types</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: pure tui-vfx-content spec shape for enter/exit cell motion.</WCTX>
// <CLOG>0.1.0: add CellMotionSpec, phase specs, affect, scope, and quantization validation.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_geometry::types::{EasingCurve, PathType, SnappingStrategy};

use super::{CellCollisionMode, CellMotionPhase, CellMotionVisibility, CellPlacement, CellStagger};

/// Top-level per-cell motion spec with optional enter and exit phases.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellMotionSpec {
    /// Optional enter-phase scheduler spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter: Option<CellMotionPhaseSpec>,
    /// Optional exit-phase scheduler spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<CellMotionPhaseSpec>,
}

impl CellMotionSpec {
    /// Resolve the phase spec for the sampled phase, applying phase visibility defaults.
    pub fn phase_spec(&self, phase: CellMotionPhase) -> Option<CellMotionPhaseSpec> {
        let mut spec = match phase {
            CellMotionPhase::Enter => self.enter.clone()?,
            CellMotionPhase::Exit => self.exit.clone()?,
        };
        if matches!(phase, CellMotionPhase::Exit)
            && spec.visibility == CellMotionVisibility::enter_default()
        {
            spec.visibility = CellMotionVisibility::exit_default();
        }
        Some(spec)
    }
}

/// One enter or exit phase of per-cell source remapping.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellMotionPhaseSpec {
    /// Duration of actor motion after its stagger gate has elapsed.
    pub duration_ms: u64,
    /// Easing applied after optional quantization.
    #[serde(default)]
    pub easing: EasingCurve,
    /// Carrier path sampled between resolved from/via/to placements.
    #[serde(default = "default_route_linear")]
    pub route: PathType,
    /// Dynamic path treatments layered over the route. Uses PathType until V3 dynamic DTOs move here.
    #[serde(default)]
    pub dynamics: Vec<PathType>,
    /// Required phase start placement.
    pub from: CellPlacement,
    /// Optional waypoint/control placement for path types that use one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<CellPlacement>,
    /// Required phase destination placement.
    pub to: CellPlacement,
    /// Actor-specific start delay.
    #[serde(default)]
    pub stagger: CellStagger,
    /// Grid snapping policy for sampled coordinates.
    #[serde(default = "default_snap_round")]
    pub snap: SnappingStrategy,
    /// Optional stop-motion quantization. Runtime rejects values below 2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantize_steps: Option<u32>,
    /// Collision policy for moved actors targeting the same destination.
    #[serde(default)]
    pub collision: CellCollisionMode,
    /// Whether empty cells are actors.
    #[serde(default)]
    pub affect: CellMotionAffect,
    /// Authored-coordinate selection scope. `None` means all cells.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<CellMotionScope>,
    /// Visibility gates. Defaults are phase-sensitive in `CellMotionSpec::phase_spec`.
    #[serde(default)]
    pub visibility: CellMotionVisibility,
}

impl CellMotionPhaseSpec {
    /// Validate runtime invariants that recipe validation must also enforce.
    pub fn validate(&self) -> Result<(), CellMotionError> {
        if matches!(self.quantize_steps, Some(0 | 1)) {
            return Err(CellMotionError::InvalidQuantizeSteps);
        }
        Ok(())
    }
}

/// Scheduler validation/runtime-guard error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellMotionError {
    InvalidQuantizeSteps,
}

impl std::fmt::Display for CellMotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidQuantizeSteps => f.write_str("cell_motion quantize_steps must be >= 2"),
        }
    }
}
impl std::error::Error for CellMotionError {}

/// Actor-selection affect policy.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CellMotionAffect {
    #[default]
    NonEmpty,
    All,
}

/// Authored-coordinate cell selection scope for pure content scheduler tests and debug fixtures.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CellMotionScope {
    /// Select all source coordinates.
    All,
    /// Select exact row indexes.
    Rows { rows: Vec<u16> },
    /// Select rows in `[start, end)`.
    RowRange { start: u16, end: u16 },
    /// Select exact column indexes.
    Columns { columns: Vec<u16> },
    /// Select columns in `[start, end)`.
    ColumnRange { start: u16, end: u16 },
    /// Select a source-space rectangle.
    Rect {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    },
    /// Select exact source cells.
    Cells { cells: Vec<CellMotionCoord> },
}

/// Exact authored source coordinate used by [`CellMotionScope::Cells`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellMotionCoord {
    /// Authored source x coordinate.
    pub x: u16,
    /// Authored source y coordinate.
    pub y: u16,
}

impl From<(u16, u16)> for CellMotionCoord {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

fn default_route_linear() -> PathType {
    PathType::Linear
}
fn default_snap_round() -> SnappingStrategy {
    SnappingStrategy::Round
}

impl CellMotionScope {
    /// Return true when authored coordinate `(x, y)` is selected by this scope.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        match self {
            Self::All => true,
            Self::Rows { rows } => rows.contains(&y),
            Self::RowRange { start, end } => y >= *start && y < *end,
            Self::Columns { columns } => columns.contains(&x),
            Self::ColumnRange { start, end } => x >= *start && x < *end,
            Self::Rect {
                x: rx,
                y: ry,
                width,
                height,
            } => {
                x >= *rx
                    && x < rx.saturating_add(*width)
                    && y >= *ry
                    && y < ry.saturating_add(*height)
            }
            Self::Cells { cells } => cells.iter().any(|cell| cell.x == x && cell.y == y),
        }
    }
}

impl Default for CellMotionPhaseSpec {
    fn default() -> Self {
        Self {
            duration_ms: 0,
            easing: EasingCurve::default(),
            route: PathType::Linear,
            dynamics: Vec::new(),
            from: CellPlacement::Authored,
            via: None,
            to: CellPlacement::Authored,
            stagger: CellStagger::None,
            snap: SnappingStrategy::Round,
            quantize_steps: None,
            collision: CellCollisionMode::SourceOrder,
            affect: CellMotionAffect::NonEmpty,
            scope: None,
            visibility: CellMotionVisibility::enter_default(),
        }
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_motion_spec.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
