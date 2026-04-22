// <FILE>tui-vfx-style/src/models/v3/test_vfx_traveling_band_shader.rs</FILE> - <DESC>Focused tests for the V3 traveling-band family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the first real V3 family surface regression-covered while the legacy flat shader surface remains operational during cutover.</WCTX>
// <CLOG>Extract the VfxTravelingBandShader conversion tests into a dedicated sibling file to keep the production family surface OFPF-compliant.</CLOG>

use super::{
    VfxTravelingBandApplyTo, VfxTravelingBandBehavior, VfxTravelingBandColor,
    VfxTravelingBandDirection, VfxTravelingBandShader,
};
use crate::models::{
    BorderSweepShader, ColorConfig, GlistenBandShader, HighlighterShader, SpatialShaderType,
};

#[test]
fn converts_border_sweep_into_v3_family_surface() {
    let legacy = BorderSweepShader {
        speed: 1.5,
        length: 7,
        color: ColorConfig::Cyan,
        position_binding: Some("progress".to_string()),
    };

    let converted = VfxTravelingBandShader::from(&legacy);
    assert_eq!(converted.speed, 1.5);
    assert_eq!(
        converted.color,
        VfxTravelingBandColor::Solid {
            color: ColorConfig::Cyan,
        }
    );
    assert_eq!(
        converted.behavior,
        VfxTravelingBandBehavior::Border {
            length: 7,
            position_binding: Some("progress".to_string()),
        }
    );
}

#[test]
fn converts_glisten_band_into_v3_family_surface() {
    let legacy = GlistenBandShader::default();
    let converted = VfxTravelingBandShader::from(&legacy);

    assert_eq!(converted.speed, legacy.speed);
    assert_eq!(
        converted.color,
        VfxTravelingBandColor::HeadTail {
            head: legacy.head.clone(),
            tail: legacy.tail.clone(),
        }
    );
    assert!(matches!(
        converted.behavior,
        VfxTravelingBandBehavior::GlistenBand {
            band_width: 6,
            angle_deg,
            direction: VfxTravelingBandDirection::Forward,
            direction_binding: None,
            repeat_count: 0,
            apply_to: VfxTravelingBandApplyTo::Foreground,
            blend_strength,
            blend_strength_binding: None,
            speed_binding: None,
        } if (angle_deg - 25.0).abs() < f32::EPSILON && (blend_strength - 0.7).abs() < f32::EPSILON
    ));
}

#[test]
fn returns_none_for_non_traveling_band_legacy_variant() {
    let shader = SpatialShaderType::Highlighter(HighlighterShader::default());
    assert!(VfxTravelingBandShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_traveling_band_shader.rs</FILE> - <DESC>Focused tests for the V3 traveling-band family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
