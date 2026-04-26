// <FILE>crates/tui-vfx-probe/tests/test_probe_timeline_diff.rs</FILE> - <DESC>Integration tests for probe timelines and frame diffs</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Fix broken struct literals in dim_scene/shader_scene helpers after 71a6ff2 mis-placed loopback_fired_keys outside the struct braces</WCTX>
// <CLOG>Move loopback_fired_keys inside the ProbeSceneSpec struct literal in dim_scene and shader_scene; removes the syntax error introduced by the automated patch script in 71a6ff2</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::pipeline::{CompositionSpec, ShaderLayerSpec};
use tui_vfx_compositor::types::{ApplyTo, FilterSpec};
use tui_vfx_probe::{
    ProbeCellSelector, ProbeGridSpec, ProbePhase, ProbePoint, ProbeRequest, ProbeSceneSpec,
    collect_timeline, run_probe, run_probe_diff,
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
        loopback_fired_keys: Vec::new(),
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
                    head: None,
                    tail: None,
                    position_binding: None,
                }),
                region: StyleRegion::All,
            }],
            t: 0.0,
            ..CompositionSpec::default()
        },
        loopback_fired_keys: Vec::new(),
    }
}

#[test]
fn test_run_probe_with_causation_includes_filter_before_after_snapshots() {
    let report = run_probe(
        &dim_scene(),
        &ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 1.0,
            cells: ProbeCellSelector::Modified,
            with_causation: true,
        },
    )
    .expect("probe should succeed");

    let trace = &report.cells[0].trace;
    assert_eq!(trace.len(), 1);
    assert_eq!(trace[0].stage, "filter");
    assert_eq!(
        trace[0].before.as_ref().and_then(|state| state.ch),
        Some('A')
    );
    assert_eq!(
        trace[0].after.as_ref().and_then(|state| state.ch),
        Some('A')
    );
    assert_eq!(trace[0].before.as_ref().map(|state| state.fg.r), Some(255));
    assert_eq!(trace[0].after.as_ref().map(|state| state.fg.r), Some(128));
}

#[test]
fn test_collect_timeline_returns_requested_frame_count() {
    let timeline = collect_timeline(
        &dim_scene(),
        ProbePhase::Dwelling,
        3,
        &ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 0.0,
            cells: ProbeCellSelector::Modified,
            with_causation: false,
        },
    )
    .expect("timeline should succeed");

    assert_eq!(timeline.kind, "timeline");
    assert_eq!(timeline.frame_count, 3);
    assert_eq!(timeline.frames[0].timing.requested_t, 0.0);
    assert_eq!(timeline.frames[2].timing.requested_t, 1.0);
}

#[test]
fn test_run_probe_diff_reports_changed_cells_between_samples() {
    let diff = run_probe_diff(&shader_scene(), ProbePhase::Dwelling, 0.0, 0.5, true)
        .expect("diff should succeed");

    assert_eq!(diff.kind, "frame_diff");
    assert!(diff.changed_cells_count > 0);
    assert!(
        diff.cells
            .iter()
            .all(|cell| cell.trace[0].stage == "shader")
    );
    assert!(
        diff.cells
            .iter()
            .any(|cell| cell.before.fg != cell.after.fg || cell.before.bg != cell.after.bg)
    );
}

// <FILE>crates/tui-vfx-probe/tests/test_probe_timeline_diff.rs</FILE> - <DESC>Integration tests for probe timelines and frame diffs</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
