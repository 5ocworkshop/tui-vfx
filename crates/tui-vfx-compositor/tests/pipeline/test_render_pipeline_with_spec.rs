// <FILE>tui-vfx-compositor/tests/pipeline/test_render_pipeline_with_spec.rs</FILE>
// <DESC>Spec-based pipeline wrapper tests</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Sub-plan A Phase A.2.3 — route every call-site through the `_legacy` shim wrapper so these tests keep speaking the old `&mut OwnedGrid` call shape</WCTX>
// <CLOG>0.6.0: add grouped-V3 construction coverage so render_pipeline_with_spec matches grouped-family runtime callers as well as legacy layer literals.
// 0.5.0: MINOR — switch to `_legacy` shim helpers; no behavior change.
// 0.4.0: Track the new runtime_params field in CompositionSpec fixtures so spec-equivalence coverage keeps compiling as the runtime shader context surface evolves</CLOG>

#[path = "test_helpers.rs"]
mod test_helpers;
use test_helpers::{render_pipeline_legacy, render_pipeline_with_spec_legacy, render_pipeline_with_spec_area_legacy};

use tui_vfx_compositor::pipeline::{
    CompositionOptions, CompositionSpec, RenderArea, ShaderLayerSpec,
};
use tui_vfx_compositor::types::{MaskCombineMode, ShadowSpec};
use tui_vfx_shadow::{ShadowConfig, ShadowEdges};
use tui_vfx_style::models::{
    BorderSweepShader, ColorConfig, SpatialShaderType, StyleRegion,
    VfxSpatialComposedPrimitive, VfxSpatialShaderFamily,
};
use tui_vfx_style::traits::cls_shader_context::ShaderRuntimeParams;
use tui_vfx_types::{Cell, Color, Grid, OwnedGrid};

fn create_source_grid(width: usize, height: usize, fill_char: char) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let cell = Cell {
                ch: fill_char,
                fg: Color::WHITE,
                bg: Color::BLACK,
                ..Default::default()
            };
            grid.set(x, y, cell);
        }
    }
    grid
}

fn assert_grids_equal(left: &OwnedGrid, right: &OwnedGrid) {
    assert_eq!(left.width(), right.width());
    assert_eq!(left.height(), right.height());
    for y in 0..left.height() {
        for x in 0..left.width() {
            assert_eq!(
                left.get(x, y),
                right.get(x, y),
                "cell mismatch at ({x}, {y})"
            );
        }
    }
}

#[test]
fn test_render_pipeline_with_spec_matches_options_no_effects() {
    let source = create_source_grid(6, 4, 'X');
    let mut dest_options = OwnedGrid::new(6, 4);
    let mut dest_spec = OwnedGrid::new(6, 4);

    let options = CompositionOptions::default();
    let spec = CompositionSpec::default();

    render_pipeline_legacy(&source, &mut dest_options, 6, 4, 0, 0, options, None);
    render_pipeline_with_spec_legacy(&source, &mut dest_spec, 6, 4, 0, 0, &spec, None);

    assert_grids_equal(&dest_options, &dest_spec);
}

#[test]
fn test_render_pipeline_with_spec_matches_options_shader_layer() {
    let source = create_source_grid(8, 5, 'S');
    let mut dest_options = OwnedGrid::new(8, 5);
    let mut dest_spec = OwnedGrid::new(8, 5);

    let shader = BorderSweepShader {
        speed: 1.0,
        length: 3,
        color: ColorConfig::Red,
        position_binding: None,
    };
    let spec_shader = shader.clone();

    let mut options = CompositionOptions::default()
        .with_shader_layer(&shader, StyleRegion::All)
        .with_mask_combine_mode(MaskCombineMode::All);
    options.t = 0.5;

    let spec = CompositionSpec {
        sampler_spec: None,
        masks: Vec::new(),
        mask_combine_mode: MaskCombineMode::All,
        filters: Vec::new(),
        shader_layers: vec![ShaderLayerSpec {
            shader: SpatialShaderType::BorderSweep(spec_shader),
            region: StyleRegion::All,
        }],
        shadow: None,
        preserve_unfilled: true,
        t: 0.5,
        loop_t: None,
        phase: None,
        runtime_params: ShaderRuntimeParams::default(),
    };

    render_pipeline_legacy(&source, &mut dest_options, 8, 5, 0, 0, options, None);
    render_pipeline_with_spec_legacy(&source, &mut dest_spec, 8, 5, 0, 0, &spec, None);

    assert_grids_equal(&dest_options, &dest_spec);
}

#[test]
fn test_render_pipeline_with_spec_matches_grouped_v3_options_shader_layer() {
    let source = create_source_grid(8, 5, 'S');
    let mut dest_options = OwnedGrid::new(8, 5);
    let mut dest_spec = OwnedGrid::new(8, 5);

    let shader = BorderSweepShader {
        speed: 1.0,
        length: 3,
        color: ColorConfig::Red,
        position_binding: None,
    };
    let family = VfxSpatialShaderFamily::ComposedPrimitive(
        VfxSpatialComposedPrimitive::TravelingBand((&shader).into()),
    );

    let mut options = CompositionOptions::default()
        .try_with_v3_shader_family(&family, &shader, StyleRegion::All)
        .expect("grouped family should lower");
    options.t = 0.5;

    let mut spec = CompositionSpec::default()
        .try_with_v3_shader_family(&family, StyleRegion::All)
        .expect("grouped family should lower");
    spec.t = 0.5;

    render_pipeline_legacy(&source, &mut dest_options, 8, 5, 0, 0, options, None);
    render_pipeline_with_spec_legacy(&source, &mut dest_spec, 8, 5, 0, 0, &spec, None);

    assert_grids_equal(&dest_options, &dest_spec);
}

#[test]
fn test_render_pipeline_with_spec_area_matches_options() {
    let source = create_source_grid(4, 3, 'A');
    let mut dest_options = OwnedGrid::new(6, 5);
    let mut dest_spec = OwnedGrid::new(6, 5);
    let area = RenderArea {
        width: 4,
        height: 3,
        offset_x: 1,
        offset_y: 1,
    };

    let options = CompositionOptions::default();
    let spec = CompositionSpec::default();

    render_pipeline_legacy(
        &source,
        &mut dest_options,
        area.width,
        area.height,
        area.offset_x,
        area.offset_y,
        options,
        None,
    );
    render_pipeline_with_spec_area_legacy(&source, &mut dest_spec, area, &spec, None);

    assert_grids_equal(&dest_options, &dest_spec);
}

#[test]
fn test_render_pipeline_with_spec_matches_options_shadow() {
    let source = create_source_grid(6, 4, 'S');
    let mut dest_options = OwnedGrid::new(6, 4);
    let mut dest_spec = OwnedGrid::new(6, 4);

    let shadow = ShadowSpec::new(
        ShadowConfig::new(Color::BLACK.with_alpha(128))
            .with_offset(1, 1)
            .with_edges(ShadowEdges::BOTTOM_RIGHT),
    );

    let options = CompositionOptions {
        shadow: Some(shadow.clone()),
        preserve_unfilled: false,
        ..Default::default()
    };

    let spec = CompositionSpec {
        sampler_spec: None,
        masks: Vec::new(),
        mask_combine_mode: MaskCombineMode::All,
        filters: Vec::new(),
        shader_layers: Vec::new(),
        shadow: Some(shadow),
        preserve_unfilled: false,
        t: 0.0,
        loop_t: None,
        phase: None,
        runtime_params: ShaderRuntimeParams::default(),
    };

    render_pipeline_legacy(&source, &mut dest_options, 6, 4, 0, 0, options, None);
    render_pipeline_with_spec_legacy(&source, &mut dest_spec, 6, 4, 0, 0, &spec, None);

    assert_grids_equal(&dest_options, &dest_spec);
}

#[test]
fn test_render_pipeline_with_spec_matches_options_shadow_grade_underlying() {
    let source = create_source_grid(6, 4, 'G');

    // Pre-fill dest grids with content so grading has something to grade
    let fill_cell = Cell {
        ch: 'B',
        fg: Color::new(220, 180, 80, 255),
        bg: Color::new(90, 110, 140, 255),
        ..Default::default()
    };
    let mut dest_options = OwnedGrid::new(8, 6);
    let mut dest_spec = OwnedGrid::new(8, 6);
    for y in 0..6 {
        for x in 0..8 {
            dest_options.set(x, y, fill_cell);
            dest_spec.set(x, y, fill_cell);
        }
    }

    let shadow = ShadowSpec::new(
        ShadowConfig::new(Color::BLACK.with_alpha(180))
            .with_offset(2, 2)
            .with_edges(ShadowEdges::BOTTOM_RIGHT)
            .with_style(tui_vfx_shadow::ShadowStyle::Solid)
            .with_dramatic_grade(),
    );

    let options = CompositionOptions {
        shadow: Some(shadow.clone()),
        preserve_unfilled: true,
        t: 1.0,
        ..Default::default()
    };

    let spec = CompositionSpec {
        sampler_spec: None,
        masks: Vec::new(),
        mask_combine_mode: MaskCombineMode::All,
        filters: Vec::new(),
        shader_layers: Vec::new(),
        shadow: Some(shadow),
        preserve_unfilled: true,
        t: 1.0,
        loop_t: None,
        phase: None,
        runtime_params: ShaderRuntimeParams::default(),
    };

    render_pipeline_legacy(&source, &mut dest_options, 6, 4, 0, 0, options, None);
    render_pipeline_with_spec_legacy(&source, &mut dest_spec, 6, 4, 0, 0, &spec, None);

    assert_grids_equal(&dest_options, &dest_spec);
}

// <FILE>tui-vfx-compositor/tests/pipeline/test_render_pipeline_with_spec.rs</FILE>
// <DESC>Spec-based pipeline wrapper tests</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
