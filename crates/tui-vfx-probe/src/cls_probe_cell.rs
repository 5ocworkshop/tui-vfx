// <FILE>crates/tui-vfx-probe/src/cls_probe_cell.rs</FILE> - <DESC>Per-cell probe report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add the structured per-cell report DTO used by frame dumps</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_cell_root_cause::ProbeCellRootCause;
use crate::cls_probe_color::ProbeColor;
use crate::cls_probe_last_touch::ProbeLastTouch;
use crate::cls_probe_report::ProbePoint;
use crate::cls_probe_trace_event::ProbeTraceEvent;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeCell {
    pub abs: ProbePoint,
    pub widget_local: ProbePoint,
    pub ch: char,
    pub fg: ProbeColor,
    pub bg: ProbeColor,
    pub modifiers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_touch: Option<ProbeLastTouch>,
    pub trace: Vec<ProbeTraceEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause: Option<ProbeCellRootCause>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_cell.rs</FILE> - <DESC>Per-cell probe report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
