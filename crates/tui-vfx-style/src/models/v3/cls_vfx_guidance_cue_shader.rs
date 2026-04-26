// <FILE>tui-vfx-style/src/models/v3/cls_vfx_guidance_cue_shader.rs</FILE> - <DESC>V3 guidance-cue family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a real grouped V3 guidance-cue surface so FocusedRowGradient, AffordanceWake, and WayfindingNode become migration inputs instead of the lasting conceptual model.</WCTX>
// <CLOG>Introduce VfxGuidanceCueShader plus conversion helpers from the legacy guidance-cue variants and SpatialShaderType.</CLOG>

//! V3 family surface for guidance-cue shaders.
//!
//! This grouped type provides a forward-looking V3 home for the calm UX
//! guidance cues that currently live as separate flat variants.

use crate::models::v3::enum_vfx_guidance_cue_behavior::{
    VfxAffordanceWakeZone, VfxFocusFieldShape, VfxGuidanceCueApplyTo, VfxGuidanceCueBehavior,
    VfxWayfindingNode,
};
use crate::models::{
    AffordanceWakeApplyTo, AffordanceWakeShader, AffordanceWakeZone, ApplyToColor,
    FocusFieldApplyTo, FocusFieldShader, FocusFieldShape, FocusedRowGradientShader,
    SpatialShaderType, WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for guidance-cue shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxGuidanceCueShader {
    /// Behavior/configuration surface for the chosen guidance-cue family member.
    pub behavior: VfxGuidanceCueBehavior,
}

impl VfxGuidanceCueShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// guidance-cue family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::FocusedRowGradient(shader) => Some(Self::from(shader)),
            SpatialShaderType::FocusField(shader) => Some(Self::from(shader)),
            SpatialShaderType::AffordanceWake(shader) => Some(Self::from(shader)),
            SpatialShaderType::WayfindingNode(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&FocusedRowGradientShader> for VfxGuidanceCueShader {
    fn from(shader: &FocusedRowGradientShader) -> Self {
        Self {
            behavior: VfxGuidanceCueBehavior::FocusedRow {
                selected_row: shader.selected_row,
                selected_row_binding: shader.selected_row_binding.clone(),
                selected_row_ratio: shader.selected_row_ratio,
                selected_row_ratio_binding: shader.selected_row_ratio_binding.clone(),
                falloff_distance: shader.falloff_distance,
                bright_color: shader.bright_color,
                dim_color: shader.dim_color,
                apply_to: shader.apply_to.into(),
            },
        }
    }
}

impl From<&FocusFieldShader> for VfxGuidanceCueShader {
    fn from(shader: &FocusFieldShader) -> Self {
        Self {
            behavior: VfxGuidanceCueBehavior::FocusField {
                color: shader.color,
                shape: shader.shape.into(),
                center_x: shader.center_x,
                center_y: shader.center_y,
                center_x_binding: shader.center_x_binding.clone(),
                center_y_binding: shader.center_y_binding.clone(),
                radius_x: shader.radius_x,
                radius_y: shader.radius_y,
                rect_x: shader.rect_x,
                rect_y: shader.rect_y,
                rect_width: shader.rect_width,
                rect_height: shader.rect_height,
                rect_x_binding: shader.rect_x_binding.clone(),
                rect_y_binding: shader.rect_y_binding.clone(),
                rect_width_binding: shader.rect_width_binding.clone(),
                rect_height_binding: shader.rect_height_binding.clone(),
                feather: shader.feather,
                falloff: shader.falloff,
                intensity: shader.intensity,
                apply_to: shader.apply_to.into(),
                pulse_speed: shader.pulse_speed,
            },
        }
    }
}

impl From<&AffordanceWakeShader> for VfxGuidanceCueShader {
    fn from(shader: &AffordanceWakeShader) -> Self {
        Self {
            behavior: VfxGuidanceCueBehavior::AffordanceWake {
                color: shader.color,
                zone: shader.zone.into(),
                radius: shader.radius,
                falloff: shader.falloff,
                progress: shader.progress,
                progress_binding: shader.progress_binding.clone(),
                rest_intensity: shader.rest_intensity,
                peak_intensity: shader.peak_intensity,
                apply_to: shader.apply_to.into(),
            },
        }
    }
}

impl From<&WayfindingNodeShader> for VfxGuidanceCueShader {
    fn from(shader: &WayfindingNodeShader) -> Self {
        Self {
            behavior: VfxGuidanceCueBehavior::WayfindingNode {
                color: shader.color,
                nodes: shader.nodes.iter().copied().map(Into::into).collect(),
                radius: shader.radius,
                intensity: shader.intensity,
                current_index: shader.current_index,
                current_index_binding: shader.current_index_binding.clone(),
                previous_strength: shader.previous_strength,
                future_strength: shader.future_strength,
                pulse_speed: shader.pulse_speed,
                apply_to: shader.apply_to.into(),
            },
        }
    }
}

impl From<ApplyToColor> for VfxGuidanceCueApplyTo {
    fn from(value: ApplyToColor) -> Self {
        match value {
            ApplyToColor::Foreground => Self::Foreground,
            ApplyToColor::Background => Self::Background,
            ApplyToColor::Both => Self::Both,
        }
    }
}

impl From<FocusFieldShape> for VfxFocusFieldShape {
    fn from(value: FocusFieldShape) -> Self {
        match value {
            FocusFieldShape::Ellipse => Self::Ellipse,
            FocusFieldShape::Rect => Self::Rect,
        }
    }
}

impl From<FocusFieldApplyTo> for VfxGuidanceCueApplyTo {
    fn from(value: FocusFieldApplyTo) -> Self {
        match value {
            FocusFieldApplyTo::Foreground => Self::Foreground,
            FocusFieldApplyTo::Background => Self::Background,
            FocusFieldApplyTo::Both => Self::Both,
        }
    }
}

impl From<AffordanceWakeZone> for VfxAffordanceWakeZone {
    fn from(value: AffordanceWakeZone) -> Self {
        match value {
            AffordanceWakeZone::AllEdges => Self::AllEdges,
            AffordanceWakeZone::Corners => Self::Corners,
            AffordanceWakeZone::LeftRail => Self::LeftRail,
            AffordanceWakeZone::RightRail => Self::RightRail,
            AffordanceWakeZone::TopRail => Self::TopRail,
            AffordanceWakeZone::BottomRail => Self::BottomRail,
        }
    }
}

impl From<AffordanceWakeApplyTo> for VfxGuidanceCueApplyTo {
    fn from(value: AffordanceWakeApplyTo) -> Self {
        match value {
            AffordanceWakeApplyTo::Foreground => Self::Foreground,
            AffordanceWakeApplyTo::Background => Self::Background,
            AffordanceWakeApplyTo::Both => Self::Both,
        }
    }
}

impl From<WayfindingNodeApplyTo> for VfxGuidanceCueApplyTo {
    fn from(value: WayfindingNodeApplyTo) -> Self {
        match value {
            WayfindingNodeApplyTo::Foreground => Self::Foreground,
            WayfindingNodeApplyTo::Background => Self::Background,
            WayfindingNodeApplyTo::Both => Self::Both,
        }
    }
}

impl From<WayfindingNode> for VfxWayfindingNode {
    fn from(value: WayfindingNode) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_guidance_cue_shader.rs</FILE> - <DESC>V3 guidance-cue family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
