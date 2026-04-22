// <FILE>tui-vfx-style/src/models/v3/enum_vfx_guidance_cue_behavior.rs</FILE> - <DESC>V3 guidance-cue family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a grouped V3 home for focus/wayfinding cue shaders while preserving the legacy FocusedRowGradient, AffordanceWake, and WayfindingNode variants for current playback.</WCTX>
// <CLOG>Define the V3 guidance-cue family enums and payloads that lift the shared apply-to and cue behavior surface out of the legacy flat shader catalog.</CLOG>

//! V3 behavior surface for guidance-cue family shaders.
//!
//! This family groups the calm UX guidance cues currently exposed as
//! `FocusedRowGradient`, `AffordanceWake`, and `WayfindingNode`.

use crate::models::{ColorConfig, FalloffType};
use serde::{Deserialize, Serialize};

/// Shared channel-target surface for V3 guidance-cue shaders.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxGuidanceCueApplyTo {
    /// Affect only the foreground channel.
    #[default]
    Foreground,
    /// Affect only the background channel.
    Background,
    /// Affect both channels.
    Both,
}

/// Zone family for affordance wake cues.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxAffordanceWakeZone {
    /// All outer edges.
    #[default]
    AllEdges,
    Corners,
    LeftRail,
    RightRail,
    TopRail,
    BottomRail,
}

/// Node payload for wayfinding guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxWayfindingNode {
    /// Node column.
    pub x: u16,
    /// Node row.
    pub y: u16,
}

/// Behavior surface for the V3 guidance-cue family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxGuidanceCueBehavior {
    /// Row-centered focus gradient for lists and stacked items.
    FocusedRow {
        /// Explicit selected row index.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        selected_row: Option<u16>,
        /// Optional runtime binding overriding the selected row.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        selected_row_binding: Option<String>,
        /// Ratio used when no explicit selected row is supplied.
        #[config(default = 0.5)]
        selected_row_ratio: f32,
        /// Optional runtime binding overriding the selected row ratio.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        selected_row_ratio_binding: Option<String>,
        /// Spread/falloff distance in rows.
        #[config(default = 5)]
        falloff_distance: u16,
        /// Brightest color at the focused row.
        bright_color: ColorConfig,
        /// Dimmest color at the edge of the falloff.
        dim_color: ColorConfig,
        /// Channel-target policy.
        #[serde(default)]
        apply_to: VfxGuidanceCueApplyTo,
    },
    /// Dormant-to-active edge/corner wake cue.
    AffordanceWake {
        /// Cue tint.
        color: ColorConfig,
        /// Zone family to animate.
        #[serde(default)]
        zone: VfxAffordanceWakeZone,
        /// Radius in cells.
        #[config(default = 2)]
        radius: u8,
        /// Falloff curve.
        #[serde(default)]
        falloff: FalloffType,
        /// Static cue progress.
        #[config(default = 0.0)]
        progress: f32,
        /// Optional runtime binding overriding the progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        progress_binding: Option<String>,
        /// Baseline/rest intensity.
        #[config(default = 0.0)]
        rest_intensity: f32,
        /// Peak intensity.
        #[config(default = 0.25)]
        peak_intensity: f32,
        /// Channel-target policy.
        #[serde(default)]
        apply_to: VfxGuidanceCueApplyTo,
    },
    /// Explicit node-based wayfinding cue for breadcrumbs and progress steps.
    WayfindingNode {
        /// Cue tint.
        color: ColorConfig,
        /// Authored node positions.
        nodes: Vec<VfxWayfindingNode>,
        /// Radius in cells.
        #[config(default = 2)]
        radius: u8,
        /// Peak intensity.
        #[config(default = 0.22)]
        intensity: f32,
        /// Static current node index.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        current_index: Option<u16>,
        /// Optional runtime binding overriding the current index.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        current_index_binding: Option<String>,
        /// Strength applied to previous nodes.
        #[config(default = 0.45)]
        previous_strength: f32,
        /// Strength applied to future nodes.
        #[config(default = 0.0)]
        future_strength: f32,
        /// Optional pulse speed for the current node.
        #[serde(default)]
        pulse_speed: f32,
        /// Channel-target policy.
        #[serde(default)]
        apply_to: VfxGuidanceCueApplyTo,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_guidance_cue_behavior.rs</FILE> - <DESC>V3 guidance-cue family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
