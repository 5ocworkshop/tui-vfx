// <FILE>crates/tui-vfx-style/tests/models/test_cls_concealed_light_shader.rs</FILE> - <DESC>Integration tests for ConcealedLightShader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validate hidden-source architectural light behavior and distinctness from Glow</WCTX>
// <CLOG>Initial tests for defaults, cutoff, directionality, static behavior, low-amplitude pulse, and serde roundtrip</CLOG>

use crate::common::{make_ctx, make_style};

use tui_vfx_style::models::{
    ConcealedLightMode, ConcealedLightShader, ConcealedLightSource, GlowShader,
};
use tui_vfx_style::traits::StyleShader;

#[test]
fn default_values_are_conservative() {
    let shader = ConcealedLightShader::default();
    assert_eq!(shader.source, ConcealedLightSource::Top);
    assert_eq!(shader.spread, 4);
    assert_eq!(shader.edge_width, 1);
    assert_eq!(shader.mode, ConcealedLightMode::Static);
    assert!(shader.intensity > 0.0 && shader.intensity < 0.3);
}

#[test]
fn zero_intensity_no_change() {
    let shader = ConcealedLightShader {
        intensity: 0.0,
        ..Default::default()
    };
    let base = make_style();
    assert_eq!(shader.style_at(&make_ctx(2, 1, 12, 8, 0.0), base), base);
}

#[test]
fn concealed_cutoff_keeps_source_edge_dark() {
    let shader = ConcealedLightShader {
        intensity: 0.6,
        spread: 5,
        edge_width: 1,
        source_cutoff: 0.25,
        ..Default::default()
    };
    let base = make_style();
    let at_edge = shader.style_at(&make_ctx(3, 0, 12, 8, 0.0), base);
    let one_inward = shader.style_at(&make_ctx(3, 2, 12, 8, 0.0), base);
    assert_eq!(at_edge, base);
    assert_ne!(one_inward, base);
}

#[test]
fn top_source_is_directional() {
    let shader = ConcealedLightShader {
        intensity: 0.6,
        spread: 5,
        source_cutoff: 0.0,
        ..Default::default()
    };
    let base = make_style();
    let near_top = shader.style_at(&make_ctx(3, 1, 12, 8, 0.0), base);
    let near_bottom = shader.style_at(&make_ctx(3, 6, 12, 8, 0.0), base);
    assert_ne!(near_top, base);
    assert_eq!(near_bottom, base);
}

#[test]
fn static_mode_ignores_time() {
    let shader = ConcealedLightShader {
        intensity: 0.4,
        spread: 5,
        source_cutoff: 0.0,
        ..Default::default()
    };
    let base = make_style();
    let a = shader.style_at(&make_ctx(3, 1, 12, 8, 0.0), base);
    let b = shader.style_at(&make_ctx(3, 1, 12, 8, 0.8), base);
    assert_eq!(a, b);
}

#[test]
fn pulse_mode_changes_low_amplitude() {
    let shader = ConcealedLightShader {
        intensity: 0.3,
        spread: 5,
        source_cutoff: 0.0,
        mode: ConcealedLightMode::Pulse,
        pulse_speed: 1.0,
        ..Default::default()
    };
    let base = make_style();
    let a = shader.style_at(&make_ctx(3, 1, 12, 8, 0.0), base);
    let b = shader.style_at(&make_ctx(3, 1, 12, 8, 0.25), base);
    assert_ne!(a, b);
}

#[test]
fn distinct_from_glow_on_source_edge() {
    let concealed = ConcealedLightShader {
        intensity: 0.5,
        spread: 5,
        edge_width: 1,
        source_cutoff: 0.25,
        ..Default::default()
    };
    let glow = GlowShader::default();
    let base = make_style();
    let concealed_edge = concealed.style_at(&make_ctx(3, 0, 12, 8, 0.0), base);
    let glow_edge = glow.style_at(&make_ctx(3, 0, 12, 8, 0.0), base);
    assert_ne!(concealed_edge, glow_edge);
}

#[test]
fn serde_roundtrip() {
    let shader = ConcealedLightShader {
        spread: 6,
        edge_width: 2,
        intensity: 0.22,
        mode: ConcealedLightMode::Drift,
        pulse_speed: 0.15,
        source_cutoff: 0.2,
        source: ConcealedLightSource::Left,
        ..Default::default()
    };
    let json = serde_json::to_string(&shader).unwrap();
    let parsed: ConcealedLightShader = serde_json::from_str(&json).unwrap();
    assert_eq!(shader, parsed);
}

// <FILE>crates/tui-vfx-style/tests/models/test_cls_concealed_light_shader.rs</FILE> - <DESC>Integration tests for ConcealedLightShader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
