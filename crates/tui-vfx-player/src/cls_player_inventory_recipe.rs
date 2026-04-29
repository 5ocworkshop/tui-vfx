// <FILE>crates/tui-vfx-player/src/cls_player_inventory_recipe.rs</FILE> - <DESC>Per-recipe inventory DTO for K0 debug fixture gates</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.0: report recipe coverage without changing rendering semantics.</WCTX>
// <CLOG>0.1.0: INIT — add per-recipe ids, descriptor coverage, unsupported adapters, and errors.</CLOG>

use crate::{PlayerError, PlayerStatus};

/// Machine-readable inventory for one canonical v3.1 recipe file.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventoryRecipe {
    /// Filesystem path for the inventoried recipe.
    pub path: String,
    /// Stable canonical recipe id, or a sentinel when the file cannot deserialize.
    pub recipe_id: String,
    /// Current K0 render status for the sampled recipe.
    pub status: PlayerStatus,
    /// Source descriptor ids referenced by source instances.
    pub source_ids: Vec<String>,
    /// Effect descriptor ids referenced by graph nodes.
    pub effect_ids: Vec<String>,
    /// Effect ids represented by this recipe and covered by loaded descriptor packs.
    pub descriptor_covered_effect_ids: Vec<String>,
    /// Effect ids represented by this recipe but missing from loaded descriptor packs.
    pub missing_descriptor_effect_ids: Vec<String>,
    /// Source ids represented by this recipe and covered by loaded descriptor packs.
    pub descriptor_covered_source_ids: Vec<String>,
    /// Source ids represented by this recipe but missing from loaded descriptor packs.
    pub missing_descriptor_source_ids: Vec<String>,
    /// Effect ids that K0 reports as unsupported adapters for this recipe.
    pub unsupported_effect_ids: Vec<String>,
    /// Hard load/validation/render diagnostics observed while inventorying.
    pub errors: Vec<PlayerError>,
}

// <FILE>crates/tui-vfx-player/src/cls_player_inventory_recipe.rs</FILE> - <DESC>Per-recipe inventory DTO for K0 debug fixture gates</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
