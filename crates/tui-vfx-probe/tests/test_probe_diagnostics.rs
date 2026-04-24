// <FILE>crates/tui-vfx-probe/tests/test_probe_diagnostics.rs</FILE> - <DESC>Regression tests for basic probe diagnostics helpers</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>TDD for initial probe-side diagnostics covering border contamination and underline placement</WCTX>
// <CLOG>0.3.0: prove plain text rows without border glyphs no longer trigger border-only alpha diagnostics.</CLOG>

use tui_vfx_probe::{
    ProbeCell, ProbeCellSelector, ProbeColor, ProbeFrame, ProbePhase, ProbePipelineInventory,
    ProbePoint, ProbeReport, ProbeReportSource, ProbeRequest, ProbeSize, ProbeSummary, ProbeTiming,
    ProbeWidget, collect_basic_diagnostics, row_text,
};

fn probe_color() -> ProbeColor {
    ProbeColor {
        space: "rgb".to_string(),
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }
}

fn make_report(rows: &[&str]) -> ProbeReport {
    let width = rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or_default() as u16;
    let height = rows.len() as u16;
    let mut cells = Vec::new();

    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            cells.push(ProbeCell {
                abs: ProbePoint {
                    x: x as u16,
                    y: y as u16,
                },
                widget_local: ProbePoint {
                    x: x as u16,
                    y: y as u16,
                },
                ch,
                fg: probe_color(),
                bg: probe_color(),
                modifiers: Vec::new(),
                last_touch: None,
                trace: Vec::new(),
                root_cause: None,
            });
        }
    }

    ProbeReport {
        schema_version: "0.1.0".to_string(),
        kind: "frame_dump".to_string(),
        source: ProbeReportSource {
            input_kind: "test".to_string(),
        },
        request: ProbeRequest {
            phase: ProbePhase::Dwelling,
            sample_t: 1.0,
            cells: ProbeCellSelector::All,
            with_causation: false,
        },
        timing: ProbeTiming {
            requested_phase: ProbePhase::Dwelling,
            requested_t: 1.0,
            effective_phase: ProbePhase::Dwelling,
            effective_t: 1.0,
            tick_ms: None,
        },
        frame: ProbeFrame {
            size: ProbeSize { width, height },
        },
        widget: ProbeWidget {
            abs_origin: ProbePoint { x: 0, y: 0 },
            size: ProbeSize { width, height },
        },
        pipeline: ProbePipelineInventory {
            sampler: None,
            sampler_count: 0,
            sampler_effects: Vec::new(),
            mask_count: 0,
            mask_effects: Vec::new(),
            filter_count: 0,
            filter_effects: Vec::new(),
            shader_count: 0,
            shader_effects: Vec::new(),
            shader_families: Vec::new(),
            style_count: 0,
            style_effects: Vec::new(),
            style_effect_families: Vec::new(),
            content_count: 0,
            content_effects: Vec::new(),
        },
        runtime: None,
        summary: ProbeSummary {
            total_cells: cells.len(),
            non_empty_cells: cells.len(),
            modified_cells: 0,
        },
        diagnostics: Vec::new(),
        cells,
    }
}

#[test]
fn test_collect_basic_diagnostics_reports_alpha_on_border_rows() {
    let report = make_report(&["A──╮", "│OK│", "╰──Z"]);
    let diagnostics = collect_basic_diagnostics(&report);
    let codes = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect::<Vec<_>>();
    assert!(codes.contains(&"alpha_on_top_border"));
    assert!(codes.contains(&"alpha_on_bottom_border"));
}

#[test]
fn test_collect_basic_diagnostics_reports_underline_on_bottom_border() {
    let report = make_report(&["╭──╮", "│OK│", "╰▁▁╯"]);
    let diagnostics = collect_basic_diagnostics(&report);
    let codes = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect::<Vec<_>>();
    assert!(codes.contains(&"underline_on_bottom_border"));
}

#[test]
fn test_collect_basic_diagnostics_is_empty_for_clean_border_rows() {
    let report = make_report(&["╭──╮", "│OK│", "╰──╯"]);
    let diagnostics = collect_basic_diagnostics(&report);
    assert!(diagnostics.is_empty());
}

#[test]
fn test_collect_basic_diagnostics_ignores_plain_text_top_and_bottom_rows() {
    let report = make_report(&["HELLO", " M ", "WORLD"]);
    let diagnostics = collect_basic_diagnostics(&report);
    assert!(diagnostics.is_empty());
}

#[test]
fn test_row_text_preserves_horizontal_gaps_from_widget_coordinates() {
    let mut report = make_report(&["╭──╮", "│OK│", "╰──╯"]);
    report.widget.size.width = 5;
    report.cells.retain(|cell| {
        !(cell.widget_local.y == 1 && (cell.widget_local.x == 1 || cell.widget_local.x == 3))
    });

    assert_eq!(row_text(&report, 1), "│ K  ");
}

// <FILE>crates/tui-vfx-probe/tests/test_probe_diagnostics.rs</FILE> - <DESC>Regression tests for basic probe diagnostics helpers</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
