// <FILE>crates/tui-vfx-player/src/fnc_collect_descriptor_inventory_ids.rs</FILE> - <DESC>Collect descriptor ids for inventory reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate descriptor id collection.</WCTX>
// <CLOG>0.1.0: INIT — split descriptor effect/source id helpers from report DTO.</CLOG>

use std::collections::BTreeSet;

use tui_vfx_contract::DescriptorCatalog;

/// Collect effect ids supplied by loaded descriptor packs.
pub(crate) fn catalog_effect_ids(catalog: &DescriptorCatalog) -> BTreeSet<String> {
    catalog
        .packs
        .values()
        .flat_map(|pack| pack.effects.keys())
        .map(|id| id.as_str().to_string())
        .collect()
}

/// Collect source ids supplied by loaded descriptor packs.
pub(crate) fn catalog_source_ids(catalog: &DescriptorCatalog) -> BTreeSet<String> {
    catalog
        .packs
        .values()
        .flat_map(|pack| pack.source_descriptors.keys())
        .map(|id| id.as_str().to_string())
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_descriptor_inventory_ids.rs</FILE> - <DESC>Collect descriptor ids for inventory reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
