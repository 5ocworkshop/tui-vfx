// <FILE>tui-vfx-style/src/models/v3/cls_vfx_traveling_band_shader.rs</FILE> - <DESC>V3 traveling-band family shader surface</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Decision 2 migration slice — create a real V3 traveling-band family surface now, grouped by family while keeping the legacy flat shader variants intact for current playback and staged cutover.</WCTX>
// <CLOG>Lift legacy border, reflect, trace_propagation, and trace_path head/tail fields into the V3 traveling-band head_tail color policy losslessly.</CLOG>

//! V3 family surface for traveling-band / sweep shaders.
//!
//! This is the first concrete V3-side family model in `tui-vfx-style`.
//! Instead of only tagging the legacy flat variants for later, the V3 family
//! now has a real parallel type that can absorb future schema/runtime work while
//! the legacy playback path continues to use the existing per-variant structs.

use crate::models::v3::enum_vfx_traveling_band_behavior::{
    VfxTracePathTailMode, VfxTravelingBandApplyTo, VfxTravelingBandBehavior, VfxTravelingBandColor,
    VfxTravelingBandDirection,
};
use crate::models::{
    BorderSweepShader, GlistenApplyTo, GlistenBandShader, GlistenDirection, ReflectShader,
    SpatialShaderType, TraceApplyTo, TracePathShader, TracePropagationShader,
    cls_trace_path_shader::TraceTailMode,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for traveling-band / sweep shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxTravelingBandShader {
    /// Shared speed multiplier for the traveling-band family.
    #[config(default = 1.0)]
    pub speed: f32,
    /// Color policy for the family.
    pub color: VfxTravelingBandColor,
    /// Route/behavior policy for the family.
    pub behavior: VfxTravelingBandBehavior,
}

impl VfxTravelingBandShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// traveling-band family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::BorderSweep(shader) => Some(Self::from(shader)),
            SpatialShaderType::Reflect(shader) => Some(Self::from(shader)),
            SpatialShaderType::GlistenBand(shader) => Some(Self::from(shader)),
            SpatialShaderType::TracePropagation(shader) => Some(Self::from(shader)),
            SpatialShaderType::TracePath(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&BorderSweepShader> for VfxTravelingBandShader {
    fn from(shader: &BorderSweepShader) -> Self {
        Self {
            speed: shader.speed,
            color: traveling_band_color_from_legacy(
                &shader.color,
                shader.head.as_ref(),
                shader.tail.as_ref(),
            ),
            behavior: VfxTravelingBandBehavior::Border {
                length: shader.length,
                position_binding: shader.position_binding.clone(),
            },
        }
    }
}

impl From<&ReflectShader> for VfxTravelingBandShader {
    fn from(shader: &ReflectShader) -> Self {
        Self {
            speed: shader.speed,
            color: traveling_band_color_from_legacy(
                &shader.color,
                shader.head.as_ref(),
                shader.tail.as_ref(),
            ),
            behavior: VfxTravelingBandBehavior::Reflect {
                gap: shader.gap,
                width: shader.width,
            },
        }
    }
}

impl From<&GlistenBandShader> for VfxTravelingBandShader {
    fn from(shader: &GlistenBandShader) -> Self {
        Self {
            speed: shader.speed,
            color: VfxTravelingBandColor::HeadTail {
                head: shader.head,
                tail: shader.tail,
            },
            behavior: VfxTravelingBandBehavior::GlistenBand {
                band_width: shader.band_width,
                angle_deg: shader.angle_deg,
                direction: shader.direction.into(),
                direction_binding: shader.direction_binding.clone(),
                repeat_count: shader.repeat_count,
                apply_to: shader.apply_to.into(),
                blend_strength: shader.blend_strength,
                blend_strength_binding: shader.blend_strength_binding.clone(),
                speed_binding: shader.speed_binding.clone(),
            },
        }
    }
}

impl From<&TracePropagationShader> for VfxTravelingBandShader {
    fn from(shader: &TracePropagationShader) -> Self {
        Self {
            speed: shader.speed,
            color: traveling_band_color_from_legacy(
                &shader.color,
                shader.head.as_ref(),
                shader.tail.as_ref(),
            ),
            behavior: VfxTravelingBandBehavior::TracePropagation {
                grid_spacing: shader.grid_spacing,
                line_width: shader.line_width,
                tail_length: shader.tail_length,
                intensity: shader.intensity,
                origin: shader.origin,
                apply_to: shader.apply_to.into(),
            },
        }
    }
}

impl From<&TracePathShader> for VfxTravelingBandShader {
    fn from(shader: &TracePathShader) -> Self {
        Self {
            speed: shader.speed,
            color: traveling_band_color_from_legacy(
                &shader.color,
                shader.head.as_ref(),
                shader.tail.as_ref(),
            ),
            behavior: VfxTravelingBandBehavior::TracePath {
                tail_length: shader.tail_length,
                vertical_weight: shader.vertical_weight,
                thickness: shader.thickness,
                intensity: shader.intensity,
                junction_boost: shader.junction_boost,
                junction_glow: shader.junction_glow,
                tail_mode: shader.tail_mode.into(),
                apply_to: shader.apply_to.into(),
                paths: shader.paths.clone(),
            },
        }
    }
}

fn traveling_band_color_from_legacy(
    color: &crate::models::ColorConfig,
    head: Option<&crate::models::ColorConfig>,
    tail: Option<&crate::models::ColorConfig>,
) -> VfxTravelingBandColor {
    match (head, tail) {
        (None, None) => VfxTravelingBandColor::Solid { color: *color },
        _ => VfxTravelingBandColor::HeadTail {
            head: *head.unwrap_or(color),
            tail: *tail.unwrap_or(color),
        },
    }
}

impl From<GlistenDirection> for VfxTravelingBandDirection {
    fn from(direction: GlistenDirection) -> Self {
        match direction {
            GlistenDirection::Forward => Self::Forward,
            GlistenDirection::Reverse => Self::Reverse,
            GlistenDirection::PingPong => Self::PingPong,
        }
    }
}

impl From<GlistenApplyTo> for VfxTravelingBandApplyTo {
    fn from(apply_to: GlistenApplyTo) -> Self {
        match apply_to {
            GlistenApplyTo::Foreground => Self::Foreground,
            GlistenApplyTo::Background => Self::Background,
            GlistenApplyTo::Both => Self::Both,
        }
    }
}

impl From<TraceApplyTo> for VfxTravelingBandApplyTo {
    fn from(apply_to: TraceApplyTo) -> Self {
        match apply_to {
            TraceApplyTo::Foreground => Self::Foreground,
            TraceApplyTo::Background => Self::Background,
            TraceApplyTo::Both => Self::Both,
        }
    }
}

impl From<TraceTailMode> for VfxTracePathTailMode {
    fn from(mode: TraceTailMode) -> Self {
        match mode {
            TraceTailMode::Path => Self::Path,
            TraceTailMode::Segment => Self::Segment,
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_traveling_band_shader.rs</FILE> - <DESC>V3 traveling-band family shader surface</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
