// <FILE>crates/tui-vfx-style/tests/models/test_cls_diffusion_shader.rs</FILE> - <DESC>Integration tests for DiffusionShader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validate soft material-light diffusion behavior and distinctness from Glow</WCTX>
// <CLOG>Initial tests for defaults, source geometry, softness, frame discipline, drift, and serde roundtrip</CLOG>

use crate::common::{make_ctx, make_style};

use mixed_signals::types::SignalOrFloat;
use tui_vfx_style::models::{DiffusionMode, DiffusionShader, DiffusionSource, GlowShader};
use tui_vfx_style::traits::StyleShader;

#[test]
fn default_values_are_conservative() {
    let shader = DiffusionShader::default();
    assert_eq!(shader.source, DiffusionSource::Center);
    assert_eq!(shader.radius, 6);
    assert_eq!(shader.mode, DiffusionMode::Static);
    assert_eq!(shader.intensity, SignalOrFloat::Static(0.2));
}

#[test]
fn zero_intensity_no_change() {
    let shader = DiffusionShader {
        intensity: SignalOrFloat::Static(0.0),
        ..Default::default()
    };
    let base = make_style();
    assert_eq!(shader.style_at(&make_ctx(5, 4, 12, 8, 0.0), base), base);
}

#[test]
fn center_source_affects_interior() {
    let shader = DiffusionShader {
        intensity: SignalOrFloat::Static(0.5),
        radius: 4,
        ..Default::default()
    };
    let base = make_style();
    let center = shader.style_at(&make_ctx(6, 4, 12, 8, 0.0), base);
    let corner = shader.style_at(&make_ctx(0, 0, 12, 8, 0.0), base);
    assert_ne!(center, base);
    assert_eq!(corner, base);
}

#[test]
fn top_source_is_directional() {
    let shader = DiffusionShader {
        source: DiffusionSource::Top,
        intensity: SignalOrFloat::Static(0.5),
        radius: 5,
        ..Default::default()
    };
    let base = make_style();
    let near_top = shader.style_at(&make_ctx(5, 1, 12, 8, 0.0), base);
    let near_bottom = shader.style_at(&make_ctx(5, 7, 12, 8, 0.0), base);
    assert_ne!(near_top, base);
    assert_eq!(near_bottom, base);
}

#[test]
fn softness_broadens_transition() {
    let sharp = DiffusionShader {
        softness: 0.0,
        intensity: SignalOrFloat::Static(0.6),
        radius: 6,
        ..Default::default()
    };
    let soft = DiffusionShader {
        softness: 1.0,
        ..sharp.clone()
    };
    let base = make_style();
    let sharp_style = sharp.style_at(&make_ctx(7, 4, 12, 8, 0.0), base);
    let soft_style = soft.style_at(&make_ctx(7, 4, 12, 8, 0.0), base);
    assert_ne!(sharp_style, soft_style);
}

#[test]
fn edge_firmness_changes_perimeter_behavior() {
    let soft = DiffusionShader {
        source: DiffusionSource::Center,
        intensity: SignalOrFloat::Static(0.5),
        radius: 8,
        edge_firmness: 0.0,
        ..Default::default()
    };
    let firm = DiffusionShader {
        edge_firmness: 0.9,
        ..soft.clone()
    };
    let base = make_style();
    let soft_edge = soft.style_at(&make_ctx(1, 1, 12, 8, 0.0), base);
    let firm_edge = firm.style_at(&make_ctx(1, 1, 12, 8, 0.0), base);
    assert_ne!(soft_edge, firm_edge);
}

#[test]
fn breath_mode_changes_gently_over_time() {
    let shader = DiffusionShader {
        intensity: SignalOrFloat::Static(0.25),
        radius: 6,
        mode: DiffusionMode::Breath,
        drift_speed: 0.5,
        drift_amount: 0.06,
        ..Default::default()
    };
    let base = make_style();
    let a = shader.style_at(&make_ctx(6, 4, 12, 8, 0.0), base);
    let b = shader.style_at(&make_ctx(6, 4, 12, 8, 0.5), base);
    assert_ne!(a, b);
}

#[test]
fn distinct_from_glow_for_center_field() {
    let diffusion = DiffusionShader {
        intensity: SignalOrFloat::Static(0.5),
        radius: 8,
        ..Default::default()
    };
    let glow = GlowShader::default();
    let base = make_style();
    let diffusion_center = diffusion.style_at(&make_ctx(6, 4, 12, 8, 0.0), base);
    let glow_center = glow.style_at(&make_ctx(6, 4, 12, 8, 0.0), base);
    assert_ne!(diffusion_center, glow_center);
}

#[test]
fn serde_roundtrip() {
    let shader = DiffusionShader {
        source: DiffusionSource::TopLeft,
        radius: 7,
        softness: 0.8,
        edge_firmness: 0.35,
        intensity: SignalOrFloat::Static(0.22),
        mode: DiffusionMode::WarmDrift,
        drift_speed: 0.08,
        drift_amount: 0.04,
        ..Default::default()
    };
    let json = serde_json::to_string(&shader).unwrap();
    let parsed: DiffusionShader = serde_json::from_str(&json).unwrap();
    assert_eq!(shader, parsed);
}

#[test]
fn signal_intensity_changes_with_context() {
    let shader = DiffusionShader {
        intensity: SignalOrFloat::Signal {
            spec: mixed_signals::types::SignalSpec::SampleNormX,
            cache: std::sync::OnceLock::new(),
        },
        radius: 8,
        ..Default::default()
    };
    let base = make_style();
    let left = shader.style_at(&make_ctx(0, 4, 12, 8, 0.0), base);
    let right = shader.style_at(&make_ctx(11, 4, 12, 8, 0.0), base);
    assert_ne!(left, right);
}

// <FILE>crates/tui-vfx-style/tests/models/test_cls_diffusion_shader.rs</FILE> - <DESC>Integration tests for DiffusionShader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
