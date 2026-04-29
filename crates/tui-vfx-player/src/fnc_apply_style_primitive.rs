// <FILE>crates/tui-vfx-player/src/fnc_apply_style_primitive.rs</FILE> - <DESC>Apply style primitives to player styled grids</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Player adapter de-slop: reuse centralized RGBA report labels.</WCTX>
// <CLOG>0.1.1: PATCH — remove duplicate style-local RGBA label formatting.</CLOG>

use tui_vfx_contract::{NodeSpec, ScopeSpec};

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_collect_styled_grid_scope_cells::collect_styled_grid_scope_cells,
    fnc_resolve_effect_input::{ResolvedColor, resolve_effect_color, resolve_effect_enum},
};

/// Apply a supported style primitive to the styled grid.
pub(crate) fn apply_style_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) -> bool {
    match node.effect.as_str() {
        "style.colorFade" => apply_color_fade(node, request, styled_grid),
        "style.baseStyleOverride"
        | "style.outerBand"
        | "style.moduloRows"
        | "style.moduloColumns"
        | "style.nonEmpty"
        | "style.inner" => apply_base_style_override(node, request, styled_grid),
        _ => return false,
    }
    true
}

fn apply_color_fade(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let target = resolve_effect_color(node, request, "target", ResolvedColor::rgb(255, 200, 50));
    let color_space = resolve_effect_enum(node, request, "colorSpace", "rgb");
    let color = ResolvedColor::rgb(255, 255, 255).lerp_in_color_space(
        target,
        request.phase_t as f32,
        &color_space,
    );
    apply_color_to_scope(node, styled_grid, color.rgba_label(), "transparent", None);
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
    apply_color_to_scope(
        node,
        styled_grid,
        foreground.rgba_label(),
        &background.rgba_label(),
        role_label_for_scope(node.scope.as_ref()),
    );
}

fn apply_color_to_scope(
    node: &NodeSpec,
    styled_grid: &mut PlayerStyledGrid,
    foreground: String,
    background: &str,
    role: Option<String>,
) {
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        styled_grid.set_cell_style(x, y, &foreground, background, vec![], role.clone());
    }
}

fn role_label_for_scope(scope: Option<&ScopeSpec>) -> Option<String> {
    match scope {
        Some(ScopeSpec::Role { role }) => Some(pascal_role_label(&role.shorthand_name())),
        _ => None,
    }
}

fn pascal_role_label(shorthand: &str) -> String {
    let mut chars = shorthand.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_style_primitive.rs</FILE> - <DESC>Apply style primitives to player styled grids</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
