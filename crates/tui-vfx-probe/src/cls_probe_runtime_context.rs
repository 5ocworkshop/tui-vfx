// <FILE>crates/tui-vfx-probe/src/cls_probe_runtime_context.rs</FILE> - <DESC>Runtime parameter and binding observability DTOs for probe reports</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Loopback Phase L5 (deferred follow-on): surface the per-frame loopback fired_keys list so the new fnc_collect_loopback_fire_diagnostics can emit a Warning per key without re-running the merge.</WCTX>
// <CLOG>Add `loopback_fired_keys: Vec<String>` field with serde-default-empty so existing probe payloads deserialize unchanged. The recipes-side probe-scene builder populates this via the L4 with_loopback_applied tuple return.</CLOG>

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tui_vfx_style::traits::cls_shader_context::{
    ShaderRuntimeBindingRequest, ShaderRuntimeBindingResolution,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeRuntimeContext {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplied_params: Vec<ProbeRuntimeParam>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binding_requests: Vec<ShaderRuntimeBindingRequest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binding_resolutions: Vec<ShaderRuntimeBindingResolution>,
    /// Binding names whose value came from (or would have come from) the
    /// recipe-author-declared loopback during the probed frame's merge.
    ///
    /// Empty when no loopback fired (host supplied every declared
    /// binding, or the recipe declared no `requires_bindings`). Stable
    /// BTreeMap iteration order. Drives the
    /// [`crate::fnc_collect_loopback_fire_diagnostics::collect_loopback_fire_diagnostics`]
    /// emission of one `loopback_fire` Warning per key.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loopback_fired_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeRuntimeParam {
    pub key: String,
    pub kind: String,
    pub value: Value,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_runtime_context.rs</FILE> - <DESC>Runtime parameter and binding observability DTOs for probe reports</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
