// <FILE>crates/tui-vfx-compost/src/source/fnc_text_source_grid_from_inputs.rs</FILE> - <DESC>Create source.text grid from canonical v3.1 inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>source.text materialization keeps plain text surfaces separate from card sources.</WCTX>
// <CLOG>0.1.0: INIT — render source.text message, optional bounds, colors, and bold styling.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{SourceInputId, ValueSource};
use tui_vfx_types::{Cell, Color, Grid, Modifiers, OwnedGrid, RoleMap, RoleTag, SemanticScene};

use crate::RenderError;
use crate::runtime::RuntimeContext;

use super::col_literal_source_input::{
    literal_integer, literal_text, optional_bool, optional_color,
};

pub(crate) fn text_source_grid_from_inputs(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    context: &RuntimeContext,
) -> Result<SemanticScene, RenderError> {
    let message = literal_text(inputs, "message", context)?;
    let width = optional_dimension(inputs, "width", inferred_width(&message), context)?;
    let height = optional_dimension(inputs, "height", inferred_height(&message), context)?;
    let foreground = optional_color(inputs, "foreground", Color::WHITE, context)?;
    let background = optional_color(inputs, "background", Color::TRANSPARENT, context)?;
    let modifiers = if optional_bool(inputs, "bold", false, context)? {
        Modifiers::bold()
    } else {
        Modifiers::NONE
    };

    let mut grid = OwnedGrid::new(width, height);
    let mut roles = RoleMap::new_with_default(width as u16, height as u16, RoleTag::Background);
    for (y, line) in message.lines().take(height).enumerate() {
        for (x, ch) in line.chars().take(width).enumerate() {
            grid.set(x, y, Cell::styled(ch, foreground, background, modifiers));
            roles.set((x as u16, y as u16), RoleTag::Text);
        }
    }
    Ok(SemanticScene::new(grid, roles))
}

fn optional_dimension(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    id: &str,
    default: usize,
    context: &RuntimeContext,
) -> Result<usize, RenderError> {
    if inputs.contains_key(&SourceInputId::new(id)) {
        literal_integer(inputs, id, context)
    } else {
        Ok(default)
    }
}

fn inferred_width(message: &str) -> usize {
    message
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
        .max(1)
}

fn inferred_height(message: &str) -> usize {
    message.lines().count().max(1)
}

// <FILE>crates/tui-vfx-compost/src/source/fnc_text_source_grid_from_inputs.rs</FILE> - <DESC>Create source.text grid from canonical v3.1 inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
