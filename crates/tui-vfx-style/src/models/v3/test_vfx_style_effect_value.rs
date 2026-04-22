// <FILE>tui-vfx-style/src/models/v3/test_vfx_style_effect_value.rs</FILE> - <DESC>Focused tests for grouped V3 overall style-effect values lowering back into the legacy runtime surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 runtime follow-on — prove grouped V3 overall effect values can round-trip back into StyleEffect for representative non-spatial and spatial families.</WCTX>
// <CLOG>0.1.0: add grouped overall style-effect roundtrip coverage plus one mismatched-variant failure case.</CLOG>

use crate::models::{StyleEffect, VfxStyleEffectValue};
use crate::models::v3::{TryLowerV3StyleEffectError, VfxStyleEffectFamily};

#[test]
fn roundtrips_style_fade_value_back_to_legacy_effect() {
    let legacy = StyleEffect::FadeIn {
        apply_to: crate::models::FadeApplyTo::Foreground,
        ease: tui_vfx_geometry::types::EasingCurve::default(),
        from: crate::models::FadeTarget::Black,
    };
    let grouped = VfxStyleEffectValue::from_legacy_style_effect(&legacy);

    assert_eq!(grouped.try_to_legacy_style_effect().unwrap(), legacy);
    assert_eq!(grouped.family(), VfxStyleEffectFamily::StyleFade);
}

#[test]
fn roundtrips_style_modulation_value_back_to_legacy_effect() {
    let legacy = StyleEffect::Rainbow { speed: 1.0 };
    let grouped = VfxStyleEffectValue::from_legacy_style_effect(&legacy);

    assert_eq!(grouped.try_to_legacy_style_effect().unwrap(), legacy);
    assert_eq!(grouped.family(), VfxStyleEffectFamily::StyleModulation);
}

#[test]
fn roundtrips_spatial_value_back_to_legacy_effect() {
    let legacy = StyleEffect::Spatial {
        shader: crate::models::SpatialShaderType::Glow(crate::models::GlowShader::default()),
    };
    let grouped = VfxStyleEffectValue::from_legacy_style_effect(&legacy);

    assert_eq!(grouped.try_to_legacy_style_effect().unwrap(), legacy);
}

#[test]
fn rejects_mismatched_non_spatial_grouped_value() {
    let grouped = VfxStyleEffectValue::StyleFade(StyleEffect::Rainbow { speed: 1.0 });

    assert!(matches!(
        grouped.try_to_legacy_style_effect(),
        Err(TryLowerV3StyleEffectError::MismatchedVariant { expected_family: "style_fade", actual_effect: "Rainbow" })
    ));
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_style_effect_value.rs</FILE> - <DESC>Focused tests for grouped V3 overall style-effect values lowering back into the legacy runtime surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
