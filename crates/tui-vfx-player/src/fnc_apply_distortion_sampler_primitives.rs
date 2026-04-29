// <FILE>crates/tui-vfx-player/src/fnc_apply_distortion_sampler_primitives.rs</FILE> - <DESC>Apply bounded distortion-sampler adapters to text-grid rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 descriptor/adapter migration: add honest distortion-sampler evidence.</WCTX>
// <CLOG>0.1.0: INIT — add shredder, fault-line, and radial-twist approximations.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest,
    fnc_resolve_effect_input::{resolve_effect_integer, resolve_effect_number},
};

pub(crate) fn apply_distortion_sampler_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) -> bool {
    match node.effect.as_str() {
        "sampler.shredder" => apply_shredder(node, request, rows),
        "sampler.faultLine" => apply_fault_line(node, request, rows),
        "sampler.radialTwist" => apply_radial_twist(node, request, rows),
        _ => return false,
    }
    true
}

fn apply_shredder(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let slice_width = resolve_effect_integer(node, request, "sliceWidth", 2).max(1) as usize;
    let offset = resolve_effect_integer(node, request, "offset", 1);
    for (y, row) in rows.iter_mut().enumerate() {
        if y % 2 == 0 {
            *row = shift_chunks(row, slice_width, offset);
        }
    }
}

fn apply_fault_line(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let offset = resolve_effect_integer(node, request, "offset", 2);
    let split = rows.len() / 2;
    for row in rows.iter_mut().skip(split) {
        *row = shift_row(row, offset);
    }
}

fn apply_radial_twist(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let strength = resolve_effect_number(node, request, "strength", 1.0).max(0.0);
    let center = rows.len() as f64 / 2.0;
    for (y, row) in rows.iter_mut().enumerate() {
        let offset = ((y as f64 - center) * strength * request.phase_t).round() as i64;
        *row = shift_row(row, offset);
    }
}

fn shift_chunks(row: &str, chunk_width: usize, offset: i64) -> String {
    row.chars()
        .collect::<Vec<_>>()
        .chunks(chunk_width)
        .enumerate()
        .flat_map(|(index, chunk)| {
            let text = chunk.iter().collect::<String>();
            let local_offset = if index % 2 == 0 { offset } else { -offset };
            shift_row(&text, local_offset).chars().collect::<Vec<_>>()
        })
        .collect()
}

fn shift_row(row: &str, offset: i64) -> String {
    let chars = row.chars().collect::<Vec<_>>();
    let width = chars.len();
    let mut shifted = vec![' '; width];
    for (x, glyph) in chars.into_iter().enumerate() {
        let target_x = x as i64 + offset;
        if target_x >= 0 && (target_x as usize) < width {
            shifted[target_x as usize] = glyph;
        }
    }
    shifted.into_iter().collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_distortion_sampler_primitives.rs</FILE> - <DESC>Apply bounded distortion-sampler adapters to text-grid rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
