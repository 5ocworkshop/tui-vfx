// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_blocker_notes.rs</FILE> - <DESC>Collect schema-readiness blocker notes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.11 schema readiness: keep grouped blocker evidence notes compact and stable.</WCTX>
// <CLOG>0.1.0: INIT — isolate note collection from blocker grouping.</CLOG>

use crate::PlayerMigrationMappingRecord;

pub(crate) fn schema_readiness_blocker_notes(
    records: &[&PlayerMigrationMappingRecord],
) -> Vec<String> {
    let mut notes = Vec::new();
    for record in records {
        notes.extend(record.notes.clone());
        append_note(
            &mut notes,
            "required descriptors",
            &record.required_descriptor_ids,
        );
        append_note(
            &mut notes,
            "missing descriptors",
            &record.missing_descriptor_ids,
        );
        append_note(&mut notes, "required sources", &record.required_source_ids);
        append_note(&mut notes, "missing sources", &record.missing_source_ids);
        append_note(
            &mut notes,
            "unsupported fields",
            &record.unsupported_input_fields,
        );
        append_note(&mut notes, "candidate blockers", &record.candidate_blockers);
    }
    notes.sort();
    notes.dedup();
    notes
}

fn append_note(notes: &mut Vec<String>, label: &str, values: &[String]) {
    if !values.is_empty() {
        notes.push(format!("{label}: {}", values.join(", ")));
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_blocker_notes.rs</FILE> - <DESC>Collect schema-readiness blocker notes</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
