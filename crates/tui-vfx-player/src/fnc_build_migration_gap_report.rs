// <FILE>crates/tui-vfx-player/src/fnc_build_migration_gap_report.rs</FILE> - <DESC>Build debug recipe migration gap reports</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep report construction OFPF-sized.</WCTX>
// <CLOG>0.2.0: PATCH — split family policy, summary, and queue helpers.</CLOG>

use std::{collections::BTreeMap, path::Path};

use crate::{
    DescriptorPackReport, PlayerMigrationGapFamily, PlayerMigrationGapReport,
    fnc_build_migration_gap_family::build_migration_gap_family,
    fnc_collect_debug_recipe_family_inventory::{
        DebugRecipeFamilyInventory, collect_debug_recipe_family_inventory,
    },
    fnc_collect_migration_gap_family_names::migration_gap_family_names,
    fnc_recommend_migration_queue::recommended_queue,
    fnc_summarize_migration_gap_families::summarize_families,
};

/// Build a report-only migration gap report from legacy and v3.1 debug recipe roots.
pub fn build_migration_gap_report(
    legacy_root: &Path,
    v31_root: &Path,
    descriptor_packs: Vec<DescriptorPackReport>,
) -> Result<PlayerMigrationGapReport, String> {
    let legacy = collect_debug_recipe_family_inventory(legacy_root, false)?;
    let v31 = collect_debug_recipe_family_inventory(v31_root, true)?;
    let families = build_family_reports(&legacy, &v31);
    let summary = summarize_families(&families);
    Ok(PlayerMigrationGapReport {
        schema_version: "v3.1.player.migrationGap.1",
        legacy_root: legacy_root.display().to_string(),
        v31_root: v31_root.display().to_string(),
        descriptor_packs,
        summary,
        families,
        recommended_queue: recommended_queue(),
    })
}

fn build_family_reports(
    legacy: &BTreeMap<String, DebugRecipeFamilyInventory>,
    v31: &BTreeMap<String, DebugRecipeFamilyInventory>,
) -> Vec<PlayerMigrationGapFamily> {
    migration_gap_family_names(legacy, v31)
        .into_iter()
        .map(|family| build_migration_gap_family(&family, legacy.get(&family), v31.get(&family)))
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_migration_gap_report.rs</FILE> - <DESC>Build debug recipe migration gap reports</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
