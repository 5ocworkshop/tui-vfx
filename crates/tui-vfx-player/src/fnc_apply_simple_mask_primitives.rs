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
    let origin = resolve_effect_enum(node, request, "origin", "center");
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
                if normalized_origin_distance(x, y, width, height, MaskShape::Circle, &origin)
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

/// Apply a deterministic cellular mask to text-grid rows.
pub(crate) fn apply_mask_cellular(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let cell_size = resolve_effect_integer(node, request, "cellSize", 2).max(1) as usize;
    let seed = resolve_effect_integer(node, request, "seed", 7).max(0) as usize;
    let threshold = resolve_effect_number(node, request, "threshold", 0.5).clamp(0.0, 1.0);
    let reveal = request.phase_t.clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                let cell_x = x / cell_size;
                let cell_y = y / cell_size;
                let noise = deterministic_cell_noise(cell_x, cell_y, seed);
                if noise <= (threshold * 0.5 + reveal * 0.75).min(1.0) {
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
    normalized_origin_distance(x, y, width, height, shape, "center")
}

fn normalized_origin_distance(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    shape: MaskShape,
    origin: &str,
) -> f64 {
    let (origin_x, origin_y) = origin_point(width, height, origin);
    let half_width = ((width.saturating_sub(1)) as f64 / 2.0).max(1.0);
    let half_height = ((height.saturating_sub(1)) as f64 / 2.0).max(1.0);
    let dx = (x as f64 - origin_x).abs() / half_width;
    let dy = (y as f64 - origin_y).abs() / half_height;
    match shape {
        MaskShape::Circle => (dx.mul_add(dx, dy * dy)).sqrt(),
        MaskShape::Diamond => dx + dy,
    }
}

fn origin_point(width: usize, height: usize, origin: &str) -> (f64, f64) {
    let right = width.saturating_sub(1) as f64;
    let bottom = height.saturating_sub(1) as f64;
    match origin {
        "topLeft" => (0.0, 0.0),
        "topRight" => (right, 0.0),
        "bottomLeft" => (0.0, bottom),
        "bottomRight" => (right, bottom),
        _ => (right / 2.0, bottom / 2.0),
    }
}

fn deterministic_cell_noise(cell_x: usize, cell_y: usize, seed: usize) -> f64 {
    let mixed = cell_x
        .wrapping_mul(73_856_093)
        .wrapping_add(cell_y.wrapping_mul(19_349_663))
        .wrapping_add(seed.wrapping_mul(83_492_791));
    (mixed % 1000) as f64 / 999.0
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs</FILE> - <DESC>Apply simple text-grid mask primitives</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
