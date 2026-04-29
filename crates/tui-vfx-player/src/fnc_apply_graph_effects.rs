// <FILE>crates/tui-vfx-player/src/fnc_apply_graph_effects.rs</FILE> - <DESC>Apply K0 graph effect adapters to text-grid rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: separate supported and unsupported effect adapter handling.</WCTX>
// <CLOG>0.1.0: INIT — add primitive adapter switch and structured unsupported diagnostics.</CLOG>

use tui_vfx_contract::RecipeDocument;

use crate::{PlayerError, PlayerSampleRequest};

/// Apply supported graph effects and collect unsupported adapter diagnostics.
pub fn apply_graph_effects(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
    rows: &mut [String],
    errors: &mut Vec<PlayerError>,
) {
    for node_id in &recipe.graph.order {
        let Some(node) = recipe.graph.nodes.get(node_id) else {
            continue;
        };
        match node.effect.as_str() {
            "filter.dim" | "filter.tint" | "filter.invert" | "filter.greyscale" => {}
            "mask.none" | "sampler.sineWave" => {}
            "mask.wipe" => apply_wipe(rows, request.phase_t),
            "mask.checkers" => apply_checkers(rows, request.phase_t),
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

// <FILE>crates/tui-vfx-player/src/fnc_apply_graph_effects.rs</FILE> - <DESC>Apply K0 graph effect adapters to text-grid rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
