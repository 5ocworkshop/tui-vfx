// <FILE>crates/tui-vfx-compost/src/source/fnc_source_grid_from_inputs.rs</FILE> - <DESC>Create source grid from canonical v3.1 source inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Minimal source.card materialization for the first compost shader slice.</WCTX>
// <CLOG>0.1.0: INIT — add source text grid builder.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{SourceInputId, ValueSource};
use tui_vfx_types::{Cell, Grid, Modifiers, OwnedGrid};

use crate::RenderError;
use crate::source::{literal_color, literal_integer, literal_text};

pub(crate) fn source_grid_from_inputs(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
) -> Result<OwnedGrid, RenderError> {
    let width = literal_integer(inputs, "width")?;
    let height = literal_integer(inputs, "height")?;
    let message = literal_text(inputs, "message")?;
    let foreground = literal_color(inputs, "foreground")?;
    let background = literal_color(inputs, "background")?;

    let mut grid = OwnedGrid::new(width, height);
    for (index, ch) in message.chars().filter(|ch| *ch != '\n').enumerate() {
        let x = index % width;
        let y = index / width;
        if y >= height {
            break;
        }
        grid.set(
            x,
            y,
            Cell::styled(ch, foreground, background, Modifiers::NONE),
        );
    }
    Ok(grid)
}

// <FILE>crates/tui-vfx-compost/src/source/fnc_source_grid_from_inputs.rs</FILE> - <DESC>Create source grid from canonical v3.1 source inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
