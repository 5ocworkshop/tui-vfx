// <FILE>crates/tui-vfx-player/src/fnc_apply_graph_effects.rs</FILE> - <DESC>Apply graph effect adapters to player frame evidence</DESC>
// <VERS>VERSION: 0.5.1</VERS>
// <WCTX>Primitive adapter work: route text-grid and styled-cell adapters.</WCTX>
// <CLOG>0.5.1: PATCH — consolidate text-grid adapter synchronization.
// 0.5.0: MINOR — route K2.9 simple mask adapters.
// 0.4.0: MINOR — route field-aware filter adapters through player styled grids.</CLOG>

use tui_vfx_contract::RecipeDocument;

use crate::{
    PlayerError, PlayerSampleRequest, PlayerStyledGrid,
    fnc_apply_filter_primitive::apply_filter_primitive,
    fnc_apply_mask_checkers::apply_mask_checkers,
    fnc_apply_mask_dissolve::apply_mask_dissolve,
    fnc_apply_mask_wipe::apply_mask_wipe,
    fnc_apply_sampler_ripple::apply_sampler_ripple,
    fnc_apply_sampler_sine_wave::apply_sampler_sine_wave,
    fnc_apply_simple_mask_primitives::{
        apply_mask_blinds, apply_mask_diamond, apply_mask_iris, apply_mask_radial,
    },
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
        let sync_text_grid = match node.effect.as_str() {
            "filter.dim" | "filter.tint" | "filter.invert" | "filter.greyscale" => {
                apply_filter_primitive(node, request, styled_grid);
                false
            }
            "mask.none" => false,
            "sampler.sineWave" => {
                apply_sampler_sine_wave(node, request, rows);
                true
            }
            "mask.wipe" => {
                apply_mask_wipe(node, request, rows);
                true
            }
            "mask.checkers" => {
                apply_mask_checkers(node, request, rows);
                true
            }
            "mask.dissolve" => {
                apply_mask_dissolve(node, request, rows);
                true
            }
            "mask.blinds" => {
                apply_mask_blinds(node, request, rows);
                true
            }
            "mask.radial" => {
                apply_mask_radial(node, request, rows);
                true
            }
            "mask.iris" => {
                apply_mask_iris(node, request, rows);
                true
            }
            "mask.diamond" => {
                apply_mask_diamond(node, request, rows);
                true
            }
            "sampler.ripple" => {
                apply_sampler_ripple(node, request, rows);
                true
            }
            _ if apply_styled_primitive(node, request, styled_grid) => false,
            effect => {
                errors.push(PlayerError::new(
                    "unsupportedEffectAdapter",
                    format!("graph.nodes.{}.effect", node.id.as_str()),
                    format!("No player adapter registered for {effect}"),
                    Some(
                        "Implement the effect adapter or keep this fixture in unsupported smoke status.",
                    ),
                    serde_json::json!({ "effect": effect, "node": node.id.as_str() }),
                ));
                false
            }
        };
        if sync_text_grid {
            styled_grid.sync_glyphs_from_rows(rows);
        }
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_graph_effects.rs</FILE> - <DESC>Apply graph effect adapters to player frame evidence</DESC>
// <VERS>END OF VERSION: 0.5.1</VERS>
