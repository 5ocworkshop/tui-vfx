// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ui_snapshot.rs</FILE> - <DESC>Render visual player state to terminal text</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Player UI: display shared backend parity evidence in the same visual shell used for manual review.</WCTX>
// <CLOG>0.3.0: MINOR — include FPS/frame-time and black-canvas presentation state.
// 0.2.0: MINOR — show backend letter-cell evidence in UI snapshots.
// 0.1.0: INIT — add bordered frame, metadata, diagnostics, and help rendering.</CLOG>

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
        "phase: {:?}  sample_t: {:.2}  loop_t: {}  paused: {}  motion_disabled: {}  black_canvas: {}  fps: {:.1}  frame_ms: {:.1}\n",
        state.phase(),
        state.phase_t(),
        state
            .loop_t()
            .map(|value| format!("{value:.2}"))
            .unwrap_or_else(|| "none".to_string()),
        state.paused,
        state.motion_disabled,
        state.black_canvas_enabled(),
        state.fps(),
        state.frame_time_ms()
    ));
    out.push_str(&format!(
        "status: {:?}  render_hash: {}  backend: {}  composition_mode: {}  fallback_used: {}  source_render_mode: {}  native_source_isolated: {}  native_lowering_attempted: {}  native_lowering_succeeded: {}  composition_spec_non_empty: {}  lowered_nodes: {}  unlowered_nodes: {}  backend_hash: {}  non_empty_cells: {}  styled_cells: {}  letter_cells: {}  elapsed_ms: {}\n",
        report.status,
        report.render_hash,
        state.last_backend_output.backend,
        state.last_backend_output.composition_mode,
        state.last_backend_output.fallback_used,
        state.last_backend_output.source_render_mode,
        state.last_backend_output.native_source_isolated,
        state.last_backend_output.native_lowering_attempted,
        state.last_backend_output.native_lowering_succeeded,
        state.last_backend_output.composition_spec_non_empty,
        state.last_backend_output.lowered_node_count,
        state.last_backend_output.unlowered_node_count,
        state.last_backend_output.backend_hash,
        report.non_empty_cells,
        state.last_backend_output.non_default_styled_cells,
        state
            .last_backend_output
            .letter_cell_evidence
            .letter_cell_count,
        state.elapsed_ms
    ));
    out.push_str(&format!("message: {}\n", state.message));
    if state.studio {
        out.push_str("\nControls\n");
        if state.controls.is_empty() {
            out.push_str("- no descriptor-derived runtime controls for this recipe\n");
        }
        for control in &state.controls {
            let current_value = control
                .current_value
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string());
            let allowed_values = if control.allowed_values.is_empty() {
                "none".to_string()
            } else {
                control.allowed_values.join("|")
            };
            out.push_str(&format!(
                "- {} ({}) kind: {} control: {} target: {} signal: {} runtime: {} source: {} current: {} allowed: {} mutable: {}\n",
                control.label,
                control.id,
                control.value_kind,
                control.control_kind,
                control.target_kind,
                control.signal_id,
                control.runtime_input,
                control.source,
                current_value,
                allowed_values,
                control.runtime_mutability
            ));
        }
    }
    out.push_str(&frame_rows(&ansi_rows(state)));
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

fn ansi_rows(state: &PlayerUiState) -> Vec<String> {
    if state.last_backend_output.backend != "compositor" {
        return state.report().rows.clone();
    }
    let width = state
        .last_backend_output
        .rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0)
        .max(
            state
                .last_backend_output
                .styled_cells
                .iter()
                .map(|cell| cell.x + 1)
                .max()
                .unwrap_or(0),
        );
    let height = state.last_backend_output.rows.len().max(
        state
            .last_backend_output
            .styled_cells
            .iter()
            .map(|cell| cell.y + 1)
            .max()
            .unwrap_or(0),
    );
    let mut rows = vec![vec![" ".to_string(); width]; height];
    for (y, row) in state.last_backend_output.rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if y < height && x < width {
                rows[y][x] = ch.to_string();
            }
        }
    }
    for cell in &state.last_backend_output.styled_cells {
        if cell.y < height && cell.x < width {
            rows[cell.y][cell.x] = format!("{}{}", sgr_for_cell(cell), cell.glyph);
        }
    }
    rows.into_iter()
        .map(|row| format!("{}\x1b[0m", row.join("")))
        .collect()
}

fn sgr_for_cell(cell: &tui_vfx_player::PlayerRenderCell) -> String {
    let mut codes = Vec::new();
    if let Some((r, g, b)) = rgb_from_label(&cell.foreground) {
        codes.push(format!("38;2;{r};{g};{b}"));
    }
    if let Some((r, g, b)) = rgb_from_label(&cell.background) {
        codes.push(format!("48;2;{r};{g};{b}"));
    }
    if codes.is_empty() {
        "\x1b[0m".to_string()
    } else {
        format!("\x1b[{}m", codes.join(";"))
    }
}

fn rgb_from_label(label: &str) -> Option<(u8, u8, u8)> {
    let inner = label.strip_prefix("rgba(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(str::trim);
    let r = parts.next()?.parse::<u8>().ok()?;
    let g = parts.next()?.parse::<u8>().ok()?;
    let b = parts.next()?.parse::<u8>().ok()?;
    let a = parts.next()?.parse::<u8>().ok()?;
    if a == 0 || parts.next().is_some() {
        None
    } else {
        Some((r, g, b))
    }
}

fn help_text() -> &'static str {
    "\nhelp:\n  q       quit\n  ?       toggle help\n  space   pause/resume\n  r       reload active recipe JSON from disk and reset player session\n  m       motion-disabled stable sample\n  [ / ]   previous/next phase\n  left    sample_t - 0.05\n  right   sample_t + 0.05\n  t       fire canonical signal-backed dwell trigger\n  tick    advance elapsed time when unpaused\n"
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ui_snapshot.rs</FILE> - <DESC>Render visual player state to terminal text</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
