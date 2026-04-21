// <FILE>src/recipe_schema/scene/enum_ra_layer_overflow.rs</FILE> - <DESC>Overflow policy for individual scene layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema enum for per-layer overflow behavior.</WCTX>
// <CLOG>0.1.0: add clip/hide/wrap layer overflow enum.</CLOG>

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaLayerOverflow {
    #[default]
    Clip,
    Hide,
    Wrap,
}

// <FILE>src/recipe_schema/scene/enum_ra_layer_overflow.rs</FILE> - <DESC>RaLayerOverflow</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
