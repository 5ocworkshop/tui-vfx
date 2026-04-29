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
