// <FILE>crates/tui-vfx-next/src/fnc_rewrite_glyph_cell.rs</FILE> - <DESC>Build a sampled-source role-preserving glyph rewrite</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase C: keep test-helper glyph stage logic isolated under OFPF rules.</WCTX>
// <CLOG>0.1.0: ADD — rewrite matching glyphs while copying sampled-source roles.</CLOG>

use tui_vfx_types::Cell;

use crate::{CellWrite, CellWritePolicy, RoleWritePolicy};

/// Rewrite one glyph while preserving sampled-source role semantics.
pub fn rewrite_glyph_cell(sampled_cell: Cell, from: char, to: char) -> CellWrite {
    let mut cell = sampled_cell;
    if cell.ch == from {
        cell.ch = to;
    }
    CellWrite {
        cell,
        cell_policy: CellWritePolicy::WriteCell,
        role_policy: RoleWritePolicy::CopySampledSource,
    }
}

// <FILE>crates/tui-vfx-next/src/fnc_rewrite_glyph_cell.rs</FILE> - <DESC>Build a sampled-source role-preserving glyph rewrite</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
