// <FILE>crates/tui-vfx-next/src/fnc_apply_surface_delta.rs</FILE> - <DESC>Apply proof channel deltas to a surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: merge branch deltas channel-by-channel.</WCTX>
// <CLOG>0.1.0: INIT — write individual cell channels without whole-cell overwrite.</CLOG>

use crate::{CellChannelWrite, CellDelta, Surface};

pub(crate) fn apply_surface_delta(surface: &mut Surface, delta: &CellDelta) {
    match &delta.write {
        CellChannelWrite::Glyph(value) => update_cell(surface, delta.x, delta.y, |cell| {
            cell.ch = *value;
        }),
        CellChannelWrite::Foreground(value) => update_cell(surface, delta.x, delta.y, |cell| {
            cell.fg = *value;
        }),
        CellChannelWrite::Background(value) => update_cell(surface, delta.x, delta.y, |cell| {
            cell.bg = *value;
        }),
        CellChannelWrite::Modifiers(value) => update_cell(surface, delta.x, delta.y, |cell| {
            cell.mods = *value;
        }),
        CellChannelWrite::ModifierAlpha(value) => {
            update_cell(surface, delta.x, delta.y, |cell| cell.mod_alpha = *value)
        }
        CellChannelWrite::Role(value) => surface.set_role(delta.x, delta.y, value.clone()),
    }
}

fn update_cell(
    surface: &mut Surface,
    x: usize,
    y: usize,
    update: impl FnOnce(&mut tui_vfx_types::Cell),
) {
    let Some(current) = surface.cell(x, y).copied() else {
        return;
    };
    let mut cell = current;
    update(&mut cell);
    surface.set_cell(x, y, cell);
}

// <FILE>crates/tui-vfx-next/src/fnc_apply_surface_delta.rs</FILE> - <DESC>Apply proof channel deltas to a surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
