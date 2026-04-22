// <FILE>tui-vfx-style/src/models/v3/test_vfx_guidance_cue_shader.rs</FILE> - <DESC>Focused tests for the V3 guidance-cue family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 guidance-cue surface regression-covered while the legacy flat variants remain operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxGuidanceCueShader into a dedicated sibling file.</CLOG>

use super::{
    VfxAffordanceWakeZone, VfxGuidanceCueApplyTo, VfxGuidanceCueBehavior, VfxGuidanceCueShader,
    VfxWayfindingNode,
};
use crate::models::ColorConfig;
use crate::models::{
    AffordanceWakeApplyTo, AffordanceWakeShader, AffordanceWakeZone, ApplyToColor,
    FocusedRowGradientShader, SpatialShaderType, WayfindingNode, WayfindingNodeApplyTo,
    WayfindingNodeShader,
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
