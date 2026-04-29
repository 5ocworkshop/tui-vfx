// <FILE>crates/tui-vfx-player/src/fnc_build_legacy_migration_mapping_summary.rs</FILE> - <DESC>Build legacy migration mapping evidence summaries</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.10 corpus mapping: keep compact evidence text construction focused.</WCTX>
// <CLOG>0.1.0: INIT — add compact evidence summary formatter.</CLOG>

/// Build compact human-readable evidence text for one legacy mapping record.
pub(crate) fn build_legacy_migration_mapping_summary(
    descriptors: &[String],
    sources: &[String],
    families: &[String],
) -> String {
    let descriptor_summary = joined_or_none(descriptors);
    let source_summary = joined_or_none(sources);
    let family_summary = joined_or_none(families);
    format!(
        "legacy effect families: {family_summary}; descriptors: {descriptor_summary}; sources: {source_summary}"
    )
}

fn joined_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_legacy_migration_mapping_summary.rs</FILE> - <DESC>Build legacy migration mapping evidence summaries</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
