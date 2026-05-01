// <FILE>crates/tui-vfx-compositor-next/tests/types/test_bindable_value.rs</FILE> - <DESC>Regression tests pinning BindableValue's public surface against the post-1.2.A consolidation. Asserts that BindableValue (now an alias for tui_vfx_core::bindable::VfxBindableValue = VfxBindable<f32, SignalOrFloat>) preserves evaluator output, accepts the canonical wire forms, and routes static values through the Literal arm rather than Signal(SignalOrFloat::Static(_)).</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.A — these tests survived the consolidation as the cross-crate regression surface that the public alias must keep satisfying. Updated assertions reflect the unified design: bare numbers parse as Literal, From<f32> and From<SignalOrFloat::Static> collapse into Literal, and only proper SignalSpec-shaped payloads land in Signal.</WCTX>
// <CLOG>0.2.0: update assertions for the post-1.2.A semantics — bare 0.75 → Literal(0.75); From<SignalOrFloat::Static(v)> → Literal(v); Default → Literal(0.0); wire format for Literal is {"literal":<n>}, Signal serialises as {"signal":<SignalOrFloat>}. Behaviour around Binding evaluation is unchanged.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use mixed_signals::traits::SignalContext;
use tui_vfx_compositor_next::types::BindableValue;
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
fn from_f32_produces_literal() {
    // Post-1.2.A: From<f32> routes through the blanket From<T> impl on
    // VfxBindable, which constructs Literal(value). Evaluator output
    // matches the pre-consolidation behaviour.
    let value: BindableValue = 0.5_f32.into();
    let ctx = empty_ctx();
    let rp = ShaderRuntimeParams::new();

    assert_eq!(value.evaluate(0.0, &ctx, &rp), Some(0.5));
    assert_eq!(value, BindableValue::Literal(0.5));
    assert_eq!(value, BindableValue::static_f32(0.5));
}

#[test]
fn from_signal_or_float_static_collapses_to_literal() {
    // Post-1.2.A: SignalOrFloat::Static(v) collapses into Literal(v) so
    // the literal path is canonical for static values. Non-Static
    // SignalOrFloat variants stay in the Signal arm.
    let source = SignalOrFloat::Static(0.25);
    let value: BindableValue = source.into();

    assert_eq!(value, BindableValue::Literal(0.25));
}

#[test]
fn default_is_zero_literal() {
    // Post-1.2.A: Default routes through Literal(T::default()) on the
    // generic, so f32::default() = 0.0 lands in Literal. Evaluator
    // output is unchanged.
    let value = BindableValue::default();
    let ctx = empty_ctx();
    let rp = ShaderRuntimeParams::new();

    assert_eq!(value, BindableValue::Literal(0.0));
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
fn deserializes_raw_number_as_literal() {
    // Post-1.2.A: bare numbers route through the canonical Literal arm.
    // Existing recipes emitting `"progress": 0.75` continue to parse;
    // the in-memory variant is now Literal(0.75) rather than the
    // legacy Signal(SignalOrFloat::Static(0.75)).
    let parsed: BindableValue = serde_json::from_str("0.75").unwrap();
    assert_eq!(parsed, BindableValue::Literal(0.75));
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
    // Post-1.2.A: an explicitly-constructed Signal(Static(_)) still
    // serialises through the Signal arm. SignalOrFloat is untagged so
    // Static(0.5) emits as a bare 0.5 inside the `signal` tag, giving
    // `{"signal":0.5}`. Authors who want canonical static values use
    // Literal(0.5) → `{"literal":0.5}` instead.
    let value = BindableValue::Signal(SignalOrFloat::Static(0.5));
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(json, r#"{"signal":0.5}"#);
}

#[test]
fn literal_serializes_as_tagged_form() {
    // The canonical static path post-1.2.A.
    let value = BindableValue::Literal(0.5);
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(json, r#"{"literal":0.5}"#);
}

// <FILE>tui-vfx-compositor-next/tests/types/test_bindable_value.rs</FILE> - <DESC>Tests for BindableValue evaluate, conversions, and serde</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
