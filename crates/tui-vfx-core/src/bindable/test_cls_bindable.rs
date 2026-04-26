// <FILE>crates/tui-vfx-core/src/bindable/test_cls_bindable.rs</FILE> - <DESC>Peer tests for the generic VfxBindable<T, S> envelope and the three concrete aliases.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.A — peer tests covering construction, lenient deserialization (bare T + bare S + tagged), default, ConfigSchema with and without the Signal arm, and the per-type inherent evaluate signatures via a mock RuntimeParamsRead.</WCTX>
// <CLOG>0.1.0: cover Literal/Binding/Signal construction, From<T> blanket and From<SignalOrFloat> Static-collapse, Default = Literal(T::default()), serde tagged + bare lenient + roundtrip-normalizes-to-tagged, ConfigSchema variant counts (2 for Infallible, 3 for SignalOrFloat), and the three specialized evaluate signatures.</CLOG>

//! Peer tests for [`super::cls_bindable`].

use std::collections::BTreeMap;

use mixed_signals::traits::SignalContext;
use mixed_signals::types::SignalOrFloat;

use crate::schema::{ConfigSchema, SchemaNode};

use super::{RuntimeParamsRead, VfxBindable, VfxBindableString, VfxBindableU16, VfxBindableValue};

/// In-test mock for the runtime-params surface: a flat string-keyed map of
/// strongly-typed buckets. Real consumers (`ShaderRuntimeParams`) implement
/// the same trait against richer storage.
#[derive(Default)]
struct MockRuntimeParams {
    u16s: BTreeMap<String, u16>,
    texts: BTreeMap<String, String>,
    f32s: BTreeMap<String, f32>,
}

impl RuntimeParamsRead for MockRuntimeParams {
    fn get_u16(&self, key: &str) -> Option<u16> {
        self.u16s.get(key).copied()
    }
    fn get_text(&self, key: &str) -> Option<&str> {
        self.texts.get(key).map(String::as_str)
    }
    fn get_f32(&self, key: &str) -> Option<f32> {
        self.f32s.get(key).copied()
    }
}

fn empty_signal_ctx() -> SignalContext {
    SignalContext::for_loop(0.0, 0)
}

// ---- Construction & From<T> blanket ----

#[test]
fn literal_constructs_for_each_alias() {
    let _: VfxBindableU16 = VfxBindable::Literal(20);
    let _: VfxBindableString = VfxBindable::Literal("line-3x3".to_string());
    let _: VfxBindableValue = VfxBindable::Literal(0.5);
}

#[test]
fn binding_constructs_for_each_alias() {
    let _: VfxBindableU16 = VfxBindable::Binding("hovered_col".to_string());
    let _: VfxBindableString = VfxBindable::Binding("drum_font".to_string());
    let _: VfxBindableValue = VfxBindable::Binding("progress".to_string());
}

#[test]
fn from_t_produces_literal_via_blanket_impl() {
    let v: VfxBindableU16 = 42_u16.into();
    assert_eq!(v, VfxBindable::Literal(42));

    let v: VfxBindableString = "line-3x3".to_string().into();
    assert_eq!(v, VfxBindable::Literal("line-3x3".to_string()));

    let v: VfxBindableValue = 0.5_f32.into();
    assert_eq!(v, VfxBindable::Literal(0.5));
}

#[test]
fn from_signal_or_float_collapses_static_to_literal() {
    let v: VfxBindableValue = SignalOrFloat::Static(0.25).into();
    assert_eq!(v, VfxBindable::Literal(0.25));
}

#[test]
fn static_f32_constructor_produces_literal() {
    let v = VfxBindableValue::static_f32(0.7);
    assert_eq!(v, VfxBindable::Literal(0.7));
}

// ---- Default ----

#[test]
fn default_is_literal_t_default() {
    assert_eq!(VfxBindableU16::default(), VfxBindable::Literal(0));
    assert_eq!(VfxBindableString::default(), VfxBindable::Literal(String::new()));
    assert_eq!(VfxBindableValue::default(), VfxBindable::Literal(0.0));
}

// ---- Serde: tagged shapes parse on every alias ----

#[test]
fn deserializes_tagged_literal_u16() {
    let parsed: VfxBindableU16 = serde_json::from_str(r#"{"literal":20}"#).unwrap();
    assert_eq!(parsed, VfxBindable::Literal(20));
}

#[test]
fn deserializes_tagged_binding_u16() {
    let parsed: VfxBindableU16 = serde_json::from_str(r#"{"binding":"hovered_col"}"#).unwrap();
    assert_eq!(parsed, VfxBindable::Binding("hovered_col".to_string()));
}

#[test]
fn deserializes_tagged_literal_string() {
    let parsed: VfxBindableString = serde_json::from_str(r#"{"literal":"line-3x3"}"#).unwrap();
    assert_eq!(parsed, VfxBindable::Literal("line-3x3".to_string()));
}

#[test]
fn deserializes_tagged_binding_string() {
    let parsed: VfxBindableString = serde_json::from_str(r#"{"binding":"drum_font"}"#).unwrap();
    assert_eq!(parsed, VfxBindable::Binding("drum_font".to_string()));
}

#[test]
fn deserializes_tagged_literal_f32() {
    let parsed: VfxBindableValue = serde_json::from_str(r#"{"literal":0.5}"#).unwrap();
    assert_eq!(parsed, VfxBindable::Literal(0.5));
}

#[test]
fn deserializes_tagged_binding_f32() {
    let parsed: VfxBindableValue = serde_json::from_str(r#"{"binding":"progress"}"#).unwrap();
    assert_eq!(parsed, VfxBindable::Binding("progress".to_string()));
}

#[test]
fn deserializes_tagged_signal_for_signal_value() {
    // Signal arm carries SignalOrFloat. `{"signal": 0.5}` reads as
    // `Signal(SignalOrFloat::Static(0.5))` because SignalOrFloat is
    // untagged and accepts a bare number for its Static variant.
    let parsed: VfxBindableValue = serde_json::from_str(r#"{"signal":0.5}"#).unwrap();
    assert_eq!(parsed, VfxBindable::Signal(SignalOrFloat::Static(0.5)));
}

// ---- Serde: lenient bare forms ----

#[test]
fn deserializes_bare_u16_as_literal() {
    let parsed: VfxBindableU16 = serde_json::from_str("20").unwrap();
    assert_eq!(parsed, VfxBindable::Literal(20));
}

#[test]
fn deserializes_bare_string_as_literal() {
    let parsed: VfxBindableString = serde_json::from_str(r#""line-3x3""#).unwrap();
    assert_eq!(parsed, VfxBindable::Literal("line-3x3".to_string()));
}

#[test]
fn deserializes_bare_f32_as_literal() {
    let parsed: VfxBindableValue = serde_json::from_str("0.75").unwrap();
    assert_eq!(parsed, VfxBindable::Literal(0.75));
}

// ---- Serde: roundtrip normalizes to tagged output ----

#[test]
fn roundtrip_bare_u16_normalizes_to_tagged() {
    let parsed: VfxBindableU16 = serde_json::from_str("42").unwrap();
    let json = serde_json::to_string(&parsed).unwrap();
    assert_eq!(json, r#"{"literal":42}"#);
}

#[test]
fn roundtrip_bare_string_normalizes_to_tagged() {
    let parsed: VfxBindableString = serde_json::from_str(r#""hello""#).unwrap();
    let json = serde_json::to_string(&parsed).unwrap();
    assert_eq!(json, r#"{"literal":"hello"}"#);
}

#[test]
fn roundtrip_bare_f32_normalizes_to_tagged() {
    let parsed: VfxBindableValue = serde_json::from_str("0.5").unwrap();
    let json = serde_json::to_string(&parsed).unwrap();
    assert_eq!(json, r#"{"literal":0.5}"#);
}

#[test]
fn binding_serializes_as_tagged_form() {
    let v: VfxBindableU16 = VfxBindable::Binding("x".to_string());
    let json = serde_json::to_string(&v).unwrap();
    assert_eq!(json, r#"{"binding":"x"}"#);
}

// ---- Serde: tagged signal arm cannot be deserialized for non-signal types ----

#[test]
fn vfx_bindable_u16_rejects_signal_shape() {
    // S = Infallible: `{"signal": ...}` cannot deserialize. We don't
    // assert the error message — only that deserialization fails. (No
    // panic, no spurious match; serde returns an Err.)
    let err = serde_json::from_str::<VfxBindableU16>(r#"{"signal":42}"#);
    assert!(err.is_err(), "expected error, got {err:?}");
}

// ---- ConfigSchema: variant count reflects S's inhabitedness ----

#[test]
fn schema_omits_signal_variant_for_never() {
    let schema = VfxBindableU16::schema();
    let SchemaNode::Enum { variants, .. } = schema else {
        panic!("expected Enum schema, got {schema:?}");
    };
    assert_eq!(variants.len(), 2);
    let names: Vec<&str> = variants
        .iter()
        .map(|v| match v {
            crate::schema::SchemaVariant::Tuple { name, .. } => name.as_str(),
            crate::schema::SchemaVariant::Struct { name, .. } => name.as_str(),
            crate::schema::SchemaVariant::Unit { name, .. } => name.as_str(),
        })
        .collect();
    assert!(names.contains(&"Literal"));
    assert!(names.contains(&"Binding"));
    assert!(!names.contains(&"Signal"));
}

#[test]
fn schema_includes_signal_variant_for_signal_or_float() {
    let schema = VfxBindableValue::schema();
    let SchemaNode::Enum { variants, .. } = schema else {
        panic!("expected Enum schema, got {schema:?}");
    };
    assert_eq!(variants.len(), 3);
    let names: Vec<&str> = variants
        .iter()
        .map(|v| match v {
            crate::schema::SchemaVariant::Tuple { name, .. } => name.as_str(),
            crate::schema::SchemaVariant::Struct { name, .. } => name.as_str(),
            crate::schema::SchemaVariant::Unit { name, .. } => name.as_str(),
        })
        .collect();
    assert!(names.contains(&"Literal"));
    assert!(names.contains(&"Binding"));
    assert!(names.contains(&"Signal"));
}

// ---- Specialized evaluate methods ----

#[test]
fn u16_evaluate_literal_returns_value() {
    let v: VfxBindableU16 = VfxBindable::Literal(20);
    let rp = MockRuntimeParams::default();
    assert_eq!(v.evaluate(&rp), Some(20));
}

#[test]
fn u16_evaluate_binding_resolves_from_runtime_params() {
    let v: VfxBindableU16 = VfxBindable::Binding("hovered_col".to_string());
    let mut rp = MockRuntimeParams::default();
    rp.u16s.insert("hovered_col".to_string(), 37);
    assert_eq!(v.evaluate(&rp), Some(37));
}

#[test]
fn u16_evaluate_missing_binding_returns_none() {
    let v: VfxBindableU16 = VfxBindable::Binding("missing".to_string());
    let rp = MockRuntimeParams::default();
    assert_eq!(v.evaluate(&rp), None);
}

#[test]
fn u16_literal_accessor() {
    let lit: VfxBindableU16 = VfxBindable::Literal(20);
    assert_eq!(lit.literal(), Some(20));

    let bind: VfxBindableU16 = VfxBindable::Binding("x".to_string());
    assert_eq!(bind.literal(), None);
}

#[test]
fn string_evaluate_literal_borrows_from_self() {
    let v: VfxBindableString = VfxBindable::Literal("line-3x3".to_string());
    let rp = MockRuntimeParams::default();
    assert_eq!(v.evaluate(&rp), Some("line-3x3"));
}

#[test]
fn string_evaluate_binding_borrows_from_runtime_params() {
    let v: VfxBindableString = VfxBindable::Binding("drum_font".to_string());
    let mut rp = MockRuntimeParams::default();
    rp.texts
        .insert("drum_font".to_string(), "dejavu-bold".to_string());
    assert_eq!(v.evaluate(&rp), Some("dejavu-bold"));
}

#[test]
fn string_binding_key_accessor() {
    let bind: VfxBindableString = VfxBindable::Binding("drum_font".to_string());
    assert_eq!(bind.binding_key(), Some("drum_font"));

    let lit: VfxBindableString = VfxBindable::Literal("line-3x3".to_string());
    assert_eq!(lit.binding_key(), None);
}

#[test]
fn value_evaluate_literal() {
    let v: VfxBindableValue = VfxBindable::Literal(0.5);
    let ctx = empty_signal_ctx();
    let rp = MockRuntimeParams::default();
    assert_eq!(v.evaluate(0.0, &ctx, &rp), Some(0.5));
}

#[test]
fn value_evaluate_binding() {
    let v: VfxBindableValue = VfxBindable::Binding("progress".to_string());
    let mut rp = MockRuntimeParams::default();
    rp.f32s.insert("progress".to_string(), 0.75);
    let ctx = empty_signal_ctx();
    assert_eq!(v.evaluate(0.0, &ctx, &rp), Some(0.75));
}

#[test]
fn value_evaluate_signal_static_returns_inner() {
    let v: VfxBindableValue = VfxBindable::Signal(SignalOrFloat::Static(0.5));
    let ctx = empty_signal_ctx();
    let rp = MockRuntimeParams::default();
    assert_eq!(v.evaluate(0.0, &ctx, &rp), Some(0.5));
}

#[test]
fn value_binding_ignores_loop_t() {
    // Binding lookups bypass the signal context — varying loop_t cannot
    // affect the result.
    let v: VfxBindableValue = VfxBindable::Binding("progress".to_string());
    let mut rp = MockRuntimeParams::default();
    rp.f32s.insert("progress".to_string(), 0.25);
    let ctx = empty_signal_ctx();
    assert_eq!(v.evaluate(0.0, &ctx, &rp), Some(0.25));
    assert_eq!(v.evaluate(0.5, &ctx, &rp), Some(0.25));
    assert_eq!(v.evaluate(0.99, &ctx, &rp), Some(0.25));
}

// ---- Type-system check: Infallible Signal arm is uninhabited ----

#[test]
fn never_signal_arm_is_uninhabited() {
    // `VfxBindableU16::Signal(_)` cannot be constructed because `Never`
    // has no inhabitants. A match on a real value never sees the Signal
    // arm; this test exists as a documentation anchor that the match
    // remains exhaustive without a body for that arm.
    fn _accepts_only_two_arms(b: &VfxBindableU16) -> &'static str {
        match b {
            VfxBindable::Literal(_) => "literal",
            VfxBindable::Binding(_) => "binding",
            VfxBindable::Signal(never) => match *never {},
        }
    }
}

// <FILE>crates/tui-vfx-core/src/bindable/test_cls_bindable.rs</FILE> - <DESC>Peer tests for the generic VfxBindable<T, S> envelope and the three concrete aliases.</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
