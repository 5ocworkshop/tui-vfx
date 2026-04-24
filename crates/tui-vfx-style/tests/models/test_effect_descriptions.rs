// <FILE>tui-vfx-style/tests/models/test_effect_descriptions.rs</FILE> - <DESC>Tests for effect documentation methods</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Cover Orbit shader in spatial effect documentation tests</WCTX>
// <CLOG>Add Orbit shader to name/description coverage</CLOG>

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_style::models::ColorConfig;
use tui_vfx_style::models::Gradient;
use tui_vfx_style::models::cls_affordance_wake_shader::AffordanceWakeShader;
use tui_vfx_style::models::cls_barber_pole_shader::BarberPoleShader;
use tui_vfx_style::models::cls_border_sweep_shader::BorderSweepShader;
use tui_vfx_style::models::cls_chromatic_edge_shader::ChromaticEdgeShader;
use tui_vfx_style::models::cls_concealed_light_shader::ConcealedLightShader;
use tui_vfx_style::models::cls_diffusion_shader::DiffusionShader;
use tui_vfx_style::models::cls_edge_sheen_shader::EdgeSheenShader;
use tui_vfx_style::models::cls_focus_field_shader::FocusFieldShader;
use tui_vfx_style::models::cls_focused_row_gradient_shader::FocusedRowGradientShader;
use tui_vfx_style::models::cls_glisten_band_shader::GlistenBandShader;
use tui_vfx_style::models::cls_glitch_lines_shader::GlitchLinesShader;
use tui_vfx_style::models::cls_highlighter_shader::HighlighterShader;
use tui_vfx_style::models::cls_linear_gradient_shader::LinearGradientShader;
use tui_vfx_style::models::cls_neon_flicker_shader::NeonFlickerShader;
use tui_vfx_style::models::cls_orbit_shader::OrbitShader;
use tui_vfx_style::models::cls_pulse_wave_shader::PulseWaveShader;
use tui_vfx_style::models::cls_radar_shader::RadarShader;
use tui_vfx_style::models::cls_reflect_shader::ReflectShader;
use tui_vfx_style::models::cls_trace_path_shader::TracePathShader;
use tui_vfx_style::models::cls_trace_propagation_shader::TracePropagationShader;
use tui_vfx_style::models::cls_wayfinding_node_shader::WayfindingNodeShader;
use tui_vfx_style::models::{ColorSpace, FadeApplyTo, FadeTarget, SpatialShaderType, StyleEffect};
use tui_vfx_style::traits::StyleInterpolator;
use tui_vfx_types::Color;

// ============================================================================
// SpatialShaderType Tests
// ============================================================================

#[test]
fn test_spatial_shader_name_returns_nonempty() {
    let shaders: Vec<SpatialShaderType> = vec![
        SpatialShaderType::GlistenBand(GlistenBandShader::default()),
        SpatialShaderType::PulseWave(PulseWaveShader::default()),
        SpatialShaderType::ChromaticEdge(ChromaticEdgeShader::default()),
        SpatialShaderType::Radar(RadarShader {
            speed: 1.0,
            tail_length: 1.0,
            color: ColorConfig::Green,
        }),
        SpatialShaderType::Orbit(OrbitShader {
            speed: 1.0,
            dot_count: 3,
            color: ColorConfig::White,
        }),
        SpatialShaderType::BarberPole(BarberPoleShader {
            speed: 1.0,
            stripe_width: 2,
            gap_width: 2,
            color: ColorConfig::Red,
        }),
        SpatialShaderType::BorderSweep(BorderSweepShader {
            speed: 1.0,
            length: 5,
            color: ColorConfig::Cyan,
            head: None,
            tail: None,
            position_binding: None,
        }),
        SpatialShaderType::EdgeSheen(EdgeSheenShader::default()),
        SpatialShaderType::ConcealedLight(ConcealedLightShader::default()),
        SpatialShaderType::Diffusion(DiffusionShader::default()),
        SpatialShaderType::FocusField(FocusFieldShader::default()),
        SpatialShaderType::AffordanceWake(AffordanceWakeShader::default()),
        SpatialShaderType::WayfindingNode(WayfindingNodeShader::default()),
        SpatialShaderType::Highlighter(HighlighterShader::new(ColorConfig::Yellow)),
        SpatialShaderType::Reflect(ReflectShader {
            speed: 2.0,
            color: ColorConfig::White,
            ..ReflectShader::default()
        }),
        SpatialShaderType::GlitchLines(GlitchLinesShader::default()),
        SpatialShaderType::NeonFlicker(NeonFlickerShader::default()),
        SpatialShaderType::TracePropagation(TracePropagationShader::default()),
        SpatialShaderType::TracePath(TracePathShader::default()),
        SpatialShaderType::FocusedRowGradient(FocusedRowGradientShader::default()),
        SpatialShaderType::LinearGradient(LinearGradientShader {
            gradient: Gradient::default(),
            angle_deg: 0.0,
        }),
    ];

    for shader in &shaders {
        let name = shader.name();
        assert!(!name.is_empty(), "Shader name should not be empty");
        assert!(name.len() > 2, "Shader name should be meaningful: {}", name);
    }
}

#[test]
fn test_spatial_shader_description_returns_nonempty() {
    let shaders: Vec<SpatialShaderType> = vec![
        SpatialShaderType::GlistenBand(GlistenBandShader::default()),
        SpatialShaderType::PulseWave(PulseWaveShader::default()),
        SpatialShaderType::ChromaticEdge(ChromaticEdgeShader::default()),
        SpatialShaderType::Radar(RadarShader {
            speed: 1.0,
            tail_length: 1.0,
            color: ColorConfig::Green,
        }),
        SpatialShaderType::Orbit(OrbitShader {
            speed: 1.0,
            dot_count: 3,
            color: ColorConfig::White,
        }),
        SpatialShaderType::BarberPole(BarberPoleShader {
            speed: 1.0,
            stripe_width: 2,
            gap_width: 2,
            color: ColorConfig::Red,
        }),
        SpatialShaderType::BorderSweep(BorderSweepShader {
            speed: 1.0,
            length: 5,
            color: ColorConfig::Cyan,
            head: None,
            tail: None,
            position_binding: None,
        }),
        SpatialShaderType::EdgeSheen(EdgeSheenShader::default()),
        SpatialShaderType::ConcealedLight(ConcealedLightShader::default()),
        SpatialShaderType::Diffusion(DiffusionShader::default()),
        SpatialShaderType::FocusField(FocusFieldShader::default()),
        SpatialShaderType::AffordanceWake(AffordanceWakeShader::default()),
        SpatialShaderType::WayfindingNode(WayfindingNodeShader::default()),
        SpatialShaderType::Highlighter(HighlighterShader::new(ColorConfig::Yellow)),
        SpatialShaderType::Reflect(ReflectShader {
            speed: 2.0,
            color: ColorConfig::White,
            ..ReflectShader::default()
        }),
        SpatialShaderType::GlitchLines(GlitchLinesShader::default()),
        SpatialShaderType::NeonFlicker(NeonFlickerShader::default()),
        SpatialShaderType::TracePropagation(TracePropagationShader::default()),
        SpatialShaderType::TracePath(TracePathShader::default()),
        SpatialShaderType::FocusedRowGradient(FocusedRowGradientShader::default()),
        SpatialShaderType::LinearGradient(LinearGradientShader {
            gradient: Gradient::default(),
            angle_deg: 0.0,
        }),
    ];

    for shader in &shaders {
        let desc = shader.terse_description();
        assert!(!desc.is_empty(), "Shader description should not be empty");
        assert!(
            desc.len() > 10,
            "Shader description should be meaningful: {}",
            desc
        );
    }
}

#[test]
fn test_spatial_shader_key_parameters_format() {
    let shader = SpatialShaderType::GlistenBand(GlistenBandShader::default());
    let params = shader.key_parameters();

    // GlistenBand should have speed, band_width, direction, angle_deg
    assert!(!params.is_empty(), "GlistenBand should have key parameters");
    assert!(
        params.len() >= 3,
        "GlistenBand should have at least 3 key parameters"
    );

    // Each parameter should have a non-empty name
    for (name, value) in &params {
        assert!(!name.is_empty(), "Parameter name should not be empty");
        assert!(!value.is_empty(), "Parameter value should not be empty");
    }
}

#[test]
fn test_glisten_band_has_expected_params() {
    let shader = SpatialShaderType::GlistenBand(GlistenBandShader::default());
    let params = shader.key_parameters();
    let param_names: Vec<&str> = params.iter().map(|(n, _)| *n).collect();

    assert!(param_names.contains(&"speed"), "Should include speed param");
    assert!(
        param_names.contains(&"band_width"),
        "Should include band_width param"
    );
    assert!(
        param_names.contains(&"direction"),
        "Should include direction param"
    );
}

#[test]
fn test_pulse_wave_has_expected_params() {
    let shader = SpatialShaderType::PulseWave(PulseWaveShader::default());
    let params = shader.key_parameters();
    let param_names: Vec<&str> = params.iter().map(|(n, _)| *n).collect();

    assert!(
        param_names.contains(&"frequency"),
        "Should include frequency param"
    );
    assert!(param_names.contains(&"speed"), "Should include speed param");
    assert!(
        param_names.contains(&"direction"),
        "Should include direction param"
    );
}

// ============================================================================
// StyleEffect Tests
// ============================================================================

#[test]
fn test_style_effect_type_names() {
    let effects: Vec<StyleEffect> = vec![
        StyleEffect::FadeIn {
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
            from: FadeTarget::Black,
        },
        StyleEffect::FadeOut {
            apply_to: FadeApplyTo::Foreground,
            ease: EasingCurve::Type(EasingType::Linear),
            to: FadeTarget::Black,
        },
        StyleEffect::Pulse {
            frequency: 1.0,
            color: Color::CYAN,
        },
        StyleEffect::Rainbow { speed: 1.0 },
        StyleEffect::Glitch {
            seed: 42,
            intensity: 0.5,
            italic_start: None,
            italic_end: None,
        },
        StyleEffect::NeonFlicker { stability: 0.7 },
        StyleEffect::ItalicWindow {
            start: 0.0,
            end: 0.5,
        },
        StyleEffect::ColorShift {
            hue_shift: 30.0,
            saturation_shift: 0.1,
            lightness_shift: 0.0,
        },
        StyleEffect::ColorFade {
            target: Color::BLACK,
            color_space: ColorSpace::Rgb,
        },
    ];

    for effect in &effects {
        let name = effect.effect_type_name();
        assert!(!name.is_empty(), "Effect type name should not be empty");
    }
}

#[test]
fn test_style_effect_descriptions() {
    let effects: Vec<StyleEffect> = vec![
        StyleEffect::FadeIn {
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
            from: FadeTarget::Black,
        },
        StyleEffect::FadeOut {
            apply_to: FadeApplyTo::Foreground,
            ease: EasingCurve::Type(EasingType::Linear),
            to: FadeTarget::Black,
        },
        StyleEffect::Pulse {
            frequency: 1.0,
            color: Color::CYAN,
        },
        StyleEffect::Rainbow { speed: 1.0 },
        StyleEffect::Glitch {
            seed: 42,
            intensity: 0.5,
            italic_start: None,
            italic_end: None,
        },
        StyleEffect::NeonFlicker { stability: 0.7 },
        StyleEffect::ItalicWindow {
            start: 0.0,
            end: 0.5,
        },
        StyleEffect::ColorShift {
            hue_shift: 30.0,
            saturation_shift: 0.1,
            lightness_shift: 0.0,
        },
        StyleEffect::ColorFade {
            target: Color::BLACK,
            color_space: ColorSpace::Rgb,
        },
    ];

    for effect in &effects {
        let desc = effect.terse_description();
        assert!(!desc.is_empty(), "Effect description should not be empty");
        assert!(
            desc.len() > 10,
            "Effect description should be meaningful: {}",
            desc
        );
    }
}

#[test]
fn test_spatial_effect_delegates_description() {
    let shader = SpatialShaderType::GlistenBand(GlistenBandShader::default());
    let effect = StyleEffect::Spatial {
        shader: shader.clone(),
    };

    // The Spatial variant should delegate to the shader's description
    assert_eq!(effect.terse_description(), shader.terse_description());
}

#[test]
fn test_spatial_effect_delegates_key_parameters() {
    let shader = SpatialShaderType::GlistenBand(GlistenBandShader::default());
    let effect = StyleEffect::Spatial {
        shader: shader.clone(),
    };

    // The Spatial variant should delegate to the shader's key_parameters
    assert_eq!(effect.key_parameters(), shader.key_parameters());
}

#[test]
fn test_fade_effect_key_parameters() {
    let effect = StyleEffect::FadeIn {
        apply_to: FadeApplyTo::Both,
        ease: EasingCurve::Type(EasingType::Linear),
        from: FadeTarget::Black,
    };
    let params = effect.key_parameters();
    let param_names: Vec<&str> = params.iter().map(|(n, _)| *n).collect();

    assert!(
        param_names.contains(&"apply_to"),
        "FadeIn should include apply_to param"
    );
    assert!(
        param_names.contains(&"ease"),
        "FadeIn should include ease param"
    );
    assert!(
        param_names.contains(&"from"),
        "FadeIn should include from param"
    );
}

#[test]
fn fade_in_default_from_is_black_for_backward_compat() {
    // Regression: confirm that omitting `from` in a recipe still parses to
    // FadeTarget::Black, so existing recipes don't break.
    let json = r#"{"type": "fade_in", "apply_to": "both", "ease": "linear"}"#;
    let effect: StyleEffect = serde_json::from_str(json).unwrap();
    match effect {
        StyleEffect::FadeIn { from, .. } => assert_eq!(from, FadeTarget::Black),
        _ => panic!("expected FadeIn"),
    }
}

#[test]
fn fade_in_from_rgb_round_trips_through_serde() {
    // The whole point of the new field: a recipe can specify a custom from
    // color (typically the canvas color) to fade from instead of black.
    let effect = StyleEffect::FadeIn {
        apply_to: FadeApplyTo::Both,
        ease: EasingCurve::Type(EasingType::Linear),
        from: FadeTarget::Color {
            color: ColorConfig::Rgb {
                r: 120,
                g: 20,
                b: 100,
            },
        },
    };
    let serialized = serde_json::to_string(&effect).unwrap();
    let parsed: StyleEffect = serde_json::from_str(&serialized).unwrap();
    assert_eq!(effect, parsed);
}

#[test]
fn fade_in_from_canvas_color_paints_canvas_at_t_zero() {
    // The whole point of the fix: at t=0 the result should be the canvas
    // color, not black, so the widget appears to grow out of the canvas
    // instead of stamping near-black over it.
    let canvas = tui_vfx_types::Color::rgb(120, 20, 100);
    let widget = tui_vfx_types::Color::rgb(252, 252, 248);
    let effect = StyleEffect::FadeIn {
        apply_to: FadeApplyTo::Both,
        ease: EasingCurve::Type(EasingType::Linear),
        from: FadeTarget::Color {
            color: ColorConfig::Rgb {
                r: 120,
                g: 20,
                b: 100,
            },
        },
    };
    let base = tui_vfx_types::Style {
        fg: widget,
        bg: widget,
        mods: tui_vfx_types::Modifiers::default(),
    };

    let at_zero = effect.calculate(0.0, base);
    assert_eq!(
        at_zero.bg, canvas,
        "at t=0 the fade should paint the canvas color, not black"
    );

    let at_one = effect.calculate(1.0, base);
    assert_eq!(
        at_one.bg, widget,
        "at t=1 the fade should paint the base widget color"
    );
}

#[test]
fn test_color_shift_key_parameters() {
    let effect = StyleEffect::ColorShift {
        hue_shift: 45.0,
        saturation_shift: 0.2,
        lightness_shift: -0.1,
    };
    let params = effect.key_parameters();
    let param_names: Vec<&str> = params.iter().map(|(n, _)| *n).collect();

    assert!(
        param_names.contains(&"hue_shift"),
        "ColorShift should include hue_shift"
    );
    assert!(
        param_names.contains(&"saturation_shift"),
        "ColorShift should include saturation_shift"
    );
    assert!(
        param_names.contains(&"lightness_shift"),
        "ColorShift should include lightness_shift"
    );
}

// <FILE>tui-vfx-style/tests/models/test_effect_descriptions.rs</FILE> - <DESC>Tests for effect documentation methods</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
