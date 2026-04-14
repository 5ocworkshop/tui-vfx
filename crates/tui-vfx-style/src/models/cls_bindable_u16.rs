// <FILE>tui-vfx-style/src/models/cls_bindable_u16.rs</FILE> - <DESC>Bindable u16 value for cell coordinates and runtime-parameter position bindings</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 P0.2 — position_binding on cell regions for sliding hover indicators</WCTX>
// <CLOG>Initial BindableU16 enum with Literal/Binding variants, evaluate(), literal(), From<u16>, lenient serde, ConfigSchema</CLOG>

//! # BindableU16
//!
//! A u16 coordinate value that resolves either to a literal integer or to a
//! named runtime parameter looked up from [`ShaderRuntimeParams`] at render
//! time. The sibling of [`crate::models::BindableValue`] in spirit, but scoped
//! to integer coordinate space rather than float signal math.
//!
//! Used by [`StyleRegion::Cell`] so the x/y of a single-cell style region can
//! be driven by a runtime parameter like `"hovered_button_x"` — the HLL modal
//! hover-bar slide-between-buttons use case that ships with the phase 0
//! binding generalization.
//!
//! ## Accepted JSON input shapes
//!
//! All three shapes deserialize into a `BindableU16`. Existing recipes that
//! emit a raw integer for a cell coordinate keep working without migration.
//!
//! ```json
//! 20
//! { "literal": 20 }
//! { "binding": "hovered_button_x" }
//! ```
//!
//! Serialization always emits the normalized tagged form
//! (`{"literal": 20}` or `{"binding": "..."}`).

use crate::traits::ShaderRuntimeParams;
use serde::{Deserialize, Serialize};
use tui_vfx_core::schema::{
    ConfigSchema, FieldMeta, ScalarValue, SchemaField, SchemaNode, SchemaVariant,
};

/// A cell-coordinate value resolved at frame-prepare time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", from = "BindableU16Repr")]
pub enum BindableU16 {
    /// A concrete coordinate literal.
    Literal(u16),
    /// A named runtime parameter looked up in [`ShaderRuntimeParams`] per frame.
    Binding(String),
}

impl BindableU16 {
    /// Evaluate this value against the current frame's runtime parameter
    /// map. Returns `None` if the binding is missing — callers typically
    /// `unwrap_or` a coordinate-specific default.
    pub fn evaluate(&self, runtime_params: &ShaderRuntimeParams) -> Option<u16> {
        match self {
            BindableU16::Literal(v) => Some(*v),
            BindableU16::Binding(key) => runtime_params.get_u16(key),
        }
    }

    /// Return the literal value if this is a [`BindableU16::Literal`] variant,
    /// else `None`. Used by synchronous code paths that do not carry a
    /// `ShaderRuntimeParams` reference; callers that need binding resolution
    /// should call [`StyleRegion::resolved`] to produce a resolved clone
    /// before reading.
    pub fn literal(&self) -> Option<u16> {
        match self {
            BindableU16::Literal(v) => Some(*v),
            BindableU16::Binding(_) => None,
        }
    }
}

impl From<u16> for BindableU16 {
    fn from(value: u16) -> Self {
        BindableU16::Literal(value)
    }
}

impl Default for BindableU16 {
    fn default() -> Self {
        BindableU16::Literal(0)
    }
}

/// Lenient on-disk representation. Accepts raw integers, `{"literal": n}`,
/// or `{"binding": "name"}`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BindableU16Repr {
    /// `{"binding": "name"}`
    Binding { binding: String },
    /// `{"literal": 20}`
    Literal { literal: u16 },
    /// Bare integer: `20`
    Bare(u16),
}

impl From<BindableU16Repr> for BindableU16 {
    fn from(repr: BindableU16Repr) -> Self {
        match repr {
            BindableU16Repr::Binding { binding } => BindableU16::Binding(binding),
            BindableU16Repr::Literal { literal } => BindableU16::Literal(literal),
            BindableU16Repr::Bare(v) => BindableU16::Literal(v),
        }
    }
}

impl ConfigSchema for BindableU16 {
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "BindableU16".to_string(),
            description: Some(
                "A u16 coordinate that is either a literal or a runtime parameter binding"
                    .to_string(),
            ),
            json_name: None,
            tag_field: None,
            variants: vec![
                SchemaVariant::Tuple {
                    name: "Literal".to_string(),
                    description: Some("A concrete integer coordinate".to_string()),
                    json_value: Some("literal".to_string()),
                    items: vec![SchemaField::new(
                        "value",
                        SchemaNode::Primitive {
                            type_name: "u16".to_string(),
                            range: None,
                        },
                        FieldMeta {
                            help: Some("Integer coordinate value".to_string()),
                            description: None,
                            default: Some(ScalarValue::number("0")),
                            range: None,
                            json_key: None,
                            optional: false,
                        },
                    )],
                },
                SchemaVariant::Tuple {
                    name: "Binding".to_string(),
                    description: Some(
                        "A named runtime parameter resolved per frame".to_string(),
                    ),
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
        let v = BindableU16::Literal(20);
        let rp = ShaderRuntimeParams::new();
        assert_eq!(v.evaluate(&rp), Some(20));
        assert_eq!(v.literal(), Some(20));
    }

    #[test]
    fn binding_resolves_from_runtime_params() {
        let v = BindableU16::Binding("hovered_button_x".to_string());
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("hovered_button_x", 37_u16);
        assert_eq!(v.evaluate(&rp), Some(37));
    }

    #[test]
    fn binding_literal_is_none() {
        let v = BindableU16::Binding("x".to_string());
        assert_eq!(v.literal(), None);
    }

    #[test]
    fn binding_missing_param_returns_none() {
        let v = BindableU16::Binding("missing".to_string());
        let rp = ShaderRuntimeParams::new();
        assert_eq!(v.evaluate(&rp), None);
    }

    #[test]
    fn from_u16_produces_literal() {
        let v: BindableU16 = 15_u16.into();
        assert_eq!(v, BindableU16::Literal(15));
    }

    #[test]
    fn default_is_zero_literal() {
        assert_eq!(BindableU16::default(), BindableU16::Literal(0));
    }

    #[test]
    fn deserializes_raw_integer() {
        let parsed: BindableU16 = serde_json::from_str("20").unwrap();
        assert_eq!(parsed, BindableU16::Literal(20));
    }

    #[test]
    fn deserializes_tagged_literal() {
        let parsed: BindableU16 = serde_json::from_str(r#"{"literal":20}"#).unwrap();
        assert_eq!(parsed, BindableU16::Literal(20));
    }

    #[test]
    fn deserializes_tagged_binding() {
        let parsed: BindableU16 =
            serde_json::from_str(r#"{"binding":"hovered_button_x"}"#).unwrap();
        assert_eq!(parsed, BindableU16::Binding("hovered_button_x".to_string()));
    }

    #[test]
    fn literal_serializes_as_tagged_form() {
        // Roundtrip: Literal -> {"literal": N}
        let v = BindableU16::Literal(20);
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#"{"literal":20}"#);
    }

    #[test]
    fn binding_serializes_as_tagged_form() {
        let v = BindableU16::Binding("x".to_string());
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#"{"binding":"x"}"#);
    }

    #[test]
    fn roundtrip_raw_input_normalizes_to_tagged_output() {
        let parsed: BindableU16 = serde_json::from_str("42").unwrap();
        let json = serde_json::to_string(&parsed).unwrap();
        assert_eq!(json, r#"{"literal":42}"#);
    }
}

// <FILE>tui-vfx-style/src/models/cls_bindable_u16.rs</FILE> - <DESC>Bindable u16 value for cell coordinates and runtime-parameter position bindings</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
