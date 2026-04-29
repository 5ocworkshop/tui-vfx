// <FILE>crates/tui-vfx-player/src/fnc_build_player_frame.rs</FILE> - <DESC>Build deterministic player frame DTOs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: separate frame construction from RecipePlayer orchestration.</WCTX>
// <CLOG>0.1.0: INIT — compute dimensions, non-empty cells, and render hash.</CLOG>

use tui_vfx_contract::RecipeDocument;

use crate::{PlayerError, PlayerFrame, PlayerSampleRequest, fnc_render_hash::render_hash};

/// Build a deterministic frame from sampled text rows and diagnostics.
pub fn build_player_frame(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
    rows: &[String],
    errors: &[PlayerError],
) -> PlayerFrame {
    let width = rows.first().map_or(0, String::len);
    let height = rows.len();
    let mut parts = vec![
        recipe.id.as_str().to_string(),
        format!(
            "{:?}:{:.6}:{:?}",
            request.phase, request.phase_t, request.loop_t
        ),
    ];
    parts.extend(recipe.graph.order.iter().map(|id| id.as_str().to_string()));
    parts.extend(rows.iter().cloned());
    parts.extend(errors.iter().map(|error| error.code.clone()));
    PlayerFrame {
        width,
        height,
        render_hash: render_hash(&parts),
        non_empty_cells: PlayerFrame::count_non_empty(rows),
        rows: rows.to_vec(),
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_player_frame.rs</FILE> - <DESC>Build deterministic player frame DTOs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
