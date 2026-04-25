// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_motion_visibility.rs</FILE> - <DESC>Cell motion visibility gates</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: enter/exit before-start and after-complete visibility.</WCTX>
// <CLOG>0.1.0: add visibility modes and phase defaults.</CLOG>

use serde::{Deserialize, Serialize};

/// Visibility behavior used before an actor starts and after it completes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum CellVisibilityMode {
    /// Do not write the actor.
    Hidden,
    /// Write the actor at its resolved `from` placement.
    AtFrom,
    /// Write the actor at its resolved `to` placement.
    AtTo,
    /// Hold the boundary placement for this stateless sample.
    Hold,
}

/// Visibility gates for one cell-motion phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellMotionVisibility {
    /// Mode used before the actor-specific stagger offset has elapsed.
    pub before_start: CellVisibilityMode,
    /// Mode used after the actor-specific duration has completed.
    pub after_complete: CellVisibilityMode,
}

impl CellMotionVisibility {
    /// Default enter visibility: hidden before start, hold after completion.
    pub const fn enter_default() -> Self {
        Self {
            before_start: CellVisibilityMode::Hidden,
            after_complete: CellVisibilityMode::Hold,
        }
    }

    /// Default exit visibility: hold before start, hidden after completion.
    pub const fn exit_default() -> Self {
        Self {
            before_start: CellVisibilityMode::Hold,
            after_complete: CellVisibilityMode::Hidden,
        }
    }
}

impl Default for CellMotionVisibility {
    fn default() -> Self {
        Self::enter_default()
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_motion_visibility.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
