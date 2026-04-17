// <FILE>tui-vfx-style/src/models/cls_cursor_shader.rs</FILE> - <DESC>CursorShader — paints primary-cell alpha and wake trail tint/ghost</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>feat/cursor-primitive T26: wake Tint mode — trail cells alpha-blend the configured tint color onto the base fg via a local blend_rgb helper. ColorConfig resolves to Color via `into()` (matches the established pattern in cls_highlighter_shader).</WCTX>
// <CLOG>MINOR: add blend_rgb helper + Tint-arm trail painting; zero-alpha trail leaves base untouched; Ghost-arm renders identical tint (glyph overwrite is a content-crate helper concern — T27)</CLOG>

use super::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

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
            let blended = blend_rgb(base.fg, tint, alpha);
            return base.with_fg(blended);
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

// Constructor + SpatialShaderType dispatch registration arrive in T28.

// <FILE>tui-vfx-style/src/models/cls_cursor_shader.rs</FILE> - <DESC>CursorShader — paints primary-cell alpha and wake trail tint/ghost</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
