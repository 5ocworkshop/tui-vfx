// <FILE>tui-vfx-style/src/models/cls_cursor_shader.rs</FILE> - <DESC>CursorShader — paints primary-cell alpha and wake trail tint/ghost</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>feat/cursor-primitive T25: StyleShader impl — primary-cell alpha modulation. Off mode short-circuits to base; non-primary cells with no trail untouched. Trail painting lives in T26–T27.</WCTX>
// <CLOG>MINOR: impl StyleShader for CursorShader handling Off short-circuit and primary-cell fg.a scaling by alpha.clamp(0,1) * 255</CLOG>

use super::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::Style;

/// Mirror of `tui_vfx_content::cursor::WakeMode`, declared in `tui-vfx-style`
/// to avoid a reverse dependency on the content crate. Consumers convert.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CursorShaderMode {
    /// No painting at all — shader short-circuits to the base style.
    Off,
    /// Trail cells tint the fg color in place via alpha-blending.
    Tint,
    /// Trail cells tint identically to Tint; glyph overwrite is a consumer
    /// responsibility (see `tui_vfx_content::cursor::fnc_apply_ghost_glyphs_to_grid`).
    Ghost,
}

impl Default for CursorShaderMode {
    fn default() -> Self {
        Self::Off
    }
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

/// Shader that paints cursor primary-cell alpha and wake trail tint/ghost.
///
/// Constructed per-frame by the consumer from a `CursorPaintOps` snapshot
/// (see `fnc_build_cursor_shader` in `tui-vfx-content`). The shader itself
/// is stateless beyond the per-frame snapshot it holds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CursorShader {
    pub mode: CursorShaderMode,
    /// Tint color applied in both Tint and Ghost modes.
    pub tint: ColorConfig,
    pub primary: Option<CursorShaderPrimary>,
    pub trail: Vec<CursorShaderTrail>,
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
        if let Some(p) = self.primary.as_ref() {
            if p.position == cell {
                let alpha = p.alpha.clamp(0.0, 1.0);
                let mut fg = base.fg;
                fg.a = (alpha * 255.0).round() as u8;
                return base.with_fg(fg);
            }
        }

        // Trail painting lives in T26-T27.
        base
    }

    fn name(&self) -> &'static str {
        "Cursor"
    }
}

// Constructor + SpatialShaderType dispatch registration arrive in T28.

// <FILE>tui-vfx-style/src/models/cls_cursor_shader.rs</FILE> - <DESC>CursorShader — paints primary-cell alpha and wake trail tint/ghost</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
