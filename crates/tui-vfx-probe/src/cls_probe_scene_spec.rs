// <FILE>crates/tui-vfx-probe/src/cls_probe_scene_spec.rs</FILE> - <DESC>Serializable probe scene wrapper for direct engine runs</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Loopback Phase L5 (deferred follow-on): carry the recipes-side loopback merge's fired_keys into the probe scene so orc_run_probe can splice them onto ProbeRuntimeContext.loopback_fired_keys and collect_loopback_fire_diagnostics emits one Warning per key.</WCTX>
// <CLOG>Add `loopback_fired_keys: Vec<String>` field with serde-default-empty so existing scene specs deserialize unchanged. Recipes-side build_probe_scene_spec_from_compiled_plan_timed_with_overrides populates this; orc_run_probe spreads it onto the runtime context.</CLOG>

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
    /// Binding names whose value came from (or would have come from)
    /// the recipe-author-declared loopback during this scene's frame.
    ///
    /// Empty for scenes built without recipes-side loopback merge
    /// (direct-from-CompositionSpec scenes, JSON-loaded fixtures, etc.).
    /// Recipes-side scene builders populate this from the L4
    /// `with_loopback_applied` tuple return so the probe can emit
    /// `loopback_fire` Warning diagnostics for each fired key.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loopback_fired_keys: Vec<String>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_scene_spec.rs</FILE> - <DESC>Serializable probe scene wrapper for direct engine runs</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
