// <FILE>crates/tui-vfx-contract/src/cls_style_color_source.rs</FILE> - <DESC>Style color interpolation source DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 transition recipe-oracle pass: canvas-aware style fades are distinct from opacity fades.</WCTX>
// <CLOG>0.1.0: INIT — add style color source vocabulary for color-fade tracks.</CLOG>

use tui_vfx_types::Color;

/// Source of a color used by style interpolation tracks.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum StyleColorSource {
    /// Explicit RGBA color.
    ExplicitColor {
        /// Explicit RGBA color value.
        explicit_color: Color,
    },
    /// Current canvas/background color under the subject.
    Canvas,
    /// Fully transparent color.
    Transparent,
    /// Subject's current authored color.
    Current,
    /// Sampled source surface color.
    SampledSource,
    /// Destination/composited color.
    Destination,
}

// <FILE>crates/tui-vfx-contract/src/cls_style_color_source.rs</FILE> - <DESC>Style color interpolation source DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
