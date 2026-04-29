// <FILE>crates/tui-vfx-player/src/fnc_render_recipe_file.rs</FILE> - <DESC>Render one recipe JSON file through RecipePlayer</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: bridge filesystem recipes to sampled frame reports.</WCTX>
// <CLOG>0.1.0: INIT — add read/deserialize/render wrapper with path-bearing diagnostics.</CLOG>

use std::path::Path;

use tui_vfx_contract::RecipeDocument;

use crate::{
    PlayerError, PlayerFrame, PlayerFrameReport, PlayerSampleRequest, PlayerStatus, RecipePlayer,
    fnc_render_hash::render_hash,
};

/// Render one canonical recipe JSON file into a frame report.
pub fn render_recipe_file(
    player: &RecipePlayer,
    path: &Path,
    request: &PlayerSampleRequest,
) -> PlayerFrameReport {
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
    let mut report = player.render_recipe(&recipe, request);
    report.path = Some(path_label);
    report
}

fn file_error(
    path: String,
    request: &PlayerSampleRequest,
    code: &str,
    message: String,
) -> PlayerFrameReport {
    let frame = PlayerFrame {
        width: request.width.unwrap_or(0),
        height: request.height.unwrap_or(0),
        render_hash: render_hash(&[path.clone(), code.to_string(), message.clone()]),
        non_empty_cells: 0,
        rows: vec![],
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
    report
}

// <FILE>crates/tui-vfx-player/src/fnc_render_recipe_file.rs</FILE> - <DESC>Render one recipe JSON file through RecipePlayer</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
