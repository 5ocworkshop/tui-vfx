// <FILE>crates/tui-vfx-player/src/fnc_apply_style_primitive.rs</FILE> - <DESC>Apply style primitives to player styled grids</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player adapter de-slop: reuse centralized RGBA report labels.</WCTX>
// <CLOG>0.2.0: MINOR — fade style color channels from existing cell styles for V2 ColorFade parity.
// 0.1.1: PATCH — remove duplicate style-local RGBA label formatting.</CLOG>

use tui_vfx_contract::{NodeSpec, ScopeSpec};

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_collect_styled_grid_scope_cells::collect_styled_grid_scope_cells,
    fnc_resolve_effect_input::{
        ResolvedColor, resolve_effect_bool, resolve_effect_color, resolve_effect_enum,
        resolve_effect_number,
    },
};

/// Apply a supported style primitive to the styled grid.
pub(crate) fn apply_style_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) -> bool {
    match node.effect.as_str() {
        "style.colorFade" => apply_color_fade(node, request, styled_grid),
        "style.fadeIn" => apply_fade_in(node, request, styled_grid),
        "style.fadeOut" => apply_fade_out(node, request, styled_grid),
        "style.pulse" => apply_pulse(node, request, styled_grid),
        "style.italicWindow" => apply_italic_window(node, request, styled_grid),
        "style.neonFlicker" => apply_neon_flicker(node, request, styled_grid),
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
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let existing = styled_grid
            .cells()
            .iter()
            .find(|cell| cell.x == x && cell.y == y)
            .cloned();
        let Some(existing) = existing else {
            continue;
        };
        let foreground = legacy_color_fade_label(
            &existing.foreground,
            target,
            request.phase_t as f32,
            &color_space,
        )
        .unwrap_or(existing.foreground);
        let background = legacy_color_fade_label(
            &existing.background,
            target,
            request.phase_t as f32,
            &color_space,
        )
        .unwrap_or(existing.background);
        styled_grid.set_cell_style(
            x,
            y,
            &foreground,
            &background,
            existing.modifiers,
            existing.role,
        );
    }
}

fn apply_fade_in(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    apply_fade(
        node,
        request,
        styled_grid,
        ResolvedColor::rgb(0, 0, 0),
        ResolvedColor::rgb(255, 255, 255),
    );
}

fn apply_fade_out(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    apply_fade(
        node,
        request,
        styled_grid,
        ResolvedColor::rgb(255, 255, 255),
        ResolvedColor::rgb(0, 0, 0),
    );
}

fn apply_fade(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
    fallback_from: ResolvedColor,
    fallback_to: ResolvedColor,
) {
    let from = resolve_effect_color(node, request, "from", fallback_from);
    let to = resolve_effect_color(node, request, "to", fallback_to);
    let apply_to = resolve_effect_enum(node, request, "applyTo", "foreground");
    let color_space = resolve_effect_enum(node, request, "colorSpace", "rgb");
    let color = from.lerp_in_color_space(to, eased_phase(node, request), &color_space);
    apply_style_to_scope(
        node,
        styled_grid,
        Some(&apply_to),
        Some(color.rgba_label()),
        None,
    );
}

fn apply_pulse(node: &NodeSpec, request: &PlayerSampleRequest, styled_grid: &mut PlayerStyledGrid) {
    let pulse_color = resolve_effect_color(
        node,
        request,
        "pulseColor",
        ResolvedColor::rgb(255, 80, 160),
    );
    let color = resolve_effect_color(node, request, "color", pulse_color);
    let frequency = resolve_effect_number(node, request, "frequency", 1.0).max(0.0);
    let apply_to = resolve_effect_enum(node, request, "applyTo", "foreground");
    let clock = request.loop_t.unwrap_or(request.phase_t);
    let strength = ((clock * frequency * std::f64::consts::TAU).sin() * 0.5 + 0.5) as f32;
    let pulsed_color = ResolvedColor::rgb(255, 255, 255).lerp(color, strength);
    apply_style_to_scope(
        node,
        styled_grid,
        Some(&apply_to),
        Some(pulsed_color.rgba_label()),
        None,
    );
}

fn apply_italic_window(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let start = resolve_effect_number(node, request, "start", 0.0).clamp(0.0, 1.0);
    let end = resolve_effect_number(node, request, "end", 1.0).clamp(start, 1.0);
    if (start..=end).contains(&request.phase_t) {
        apply_style_to_scope(node, styled_grid, None, None, Some("italic"));
    }
}

fn apply_neon_flicker(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(node, request, "color", ResolvedColor::rgb(80, 255, 220));
    let stability = resolve_effect_number(node, request, "stability", 0.7).clamp(0.0, 1.0) as f32;
    let dim_amount = resolve_effect_number(node, request, "dimAmount", 0.5).clamp(0.0, 1.0) as f32;
    let italic = resolve_effect_bool(node, request, "italicWindow", false);
    let clock = request.loop_t.unwrap_or(request.phase_t);
    let flicker = ((clock * 37.0).sin() * 0.5 + 0.5) as f32;
    let active_strength = if flicker <= stability {
        1.0
    } else {
        1.0 - dim_amount
    };
    let neon_color = ResolvedColor::rgb(0, 0, 0).lerp(color, active_strength);
    apply_style_to_scope(
        node,
        styled_grid,
        Some("foreground"),
        Some(neon_color.rgba_label()),
        if italic { Some("italic") } else { None },
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

fn apply_style_to_scope(
    node: &NodeSpec,
    styled_grid: &mut PlayerStyledGrid,
    apply_to: Option<&str>,
    color: Option<String>,
    modifier: Option<&str>,
) {
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let existing = styled_grid
            .cells()
            .iter()
            .find(|cell| cell.x == x && cell.y == y);
        let existing_foreground = existing
            .map(|cell| cell.foreground.clone())
            .unwrap_or_else(|| "defaultForeground".to_string());
        let existing_background = existing
            .map(|cell| cell.background.clone())
            .unwrap_or_else(|| "transparent".to_string());
        let mut modifiers = existing
            .map(|cell| cell.modifiers.clone())
            .unwrap_or_default();
        if let Some(modifier) = modifier
            && !modifiers.iter().any(|existing| existing == modifier)
        {
            modifiers.push(modifier.to_string());
        }
        let foreground = if matches!(apply_to, Some("foreground" | "both")) {
            color.clone().unwrap_or(existing_foreground)
        } else {
            existing_foreground
        };
        let background = if matches!(apply_to, Some("background" | "both")) {
            color.clone().unwrap_or(existing_background)
        } else {
            existing_background
        };
        styled_grid.set_cell_style(x, y, &foreground, &background, modifiers, None);
    }
}

fn eased_phase(node: &NodeSpec, request: &PlayerSampleRequest) -> f32 {
    let phase = request.phase_t.clamp(0.0, 1.0) as f32;
    let easing = resolve_effect_enum(node, request, "easing", "linear");
    match resolve_effect_enum(node, request, "ease", &easing).as_str() {
        "easeIn" => phase * phase,
        "easeOut" => 1.0 - (1.0 - phase) * (1.0 - phase),
        "easeInOut" => {
            if phase < 0.5 {
                2.0 * phase * phase
            } else {
                1.0 - (-2.0 * phase + 2.0).powi(2) / 2.0
            }
        }
        _ => phase,
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

fn resolved_color_from_rgba_label(label: &str) -> Option<ResolvedColor> {
    let inner = label.strip_prefix("rgba(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(str::trim);
    Some(ResolvedColor::new(
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    ))
}

fn legacy_color_fade_label(
    from_label: &str,
    target: ResolvedColor,
    t: f32,
    color_space: &str,
) -> Option<String> {
    let from = resolved_color_from_rgba_label(from_label)?;
    let blended = if color_space.eq_ignore_ascii_case("hsl") {
        legacy_hsl_color_fade(from, target, t)
    } else {
        legacy_rgb_color_fade(from, target, t)
    };
    Some(blended.rgba_label())
}

fn legacy_rgb_color_fade(from: ResolvedColor, target: ResolvedColor, t: f32) -> ResolvedColor {
    let t = t.clamp(0.0, 1.0);
    ResolvedColor::new(
        legacy_lerp_channel(from.r, target.r, t),
        legacy_lerp_channel(from.g, target.g, t),
        legacy_lerp_channel(from.b, target.b, t),
        legacy_lerp_channel(from.a, target.a, t),
    )
}

fn legacy_hsl_color_fade(from: ResolvedColor, target: ResolvedColor, t: f32) -> ResolvedColor {
    let t = t.clamp(0.0, 1.0);
    let (start_hue, start_saturation, start_lightness) = rgb_to_hsl(from.r, from.g, from.b);
    let (end_hue, end_saturation, end_lightness) = rgb_to_hsl(target.r, target.g, target.b);
    let hue_delta = if end_hue - start_hue > 180.0 {
        end_hue - start_hue - 360.0
    } else if end_hue - start_hue < -180.0 {
        end_hue - start_hue + 360.0
    } else {
        end_hue - start_hue
    };
    let hue = (start_hue + hue_delta * t).rem_euclid(360.0);
    let saturation = start_saturation + (end_saturation - start_saturation) * t;
    let lightness = start_lightness + (end_lightness - start_lightness) * t;
    let (r, g, b) = hsl_to_rgb(hue, saturation, lightness);
    ResolvedColor::new(r, g, b, legacy_lerp_channel(from.a, target.a, t))
}

fn legacy_lerp_channel(start: u8, end: u8, t: f32) -> u8 {
    (start as f32 + (end as f32 - start as f32) * t) as u8
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let lightness = (max + min) / 2.0;
    if max == min {
        return (0.0, 0.0, lightness);
    }
    let delta = max - min;
    let saturation = if lightness > 0.5 {
        delta / (2.0 - max - min)
    } else {
        delta / (max + min)
    };
    let hue = if max == r {
        (g - b) / delta + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };
    (hue * 60.0, saturation, lightness)
}

fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (u8, u8, u8) {
    if saturation == 0.0 {
        let value = (lightness * 255.0) as u8;
        return (value, value, value);
    }
    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - lightness * saturation
    };
    let p = 2.0 * lightness - q;
    let r = hue_to_rgb(p, q, hue / 360.0 + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, hue / 360.0);
    let b = hue_to_rgb(p, q, hue / 360.0 - 1.0 / 3.0);
    (
        (r * 255.0 + 0.0001) as u8,
        (g * 255.0 + 0.0001) as u8,
        (b * 255.0 + 0.0001) as u8,
    )
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_style_primitive.rs</FILE> - <DESC>Apply style primitives to player styled grids</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
