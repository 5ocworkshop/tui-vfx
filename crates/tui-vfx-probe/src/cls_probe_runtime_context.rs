// <FILE>crates/tui-vfx-probe/src/cls_probe_runtime_context.rs</FILE> - <DESC>Runtime parameter and binding observability DTOs for probe reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime-binding observability for probe debugging</WCTX>
// <CLOG>NEW: Add typed DTOs describing supplied runtime params plus shader binding requests/resolutions so probe output can explain dynamic parameter behavior explicitly</CLOG>

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeRuntimeParam {
    pub key: String,
    pub kind: String,
    pub value: Value,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_runtime_context.rs</FILE> - <DESC>Runtime parameter and binding observability DTOs for probe reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
