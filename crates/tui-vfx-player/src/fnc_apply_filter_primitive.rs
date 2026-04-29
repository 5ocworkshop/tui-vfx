// <FILE>crates/tui-vfx-player/src/fnc_apply_filter_primitive.rs</FILE> - <DESC>Apply styled filter primitives to player styled grids</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Player adapter de-slop: reuse centralized RGBA report labels.</WCTX>
// <CLOG>0.1.1: PATCH — remove duplicate filter-local RGBA label formatting.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_collect_styled_grid_scope_cells::collect_styled_grid_scope_cells,
    fnc_resolve_effect_input::{
        ResolvedColor, resolve_effect_color, resolve_effect_enum, resolve_effect_number,
    },
};

pub(crate) fn apply_filter_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) -> bool {
    match node.effect.as_str() {
        "filter.dim" => apply_dim_filter(node, request, styled_grid),
        "filter.tint" => apply_tint_filter(node, request, styled_grid),
        "filter.invert" => apply_invert_filter(node, request, styled_grid),
        "filter.greyscale" => apply_greyscale_filter(node, request, styled_grid),
        _ => return false,
    }
    true
}
fn apply_dim_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let factor = resolve_effect_number(node, request, "factor", 0.5).clamp(0.0, 1.0);
    let level = ((255.0 * factor) + 0.5) as u8;
    apply_filter_style(
        node,
        styled_grid,
        &resolve_effect_enum(node, request, "applyTo", "both"),
        ResolvedColor::rgb(level, level, level).rgba_label(),
        "transparent".to_string(),
        Some("FilterDim".to_string()),
    );
}
fn apply_tint_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let tint = resolve_effect_color(node, request, "color", ResolvedColor::rgb(255, 180, 80));
    let strength = resolve_effect_number(node, request, "strength", 0.5).clamp(0.0, 1.0) as f32;
    let color = ResolvedColor::rgb(255, 255, 255).lerp(tint, strength);
    apply_filter_style(
        node,
        styled_grid,
        &resolve_effect_enum(node, request, "applyTo", "both"),
        color.rgba_label(),
        color.lerp(ResolvedColor::rgb(0, 0, 0), 0.65).rgba_label(),
        Some("FilterTint".to_string()),
    );
}
fn apply_invert_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    apply_filter_style(
        node,
        styled_grid,
        &resolve_effect_enum(node, request, "applyTo", "both"),
        "rgba(0,0,0,255)".to_string(),
        "rgba(255,255,255,255)".to_string(),
        Some("FilterInvert".to_string()),
    );
}
fn apply_greyscale_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let strength = resolve_effect_number(node, request, "strength", 1.0).clamp(0.0, 1.0);
    let level = ((255.0 * (1.0 - strength * 0.5)) + 0.5) as u8;
    apply_filter_style(
        node,
        styled_grid,
        &resolve_effect_enum(node, request, "applyTo", "both"),
        ResolvedColor::rgb(level, level, level).rgba_label(),
        ResolvedColor::rgb(level / 4, level / 4, level / 4).rgba_label(),
        Some("FilterGreyscale".to_string()),
    );
}
fn apply_filter_style(
    node: &NodeSpec,
    styled_grid: &mut PlayerStyledGrid,
    apply_to: &str,
    foreground: String,
    background: String,
    role: Option<String>,
) {
    let foreground = if matches!(apply_to, "foreground" | "both") {
        foreground
    } else {
        "defaultForeground".to_string()
    };
    let background = if matches!(apply_to, "background" | "both") {
        background
    } else {
        "transparent".to_string()
    };
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        styled_grid.set_cell_style(x, y, &foreground, &background, vec![], role.clone());
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_filter_primitive.rs</FILE> - <DESC>Apply styled filter primitives to player styled grids</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
