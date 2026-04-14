// <FILE>crates/tui-vfx-probe/tests/test_probe_sqlite_store.rs</FILE> - <DESC>Tests for the in-memory SQLite playback index</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>TDD for the embedded SQLite query backend including full trace snapshots</WCTX>
// <CLOG>MINOR: Extend SQLite store coverage to prove report-level diagnostics are indexed and queryable alongside raw probe frames/traces and operational analysis</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use serde_json::json;
use tui_vfx_compositor::pipeline::{CompositionSpec, ShaderLayerSpec};
use tui_vfx_compositor::types::{ApplyTo, FilterSpec};
use tui_vfx_probe::{
    ProbeCellSelector, ProbeGridSpec, ProbePhase, ProbePoint, ProbeRequest, ProbeSceneSpec,
    ProbeSqliteStore, collect_timeline, run_probe, run_probe_diff,
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

fn dim_scene() -> ProbeSceneSpec {
    ProbeSceneSpec {
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
    }
}

fn shader_scene() -> ProbeSceneSpec {
    ProbeSceneSpec {
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
            ..CompositionSpec::default()
        },
    }
}

#[test]
fn test_sqlite_store_indexes_frame_report_cells_and_trace_events() {
    let report = run_probe(
        &dim_scene(),
        &ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 1.0,
            cells: ProbeCellSelector::Modified,
            with_causation: true,
        },
    )
    .unwrap();
    let store = ProbeSqliteStore::new_in_memory().unwrap();
    store.ingest_report("frame", &report).unwrap();

    let rows = store
        .query_json("select count(*) as count from probe_cells")
        .unwrap();
    assert_eq!(rows[0]["count"], 6);
    let trace_rows = store
        .query_json("select count(*) as count from probe_trace_events where stage = 'filter'")
        .unwrap();
    assert_eq!(trace_rows[0]["count"], 6);
    let bg_snapshot_rows = store
        .query_json("select count(*) as count from probe_trace_events where stage = 'filter' and before_bg_r is not null and after_bg_r is not null")
        .unwrap();
    assert_eq!(bg_snapshot_rows[0]["count"], 6);
    let diagnostic_rows = store
        .query_json("select code from probe_diagnostics order by code")
        .unwrap();
    assert_eq!(diagnostic_rows.len(), 2);
    assert_eq!(diagnostic_rows[0]["code"], "alpha_on_bottom_border");
    assert_eq!(diagnostic_rows[1]["code"], "alpha_on_top_border");
}

#[test]
fn test_sqlite_store_indexes_timeline_frames() {
    let timeline = collect_timeline(
        &shader_scene(),
        ProbePhase::Dwelling,
        3,
        &ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 0.0,
            cells: ProbeCellSelector::Modified,
            with_causation: false,
        },
    )
    .unwrap();
    let store = ProbeSqliteStore::new_in_memory().unwrap();
    store.ingest_timeline("timeline", &timeline).unwrap();

    let rows = store
        .query_json("select count(*) as count from probe_frames")
        .unwrap();
    assert_eq!(rows[0]["count"], 3);
}

#[test]
fn test_sqlite_store_indexes_diff_rows() {
    let diff = run_probe_diff(&shader_scene(), ProbePhase::Dwelling, 0.0, 0.5, true).unwrap();
    let store = ProbeSqliteStore::new_in_memory().unwrap();
    store.ingest_diff("diff", &diff).unwrap();

    let rows = store.query_json("select changed_cells_count from probe_runs join (select count(*) as changed_cells_count from probe_diff_cells where run_id = 'diff')").unwrap();
    assert!(rows[0]["changed_cells_count"].as_i64().unwrap() > 0);
}

#[test]
fn test_sqlite_store_indexes_operational_analysis_rows() {
    let store = ProbeSqliteStore::new_in_memory().unwrap();
    store
        .ingest_operational_analysis_value(
            "analysis",
            &json!({
                "scope": "frame",
                "stages": [
                    {
                        "stage": "filter",
                        "configured": true,
                        "configured_count": 1,
                        "touched_cells": 6,
                        "observed_event_count": 6,
                        "observed_effects": ["Dim"],
                        "effects": [
                            {
                                "effect": "Dim",
                                "configured": true,
                                "touched_cells": 6,
                                "observed_event_count": 6,
                                "status": "success"
                            }
                        ],
                        "status": "success"
                    }
                ],
                "combined": {
                    "status": "success",
                    "error_diagnostics": 0,
                    "warning_diagnostics": 0,
                    "failing_stages": [],
                    "diagnostic_codes": []
                }
            }),
        )
        .unwrap();
    store
        .ingest_lifecycle_analysis_value(
            "analysis",
            &json!({
                "stages": [
                    {
                        "stage": "filter",
                        "configured": true,
                        "configured_count": 1,
                        "touched_cells": 12,
                        "observed_event_count": 12,
                        "observed_effects": ["Dim"],
                        "effects": [
                            {
                                "effect": "Dim",
                                "configured": true,
                                "touched_cells": 12,
                                "observed_event_count": 12,
                                "status": "success"
                            }
                        ],
                        "status": "success"
                    }
                ],
                "combined": {
                    "status": "success",
                    "error_diagnostics": 0,
                    "warning_diagnostics": 0,
                    "failing_stages": [],
                    "diagnostic_codes": []
                },
                "phases": [
                    {
                        "phase": "entering",
                        "sample_t": 0.5,
                        "analysis": {
                            "stages": [
                                {
                                    "stage": "filter",
                                    "configured": true,
                                "configured_count": 1,
                                "touched_cells": 4,
                                "observed_event_count": 4,
                                "observed_effects": ["Dim"],
                                "effects": [
                                    {
                                        "effect": "Dim",
                                        "configured": true,
                                        "touched_cells": 4,
                                        "observed_event_count": 4,
                                        "status": "success"
                                    }
                                ],
                                "status": "success"
                            }
                        ],
                            "combined": {
                                "status": "success",
                                "error_diagnostics": 0,
                                "warning_diagnostics": 0,
                                "failing_stages": [],
                                "diagnostic_codes": []
                            }
                        }
                    }
                ]
            }),
        )
        .unwrap();

    let stage_rows = store
        .query_json("select count(*) as count from probe_analysis_stages where stage = 'filter'")
        .unwrap();
    assert_eq!(stage_rows[0]["count"], 3);
    let lifecycle_rows = store
        .query_json("select count(*) as count from probe_analysis_combined where scope = 'lifecycle_phase' and phase = 'entering'")
        .unwrap();
    assert_eq!(lifecycle_rows[0]["count"], 1);
    let effect_rows = store
        .query_json("select count(*) as count from probe_analysis_effects where effect = 'Dim'")
        .unwrap();
    assert_eq!(effect_rows[0]["count"], 3);
}

#[test]
fn test_sqlite_store_indexes_report_diagnostics_rows() {
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
            cells: ProbeCellSelector::Modified,
            with_causation: false,
        },
    )
    .unwrap();
    let store = ProbeSqliteStore::new_in_memory().unwrap();
    store.ingest_report("diag", &report).unwrap();

    let rows = store
        .query_json("select code, severity from probe_diagnostics order by code")
        .unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["code"], "alpha_on_bottom_border");
    assert_eq!(rows[0]["severity"], "error");
}

// <FILE>crates/tui-vfx-probe/tests/test_probe_sqlite_store.rs</FILE> - <DESC>Tests for the in-memory SQLite playback index</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
