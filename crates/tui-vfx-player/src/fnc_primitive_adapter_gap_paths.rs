// <FILE>crates/tui-vfx-player/src/fnc_primitive_adapter_gap_paths.rs</FILE> - <DESC>Classify primitive adapter gaps for recipe paths</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: expose focused gap report entry point.</WCTX>
// <CLOG>0.1.0: INIT — build primitive gap reports from collected paths.</CLOG>

use std::path::PathBuf;

use tui_vfx_contract::DescriptorCatalog;

use crate::{
    DescriptorPackReport, PlayerPrimitiveAdapterGapReport, PlayerSampleRequest, RecipePlayer,
    build_primitive_adapter_gap_report, inventory_recipe_paths,
};

/// Classify primitive adapter gaps for collected recipe paths.
pub fn primitive_adapter_gap_paths(
    player: &RecipePlayer,
    catalog: &DescriptorCatalog,
    descriptor_packs: Vec<DescriptorPackReport>,
    paths: &[PathBuf],
    root: String,
    request: &PlayerSampleRequest,
) -> PlayerPrimitiveAdapterGapReport {
    let inventory = inventory_recipe_paths(
        player,
        catalog,
        descriptor_packs.clone(),
        paths,
        root.clone(),
        request,
    );
    build_primitive_adapter_gap_report(root, descriptor_packs, &inventory)
}

// <FILE>crates/tui-vfx-player/src/fnc_primitive_adapter_gap_paths.rs</FILE> - <DESC>Classify primitive adapter gaps for recipe paths</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
