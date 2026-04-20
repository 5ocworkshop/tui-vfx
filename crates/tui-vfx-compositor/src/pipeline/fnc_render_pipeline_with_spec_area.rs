// <FILE>tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec_area.rs</FILE>
// <DESC>Render pipeline wrapper for CompositionSpec and RenderArea</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Sub-plan A Phase A.2.2 — role-aware signature: accept `&RoleMap` and `&mut SemanticScene` matching the role-aware contract of render_pipeline_with_spec</WCTX>
// <CLOG>0.2.0: MAJOR — new signature carries `source_roles: &RoleMap` after `source`, and destination becomes `&mut SemanticScene` (was `&mut dyn Grid`).</CLOG>

use crate::pipeline::cls_composition_spec::CompositionSpec;
use crate::pipeline::cls_render_area::RenderArea;
use crate::pipeline::fnc_render_pipeline_with_spec::render_pipeline_with_spec;
use crate::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_types::{Grid, RoleMap, SemanticScene};

/// Convenience wrapper for CompositionSpec using RenderArea.
#[allow(clippy::too_many_arguments)]
pub fn render_pipeline_with_spec_area(
    source: &dyn Grid,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    area: RenderArea,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    render_pipeline_with_spec(
        source,
        source_roles,
        destination,
        area.width,
        area.height,
        area.offset_x,
        area.offset_y,
        spec,
        inspector,
    );
}

// <FILE>tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec_area.rs</FILE>
// <DESC>Render pipeline wrapper for CompositionSpec and RenderArea</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
