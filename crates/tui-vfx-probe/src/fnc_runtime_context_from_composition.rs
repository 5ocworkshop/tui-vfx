// <FILE>crates/tui-vfx-probe/src/fnc_runtime_context_from_composition.rs</FILE> - <DESC>Build runtime binding observability from a composition spec</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime-binding observability for probe debugging</WCTX>
// <CLOG>NEW: Convert composition runtime params and shader binding declarations/resolutions into a structured probe runtime context</CLOG>

use tui_vfx_compositor::pipeline::CompositionSpec;
use tui_vfx_style::traits::ShaderRuntimeParamValue;

use crate::{ProbeRuntimeContext, ProbeRuntimeParam};

pub fn runtime_context_from_composition(
    composition: &CompositionSpec,
) -> Option<ProbeRuntimeContext> {
    let supplied_params = composition
        .runtime_params
        .0
        .iter()
        .map(|(key, value)| ProbeRuntimeParam {
            key: key.clone(),
            kind: value.kind_name().to_string(),
            value: serde_json::Value::from(value),
        })
        .collect::<Vec<_>>();

    let binding_requests = composition
        .shader_layers
        .iter()
        .flat_map(|layer| layer.shader.runtime_binding_requests())
        .collect::<Vec<_>>();

    let binding_resolutions = composition
        .shader_layers
        .iter()
        .flat_map(|layer| {
            let ctx = tui_vfx_style::traits::ShaderContext::new(
                0,
                0,
                0,
                0,
                0,
                0,
                composition.loop_t.unwrap_or(composition.t),
                composition.phase,
                Some(composition.runtime_params.clone().into()),
            );
            layer.shader.runtime_binding_resolutions(&ctx)
        })
        .collect::<Vec<_>>();

    if supplied_params.is_empty() && binding_requests.is_empty() && binding_resolutions.is_empty() {
        None
    } else {
        Some(ProbeRuntimeContext {
            supplied_params,
            binding_requests,
            binding_resolutions,
        })
    }
}

pub fn runtime_value_json(value: &ShaderRuntimeParamValue) -> serde_json::Value {
    serde_json::Value::from(value)
}

// <FILE>crates/tui-vfx-probe/src/fnc_runtime_context_from_composition.rs</FILE> - <DESC>Build runtime binding observability from a composition spec</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
