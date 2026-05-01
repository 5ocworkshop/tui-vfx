// <FILE>crates/tui-vfx-style/src/models/v3/cls_vfx_modifier_window_shader.rs</FILE> - <DESC>V3 modifier-window shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>style.italicWindow migration — expose a Vfx-prefixed V3 wire name for the time-windowed modifier shader.</WCTX>
// <CLOG>0.1.0: INIT — alias ModifierWindowShader behind a Vfx-prefixed V3 surface.</CLOG>

use crate::models::ModifierWindowShader;

/// V3 wire-surface name for the time-windowed modifier shader.
///
/// The runtime payload is intentionally the same as [`ModifierWindowShader`]:
/// normalized inclusive `start`/`end` bounds and OR-combined text modifiers.
pub type VfxModifierWindowShader = ModifierWindowShader;

// <FILE>crates/tui-vfx-style/src/models/v3/cls_vfx_modifier_window_shader.rs</FILE> - <DESC>V3 modifier-window shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
