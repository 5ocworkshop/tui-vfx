// <FILE>crates/tui-vfx-player/src/fnc_apply_styled_primitive.rs</FILE> - <DESC>Apply styled primitive adapters to player styled grids</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.5 styled primitive work: emit honest styled-cell evidence without legacy runtime dependencies.</WCTX>
// <CLOG>0.1.0: INIT — add deterministic color/style adapters for K2.5 primitive fixtures.</CLOG>

use tui_vfx_contract::{NodeSpec, ScopeSpec};

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_resolve_effect_input::{
        ResolvedColor, resolve_effect_color, resolve_effect_integer, resolve_effect_number,
    },
};

/// Apply a supported styled primitive effect to the styled grid.
pub(crate) fn apply_styled_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) -> bool {
    match node.effect.as_str() {
        "style.colorFade" => {
            apply_color_fade(node, request, styled_grid);
            true
        }
        "style.baseStyleOverride" => {
            apply_base_style_override(node, request, styled_grid);
            true
        }
        "shader.linearGradient" => {
            apply_linear_gradient(node, request, styled_grid);
            true
        }
        "shader.borderSweep" => {
            apply_border_sweep(node, request, styled_grid);
            true
        }
        _ => false,
    }
}

fn apply_color_fade(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let target = resolve_effect_color(node, request, "target", ResolvedColor::rgb(255, 200, 50));
    let color =
        ResolvedColor::rgb(255, 255, 255).lerp(target, request.phase_t.clamp(0.0, 1.0) as f32);
    apply_color_to_scope(
        node,
        styled_grid,
        color_label(color),
        "transparent",
        vec![],
        None,
    );
}

fn apply_base_style_override(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let foreground =
        resolve_effect_color(node, request, "foreground", ResolvedColor::rgb(0, 255, 255));
    let background =
        resolve_effect_color(node, request, "background", ResolvedColor::rgb(15, 40, 55));
    let role = role_label_for_scope(node.scope.as_ref());
    apply_color_to_scope(
        node,
        styled_grid,
        color_label(foreground),
        &color_label(background),
        vec![],
        role,
    );
}

fn apply_linear_gradient(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let start = resolve_effect_color(
        node,
        request,
        "startColor",
        ResolvedColor::rgb(255, 100, 50),
    );
    let end = resolve_effect_color(node, request, "endColor", ResolvedColor::rgb(50, 100, 255));
    let angle = resolve_effect_number(node, request, "angleDeg", 45.0).to_radians();
    let intensity = resolve_effect_number(node, request, "intensity", 1.0).clamp(0.0, 1.0) as f32;
    let max_x = styled_grid.width().saturating_sub(1).max(1) as f64;
    let max_y = styled_grid.height().saturating_sub(1).max(1) as f64;
    for (x, y) in scoped_cells(node.scope.as_ref(), styled_grid) {
        let nx = x as f64 / max_x;
        let ny = y as f64 / max_y;
        let projection = (nx * angle.cos() + ny * angle.sin() + 1.0) / 2.0;
        let color = start.lerp(end, (projection.clamp(0.0, 1.0) as f32) * intensity);
        styled_grid.set_cell_style(x, y, &color_label(color), "transparent", vec![], None);
    }
}

fn apply_border_sweep(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = color_label(resolve_effect_color(
        node,
        request,
        "color",
        ResolvedColor::rgb(0, 255, 255),
    ));
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let length = resolve_effect_integer(node, request, "length", 10).max(1) as usize;
    let edge_cells = border_cells(styled_grid);
    if edge_cells.is_empty() {
        return;
    }
    let offset = ((request.phase_t.clamp(0.0, 1.0) * speed) * edge_cells.len() as f64).round()
        as usize
        % edge_cells.len();
    for index in 0..length.min(edge_cells.len()) {
        let (x, y) = edge_cells[(offset + index) % edge_cells.len()];
        if scope_matches(node.scope.as_ref(), styled_grid, x, y) {
            styled_grid.set_cell_style(
                x,
                y,
                &color,
                "transparent",
                vec!["bold".to_string()],
                Some("Border".to_string()),
            );
        }
    }
}

fn apply_color_to_scope(
    node: &NodeSpec,
    styled_grid: &mut PlayerStyledGrid,
    foreground: String,
    background: &str,
    modifiers: Vec<String>,
    role: Option<String>,
) {
    for (x, y) in scoped_cells(node.scope.as_ref(), styled_grid) {
        styled_grid.set_cell_style(
            x,
            y,
            &foreground,
            background,
            modifiers.clone(),
            role.clone(),
        );
    }
}

fn scoped_cells(scope: Option<&ScopeSpec>, styled_grid: &PlayerStyledGrid) -> Vec<(usize, usize)> {
    (0..styled_grid.height())
        .flat_map(|y| (0..styled_grid.width()).map(move |x| (x, y)))
        .filter(|(x, y)| scope_matches(scope, styled_grid, *x, *y))
        .collect()
}

fn scope_matches(
    scope: Option<&ScopeSpec>,
    styled_grid: &PlayerStyledGrid,
    x: usize,
    y: usize,
) -> bool {
    match scope {
        None | Some(ScopeSpec::All) => true,
        Some(ScopeSpec::Role { role }) => {
            role.shorthand_name() == "border" && is_border_cell(styled_grid, x, y)
        }
        Some(ScopeSpec::Rect { rect }) => rect.contains(x as u16, y as u16),
        Some(ScopeSpec::RowRange { start, end }) => y >= *start && y < *end,
        Some(ScopeSpec::ColumnRange { start, end }) => x >= *start && x < *end,
    }
}

fn role_label_for_scope(scope: Option<&ScopeSpec>) -> Option<String> {
    match scope {
        Some(ScopeSpec::Role { role }) => Some(pascal_role_label(&role.shorthand_name())),
        _ => None,
    }
}

fn border_cells(styled_grid: &PlayerStyledGrid) -> Vec<(usize, usize)> {
    (0..styled_grid.height())
        .flat_map(|y| (0..styled_grid.width()).map(move |x| (x, y)))
        .filter(|(x, y)| is_border_cell(styled_grid, *x, *y))
        .collect()
}

fn is_border_cell(styled_grid: &PlayerStyledGrid, x: usize, y: usize) -> bool {
    styled_grid.contains(x, y)
        && (x == 0 || y == 0 || x + 1 == styled_grid.width() || y + 1 == styled_grid.height())
}

fn color_label(color: ResolvedColor) -> String {
    format!("rgba({},{},{},{})", color.r, color.g, color.b, color.a)
}

fn pascal_role_label(shorthand: &str) -> String {
    let mut chars = shorthand.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_styled_primitive.rs</FILE> - <DESC>Apply styled primitive adapters to player styled grids</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
