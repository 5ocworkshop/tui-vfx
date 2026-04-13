// <FILE>crates/tui-vfx-probe/src/cls_probe_color.rs</FILE> - <DESC>Normalized probe color DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add structured normalized color output for probe cell reports</CLOG>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeColor {
    pub space: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_color.rs</FILE> - <DESC>Normalized probe color DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
