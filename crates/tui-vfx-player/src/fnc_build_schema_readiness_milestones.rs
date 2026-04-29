// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_milestones.rs</FILE> - <DESC>Build schema-readiness milestones</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.11 schema readiness: expose declaration gates from the readiness summary.</WCTX>
// <CLOG>0.1.0: INIT — move milestone construction out of report orchestration.</CLOG>

use crate::{PlayerSchemaReadinessMilestone, PlayerSchemaReadinessSummary};

pub(crate) fn build_schema_readiness_milestones(
    summary: &PlayerSchemaReadinessSummary,
) -> Vec<PlayerSchemaReadinessMilestone> {
    vec![
        milestone(
            "schema-model-decisions",
            "Resolve or explicitly defer schema/model blockers",
            summary.schema_blocked_records == 0,
        ),
        milestone(
            "source-descriptor-decisions",
            "Resolve source/content descriptor blockers",
            summary.source_blocked_records == 0,
        ),
        milestone(
            "field-coverage-decisions",
            "Resolve field-coverage blockers",
            summary.field_coverage_blocked_records == 0,
        ),
        milestone(
            "owner-audit-normalized",
            "Classify owner-audit records into explicit blocker buckets",
            summary.owner_audit_records == 0 && summary.unknown_records == 0,
        ),
    ]
}

fn milestone(id: &str, description: &str, complete: bool) -> PlayerSchemaReadinessMilestone {
    PlayerSchemaReadinessMilestone {
        id: id.to_string(),
        description: description.to_string(),
        complete,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_milestones.rs</FILE> - <DESC>Build schema-readiness milestones</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
