// <FILE>tui-vfx-style/src/models/v3/test_vfx_guidance_cue_shader.rs</FILE> - <DESC>Focused tests for the V3 guidance-cue family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 guidance-cue surface regression-covered while the legacy flat variants remain operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxGuidanceCueShader into a dedicated sibling file.</CLOG>

use super::{
    VfxAffordanceWakeZone, VfxFocusFieldShape, VfxGuidanceCueApplyTo, VfxGuidanceCueBehavior,
    VfxGuidanceCueShader, VfxWayfindingNode,
};
use crate::models::ColorConfig;
use crate::models::{
    AffordanceWakeApplyTo, AffordanceWakeShader, AffordanceWakeZone, ApplyToColor,
    FocusFieldApplyTo, FocusFieldShader, FocusFieldShape, FocusedRowGradientShader,
    SpatialShaderType, WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader,
};

#[test]
fn converts_focused_row_gradient_into_v3_guidance_surface() {
    let legacy = FocusedRowGradientShader {
        selected_row: Some(4),
        selected_row_binding: Some("row".to_string()),
        selected_row_ratio: 0.6,
        selected_row_ratio_binding: Some("ratio".to_string()),
        falloff_distance: 7,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Both,
    };

    let converted = VfxGuidanceCueShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxGuidanceCueBehavior::FocusedRow {
            selected_row: Some(4),
            selected_row_binding: Some("row".to_string()),
            selected_row_ratio: 0.6,
            selected_row_ratio_binding: Some("ratio".to_string()),
            falloff_distance: 7,
            bright_color: ColorConfig::White,
            dim_color: ColorConfig::Black,
            apply_to: VfxGuidanceCueApplyTo::Both,
        }
    );
}

#[test]
fn converts_focus_field_into_v3_guidance_surface() {
    let legacy = FocusFieldShader {
        color: ColorConfig::Cyan,
        shape: FocusFieldShape::Rect,
        center_x: 10,
        center_y: 4,
        center_x_binding: Some("cx".to_string()),
        center_y_binding: Some("cy".to_string()),
        radius_x: 8,
        radius_y: 3,
        rect_x: 2,
        rect_y: 1,
        rect_width: 12,
        rect_height: 5,
        rect_x_binding: Some("rx".to_string()),
        rect_y_binding: None,
        rect_width_binding: Some("rw".to_string()),
        rect_height_binding: Some("rh".to_string()),
        feather: 2,
        falloff: crate::models::FalloffType::Quadratic,
        intensity: 0.4,
        apply_to: FocusFieldApplyTo::Both,
        pulse_speed: 1.1,
    };

    let converted = VfxGuidanceCueShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxGuidanceCueBehavior::FocusField {
            color: ColorConfig::Cyan,
            shape: VfxFocusFieldShape::Rect,
            center_x: 10,
            center_y: 4,
            center_x_binding: Some("cx".to_string()),
            center_y_binding: Some("cy".to_string()),
            radius_x: 8,
            radius_y: 3,
            rect_x: 2,
            rect_y: 1,
            rect_width: 12,
            rect_height: 5,
            rect_x_binding: Some("rx".to_string()),
            rect_y_binding: None,
            rect_width_binding: Some("rw".to_string()),
            rect_height_binding: Some("rh".to_string()),
            feather: 2,
            falloff: crate::models::FalloffType::Quadratic,
            intensity: 0.4,
            apply_to: VfxGuidanceCueApplyTo::Both,
            pulse_speed: 1.1,
        }
    );
}

#[test]
fn converts_affordance_wake_into_v3_guidance_surface() {
    let legacy = AffordanceWakeShader {
        color: ColorConfig::Cyan,
        zone: AffordanceWakeZone::Corners,
        radius: 3,
        falloff: crate::models::FalloffType::Quadratic,
        progress: 0.4,
        progress_binding: Some("progress".to_string()),
        rest_intensity: 0.05,
        peak_intensity: 0.33,
        apply_to: AffordanceWakeApplyTo::Background,
    };

    let converted = VfxGuidanceCueShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxGuidanceCueBehavior::AffordanceWake {
            color: ColorConfig::Cyan,
            zone: VfxAffordanceWakeZone::Corners,
            radius: 3,
            falloff: crate::models::FalloffType::Quadratic,
            progress: 0.4,
            progress_binding: Some("progress".to_string()),
            rest_intensity: 0.05,
            peak_intensity: 0.33,
            apply_to: VfxGuidanceCueApplyTo::Background,
        }
    );
}

#[test]
fn converts_wayfinding_node_into_v3_guidance_surface() {
    let legacy = WayfindingNodeShader {
        color: ColorConfig::Yellow,
        nodes: vec![WayfindingNode { x: 1, y: 2 }, WayfindingNode { x: 5, y: 2 }],
        radius: 2,
        intensity: 0.3,
        current_index: Some(1),
        current_index_binding: Some("node".to_string()),
        previous_strength: 0.5,
        future_strength: 0.1,
        pulse_speed: 1.0,
        apply_to: WayfindingNodeApplyTo::Both,
    };

    let converted = VfxGuidanceCueShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxGuidanceCueBehavior::WayfindingNode {
            color: ColorConfig::Yellow,
            nodes: vec![
                VfxWayfindingNode { x: 1, y: 2 },
                VfxWayfindingNode { x: 5, y: 2 }
            ],
            radius: 2,
            intensity: 0.3,
            current_index: Some(1),
            current_index_binding: Some("node".to_string()),
            previous_strength: 0.5,
            future_strength: 0.1,
            pulse_speed: 1.0,
            apply_to: VfxGuidanceCueApplyTo::Both,
        }
    );
}

#[test]
fn returns_none_for_non_guidance_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxGuidanceCueShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_guidance_cue_shader.rs</FILE> - <DESC>Focused tests for the V3 guidance-cue family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
