// <FILE>tui-vfx-style/src/models/v3/enum_vfx_traveling_band_behavior.rs</FILE> - <DESC>V3 traveling-band family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — introduce a parallel V3 family surface for traveling-band / sweep shaders while preserving the legacy flat variants for cutover playback and incremental runtime migration.</WCTX>
// <CLOG>Define the V3 traveling-band behavior, color, direction, and apply-to enums as the first concrete family-specific V3 model surface in tui-vfx-style.</CLOG>

//! V3 behavior surface for the traveling-band / sweep shader family.
//!
//! These enums intentionally live beside the legacy flat shader variants rather
//! than replacing them. The cutover strategy is to create real V3 family
//! surfaces now, leave the legacy V2-era variants operational for current
//! playback, and retire the old surface only once the wider migration is ready.

use crate::models::{cls_trace_common::TraceOrigin, cls_trace_common::TracePolyline, ColorConfig};
use serde::{Deserialize, Serialize};

/// Shared channel-target surface for V3 traveling-band shaders.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxTravelingBandApplyTo {
    /// Apply only to the foreground channel.
    #[default]
    Foreground,
    /// Apply only to the background channel.
    Background,
    /// Apply to both foreground and background.
    Both,
}

/// Shared directional policy for sweep-style motion.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxTravelingBandDirection {
    /// Move from start to end.
    #[default]
    Forward,
    /// Move from end to start.
    Reverse,
    /// Oscillate back and forth.
    PingPong,
}

/// How a path-authored traveling band should treat its tail.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxTracePathTailMode {
    /// Tail spans the whole authored path.
    #[default]
    Path,
    /// Tail primarily hugs the active segment.
    Segment,
}

/// Color policy for a traveling-band family shader.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxTravelingBandColor {
    /// One solid accent color throughout the traveling band.
    Solid {
        /// Accent color used by the band.
        color: ColorConfig,
    },
    /// Brighter head with a softer trailing tail.
    HeadTail {
        /// Color at the leading edge.
        head: ColorConfig,
        /// Color used behind the leading edge.
        tail: ColorConfig,
    },
}

/// Route/behavior policy for the traveling-band family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxTravelingBandBehavior {
    /// Border-following perimeter sweep.
    Border {
        /// Length of the illuminated border segment.
        #[config(default = 5)]
        length: u16,
        /// Optional runtime binding overriding the perimeter position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        position_binding: Option<String>,
    },
    /// Simple horizontal glint with a configurable gap.
    Reflect {
        /// Horizontal gap appended to the active width when looping.
        #[config(default = 20.0)]
        gap: f32,
        /// Effective glint width in cells.
        #[config(default = 2.0)]
        width: f32,
    },
    /// Angled glisten band with direction and blend controls.
    GlistenBand {
        /// Width of the illuminated band in cells.
        #[config(default = 6)]
        band_width: u16,
        /// Band angle in degrees.
        #[config(default = 25.0)]
        angle_deg: f32,
        /// Sweep direction policy.
        #[serde(default)]
        direction: VfxTravelingBandDirection,
        /// Optional runtime binding overriding the direction code.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        direction_binding: Option<String>,
        /// How many times to repeat; `0` means continuous.
        #[serde(default)]
        repeat_count: u8,
        /// Where to apply the band.
        #[serde(default)]
        apply_to: VfxTravelingBandApplyTo,
        /// Strength of the blend into the base style.
        #[config(default = 0.7)]
        blend_strength: f32,
        /// Optional runtime binding overriding blend strength.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blend_strength_binding: Option<String>,
        /// Optional runtime binding overriding speed.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        speed_binding: Option<String>,
    },
    /// Orthogonal routed propagation over inferred lanes.
    TracePropagation {
        /// Distance between inferred lanes.
        #[config(default = 6)]
        grid_spacing: u16,
        /// Width of each lane in cells.
        #[config(default = 1)]
        line_width: u16,
        /// Tail length behind the moving head.
        #[config(default = 5.0)]
        tail_length: f32,
        /// Blend strength.
        #[config(default = 0.8)]
        intensity: f32,
        /// Where propagation begins.
        #[serde(default)]
        origin: TraceOrigin,
        /// Which channel(s) to affect.
        #[serde(default)]
        apply_to: VfxTravelingBandApplyTo,
    },
    /// Explicit path-authored traveling band.
    TracePath {
        /// Tail length behind the moving head.
        #[config(default = 7.0)]
        tail_length: f32,
        /// Perceptual weighting for vertical travel.
        #[config(default = 1.0)]
        vertical_weight: f32,
        /// Maximum route thickness.
        #[config(default = 1)]
        thickness: u16,
        /// Blend strength.
        #[config(default = 0.85)]
        intensity: f32,
        /// Extra emphasis at junctions.
        #[config(default = 0.2)]
        junction_boost: f32,
        /// Local glow boost at the turn cell itself.
        #[config(default = 0.15)]
        junction_glow: f32,
        /// Tail policy along the path.
        #[serde(default)]
        tail_mode: VfxTracePathTailMode,
        /// Which channel(s) to affect.
        #[serde(default)]
        apply_to: VfxTravelingBandApplyTo,
        /// One or more authored paths.
        paths: Vec<TracePolyline>,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_traveling_band_behavior.rs</FILE> - <DESC>V3 traveling-band family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
