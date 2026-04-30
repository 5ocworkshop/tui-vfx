// <FILE>crates/tui-vfx-player/src/fnc_apply_distortion_sampler_primitives.rs</FILE> - <DESC>Apply bounded distortion-sampler adapters to text-grid rows</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>v3.1 descriptor/adapter migration: add honest distortion-sampler evidence.</WCTX>
// <CLOG>0.2.0: MINOR — align fault-line sampler with V2 seed/intensity/splitBias dynamics.
// 0.1.0: INIT — add shredder, fault-line, and radial-twist approximations.</CLOG>

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
        "sampler.crt" => apply_crt_sampler(node, request, rows),
        "sampler.crtJitter" => apply_crt_jitter_sampler(node, request, rows),
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
    let explicit_offset = resolve_optional_effect_integer(node, request, "offset");
    let split = explicit_offset
        .map(|_| rows.len() / 2)
        .unwrap_or_else(|| fault_line_split(rows.len(), node, request));
    let dynamic_offset =
        explicit_offset.unwrap_or_else(|| fault_line_dynamic_offset(node, request));
    for (row_index, row) in rows.iter_mut().enumerate() {
        if row_index < split {
            *row = shift_row(row, dynamic_offset);
        } else {
            *row = shift_row(row, -dynamic_offset);
        }
    }
}

fn fault_line_split(row_count: usize, node: &NodeSpec, request: &PlayerSampleRequest) -> usize {
    if row_count < 3 {
        return row_count / 2;
    }
    let seed = resolve_effect_integer(node, request, "seed", 0).max(0) as u64;
    let split_bias = resolve_effect_number(node, request, "splitBias", 0.0).clamp(-1.0, 1.0);
    let base_split = ((seed.wrapping_mul(31)) % row_count as u64) as f64;
    (base_split + split_bias * row_count as f64 * 0.3).clamp(1.0, (row_count - 1) as f64) as usize
}

fn fault_line_dynamic_offset(node: &NodeSpec, request: &PlayerSampleRequest) -> i64 {
    let intensity = resolve_effect_number(node, request, "intensity", 1.0).max(0.0);
    ((1.0 - request.phase_t.clamp(0.0, 1.0)) * 20.0 * intensity).round() as i64
}

fn resolve_optional_effect_integer(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    key: &str,
) -> Option<i64> {
    node.inputs
        .get(&tui_vfx_contract::EffectInputId::new(key))
        .map(|_| resolve_effect_integer(node, request, key, 0))
}

fn apply_radial_twist(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let strength = resolve_effect_number(node, request, "strength", 1.0).max(0.0);
    let center = rows.len() as f64 / 2.0;
    for (y, row) in rows.iter_mut().enumerate() {
        let offset = ((y as f64 - center) * strength * request.phase_t).round() as i64;
        *row = shift_row(row, offset);
    }
}

fn apply_crt_sampler(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let curvature = resolve_effect_number(node, request, "curvature", 0.4).clamp(0.0, 1.0);
    let scanline_strength =
        resolve_effect_number(node, request, "scanlineStrength", 0.35).clamp(0.0, 1.0);
    let jitter = resolve_effect_number(node, request, "jitter", 0.0).max(0.0);
    let center = rows.len() as f64 / 2.0;
    for (y, row) in rows.iter_mut().enumerate() {
        let bow = ((y as f64 - center) * curvature * request.phase_t).round() as i64;
        let time_jitter = ((request.loop_t.unwrap_or(request.phase_t) * 10.0 + y as f64).sin()
            * jitter)
            .round() as i64;
        let shifted = shift_row(row, bow + time_jitter);
        if scanline_strength > 0.0 && y % 2 == 1 {
            *row = drop_every_nth_glyph(&shifted, scanline_strength);
        } else {
            *row = shifted;
        }
    }
}

fn apply_crt_jitter_sampler(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let amplitude = resolve_effect_number(node, request, "amplitude", 1.0).max(0.0);
    let frequency = resolve_effect_number(node, request, "frequency", 2.0).max(0.0);
    let seed = resolve_effect_integer(node, request, "seed", 13).max(0) as f64;
    let time = request.loop_t.unwrap_or(request.phase_t);
    for (y, row) in rows.iter_mut().enumerate() {
        let wave = ((time * frequency + y as f64 * 0.37 + seed * 0.01).sin() * amplitude).round();
        *row = shift_row(row, wave as i64);
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

fn drop_every_nth_glyph(row: &str, scanline_strength: f64) -> String {
    let interval = if scanline_strength >= 0.66 {
        3
    } else if scanline_strength >= 0.33 {
        5
    } else {
        8
    };
    row.chars()
        .enumerate()
        .map(|(index, glyph)| if index % interval == 0 { ' ' } else { glyph })
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_distortion_sampler_primitives.rs</FILE> - <DESC>Apply bounded distortion-sampler adapters to text-grid rows</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
