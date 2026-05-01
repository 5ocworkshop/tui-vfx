// <FILE>crates/tui-vfx-compost/src/source/fnc_source_grid_from_inputs.rs</FILE> - <DESC>Create source grid from canonical v3.1 source inputs</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Materialize load-validated source.card text into a bounded grid while preserving line boundaries.</WCTX>
// <CLOG>0.2.1: PATCH — read literal source helpers from their owning module.
// 0.2.0: PATCH — preserve source.card message line boundaries.
// 0.1.0: INIT — add source text grid builder.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{SourceInputId, ValueSource};
use tui_vfx_types::{Cell, Grid, Modifiers, OwnedGrid};

use crate::RenderError;

use super::col_literal_source_input::{literal_color, literal_integer, literal_text};

pub(crate) fn source_grid_from_inputs(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
) -> Result<OwnedGrid, RenderError> {
    let width = literal_integer(inputs, "width")?;
    let height = literal_integer(inputs, "height")?;
    let message = literal_text(inputs, "message")?;
    let foreground = literal_color(inputs, "foreground")?;
    let background = literal_color(inputs, "background")?;

    let mut grid = OwnedGrid::new(width, height);
    for (y, line) in message.lines().take(height).enumerate() {
        for (x, ch) in line.chars().take(width).enumerate() {
            grid.set(
                x,
                y,
                Cell::styled(ch, foreground, background, Modifiers::NONE),
            );
        }
    }
    Ok(grid)
}

// <FILE>crates/tui-vfx-compost/src/source/fnc_source_grid_from_inputs.rs</FILE> - <DESC>Create source grid from canonical v3.1 source inputs</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
