// <FILE>crates/tui-vfx-player/src/fnc_apply_filter_primitive.rs</FILE> - <DESC>Apply styled filter primitives to player styled grids</DESC>
// <VERS>VERSION: 0.1.2</VERS>
// <WCTX>Player adapter de-slop: reuse centralized RGBA report labels.</WCTX>
// <CLOG>0.1.2: PATCH — honor KittScanner axis in styled-player evidence.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_collect_styled_grid_scope_cells::collect_styled_grid_scope_cells,
    fnc_resolve_effect_input::{
        ResolvedColor, resolve_effect_color, resolve_effect_enum, resolve_effect_integer,
        resolve_effect_number, resolve_effect_text,
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
        "filter.vignette" => apply_vignette_filter(node, request, styled_grid),
        "filter.bracketEmphasis" => apply_bracket_emphasis_filter(node, request, styled_grid),
        "filter.dotIndicator" => apply_dot_indicator_filter(node, request, styled_grid),
        "filter.edgeGrow" => apply_edge_grow_filter(node, request, styled_grid),
        "filter.hoverBar" => apply_hover_bar_filter(node, request, styled_grid),
        "filter.kittScanner" => apply_kitt_scanner_filter(node, request, styled_grid),
        "filter.underlineWipe" => apply_underline_wipe_filter(node, request, styled_grid),
        "filter.subPixelBar" => apply_sub_pixel_bar_filter(node, request, styled_grid),
        _ => return false,
    }
    true
}
fn apply_dim_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let factor = resolve_effect_number(node, request, "factor", 0.5).clamp(0.0, 1.0) as f32;
    apply_cell_color_filter(
        node,
        request,
        styled_grid,
        &resolve_effect_enum(node, request, "applyTo", "both"),
        "FilterDim",
        |color| scale_color(color, 1.0 - factor),
    );
}
fn apply_tint_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let tint = resolve_effect_color(node, request, "color", ResolvedColor::rgb(255, 180, 80));
    let strength = resolve_effect_number(node, request, "strength", 0.5).clamp(0.0, 1.0) as f32;
    let apply_to = resolve_effect_enum(node, request, "applyTo", "both");
    let coordinates = collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request);
    for (x, y) in coordinates {
        let Some(cell) = styled_grid
            .cells()
            .iter()
            .find(|cell| cell.x == x && cell.y == y)
            .cloned()
        else {
            continue;
        };
        let foreground = if matches!(apply_to.as_str(), "foreground" | "both") {
            parse_rgba_label(&cell.foreground, ResolvedColor::rgb(255, 255, 255))
                .lerp(tint, strength)
                .rgba_label()
        } else {
            cell.foreground.clone()
        };
        let background = if matches!(apply_to.as_str(), "background" | "both") {
            parse_rgba_label(&cell.background, ResolvedColor::rgb(0, 0, 0))
                .lerp(tint, strength)
                .rgba_label()
        } else {
            cell.background.clone()
        };
        styled_grid.set_cell_style(
            x,
            y,
            &foreground,
            &background,
            cell.modifiers.clone(),
            Some("FilterTint".to_string()),
        );
    }
}
fn apply_invert_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let apply_to = resolve_effect_enum(node, request, "applyTo", "both");
    let coordinates = collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request);
    for (x, y) in coordinates {
        let Some(cell) = styled_grid
            .cells()
            .iter()
            .find(|cell| cell.x == x && cell.y == y)
            .cloned()
        else {
            continue;
        };
        let old_foreground = parse_rgba_label(&cell.foreground, ResolvedColor::rgb(255, 255, 255));
        let old_background = parse_rgba_label(&cell.background, ResolvedColor::rgb(0, 0, 0));
        let (foreground, background) = match apply_to.as_str() {
            "foreground" => (old_background.rgba_label(), cell.background.clone()),
            "background" => (cell.foreground.clone(), old_foreground.rgba_label()),
            _ => (old_background.rgba_label(), old_foreground.rgba_label()),
        };
        styled_grid.set_cell_style(
            x,
            y,
            &foreground,
            &background,
            cell.modifiers.clone(),
            Some("FilterInvert".to_string()),
        );
    }
}
fn apply_greyscale_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let strength = resolve_effect_number(node, request, "strength", 1.0).clamp(0.0, 1.0) as f32;
    apply_cell_color_filter(
        node,
        request,
        styled_grid,
        &resolve_effect_enum(node, request, "applyTo", "both"),
        "FilterGreyscale",
        |color| greyscale_color(color, strength),
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
        request,
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
        request,
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
        request,
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
        request,
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
        request,
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

fn apply_vignette_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let strength = resolve_effect_number(node, request, "strength", 0.6).clamp(0.0, 1.0) as f32;
    let edge_color =
        resolve_effect_color(node, request, "edgeColor", ResolvedColor::rgb(10, 20, 36));
    let apply_to = resolve_effect_enum(node, request, "applyTo", "both");
    let max_distance = vignette_corner_distance(styled_grid.width(), styled_grid.height()).max(1.0);
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        let distance =
            vignette_distance_from_center(x, y, styled_grid.width(), styled_grid.height());
        let mix = ((distance / max_distance) as f32 * strength).clamp(0.0, 1.0);
        let foreground = ResolvedColor::rgb(255, 255, 255)
            .lerp(edge_color, mix)
            .rgba_label();
        let background = edge_color
            .lerp(ResolvedColor::rgb(0, 0, 0), 1.0 - mix)
            .rgba_label();
        set_filter_cell(
            styled_grid,
            x,
            y,
            &apply_to,
            foreground,
            background,
            "FilterVignette",
        );
    }
}

fn apply_bracket_emphasis_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(
        node,
        request,
        "emphasisColor",
        ResolvedColor::rgb(255, 210, 90),
    );
    let edge_width = resolve_effect_integer(node, request, "edgeWidth", 1).max(0) as usize;
    let apply_to = resolve_effect_enum(node, request, "applyTo", "foreground");
    let width = styled_grid.width();
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        let on_edge = x < edge_width || x + edge_width >= width;
        let foreground = if on_edge {
            color.rgba_label()
        } else {
            color
                .lerp(ResolvedColor::rgb(255, 255, 255), 0.7)
                .rgba_label()
        };
        set_filter_cell(
            styled_grid,
            x,
            y,
            &apply_to,
            foreground,
            "transparent".to_string(),
            "FilterBracketEmphasis",
        );
    }
}

fn apply_dot_indicator_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let indicator_char = resolve_effect_text(node, request, "indicatorChar", "•")
        .chars()
        .next()
        .unwrap_or('•')
        .to_string();
    let position = resolve_effect_enum(node, request, "position", "left");
    let color = resolve_effect_color_alias(
        node,
        request,
        &["activeColor", "color"],
        ResolvedColor::rgb(100, 150, 200),
    );
    let background = resolve_effect_color_alias(
        node,
        request,
        &["inactiveColor", "bgColor"],
        ResolvedColor::rgb(30, 30, 30),
    );
    let progress = resolve_effect_number(node, request, "progress", 1.0).clamp(0.0, 1.0) as f32;
    if progress <= 0.0 {
        return;
    }
    let foreground = background.lerp(color, progress).rgba_label();
    let background = background.rgba_label();
    let width = styled_grid.width().max(1);
    let height = styled_grid.height().max(1);
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        let is_target = match position.as_str() {
            "right" => x == width.saturating_sub(1),
            "top" => y == 0 && x == width / 2,
            "bottom" => y == height.saturating_sub(1) && x == width / 2,
            _ => x == 0,
        };
        if !is_target {
            continue;
        };
        styled_grid.set_cell_glyph_and_style(
            x,
            y,
            &indicator_char,
            &foreground,
            &background,
            vec![],
            Some("FilterDotIndicator".to_string()),
        );
    }
}

fn apply_edge_grow_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let direction = resolve_effect_enum(node, request, "direction", "left");
    let progress =
        resolve_effect_number(node, request, "progress", request.phase_t).clamp(0.0, 1.0);
    let color = resolve_effect_color(node, request, "edgeColor", ResolvedColor::rgb(255, 120, 80));
    let apply_to = resolve_effect_enum(node, request, "applyTo", "both");
    let width = styled_grid.width().max(1);
    let height = styled_grid.height().max(1);
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        let coordinate = match direction.as_str() {
            "right" => width.saturating_sub(1).saturating_sub(x),
            "top" => y,
            "bottom" => height.saturating_sub(1).saturating_sub(y),
            _ => x,
        };
        let limit = match direction.as_str() {
            "top" | "bottom" => (height as f64 * progress).ceil() as usize,
            _ => (width as f64 * progress).ceil() as usize,
        };
        let mix = if coordinate < limit { 0.0 } else { 0.75 };
        let foreground = color
            .lerp(ResolvedColor::rgb(255, 255, 255), mix)
            .rgba_label();
        let background = color
            .lerp(ResolvedColor::rgb(0, 0, 0), 0.7 + mix * 0.2)
            .rgba_label();
        set_filter_cell(
            styled_grid,
            x,
            y,
            &apply_to,
            foreground,
            background,
            "FilterEdgeGrow",
        );
    }
}

fn apply_hover_bar_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(node, request, "barColor", ResolvedColor::rgb(80, 190, 255));
    let thickness = resolve_effect_integer(node, request, "thickness", 1).max(1) as usize;
    let position =
        resolve_effect_number(node, request, "position", request.phase_t).clamp(0.0, 1.0);
    let apply_to = resolve_effect_enum(node, request, "applyTo", "background");
    let center_y = ((styled_grid.height().saturating_sub(1)) as f64 * position).round() as usize;
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        let distance = y.abs_diff(center_y);
        let mix = if distance < thickness { 0.0 } else { 0.8 };
        let foreground = color
            .lerp(ResolvedColor::rgb(255, 255, 255), 0.4 + mix * 0.3)
            .rgba_label();
        let background = color.lerp(ResolvedColor::rgb(0, 0, 0), mix).rgba_label();
        set_filter_cell(
            styled_grid,
            x,
            y,
            &apply_to,
            foreground,
            background,
            "FilterHoverBar",
        );
    }
}

fn apply_kitt_scanner_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let scan = resolve_effect_color(node, request, "scanColor", ResolvedColor::rgb(255, 40, 40));
    let trail = resolve_effect_color(node, request, "trailColor", ResolvedColor::rgb(110, 20, 20));
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let scanner_width = resolve_effect_integer(node, request, "width", 3).max(1) as usize;
    let apply_to = resolve_effect_enum(node, request, "applyTo", "both");
    let axis = resolve_effect_enum(node, request, "axis", "horizontal");
    let sweep_extent = if is_vertical_axis(&axis) {
        styled_grid.height().max(1)
    } else {
        styled_grid.width().max(1)
    };
    let span = sweep_extent.saturating_sub(1).max(1);
    let sweep = ((request.loop_t.unwrap_or(request.phase_t) * speed).fract() * (span * 2) as f64)
        .round() as usize;
    let center = if sweep <= span {
        sweep
    } else {
        span * 2 - sweep
    };
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        let coordinate = if is_vertical_axis(&axis) { y } else { x };
        let distance = coordinate.abs_diff(center);
        let mix = (distance as f32 / scanner_width as f32).clamp(0.0, 1.0);
        let foreground = scan.lerp(trail, mix).rgba_label();
        let background = trail.lerp(ResolvedColor::rgb(0, 0, 0), mix).rgba_label();
        set_filter_cell(
            styled_grid,
            x,
            y,
            &apply_to,
            foreground,
            background,
            "FilterKittScanner",
        );
    }
}

fn is_vertical_axis(axis: &str) -> bool {
    matches!(axis, "vertical" | "y" | "Y")
}

fn apply_underline_wipe_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(
        node,
        request,
        "underlineColor",
        ResolvedColor::rgb(120, 220, 255),
    );
    let progress =
        resolve_effect_number(node, request, "progress", request.phase_t).clamp(0.0, 1.0);
    let thickness = resolve_effect_integer(node, request, "thickness", 1).max(1) as usize;
    let apply_to = resolve_effect_enum(node, request, "applyTo", "foreground");
    let cutoff = (styled_grid.width() as f64 * progress).ceil() as usize;
    let height = styled_grid.height();
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        let underlined = x < cutoff && y + thickness >= height;
        let foreground = if underlined {
            color.rgba_label()
        } else {
            color
                .lerp(ResolvedColor::rgb(255, 255, 255), 0.75)
                .rgba_label()
        };
        let modifiers = if underlined {
            vec!["underline".to_string()]
        } else {
            vec![]
        };
        set_filter_cell_with_modifiers(
            styled_grid,
            x,
            y,
            &apply_to,
            foreground,
            "transparent".to_string(),
            modifiers,
            "FilterUnderlineWipe",
        );
    }
}

fn apply_sub_pixel_bar_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let filled_color = resolve_effect_color(
        node,
        request,
        "filledColor",
        ResolvedColor::rgb(100, 220, 255),
    );
    let unfilled_color = resolve_effect_color(
        node,
        request,
        "unfilledColor",
        ResolvedColor::rgb(30, 40, 50),
    );
    let progress =
        resolve_effect_number(node, request, "progress", request.phase_t).clamp(0.0, 1.0);
    let direction = resolve_effect_enum(node, request, "direction", "horizontal");
    let width = styled_grid.width().max(1);
    let height = styled_grid.height().max(1);
    let horizontal = !matches!(
        direction.as_str(),
        "vertical" | "topToBottom" | "bottomToTop"
    );
    let total_subcells = if horizontal { width } else { height }.saturating_mul(8);
    let filled_subcells = (total_subcells as f64 * progress).ceil() as usize;
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        let coordinate = if horizontal { x } else { y };
        let filled = filled_subcells
            .saturating_sub(coordinate.saturating_mul(8))
            .min(8);
        let glyph = sub_pixel_bar_glyph(filled);
        let foreground = if filled == 0 {
            unfilled_color
        } else {
            filled_color
        };
        styled_grid.set_cell_glyph_and_style(
            x,
            y,
            glyph,
            &foreground.rgba_label(),
            &unfilled_color.rgba_label(),
            vec![],
            Some("FilterSubPixelBar".to_string()),
        );
    }
}

fn sub_pixel_bar_glyph(filled: usize) -> &'static str {
    match filled {
        0 => " ",
        1 => "▏",
        2 => "▎",
        3 => "▍",
        4 => "▌",
        5 => "▋",
        6 => "▊",
        7 => "▉",
        _ => "█",
    }
}

fn apply_cell_color_filter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
    apply_to: &str,
    role: &str,
    transform: impl Fn(ResolvedColor) -> ResolvedColor,
) {
    let coordinates = collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request);
    for (x, y) in coordinates {
        let Some(cell) = styled_grid
            .cells()
            .iter()
            .find(|cell| cell.x == x && cell.y == y)
            .cloned()
        else {
            continue;
        };
        let foreground = if matches!(apply_to, "foreground" | "both") {
            transform(parse_rgba_label(
                &cell.foreground,
                ResolvedColor::rgb(255, 255, 255),
            ))
            .rgba_label()
        } else {
            cell.foreground.clone()
        };
        let background = if matches!(apply_to, "background" | "both") {
            transform(parse_rgba_label(
                &cell.background,
                ResolvedColor::rgb(0, 0, 0),
            ))
            .rgba_label()
        } else {
            cell.background.clone()
        };
        styled_grid.set_cell_style(
            x,
            y,
            &foreground,
            &background,
            cell.modifiers.clone(),
            Some(role.to_string()),
        );
    }
}

fn resolve_effect_color_alias(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_ids: &[&str],
    fallback: ResolvedColor,
) -> ResolvedColor {
    input_ids.iter().fold(fallback, |current, input_id| {
        resolve_effect_color(node, request, input_id, current)
    })
}

fn scale_color(color: ResolvedColor, factor: f32) -> ResolvedColor {
    let factor = factor.clamp(0.0, 1.0);
    ResolvedColor::new(
        (color.r as f32 * factor).round() as u8,
        (color.g as f32 * factor).round() as u8,
        (color.b as f32 * factor).round() as u8,
        color.a,
    )
}

fn greyscale_color(color: ResolvedColor, strength: f32) -> ResolvedColor {
    let strength = strength.clamp(0.0, 1.0);
    let grey =
        (0.299 * color.r as f32 + 0.587 * color.g as f32 + 0.114 * color.b as f32).round() as u8;
    color.lerp(ResolvedColor::rgb(grey, grey, grey), strength)
}

fn parse_rgba_label(label: &str, fallback: ResolvedColor) -> ResolvedColor {
    let Some(body) = label
        .strip_prefix("rgba(")
        .and_then(|value| value.strip_suffix(')'))
    else {
        return fallback;
    };
    let mut channels = body.split(',').map(str::trim).map(str::parse::<u8>);
    match (
        channels.next(),
        channels.next(),
        channels.next(),
        channels.next(),
    ) {
        (Some(Ok(r)), Some(Ok(g)), Some(Ok(b)), Some(Ok(a))) => ResolvedColor::new(r, g, b, a),
        _ => fallback,
    }
}

fn apply_filter_style(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
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
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid, request) {
        styled_grid.set_cell_style(x, y, &foreground, &background, vec![], role.clone());
    }
}

fn set_filter_cell(
    styled_grid: &mut PlayerStyledGrid,
    x: usize,
    y: usize,
    apply_to: &str,
    foreground: String,
    background: String,
    role: &str,
) {
    set_filter_cell_with_modifiers(
        styled_grid,
        x,
        y,
        apply_to,
        foreground,
        background,
        vec![],
        role,
    );
}

fn set_filter_cell_with_modifiers(
    styled_grid: &mut PlayerStyledGrid,
    x: usize,
    y: usize,
    apply_to: &str,
    foreground: String,
    background: String,
    modifiers: Vec<String>,
    role: &str,
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
    styled_grid.set_cell_style(
        x,
        y,
        &foreground,
        &background,
        modifiers,
        Some(role.to_string()),
    );
}

fn vignette_distance_from_center(x: usize, y: usize, width: usize, height: usize) -> f64 {
    let center_x = (width.saturating_sub(1)) as f64 / 2.0;
    let center_y = (height.saturating_sub(1)) as f64 / 2.0;
    let dx = x as f64 - center_x;
    let dy = y as f64 - center_y;
    dx.mul_add(dx, dy * dy).sqrt()
}

fn vignette_corner_distance(width: usize, height: usize) -> f64 {
    vignette_distance_from_center(0, 0, width, height)
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_filter_primitive.rs</FILE> - <DESC>Apply styled filter primitives to player styled grids</DESC>
// <VERS>END OF VERSION: 0.1.2</VERS>
