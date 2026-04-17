// <FILE>crates/tui-vfx-style/tests/models/test_cls_affordance_wake_shader.rs</FILE> - <DESC>Integration tests for AffordanceWakeShader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validate latent affordance wake behavior and runtime progress binding support</WCTX>
// <CLOG>Initial tests for defaults, edge/corner targeting, progress binding, and serde roundtrip</CLOG>

use crate::common::{make_ctx, make_style};

use std::sync::Arc;
use tui_vfx_style::models::{AffordanceWakeShader, AffordanceWakeZone};
use tui_vfx_style::traits::{ShaderContext, ShaderRuntimeParams, StyleShader};

#[test]
fn default_values_are_conservative() {
    let shader = AffordanceWakeShader::default();
    assert_eq!(shader.zone, AffordanceWakeZone::AllEdges);
    assert_eq!(shader.radius, 2);
    assert_eq!(shader.progress, 0.0);
    assert!(shader.peak_intensity > 0.0 && shader.peak_intensity < 0.4);
}

#[test]
fn zero_progress_no_change_by_default() {
    let shader = AffordanceWakeShader::default();
    let base = make_style();
    assert_eq!(shader.style_at(&make_ctx(0, 2, 12, 8, 0.0), base), base);
}

#[test]
fn edges_wake_with_progress() {
    let shader = AffordanceWakeShader {
        progress: 1.0,
        ..Default::default()
    };
    let base = make_style();
    let edge = shader.style_at(&make_ctx(0, 2, 12, 8, 0.0), base);
    let center = shader.style_at(&make_ctx(6, 4, 12, 8, 0.0), base);
    assert_ne!(edge, base);
    assert_eq!(center, base);
}

#[test]
fn corners_zone_is_localized() {
    let shader = AffordanceWakeShader {
        zone: AffordanceWakeZone::Corners,
        progress: 1.0,
        radius: 3,
        ..Default::default()
    };
    let base = make_style();
    let corner = shader.style_at(&make_ctx(0, 0, 12, 8, 0.0), base);
    let top_mid = shader.style_at(&make_ctx(6, 0, 12, 8, 0.0), base);
    assert_ne!(corner, base);
    assert_eq!(top_mid, base);
}

#[test]
fn progress_binding_overrides_static_value() {
    let shader = AffordanceWakeShader {
        progress: 0.0,
        progress_binding: Some("wake".to_string()),
        ..Default::default()
    };
    let base = make_style();
    let params = [("wake", 1.0_f32)]
        .into_iter()
        .collect::<ShaderRuntimeParams>();
    let ctx = ShaderContext::new(0, 2, 12, 8, 0, 0, 0.0, None, Some(Arc::new(params)));
    let styled = shader.style_at(&ctx, base);
    assert_ne!(styled, base);
}

#[test]
fn serde_roundtrip() {
    let shader = AffordanceWakeShader {
        zone: AffordanceWakeZone::RightRail,
        progress: 0.4,
        rest_intensity: 0.02,
        peak_intensity: 0.2,
        progress_binding: Some("wake".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&shader).unwrap();
    let parsed: AffordanceWakeShader = serde_json::from_str(&json).unwrap();
    assert_eq!(shader, parsed);
}

// <FILE>crates/tui-vfx-style/tests/models/test_cls_affordance_wake_shader.rs</FILE> - <DESC>Integration tests for AffordanceWakeShader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
