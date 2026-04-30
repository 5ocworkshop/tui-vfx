// <FILE>crates/tui-vfx-player-ui/src/col_player_ui_stats_drawer.rs</FILE> - <DESC>Stats drawer presentation helpers for the player UI</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player UI: keep rapidly changing playback/backend evidence out of the top status line.</WCTX>
// <CLOG>0.2.0: MINOR — add Eichler-inspired color-coded stats drawer lines.</CLOG>

use ratatui::text::{Line, Span};

use crate::{PlayerUiApp, cls_player_ui_theme::PlayerUiTheme};

/// Build the rapidly changing stats text shown in the right-side drawer.
pub(crate) fn player_ui_stats_drawer_text_lines(app: &PlayerUiApp) -> Vec<String> {
    let report = app.player.report();
    vec![
        format!(
            "phase={:?} sample_t={:.2}",
            app.player.phase(),
            app.player.phase_t()
        ),
        format!(
            "loop_t={} elapsed={}ms",
            app.player
                .loop_t()
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "none".to_string()),
            app.player.elapsed_ms
        ),
        format!(
            "hash={} non_empty={}",
            report.render_hash, report.non_empty_cells
        ),
        format!(
            "backend={} mode={}",
            app.player.last_backend_output.backend, app.player.last_backend_output.composition_mode
        ),
        format!(
            "source={} fallback={}",
            app.player.last_backend_output.source_render_mode,
            app.player.last_backend_output.fallback_used
        ),
        format!(
            "native_source={} backend_hash={}",
            app.player.last_backend_output.native_source_isolated,
            app.player.last_backend_output.backend_hash
        ),
    ]
}

/// Build color-coded stats lines shown in the right-side drawer.
pub(crate) fn player_ui_stats_drawer_lines(app: &PlayerUiApp) -> Vec<Line<'static>> {
    let theme = PlayerUiTheme::eichler();
    let report = app.player.report();
    vec![
        Line::from(vec![
            Span::styled("phase=", theme.metric_label_style()),
            Span::styled(
                format!("{:?}", app.player.phase()),
                theme.healthy_status_style(),
            ),
            Span::raw(" "),
            Span::styled("sample_t=", theme.metric_label_style()),
            Span::styled(
                format!("{:.2}", app.player.phase_t()),
                theme.healthy_status_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("loop_t=", theme.metric_label_style()),
            Span::styled(
                app.player
                    .loop_t()
                    .map(|value| format!("{value:.2}"))
                    .unwrap_or_else(|| "none".to_string()),
                theme.healthy_status_style(),
            ),
            Span::raw(" "),
            Span::styled("elapsed=", theme.metric_label_style()),
            Span::styled(
                format!("{}ms", app.player.elapsed_ms),
                theme.metric_label_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("hash=", theme.evidence_style()),
            Span::styled(report.render_hash.to_string(), theme.evidence_style()),
            Span::raw(" "),
            Span::styled("non_empty=", theme.metric_label_style()),
            Span::styled(
                report.non_empty_cells.to_string(),
                theme.healthy_status_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("backend=", theme.metric_label_style()),
            Span::styled(
                app.player.last_backend_output.backend.to_string(),
                theme.healthy_status_style(),
            ),
            Span::raw(" "),
            Span::styled("mode=", theme.metric_label_style()),
            Span::styled(
                app.player.last_backend_output.composition_mode.to_string(),
                theme.healthy_status_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("source=", theme.metric_label_style()),
            Span::styled(
                app.player
                    .last_backend_output
                    .source_render_mode
                    .to_string(),
                theme.healthy_status_style(),
            ),
            Span::raw(" "),
            Span::styled("fallback=", theme.metric_label_style()),
            Span::styled(
                app.player.last_backend_output.fallback_used.to_string(),
                theme.boolean_status_style(true, app.player.last_backend_output.fallback_used),
            ),
        ]),
        Line::from(vec![
            Span::styled("native_source=", theme.metric_label_style()),
            Span::styled(
                app.player
                    .last_backend_output
                    .native_source_isolated
                    .to_string(),
                theme.boolean_status_style(
                    false,
                    app.player.last_backend_output.native_source_isolated,
                ),
            ),
            Span::raw(" "),
            Span::styled("backend_hash=", theme.evidence_style()),
            Span::styled(
                app.player.last_backend_output.backend_hash.to_string(),
                theme.evidence_style(),
            ),
        ]),
    ]
}

/// Return the exact drawer width: widest stats line plus left and right borders.
pub(crate) fn player_ui_stats_drawer_width(app: &PlayerUiApp) -> u16 {
    player_ui_stats_drawer_text_lines(app)
        .iter()
        .map(|line| line.chars().count() as u16)
        .max()
        .unwrap_or(0)
        .saturating_add(2)
}

// <FILE>crates/tui-vfx-player-ui/src/col_player_ui_stats_drawer.rs</FILE> - <DESC>Stats drawer presentation helpers for the player UI</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
