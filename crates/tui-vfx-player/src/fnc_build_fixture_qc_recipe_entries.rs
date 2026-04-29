// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_recipe_entries.rs</FILE> - <DESC>Build fixture QC recipe entries</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: keep fixture QC per-recipe validation separate.</WCTX>
// <CLOG>0.1.0: INIT — extract per-recipe fixture QC entry construction.</CLOG>

use std::path::{Path, PathBuf};

use tui_vfx_contract::{DescriptorCatalog, RecipeDocument};

use crate::{PlayerFixtureQcRecipe, PlayerFrameReport, PlayerStatus};

/// Build per-recipe fixture QC entries.
pub(crate) fn build_fixture_qc_recipe_entries(
    catalog: &DescriptorCatalog,
    paths: &[PathBuf],
    frames: &[PlayerFrameReport],
) -> Vec<PlayerFixtureQcRecipe> {
    paths
        .iter()
        .zip(frames)
        .map(|(path, frame)| recipe_entry(catalog, path, frame))
        .collect()
}

fn recipe_entry(
    catalog: &DescriptorCatalog,
    path: &Path,
    frame: &PlayerFrameReport,
) -> PlayerFixtureQcRecipe {
    let validation_error = validate_recipe_path(catalog, path).err();
    let validated = validation_error.is_none();
    let mut errors = validation_error.into_iter().collect::<Vec<_>>();
    errors.extend(
        frame
            .errors
            .iter()
            .map(|error| format!("{}: {}", error.code, error.message)),
    );
    PlayerFixtureQcRecipe {
        recipe_path: path.display().to_string(),
        validated,
        player_status: status_label(frame.status).to_string(),
        render_hash: frame.render_hash,
        non_empty_cells: frame.non_empty_cells,
        errors,
    }
}

fn validate_recipe_path(catalog: &DescriptorCatalog, path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read `{}` failed: {error}", path.display()))?;
    let recipe = serde_json::from_str::<RecipeDocument>(&text)
        .map_err(|error| format!("parse `{}` failed: {error}", path.display()))?;
    recipe
        .validate_with_catalog(catalog)
        .map_err(|error| format!("validate `{}` failed: {error:?}", path.display()))
}

fn status_label(status: PlayerStatus) -> &'static str {
    match status {
        PlayerStatus::Rendered => "rendered",
        PlayerStatus::Unsupported => "unsupported",
        PlayerStatus::Error => "error",
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_recipe_entries.rs</FILE> - <DESC>Build fixture QC recipe entries</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
