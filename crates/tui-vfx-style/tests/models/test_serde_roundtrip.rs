// <FILE>tui-vfx-style/tests/models/test_serde_roundtrip.rs</FILE> - <DESC>Serde roundtrip tests for config-ish models</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>Ensure Orbit spatial shader roundtrips via serde</WCTX>
// <CLOG>Add Orbit spatial shader roundtrip test</CLOG>

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_style::models::cls_chromatic_edge_shader::ChromaticEdgeShader;
use tui_vfx_style::models::cls_orbit_shader::OrbitShader;
use tui_vfx_style::models::{
    ColorSpace, Gradient, LinearGradientShader, SpatialShaderType, StyleEffect, StyleTransition,
};
use tui_vfx_types::{Color, Modifiers, Style};

#[test]
fn test_gradient_roundtrip() {
    let gradient = Gradient::new(vec![
        (0.0, Color::BLACK),
        (0.5, Color::RED),
        (1.0, Color::WHITE),
    ]);
    let json = serde_json::to_string(&gradient).expect("serialize gradient");
    let parsed: Gradient = serde_json::from_str(&json).expect("deserialize gradient");
    assert_eq!(gradient, parsed);
}

#[test]
fn test_style_transition_roundtrip() {
    let start = Style::new(
        Color::RED,
        Color::BLACK,
        Modifiers::bold().combine(Modifiers::underline()),
    );
    let end = Style::new(
        Color::rgb(10, 20, 30),
        Color::rgb(200, 200, 200),
        Modifiers::italic(),
    );

    let transition = StyleTransition::new(start, end)
        .with_ease(EasingCurve::Type(EasingType::QuadInOut))
        .with_color_space(ColorSpace::Hsl);

    let json = serde_json::to_string(&transition).expect("serialize transition");
    let parsed: StyleTransition = serde_json::from_str(&json).expect("deserialize transition");
    assert_eq!(transition, parsed);
}

#[test]
fn test_style_effect_roundtrip_pulse() {
    let effect = StyleEffect::Pulse {
        frequency: 1.0,
        color: Color::CYAN, // Using CYAN as a substitute for LightBlue
    };
    let json = serde_json::to_string(&effect).expect("serialize style effect");
    let parsed: StyleEffect = serde_json::from_str(&json).expect("deserialize style effect");
    assert_eq!(effect, parsed);
}

#[test]
fn test_style_effect_roundtrip_spatial_linear_gradient() {
    let gradient = Gradient::new(vec![(0.0, Color::BLACK), (1.0, Color::WHITE)]);
    let shader = LinearGradientShader::vertical(gradient);
    let effect = StyleEffect::Spatial {
        shader: SpatialShaderType::LinearGradient(shader),
    };
    let json = serde_json::to_string(&effect).expect("serialize spatial effect");
    let parsed: StyleEffect = serde_json::from_str(&json).expect("deserialize spatial effect");
    assert_eq!(effect, parsed);
}

#[test]
fn test_style_effect_roundtrip_spatial_chromatic_edge() {
    let shader = ChromaticEdgeShader::default();
    let effect = StyleEffect::Spatial {
        shader: SpatialShaderType::ChromaticEdge(shader),
    };
    let json = serde_json::to_string(&effect).expect("serialize spatial effect");
    let parsed: StyleEffect = serde_json::from_str(&json).expect("deserialize spatial effect");
    assert_eq!(effect, parsed);
}

#[test]
fn test_style_effect_roundtrip_spatial_orbit() {
    let shader = OrbitShader {
        speed: 1.0,
        dot_count: 3,
        color: tui_vfx_style::models::ColorConfig::White,
    };
    let effect = StyleEffect::Spatial {
        shader: SpatialShaderType::Orbit(shader),
    };
    let json = serde_json::to_string(&effect).expect("serialize spatial effect");
    let parsed: StyleEffect = serde_json::from_str(&json).expect("deserialize spatial effect");
    assert_eq!(effect, parsed);
}

// <FILE>tui-vfx-style/tests/models/test_serde_roundtrip.rs</FILE> - <DESC>Serde roundtrip tests for config-ish models</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
