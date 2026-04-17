// <FILE>tui-vfx-content/src/cursor/fnc_build_cursor_shader.rs</FILE> - <DESC>Build a CursorShader snapshot from CursorPaintOps + Wake config</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive T28: content-side bridge — converts a CursorPaintOps snapshot plus the authoring-side Wake config into the flat tui-vfx-style CursorShader the compositor consumes. This is the directional bridge that keeps tui-vfx-style independent of tui-vfx-content (style→content would cycle).</WCTX>
// <CLOG>Initial impl — maps WakeMode → CursorShaderMode, PrimaryOp → CursorShaderPrimary, TrailOp → CursorShaderTrail (preserving glyph Option for ghost distinction), and forwards wake.tint as the shader tint</CLOG>

use super::{CursorPaintOps, Wake, WakeMode};
use tui_vfx_style::models::{
    CursorShader, CursorShaderMode, CursorShaderPrimary, CursorShaderTrail,
};

/// Convert a [`CursorPaintOps`] snapshot + [`Wake`] config into a
/// [`CursorShader`] ready to install on a composition spec.
///
/// The shader is pure runtime state — a flat copy of the per-frame paint ops
/// plus the authoring-side wake mode / tint — so it can be rebuilt each
/// frame without touching persistent state. See spec for the full flow.
pub fn fnc_build_cursor_shader(ops: &CursorPaintOps, wake: &Wake) -> CursorShader {
    let mode = match wake.mode {
        WakeMode::Off => CursorShaderMode::Off,
        WakeMode::Tint => CursorShaderMode::Tint,
        WakeMode::Ghost => CursorShaderMode::Ghost,
    };
    let primary = ops.primary.as_ref().map(|p| CursorShaderPrimary {
        position: p.position,
        alpha: p.alpha,
    });
    let trail = ops
        .trail
        .iter()
        .map(|t| CursorShaderTrail {
            position: t.position,
            alpha: t.alpha,
            glyph: t.glyph.clone(),
        })
        .collect();
    CursorShader::new(mode, wake.tint, primary, trail)
}

// <FILE>tui-vfx-content/src/cursor/fnc_build_cursor_shader.rs</FILE> - <DESC>Build a CursorShader snapshot from CursorPaintOps + Wake config</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
