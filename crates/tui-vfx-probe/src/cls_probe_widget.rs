// <FILE>crates/tui-vfx-probe/src/cls_probe_widget.rs</FILE> - <DESC>Widget placement DTO for probe reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add widget placement metadata DTO with absolute origin and size</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_report::{ProbePoint, ProbeSize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeWidget {
    pub abs_origin: ProbePoint,
    pub size: ProbeSize,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_widget.rs</FILE> - <DESC>Widget placement DTO for probe reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
