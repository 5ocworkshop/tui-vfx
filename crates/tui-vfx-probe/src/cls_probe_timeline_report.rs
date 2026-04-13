// <FILE>crates/tui-vfx-probe/src/cls_probe_timeline_report.rs</FILE> - <DESC>Top-level timeline report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1.5 probe timeline support</WCTX>
// <CLOG>NEW: Add a timeline report that wraps multiple frame dumps sampled across one phase</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_report::{ProbeReport, ProbeReportSource};
use crate::cls_probe_request::ProbePhase;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeTimelineReport {
    pub schema_version: String,
    pub kind: String,
    pub source: ProbeReportSource,
    pub phase: ProbePhase,
    pub frame_count: usize,
    pub frames: Vec<ProbeReport>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_timeline_report.rs</FILE> - <DESC>Top-level timeline report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
