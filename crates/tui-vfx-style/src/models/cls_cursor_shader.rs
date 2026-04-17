// <FILE>tui-vfx-style/src/models/cls_cursor_shader.rs</FILE> - <DESC>CursorShader — paints primary-cell alpha and wake trail tint/ghost</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive T24: CursorShader skeleton — flat per-frame snapshot of cursor paint ops declared in tui-vfx-style to avoid a style→content dep. StyleShader impl + constructors + dispatch arrive in T25–T28.</WCTX>
// <CLOG>Initial struct + Default + companion types CursorShaderMode / CursorShaderPrimary / CursorShaderTrail</CLOG>

use super::ColorConfig;
use serde::{Deserialize, Serialize};

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

// StyleShader impl + constructor + SpatialShaderType dispatch registration
// arrive in T25–T28.

// <FILE>tui-vfx-style/src/models/cls_cursor_shader.rs</FILE> - <DESC>CursorShader — paints primary-cell alpha and wake trail tint/ghost</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
