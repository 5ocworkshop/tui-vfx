// <FILE>crates/tui-vfx-compositor/tests/pipeline/test_helpers.rs</FILE> - <DESC>Test-only helpers that wrap the role-aware render_pipeline* entrypoints with the legacy `&mut OwnedGrid` call shape, so existing test bodies migrate by renaming the function call rather than restructuring</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.2.3 — in-tree test consumer migration. Rather than rewrite every one of the ~35 call-sites individually, expose a thin `*_legacy` shim that constructs the role-aware arguments (RoleMap::all_background + SemanticScene::from_grid_with_default_role, copying the destination grid back out).</WCTX>
// <CLOG>0.1.0: initial shims for render_pipeline_legacy, render_pipeline_with_spec_legacy, render_pipeline_with_spec_area_legacy. They defer to the real entrypoints — there is no separate code path under test — so the `_legacy` suffix purely describes the call shape, not the implementation.</CLOG>

#![allow(dead_code)]

use tui_vfx_compositor::pipeline::{
    render_pipeline, render_pipeline_with_spec, render_pipeline_with_spec_area, CompositionOptions,
    CompositionSpec, RenderArea,
};
use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_types::{Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

/// Legacy-shape wrapper around `render_pipeline` for tests that still speak
/// `&mut OwnedGrid` on the destination. Role info defaults to
/// `RoleMap::all_background` matching the source's dimensions (the A.2
/// no-info migration default).
#[allow(clippy::too_many_arguments)]
pub fn render_pipeline_legacy(
    source: &OwnedGrid,
    dest: &mut OwnedGrid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    let source_roles = RoleMap::all_background(source.width() as u16, source.height() as u16);
    let taken = std::mem::replace(dest, OwnedGrid::new(0, 0));
    let mut scene = SemanticScene::from_grid_with_default_role(taken, RoleTag::Background);
    render_pipeline(
        source,
        &source_roles,
        &mut scene,
        width,
        height,
        offset_x,
        offset_y,
        options,
        inspector,
    );
    *dest = scene.grid().clone();
}

/// Legacy-shape wrapper around `render_pipeline_with_spec`.
#[allow(clippy::too_many_arguments)]
pub fn render_pipeline_with_spec_legacy(
    source: &OwnedGrid,
    dest: &mut OwnedGrid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    let source_roles = RoleMap::all_background(source.width() as u16, source.height() as u16);
    let taken = std::mem::replace(dest, OwnedGrid::new(0, 0));
    let mut scene = SemanticScene::from_grid_with_default_role(taken, RoleTag::Background);
    render_pipeline_with_spec(
        source,
        &source_roles,
        &mut scene,
        width,
        height,
        offset_x,
        offset_y,
        spec,
        inspector,
    );
    *dest = scene.grid().clone();
}

/// Legacy-shape wrapper around `render_pipeline_with_spec_area`.
pub fn render_pipeline_with_spec_area_legacy(
    source: &OwnedGrid,
    dest: &mut OwnedGrid,
    area: RenderArea,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    let source_roles = RoleMap::all_background(source.width() as u16, source.height() as u16);
    let taken = std::mem::replace(dest, OwnedGrid::new(0, 0));
    let mut scene = SemanticScene::from_grid_with_default_role(taken, RoleTag::Background);
    render_pipeline_with_spec_area(source, &source_roles, &mut scene, area, spec, inspector);
    *dest = scene.grid().clone();
}

// <FILE>crates/tui-vfx-compositor/tests/pipeline/test_helpers.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
