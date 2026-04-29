// <FILE>crates/tui-vfx-player/src/fnc_apply_mask_wipe.rs</FILE> - <DESC>Apply text-grid wipe mask</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: add field-aware wipe mask evidence.</WCTX>
// <CLOG>0.1.0: INIT — add direction and soft-edge aware wipe mask.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest,
    fnc_resolve_effect_input::{resolve_effect_bool, resolve_effect_enum},
};

/// Apply a wipe mask to text-grid rows.
pub(crate) fn apply_mask_wipe(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let reveal = request.phase_t.clamp(0.0, 1.0);
    let direction = resolve_effect_enum(node, request, "direction", "leftToRight");
    let soft_edge = resolve_effect_bool(node, request, "softEdge", false);
    for row in rows {
        let width = row.chars().count();
        let cutoff = wipe_cutoff(width, reveal, soft_edge);
        *row = row
            .chars()
            .enumerate()
            .map(|(index, ch)| {
                if wipe_keeps_cell(index, width, cutoff, &direction) {
                    ch
                } else {
                    ' '
                }
            })
            .collect();
    }
}

fn wipe_cutoff(width: usize, reveal: f64, soft_edge: bool) -> usize {
    let scaled = width as f64 * reveal;
    if soft_edge {
        scaled.round() as usize
    } else {
        scaled.floor() as usize
    }
}

fn wipe_keeps_cell(index: usize, width: usize, cutoff: usize, direction: &str) -> bool {
    match direction {
        "rightToLeft" => index >= width.saturating_sub(cutoff),
        "topToBottom" | "bottomToTop" => index < cutoff,
        _ => index < cutoff,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_mask_wipe.rs</FILE> - <DESC>Apply text-grid wipe mask</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
