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
    let sample = &app.player.last_backend_output.sample;
    vec![
        format!(
            "clock={} period={}",
            sample.clock_mode,
            sample
                .clock_period_ms
                .map(|value| format!("{value:.0}ms"))
                .unwrap_or_else(|| "none".to_string())
        ),
        format!(
            "sample abs={} loop={}",
            sample_absolute_label(app),
            sample_loop_label(app)
        ),
        format!("loopback={}", loopback_status_label(app)),
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
    let sample = &app.player.last_backend_output.sample;
    vec![
        Line::from(vec![
            Span::styled("clock=", theme.metric_label_style()),
            Span::styled(sample.clock_mode.clone(), theme.healthy_status_style()),
            Span::raw(" "),
            Span::styled("period=", theme.metric_label_style()),
            Span::styled(
                sample
                    .clock_period_ms
                    .map(|value| format!("{value:.0}ms"))
                    .unwrap_or_else(|| "none".to_string()),
                theme.healthy_status_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("abs=", theme.metric_label_style()),
            Span::styled(sample_absolute_label(app), theme.healthy_status_style()),
            Span::raw(" "),
            Span::styled("loop=", theme.metric_label_style()),
            Span::styled(sample_loop_label(app), theme.healthy_status_style()),
        ]),
        Line::from(vec![
            Span::styled("loopback=", theme.metric_label_style()),
            Span::styled(loopback_status_label(app), theme.healthy_status_style()),
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

fn sample_absolute_label(app: &PlayerUiApp) -> String {
    app.player
        .last_backend_output
        .sample
        .absolute_t_ms
        .map(|value| format!("{value:.0}ms"))
        .unwrap_or_else(|| format!("ui:{}ms", app.player.elapsed_ms))
}

fn sample_loop_label(app: &PlayerUiApp) -> String {
    app.player
        .last_backend_output
        .sample
        .loop_t
        .map(|value| format!("{value:.2}"))
        .unwrap_or_else(|| "none".to_string())
}

fn loopback_status_label(app: &PlayerUiApp) -> String {
    let indicator_cells = app
        .player
        .last_backend_output
        .styled_cells
        .iter()
        .filter(|cell| cell.role.as_deref() == Some("AuthoredLoopbackIndicator"))
        .count();
    let suppressed = app
        .player
        .report()
        .warnings
        .iter()
        .filter(|warning| warning.code == "authoredLoopbackSuppressed")
        .count();
    match (indicator_cells > 0, suppressed) {
        (true, 0) => "active".to_string(),
        (true, count) => format!("active suppressed={count}"),
        (false, 0) => "none".to_string(),
        (false, count) => format!("suppressed={count}"),
    }
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
