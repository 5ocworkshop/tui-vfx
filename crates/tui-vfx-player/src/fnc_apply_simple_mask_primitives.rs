// <FILE>crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs</FILE> - <DESC>Apply simple text-grid mask primitives</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>K2.9 simple masks: render bounded descriptor additions as honest text-grid evidence.</WCTX>
// <CLOG>0.1.1: PATCH — avoid rebinding resolved iris shape text as mask geometry.
// 0.1.0: INIT — add blinds, radial, iris, and diamond masks.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest,
    fnc_resolve_effect_input::{
        resolve_effect_bool, resolve_effect_enum, resolve_effect_integer, resolve_effect_number,
    },
};

/// Apply a blinds mask to text-grid rows.
pub(crate) fn apply_mask_blinds(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let orientation = resolve_effect_enum(node, request, "orientation", "horizontal");
    let count = resolve_effect_integer(node, request, "count", 4).max(1) as usize;
    let reveal = request.phase_t.clamp(0.0, 1.0);
    let height = rows.len().max(1);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                let band = blinds_band(x, y, width, height, count, &orientation);
                if (band + 1) as f64 / count as f64 <= reveal {
                    glyph
                } else {
                    ' '
                }
            })
            .collect();
    }
}

/// Apply a radial mask to text-grid rows.
pub(crate) fn apply_mask_radial(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let _center_only_origin = resolve_effect_enum(node, request, "origin", "center");
    let soft_edge = resolve_effect_bool(node, request, "softEdge", true);
    apply_shape_mask(rows, request.phase_t, soft_edge, MaskShape::Circle);
}

/// Apply a materialize mask to text-grid rows.
pub(crate) fn apply_mask_materialize(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let _origin = resolve_effect_enum(node, request, "origin", "center");
    let soft_edge = resolve_effect_bool(node, request, "softEdge", true);
    let chunk_size = resolve_effect_integer(node, request, "chunkSize", 1).max(1) as usize;
    let noise = resolve_effect_number(node, request, "noise", 0.0).clamp(0.0, 1.0);
    let seed = resolve_effect_integer(node, request, "seed", 42).max(0) as usize;
    let reveal = reveal_threshold((request.phase_t + noise * 0.1).clamp(0.0, 1.0), soft_edge);
    let height = rows.len().max(1);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                let quantized_x = x / chunk_size;
                let quantized_y = y / chunk_size;
                let jitter = ((quantized_x * 31 + quantized_y * 17 + seed) % 100) as f64 / 100.0;
                if normalized_distance(x, y, width, height, MaskShape::Circle)
                    <= reveal + jitter * noise
                {
                    glyph
                } else {
                    ' '
                }
            })
            .collect();
    }
}

/// Apply an iris mask to text-grid rows.
pub(crate) fn apply_mask_iris(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let shape_value = resolve_effect_enum(node, request, "shape", "circle");
    let soft_edge = resolve_effect_bool(node, request, "softEdge", true);
    let shape = match shape_value.as_str() {
        "diamond" => MaskShape::Diamond,
        _ => MaskShape::Circle,
    };
    apply_shape_mask(rows, request.phase_t, soft_edge, shape);
}

/// Apply a diamond mask to text-grid rows.
pub(crate) fn apply_mask_diamond(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let soft_edge = resolve_effect_bool(node, request, "softEdge", true);
    apply_shape_mask(rows, request.phase_t, soft_edge, MaskShape::Diamond);
}

fn blinds_band(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    count: usize,
    orientation: &str,
) -> usize {
    if orientation == "vertical" {
        (x * count / width).min(count.saturating_sub(1))
    } else {
        (y * count / height).min(count.saturating_sub(1))
    }
}

#[derive(Clone, Copy)]
enum MaskShape {
    Circle,
    Diamond,
}

fn apply_shape_mask(rows: &mut [String], phase_t: f64, soft_edge: bool, shape: MaskShape) {
    let reveal = reveal_threshold(phase_t, soft_edge);
    let height = rows.len().max(1);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if normalized_distance(x, y, width, height, shape) <= reveal {
                    glyph
                } else {
                    ' '
                }
            })
            .collect();
    }
}

fn reveal_threshold(phase_t: f64, soft_edge: bool) -> f64 {
    let reveal = phase_t.clamp(0.0, 1.0);
    if soft_edge {
        (reveal + 0.06).min(1.0)
    } else {
        reveal
    }
}

fn normalized_distance(x: usize, y: usize, width: usize, height: usize, shape: MaskShape) -> f64 {
    let half_width = ((width.saturating_sub(1)) as f64 / 2.0).max(1.0);
    let half_height = ((height.saturating_sub(1)) as f64 / 2.0).max(1.0);
    let dx = (x as f64 - half_width).abs() / half_width;
    let dy = (y as f64 - half_height).abs() / half_height;
    match shape {
        MaskShape::Circle => (dx.mul_add(dx, dy * dy)).sqrt(),
        MaskShape::Diamond => dx + dy,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs</FILE> - <DESC>Apply simple text-grid mask primitives</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
