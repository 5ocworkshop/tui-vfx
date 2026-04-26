// <FILE>tui-vfx-compositor/tests/pipeline/test_render_pipeline_with_spec.rs</FILE>
// <DESC>Spec-based pipeline wrapper tests</DESC>
// <VERS>VERSION: 0.7.1</VERS>
// <WCTX>Sub-plan A Phase A.2.3 — route every call-site through the `_legacy` shim wrapper so these tests keep speaking the old `&mut OwnedGrid` call shape</WCTX>
// <CLOG>0.7.1: drop local `mod test_helpers;` declaration; pull from crate root (test_pipeline.rs declares once) to clear clippy::duplicate_mod.</CLOG>

use crate::test_helpers::{
    render_pipeline_legacy, render_pipeline_with_spec_area_legacy, render_pipeline_with_spec_legacy,
};

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::pipeline::{
    CompositionOptions, CompositionSpec, RenderArea, ShaderLayerSpec,
};
use tui_vfx_compositor::types::{Axis, MaskCombineMode, SamplerSpec, ShadowSpec};
use tui_vfx_shadow::{ShadowConfig, ShadowEdges};
use tui_vfx_style::models::{
    BorderSweepShader, ColorConfig, SpatialShaderType, StyleRegion, VfxSpatialComposedPrimitive,
    VfxSpatialShaderFamily,
};
use tui_vfx_style::traits::StyleShader;
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

fn create_coordinate_grid(width: usize, height: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            grid.set(
                x,
                y,
                Cell {
                    ch: char::from(b'A' + idx as u8),
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                    ..Default::default()
                },
            );
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

fn assert_grouped_family_matches_legacy_runtime<T>(
    shader: T,
    family: VfxSpatialShaderFamily,
    t: f64,
) where
    T: StyleShader + Clone,
{
    let source = create_source_grid(8, 5, 'S');
    let mut dest_options = OwnedGrid::new(8, 5);
    let mut dest_spec = OwnedGrid::new(8, 5);

    let mut options = CompositionOptions::default()
        .try_with_v3_shader_family(&family, &shader, StyleRegion::All)
        .expect("grouped family should lower");
    options.t = t;

    let mut spec = CompositionSpec::default()
        .try_with_v3_shader_family(&family, StyleRegion::All)
        .expect("grouped family should lower");
    spec.t = t;

    render_pipeline_legacy(&source, &mut dest_options, 8, 5, 0, 0, options, None);
    render_pipeline_with_spec_legacy(&source, &mut dest_spec, 8, 5, 0, 0, &spec, None);

    assert_grids_equal(&dest_options, &dest_spec);
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
        head: None,
        tail: None,
        position_binding: None,
    };
    let spec_shader = shader.clone();

    let mut options = CompositionOptions::default()
        .with_shader_layer(&shader, StyleRegion::All)
        .with_mask_combine_mode(MaskCombineMode::All);
    options.t = 0.5;

    let spec = CompositionSpec {
        sampler_spec: None,
        samplers: Vec::new(),
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
        head: None,
        tail: None,
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
fn test_render_pipeline_with_spec_matches_grouped_v3_options_across_representative_families() {
    let glow = tui_vfx_style::models::GlowShader::default();
    assert_grouped_family_matches_legacy_runtime(
        glow.clone(),
        VfxSpatialShaderFamily::Primitive(
            tui_vfx_style::models::VfxSpatialPrimitive::SurfaceDepth((&glow).into()),
        ),
        0.5,
    );

    let pulse = tui_vfx_style::models::PulseWaveShader::default();
    assert_grouped_family_matches_legacy_runtime(
        pulse.clone(),
        VfxSpatialShaderFamily::Primitive(tui_vfx_style::models::VfxSpatialPrimitive::MotionField(
            (&pulse).into(),
        )),
        0.5,
    );

    let glitch = tui_vfx_style::models::GlitchLinesShader::default();
    assert_grouped_family_matches_legacy_runtime(
        glitch.clone(),
        VfxSpatialShaderFamily::Primitive(
            tui_vfx_style::models::VfxSpatialPrimitive::EdgeDistortion((&glitch).into()),
        ),
        0.5,
    );

    let gradient = tui_vfx_style::models::LinearGradientShader::new(
        tui_vfx_style::models::Gradient::default(),
    );
    assert_grouped_family_matches_legacy_runtime(
        gradient.clone(),
        VfxSpatialShaderFamily::Primitive(
            tui_vfx_style::models::VfxSpatialPrimitive::GradientReveal((&gradient).into()),
        ),
        0.0,
    );

    let highlighter = tui_vfx_style::models::HighlighterShader::default();
    assert_grouped_family_matches_legacy_runtime(
        highlighter.clone(),
        VfxSpatialShaderFamily::ComposedPrimitive(
            tui_vfx_style::models::VfxSpatialComposedPrimitive::ProgressEmphasis(
                (&highlighter).into(),
            ),
        ),
        0.5,
    );

    let diffusion = tui_vfx_style::models::DiffusionShader::default();
    assert_grouped_family_matches_legacy_runtime(
        diffusion.clone(),
        VfxSpatialShaderFamily::ComposedPrimitive(
            tui_vfx_style::models::VfxSpatialComposedPrimitive::MaterialLight((&diffusion).into()),
        ),
        0.5,
    );

    let guidance = tui_vfx_style::models::FocusedRowGradientShader::default();
    assert_grouped_family_matches_legacy_runtime(
        guidance.clone(),
        VfxSpatialShaderFamily::ComposedPrimitive(
            tui_vfx_style::models::VfxSpatialComposedPrimitive::GuidanceCue((&guidance).into()),
        ),
        0.5,
    );

    let stochastic = tui_vfx_style::models::NeonFlickerShader::default();
    assert_grouped_family_matches_legacy_runtime(
        stochastic.clone(),
        VfxSpatialShaderFamily::ComposedPrimitive(
            tui_vfx_style::models::VfxSpatialComposedPrimitive::StochasticTexture(
                (&stochastic).into(),
            ),
        ),
        0.5,
    );

    let stripe = tui_vfx_style::models::BarberPoleShader {
        speed: 1.0,
        stripe_width: 2,
        gap_width: 2,
        color: ColorConfig::Red,
    };
    assert_grouped_family_matches_legacy_runtime(
        stripe.clone(),
        VfxSpatialShaderFamily::ComposedPrimitive(
            tui_vfx_style::models::VfxSpatialComposedPrimitive::StripeMotion((&stripe).into()),
        ),
        0.5,
    );
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
fn test_render_pipeline_with_spec_matches_options_sampler_chain() {
    let source = create_coordinate_grid(6, 4);
    let mut dest_options = OwnedGrid::new(6, 4);
    let mut dest_spec = OwnedGrid::new(6, 4);
    let gravity = SamplerSpec::Gravity {
        axis: Axis::X,
        acceleration: SignalOrFloat::Static(2.0),
        terminal_velocity: SignalOrFloat::Static(2.0),
    };
    let pendulum = SamplerSpec::Pendulum {
        axis: Axis::Y,
        amplitude: SignalOrFloat::Static(1.0),
        speed: SignalOrFloat::Static(0.0),
        phase_spread: SignalOrFloat::Static(std::f32::consts::FRAC_PI_2),
    };

    let mut options =
        CompositionOptions::default().with_samplers(vec![gravity.clone(), pendulum.clone()]);
    options.t = 1.0;

    let spec = CompositionSpec {
        sampler_spec: Some(gravity),
        samplers: vec![
            SamplerSpec::Gravity {
                axis: Axis::X,
                acceleration: SignalOrFloat::Static(2.0),
                terminal_velocity: SignalOrFloat::Static(2.0),
            },
            pendulum,
        ],
        t: 1.0,
        ..CompositionSpec::default()
    };

    render_pipeline_legacy(&source, &mut dest_options, 6, 4, 0, 0, options, None);
    render_pipeline_with_spec_legacy(&source, &mut dest_spec, 6, 4, 0, 0, &spec, None);

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
        samplers: Vec::new(),
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
        samplers: Vec::new(),
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
// <VERS>END OF VERSION: 0.7.0</VERS>
