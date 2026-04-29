// <FILE>crates/tui-vfx-player/src/cls_player_inventory_effect.rs</FILE> - <DESC>Inventory report effect DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep inventory DTOs OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split effect coverage DTO from aggregate report helpers.</CLOG>

/// Descriptor, recipe, and adapter coverage for one effect id.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventoryEffect {
    /// Effect descriptor id.
    pub id: String,
    /// Whether the id is supplied by a loaded descriptor pack.
    pub descriptor_covered: bool,
    /// Whether any inventoried recipe references the id.
    pub represented_by_recipes: bool,
    /// Current K0 adapter classification for this effect id.
    pub adapter_status: String,
    /// Recipe paths that reference the id.
    pub recipe_paths: Vec<String>,
}

// <FILE>crates/tui-vfx-player/src/cls_player_inventory_effect.rs</FILE> - <DESC>Inventory report effect DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
