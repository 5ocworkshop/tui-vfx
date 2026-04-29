// <FILE>crates/tui-vfx-player/src/cls_player_inventory_report.rs</FILE> - <DESC>Aggregate inventory report DTO for K0 debug fixture gates</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep aggregate report DTO OFPF-sized.</WCTX>
// <CLOG>0.2.0: PATCH — split DTO rows and aggregation helpers into focused files.</CLOG>

use tui_vfx_contract::DescriptorCatalog;

use crate::{
    DescriptorPackReport, PlayerInventoryEffect, PlayerInventoryRecipe, PlayerInventorySource,
    PlayerInventorySummary,
    fnc_aggregate_player_inventory_effects::aggregate_effects,
    fnc_aggregate_player_inventory_sources::aggregate_sources,
    fnc_collect_descriptor_inventory_ids::{catalog_effect_ids, catalog_source_ids},
    fnc_summarize_player_inventory::summarize_inventory,
};

/// Stable machine-readable report for recipe fixture inventory gates.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventoryReport {
    /// Stable inventory report schema label.
    pub schema_version: &'static str,
    /// Root path or invocation label.
    pub root: String,
    /// Descriptor packs loaded for this inventory invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Aggregate fixture, descriptor, and adapter coverage counts.
    pub summary: PlayerInventorySummary,
    /// Per-recipe inventory entries.
    pub recipes: Vec<PlayerInventoryRecipe>,
    /// Per-effect descriptor/recipe/adapter coverage entries.
    pub effects: Vec<PlayerInventoryEffect>,
    /// Per-source descriptor/recipe/adapter coverage entries.
    pub sources: Vec<PlayerInventorySource>,
}

impl PlayerInventoryReport {
    /// Build an aggregate inventory report from per-recipe inventory entries.
    pub fn new(
        root: String,
        descriptor_packs: Vec<DescriptorPackReport>,
        catalog: &DescriptorCatalog,
        recipes: Vec<PlayerInventoryRecipe>,
    ) -> Self {
        let descriptor_effect_ids = catalog_effect_ids(catalog);
        let descriptor_source_ids = catalog_source_ids(catalog);
        let effects = aggregate_effects(&recipes, &descriptor_effect_ids);
        let sources = aggregate_sources(&recipes, &descriptor_source_ids);
        let summary = summarize_inventory(&recipes, &descriptor_effect_ids, &effects, &sources);
        Self {
            schema_version: "v3.1.player.inventory.1",
            root,
            descriptor_packs,
            summary,
            recipes,
            effects,
            sources,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_inventory_report.rs</FILE> - <DESC>Aggregate inventory report DTO for K0 debug fixture gates</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
