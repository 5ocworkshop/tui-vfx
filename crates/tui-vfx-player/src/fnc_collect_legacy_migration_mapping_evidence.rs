// <FILE>crates/tui-vfx-player/src/fnc_collect_legacy_migration_mapping_evidence.rs</FILE> - <DESC>Collect legacy migration mapping evidence</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.10 corpus mapping: keep the public evidence collection entry point focused.</WCTX>
// <CLOG>0.2.0: REFACTOR — delegate traversal state to the evidence collector.</CLOG>

use serde_json::Value;

use crate::{LegacyMigrationMappingEvidence, LegacyMigrationMappingEvidenceCollector};

/// Collect conservative migration evidence from one legacy recipe JSON value.
pub(crate) fn collect_legacy_migration_mapping_evidence(
    family: &str,
    value: &Value,
) -> LegacyMigrationMappingEvidence {
    let mut collector = LegacyMigrationMappingEvidenceCollector::default();
    collector.visit(value);
    collector.add_family_source_candidates(family);
    collector.finish()
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_legacy_migration_mapping_evidence.rs</FILE> - <DESC>Collect legacy migration mapping evidence</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
