// <FILE>crates/tui-vfx-compost/src/render/fnc_blend_shadow_cell.rs</FILE> - <DESC>Blend rendered shadow glyph cells over scene cells</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Lift the mature glyph-overlay shadow blend from tui-vfx-compositor for native compost surface shadows.</WCTX>
// <CLOG>0.1.0: INIT — add source-over shadow cell blending.</CLOG>

use tui_vfx_contract::ShadowBlendMode;
use tui_vfx_types::Cell;

use crate::render::blend_shadow_color;

pub(crate) fn blend_shadow_cell(
    shadow_cell: &Cell,
    dest_cell: &Cell,
    blend_mode: ShadowBlendMode,
) -> Cell {
    let blended_bg = if shadow_cell.bg.a > 0 {
        blend_shadow_color(shadow_cell.bg, dest_cell.bg, blend_mode)
    } else {
        dest_cell.bg
    };

    let blended_fg = if shadow_cell.fg.a > 0 {
        blend_shadow_color(shadow_cell.fg, dest_cell.bg, blend_mode)
    } else {
        dest_cell.bg
    };

    Cell::styled(shadow_cell.ch, blended_fg, blended_bg, shadow_cell.mods)
        .with_mod_alpha(shadow_cell.mod_alpha)
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_blend_shadow_cell.rs</FILE> - <DESC>Blend rendered shadow glyph cells over scene cells</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
