// <FILE>crates/tui-vfx-player/src/fnc_summarize_migration_gap_families.rs</FILE> - <DESC>Summarize migration gap family rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate migration gap summary counting.</WCTX>
// <CLOG>0.1.0: INIT — split family summary counts from report construction.</CLOG>

use crate::{PlayerMigrationGapFamily, PlayerMigrationGapSummary};

/// Summarize migration gap family rows into aggregate counts.
pub(crate) fn summarize_families(
    families: &[PlayerMigrationGapFamily],
) -> PlayerMigrationGapSummary {
    PlayerMigrationGapSummary {
        legacy_recipes: families.iter().map(|family| family.legacy_count).sum(),
        v31_recipes: families.iter().map(|family| family.v31_count).sum(),
        represented_families: families
            .iter()
            .filter(|family| family.v31_count > 0)
            .count(),
        unrepresented_families: families
            .iter()
            .filter(|family| family.legacy_count > 0 && family.v31_count == 0)
            .count(),
        partially_represented_families: families
            .iter()
            .filter(|family| family.coverage == "partial")
            .count(),
        ready_families: families
            .iter()
            .filter(|family| is_ready_status(&family.status))
            .count(),
        blocked_families: families
            .iter()
            .filter(|family| is_blocked_status(&family.status))
            .count(),
    }
}

fn is_ready_status(status: &str) -> bool {
    matches!(status, "adapterExpansionReady" | "migrationCandidateReady")
}

fn is_blocked_status(status: &str) -> bool {
    matches!(
        status,
        "schemaDecisionNeeded" | "descriptorDecisionNeeded" | "ownerAuditNeeded"
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_summarize_migration_gap_families.rs</FILE> - <DESC>Summarize migration gap family rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
