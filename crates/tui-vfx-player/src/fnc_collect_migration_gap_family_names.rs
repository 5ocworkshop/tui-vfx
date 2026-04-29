// <FILE>crates/tui-vfx-player/src/fnc_collect_migration_gap_family_names.rs</FILE> - <DESC>Collect ordered migration gap family names</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep migration gap builder OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split canonical family ordering from report construction.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use crate::fnc_collect_debug_recipe_family_inventory::DebugRecipeFamilyInventory;

const FAMILY_ORDER: &[&str] = &[
    "baseline",
    "filters",
    "masks",
    "samplers",
    "shaders/primitives",
    "shaders/compositions",
    "styles",
    "content",
    "scene",
    "shadows",
    "complex",
    "event_driven_dwell",
    "signals",
    "easings",
    "subcell_shapes",
    "motion_routes",
    "loopback",
    "bindable_rates",
    "fixtures",
    "other",
];

/// Return canonical family names plus any families discovered in inventories.
pub(crate) fn migration_gap_family_names(
    legacy: &BTreeMap<String, DebugRecipeFamilyInventory>,
    v31: &BTreeMap<String, DebugRecipeFamilyInventory>,
) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut names = Vec::new();
    for family in FAMILY_ORDER
        .iter()
        .map(|family| family.to_string())
        .chain(legacy.keys().chain(v31.keys()).cloned())
    {
        if seen.insert(family.clone()) {
            names.push(family);
        }
    }
    names
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_migration_gap_family_names.rs</FILE> - <DESC>Collect ordered migration gap family names</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
