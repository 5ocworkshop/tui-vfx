// <FILE>crates/tui-vfx-probe/src/cls_probe_scene_spec.rs</FILE> - <DESC>Serializable probe scene wrapper for direct engine runs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add ProbeSceneSpec combining source grid, destination frame, widget offset, and CompositionSpec</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_compositor::pipeline::CompositionSpec;

use crate::cls_probe_grid_spec::ProbeGridSpec;
use crate::cls_probe_report::ProbePoint;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeSceneSpec {
    pub source: ProbeGridSpec,
    pub destination: ProbeGridSpec,
    pub widget_offset: ProbePoint,
    pub composition: CompositionSpec,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_scene_spec.rs</FILE> - <DESC>Serializable probe scene wrapper for direct engine runs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
