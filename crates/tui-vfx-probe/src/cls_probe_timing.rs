// <FILE>crates/tui-vfx-probe/src/cls_probe_timing.rs</FILE> - <DESC>Timing metadata DTO for probe reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add requested/effective timing fields for explicit probe timing metadata</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_request::ProbePhase;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeTiming {
    pub requested_phase: ProbePhase,
    pub requested_t: f64,
    pub effective_phase: ProbePhase,
    pub effective_t: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_ms: Option<u64>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_timing.rs</FILE> - <DESC>Timing metadata DTO for probe reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
