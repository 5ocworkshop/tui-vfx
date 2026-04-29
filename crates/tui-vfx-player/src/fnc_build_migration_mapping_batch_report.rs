// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_batch_report.rs</FILE> - <DESC>Build migration mapping batch reports</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.9 migration mapping: classify legacy recipe records conservatively.</WCTX>
// <CLOG>0.2.0: REFACTOR — delegate path collection, record construction, and summary aggregation.
// 0.1.0: INIT — add read-only migration mapping report builder.</CLOG>

use std::{collections::BTreeSet, path::Path};

use tui_vfx_contract::DescriptorCatalog;

use crate::{
    DescriptorPackReport, PlayerMigrationMappingBatchReport,
    fnc_build_migration_mapping_record::build_migration_mapping_record,
    fnc_collect_migration_mapping_batch_paths::collect_migration_mapping_batch_paths,
    fnc_summarize_migration_mapping_batch::{
        build_migration_mapping_recommendation_queue, migration_mapping_record_families,
        summarize_migration_mapping_records,
    },
};

/// Build a read-only migration mapping batch from legacy and v3.1 roots.
pub fn build_migration_mapping_batch_report(
    legacy_root: &Path,
    v31_root: &Path,
    descriptor_packs: Vec<DescriptorPackReport>,
    catalog: &DescriptorCatalog,
    family: Option<&str>,
    recursive: bool,
) -> Result<PlayerMigrationMappingBatchReport, String> {
    let paths = collect_migration_mapping_batch_paths(legacy_root, family, recursive)?;
    let descriptor_ids = catalog_effect_ids(catalog);
    let source_ids = catalog_source_ids(catalog);
    let mut records = paths
        .iter()
        .map(|path| {
            build_migration_mapping_record(
                legacy_root,
                v31_root,
                path,
                &descriptor_ids,
                &source_ids,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    records.sort_by(|left, right| left.legacy_path.cmp(&right.legacy_path));
    let families = migration_mapping_record_families(&records);
    let summary = summarize_migration_mapping_records(&families, &records);
    let recommendation_queue = build_migration_mapping_recommendation_queue(&records);
    Ok(PlayerMigrationMappingBatchReport {
        schema_version: "v3.1.player.migrationMappingBatch.1",
        legacy_root: legacy_root.display().to_string(),
        v31_root: v31_root.display().to_string(),
        descriptor_packs,
        families,
        summary,
        records,
        recommendation_queue,
        warnings: Vec::new(),
        errors: Vec::new(),
    })
}

fn catalog_effect_ids(catalog: &DescriptorCatalog) -> BTreeSet<String> {
    catalog
        .packs
        .values()
        .flat_map(|pack| pack.effects.keys())
        .map(|id| id.as_str().to_string())
        .collect()
}

fn catalog_source_ids(catalog: &DescriptorCatalog) -> BTreeSet<String> {
    catalog
        .packs
        .values()
        .flat_map(|pack| pack.source_descriptors.keys())
        .map(|id| id.as_str().to_string())
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_batch_report.rs</FILE> - <DESC>Build migration mapping batch reports</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
