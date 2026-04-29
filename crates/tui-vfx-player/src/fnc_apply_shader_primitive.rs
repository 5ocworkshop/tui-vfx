// <FILE>crates/tui-vfx-player/src/fnc_apply_shader_primitive.rs</FILE> - <DESC>Apply shader primitives to player styled grids</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Player adapter de-slop: reuse centralized RGBA report labels.</WCTX>
// <CLOG>0.1.1: PATCH — remove duplicate shader-local RGBA label formatting.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_collect_styled_grid_scope_cells::{
        collect_styled_grid_border_cells, collect_styled_grid_scope_cells,
    },
    fnc_resolve_effect_input::{
        ResolvedColor, resolve_effect_color, resolve_effect_enum, resolve_effect_integer,
        resolve_effect_number,
    },
};

/// Apply a supported shader primitive to the styled grid.
pub(crate) fn apply_shader_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) -> bool {
    match node.effect.as_str() {
        "shader.linearGradient" => apply_linear_gradient(node, request, styled_grid),
        "shader.borderSweep" => apply_border_sweep(node, request, styled_grid),
        _ => return false,
    }
    true
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
    let color_space = resolve_effect_enum(node, request, "colorSpace", "rgb");
    let angle = resolve_effect_number(node, request, "angleDeg", 45.0).to_radians();
    let intensity = resolve_effect_number(node, request, "intensity", 1.0).clamp(0.0, 1.0) as f32;
    let max_x = styled_grid.width().saturating_sub(1).max(1) as f64;
    let max_y = styled_grid.height().saturating_sub(1).max(1) as f64;
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let nx = x as f64 / max_x;
        let ny = y as f64 / max_y;
        let projection = (nx * angle.cos() + ny * angle.sin() + 1.0) / 2.0;
        let color = start.lerp_in_color_space(
            end,
            (projection.clamp(0.0, 1.0) as f32) * intensity,
            &color_space,
        );
        styled_grid.set_cell_style(x, y, &color.rgba_label(), "transparent", vec![], None);
    }
}

fn apply_border_sweep(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color =
        resolve_effect_color(node, request, "color", ResolvedColor::rgb(0, 255, 255)).rgba_label();
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let length = resolve_effect_integer(node, request, "length", 10).max(1) as usize;
    let edge_cells = collect_styled_grid_border_cells(styled_grid);
    if edge_cells.is_empty() {
        return;
    }
    let offset =
        ((request.phase_t * speed) * edge_cells.len() as f64).round() as usize % edge_cells.len();
    for index in 0..length.min(edge_cells.len()) {
        let (x, y) = edge_cells[(offset + index) % edge_cells.len()];
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

// <FILE>crates/tui-vfx-player/src/fnc_apply_shader_primitive.rs</FILE> - <DESC>Apply shader primitives to player styled grids</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
