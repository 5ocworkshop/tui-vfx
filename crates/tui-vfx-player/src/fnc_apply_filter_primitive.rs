// <FILE>crates/tui-vfx-player/src/fnc_apply_filter_primitive.rs</FILE> - <DESC>Apply styled filter primitives to player styled grids</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Player adapter de-slop: reuse centralized RGBA report labels.</WCTX>
// <CLOG>0.1.1: PATCH — remove duplicate filter-local RGBA label formatting.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_collect_styled_grid_scope_cells::collect_styled_grid_scope_cells,
    fnc_resolve_effect_input::{
        ResolvedColor, resolve_effect_color, resolve_effect_enum, resolve_effect_integer,
        resolve_effect_number,
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
        "filter.pillButton" => apply_pill_button_filter(node, request, styled_grid),
        "filter.fadeToCanvas" => apply_fade_to_canvas_filter(node, request, styled_grid),
        "filter.patternFill" => apply_pattern_fill_filter(node, request, styled_grid),
        "filter.crt" => apply_crt_filter(node, request, styled_grid),
        "filter.matrixRain" => apply_matrix_rain_filter(node, request, styled_grid),
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

fn apply_pill_button_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let progress =
        resolve_effect_number(node, request, "progress", request.phase_t).clamp(0.0, 1.0);
    let active = resolve_effect_color(
        node,
        request,
        "buttonColor",
        resolve_effect_color(
            node,
            request,
            "activeColor",
            ResolvedColor::rgb(80, 220, 180),
        ),
    );
    let background = resolve_effect_color(
        node,
        request,
        "bgColor",
        resolve_effect_color(
            node,
            request,
            "inactiveColor",
            ResolvedColor::rgb(20, 60, 55),
        ),
    );
    let edge_width = resolve_effect_integer(node, request, "edgeWidth", 1).max(0) as f32;
    let glisten = resolve_effect_number(node, request, "glisten", 0.0).clamp(0.0, 1.0) as f32;
    let background = background.lerp(
        ResolvedColor::rgb(255, 255, 255),
        (glisten + edge_width * 0.02).min(1.0),
    );
    let active = active
        .lerp(ResolvedColor::rgb(255, 255, 255), (progress * 0.12) as f32)
        .rgba_label();
    apply_filter_style(
        node,
        styled_grid,
        "both",
        active,
        background.rgba_label(),
        Some("FilterPillButton".to_string()),
    );
}

fn apply_fade_to_canvas_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let canvas = resolve_effect_color(node, request, "canvasColor", ResolvedColor::rgb(8, 12, 20));
    let amount = resolve_effect_number(
        node,
        request,
        "amount",
        resolve_effect_number(node, request, "strength", request.phase_t),
    )
    .clamp(0.0, 1.0) as f32;
    let foreground = ResolvedColor::rgb(255, 255, 255)
        .lerp(canvas, amount)
        .rgba_label();
    apply_filter_style(
        node,
        styled_grid,
        "both",
        foreground,
        canvas.rgba_label(),
        Some("FilterFadeToCanvas".to_string()),
    );
}

fn apply_pattern_fill_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let density = resolve_effect_number(node, request, "density", 0.5).clamp(0.0, 1.0);
    let level = (90.0 + density * 120.0) as u8;
    apply_filter_style(
        node,
        styled_grid,
        "foreground",
        ResolvedColor::rgb(level, level, 255).rgba_label(),
        "transparent".to_string(),
        Some("FilterPatternFill".to_string()),
    );
}

fn apply_crt_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let intensity = resolve_effect_number(node, request, "intensity", 0.5).clamp(0.0, 1.0);
    let glow = resolve_effect_number(node, request, "glow", 0.0).clamp(0.0, 1.0);
    let scanline = resolve_effect_number(node, request, "scanlineStrength", 0.0).clamp(0.0, 1.0);
    let level = (120.0 + (intensity + glow * 0.5 + scanline * 0.25).min(1.0) * 100.0) as u8;
    apply_filter_style(
        node,
        styled_grid,
        "foreground",
        ResolvedColor::rgb(level, 255, level).rgba_label(),
        "transparent".to_string(),
        Some("FilterCrt".to_string()),
    );
}

fn apply_matrix_rain_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let speed_multiplier = resolve_effect_number(node, request, "speedMultiplier", 1.0).max(0.0);
    let speed_min = resolve_effect_number(node, request, "speedMin", 0.0).max(0.0);
    let speed_max = resolve_effect_number(
        node,
        request,
        "speedMax",
        resolve_effect_number(node, request, "speed", 1.0),
    )
    .max(speed_min);
    let glyph_hz = resolve_effect_number(node, request, "glyphChangeHz", 8.0).max(0.0);
    let density = resolve_effect_number(node, request, "density", 0.5).clamp(0.0, 1.0);
    let seed = resolve_effect_integer(node, request, "seed", 1).max(0) as f64;
    let trail_min = resolve_effect_integer(node, request, "trailMin", 2).max(0) as f64;
    let trail_max =
        resolve_effect_integer(node, request, "trailMax", 8).max(trail_min as i64) as f64;
    let affect = resolve_effect_enum(node, request, "affect", "foreground");
    let chars = resolve_effect_enum(node, request, "chars", "01");
    let mode = resolve_effect_enum(node, request, "mode", "rain");
    let preset = resolve_effect_enum(node, request, "preset", "default");
    let head = resolve_effect_color(node, request, "headColor", ResolvedColor::rgb(40, 255, 80));
    let tail = resolve_effect_color(node, request, "tailColor", ResolvedColor::rgb(20, 120, 40));
    let speed = ((speed_min + speed_max) * 0.5) * speed_multiplier;
    let trail = ((trail_min + trail_max) * 0.5).max(1.0);
    let text_factor =
        (chars.chars().count() as f64 + mode.len() as f64 + preset.len() as f64 + seed) % 7.0;
    let level = (100.0
        + ((request.phase_t * speed + glyph_hz * 0.01 + density + text_factor * 0.01) / trail)
            .fract()
            * 155.0) as u8;
    let color = head
        .lerp(tail, (1.0 - density as f32).clamp(0.0, 1.0))
        .lerp(ResolvedColor::rgb(40, level, 80), 0.5)
        .rgba_label();
    apply_filter_style(
        node,
        styled_grid,
        "foreground",
        color,
        if affect == "background" || affect == "both" {
            ResolvedColor::rgb(0, level / 3, 0).rgba_label()
        } else {
            "transparent".to_string()
        },
        Some("FilterMatrixRain".to_string()),
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
