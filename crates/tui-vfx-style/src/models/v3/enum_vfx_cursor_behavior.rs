// <FILE>tui-vfx-style/src/models/v3/enum_vfx_cursor_behavior.rs</FILE> - <DESC>V3 cursor family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a grouped V3 home for the cursor shader while preserving the legacy Cursor variant for current playback and per-frame cutover wiring.</WCTX>
// <CLOG>Define the V3 cursor mode enum and flattened payload mirrors for the cursor family surface.</CLOG>

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
// <VERS>END OF VERSION: 0.1.0</VERS>
