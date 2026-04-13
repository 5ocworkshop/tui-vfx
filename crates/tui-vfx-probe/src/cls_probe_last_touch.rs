// <FILE>crates/tui-vfx-probe/src/cls_probe_last_touch.rs</FILE> - <DESC>Last-touch attribution DTO for probe cells</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add stage/effect attribution DTO for per-cell last-touch reporting</CLOG>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeLastTouch {
    pub stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_last_touch.rs</FILE> - <DESC>Last-touch attribution DTO for probe cells</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
