// <FILE>tui-vfx-style/src/models/v3/enum_vfx_cursor_behavior.rs</FILE> - <DESC>V3 cursor family behavior surface</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Packet 1.9.A.followup US-003: justify the two hand-written VfxCursorPrimary/Trail ConfigSchema impls now that the audit scanner sees qualified-path forms.</WCTX>
// <CLOG>0.1.1: PATCH — add CONFIGSCHEMA-JUSTIFICATION comments above the two hand-written ConfigSchema impls (kind=intentional-divergence-from-derive-output). Same blocker as cls_cursor_shader.rs: (u16, u16) tuple field has no workspace ConfigSchema impl. No behavior change.</CLOG>

//! V3 behavior surface for cursor shaders.
//!
//! Cursor is a special-case family because it is assembled per frame from
//! cursor paint ops rather than authored like a normal static recipe leaf.

use serde::{Deserialize, Serialize};

/// Wake/trail rendering mode for the V3 cursor family.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxCursorMode {
    /// No painting at all.
    #[default]
    Off,
    /// Tint trail cells in place.
    Tint,
    /// Tint trail cells and allow the consumer to overwrite glyphs.
    Ghost,
}

/// Flattened primary-cell op for the V3 cursor family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VfxCursorPrimary {
    /// (row, col) in local widget coordinates.
    pub position: (u16, u16),
    /// Effective primary-cell alpha in 0..=1.
    pub alpha: f32,
}

// CONFIGSCHEMA-JUSTIFICATION: intentional-divergence-from-derive-output: the (u16, u16) tuple field has no ConfigSchema impl in the workspace; hand-written impl emits a Primitive schema node as a non-fatal stand-in for the surrounding V3 cursor enum derive (same pattern as cls_cursor_shader.rs's CursorShaderPrimary/Trail).
impl tui_vfx_core::ConfigSchema for VfxCursorPrimary {
    fn schema() -> tui_vfx_core::SchemaNode {
        use tui_vfx_core::{FieldMeta, SchemaField, SchemaNode};
        SchemaNode::Struct {
            name: "VfxCursorPrimary".to_string(),
            description: Some("Flattened V3 cursor primary-cell op".to_string()),
            json_name: None,
            fields: vec![
                SchemaField::new(
                    "position",
                    SchemaNode::Primitive {
                        type_name: "(u16, u16)".to_string(),
                        range: None,
                    },
                    FieldMeta {
                        description: Some("(row, col) in local widget coordinates".to_string()),
                        ..Default::default()
                    },
                ),
                SchemaField::new("alpha", f32::schema(), FieldMeta::default()),
            ],
        }
    }
}

/// Flattened trail-cell op for the V3 cursor family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VfxCursorTrail {
    /// (row, col) in local widget coordinates.
    pub position: (u16, u16),
    /// Effective trail alpha in 0..=1.
    pub alpha: f32,
    /// Optional glyph used for ghost-mode entries.
    pub glyph: Option<String>,
}

// CONFIGSCHEMA-JUSTIFICATION: intentional-divergence-from-derive-output: the (u16, u16) tuple field has no ConfigSchema impl in the workspace; hand-written impl emits a Primitive schema node as a non-fatal stand-in (same pattern as VfxCursorPrimary above).
impl tui_vfx_core::ConfigSchema for VfxCursorTrail {
    fn schema() -> tui_vfx_core::SchemaNode {
        use tui_vfx_core::{FieldMeta, SchemaField, SchemaNode};
        SchemaNode::Struct {
            name: "VfxCursorTrail".to_string(),
            description: Some("Flattened V3 cursor trail-cell op".to_string()),
            json_name: None,
            fields: vec![
                SchemaField::new(
                    "position",
                    SchemaNode::Primitive {
                        type_name: "(u16, u16)".to_string(),
                        range: None,
                    },
                    FieldMeta {
                        description: Some("(row, col) in local widget coordinates".to_string()),
                        ..Default::default()
                    },
                ),
                SchemaField::new("alpha", f32::schema(), FieldMeta::default()),
                SchemaField::new(
                    "glyph",
                    <Option<String> as tui_vfx_core::ConfigSchema>::schema(),
                    FieldMeta {
                        optional: true,
                        ..Default::default()
                    },
                ),
            ],
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_cursor_behavior.rs</FILE> - <DESC>V3 cursor family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
