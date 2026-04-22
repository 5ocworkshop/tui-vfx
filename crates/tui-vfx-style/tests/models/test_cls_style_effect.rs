// <FILE>tui-vfx-style/tests/models/test_cls_style_effect.rs</FILE> - <DESC>Integration tests for StyleEffect</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>L2/L3 abstraction: tui-style-fx uses mixed-types</WCTX>
// <CLOG>Updated rainbow test for bright Color::RED behavior</CLOG>

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_style::models::{
    FadeApplyTo, StyleEffect, VfxSpatialComposedPrimitive, VfxSpatialPrimitive,
    VfxSpatialShaderFamily, VfxStyleEffectFamily,
};
use tui_vfx_style::traits::StyleInterpolator;
use tui_vfx_types::{Color, Style};
#[test]
fn test_rainbow_hue_shift() {
    let base = Style::fg(Color::RED);
    let effect = StyleEffect::Rainbow { speed: 0.5 };
    let result = effect.calculate(1.0, base);
    // Color::RED is bright red (255,0,0), so rainbow shift produces bright cyan
    // Allow slight rounding variance in HSL conversion
    assert!(result.fg.r == 0);
    assert!(result.fg.g >= 254); // Allow for rounding
    assert!(result.fg.b >= 254);
}
#[test]
fn test_fade_out() {
    let base = Style::fg(Color::WHITE);
    let effect = StyleEffect::FadeOut {
        apply_to: FadeApplyTo::Foreground,
        ease: EasingCurve::Type(EasingType::Linear),
        to: tui_vfx_style::models::FadeTarget::Black,
    };
    let result = effect.calculate(0.5, base);
    assert_eq!(result.fg, Color::rgb(127, 127, 127));
}
#[test]
fn test_pulse() {
    let base = Style::fg(Color::RED);
    let target = Color::BLUE;
    let effect = StyleEffect::Pulse {
        frequency: 1.0,
        color: target,
    };
    let result = effect.calculate(0.25, base);
    assert_eq!(result.fg, Color::BLUE);
    let result_trough = effect.calculate(0.75, base);
    assert_eq!(result_trough.fg, Color::RED);
}
#[test]
fn test_glitch_deterministic() {
    let base = Style::default();
    let effect = StyleEffect::Glitch {
        seed: 12345,
        intensity: 1.0,
        italic_start: None,
        italic_end: None,
    };
    // With intensity 1.0, it should always apply a modifier
    let result1 = effect.calculate(0.1, base);
    let result2 = effect.calculate(0.1, base);
    // Determinism check
    assert_eq!(result1, result2);
    // Should have some modifier (4 choices: BOLD, UNDERLINED, ITALIC, REVERSED)
    let mods = result1.mods;
    assert!(mods.bold || mods.underline || mods.italic || mods.reverse);
}
#[test]
fn test_glitch_zero_intensity() {
    let base = Style::default();
    let effect = StyleEffect::Glitch {
        seed: 12345,
        intensity: 0.0,
        italic_start: None,
        italic_end: None,
    };
    let result = effect.calculate(0.5, base);
    assert_eq!(result, base);
}

#[test]
fn test_spatial_effect_exposes_v3_family() {
    let effect = StyleEffect::Spatial {
        shader: tui_vfx_style::models::SpatialShaderType::Highlighter(
            tui_vfx_style::models::HighlighterShader::default(),
        ),
    };

    assert!(matches!(
        effect.spatial_shader_family(),
        Some(VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::ProgressEmphasis(_)
        ))
    ));
}

#[test]
fn test_non_spatial_effect_has_no_v3_family() {
    let effect = StyleEffect::Rainbow { speed: 1.0 };
    assert_eq!(effect.spatial_shader_family(), None);
}

#[test]
fn test_spatial_effect_can_lower_to_primitive_family() {
    let effect = StyleEffect::Spatial {
        shader: tui_vfx_style::models::SpatialShaderType::Glow(
            tui_vfx_style::models::GlowShader::default(),
        ),
    };

    assert!(matches!(
        effect.spatial_shader_family(),
        Some(VfxSpatialShaderFamily::Primitive(
            VfxSpatialPrimitive::SurfaceDepth(_)
        ))
    ));
}

#[test]
fn test_fade_effect_exposes_v3_effect_family() {
    let effect = StyleEffect::FadeIn {
        apply_to: FadeApplyTo::Foreground,
        ease: EasingCurve::Type(EasingType::Linear),
        from: tui_vfx_style::models::FadeTarget::Black,
    };

    assert_eq!(effect.v3_effect_family(), VfxStyleEffectFamily::StyleFade);
}

#[test]
fn test_style_modulation_effect_exposes_v3_effect_family() {
    let effect = StyleEffect::Rainbow { speed: 1.0 };
    assert_eq!(effect.v3_effect_family(), VfxStyleEffectFamily::StyleModulation);
}

#[test]
fn test_spatial_effect_exposes_v3_effect_family() {
    let effect = StyleEffect::Spatial {
        shader: tui_vfx_style::models::SpatialShaderType::Glow(
            tui_vfx_style::models::GlowShader::default(),
        ),
    };

    assert!(matches!(
        effect.v3_effect_family(),
        VfxStyleEffectFamily::Spatial(VfxSpatialShaderFamily::Primitive(
            VfxSpatialPrimitive::SurfaceDepth(_)
        ))
    ));
}

#[test]
fn test_paired_style_effect_exposes_v3_effect_family() {
    let effect = StyleEffect::RigidShakeStyle {
        shake_period: 0.25,
        num_shakes: 2,
        pause_duration: 0.5,
    };

    assert_eq!(effect.v3_effect_family(), VfxStyleEffectFamily::PairedCapability);
}

// <FILE>tui-vfx-style/tests/models/test_cls_style_effect.rs</FILE> - <DESC>Integration tests for StyleEffect</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
