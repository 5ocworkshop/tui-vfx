// <FILE>crates/tui-vfx-player/src/fnc_apply_typewriter_content_primitive.rs</FILE> - <DESC>Apply the typewriter content primitive to text-grid rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 content adapter extraction: keep typewriter behavior isolated for OFPF cleanup.</WCTX>
// <CLOG>0.1.0: INIT — extract typewriter content primitive behavior from the content dispatcher.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest,
    fnc_resolve_effect_input::{
        resolve_effect_enum, resolve_effect_integer, resolve_effect_number,
    },
};

/// Apply typewriter reveal, cursor, and wake glyph behavior to text rows.
pub(crate) fn apply_typewriter_content_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let speed_variance = resolve_effect_number(node, request, "speedVariance", 0.0).clamp(0.0, 1.0);
    let visible_fraction =
        (request.phase_t * (speed + speed_variance * request.phase_t)).clamp(0.0, 1.0);
    let total = rows.iter().map(|row| row.chars().count()).sum::<usize>();
    let visible = (total as f64 * visible_fraction).round() as usize;
    let mut seen = 0usize;
    let cursor = resolve_effect_enum(node, request, "cursorCharacter", "▌")
        .chars()
        .next()
        .unwrap_or('▌');
    let wake_mode = resolve_effect_enum(node, request, "cursorWake", "off");
    let wake_cells = resolve_effect_integer(node, request, "wakeCells", 1).max(0) as usize;
    for row in rows {
        let mut wrote_cursor = false;
        let mut cursor_index = None;
        let mut chars = Vec::new();
        for (index, glyph) in row.chars().enumerate() {
            seen += 1;
            if seen <= visible || glyph == ' ' {
                chars.push(glyph);
            } else if !wrote_cursor {
                wrote_cursor = true;
                cursor_index = Some(index);
                chars.push(cursor);
            } else {
                chars.push(' ');
            }
        }
        if (wake_mode == "ghost" || wake_mode == "tint")
            && let Some(cursor_index) = cursor_index
        {
            let start = cursor_index.saturating_sub(wake_cells);
            for glyph in &mut chars[start..cursor_index] {
                if *glyph != ' ' {
                    *glyph = if wake_mode == "ghost" { '░' } else { '·' };
                }
            }
        }
        *row = chars.into_iter().collect();
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_typewriter_content_primitive.rs</FILE> - <DESC>Apply the typewriter content primitive to text-grid rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
