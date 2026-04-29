// <FILE>crates/tui-vfx-player/src/fnc_apply_graph_effects.rs</FILE> - <DESC>Apply graph effect adapters to player frame evidence</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Primitive adapter work: route text-grid and styled-cell adapters.</WCTX>
// <CLOG>0.3.0: MINOR — route K2.5 styled primitive adapters through player styled grids.
// 0.2.0: MINOR — add dissolve mask and ripple sampler support.
// 0.1.0: INIT — add primitive adapter switch and structured unsupported diagnostics.</CLOG>

use tui_vfx_contract::RecipeDocument;

use crate::{
    PlayerError, PlayerSampleRequest, PlayerStyledGrid,
    fnc_apply_mask_dissolve::apply_mask_dissolve, fnc_apply_sampler_ripple::apply_sampler_ripple,
    fnc_apply_styled_primitive::apply_styled_primitive,
};

/// Apply supported graph effects and collect unsupported adapter diagnostics.
pub fn apply_graph_effects(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    errors: &mut Vec<PlayerError>,
) {
    for node_id in &recipe.graph.order {
        let Some(node) = recipe.graph.nodes.get(node_id) else {
            continue;
        };
        match node.effect.as_str() {
            "filter.dim" | "filter.tint" | "filter.invert" | "filter.greyscale" => {}
            "mask.none" | "sampler.sineWave" => {}
            "mask.wipe" => {
                apply_wipe(rows, request.phase_t);
                styled_grid.sync_glyphs_from_rows(rows);
            }
            "mask.checkers" => {
                apply_checkers(rows, request.phase_t);
                styled_grid.sync_glyphs_from_rows(rows);
            }
            "mask.dissolve" => {
                apply_mask_dissolve(node, request, rows);
                styled_grid.sync_glyphs_from_rows(rows);
            }
            "sampler.ripple" => {
                apply_sampler_ripple(node, request, rows);
                styled_grid.sync_glyphs_from_rows(rows);
            }
            _ if apply_styled_primitive(node, request, styled_grid) => {}
            effect => errors.push(PlayerError::new(
                "unsupportedEffectAdapter",
                format!("graph.nodes.{}.effect", node.id.as_str()),
                format!("No player adapter registered for {effect}"),
                Some("Implement the effect adapter or keep this fixture in unsupported smoke status."),
                serde_json::json!({ "effect": effect, "node": node.id.as_str() }),
            )),
        }
    }
}

fn apply_wipe(rows: &mut [String], phase_t: f64) {
    let reveal = phase_t.clamp(0.0, 1.0);
    for row in rows {
        let cutoff = ((row.chars().count() as f64) * reveal).round() as usize;
        *row = row
            .chars()
            .enumerate()
            .map(|(index, ch)| if index < cutoff { ch } else { ' ' })
            .collect();
    }
}

fn apply_checkers(rows: &mut [String], phase_t: f64) {
    if phase_t >= 1.0 {
        return;
    }
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, ch)| if (x + y) % 2 == 0 { ch } else { ' ' })
            .collect();
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_graph_effects.rs</FILE> - <DESC>Apply graph effect adapters to player frame evidence</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
