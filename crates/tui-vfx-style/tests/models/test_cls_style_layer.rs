// <FILE>tui-vfx-style/tests/models/test_cls_style_layer.rs</FILE> - <DESC>Integration tests for StyleLayer's grouped V3 effect-family seam</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 follow-on — verify that style layers can expose grouped V3 family identity across enter/dwell/exit effects without re-classifying the underlying legacy StyleEffect enum.</WCTX>
// <CLOG>0.1.0: add grouped V3 family coverage for fade, modulation, paired, and spatial style-layer effects.</CLOG>

use tui_vfx_style::models::{
    FadeApplyTo, StyleEffect, StyleLayer, StyleRegion, TryLowerV3StyleEffectError,
    VfxSpatialPrimitive, VfxSpatialShaderFamily, VfxStyleEffectFamily, VfxStyleEffectValue,
};

#[test]
fn style_layer_reports_v3_effect_families_in_phase_order() {
    let layer = StyleLayer::new(StyleRegion::All)
        .with_enter(
            StyleEffect::FadeIn {
                apply_to: FadeApplyTo::Foreground,
                ease: tui_vfx_geometry::types::EasingCurve::default(),
                from: tui_vfx_style::models::FadeTarget::Black,
            },
            None,
        )
        .with_dwell(StyleEffect::Rainbow { speed: 1.0 }, None)
        .with_exit(
            StyleEffect::RigidShakeStyle {
                shake_period: 0.25,
                num_shakes: 2,
                pause_duration: 0.5,
            },
            None,
        );

    assert_eq!(
        layer.v3_effect_families(),
        vec![
            VfxStyleEffectFamily::StyleFade,
            VfxStyleEffectFamily::StyleModulation,
            VfxStyleEffectFamily::PairedCapability,
        ]
    );
}

#[test]
fn style_layer_reports_spatial_family_through_grouped_effect_seam() {
    let layer = StyleLayer::new(StyleRegion::All).with_dwell(
        StyleEffect::Spatial {
            shader: tui_vfx_style::models::SpatialShaderType::Glow(
                tui_vfx_style::models::GlowShader::default(),
            ),
        },
        None,
    );

    assert_eq!(
        layer.v3_effect_families(),
        vec![VfxStyleEffectFamily::Spatial(
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(
                tui_vfx_style::models::v3::VfxSurfaceDepthShader::from_legacy_spatial_shader(
                    &tui_vfx_style::models::SpatialShaderType::Glow(
                        tui_vfx_style::models::GlowShader::default(),
                    ),
                )
                .expect("surface-depth family"),
            )),
        )]
    );
}

#[test]
fn style_layer_can_accept_grouped_v3_non_spatial_effects() {
    let effect =
        VfxStyleEffectValue::from_legacy_style_effect(&StyleEffect::Rainbow { speed: 1.0 });

    let layer = StyleLayer::new(StyleRegion::All)
        .try_with_v3_dwell(&effect, None)
        .expect("lowers");

    assert_eq!(
        layer.v3_effect_families(),
        vec![VfxStyleEffectFamily::StyleModulation]
    );
}

#[test]
fn style_layer_can_accept_grouped_v3_spatial_effects() {
    let effect = VfxStyleEffectValue::from_legacy_style_effect(&StyleEffect::Spatial {
        shader: tui_vfx_style::models::SpatialShaderType::Glow(
            tui_vfx_style::models::GlowShader::default(),
        ),
    });

    let layer = StyleLayer::new(StyleRegion::All)
        .try_with_v3_dwell(&effect, None)
        .expect("lowers");

    assert!(matches!(
        layer.dwell_v3_effect_family(),
        Some(VfxStyleEffectFamily::Spatial(
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(_))
        ))
    ));
}

#[test]
fn style_layer_rejects_mismatched_grouped_v3_effect_variants() {
    let effect = VfxStyleEffectValue::StyleFade(StyleEffect::Rainbow { speed: 1.0 });

    let error = StyleLayer::new(StyleRegion::All)
        .try_with_v3_dwell(&effect, None)
        .expect_err("mismatch should fail");

    assert!(matches!(
        error,
        TryLowerV3StyleEffectError::MismatchedVariant {
            expected_family: "style_fade",
            actual_effect: "Rainbow"
        }
    ));
}
// <FILE>tui-vfx-style/tests/models/test_cls_style_layer.rs</FILE> - <DESC>Integration tests for StyleLayer's grouped V3 effect-family seam</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
