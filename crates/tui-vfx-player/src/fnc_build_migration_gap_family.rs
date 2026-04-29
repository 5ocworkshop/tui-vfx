// <FILE>crates/tui-vfx-player/src/fnc_build_migration_gap_family.rs</FILE> - <DESC>Build one migration gap family report row</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate per-family status policy.</WCTX>
// <CLOG>0.1.0: INIT — split coverage, status, blockers, and candidate selection.</CLOG>

use crate::{
    PlayerMigrationGapFamily, fnc_collect_debug_recipe_family_inventory::DebugRecipeFamilyInventory,
};

/// Build one family row from optional legacy and v3.1 inventories.
pub(crate) fn build_migration_gap_family(
    family: &str,
    legacy: Option<&DebugRecipeFamilyInventory>,
    v31: Option<&DebugRecipeFamilyInventory>,
) -> PlayerMigrationGapFamily {
    let legacy_count = legacy.map_or(0, DebugRecipeFamilyInventory::count);
    let v31_count = v31.map_or(0, DebugRecipeFamilyInventory::count);
    let coverage = coverage_for(family, legacy_count, v31_count);
    let status = status_for(family, &coverage);
    PlayerMigrationGapFamily {
        family: family.to_string(),
        legacy_count,
        v31_count,
        known_v31_effect_ids: v31_effect_ids(v31),
        blockers: blockers_for(family, &coverage, &status),
        recommended_next_candidates: next_candidates(legacy),
        coverage,
        status,
    }
}

fn coverage_for(family: &str, legacy_count: usize, v31_count: usize) -> String {
    if legacy_count == 0 && v31_count == 0 {
        return "notApplicable".to_string();
    }
    if v31_count == 0 {
        return "none".to_string();
    }
    if family == "baseline" || v31_count >= legacy_count {
        return "represented".to_string();
    }
    "partial".to_string()
}

fn status_for(family: &str, coverage: &str) -> String {
    match (family, coverage) {
        (_, "notApplicable") => "notYetClassified".to_string(),
        ("baseline", _) => "migrationCandidateReady".to_string(),
        (
            "filters"
            | "masks"
            | "samplers"
            | "shaders/primitives"
            | "shaders/compositions"
            | "styles",
            _,
        ) => "adapterExpansionReady".to_string(),
        ("content" | "scene", "none") => "migrationCandidateReady".to_string(),
        ("shadows" | "subcell_shapes", "none") => "descriptorDecisionNeeded".to_string(),
        ("signals" | "easings" | "motion_routes" | "loopback", "none") => {
            "schemaDecisionNeeded".to_string()
        }
        ("complex", "none") => "adapterExpansionReady".to_string(),
        ("event_driven_dwell", _) => "ownerAuditNeeded".to_string(),
        ("bindable_rates" | "fixtures" | "other", _) => "ownerAuditNeeded".to_string(),
        _ => "notYetClassified".to_string(),
    }
}

fn blockers_for(family: &str, coverage: &str, status: &str) -> Vec<String> {
    let mut blockers = no_coverage_blocker(coverage);
    match status {
        "schemaDecisionNeeded" => {
            blockers.push("schema/lifecycle semantics need owner decision".to_string())
        }
        "descriptorDecisionNeeded" => {
            blockers.push("descriptor shape needs owner decision".to_string())
        }
        "adapterExpansionReady" if family == "complex" => blockers
            .push("requires primitive adapters before useful complex parity demo".to_string()),
        "ownerAuditNeeded" => {
            blockers.push("owner audit needed before migration batching".to_string())
        }
        _ => {}
    }
    blockers
}

fn no_coverage_blocker(coverage: &str) -> Vec<String> {
    if coverage == "none" {
        vec!["no v3.1 representative fixture".to_string()]
    } else {
        Vec::new()
    }
}

fn next_candidates(legacy: Option<&DebugRecipeFamilyInventory>) -> Vec<String> {
    legacy
        .into_iter()
        .flat_map(|inventory| inventory.recipe_paths.iter())
        .filter(|path| !path.contains("_DEPRECATED"))
        .take(5)
        .cloned()
        .collect()
}

fn v31_effect_ids(v31: Option<&DebugRecipeFamilyInventory>) -> Vec<String> {
    v31.into_iter()
        .flat_map(|inventory| inventory.effect_ids.iter().cloned())
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_migration_gap_family.rs</FILE> - <DESC>Build one migration gap family report row</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
