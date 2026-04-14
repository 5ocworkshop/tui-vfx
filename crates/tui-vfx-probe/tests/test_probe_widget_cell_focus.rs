// <FILE>crates/tui-vfx-probe/tests/test_probe_widget_cell_focus.rs</FILE> - <DESC>Regression tests for focused widget-cell lookup in direct probe reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>TDD for root-cause-first direct probe CLI focus flow</WCTX>
// <CLOG>NEW: Add a small regression proving direct probe consumers can retrieve one widget-local cell and its root cause directly from a full report</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::pipeline::CompositionSpec;
use tui_vfx_compositor::types::{ApplyTo, FilterSpec};
use tui_vfx_probe::{
    ProbeCellSelector, ProbeGridSpec, ProbePhase, ProbePoint, ProbeRequest, ProbeSceneSpec,
    find_widget_cell, run_probe,
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
fn test_find_widget_cell_returns_root_cause_ready_cell() {
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
    .expect("probe should run");

    let focus = find_widget_cell(&report, 0, 0).expect("cell should exist");
    assert_eq!(focus.widget_local.x, 0);
    assert_eq!(focus.widget_local.y, 0);
    assert!(focus.root_cause.is_some());
}

// <FILE>crates/tui-vfx-probe/tests/test_probe_widget_cell_focus.rs</FILE> - <DESC>Regression tests for focused widget-cell lookup in direct probe reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
