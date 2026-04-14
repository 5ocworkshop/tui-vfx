// <FILE>tui-vfx-compositor/tests/types/test_bindable_value.rs</FILE> - <DESC>Tests for BindableValue evaluate, conversions, and serde</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 P0.1 — progress_binding infrastructure</WCTX>
// <CLOG>Initial tests for BindableValue: static literal, runtime binding hit/miss, From<f32>, serde roundtrips</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use mixed_signals::traits::SignalContext;
use tui_vfx_compositor::types::BindableValue;
use tui_vfx_style::traits::ShaderRuntimeParams;

fn empty_ctx() -> SignalContext {
    SignalContext::for_loop(0.0, 0)
}

#[test]
fn signal_static_evaluates_to_literal() {
    let value = BindableValue::Signal(SignalOrFloat::Static(0.5));
    let ctx = empty_ctx();
    let rp = ShaderRuntimeParams::new();

    assert_eq!(value.evaluate(0.0, &ctx, &rp), Some(0.5));
}

#[test]
fn binding_hit_returns_runtime_param() {
    let value = BindableValue::Binding("progress".to_string());
    let ctx = empty_ctx();
    let mut rp = ShaderRuntimeParams::new();
    rp.insert("progress", 0.7_f32);

    assert_eq!(value.evaluate(0.0, &ctx, &rp), Some(0.7));
}

#[test]
fn binding_miss_returns_none() {
    let value = BindableValue::Binding("progress".to_string());
    let ctx = empty_ctx();
    let rp = ShaderRuntimeParams::new();

    assert_eq!(value.evaluate(0.0, &ctx, &rp), None);
}

#[test]
fn binding_ignores_signal_context_time() {
    // A binding value is looked up directly from runtime_params; varying loop_t
    // must not affect the result.
    let value = BindableValue::Binding("progress".to_string());
    let ctx = empty_ctx();
    let mut rp = ShaderRuntimeParams::new();
    rp.insert("progress", 0.25_f32);

    assert_eq!(value.evaluate(0.0, &ctx, &rp), Some(0.25));
    assert_eq!(value.evaluate(0.5, &ctx, &rp), Some(0.25));
    assert_eq!(value.evaluate(0.99, &ctx, &rp), Some(0.25));
}

#[test]
fn from_f32_produces_static_signal() {
    let value: BindableValue = 0.5_f32.into();
    let ctx = empty_ctx();
    let rp = ShaderRuntimeParams::new();

    assert_eq!(value.evaluate(0.0, &ctx, &rp), Some(0.5));
    assert_eq!(value, BindableValue::static_f32(0.5));
}

#[test]
fn from_signal_or_float_wraps_in_signal_variant() {
    let source = SignalOrFloat::Static(0.25);
    let value: BindableValue = source.into();

    assert_eq!(value, BindableValue::Signal(SignalOrFloat::Static(0.25)));
}

#[test]
fn default_is_zero_static() {
    let value = BindableValue::default();
    let ctx = empty_ctx();
    let rp = ShaderRuntimeParams::new();

    assert_eq!(value.evaluate(0.0, &ctx, &rp), Some(0.0));
}

#[test]
fn signal_serde_roundtrip() {
    let value = BindableValue::Signal(SignalOrFloat::Static(0.5));
    let json = serde_json::to_string(&value).unwrap();
    let parsed: BindableValue = serde_json::from_str(&json).unwrap();

    assert_eq!(value, parsed);
}

#[test]
fn binding_serde_roundtrip() {
    let value = BindableValue::Binding("progress".to_string());
    let json = serde_json::to_string(&value).unwrap();
    let parsed: BindableValue = serde_json::from_str(&json).unwrap();

    assert_eq!(value, parsed);
}

#[test]
fn binding_json_shape_is_externally_tagged_snake_case() {
    let value = BindableValue::Binding("progress_ratio".to_string());
    let json = serde_json::to_string(&value).unwrap();

    assert_eq!(json, r#"{"binding":"progress_ratio"}"#);
}

#[test]
fn deserializes_raw_number_as_static_signal() {
    // Existing recipes emit `"progress": 0.75` — accept it as a static literal.
    let parsed: BindableValue = serde_json::from_str("0.75").unwrap();
    assert_eq!(parsed, BindableValue::Signal(SignalOrFloat::Static(0.75)));
}

#[test]
fn deserializes_tagged_signal_object_with_raw_number_inner() {
    // `SignalOrFloat::Static` serializes as a bare number, so `{"signal": 0.5}`
    // is the tagged form when `signal_serde_roundtrip` round-trips a static.
    let parsed: BindableValue = serde_json::from_str(r#"{"signal":0.5}"#).unwrap();
    assert_eq!(parsed, BindableValue::Signal(SignalOrFloat::Static(0.5)));
}

#[test]
fn deserializes_tagged_binding_object() {
    let parsed: BindableValue = serde_json::from_str(r#"{"binding":"progress"}"#).unwrap();
    assert_eq!(parsed, BindableValue::Binding("progress".to_string()));
}

#[test]
fn signal_static_serializes_as_tagged_raw_number() {
    // Documents the wire format: a static signal emits as `{"signal": <num>}`,
    // not `{"signal": {"static": <num>}}`, because SignalOrFloat::Static uses
    // untagged serde via SignalOrFloatSerde.
    let value = BindableValue::Signal(SignalOrFloat::Static(0.5));
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(json, r#"{"signal":0.5}"#);
}

// <FILE>tui-vfx-compositor/tests/types/test_bindable_value.rs</FILE> - <DESC>Tests for BindableValue evaluate, conversions, and serde</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
