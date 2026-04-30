// <FILE>crates/tui-vfx-player/src/fnc_render_recipe_file_ir.rs</FILE> - <DESC>Render one recipe JSON file into player render IR</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player render evidence work: expose filesystem bridge for render-ir CLI.</WCTX>
// <CLOG>0.1.0: INIT — add path-bearing render IR wrapper.</CLOG>

use std::path::Path;

use tui_vfx_contract::RecipeDocument;

use crate::{
    PlayerError, PlayerFrame, PlayerFrameReport, PlayerRenderClockSample, PlayerRenderIrReport,
    PlayerSampleRequest, PlayerStatus, RecipePlayer, fnc_render_hash::render_hash,
};

/// Render one canonical recipe JSON file into a player render IR report.
pub fn render_recipe_file_ir(
    player: &RecipePlayer,
    path: &Path,
    request: &PlayerSampleRequest,
) -> PlayerRenderIrReport {
    let path_label = path.display().to_string();
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => return file_error(path_label, request, "readFailed", error.to_string()),
    };
    let recipe: RecipeDocument = match serde_json::from_str(&text) {
        Ok(recipe) => recipe,
        Err(error) => {
            return file_error(path_label, request, "deserializeFailed", error.to_string());
        }
    };
    let mut report = player.render_recipe_ir(&recipe, request);
    report.path = Some(path_label);
    report
}

fn file_error(
    path: String,
    request: &PlayerSampleRequest,
    code: &str,
    message: String,
) -> PlayerRenderIrReport {
    let frame = PlayerFrame {
        width: request.width.unwrap_or(0),
        height: request.height.unwrap_or(0),
        render_hash: render_hash(&[path.clone(), code.to_string(), message.clone()]),
        non_empty_cells: 0,
        rows: vec![],
        styled_grid: None,
    };
    let mut report = PlayerFrameReport::from_frame(
        "<unreadable>".to_string(),
        frame,
        PlayerStatus::Error,
        request,
        false,
        vec![PlayerError::new(
            code,
            path.clone(),
            message,
            Some("Ensure the path points to a readable canonical v3.1 RecipeDocument JSON file."),
            serde_json::Value::Null,
        )],
    );
    report.path = Some(path);
    PlayerRenderIrReport {
        schema_version: "v3.1.player.renderIr.1",
        recipe_id: report.recipe_id,
        path: report.path,
        status: report.status,
        phase: report.phase,
        phase_t: report.phase_t,
        loop_t: report.loop_t,
        clock: PlayerRenderClockSample {
            mode: "unavailable".to_string(),
            period_ms: None,
            absolute_t_ms: request.absolute_t_ms,
            loop_t: request.loop_t,
        },
        width: report.width,
        height: report.height,
        render_hash: report.render_hash,
        non_empty_cells: report.non_empty_cells,
        rows: report.rows,
        styled_cells: vec![],
        provenance: vec![],
        layers: vec![],
        graph_values: vec![],
        errors: report.errors,
        warnings: report.warnings,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_render_recipe_file_ir.rs</FILE> - <DESC>Render one recipe JSON file into player render IR</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
