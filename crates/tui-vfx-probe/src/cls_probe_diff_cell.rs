// <FILE>crates/tui-vfx-probe/src/cls_probe_diff_cell.rs</FILE> - <DESC>Changed-cell DTO for probe frame diffs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1.5 probe diff support</WCTX>
// <CLOG>NEW: Add a changed-cell DTO that carries before/after state plus optional causation data</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_last_touch::ProbeLastTouch;
use crate::cls_probe_report::ProbePoint;
use crate::cls_probe_state_snapshot::ProbeStateSnapshot;
use crate::cls_probe_trace_event::ProbeTraceEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeDiffCell {
    pub abs: ProbePoint,
    pub widget_local: ProbePoint,
    pub before: ProbeStateSnapshot,
    pub after: ProbeStateSnapshot,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_touch: Option<ProbeLastTouch>,
    pub trace: Vec<ProbeTraceEvent>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_diff_cell.rs</FILE> - <DESC>Changed-cell DTO for probe frame diffs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
