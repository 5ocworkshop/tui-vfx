// <FILE>crates/tui-vfx-player/src/fnc_collect_unsupported_migration_mapping_fields.rs</FILE> - <DESC>Collect unsupported migration mapping fields</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.10 corpus mapping: isolate descriptor field coverage comparison.</WCTX>
// <CLOG>0.1.0: INIT — add unsupported field collector for migration mapping records.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

/// Return required fields absent from all referenced descriptor inputs.
pub(crate) fn collect_unsupported_migration_mapping_fields(
    descriptor_ids: &[String],
    input_fields: &[String],
    descriptor_input_fields: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<String> {
    let supported = descriptor_ids
        .iter()
        .filter_map(|descriptor_id| descriptor_input_fields.get(descriptor_id))
        .flat_map(|fields| fields.iter().cloned())
        .collect::<BTreeSet<_>>();
    input_fields
        .iter()
        .filter(|field| !supported.contains(*field))
        .cloned()
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_unsupported_migration_mapping_fields.rs</FILE> - <DESC>Collect unsupported migration mapping fields</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
