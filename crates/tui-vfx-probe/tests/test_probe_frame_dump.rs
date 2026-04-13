// <FILE>crates/tui-vfx-probe/tests/test_probe_frame_dump.rs</FILE> - <DESC>Integration tests for phase-1 probe frame dumps</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>TDD for the first engine-owned probe slice including no-effect attribution guards and with-causation coverage</WCTX>
// <CLOG>MINOR: Add regression tests proving no-effect probes omit fake last-touch attribution and with-causation requests emit a trace entry for modified cells</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::pipeline::{CompositionSpec, ShaderLayerSpec};
use tui_vfx_compositor::types::{ApplyTo, FilterSpec};
use tui_vfx_probe::{
    ProbeCellSelector, ProbeGridSpec, ProbePhase, ProbePoint, ProbeRequest, ProbeSceneSpec,
    run_probe,
};
use tui_vfx_style::models::BorderSweepShader;
use tui_vfx_style::models::{ColorConfig, SpatialShaderType, StyleRegion};
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
fn test_run_probe_reports_full_widget_dump() {
    let scene = ProbeSceneSpec {
        source: make_grid(3, 2, 'A'),
        destination: make_grid(8, 6, ' '),
        widget_offset: ProbePoint { x: 2, y: 1 },
        composition: CompositionSpec::default(),
    };
    let request = ProbeRequest {
        phase: ProbePhase::Dwelling,
        sample_t: 0.5,
        cells: ProbeCellSelector::All,
        with_causation: false,
    };

    let report = run_probe(&scene, &request).expect("probe should succeed");

    assert_eq!(report.cells.len(), 6);
    assert_eq!(report.widget.abs_origin.x, 2);
    assert_eq!(report.widget.abs_origin.y, 1);
    assert_eq!(report.widget.size.width, 3);
    assert_eq!(report.widget.size.height, 2);
    assert_eq!(report.summary.total_cells, 6);
    assert_eq!(report.summary.modified_cells, 0);
    assert_eq!(report.timing.requested_phase, ProbePhase::Dwelling);
    assert_eq!(report.timing.effective_phase, ProbePhase::Dwelling);
}

#[test]
fn test_run_probe_modified_selector_reports_only_filter_touched_cells() {
    let scene = ProbeSceneSpec {
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
    };
    let request = ProbeRequest {
        phase: ProbePhase::Dwelling,
        sample_t: 1.0,
        cells: ProbeCellSelector::Modified,
        with_causation: false,
    };

    let report = run_probe(&scene, &request).expect("probe should succeed");

    assert_eq!(report.cells.len(), 6);
    assert_eq!(report.summary.modified_cells, 6);
    assert!(report.cells.iter().all(|cell| {
        cell.last_touch
            .as_ref()
            .is_some_and(|touch| touch.stage == "filter")
    }));
}

#[test]
fn test_run_probe_modified_selector_reports_only_shader_touched_cells() {
    let scene = ProbeSceneSpec {
        source: make_grid(4, 3, 'S'),
        destination: make_grid(8, 6, ' '),
        widget_offset: ProbePoint { x: 1, y: 1 },
        composition: CompositionSpec {
            shader_layers: vec![ShaderLayerSpec {
                shader: SpatialShaderType::BorderSweep(BorderSweepShader {
                    speed: 1.0,
                    length: 2,
                    color: ColorConfig::Red,
                }),
                region: StyleRegion::All,
            }],
            t: 0.5,
            ..CompositionSpec::default()
        },
    };
    let request = ProbeRequest {
        phase: ProbePhase::Dwelling,
        sample_t: 0.5,
        cells: ProbeCellSelector::Modified,
        with_causation: false,
    };

    let report = run_probe(&scene, &request).expect("probe should succeed");

    assert!(!report.cells.is_empty());
    assert!(report.cells.iter().all(|cell| {
        cell.last_touch
            .as_ref()
            .is_some_and(|touch| touch.stage == "shader")
    }));
}

#[test]
fn test_run_probe_no_effect_dump_omits_last_touch_and_trace() {
    let scene = ProbeSceneSpec {
        source: make_grid(2, 1, 'N'),
        destination: make_grid(4, 3, ' '),
        widget_offset: ProbePoint { x: 1, y: 1 },
        composition: CompositionSpec::default(),
    };
    let request = ProbeRequest {
        phase: ProbePhase::Dwelling,
        sample_t: 0.5,
        cells: ProbeCellSelector::All,
        with_causation: true,
    };

    let report = run_probe(&scene, &request).expect("probe should succeed");

    assert!(report.cells.iter().all(|cell| cell.last_touch.is_none()));
    assert!(report.cells.iter().all(|cell| cell.trace.is_empty()));
}

#[test]
fn test_run_probe_with_causation_emits_trace_for_modified_cells() {
    let scene = ProbeSceneSpec {
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
    };
    let request = ProbeRequest {
        phase: ProbePhase::Dwelling,
        sample_t: 1.0,
        cells: ProbeCellSelector::Modified,
        with_causation: true,
    };

    let report = run_probe(&scene, &request).expect("probe should succeed");

    assert!(report.cells.iter().all(|cell| cell.trace.len() == 1));
    assert!(
        report
            .cells
            .iter()
            .all(|cell| cell.trace[0].stage == "filter")
    );
}

#[test]
fn test_run_probe_rejects_grid_shape_mismatch() {
    let scene = ProbeSceneSpec {
        source: ProbeGridSpec {
            width: 2,
            height: 2,
            cells: vec![Cell::new('x'); 3],
        },
        destination: make_grid(4, 4, ' '),
        widget_offset: ProbePoint { x: 0, y: 0 },
        composition: CompositionSpec::default(),
    };
    let request = ProbeRequest {
        phase: ProbePhase::Dwelling,
        sample_t: 0.5,
        cells: ProbeCellSelector::All,
        with_causation: false,
    };

    let error = run_probe(&scene, &request).expect_err("shape mismatch should fail");
    assert!(error.to_string().contains("invalid scene"));
}

// <FILE>crates/tui-vfx-probe/tests/test_probe_frame_dump.rs</FILE> - <DESC>Integration tests for phase-1 probe frame dumps</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
