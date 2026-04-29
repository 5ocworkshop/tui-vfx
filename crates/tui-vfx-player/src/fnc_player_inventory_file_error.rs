// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_file_error.rs</FILE> - <DESC>Build inventory file error rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep recipe inventory loading OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split unreadable recipe inventory error construction.</CLOG>

use crate::{PlayerError, PlayerInventoryRecipe, PlayerStatus};

/// Build an inventory row for unreadable or invalid recipe files.
pub(crate) fn player_inventory_file_error(
    path: String,
    code: &str,
    message: String,
) -> PlayerInventoryRecipe {
    PlayerInventoryRecipe {
        path: path.clone(),
        recipe_id: "<unreadable>".to_string(),
        status: PlayerStatus::Error,
        source_ids: vec![],
        effect_ids: vec![],
        descriptor_covered_effect_ids: vec![],
        missing_descriptor_effect_ids: vec![],
        descriptor_covered_source_ids: vec![],
        missing_descriptor_source_ids: vec![],
        unsupported_effect_ids: vec![],
        errors: vec![PlayerError::new(
            code,
            path,
            message,
            Some("Ensure the path points to a readable canonical v3.1 RecipeDocument JSON file."),
            serde_json::Value::Null,
        )],
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_file_error.rs</FILE> - <DESC>Build inventory file error rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
