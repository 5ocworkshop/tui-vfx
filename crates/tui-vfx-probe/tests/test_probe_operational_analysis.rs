// <FILE>crates/tui-vfx-probe/tests/test_probe_operational_analysis.rs</FILE> - <DESC>Regression tests for direct probe operational analysis</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>TDD for direct compositor-stage success/failure summaries layered on top of probe reports</WCTX>
// <CLOG>0.4.0: lock multi-sampler chain analysis so configured sampler rows stay successful when inspected traces collapse them into one ordered chain label.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::pipeline::CompositionSpec;
use tui_vfx_compositor::types::{ApplyTo, Axis, FilterSpec, RippleCenter, SamplerSpec};
use tui_vfx_probe::{
    ProbeCellSelector, ProbeGridSpec, ProbeOperationalStatus, ProbePhase, ProbePoint, ProbeRequest,
    ProbeSceneSpec, collect_probe_operational_analysis, run_probe,
};
use tui_vfx_style::models::{BorderSweepShader, ColorConfig, SpatialShaderType, StyleRegion};
use tui_vfx_types::{Cell, Color, Modifiers};

fn make_grid(width: u16, height: u16, ch: char) -> ProbeGridSpec {
    ProbeGridSpec {
        width,
        height,
        cells: vec![
            Cell::styled(ch, Color::WHITE, Color::BLACK, Modifiers::NONE);
            (width as usize) * (height as usize)
        ],
    }
}

fn shader_scene() -> ProbeSceneSpec {
    ProbeSceneSpec {
        source: make_grid(4, 3, 'S'),
        destination: make_grid(8, 6, ' '),
        widget_offset: ProbePoint { x: 1, y: 1 },
        composition: CompositionSpec {
            shader_layers: vec![tui_vfx_compositor::pipeline::ShaderLayerSpec {
                shader: SpatialShaderType::BorderSweep(BorderSweepShader {
                    speed: 1.0,
                    length: 2,
                    color: ColorConfig::Red,
                    head: None,
                    tail: None,
                    position_binding: None,
                }),
                region: StyleRegion::All,
            }],
            t: 0.5,
            ..CompositionSpec::default()
        },
    }
}

fn multi_sampler_scene() -> ProbeSceneSpec {
    let mut composition = CompositionSpec::default();
    composition.push_sampler(SamplerSpec::SineWave {
        axis: Axis::Y,
        amplitude: SignalOrFloat::Static(1.0),
        frequency: SignalOrFloat::Static(0.5),
        speed: SignalOrFloat::Static(1.0),
        phase: SignalOrFloat::Static(0.0),
    });
    composition.push_sampler(SamplerSpec::Ripple {
        amplitude: SignalOrFloat::Static(1.0),
        wavelength: SignalOrFloat::Static(5.0),
        speed: SignalOrFloat::Static(1.0),
        center: RippleCenter::Center,
    });
    ProbeSceneSpec {
        source: make_grid(6, 4, 'M'),
        destination: make_grid(10, 8, ' '),
        widget_offset: ProbePoint { x: 1, y: 1 },
        composition,
    }
}

#[test]
fn test_collect_probe_operational_analysis_reports_success_for_filter_stage() {
    let report = run_probe(
        &ProbeSceneSpec {
            source: make_grid(3, 2, 'A'),
            destination: make_grid(8, 6, ' '),
            widget_offset: ProbePoint { x: 1, y: 1 },
            composition: CompositionSpec {
                filters: vec![FilterSpec::Dim {
                    factor: SignalOrFloat::Static(0.5),
                    apply_to: ApplyTo::Both,
                }],
                ..CompositionSpec::default()
            },
        },
        &ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 1.0,
            cells: ProbeCellSelector::All,
            with_causation: true,
        },
    )
    .expect("report should build");

    let analysis = collect_probe_operational_analysis("frame", &[report]);
    assert_eq!(analysis.combined.status, ProbeOperationalStatus::Failure);
    assert!(analysis.stages.iter().any(|stage| stage.stage == "filter"
        && stage.status == ProbeOperationalStatus::Success
        && stage.observed_event_count > 0
        && stage.effects.iter().any(|effect| effect.effect == "Dim#1"
            && effect.configured_instances == 1
            && effect.status == ProbeOperationalStatus::Success)));
}

#[test]
fn test_collect_probe_operational_analysis_reports_shader_family_for_shader_stage() {
    let report = run_probe(
        &shader_scene(),
        &ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 0.5,
            cells: ProbeCellSelector::Modified,
            with_causation: true,
        },
    )
    .expect("report should build");

    let analysis = collect_probe_operational_analysis("frame", &[report]);
    assert!(analysis.stages.iter().any(|stage| {
        stage.stage == "shader"
            && stage.effects.iter().any(|effect| {
                effect.effect == "BorderSweep#1"
                    && effect.family.as_deref() == Some("traveling_band")
                    && effect.status == ProbeOperationalStatus::Success
            })
    }));
}

#[test]
fn test_collect_probe_operational_analysis_matches_multi_sampler_chain_labels() {
    let report = run_probe(
        &multi_sampler_scene(),
        &ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 1.0,
            cells: ProbeCellSelector::Modified,
            with_causation: true,
        },
    )
    .expect("report should build");

    let analysis = collect_probe_operational_analysis("frame", &[report]);
    let sampler_stage = analysis
        .stages
        .iter()
        .find(|stage| stage.stage == "sampler")
        .expect("sampler stage");

    assert_eq!(sampler_stage.configured_count, 2);
    assert!(
        sampler_stage
            .observed_effects
            .iter()
            .any(|effect| effect == "SineWave#1 -> Ripple#2")
    );
    assert!(sampler_stage.effects.iter().any(|effect| {
        effect.effect == "SineWave#1"
            && effect.configured_instances == 1
            && effect.status == ProbeOperationalStatus::Success
    }));
    assert!(sampler_stage.effects.iter().any(|effect| {
        effect.effect == "Ripple#2"
            && effect.configured_instances == 1
            && effect.status == ProbeOperationalStatus::Success
    }));
}

#[test]
fn test_collect_probe_operational_analysis_reports_failure_for_bad_border_scene() {
    let report = run_probe(
        &ProbeSceneSpec {
            source: ProbeGridSpec {
                width: 4,
                height: 3,
                cells: vec![
                    Cell::styled('A', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('─', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('─', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('╮', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('│', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('O', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('K', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('│', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('╰', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('▁', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('Z', Color::WHITE, Color::BLACK, Modifiers::NONE),
                    Cell::styled('╯', Color::WHITE, Color::BLACK, Modifiers::NONE),
                ],
            },
            destination: make_grid(6, 5, ' '),
            widget_offset: ProbePoint { x: 1, y: 1 },
            composition: CompositionSpec::default(),
        },
        &ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 1.0,
            cells: ProbeCellSelector::All,
            with_causation: false,
        },
    )
    .expect("report should build");

    let analysis = collect_probe_operational_analysis("frame", &[report]);
    assert_eq!(analysis.combined.status, ProbeOperationalStatus::Failure);
    assert!(
        analysis
            .combined
            .diagnostic_codes
            .contains(&"alpha_on_top_border".to_string())
    );
}

// <FILE>crates/tui-vfx-probe/tests/test_probe_operational_analysis.rs</FILE> - <DESC>Regression tests for direct probe operational analysis</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
