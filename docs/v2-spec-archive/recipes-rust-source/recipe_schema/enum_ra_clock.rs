// <FILE>src/recipe_schema/enum_ra_clock.rs</FILE> - <DESC>Unified clock-source enum for continuous blocks and per-effect overrides</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive schema enum used by RaContinuousConfig and optional per-effect clock overrides.</WCTX>
// <CLOG>0.1.0: add PhaseT/LoopT/AbsoluteT RaClock enum.</CLOG>

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaClock {
    PhaseT,
    #[default]
    LoopT,
    AbsoluteT,
}

// <FILE>src/recipe_schema/enum_ra_clock.rs</FILE> - <DESC>RaClock</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
