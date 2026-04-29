// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_reports.rs</FILE> - <DESC>Build fixture QC embedded reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: serialize composed fixture QC report payloads.</WCTX>
// <CLOG>0.1.0: INIT — extract embedded report serialization helper.</CLOG>

use crate::PlayerFixtureQcReports;

/// Build embedded report payloads for fixture QC output.
pub(crate) fn build_fixture_qc_reports(
    render: crate::PlayerRunReport,
    visual_frame: crate::PlayerVisualFrameReport,
    field_coverage: crate::PlayerPrimitiveFieldCoverageReport,
    adapter_gap: crate::PlayerPrimitiveAdapterGapReport,
    timeline: Option<crate::PlayerFrameTimelineReport>,
    diff: Option<crate::PlayerFrameDiffReport>,
) -> Result<PlayerFixtureQcReports, String> {
    Ok(PlayerFixtureQcReports {
        render: to_value(render)?,
        visual_frame: to_value(visual_frame)?,
        field_coverage: to_value(field_coverage)?,
        adapter_gap: to_value(adapter_gap)?,
        timeline: timeline.map(to_value).transpose()?,
        diff: diff.map(to_value).transpose()?,
    })
}

fn to_value(value: impl serde::Serialize) -> Result<serde_json::Value, String> {
    serde_json::to_value(value).map_err(|error| error.to_string())
}

// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_reports.rs</FILE> - <DESC>Build fixture QC embedded reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
