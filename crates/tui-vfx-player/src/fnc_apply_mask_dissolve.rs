// <FILE>crates/tui-vfx-player/src/fnc_apply_mask_dissolve.rs</FILE> - <DESC>Apply text-grid dissolve mask</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: add deterministic text-grid mask support.</WCTX>
// <CLOG>0.1.0: INIT — add seeded dissolve adapter over row glyphs.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{PlayerSampleRequest, fnc_resolve_effect_input::resolve_effect_integer};

/// Apply a deterministic dissolve mask to text-grid rows.
pub(crate) fn apply_mask_dissolve(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    let seed = resolve_effect_integer(node, request, "seed", 42).max(0) as u64;
    let chunk_size = resolve_effect_integer(node, request, "chunkSize", 1).max(1) as usize;
    let reveal_threshold = request.phase_t.clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = dissolve_row(row, y, seed, chunk_size, reveal_threshold);
    }
}

fn dissolve_row(
    row: &str,
    y: usize,
    seed: u64,
    chunk_size: usize,
    reveal_threshold: f64,
) -> String {
    row.chars()
        .enumerate()
        .map(|(x, glyph)| {
            if glyph == ' ' || cell_noise(seed, x / chunk_size, y / chunk_size) <= reveal_threshold
            {
                glyph
            } else {
                ' '
            }
        })
        .collect()
}

fn cell_noise(seed: u64, x: usize, y: usize) -> f64 {
    let mut hash = seed ^ 0xcbf2_9ce4_8422_2325u64;
    for value in [x as u64, y as u64] {
        hash ^= value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    (hash % 10_000) as f64 / 9_999.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnc_dissolve_row_keeps_all_glyphs_at_full_threshold() {
        assert_eq!(dissolve_row("AB CD", 0, 42, 1, 1.0), "AB CD");
    }

    #[test]
    fn test_fnc_cell_noise_is_deterministic_and_normalized() {
        let first = cell_noise(7, 2, 3);
        let second = cell_noise(7, 2, 3);

        assert_eq!(first, second);
        assert!((0.0..=1.0).contains(&first));
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_mask_dissolve.rs</FILE> - <DESC>Apply text-grid dissolve mask</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
