// <FILE>tui-vfx-style/src/models/v3/cls_vfx_progress_emphasis_shader.rs</FILE> - <DESC>V3 progress/emphasis family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a real V3 progress/emphasis family surface so the legacy Highlighter shader becomes one migration source, not the only lasting conceptual model.</WCTX>
// <CLOG>Introduce VfxProgressEmphasisShader with legacy conversion helpers from HighlighterShader and SpatialShaderType.</CLOG>

//! V3 family surface for progress/emphasis shaders.
//!
//! The current conversion source is `HighlighterShader`, but this grouped type
//! gives the V3 migration a place to absorb related progress/emphasis effects
//! without keeping the legacy effect name as the permanent conceptual root.

use crate::models::ColorConfig;
use crate::models::v3::enum_vfx_progress_emphasis_behavior::{
    VfxProgressEmphasisApplyTo, VfxProgressEmphasisDirection, VfxProgressEmphasisMode,
    VfxProgressEmphasisRowMask, VfxProgressEmphasisTextContrast,
};
use crate::models::{
    HighlighterApplyTo, HighlighterDirection, HighlighterMode, HighlighterRowMask,
    HighlighterShader, SpatialShaderType, TextContrast,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for progress/emphasis shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxProgressEmphasisShader {
    /// Emphasis ink color.
    pub color: ColorConfig,
    /// Channel-target policy.
    #[serde(default)]
    pub apply_to: VfxProgressEmphasisApplyTo,
    /// Foreground handling policy.
    #[serde(default)]
    pub text_contrast: VfxProgressEmphasisTextContrast,
    /// Coverage shape policy.
    #[serde(default)]
    pub mode: VfxProgressEmphasisMode,
    /// Width of the moving band in cells when `mode = band`.
    #[config(default = 6)]
    pub band_width: u16,
    /// Softness of the edge/falloff.
    #[config(default = 0.0)]
    pub soft_edge: f32,
    /// Blend strength.
    #[config(default = 1.0)]
    pub blend_strength: f32,
    /// Optional runtime binding overriding blend strength.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blend_strength_binding: Option<String>,
    /// Speed multiplier.
    #[config(default = 1.0)]
    pub speed: f32,
    /// Optional runtime binding overriding speed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed_binding: Option<String>,
    /// Motion direction.
    #[serde(default)]
    pub direction: VfxProgressEmphasisDirection,
    /// Optional runtime binding overriding direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction_binding: Option<String>,
    /// Row-selection policy.
    #[serde(default)]
    pub row_mask: VfxProgressEmphasisRowMask,
}

impl VfxProgressEmphasisShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// progress/emphasis family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::Highlighter(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&HighlighterShader> for VfxProgressEmphasisShader {
    fn from(shader: &HighlighterShader) -> Self {
        Self {
            color: shader.color,
            apply_to: shader.apply_to.into(),
            text_contrast: shader.text_contrast.clone().into(),
            mode: shader.mode.into(),
            band_width: shader.band_width,
            soft_edge: shader.soft_edge,
            blend_strength: shader.blend_strength,
            blend_strength_binding: shader.blend_strength_binding.clone(),
            speed: shader.speed,
            speed_binding: shader.speed_binding.clone(),
            direction: shader.direction.into(),
            direction_binding: shader.direction_binding.clone(),
            row_mask: shader.row_mask.into(),
        }
    }
}

impl From<HighlighterApplyTo> for VfxProgressEmphasisApplyTo {
    fn from(value: HighlighterApplyTo) -> Self {
        match value {
            HighlighterApplyTo::Background => Self::Background,
            HighlighterApplyTo::Foreground => Self::Foreground,
            HighlighterApplyTo::Both => Self::Both,
        }
    }
}

impl From<TextContrast> for VfxProgressEmphasisTextContrast {
    fn from(value: TextContrast) -> Self {
        match value {
            TextContrast::Black => Self::Black,
            TextContrast::Preserve => Self::Preserve,
            TextContrast::Explicit { color } => Self::Explicit { color },
        }
    }
}

impl From<HighlighterMode> for VfxProgressEmphasisMode {
    fn from(value: HighlighterMode) -> Self {
        match value {
            HighlighterMode::Fill => Self::Fill,
            HighlighterMode::Band => Self::Band,
        }
    }
}

impl From<HighlighterDirection> for VfxProgressEmphasisDirection {
    fn from(value: HighlighterDirection) -> Self {
        match value {
            HighlighterDirection::Forward => Self::Forward,
            HighlighterDirection::Reverse => Self::Reverse,
            HighlighterDirection::TopDown => Self::TopDown,
            HighlighterDirection::BottomUp => Self::BottomUp,
            HighlighterDirection::CenterOut => Self::CenterOut,
            HighlighterDirection::EdgesIn => Self::EdgesIn,
        }
    }
}

impl From<HighlighterRowMask> for VfxProgressEmphasisRowMask {
    fn from(value: HighlighterRowMask) -> Self {
        match value {
            HighlighterRowMask::AllRows => Self::AllRows,
            HighlighterRowMask::FirstRow => Self::FirstRow,
            HighlighterRowMask::LastRow => Self::LastRow,
            HighlighterRowMask::TopAndBottom => Self::TopAndBottom,
            HighlighterRowMask::Range { start, end } => Self::Range { start, end },
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_progress_emphasis_shader.rs</FILE> - <DESC>V3 progress/emphasis family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
