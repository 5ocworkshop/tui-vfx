// <FILE>crates/tui-vfx-style/tests/models/test_cls_focus_field_shader.rs</FILE> - <DESC>Integration tests for FocusFieldShader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validate both ellipse and rect focus field modes with runtime bindings</WCTX>
// <CLOG>Initial tests for defaults, ellipse focus, pane-following rect behavior, bindings, pulse, and serde roundtrip</CLOG>

use crate::common::{make_ctx, make_style};

use std::sync::Arc;
use tui_vfx_style::models::{FocusFieldShape, FocusFieldShader};
use tui_vfx_style::traits::{ShaderContext, ShaderRuntimeParams, StyleShader};

#[test]
fn default_values_are_conservative() {
    let shader = FocusFieldShader::default();
    assert_eq!(shader.shape, FocusFieldShape::Ellipse);
    assert!(shader.intensity > 0.0 && shader.intensity < 0.4);
    assert_eq!(shader.feather, 3);
}

#[test]
fn ellipse_focus_affects_center() {
    let shader = FocusFieldShader {
        center_x: 6,
        center_y: 4,
        radius_x: 6,
        radius_y: 3,
        intensity: 0.4,
        ..Default::default()
    };
    let base = make_style();
    let center = shader.style_at(&make_ctx(6, 4, 16, 10, 0.0), base);
    let far = shader.style_at(&make_ctx(15, 9, 16, 10, 0.0), base);
    assert_ne!(center, base);
    assert_eq!(far, base);
}

#[test]
fn rect_focus_fills_inside_pane() {
    let shader = FocusFieldShader {
        shape: FocusFieldShape::Rect,
        rect_x: 3,
        rect_y: 2,
        rect_width: 6,
        rect_height: 3,
        feather: 2,
        intensity: 0.4,
        ..Default::default()
    };
    let base = make_style();
    let inside = shader.style_at(&make_ctx(4, 3, 16, 10, 0.0), base);
    let far = shader.style_at(&make_ctx(15, 9, 16, 10, 0.0), base);
    assert_ne!(inside, base);
    assert_eq!(far, base);
}

#[test]
fn center_bindings_override_static_values() {
    let shader = FocusFieldShader {
        center_x: 0,
        center_y: 0,
        center_x_binding: Some("focus_x".to_string()),
        center_y_binding: Some("focus_y".to_string()),
        radius_x: 4,
        radius_y: 2,
        intensity: 0.4,
        ..Default::default()
    };
    let params = [("focus_x", 8_u16), ("focus_y", 3_u16)]
        .into_iter()
        .collect::<ShaderRuntimeParams>();
    let ctx = ShaderContext::new(8, 3, 16, 10, 0, 0, 0.0, None, Some(Arc::new(params)));
    let styled = shader.style_at(&ctx, make_style());
    assert_ne!(styled, make_style());
}

#[test]
fn rect_bindings_override_static_values() {
    let shader = FocusFieldShader {
        shape: FocusFieldShape::Rect,
        rect_x: 0,
        rect_y: 0,
        rect_width: 1,
        rect_height: 1,
        rect_x_binding: Some("pane_x".to_string()),
        rect_y_binding: Some("pane_y".to_string()),
        rect_width_binding: Some("pane_w".to_string()),
        rect_height_binding: Some("pane_h".to_string()),
        intensity: 0.4,
        ..Default::default()
    };
    let params = [
        ("pane_x", 4_u16),
        ("pane_y", 2_u16),
        ("pane_w", 6_u16),
        ("pane_h", 3_u16),
    ]
    .into_iter()
    .collect::<ShaderRuntimeParams>();
    let ctx = ShaderContext::new(5, 3, 16, 10, 0, 0, 0.0, None, Some(Arc::new(params)));
    let styled = shader.style_at(&ctx, make_style());
    assert_ne!(styled, make_style());
}

#[test]
fn pulse_speed_changes_field_gently() {
    let shader = FocusFieldShader {
        center_x: 6,
        center_y: 4,
        radius_x: 6,
        radius_y: 3,
        intensity: 0.25,
        pulse_speed: 0.5,
        ..Default::default()
    };
    let base = make_style();
    let a = shader.style_at(&make_ctx(6, 4, 16, 10, 0.0), base);
    let b = shader.style_at(&make_ctx(6, 4, 16, 10, 0.5), base);
    assert_ne!(a, b);
}

#[test]
fn serde_roundtrip() {
    let shader = FocusFieldShader {
        shape: FocusFieldShape::Rect,
        rect_x: 2,
        rect_y: 1,
        rect_width: 8,
        rect_height: 4,
        center_x_binding: Some("focus_x".to_string()),
        rect_width_binding: Some("pane_w".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&shader).unwrap();
    let parsed: FocusFieldShader = serde_json::from_str(&json).unwrap();
    assert_eq!(shader, parsed);
}

// <FILE>crates/tui-vfx-style/tests/models/test_cls_focus_field_shader.rs</FILE> - <DESC>Integration tests for FocusFieldShader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
