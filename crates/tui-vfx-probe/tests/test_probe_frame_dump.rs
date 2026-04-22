// <FILE>crates/tui-vfx-probe/tests/test_probe_frame_dump.rs</FILE> - <DESC>Integration tests for phase-1 probe frame dumps</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>TDD for the first engine-owned probe slice including no-effect attribution guards and with-causation coverage</WCTX>
// <CLOG>MINOR: Add regression coverage proving direct probe reports auto-populate diagnostics from the full widget dump instead of requiring callers to invoke helpers manually</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::pipeline::{CompositionSpec, ShaderLayerSpec};
use tui_vfx_compositor::types::{ApplyTo, FilterSpec};
use tui_vfx_probe::{
    ProbeCellSelector, ProbeGridSpec, ProbePhase, ProbePoint, ProbeRequest, ProbeSceneSpec,
    run_probe,
};
use tui_vfx_style::models::BorderSweepShader;
use tui_vfx_style::models::{
    ApplyToColor, ColorConfig, FocusedRowGradientShader, SpatialShaderType, StyleRegion,
};
use tui_vfx_style::traits::ShaderRuntimeParams;
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

fn make_grid_from_rows(rows: &[&str]) -> ProbeGridSpec {
    let height = rows.len() as u16;
    let width = rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or_default() as u16;
    let mut cells = Vec::with_capacity(width as usize * height as usize);
    for row in rows {
        let mut chars = row.chars().collect::<Vec<_>>();
        while chars.len() < width as usize {
            chars.push(' ');
        }
        for ch in chars {
            cells.push(Cell::styled(
                ch,
                Color::WHITE,
                Color::BLACK,
                Modifiers::NONE,
            ));
        }
    }
    ProbeGridSpec {
        width,
        height,
        cells,
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
                    position_binding: None,
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
    assert!(
        report
            .pipeline
            .shader_families
            .iter()
            .any(|family| family == "traveling_band")
    );
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
fn test_run_probe_auto_populates_basic_diagnostics() {
    let scene = ProbeSceneSpec {
        source: make_grid_from_rows(&["A──╮", "│OK│", "╰▁Z╯"]),
        destination: make_grid(6, 5, ' '),
        widget_offset: ProbePoint { x: 1, y: 1 },
        composition: CompositionSpec::default(),
    };
    let request = ProbeRequest {
        phase: ProbePhase::Dwelling,
        sample_t: 1.0,
        cells: ProbeCellSelector::Modified,
        with_causation: false,
    };

    let report = run_probe(&scene, &request).expect("probe should succeed");
    let codes = report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect::<Vec<_>>();

    assert!(codes.contains(&"alpha_on_top_border"));
    assert!(codes.contains(&"alpha_on_bottom_border"));
    assert!(codes.contains(&"underline_on_bottom_border"));
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

#[test]
fn test_run_probe_reports_runtime_bindings_params_and_root_cause() {
    let scene = ProbeSceneSpec {
        source: make_grid(1, 6, 'A'),
        destination: make_grid(4, 8, ' '),
        widget_offset: ProbePoint { x: 1, y: 1 },
        composition: CompositionSpec {
            shader_layers: vec![ShaderLayerSpec {
                shader: SpatialShaderType::FocusedRowGradient(FocusedRowGradientShader {
                    selected_row: None,
                    selected_row_binding: Some("selected_row".to_owned()),
                    selected_row_ratio: 0.0,
                    selected_row_ratio_binding: None,
                    falloff_distance: 1,
                    bright_color: ColorConfig::White,
                    dim_color: ColorConfig::Black,
                    apply_to: ApplyToColor::Background,
                }),
                region: StyleRegion::All,
            }],
            runtime_params: [("selected_row", 3_u16)]
                .into_iter()
                .collect::<ShaderRuntimeParams>(),
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
    assert_eq!(
        report
            .runtime
            .as_ref()
            .expect("runtime context")
            .supplied_params[0]
            .key,
        "selected_row"
    );
    assert!(
        report
            .runtime
            .as_ref()
            .expect("runtime context")
            .binding_resolutions
            .iter()
            .any(|resolution| resolution.field == "selected_row")
    );
    assert!(report.cells.iter().any(|cell| {
        cell.root_cause
            .as_ref()
            .is_some_and(|cause| cause.summary.contains("shader"))
    }));
    assert!(report.cells.iter().any(|cell| {
        cell.trace.iter().any(|event| {
            event.stage == "shader"
                && event.params.is_some()
                && event.params.as_ref().is_some_and(|params| {
                    params.get("binding_resolutions").is_some()
                        || params
                            .as_array()
                            .and_then(|items| items.first())
                            .and_then(|item| item.get("binding_resolutions"))
                            .is_some()
                })
        })
    }));
}

// <FILE>crates/tui-vfx-probe/tests/test_probe_frame_dump.rs</FILE> - <DESC>Integration tests for phase-1 probe frame dumps</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
