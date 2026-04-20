// <FILE>crates/tui-vfx-compositor/tests/test_render_pipeline_signature.rs</FILE> - <DESC>Compile-time signature assertions — this test file compiling is itself evidence that render_pipeline accepts the role-aware parameter tuple (&Grid, &RoleMap, &mut SemanticScene, …)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.2.2 — guard against accidental signature regression. If either parameter is dropped or reordered, this file stops compiling.</WCTX>
// <CLOG>0.1.0: initial compile-assert. Rather than a trybuild dep, we define a `fn _compile_assert_signature` body that names the full parameter tuple so a signature drift breaks the build. One runtime smoke test per entrypoint confirms the call site runs end-to-end.</CLOG>

use std::borrow::Cow;
use tui_vfx_compositor::pipeline::{
    CompositionOptions, CompositionSpec, RenderArea, render_pipeline, render_pipeline_with_area,
    render_pipeline_with_spec, render_pipeline_with_spec_area,
};
use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_types::{Cell, Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

/// Compile-time assertion: `render_pipeline`'s signature MUST include
/// `source_roles: &RoleMap` AND `destination: &mut SemanticScene`. Naming
/// every parameter here means a regression in the public signature breaks
/// the build.
#[allow(dead_code, clippy::too_many_arguments)]
fn _compile_assert_render_pipeline(
    source: &dyn Grid,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
) {
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

/// Compile-time assertion for `render_pipeline_with_area`.
#[allow(dead_code)]
fn _compile_assert_render_pipeline_with_area(
    source: &dyn Grid,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    area: RenderArea,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    render_pipeline_with_area(
        source,
        source_roles,
        destination,
        area,
        options,
        inspector,
    );
}

/// Compile-time assertion for `render_pipeline_with_spec`.
#[allow(dead_code, clippy::too_many_arguments)]
fn _compile_assert_render_pipeline_with_spec(
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
    render_pipeline_with_spec(
        source,
        source_roles,
        destination,
        width,
        height,
        offset_x,
        offset_y,
        spec,
        inspector,
    );
}

/// Compile-time assertion for `render_pipeline_with_spec_area`.
#[allow(dead_code)]
fn _compile_assert_render_pipeline_with_spec_area(
    source: &dyn Grid,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    area: RenderArea,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    render_pipeline_with_spec_area(
        source,
        source_roles,
        destination,
        area,
        spec,
        inspector,
    );
}

// Runtime smoke tests: the signature assertions above are dead code — we
// also want a live call that exercises each entrypoint end-to-end.

fn cell_x() -> Cell {
    Cell {
        ch: 'X',
        ..Default::default()
    }
}

#[test]
fn render_pipeline_smoke_runs_with_role_aware_signature() {
    let mut source = OwnedGrid::new(3, 2);
    for y in 0..2 {
        for x in 0..3 {
            source.set(x, y, cell_x());
        }
    }
    let source_roles = RoleMap::all_background(3, 2);
    let dest = OwnedGrid::new(3, 2);
    let mut scene = SemanticScene::from_grid_with_default_role(dest, RoleTag::Background);
    render_pipeline(
        &source,
        &source_roles,
        &mut scene,
        3,
        2,
        0,
        0,
        CompositionOptions::default(),
        None,
    );
    assert_eq!(scene.cell((0, 0)).unwrap().ch, 'X');
}

#[test]
fn render_pipeline_with_area_smoke() {
    let source = OwnedGrid::new(2, 2);
    let source_roles = RoleMap::all_background(2, 2);
    let dest = OwnedGrid::new(4, 4);
    let mut scene = SemanticScene::from_grid_with_default_role(dest, RoleTag::Background);
    render_pipeline_with_area(
        &source,
        &source_roles,
        &mut scene,
        RenderArea {
            width: 2,
            height: 2,
            offset_x: 1,
            offset_y: 1,
        },
        CompositionOptions::default(),
        None,
    );
}

#[test]
fn render_pipeline_with_spec_smoke() {
    let source = OwnedGrid::new(2, 2);
    let source_roles = RoleMap::all_background(2, 2);
    let dest = OwnedGrid::new(2, 2);
    let mut scene = SemanticScene::from_grid_with_default_role(dest, RoleTag::Background);
    let spec = CompositionSpec::default();
    render_pipeline_with_spec(
        &source,
        &source_roles,
        &mut scene,
        2,
        2,
        0,
        0,
        &spec,
        None,
    );
}

#[test]
fn render_pipeline_with_spec_area_smoke() {
    let source = OwnedGrid::new(2, 2);
    let source_roles = RoleMap::all_background(2, 2);
    let dest = OwnedGrid::new(4, 4);
    let mut scene = SemanticScene::from_grid_with_default_role(dest, RoleTag::Background);
    let spec = CompositionSpec::default();
    render_pipeline_with_spec_area(
        &source,
        &source_roles,
        &mut scene,
        RenderArea {
            width: 2,
            height: 2,
            offset_x: 1,
            offset_y: 1,
        },
        &spec,
        None,
    );
}

// Silence unused-Cow warnings in case the optimizer prunes.
#[allow(dead_code)]
fn _cow_lint_silencer() {
    let _: Cow<'_, [u8]> = Cow::Borrowed(&[]);
}

// <FILE>crates/tui-vfx-compositor/tests/test_render_pipeline_signature.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
