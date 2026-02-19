// <FILE>tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec_area.rs</FILE>
// <DESC>Render pipeline wrapper for CompositionSpec and RenderArea</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Spec-based pipeline API for bindings</WCTX>
// <CLOG>Initial render_pipeline_with_spec_area wrapper</CLOG>

use crate::pipeline::cls_composition_spec::CompositionSpec;
use crate::pipeline::cls_render_area::RenderArea;
use crate::pipeline::fnc_render_pipeline_with_spec::render_pipeline_with_spec;
use crate::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_types::Grid;

/// Convenience wrapper for CompositionSpec using RenderArea.
pub fn render_pipeline_with_spec_area(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    area: RenderArea,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    render_pipeline_with_spec(
        source,
        dest,
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
// <VERS>END OF VERSION: 0.1.0</VERS>
