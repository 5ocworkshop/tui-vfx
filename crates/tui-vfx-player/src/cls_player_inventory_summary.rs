// <FILE>crates/tui-vfx-player/src/cls_player_inventory_summary.rs</FILE> - <DESC>Inventory report summary DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep inventory DTOs OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split inventory summary DTO from aggregate report helpers.</CLOG>

/// Aggregate counts for one player inventory invocation.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventorySummary {
    /// Number of recipe files inventoried.
    pub total_recipes: usize,
    /// Number of recipe files that rendered within the K0 supported subset.
    pub rendered: usize,
    /// Number of valid recipe files requiring unsupported adapters.
    pub unsupported: usize,
    /// Number of recipe files with hard load/validation errors.
    pub errors: usize,
    /// Number of effect ids supplied by loaded descriptor packs.
    pub descriptor_effect_ids: usize,
    /// Number of descriptor-pack effect ids represented by recipes.
    pub represented_effect_ids: usize,
    /// Number of descriptor-pack effect ids not represented by recipes.
    pub unrepresented_effect_ids: usize,
    /// Number of represented effect ids currently reported as unsupported adapters.
    pub unsupported_effect_ids: usize,
    /// Number of unique source ids represented by recipes or descriptor packs.
    pub source_ids: usize,
}

// <FILE>crates/tui-vfx-player/src/cls_player_inventory_summary.rs</FILE> - <DESC>Inventory report summary DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
