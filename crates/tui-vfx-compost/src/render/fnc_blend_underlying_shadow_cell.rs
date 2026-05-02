// <FILE>crates/tui-vfx-compost/src/render/fnc_blend_underlying_shadow_cell.rs</FILE> - <DESC>Blend shadow color while preserving destination glyph cells</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Lift the mature destination-preserving shadow blend from tui-vfx-compositor for preserveDestination shadow material.</WCTX>
// <CLOG>0.1.0: INIT — add glyph-preserving source-over shadow blending.</CLOG>

use tui_vfx_contract::ShadowBlendMode;
use tui_vfx_types::{Cell, Color};

pub(crate) fn blend_underlying_shadow_cell(
    shadow_cell: &Cell,
    dest_cell: &Cell,
    blend_mode: ShadowBlendMode,
) -> Cell {
    let shadow_color = shadow_blend_color(shadow_cell);
    if shadow_color.a == 0 {
        return *dest_cell;
    }

    Cell {
        ch: dest_cell.ch,
        fg: dest_cell.fg,
        bg: blend_shadow_color(shadow_color, dest_cell.bg, blend_mode),
        mods: dest_cell.mods,
        mod_alpha: dest_cell.mod_alpha,
    }
}

pub(crate) fn blend_shadow_color(
    shadow_color: Color,
    destination: Color,
    blend_mode: ShadowBlendMode,
) -> Color {
    match blend_mode {
        ShadowBlendMode::SourceOver => shadow_color.blend_over(destination),
        ShadowBlendMode::Multiply => {
            let multiplied = Color::new(
                multiply_channel(destination.r, shadow_color.r),
                multiply_channel(destination.g, shadow_color.g),
                multiply_channel(destination.b, shadow_color.b),
                destination.a,
            );
            destination.lerp(multiplied, shadow_color.a as f32 / 255.0)
        }
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

fn multiply_channel(destination: u8, shadow: u8) -> u8 {
    ((u16::from(destination) * u16::from(shadow)) / 255) as u8
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_blend_underlying_shadow_cell.rs</FILE> - <DESC>Blend shadow color while preserving destination glyph cells</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
