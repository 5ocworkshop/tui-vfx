// <FILE>crates/tui-vfx-player-ui/src/col_player_ui_stats_drawer.rs</FILE> - <DESC>Stats drawer presentation helpers for the player UI</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>Player UI: keep rapidly changing playback/backend evidence out of the top status line.</WCTX>
// <CLOG>0.3.1: PATCH — move phase/sample timing out of the stats drawer.</CLOG>

use ratatui::text::{Line, Span};

use crate::{PlayerUiApp, cls_player_ui_theme::PlayerUiTheme};

const PLAYER_UI_HASH_DECIMAL_WIDTH: usize = 20;
const PLAYER_UI_STATS_DRAWER_BORDER_WIDTH: u16 = 2;
const PLAYER_UI_STATS_DRAWER_WIDTH_PADDING: u16 = 4;

/// Build the rapidly changing stats text shown in the right-side drawer.
pub(crate) fn player_ui_stats_drawer_text_lines(app: &PlayerUiApp) -> Vec<String> {
    let report = app.player.report();
    vec![
        format!(
            "loop_t={} elapsed={}ms",
            app.player
                .loop_t()
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "none".to_string()),
            app.player.elapsed_ms
        ),
        format!("hash={}", format_player_ui_hash(report.render_hash)),
        format!("non_empty={}", report.non_empty_cells),
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
            "native_source={}",
            app.player.last_backend_output.native_source_isolated
        ),
        format!(
            "backend_hash={}",
            format_player_ui_hash(app.player.last_backend_output.backend_hash)
        ),
    ]
}

/// Build color-coded stats lines shown in the right-side drawer.
pub(crate) fn player_ui_stats_drawer_lines(app: &PlayerUiApp) -> Vec<Line<'static>> {
    let theme = PlayerUiTheme::eichler();
    let report = app.player.report();
    vec![
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
            Span::styled(
                format_player_ui_hash(report.render_hash),
                theme.evidence_style(),
            ),
        ]),
        Line::from(vec![
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
        ]),
        Line::from(vec![
            Span::styled("backend_hash=", theme.evidence_style()),
            Span::styled(
                format_player_ui_hash(app.player.last_backend_output.backend_hash),
                theme.evidence_style(),
            ),
        ]),
    ]
}

/// Return the drawer width: widest stats line plus borders and jitter padding.
pub(crate) fn player_ui_stats_drawer_width(app: &PlayerUiApp) -> u16 {
    player_ui_stats_drawer_text_lines(app)
        .iter()
        .map(|line| line.chars().count() as u16)
        .max()
        .unwrap_or(0)
        .saturating_add(PLAYER_UI_STATS_DRAWER_BORDER_WIDTH + PLAYER_UI_STATS_DRAWER_WIDTH_PADDING)
}

fn format_player_ui_hash(value: u64) -> String {
    format!("{value:>PLAYER_UI_HASH_DECIMAL_WIDTH$}")
}

// <FILE>crates/tui-vfx-player-ui/src/col_player_ui_stats_drawer.rs</FILE> - <DESC>Stats drawer presentation helpers for the player UI</DESC>
// <VERS>END OF VERSION: 0.3.1</VERS>
