// <FILE>crates/tui-vfx-player/src/fnc_apply_sampler_sine_wave.rs</FILE> - <DESC>Apply text-grid sine-wave sampler</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: add deterministic sine-wave coordinate-sampler evidence.</WCTX>
// <CLOG>0.1.0: INIT — add field-aware sine-wave sampler over glyph rows.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest,
    fnc_resolve_effect_input::{resolve_effect_enum, resolve_effect_number},
};

/// Apply a sine-wave offset to text-grid rows.
pub(crate) fn apply_sampler_sine_wave(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let axis = resolve_effect_enum(node, request, "axis", "x");
    let amplitude = resolve_effect_number(node, request, "amplitude", 1.0).max(0.0);
    let frequency = resolve_effect_number(node, request, "frequency", 1.0).max(0.0);
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let phase_offset = resolve_effect_number(node, request, "phaseOffset", 0.0);
    let sample_t = request.loop_t.unwrap_or(request.phase_t);
    if axis == "y" {
        apply_vertical_wave(rows, sample_t, amplitude, frequency, speed, phase_offset);
    } else {
        apply_horizontal_wave(rows, sample_t, amplitude, frequency, speed, phase_offset);
    }
}

fn apply_horizontal_wave(
    rows: &mut [String],
    sample_t: f64,
    amplitude: f64,
    frequency: f64,
    speed: f64,
    phase_offset: f64,
) {
    for (y, row) in rows.iter_mut().enumerate() {
        let phase = y as f64 * frequency + sample_t * speed + phase_offset;
        *row = shift_row(row, sine_offset(phase, amplitude));
    }
}

fn apply_vertical_wave(
    rows: &mut [String],
    sample_t: f64,
    amplitude: f64,
    frequency: f64,
    speed: f64,
    phase_offset: f64,
) {
    if rows.is_empty() {
        return;
    }
    let grid = rows
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = grid.len();
    let width = grid.iter().map(Vec::len).max().unwrap_or(0);
    let mut shifted = vec![vec![' '; width]; height];
    for (y, row) in grid.iter().enumerate() {
        for (x, glyph) in row.iter().copied().enumerate() {
            let phase = x as f64 * frequency + sample_t * speed + phase_offset;
            let target_y = y as isize + sine_offset(phase, amplitude);
            if target_y >= 0 && (target_y as usize) < height {
                shifted[target_y as usize][x] = glyph;
            }
        }
    }
    for (row, shifted_row) in rows.iter_mut().zip(shifted) {
        *row = shifted_row.into_iter().collect();
    }
}

fn sine_offset(phase: f64, amplitude: f64) -> isize {
    (amplitude * (phase * std::f64::consts::TAU).sin()).round() as isize
}

fn shift_row(row: &str, offset: isize) -> String {
    let chars = row.chars().collect::<Vec<_>>();
    let width = chars.len();
    let mut shifted = vec![' '; width];
    for (x, glyph) in chars.into_iter().enumerate() {
        let target_x = x as isize + offset;
        if target_x >= 0 && (target_x as usize) < width {
            shifted[target_x as usize] = glyph;
        }
    }
    shifted.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnc_sine_offset_is_deterministic() {
        assert_eq!(sine_offset(0.25, 2.0), sine_offset(0.25, 2.0));
    }

    #[test]
    fn test_fnc_shift_row_preserves_width() {
        assert_eq!(shift_row("ABCD", 2).chars().count(), 4);
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_sampler_sine_wave.rs</FILE> - <DESC>Apply text-grid sine-wave sampler</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
