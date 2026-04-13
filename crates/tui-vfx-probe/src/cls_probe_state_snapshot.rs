// <FILE>crates/tui-vfx-probe/src/cls_probe_state_snapshot.rs</FILE> - <DESC>Structured before/after state snapshots for probe traces and diffs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1.5 probe causation and diff support</WCTX>
// <CLOG>NEW: Add a reusable state snapshot DTO for filter/shader trace events and frame diffs</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::{Cell, Style};

use crate::cls_probe_color::ProbeColor;
use crate::fnc_modifier_names::modifier_names;
use crate::fnc_normalize_color::normalize_color;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeStateSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch: Option<char>,
    pub fg: ProbeColor,
    pub bg: ProbeColor,
    pub modifiers: Vec<String>,
}

impl ProbeStateSnapshot {
    pub fn from_cell(cell: &Cell) -> Self {
        Self {
            ch: Some(cell.ch),
            fg: normalize_color(cell.fg),
            bg: normalize_color(cell.bg),
            modifiers: modifier_names(cell.mods),
        }
    }

    pub fn from_style(style: Style) -> Self {
        Self {
            ch: None,
            fg: normalize_color(style.fg),
            bg: normalize_color(style.bg),
            modifiers: modifier_names(style.mods),
        }
    }
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_state_snapshot.rs</FILE> - <DESC>Structured before/after state snapshots for probe traces and diffs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
