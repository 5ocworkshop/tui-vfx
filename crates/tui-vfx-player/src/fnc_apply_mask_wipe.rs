// <FILE>crates/tui-vfx-player/src/fnc_apply_mask_wipe.rs</FILE> - <DESC>Apply text-grid wipe mask</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Primitive adapter work: add field-aware wipe mask evidence.</WCTX>
// <CLOG>0.2.0: MINOR — support V2 center-out and edges-in horizontal wipe evidence.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest,
    fnc_resolve_effect_input::{resolve_effect_bool, resolve_effect_enum},
};

/// Apply a wipe mask to text-grid rows.
pub(crate) fn apply_mask_wipe(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let reveal = eased_reveal(
        request.phase_t.clamp(0.0, 1.0),
        &resolve_effect_enum(node, request, "easing", "linear"),
    );
    let direction = resolve_effect_enum(node, request, "direction", "leftToRight");
    let soft_edge = resolve_effect_bool(node, request, "softEdge", false);
    let height = rows.len().max(1);
    let vertical_cutoff = wipe_cutoff(height, reveal, soft_edge);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count();
        let cutoff = wipe_cutoff(width, reveal, soft_edge);
        *row = row
            .chars()
            .enumerate()
            .map(|(index, ch)| {
                if wipe_keeps_cell(index, y, width, height, cutoff, vertical_cutoff, &direction) {
                    ch
                } else {
                    ' '
                }
            })
            .collect();
    }
}

fn eased_reveal(reveal: f64, easing: &str) -> f64 {
    match easing {
        "easeIn" => reveal * reveal,
        "easeOut" => 1.0 - (1.0 - reveal) * (1.0 - reveal),
        "easeInOut" if reveal < 0.5 => 2.0 * reveal * reveal,
        "easeInOut" => 1.0 - (-2.0 * reveal + 2.0).powi(2) / 2.0,
        _ => reveal,
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

fn wipe_keeps_cell(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    horizontal_cutoff: usize,
    vertical_cutoff: usize,
    direction: &str,
) -> bool {
    match direction {
        "rightToLeft" => x >= width.saturating_sub(horizontal_cutoff),
        "horizontalCenterOut" | "horizontal_center_out" => {
            horizontal_center_out_keeps_cell(x, width, horizontal_cutoff)
        }
        "horizontalEdgesIn" | "horizontal_edges_in" => {
            horizontal_edges_in_keeps_cell(x, width, horizontal_cutoff)
        }
        "topToBottom" => y < vertical_cutoff,
        "bottomToTop" => y >= height.saturating_sub(vertical_cutoff),
        "outFromTopLeft" => {
            x.saturating_add(y) <= horizontal_cutoff.saturating_add(vertical_cutoff)
        }
        "outFromTopRight" => {
            width.saturating_sub(1).saturating_sub(x).saturating_add(y)
                <= horizontal_cutoff.saturating_add(vertical_cutoff)
        }
        "outFromBottomLeft" => {
            x.saturating_add(height.saturating_sub(1).saturating_sub(y))
                <= horizontal_cutoff.saturating_add(vertical_cutoff)
        }
        "outFromBottomRight" => {
            width
                .saturating_sub(1)
                .saturating_sub(x)
                .saturating_add(height.saturating_sub(1).saturating_sub(y))
                <= horizontal_cutoff.saturating_add(vertical_cutoff)
        }
        "inToTopLeft" => {
            x.saturating_add(y)
                >= width.saturating_add(height).saturating_sub(
                    horizontal_cutoff
                        .saturating_add(vertical_cutoff)
                        .saturating_add(2),
                )
        }
        "inToTopRight" => {
            width.saturating_sub(1).saturating_sub(x).saturating_add(y)
                >= width.saturating_add(height).saturating_sub(
                    horizontal_cutoff
                        .saturating_add(vertical_cutoff)
                        .saturating_add(2),
                )
        }
        "inToBottomLeft" => {
            x.saturating_add(height.saturating_sub(1).saturating_sub(y))
                >= width.saturating_add(height).saturating_sub(
                    horizontal_cutoff
                        .saturating_add(vertical_cutoff)
                        .saturating_add(2),
                )
        }
        "inToBottomRight" => {
            width
                .saturating_sub(1)
                .saturating_sub(x)
                .saturating_add(height.saturating_sub(1).saturating_sub(y))
                >= width.saturating_add(height).saturating_sub(
                    horizontal_cutoff
                        .saturating_add(vertical_cutoff)
                        .saturating_add(2),
                )
        }
        _ => x < horizontal_cutoff,
    }
}

fn horizontal_center_out_keeps_cell(x: usize, width: usize, cutoff: usize) -> bool {
    let center_twice = width.saturating_sub(1);
    let x_twice = x.saturating_mul(2);
    x_twice.abs_diff(center_twice) <= cutoff.saturating_mul(2).saturating_sub(1)
}

fn horizontal_edges_in_keeps_cell(x: usize, width: usize, cutoff: usize) -> bool {
    let edge_reveal = cutoff / 2;
    x < edge_reveal || x >= width.saturating_sub(edge_reveal)
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_mask_wipe.rs</FILE> - <DESC>Apply text-grid wipe mask</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
