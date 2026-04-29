// <FILE>crates/tui-vfx-player/src/fnc_inventory_recipe_file.rs</FILE> - <DESC>Inventory one recipe JSON file through the K0 player</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep one-file inventory wrapper OFPF-sized.</WCTX>
// <CLOG>0.2.0: PATCH — split id extraction and file-error construction helpers.</CLOG>

use std::path::Path;

use tui_vfx_contract::{DescriptorCatalog, RecipeDocument};

use crate::{
    PlayerInventoryRecipe, PlayerSampleRequest, RecipePlayer,
    fnc_collect_descriptor_inventory_ids::{catalog_effect_ids, catalog_source_ids},
    fnc_extract_recipe_inventory_ids::{
        difference, intersection, recipe_effect_ids, recipe_source_ids, unsupported_effect_ids,
    },
    fnc_player_inventory_file_error::player_inventory_file_error,
};

/// Inventory one canonical recipe JSON file without changing renderer behavior.
pub fn inventory_recipe_file(
    player: &RecipePlayer,
    catalog: &DescriptorCatalog,
    path: &Path,
    request: &PlayerSampleRequest,
) -> PlayerInventoryRecipe {
    let path_label = path.display().to_string();
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            return player_inventory_file_error(path_label, "readFailed", error.to_string());
        }
    };
    let recipe: RecipeDocument = match serde_json::from_str(&text) {
        Ok(recipe) => recipe,
        Err(error) => {
            return player_inventory_file_error(path_label, "deserializeFailed", error.to_string());
        }
    };
    recipe_inventory(player, catalog, path_label, &recipe, request)
}

fn recipe_inventory(
    player: &RecipePlayer,
    catalog: &DescriptorCatalog,
    path_label: String,
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
) -> PlayerInventoryRecipe {
    let descriptor_effect_ids = catalog_effect_ids(catalog);
    let descriptor_source_ids = catalog_source_ids(catalog);
    let source_ids = recipe_source_ids(recipe);
    let effect_ids = recipe_effect_ids(recipe);
    let report = player.render_recipe(recipe, request);
    PlayerInventoryRecipe {
        path: path_label,
        recipe_id: recipe.id.as_str().to_string(),
        status: report.status,
        descriptor_covered_effect_ids: intersection(&effect_ids, &descriptor_effect_ids),
        missing_descriptor_effect_ids: difference(&effect_ids, &descriptor_effect_ids),
        descriptor_covered_source_ids: intersection(&source_ids, &descriptor_source_ids),
        missing_descriptor_source_ids: difference(&source_ids, &descriptor_source_ids),
        source_ids: source_ids.into_iter().collect(),
        effect_ids: effect_ids.into_iter().collect(),
        unsupported_effect_ids: unsupported_effect_ids(&report.errors),
        errors: report.errors,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_inventory_recipe_file.rs</FILE> - <DESC>Inventory one recipe JSON file through the K0 player</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
