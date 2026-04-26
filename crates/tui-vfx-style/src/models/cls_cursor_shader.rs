// <FILE>tui-vfx-style/src/models/cls_cursor_shader.rs</FILE> - <DESC>CursorShader — paints primary-cell alpha and wake trail tint/ghost</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Cursor wake-trail visibility: the previous implementation only tinted base.fg, so cells without visible text showed no trail. Blend the tint onto base.bg too so the trail glows like a mouse-tail behind the cursor even on empty cells.</WCTX>
// <CLOG>MINOR: trail cells now blend the tint into both base.fg and base.bg using the same alpha; primary-cell behavior unchanged</CLOG>

use super::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Mirror of `tui_vfx_content::cursor::WakeMode`, declared in `tui-vfx-style`
/// to avoid a reverse dependency on the content crate. Consumers convert.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CursorShaderMode {
    /// No painting at all — shader short-circuits to the base style.
    #[default]
    Off,
    /// Trail cells tint the fg color in place via alpha-blending.
    Tint,
    /// Trail cells tint identically to Tint; glyph overwrite is a consumer
    /// responsibility (see `tui_vfx_content::cursor::fnc_apply_ghost_glyphs_to_grid`).
    Ghost,
}

/// Flattened primary-cell op (a cell-facing copy of `CursorPaintOps::primary`).
///
/// `position` is `(row, col)` in local widget coordinates. `alpha` is the
/// effective primary-cell visibility in `0..=1` (e.g. during grow-in).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CursorShaderPrimary {
    pub position: (u16, u16),
    pub alpha: f32,
}

// Manual ConfigSchema impl — the tui_vfx_core derive macro does not support
// `(u16, u16)` tuple fields. This shader is assembled imperatively per frame,
// not authored as recipe JSON, so the schema surface only needs to be
// "non-fatal" for the SpatialShaderType enum derive (which requires every
// variant to implement ConfigSchema).
impl tui_vfx_core::ConfigSchema for CursorShaderPrimary {
    fn schema() -> tui_vfx_core::SchemaNode {
        use tui_vfx_core::{FieldMeta, SchemaField, SchemaNode};
        SchemaNode::Struct {
            name: "CursorShaderPrimary".to_string(),
            description: Some("Flattened primary-cell op (row, col) + alpha".to_string()),
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

/// Flattened trail-cell op (a cell-facing copy of `CursorPaintOps::trail`).
///
/// `glyph = None` = Tint-mode entry (consumer paints tint on whatever is
/// beneath). `glyph = Some(_)` = Ghost-mode entry — the shader still paints
/// the tint blend; the consumer overwrites the grid glyph via
/// `fnc_apply_ghost_glyphs_to_grid`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CursorShaderTrail {
    pub position: (u16, u16),
    pub alpha: f32,
    pub glyph: Option<String>,
}

impl tui_vfx_core::ConfigSchema for CursorShaderTrail {
    fn schema() -> tui_vfx_core::SchemaNode {
        use tui_vfx_core::{FieldMeta, SchemaField, SchemaNode};
        SchemaNode::Struct {
            name: "CursorShaderTrail".to_string(),
            description: Some(
                "Flattened trail-cell op (row, col) + alpha + optional ghost glyph".to_string(),
            ),
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
                        description: Some(
                            "Some(_) for Ghost-mode entries, None for Tint-mode".to_string(),
                        ),
                        optional: true,
                        ..Default::default()
                    },
                ),
            ],
        }
    }
}

/// Shader that paints cursor primary-cell alpha and wake trail tint/ghost.
///
/// Constructed per-frame by the consumer from a `CursorPaintOps` snapshot
/// (see `fnc_build_cursor_shader` in `tui-vfx-content`). The shader itself
/// is stateless beyond the per-frame snapshot it holds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct CursorShader {
    pub mode: CursorShaderMode,
    /// Tint color applied in both Tint and Ghost modes.
    pub tint: ColorConfig,
    pub primary: Option<CursorShaderPrimary>,
    pub trail: Vec<CursorShaderTrail>,
}

impl CursorShader {
    /// Build a [`CursorShader`] from flat primary/trail data.
    ///
    /// The `tui-vfx-style` crate intentionally does *not* depend on
    /// `tui-vfx-content` (that would cycle, since content depends on
    /// style for `ColorConfig`). So `CursorShader` itself only sees flat
    /// primitives; the bridge from `tui_vfx_content::cursor::CursorPaintOps`
    /// lives in `tui_vfx_content::cursor::fnc_build_cursor_shader`.
    pub fn new(
        mode: CursorShaderMode,
        tint: ColorConfig,
        primary: Option<CursorShaderPrimary>,
        trail: Vec<CursorShaderTrail>,
    ) -> Self {
        Self {
            mode,
            tint,
            primary,
            trail,
        }
    }
}

impl StyleShader for CursorShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if matches!(self.mode, CursorShaderMode::Off) {
            return base;
        }

        // Paint-op positions use (row, col) = (y, x) convention to match
        // CursorPaintOps / PrimaryOp / TrailOp.
        let cell = (ctx.local_y, ctx.local_x);

        // Primary-cell alpha modulation (drives grow-in fade).
        if let Some(p) = self.primary.as_ref()
            && p.position == cell
        {
            let alpha = p.alpha.clamp(0.0, 1.0);
            let mut fg = base.fg;
            fg.a = (alpha * 255.0).round() as u8;
            return base.with_fg(fg);
        }

        // Wake trail: each trail cell is alpha-blended with the tint color.
        // Ghost mode renders identically to Tint at the style layer; glyph
        // overwrite is a consumer concern (see
        // `tui_vfx_content::cursor::fnc_apply_ghost_glyphs_to_grid`).
        for t in &self.trail {
            if t.position != cell {
                continue;
            }
            let alpha = t.alpha.clamp(0.0, 1.0);
            if alpha <= 0.0 {
                return base;
            }
            let tint: Color = self.tint.into();
            // Blend the tint onto BOTH fg and bg so the trail is visible even
            // on cells where no glyph is drawn — this produces the classic
            // "mouse-tail" glow behind the cursor. Using the same alpha for
            // both channels preserves the natural fade-out shape.
            let blended_fg = blend_rgb(base.fg, tint, alpha);
            let blended_bg = blend_rgb(base.bg, tint, alpha);
            return base.with_fg(blended_fg).with_bg(blended_bg);
        }

        base
    }

    fn name(&self) -> &'static str {
        "Cursor"
    }
}

/// Linear RGB blend between `base` and `tint` driven by `alpha` in `0..=1`.
///
/// Preserves the channel-wise maximum alpha so that a tint applied over a
/// transparent base still produces a visible blend.
fn blend_rgb(base: Color, tint: Color, alpha: f32) -> Color {
    let a = alpha.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| -> u8 {
        let xf = x as f32;
        let yf = y as f32;
        (xf + (yf - xf) * a).round().clamp(0.0, 255.0) as u8
    };
    Color {
        r: lerp(base.r, tint.r),
        g: lerp(base.g, tint.g),
        b: lerp(base.b, tint.b),
        a: base.a.max(tint.a),
    }
}

// SpatialShaderType::Cursor dispatch registration lives in
// cls_spatial_shader_type.rs.

// <FILE>tui-vfx-style/src/models/cls_cursor_shader.rs</FILE> - <DESC>CursorShader — paints primary-cell alpha and wake trail tint/ghost</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
