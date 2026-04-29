// <FILE>crates/tui-vfx-player/src/fnc_inventory_recipe_paths.rs</FILE> - <DESC>Inventory collected recipe paths through the K0 player</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.0: aggregate recursive debug fixture coverage for migration gates.</WCTX>
// <CLOG>0.1.0: INIT — build v3.1.player.inventory.1 reports from collected recipe paths.</CLOG>

use std::path::PathBuf;

use tui_vfx_contract::DescriptorCatalog;

use crate::{
    DescriptorPackReport, PlayerInventoryReport, PlayerSampleRequest, RecipePlayer,
    fnc_inventory_recipe_file::inventory_recipe_file,
};

/// Inventory collected recipe paths and aggregate descriptor/adapter coverage.
pub fn inventory_recipe_paths(
    player: &RecipePlayer,
    catalog: &DescriptorCatalog,
    descriptor_packs: Vec<DescriptorPackReport>,
    paths: &[PathBuf],
    root: String,
    request: &PlayerSampleRequest,
) -> PlayerInventoryReport {
    let recipes = paths
        .iter()
        .map(|path| inventory_recipe_file(player, catalog, path, request))
        .collect::<Vec<_>>();
    PlayerInventoryReport::new(root, descriptor_packs, catalog, recipes)
}

// <FILE>crates/tui-vfx-player/src/fnc_inventory_recipe_paths.rs</FILE> - <DESC>Inventory collected recipe paths through the K0 player</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
