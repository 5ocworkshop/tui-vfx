// <FILE>tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs</FILE>
// <DESC>Render pipeline wrapper for CompositionSpec</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Spec-driven pipeline parity for shadow and preserve_unfilled</WCTX>
// <CLOG>Thread shadow and preserve_unfilled from CompositionSpec</CLOG>

use crate::pipeline::cls_composition_options::{CompositionOptions, ShaderWithRegion};
use crate::pipeline::cls_composition_spec::CompositionSpec;
use crate::pipeline::orc_render_pipeline::render_pipeline;
use crate::traits::pipeline_inspector::CompositorInspector;
use smallvec::SmallVec;
use std::borrow::Cow;
use tui_vfx_style::models::SpatialShaderType;
use tui_vfx_style::traits::StyleShader;
use tui_vfx_types::Grid;

/// Render pipeline wrapper that accepts a serializable CompositionSpec.
#[allow(clippy::too_many_arguments)]
pub fn render_pipeline_with_spec(
    source: &dyn Grid,
    dest: &mut dyn Grid,
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
        });
    }

    let options = CompositionOptions {
        sampler_spec: spec.sampler_spec.clone(),
        masks: Cow::Borrowed(spec.masks.as_slice()),
        mask_combine_mode: spec.mask_combine_mode,
        filters: Cow::Borrowed(spec.filters.as_slice()),
        shader_layers,
        shadow: spec.shadow.clone(),
        preserve_unfilled: spec.preserve_unfilled,
        t: spec.t,
        loop_t: spec.loop_t,
        phase: spec.phase,
    };

    render_pipeline(
        source, dest, width, height, offset_x, offset_y, options, inspector,
    );
}

// <FILE>tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs</FILE>
// <DESC>Render pipeline wrapper for CompositionSpec</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
