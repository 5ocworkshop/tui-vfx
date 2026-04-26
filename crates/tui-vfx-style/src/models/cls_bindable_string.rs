// <FILE>tui-vfx-style/src/models/cls_bindable_string.rs</FILE> - <DESC>Bindable String value for runtime-parameter font / asset / locale name bindings</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Slice 6.1 of mechanical circular content cycles plan: foundational type for runtime-bindable string values (font names, asset names, locale tokens) consumed at call sites under the requires_bindings / requires_assets contract.</WCTX>
// <CLOG>0.1.0: introduce BindableString::Literal(String) | Binding(String) mirroring BindableU16's shape; lenient deserialization (bare string accepted as Literal); evaluate(&ShaderRuntimeParams) -> Option<&str> using the new as_text/get_text accessors; ConfigSchema derive; inline tests for serde roundtrip, lenient bare-string parse, binding lookup, missing-param contract, From<String>/From<&str> conversion, default.</CLOG>

//! # BindableString
//!
//! A string value that resolves either to a literal owned `String` or to a
//! named runtime parameter looked up from [`ShaderRuntimeParams`] at render
//! time. The sibling of [`crate::models::BindableU16`] (integer coordinate
//! bindings) and [`tui_vfx_compositor::types::BindableValue`] (f32 + signal
//! bindings), scoped to string-typed runtime values.
//!
//! Used at call sites that consume a runtime-bindable string — font names
//! (Phase 6 of the mechanical-cycles plan), asset names (Phase 7,
//! rocketsplash images on a rect), locale tokens (future). The recipe-side
//! declaration of which bindings exist lives in the recipe's
//! `requires_bindings` or `requires_assets` block per the binding-loopback
//! design (`docs/design/tui-vfx-binding-loopback.md`); this type is the
//! consuming field that resolves the reference at frame time.
//!
//! ## Accepted JSON input shapes
//!
//! All three shapes deserialize into a `BindableString`. A bare string is
//! treated as a literal so authors writing `"line-3x3"` get a concrete
//! literal without ceremony, the same way `BindableU16` accepts a bare
//! integer.
//!
//! ```json
//! "line-3x3"
//! { "literal": "line-3x3" }
//! { "binding": "drum_font" }
//! ```
//!
//! Serialization always emits the normalized tagged form
//! (`{"literal": "..."}` or `{"binding": "..."}`).

use crate::traits::ShaderRuntimeParams;
use serde::{Deserialize, Serialize};
use tui_vfx_core::schema::{
    ConfigSchema, FieldMeta, ScalarValue, SchemaField, SchemaNode, SchemaVariant,
};

/// A string value resolved at frame-prepare time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", from = "BindableStringRepr")]
pub enum BindableString {
    /// A concrete string literal.
    Literal(String),
    /// A named runtime parameter looked up in [`ShaderRuntimeParams`] per frame.
    Binding(String),
}

impl BindableString {
    /// Evaluate this value against the current frame's runtime parameter
    /// map. Returns `None` if the binding is missing — callers typically
    /// `unwrap_or` a kind-specific default (the project's Line 3x3 font
    /// for fonts per Intention 36, the canonical_path for assets, etc.).
    ///
    /// The returned slice borrows from either `self` (Literal variant) or
    /// `runtime_params` (Binding variant); the lifetime is the shorter of
    /// the two input borrows.
    pub fn evaluate<'a>(&'a self, runtime_params: &'a ShaderRuntimeParams) -> Option<&'a str> {
        match self {
            BindableString::Literal(value) => Some(value.as_str()),
            BindableString::Binding(key) => runtime_params.get_text(key),
        }
    }

    /// Return the literal value if this is a [`BindableString::Literal`]
    /// variant, else `None`. Used by synchronous code paths that don't
    /// carry a `ShaderRuntimeParams` reference; callers that need binding
    /// resolution should use [`evaluate`](Self::evaluate) instead.
    pub fn literal(&self) -> Option<&str> {
        match self {
            BindableString::Literal(value) => Some(value.as_str()),
            BindableString::Binding(_) => None,
        }
    }

    /// Return the binding key if this is a [`BindableString::Binding`]
    /// variant, else `None`. Useful for validator passes that need to
    /// enumerate binding references in a recipe without resolving them.
    pub fn binding_key(&self) -> Option<&str> {
        match self {
            BindableString::Binding(key) => Some(key.as_str()),
            BindableString::Literal(_) => None,
        }
    }
}

impl From<String> for BindableString {
    fn from(value: String) -> Self {
        BindableString::Literal(value)
    }
}

impl From<&str> for BindableString {
    fn from(value: &str) -> Self {
        BindableString::Literal(value.to_string())
    }
}

impl Default for BindableString {
    fn default() -> Self {
        BindableString::Literal(String::new())
    }
}

/// Lenient on-disk representation. Accepts bare strings,
/// `{"literal": "name"}`, or `{"binding": "name"}`.
///
/// Variant order matters for `serde(untagged)`: the tagged forms come
/// first so JSON objects with `literal:` or `binding:` keys hit those
/// arms, and the bare-string fallback comes last so plain strings still
/// parse.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BindableStringRepr {
    /// `{"binding": "name"}`
    Binding { binding: String },
    /// `{"literal": "name"}`
    Literal { literal: String },
    /// Bare string: `"name"`
    Bare(String),
}

impl From<BindableStringRepr> for BindableString {
    fn from(repr: BindableStringRepr) -> Self {
        match repr {
            BindableStringRepr::Binding { binding } => BindableString::Binding(binding),
            BindableStringRepr::Literal { literal } => BindableString::Literal(literal),
            BindableStringRepr::Bare(value) => BindableString::Literal(value),
        }
    }
}

impl ConfigSchema for BindableString {
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "BindableString".to_string(),
            description: Some(
                "A String value that is either a literal or a runtime parameter binding"
                    .to_string(),
            ),
            json_name: None,
            tag_field: None,
            variants: vec![
                SchemaVariant::Tuple {
                    name: "Literal".to_string(),
                    description: Some("A concrete string literal".to_string()),
                    json_value: Some("literal".to_string()),
                    items: vec![SchemaField::new(
                        "value",
                        SchemaNode::Primitive {
                            type_name: "String".to_string(),
                            range: None,
                        },
                        FieldMeta {
                            help: Some("Literal string value".to_string()),
                            description: None,
                            default: Some(ScalarValue::String(String::new())),
                            range: None,
                            json_key: None,
                            optional: false,
                        },
                    )],
                },
                SchemaVariant::Tuple {
                    name: "Binding".to_string(),
                    description: Some("A named runtime parameter resolved per frame".to_string()),
                    json_value: Some("binding".to_string()),
                    items: vec![SchemaField::new(
                        "name",
                        SchemaNode::Primitive {
                            type_name: "String".to_string(),
                            range: None,
                        },
                        FieldMeta {
                            help: Some("Runtime parameter name to look up".to_string()),
                            description: None,
                            default: None,
                            range: None,
                            json_key: None,
                            optional: false,
                        },
                    )],
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_evaluates_without_params() {
        let v = BindableString::Literal("line-3x3".to_string());
        let rp = ShaderRuntimeParams::new();
        assert_eq!(v.evaluate(&rp), Some("line-3x3"));
        assert_eq!(v.literal(), Some("line-3x3"));
        assert_eq!(v.binding_key(), None);
    }

    #[test]
    fn binding_resolves_from_runtime_params() {
        let v = BindableString::Binding("drum_font".to_string());
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("drum_font", "dejavusans-bold-20-braille".to_string());
        assert_eq!(v.evaluate(&rp), Some("dejavusans-bold-20-braille"));
        assert_eq!(v.literal(), None);
        assert_eq!(v.binding_key(), Some("drum_font"));
    }

    #[test]
    fn binding_missing_param_returns_none() {
        let v = BindableString::Binding("missing".to_string());
        let rp = ShaderRuntimeParams::new();
        assert_eq!(v.evaluate(&rp), None);
    }

    #[test]
    fn binding_evaluate_returns_none_when_param_is_wrong_type() {
        // Param exists but is a number, not Text — as_text returns None.
        let v = BindableString::Binding("not_a_string".to_string());
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("not_a_string", 42_u16);
        assert_eq!(v.evaluate(&rp), None);
    }

    #[test]
    fn from_string_produces_literal() {
        let v: BindableString = "line-3x3".to_string().into();
        assert_eq!(v, BindableString::Literal("line-3x3".to_string()));
    }

    #[test]
    fn from_str_produces_literal() {
        let v: BindableString = "line-3x3".into();
        assert_eq!(v, BindableString::Literal("line-3x3".to_string()));
    }

    #[test]
    fn default_is_empty_literal() {
        assert_eq!(
            BindableString::default(),
            BindableString::Literal(String::new())
        );
    }

    #[test]
    fn deserializes_bare_string() {
        let parsed: BindableString = serde_json::from_str(r#""line-3x3""#).unwrap();
        assert_eq!(parsed, BindableString::Literal("line-3x3".to_string()));
    }

    #[test]
    fn deserializes_tagged_literal() {
        let parsed: BindableString = serde_json::from_str(r#"{"literal":"line-3x3"}"#).unwrap();
        assert_eq!(parsed, BindableString::Literal("line-3x3".to_string()));
    }

    #[test]
    fn deserializes_tagged_binding() {
        let parsed: BindableString = serde_json::from_str(r#"{"binding":"drum_font"}"#).unwrap();
        assert_eq!(parsed, BindableString::Binding("drum_font".to_string()));
    }

    #[test]
    fn literal_serializes_as_tagged_form() {
        let v = BindableString::Literal("line-3x3".to_string());
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#"{"literal":"line-3x3"}"#);
    }

    #[test]
    fn binding_serializes_as_tagged_form() {
        let v = BindableString::Binding("drum_font".to_string());
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#"{"binding":"drum_font"}"#);
    }

    #[test]
    fn roundtrip_bare_input_normalizes_to_tagged_output() {
        let parsed: BindableString = serde_json::from_str(r#""hello""#).unwrap();
        let json = serde_json::to_string(&parsed).unwrap();
        assert_eq!(json, r#"{"literal":"hello"}"#);
    }

    #[test]
    fn empty_string_literal_round_trips() {
        let parsed: BindableString = serde_json::from_str(r#""""#).unwrap();
        assert_eq!(parsed, BindableString::Literal(String::new()));
        let json = serde_json::to_string(&parsed).unwrap();
        assert_eq!(json, r#"{"literal":""}"#);
    }

    #[test]
    fn snake_case_binding_name_round_trips() {
        // Binding names follow the snake_case convention per the cycle
        // plan v0.5.0 / Intention 37: validator and resolver lookups are
        // case-sensitive, so the convention is load-bearing.
        let parsed: BindableString = serde_json::from_str(r#"{"binding":"drum_font"}"#).unwrap();
        match parsed {
            BindableString::Binding(ref key) => assert_eq!(key, "drum_font"),
            _ => panic!("expected Binding, got {parsed:?}"),
        }
    }

    #[test]
    fn unicode_literal_round_trips() {
        let v = BindableString::Literal("éñ★日本語".to_string());
        let json = serde_json::to_string(&v).unwrap();
        let back: BindableString = serde_json::from_str(&json).unwrap();
        assert_eq!(back, v);
    }
}

// <FILE>tui-vfx-style/src/models/cls_bindable_string.rs</FILE> - <DESC>Bindable String value for runtime-parameter font / asset / locale name bindings</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
