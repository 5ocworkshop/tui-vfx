// <FILE>crates/tui-vfx-probe/tests/test_probe_operational_analysis.rs</FILE> - <DESC>Regression tests for direct probe operational analysis</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>TDD for direct compositor-stage success/failure summaries layered on top of probe reports</WCTX>
// <CLOG>MINOR: Assert that direct per-effect analysis discloses configured_instances so SQL consumers can tell when a row is a unique effect versus a same-name aggregate</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::pipeline::CompositionSpec;
use tui_vfx_compositor::types::{ApplyTo, FilterSpec};
use tui_vfx_probe::{
    ProbeCellSelector, ProbeGridSpec, ProbeOperationalStatus, ProbePhase, ProbePoint, ProbeRequest,
    ProbeSceneSpec, collect_probe_operational_analysis, run_probe,
};
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
// <VERS>END OF VERSION: 0.2.0</VERS>
