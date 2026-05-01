// <FILE>tui-vfx-style/src/models/v3/enum_vfx_edge_distortion_behavior.rs</FILE> - <DESC>V3 edge-distortion family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a grouped V3 home for primitive edge-distortion shaders while preserving the legacy GlitchLines, ChromaticEdge, and SubCellShake variants for current playback.</WCTX>
// <CLOG>Define the V3 edge-distortion family enums and payloads that lift the shared edge/glitch micro-treatment behavior out of the legacy flat shader catalog.</CLOG>

//! V3 behavior surface for edge-distortion family shaders.
//!
//! This family groups the primitive/substrate-aligned edge distortion and
//! glitch micro-treatments currently exposed as `GlitchLines`,
//! `ChromaticEdge`, and `SubCellShake`.

use crate::models::{ColorConfig, NoiseType};
use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

/// Axis policy for shake-style distortion.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxEdgeDistortionAxis {
    /// Horizontal only.
    Horizontal,
    /// Vertical only.
    Vertical,
    /// Both axes.
    #[default]
    Both,
}

/// Behavior surface for the V3 edge-distortion family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxEdgeDistortionBehavior {
    /// Random horizontal interference lines and optional flash behavior.
    GlitchLines {
        /// Seed for deterministic randomness.
        #[config(default = 42)]
        seed: u64,
        /// Probability of a line appearing.
        #[config(default = 0.5)]
        intensity: f32,
        /// Maximum number of interference lines.
        #[config(default = 6)]
        max_lines: u16,
        /// Speed multiplier for pattern changes.
        #[config(default = 1.0)]
        speed: f32,
        /// Chance of a full-row flash.
        #[config(default = 0.0)]
        flash_chance: f32,
        /// Optional pulse color.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pulse_color: Option<ColorConfig>,
        /// Optional foreground color installed before glitch styling.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        base_color: Option<ColorConfig>,
        /// Optional normalized time at which italic styling begins.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        italic_start: Option<f32>,
        /// Optional normalized time at which italic styling ends.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        italic_end: Option<f32>,
        /// Whether row-level interference lines are enabled.
        #[serde(default = "default_true")]
        #[config(default = true)]
        lines_enabled: bool,
        /// Pulse speed.
        #[config(default = 0.5)]
        pulse_speed: f32,
        /// Whether italic should be applied during flashes.
        #[config(default = false)]
        italic_on_flash: bool,
        /// Number of time slots a flash persists.
        #[config(default = 1)]
        flash_hold: u32,
        /// Noise distribution type.
        #[serde(default)]
        noise_type: NoiseType,
    },
    /// Terminal-friendly chromatic aberration approximation.
    ChromaticEdge {
        /// Separation intensity.
        intensity: f32,
        /// Width of the edge effect in normalized space.
        edge_width: f32,
        /// Whether the effect runs horizontally.
        #[config(default = true)]
        horizontal: bool,
    },
    /// Micro-jitter distortion through color/channel oscillation.
    SubCellShake {
        /// Brightness-variation amplitude.
        #[config(default = 0.15)]
        amplitude: f32,
        /// Shake frequency.
        #[config(default = 12.0)]
        frequency: f32,
        /// Axis policy.
        #[serde(default)]
        axis: VfxEdgeDistortionAxis,
        /// Whether chromatic aberration is enabled.
        #[serde(default)]
        chromatic: bool,
        /// Deterministic seed.
        #[config(default = 42)]
        seed: u64,
        /// Whether distortion is edge-only.
        #[serde(default)]
        edge_only: bool,
        /// Edge width in cells when edge-only is enabled.
        #[config(default = 1)]
        edge_width: u8,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_edge_distortion_behavior.rs</FILE> - <DESC>V3 edge-distortion family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
