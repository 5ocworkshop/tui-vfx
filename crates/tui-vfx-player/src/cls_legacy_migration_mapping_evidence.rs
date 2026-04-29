// <FILE>crates/tui-vfx-player/src/cls_legacy_migration_mapping_evidence.rs</FILE> - <DESC>Legacy migration mapping evidence DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.10 corpus mapping: carry conservative legacy recipe evidence into report records.</WCTX>
// <CLOG>0.2.0: MINOR — carry nested value-source decision fields for conservative classification.</CLOG>

/// Conservative evidence extracted from one legacy recipe for migration planning.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LegacyMigrationMappingEvidence {
    /// Effect descriptors required by a faithful canonical fixture.
    pub required_descriptor_ids: Vec<String>,
    /// Source descriptors expected by a faithful canonical fixture.
    pub required_source_ids: Vec<String>,
    /// Authored legacy fields that need descriptor/player coverage.
    pub required_input_fields: Vec<String>,
    /// Authored legacy fields whose nested values need value-source or signal review.
    pub value_source_decision_fields: Vec<String>,
    /// Legacy signal or signal-like keys observed in the recipe.
    pub legacy_signals: Vec<String>,
    /// Legacy binding keys observed in the recipe.
    pub legacy_bindings: Vec<String>,
    /// Legacy source-kind candidates observed in the recipe.
    pub legacy_source_kinds: Vec<String>,
    /// Effect family kinds observed in the recipe.
    pub legacy_effect_families: Vec<String>,
    /// Human-readable compact evidence summary.
    pub summary: String,
}

// <FILE>crates/tui-vfx-player/src/cls_legacy_migration_mapping_evidence.rs</FILE> - <DESC>Legacy migration mapping evidence DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
