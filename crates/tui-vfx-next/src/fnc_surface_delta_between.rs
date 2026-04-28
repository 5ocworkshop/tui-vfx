// <FILE>crates/tui-vfx-next/src/fnc_surface_delta_between.rs</FILE> - <DESC>Capture channel deltas between two surfaces</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: turn proof node output into channel-aware deltas.</WCTX>
// <CLOG>0.1.0: INIT — compare before/after surfaces by channel.</CLOG>

use crate::{CellChannel, CellChannelWrite, CellDelta, NodeId, Surface, SurfaceDelta};

pub(crate) fn surface_delta_between(
    before: &Surface,
    after: &Surface,
    node: &NodeId,
) -> SurfaceDelta {
    let mut delta = SurfaceDelta::new();
    for y in 0..after.height() {
        for x in 0..after.width() {
            let Some(before_cell) = before.cell(x, y) else {
                continue;
            };
            let Some(after_cell) = after.cell(x, y) else {
                continue;
            };
            push_cell_deltas(&mut delta, x, y, before_cell, after_cell, node);
            if before.role(x, y) != after.role(x, y) {
                let role = after.role(x, y).expect("after role is in bounds").clone();
                delta.set(CellDelta {
                    x,
                    y,
                    channel: CellChannel::Role,
                    node: node.clone(),
                    write: CellChannelWrite::Role(role),
                });
            }
        }
    }
    delta
}

fn push_cell_deltas(
    delta: &mut SurfaceDelta,
    x: usize,
    y: usize,
    before: &tui_vfx_types::Cell,
    after: &tui_vfx_types::Cell,
    node: &NodeId,
) {
    if before.ch != after.ch {
        delta.set(cell_delta(
            x,
            y,
            CellChannel::Glyph,
            node,
            CellChannelWrite::Glyph(after.ch),
        ));
    }
    if before.fg != after.fg {
        delta.set(cell_delta(
            x,
            y,
            CellChannel::Foreground,
            node,
            CellChannelWrite::Foreground(after.fg),
        ));
    }
    if before.bg != after.bg {
        delta.set(cell_delta(
            x,
            y,
            CellChannel::Background,
            node,
            CellChannelWrite::Background(after.bg),
        ));
    }
    if before.mods != after.mods {
        delta.set(cell_delta(
            x,
            y,
            CellChannel::Modifiers,
            node,
            CellChannelWrite::Modifiers(after.mods),
        ));
    }
    if before.mod_alpha != after.mod_alpha {
        delta.set(cell_delta(
            x,
            y,
            CellChannel::ModifierAlpha,
            node,
            CellChannelWrite::ModifierAlpha(after.mod_alpha),
        ));
    }
}

fn cell_delta(
    x: usize,
    y: usize,
    channel: CellChannel,
    node: &NodeId,
    write: CellChannelWrite,
) -> CellDelta {
    CellDelta {
        x,
        y,
        channel,
        node: node.clone(),
        write,
    }
}

// <FILE>crates/tui-vfx-next/src/fnc_surface_delta_between.rs</FILE> - <DESC>Capture channel deltas between two surfaces</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
