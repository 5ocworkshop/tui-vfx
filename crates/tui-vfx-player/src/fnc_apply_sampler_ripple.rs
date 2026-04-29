// <FILE>crates/tui-vfx-player/src/fnc_apply_sampler_ripple.rs</FILE> - <DESC>Apply text-grid ripple sampler</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: add deterministic coordinate-sampler evidence.</WCTX>
// <CLOG>0.1.0: INIT — add row-wise ripple offset over glyph rows.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{PlayerSampleRequest, fnc_resolve_effect_input::resolve_effect_number};

/// Apply a row-wise ripple offset to text-grid rows.
pub(crate) fn apply_sampler_ripple(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let amplitude = resolve_effect_number(node, request, "amplitude", 1.5).max(0.0);
    let wavelength = resolve_effect_number(node, request, "wavelength", 5.0).max(0.1);
    let speed = resolve_effect_number(node, request, "speed", 2.0).max(0.0);
    let sample_t = request.loop_t.unwrap_or(request.phase_t);
    for (y, row) in rows.iter_mut().enumerate() {
        let offset = ripple_offset(y, sample_t, amplitude, wavelength, speed);
        *row = shift_row(row, offset);
    }
}

fn ripple_offset(y: usize, sample_t: f64, amplitude: f64, wavelength: f64, speed: f64) -> isize {
    let row_phase = ((y + 1) as f64 / wavelength) * std::f64::consts::TAU;
    let time_phase = sample_t * speed * std::f64::consts::TAU;
    (amplitude * (row_phase + time_phase).sin()).round() as isize
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
    fn test_fnc_shift_row_moves_glyphs_right() {
        assert_eq!(shift_row("ABCD", 1), " ABC");
    }

    #[test]
    fn test_fnc_shift_row_moves_glyphs_left() {
        assert_eq!(shift_row("ABCD", -1), "BCD ");
    }

    #[test]
    fn test_fnc_ripple_offset_is_deterministic() {
        assert_eq!(
            ripple_offset(3, 0.25, 2.0, 5.0, 1.5),
            ripple_offset(3, 0.25, 2.0, 5.0, 1.5)
        );
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_sampler_ripple.rs</FILE> - <DESC>Apply text-grid ripple sampler</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
