// <FILE>crates/tui-vfx-probe/src/cls_probe_trace_event.rs</FILE> - <DESC>Per-cell causation trace DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase-1.5 probe causation support</WCTX>
// <CLOG>MINOR: Expand trace events to carry sampler source coordinates, mask visibility, and optional before/after snapshots for shader and filter stages</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_report::ProbePoint;
use crate::cls_probe_state_snapshot::ProbeStateSnapshot;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeTraceEvent {
    pub stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_from: Option<ProbePoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<ProbeStateSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<ProbeStateSnapshot>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_trace_event.rs</FILE> - <DESC>Per-cell causation trace DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
