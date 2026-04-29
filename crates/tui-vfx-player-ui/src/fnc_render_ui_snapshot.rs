// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ui_snapshot.rs</FILE> - <DESC>Render visual player state to terminal text</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player UI: display player frame rows and diagnostics in a visual shell.</WCTX>
// <CLOG>0.1.0: INIT — add bordered frame, metadata, diagnostics, and help rendering.</CLOG>

use crate::PlayerUiState;

/// Render the visual UI snapshot as terminal text.
pub fn render_ui_snapshot(state: &PlayerUiState, clear: bool) -> String {
    let report = state.report();
    let mut out = String::new();
    if clear {
        out.push_str("\x1b[2J\x1b[H");
    }
    out.push_str("tui-vfx contract-native player UI\n");
    out.push_str(&format!("recipe: {}\n", state.recipe_path.display()));
    out.push_str(&format!(
        "phase: {:?}  sample_t: {:.2}  loop_t: {}  paused: {}  motion_disabled: {}\n",
        state.phase(),
        state.phase_t(),
        state
            .loop_t()
            .map(|value| format!("{value:.2}"))
            .unwrap_or_else(|| "none".to_string()),
        state.paused,
        state.motion_disabled
    ));
    out.push_str(&format!(
        "status: {:?}  render_hash: {}  non_empty_cells: {}  elapsed_ms: {}\n",
        report.status, report.render_hash, report.non_empty_cells, state.elapsed_ms
    ));
    out.push_str(&format!("message: {}\n", state.message));
    out.push_str(&frame_rows(&report.rows));
    if report.dwell_terminated {
        out.push_str("\ndwell: terminated by canonical trigger\n");
    }
    if !report.errors.is_empty() {
        out.push_str("\ndiagnostics:\n");
        for error in &report.errors {
            out.push_str(&format!(
                "- {} at {}: {}\n",
                error.code, error.path, error.message
            ));
        }
    }
    if state.show_help {
        out.push_str(help_text());
    } else {
        out.push_str("\ncommands: q quit | ? help | space pause | r reset | m motion | [ ] phase | left/right scrub | t trigger\n");
    }
    out
}

fn frame_rows(rows: &[String]) -> String {
    let width = rows.iter().map(String::len).max().unwrap_or(0);
    let mut out = String::new();
    out.push_str(&format!("┌{}┐\n", "─".repeat(width + 2)));
    for row in rows {
        out.push_str(&format!(
            "│ {}{} │\n",
            row,
            " ".repeat(width.saturating_sub(row.len()))
        ));
    }
    out.push_str(&format!("└{}┘\n", "─".repeat(width + 2)));
    out
}

fn help_text() -> &'static str {
    "\nhelp:\n  q       quit\n  ?       toggle help\n  space   pause/resume\n  r       reset/reload player session\n  m       motion-disabled stable sample\n  [ / ]   previous/next phase\n  left    sample_t - 0.05\n  right   sample_t + 0.05\n  t       fire canonical signal-backed dwell trigger\n  tick    advance elapsed time when unpaused\n"
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ui_snapshot.rs</FILE> - <DESC>Render visual player state to terminal text</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
