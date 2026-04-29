// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_families.rs</FILE> - <DESC>Build schema-readiness family counts</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.11 schema readiness: group blocker kinds by legacy recipe family.</WCTX>
// <CLOG>0.1.0: INIT — move family aggregation out of report orchestration.</CLOG>

use std::collections::BTreeMap;

use crate::{
    PlayerMigrationMappingRecord, PlayerSchemaReadinessFamily, schema_readiness_blocker_kind,
};

pub(crate) fn build_schema_readiness_families(
    records: &[PlayerMigrationMappingRecord],
) -> Vec<PlayerSchemaReadinessFamily> {
    let mut families: BTreeMap<String, PlayerSchemaReadinessFamily> = BTreeMap::new();
    for record in records {
        let family = families
            .entry(record.legacy_family.clone())
            .or_insert_with(|| PlayerSchemaReadinessFamily {
                family: record.legacy_family.clone(),
                record_count: 0,
                blocker_counts: BTreeMap::new(),
            });
        family.record_count += 1;
        if record.status != "canonicalExists" && record.status != "candidateReady" {
            let kind = schema_readiness_blocker_kind(record).to_string();
            *family.blocker_counts.entry(kind).or_default() += 1;
        }
    }
    families.into_values().collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_families.rs</FILE> - <DESC>Build schema-readiness family counts</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
