// <FILE>crates/tui-vfx-player/src/fnc_apply_shader_primitive.rs</FILE> - <DESC>Apply shader primitives to player styled grids</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.13 field coverage closure: support gradient stops, applyTo, and direct border-sweep position.</WCTX>
// <CLOG>0.2.0: MINOR — support canonical gradient, channel target, and position inputs.
// 0.1.1: PATCH — remove duplicate shader-local RGBA label formatting.</CLOG>

use tui_vfx_contract::{GradientSpec, NodeSpec};

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_collect_styled_grid_scope_cells::{
        collect_styled_grid_border_cells, collect_styled_grid_scope_cells,
    },
    fnc_resolve_effect_input::{
        ResolvedColor, resolve_effect_color, resolve_effect_enum, resolve_effect_gradient,
        resolve_effect_integer, resolve_effect_number,
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
        "shader.revealWipe" => apply_reveal_wipe(node, request, styled_grid),
        "shader.highlighter" => apply_highlighter(node, request, styled_grid),
        "shader.focusField" => apply_focus_field(node, request, styled_grid),
        "shader.glistenBand" => apply_glisten_band(node, request, styled_grid),
        "shader.wayfindingNode" => apply_wayfinding_node(node, request, styled_grid),
        "shader.barberPole" => apply_barber_pole(node, request, styled_grid),
        "shader.diffusion" => apply_diffusion(node, request, styled_grid),
        "shader.radar" => apply_radar(node, request, styled_grid),
        _ => return false,
    }
    true
}

fn apply_linear_gradient(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let gradient = resolve_linear_gradient(node, request);
    let angle = resolve_effect_number(node, request, "angleDeg", 45.0).to_radians();
    let intensity = resolve_effect_number(node, request, "intensity", 1.0).clamp(0.0, 1.0) as f32;
    let apply_to = resolve_effect_enum(node, request, "applyTo", "foreground");
    let max_x = styled_grid.width().saturating_sub(1).max(1) as f64;
    let max_y = styled_grid.height().saturating_sub(1).max(1) as f64;
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let nx = x as f64 / max_x;
        let ny = y as f64 / max_y;
        let projection = (nx * angle.cos() + ny * angle.sin() + 1.0) / 2.0;
        let color = sample_gradient(&gradient, (projection.clamp(0.0, 1.0) as f32) * intensity);
        apply_shader_style(styled_grid, x, y, &apply_to, &color.rgba_label());
    }
}

#[derive(Clone, Debug)]
struct ResolvedGradient {
    stops: Vec<ResolvedGradientStop>,
    color_space: String,
}

#[derive(Clone, Copy, Debug)]
struct ResolvedGradientStop {
    position: f32,
    color: ResolvedColor,
}

fn resolve_linear_gradient(node: &NodeSpec, request: &PlayerSampleRequest) -> ResolvedGradient {
    if let Some(gradient) = resolve_effect_gradient(node, request, "gradient") {
        return gradient.into();
    }
    ResolvedGradient {
        stops: vec![
            ResolvedGradientStop {
                position: 0.0,
                color: resolve_effect_color(
                    node,
                    request,
                    "startColor",
                    ResolvedColor::rgb(255, 100, 50),
                ),
            },
            ResolvedGradientStop {
                position: 1.0,
                color: resolve_effect_color(
                    node,
                    request,
                    "endColor",
                    ResolvedColor::rgb(50, 100, 255),
                ),
            },
        ],
        color_space: resolve_effect_enum(node, request, "colorSpace", "rgb"),
    }
}

impl From<GradientSpec> for ResolvedGradient {
    fn from(gradient: GradientSpec) -> Self {
        let mut stops: Vec<ResolvedGradientStop> = gradient
            .stops
            .into_iter()
            .map(|stop| ResolvedGradientStop {
                position: stop.position as f32,
                color: ResolvedColor::new(stop.color.r, stop.color.g, stop.color.b, stop.color.a),
            })
            .collect();
        stops.sort_by(|left, right| left.position.total_cmp(&right.position));
        Self {
            stops,
            color_space: gradient.space,
        }
    }
}

fn sample_gradient(gradient: &ResolvedGradient, position: f32) -> ResolvedColor {
    let position = position.clamp(0.0, 1.0);
    let Some(first) = gradient.stops.first().copied() else {
        return ResolvedColor::rgb(255, 100, 50);
    };
    if position <= first.position {
        return first.color;
    }
    for pair in gradient.stops.windows(2) {
        let left = pair[0];
        let right = pair[1];
        if position <= right.position {
            let span = (right.position - left.position).max(f32::EPSILON);
            return left.color.lerp_in_color_space(
                right.color,
                (position - left.position) / span,
                &gradient.color_space,
            );
        }
    }
    gradient.stops.last().map_or(first.color, |stop| stop.color)
}

fn apply_reveal_wipe(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(node, request, "color", ResolvedColor::rgb(120, 220, 255));
    let direction = resolve_effect_enum(node, request, "direction", "leftToRight");
    let threshold = request.phase_t.clamp(0.0, 1.0);
    let width = styled_grid.width().max(1);
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let progress = if direction == "rightToLeft" {
            (width - x) as f64 / width as f64
        } else {
            (x + 1) as f64 / width as f64
        };
        if progress <= threshold {
            apply_shader_style(styled_grid, x, y, "foreground", &color.rgba_label());
        }
    }
}

fn apply_highlighter(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(node, request, "color", ResolvedColor::rgb(255, 225, 90));
    let blend_strength = resolve_effect_number(node, request, "blendStrength", 1.0).clamp(0.0, 1.0);
    let text_contrast = resolve_effect_number(node, request, "textContrast", 0.0).clamp(0.0, 1.0);
    let soft_edge = resolve_effect_enum(node, request, "softEdge", "true") != "false";
    let direction = resolve_effect_enum(node, request, "direction", "leftToRight");
    let mode = resolve_effect_enum(node, request, "mode", "band");
    let row_mask = resolve_effect_integer(node, request, "rowMask", -1);
    let band_width = (resolve_effect_number(node, request, "bandWidth", 3.0).max(1.0)
        * (1.0 + if soft_edge { 0.5 } else { 0.0 })) as usize;
    let span = if matches!(direction.as_str(), "topToBottom" | "bottomToTop") {
        styled_grid.height().max(1)
    } else {
        styled_grid.width().max(1)
    };
    let center = (request.phase_t.clamp(0.0, 1.0) * span as f64).round() as isize;
    let active_color = color.lerp(
        ResolvedColor::rgb(255, 255, 255),
        (text_contrast * 0.25) as f32,
    );
    let apply_to = if mode == "row" { "both" } else { "background" };
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let axis = if matches!(direction.as_str(), "topToBottom" | "bottomToTop") {
            y
        } else {
            x
        };
        if row_mask >= 0 && y as i64 != row_mask {
            continue;
        }
        if (axis as isize - center).unsigned_abs() <= band_width {
            let color = active_color.lerp(color, blend_strength as f32).rgba_label();
            apply_shader_style(styled_grid, x, y, apply_to, &color);
        }
    }
}

fn apply_focus_field(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(node, request, "color", ResolvedColor::rgb(120, 180, 255));
    let rect_x = resolve_effect_number(node, request, "rectX", 0.0);
    let rect_y = resolve_effect_number(node, request, "rectY", 0.0);
    let rect_width = resolve_effect_number(node, request, "rectWidth", styled_grid.width() as f64);
    let rect_height =
        resolve_effect_number(node, request, "rectHeight", styled_grid.height() as f64);
    let center_x = resolve_effect_number(node, request, "centerX", rect_x + rect_width / 2.0);
    let center_y = resolve_effect_number(node, request, "centerY", rect_y + rect_height / 2.0);
    let shape = resolve_effect_enum(node, request, "shape", "circle");
    let radius_x = resolve_effect_number(
        node,
        request,
        "radiusX",
        resolve_effect_number(node, request, "radius", 4.0),
    )
    .max(0.5);
    let radius_y = resolve_effect_number(node, request, "radiusY", radius_x).max(0.5);
    let feather = resolve_effect_number(node, request, "feather", 0.0).clamp(0.0, 1.0);
    let intensity = resolve_effect_number(node, request, "intensity", 1.0).clamp(0.0, 1.0) as f32;
    let focus_color = color.lerp(ResolvedColor::rgb(255, 255, 255), (1.0 - intensity) * 0.25);
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let dx = (x as f64 - center_x).abs();
        let dy = (y as f64 - center_y).abs();
        let inside = if shape == "rect" {
            x as f64 >= rect_x
                && y as f64 >= rect_y
                && x as f64 <= rect_x + rect_width
                && y as f64 <= rect_y + rect_height
        } else {
            let normalized =
                (dx / radius_x).mul_add(dx / radius_x, (dy / radius_y) * (dy / radius_y));
            normalized.sqrt() <= 1.0 + feather
        };
        if inside {
            apply_shader_style(styled_grid, x, y, "foreground", &focus_color.rgba_label());
        }
    }
}

fn apply_glisten_band(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(node, request, "color", ResolvedColor::rgb(255, 255, 255));
    let blend_strength =
        resolve_effect_number(node, request, "blendStrength", 1.0).clamp(0.0, 1.0) as f32;
    let angle = resolve_effect_number(node, request, "angleDeg", 0.0).to_radians();
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let head = resolve_effect_number(node, request, "head", 0.0).clamp(0.0, 1.0);
    let tail = resolve_effect_number(node, request, "tail", 1.0).clamp(0.0, 1.0);
    let band_width = resolve_effect_number(node, request, "bandWidth", 2.0).max(1.0)
        * (tail - head).abs().max(0.25);
    let direction = resolve_effect_enum(node, request, "direction", "leftToRight");
    let mut center = (request.phase_t * speed).fract();
    if direction == "rightToLeft" {
        center = 1.0 - center;
    }
    let width = styled_grid.width().saturating_sub(1).max(1) as f64;
    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let nx = x as f64 / width;
        let diagonal = (nx * angle.cos()
            + y as f64 * angle.sin() / styled_grid.height().max(1) as f64)
            .fract();
        if (diagonal - center).abs() <= band_width / width {
            let color = color
                .lerp(ResolvedColor::rgb(255, 255, 255), blend_strength)
                .rgba_label();
            apply_shader_style(styled_grid, x, y, "foreground", &color);
        }
    }
}

fn apply_wayfinding_node(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let active = resolve_effect_integer(node, request, "currentIndex", 0).max(0) as usize;
    let node_count = resolve_effect_integer(node, request, "nodes", 1).max(1) as usize;
    let previous_strength =
        resolve_effect_number(node, request, "previousStrength", 0.3).clamp(0.0, 1.0) as f32;
    let future_strength =
        resolve_effect_number(node, request, "futureStrength", 0.4).clamp(0.0, 1.0) as f32;
    let intensity = resolve_effect_number(node, request, "intensity", 1.0).clamp(0.0, 1.0) as f32;
    let radius = resolve_effect_number(node, request, "radius", 1.0).max(0.0) as usize;
    let color = resolve_effect_color(
        node,
        request,
        "activeColor",
        resolve_effect_color(node, request, "color", ResolvedColor::rgb(80, 255, 160)),
    );
    let cells = collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid);
    if cells.is_empty() {
        return;
    }
    let active_index = active % cells.len().min(node_count).max(1);
    for offset in 0..=radius {
        for (index, strength) in [
            (active_index.saturating_sub(offset), previous_strength),
            (
                (active_index + offset).min(cells.len() - 1),
                future_strength,
            ),
        ] {
            if let Some((x, y)) = cells.get(index).copied() {
                let color = color.lerp(
                    ResolvedColor::rgb(255, 255, 255),
                    (1.0 - strength * intensity).clamp(0.0, 1.0),
                );
                apply_shader_style(styled_grid, x, y, "both", &color.rgba_label());
            }
        }
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
    let explicit_position = resolve_effect_number(node, request, "position", f64::NAN);
    let progress = if explicit_position.is_nan() {
        request.phase_t * speed
    } else {
        explicit_position.clamp(0.0, 1.0)
    };
    let offset = (progress * edge_cells.len() as f64).round() as usize % edge_cells.len();
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

fn apply_barber_pole(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let stripe_color = resolve_effect_color(
        node,
        request,
        "stripeColor",
        ResolvedColor::rgb(255, 80, 80),
    );
    let background_color = resolve_effect_color(
        node,
        request,
        "backgroundColor",
        ResolvedColor::rgb(40, 10, 20),
    );
    let stripe_width = resolve_effect_integer(node, request, "stripeWidth", 3).max(1) as usize;
    let gap_width =
        resolve_effect_integer(node, request, "gapWidth", stripe_width as i64).max(1) as usize;
    let angle = resolve_effect_number(node, request, "angleDeg", 45.0).to_radians();
    let speed = resolve_effect_number(node, request, "speed", 0.0);
    let apply_to = resolve_effect_enum(node, request, "applyTo", "background");
    let period = stripe_width + gap_width;
    let phase_offset = (request.loop_t.unwrap_or(request.phase_t) * speed * period as f64).round();

    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let projection = x as f64 * angle.cos() + y as f64 * angle.sin() + phase_offset;
        let position = projection.rem_euclid(period as f64) as usize;
        let color = if position < stripe_width {
            stripe_color
        } else {
            background_color
        };
        apply_shader_style(styled_grid, x, y, &apply_to, &color.rgba_label());
    }
}

fn apply_diffusion(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) {
    let color = resolve_effect_color(node, request, "color", ResolvedColor::rgb(80, 180, 255));
    let center_x = resolve_effect_number(
        node,
        request,
        "centerX",
        styled_grid.width().saturating_sub(1) as f64 / 2.0,
    );
    let center_y = resolve_effect_number(
        node,
        request,
        "centerY",
        styled_grid.height().saturating_sub(1) as f64 / 2.0,
    );
    let radius = resolve_effect_number(node, request, "radius", 8.0).max(0.5);
    let intensity = resolve_effect_number(node, request, "intensity", 1.0).clamp(0.0, 1.0) as f32;
    let apply_to = resolve_effect_enum(node, request, "applyTo", "background");

    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let distance = ((x as f64 - center_x).powi(2) + (y as f64 - center_y).powi(2)).sqrt();
        let falloff = (1.0 - distance / radius).clamp(0.0, 1.0) as f32;
        if falloff > 0.0 {
            let diffused_color =
                ResolvedColor::rgb(0, 0, 0).lerp(color, (falloff * intensity).clamp(0.0, 1.0));
            apply_shader_style(styled_grid, x, y, &apply_to, &diffused_color.rgba_label());
        }
    }
}

fn apply_radar(node: &NodeSpec, request: &PlayerSampleRequest, styled_grid: &mut PlayerStyledGrid) {
    let color = resolve_effect_color(node, request, "color", ResolvedColor::rgb(80, 255, 160));
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let tail_length = resolve_effect_number(node, request, "tailLength", 0.25).clamp(0.01, 1.0);
    let apply_to = resolve_effect_enum(node, request, "applyTo", "foreground");
    let center_x = styled_grid.width().saturating_sub(1) as f64 / 2.0;
    let center_y = styled_grid.height().saturating_sub(1) as f64 / 2.0;
    let sweep = (request.loop_t.unwrap_or(request.phase_t) * speed).fract();

    for (x, y) in collect_styled_grid_scope_cells(node.scope.as_ref(), styled_grid) {
        let angle = ((y as f64 - center_y).atan2(x as f64 - center_x) + std::f64::consts::TAU)
            % std::f64::consts::TAU;
        let position = angle / std::f64::consts::TAU;
        let distance_behind = (sweep - position).rem_euclid(1.0);
        if distance_behind <= tail_length {
            let strength = (1.0 - distance_behind / tail_length) as f32;
            let radar_color = ResolvedColor::rgb(0, 0, 0).lerp(color, strength);
            apply_shader_style(styled_grid, x, y, &apply_to, &radar_color.rgba_label());
        }
    }
}

fn apply_shader_style(
    styled_grid: &mut PlayerStyledGrid,
    x: usize,
    y: usize,
    apply_to: &str,
    color: &str,
) {
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
    let foreground = if matches!(apply_to, "foreground" | "both") {
        color.to_string()
    } else {
        existing_foreground
    };
    let background = if matches!(apply_to, "background" | "both") {
        color.to_string()
    } else {
        existing_background
    };
    styled_grid.set_cell_style(x, y, &foreground, &background, vec![], None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use tui_vfx_contract::{EffectId, EffectInputId, NodeId, Value, ValueSource};

    #[test]
    fn test_fnc_samples_each_gradient_segment() {
        let gradient = ResolvedGradient {
            stops: vec![
                ResolvedGradientStop {
                    position: 0.0,
                    color: ResolvedColor::rgb(0, 0, 0),
                },
                ResolvedGradientStop {
                    position: 0.5,
                    color: ResolvedColor::rgb(100, 0, 0),
                },
                ResolvedGradientStop {
                    position: 1.0,
                    color: ResolvedColor::rgb(100, 100, 0),
                },
            ],
            color_space: "rgb".to_string(),
        };

        assert_eq!(
            sample_gradient(&gradient, 0.25),
            ResolvedColor::rgb(50, 0, 0)
        );
        assert_eq!(
            sample_gradient(&gradient, 0.75),
            ResolvedColor::rgb(100, 50, 0)
        );
    }

    #[test]
    fn test_fnc_shader_style_targets_foreground_background_or_both() {
        let mut foreground_grid = PlayerStyledGrid::blank(1, 1, false);
        foreground_grid.set_cell_style(0, 0, "oldForeground", "oldBackground", vec![], None);
        apply_shader_style(&mut foreground_grid, 0, 0, "foreground", "rgba(1,2,3,255)");
        assert_eq!(foreground_grid.cells()[0].foreground, "rgba(1,2,3,255)");
        assert_eq!(foreground_grid.cells()[0].background, "oldBackground");

        let mut background_grid = PlayerStyledGrid::blank(1, 1, false);
        background_grid.set_cell_style(0, 0, "oldForeground", "oldBackground", vec![], None);
        apply_shader_style(&mut background_grid, 0, 0, "background", "rgba(1,2,3,255)");
        assert_eq!(background_grid.cells()[0].foreground, "oldForeground");
        assert_eq!(background_grid.cells()[0].background, "rgba(1,2,3,255)");

        let mut both_grid = PlayerStyledGrid::blank(1, 1, false);
        apply_shader_style(&mut both_grid, 0, 0, "both", "rgba(1,2,3,255)");
        assert_eq!(both_grid.cells()[0].foreground, "rgba(1,2,3,255)");
        assert_eq!(both_grid.cells()[0].background, "rgba(1,2,3,255)");
    }

    #[test]
    fn test_fnc_border_sweep_uses_explicit_position_when_present() {
        let node = NodeSpec {
            id: NodeId::new("borderSweep"),
            effect: EffectId::new("shader.borderSweep"),
            inputs: BTreeMap::from([
                (
                    EffectInputId::new("position"),
                    ValueSource::Literal {
                        value: Value::Number(0.0),
                    },
                ),
                (
                    EffectInputId::new("length"),
                    ValueSource::Literal {
                        value: Value::Integer(1),
                    },
                ),
            ]),
            outputs: BTreeMap::new(),
            scope: None,
            cell_write_policy: None,
            role_write_policy: None,
        };
        let mut grid = PlayerStyledGrid::blank(3, 3, false);
        let request = PlayerSampleRequest {
            phase_t: 0.75,
            ..PlayerSampleRequest::default()
        };

        apply_border_sweep(&node, &request, &mut grid);

        assert_eq!(grid.cells()[0].role.as_deref(), Some("Border"));
        assert!(
            grid.cells()
                .iter()
                .skip(1)
                .all(|cell| cell.role.as_deref() != Some("Border"))
        );
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_shader_primitive.rs</FILE> - <DESC>Apply shader primitives to player styled grids</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
