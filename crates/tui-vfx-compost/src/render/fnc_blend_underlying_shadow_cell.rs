// <FILE>crates/tui-vfx-compost/src/render/fnc_blend_underlying_shadow_cell.rs</FILE> - <DESC>Blend shadow color while preserving destination glyph cells</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Lift the mature destination-preserving shadow blend from tui-vfx-compositor for preserveDestination shadow material.</WCTX>
// <CLOG>0.1.0: INIT — add glyph-preserving source-over shadow blending.</CLOG>

use tui_vfx_types::{Cell, Color};

pub(crate) fn blend_underlying_shadow_cell(shadow_cell: &Cell, dest_cell: &Cell) -> Cell {
    let shadow_color = shadow_blend_color(shadow_cell);
    if shadow_color.a == 0 {
        return *dest_cell;
    }

    Cell {
        ch: dest_cell.ch,
        fg: dest_cell.fg,
        bg: shadow_color.blend_over(dest_cell.bg),
        mods: dest_cell.mods,
        mod_alpha: dest_cell.mod_alpha,
    }
}

fn shadow_blend_color(shadow_cell: &Cell) -> Color {
    if shadow_cell.bg.a > 0 {
        shadow_cell.bg
    } else if shadow_cell.fg.a > 0 {
        shadow_cell.fg
    } else {
        Color::TRANSPARENT
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_blend_underlying_shadow_cell.rs</FILE> - <DESC>Blend shadow color while preserving destination glyph cells</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
