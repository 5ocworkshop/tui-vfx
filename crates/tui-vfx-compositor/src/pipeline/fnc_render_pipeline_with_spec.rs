// <FILE>tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs</FILE>
// <DESC>Render pipeline wrapper for CompositionSpec</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Sub-plan A Phase A.2.2 — role-aware signature: accept `&RoleMap` and `&mut SemanticScene` matching the role-aware contract of render_pipeline</WCTX>
// <CLOG>0.4.0: MAJOR — new signature carries `source_roles: &RoleMap` after `source`, and destination becomes `&mut SemanticScene` (was `&mut dyn Grid`). Callers without role information reach for `RoleMap::all_background(w, h)` and `SemanticScene::from_grid_with_default_role(grid, RoleTag::Background)`.</CLOG>

use crate::pipeline::cls_composition_options::{CompositionOptions, ShaderWithRegion};
use crate::pipeline::cls_composition_spec::CompositionSpec;
use crate::pipeline::orc_render_pipeline::render_pipeline;
use crate::traits::pipeline_inspector::CompositorInspector;
use smallvec::SmallVec;
use std::borrow::Cow;
use tui_vfx_style::models::SpatialShaderType;
use tui_vfx_style::traits::StyleShader;
use tui_vfx_types::{Grid, RoleMap, SemanticScene};

/// Render pipeline wrapper that accepts a serializable CompositionSpec.
#[allow(clippy::too_many_arguments)]
pub fn render_pipeline_with_spec(
    source: &dyn Grid,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    let mut shader_storage: SmallVec<[SpatialShaderType; 2]> = SmallVec::new();
    let mut shader_layers: SmallVec<[ShaderWithRegion; 2]> = SmallVec::new();

    for layer in &spec.shader_layers {
        shader_storage.push(layer.shader.clone());
    }

    for (index, layer) in spec.shader_layers.iter().enumerate() {
        let shader_ref: &dyn StyleShader = &shader_storage[index];
        shader_layers.push(ShaderWithRegion {
            shader: shader_ref,
            region: layer.region.clone(),
            v3_family: Some(layer.v3_shader_family()),
            shader_label: Some(layer.shader.name().to_string()),
        });
    }

    let options = CompositionOptions {
        sampler_spec: spec.sampler_spec.clone(),
        masks: Cow::Borrowed(spec.masks.as_slice()),
        mask_combine_mode: spec.mask_combine_mode,
        filters: Cow::Borrowed(spec.filters.as_slice()),
        shader_layers,
        shadow: spec.shadow.clone(),
        shadow_element_rect: None,
        preserve_unfilled: spec.preserve_unfilled,
        t: spec.t,
        loop_t: spec.loop_t,
        phase: spec.phase,
        runtime_params: spec.runtime_params.clone().into(),
    };

    render_pipeline(
        source,
        source_roles,
        destination,
        width,
        height,
        offset_x,
        offset_y,
        options,
        inspector,
    );
}

// <FILE>tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs</FILE>
// <DESC>Render pipeline wrapper for CompositionSpec</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
