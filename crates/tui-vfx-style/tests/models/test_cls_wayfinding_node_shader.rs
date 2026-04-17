// <FILE>crates/tui-vfx-style/tests/models/test_cls_wayfinding_node_shader.rs</FILE> - <DESC>Integration tests for WayfindingNodeShader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validate calm node-based wayfinding emphasis and current-index binding behavior</WCTX>
// <CLOG>Initial tests for defaults, node activation, current node vs previous node strength, bindings, and serde roundtrip</CLOG>

use crate::common::{make_ctx, make_style};

use std::sync::Arc;
use tui_vfx_style::models::{WayfindingNode, WayfindingNodeShader};
use tui_vfx_style::traits::{ShaderContext, ShaderRuntimeParams, StyleShader};

#[test]
fn default_values_are_conservative() {
    let shader = WayfindingNodeShader::default();
    assert_eq!(shader.radius, 2);
    assert!(shader.intensity > 0.0 && shader.intensity < 0.4);
    assert!(shader.nodes.is_empty());
}

#[test]
fn empty_nodes_no_change() {
    let shader = WayfindingNodeShader::default();
    let base = make_style();
    assert_eq!(shader.style_at(&make_ctx(3, 2, 12, 8, 0.0), base), base);
}

#[test]
fn node_activates_locally() {
    let shader = WayfindingNodeShader {
        nodes: vec![WayfindingNode { x: 2, y: 2 }],
        intensity: 0.5,
        radius: 2,
        ..Default::default()
    };
    let base = make_style();
    let near = shader.style_at(&make_ctx(2, 2, 12, 8, 0.0), base);
    let far = shader.style_at(&make_ctx(8, 6, 12, 8, 0.0), base);
    assert_ne!(near, base);
    assert_eq!(far, base);
}

#[test]
fn current_node_is_stronger_than_previous() {
    let shader = WayfindingNodeShader {
        nodes: vec![WayfindingNode { x: 2, y: 2 }, WayfindingNode { x: 8, y: 2 }],
        current_index: Some(1),
        intensity: 0.4,
        radius: 2,
        previous_strength: 0.3,
        ..Default::default()
    };
    let base = make_style();
    let previous = shader.style_at(&make_ctx(2, 2, 12, 8, 0.0), base);
    let current = shader.style_at(&make_ctx(8, 2, 12, 8, 0.0), base);
    assert_ne!(previous, current);
}

#[test]
fn current_index_binding_overrides_static() {
    let shader = WayfindingNodeShader {
        nodes: vec![WayfindingNode { x: 2, y: 2 }, WayfindingNode { x: 8, y: 2 }],
        current_index: Some(0),
        current_index_binding: Some("current".to_string()),
        intensity: 0.4,
        radius: 2,
        ..Default::default()
    };
    let base = make_style();
    let params = [("current", 1_u16)]
        .into_iter()
        .collect::<ShaderRuntimeParams>();
    let ctx = ShaderContext::new(8, 2, 12, 8, 0, 0, 0.0, None, Some(Arc::new(params)));
    let styled = shader.style_at(&ctx, base);
    assert_ne!(styled, base);
}

#[test]
fn serde_roundtrip() {
    let shader = WayfindingNodeShader {
        nodes: vec![WayfindingNode { x: 1, y: 1 }, WayfindingNode { x: 5, y: 1 }],
        current_index: Some(1),
        current_index_binding: Some("current".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&shader).unwrap();
    let parsed: WayfindingNodeShader = serde_json::from_str(&json).unwrap();
    assert_eq!(shader, parsed);
}

// <FILE>crates/tui-vfx-style/tests/models/test_cls_wayfinding_node_shader.rs</FILE> - <DESC>Integration tests for WayfindingNodeShader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
