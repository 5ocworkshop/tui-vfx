// <FILE>crates/tui-vfx-compost/src/source/fnc_source_grid_from_inputs.rs</FILE> - <DESC>Create source grid from canonical v3.1 source inputs</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Materialize load-validated source.card text into a bounded grid while preserving line boundaries.</WCTX>
// <CLOG>0.5.0: MINOR — render source.card border styles and tag border cells with Border roles.
// 0.4.0: MINOR — apply source.card descriptor color defaults when optional inputs are omitted.
// 0.3.0: MINOR — return a semantic source surface with text roles for materialized message cells.
// 0.2.1: PATCH — read literal source helpers from their owning module.
// 0.2.0: PATCH — preserve source.card message line boundaries.
// 0.1.0: INIT — add source text grid builder.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{SourceInputId, StructuredValue, ValueSource};
use tui_vfx_types::{Cell, Color, Grid, Modifiers, OwnedGrid, RoleMap, RoleTag, SemanticScene};

use crate::RenderError;
use crate::runtime::RuntimeContext;

use super::col_literal_source_input::{
    literal_integer, literal_text, optional_color, optional_enum, optional_structured,
};

pub(crate) fn source_grid_from_inputs(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    context: &RuntimeContext,
) -> Result<SemanticScene, RenderError> {
    let width = literal_integer(inputs, "width", context)?;
    let height = literal_integer(inputs, "height", context)?;
    let message = literal_text(inputs, "message", context)?;
    let foreground = optional_color(inputs, "foreground", Color::WHITE, context)?;
    let background = optional_color(inputs, "background", Color::TRANSPARENT, context)?;
    let border_style = optional_enum(inputs, "borderStyle", "none", context)?;
    let border_config = optional_structured(inputs, "borderConfig", context)?;

    let mut grid = OwnedGrid::new(width, height);
    let mut roles = RoleMap::new_with_default(width as u16, height as u16, RoleTag::Background);
    let content_origin = if border_style == "none" {
        (0, 0)
    } else {
        fill_card_surface(&mut grid, &mut roles, foreground, background);
        render_border(
            &mut grid,
            &mut roles,
            &border_style,
            border_config.as_ref(),
            foreground,
            background,
        );
        (1, 1)
    };
    let border_padding = usize::from(border_style != "none");
    let content_width = width.saturating_sub(content_origin.0 + border_padding);
    let content_height = height.saturating_sub(content_origin.1 + border_padding);
    for (y, line) in message.lines().take(content_height).enumerate() {
        for (x, ch) in line.chars().take(content_width).enumerate() {
            let grid_x = content_origin.0 + x;
            let grid_y = content_origin.1 + y;
            grid.set(
                grid_x,
                grid_y,
                Cell::styled(ch, foreground, background, Modifiers::NONE),
            );
            roles.set((grid_x as u16, grid_y as u16), RoleTag::Text);
        }
    }
    Ok(SemanticScene::new(grid, roles))
}

fn fill_card_surface(
    grid: &mut OwnedGrid,
    roles: &mut RoleMap,
    foreground: Color,
    background: Color,
) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            grid.set(
                x,
                y,
                Cell::styled(' ', foreground, background, Modifiers::NONE),
            );
            roles.set((x as u16, y as u16), RoleTag::Background);
        }
    }
}

fn render_border(
    grid: &mut OwnedGrid,
    roles: &mut RoleMap,
    style: &str,
    config: Option<&StructuredValue>,
    foreground: Color,
    background: Color,
) {
    if grid.width() < 2 || grid.height() < 2 {
        return;
    }
    let glyphs = border_glyphs(style, config);
    let max_x = grid.width() - 1;
    let max_y = grid.height() - 1;
    set_border_cell(grid, roles, 0, 0, glyphs.top_left, foreground, background);
    set_border_cell(
        grid,
        roles,
        max_x,
        0,
        glyphs.top_right,
        foreground,
        background,
    );
    set_border_cell(
        grid,
        roles,
        0,
        max_y,
        glyphs.bottom_left,
        foreground,
        background,
    );
    set_border_cell(
        grid,
        roles,
        max_x,
        max_y,
        glyphs.bottom_right,
        foreground,
        background,
    );
    for x in 1..max_x {
        set_border_cell(grid, roles, x, 0, glyphs.top, foreground, background);
        set_border_cell(grid, roles, x, max_y, glyphs.bottom, foreground, background);
    }
    for y in 1..max_y {
        set_border_cell(grid, roles, 0, y, glyphs.left, foreground, background);
        set_border_cell(grid, roles, max_x, y, glyphs.right, foreground, background);
    }
}

fn set_border_cell(
    grid: &mut OwnedGrid,
    roles: &mut RoleMap,
    x: usize,
    y: usize,
    ch: char,
    foreground: Color,
    background: Color,
) {
    grid.set(
        x,
        y,
        Cell::styled(ch, foreground, background, Modifiers::NONE),
    );
    roles.set((x as u16, y as u16), RoleTag::Border);
}

#[derive(Clone, Copy)]
struct BorderGlyphs {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    top: char,
    right: char,
    bottom: char,
    left: char,
}

fn border_glyphs(style: &str, config: Option<&StructuredValue>) -> BorderGlyphs {
    match style {
        "rounded" => BorderGlyphs {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            top: '─',
            right: '│',
            bottom: '─',
            left: '│',
        },
        "double" => BorderGlyphs {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            top: '═',
            right: '║',
            bottom: '═',
            left: '║',
        },
        "custom" => custom_border_glyphs(config).unwrap_or_else(plain_border_glyphs),
        "plain" => plain_border_glyphs(),
        _ => plain_border_glyphs(),
    }
}

fn plain_border_glyphs() -> BorderGlyphs {
    BorderGlyphs {
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
        top: '─',
        right: '│',
        bottom: '─',
        left: '│',
    }
}

fn custom_border_glyphs(config: Option<&StructuredValue>) -> Option<BorderGlyphs> {
    let StructuredValue::Object(root) = config? else {
        return None;
    };
    let StructuredValue::Object(frame) = root.get("frame")? else {
        return None;
    };
    let corners = structured_string_array(frame.get("corners")?)?;
    let edges = structured_string_array(frame.get("edges")?)?;
    Some(BorderGlyphs {
        top_left: first_char(corners.first()?)?,
        top_right: first_char(corners.get(1)?)?,
        bottom_left: first_char(corners.get(2)?)?,
        bottom_right: first_char(corners.get(3)?)?,
        top: first_char(edges.first()?)?,
        right: first_char(edges.get(1)?)?,
        bottom: first_char(edges.get(2)?)?,
        left: first_char(edges.get(3)?)?,
    })
}

fn structured_string_array(value: &StructuredValue) -> Option<Vec<&str>> {
    let StructuredValue::Array(values) = value else {
        return None;
    };
    values
        .iter()
        .map(|value| match value {
            StructuredValue::String(text) => Some(text.as_str()),
            _ => None,
        })
        .collect()
}

fn first_char(text: &str) -> Option<char> {
    text.chars().next()
}

// <FILE>crates/tui-vfx-compost/src/source/fnc_source_grid_from_inputs.rs</FILE> - <DESC>Create source grid from canonical v3.1 source inputs</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
