// <FILE>crates/tui-vfx-player-ui/src/fnc_render_studio_controls.rs</FILE> - <DESC>Render themed descriptor-derived studio controls</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player UI presentation: keep studio control rendering out of the main frame orchestrator.</WCTX>
// <CLOG>0.1.0: INIT — render studio controls with focused borders and readable value roles.</CLOG>

use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{PlayerUiApp, PlayerUiFocus, cls_player_ui_theme::PlayerUiTheme};

/// Render descriptor-derived studio controls.
pub(crate) fn render_studio_controls(app: &PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    let theme = PlayerUiTheme::eichler();
    let title = if app.focus == PlayerUiFocus::Studio {
        " Studio controls * "
    } else {
        " Studio controls "
    };
    let lines = if app.player.controls.is_empty() {
        vec![Line::from("no descriptor-derived controls")]
    } else {
        app.player
            .controls
            .iter()
            .enumerate()
            .map(|(index, control)| {
                let marker = if index == app.studio_control_index {
                    "▶"
                } else {
                    " "
                };
                let current_value = control
                    .current_value
                    .as_ref()
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "none".to_string());
                Line::from(vec![
                    Span::styled(marker.to_string(), theme.label_style()),
                    Span::styled(format!(" {} ", control.label), theme.body_style()),
                    Span::styled(format!("[{}] ", control.control_kind), theme.muted_style()),
                    Span::styled(control.target_kind.clone(), theme.label_style()),
                    Span::styled(format!(" -> {current_value}"), theme.body_style()),
                ])
            })
            .collect()
    };
    frame.render_widget(
        Paragraph::new(lines)
            .style(theme.elevated_panel_style())
            .block(
                Block::default()
                    .title(Line::from(Span::styled(title, theme.title_style())))
                    .borders(Borders::ALL)
                    .border_style(if app.focus == PlayerUiFocus::Studio {
                        theme.focused_border_style()
                    } else {
                        theme.elevated_border_style()
                    })
                    .style(theme.elevated_panel_style()),
            )
            .wrap(Wrap { trim: false }),
        area,
    );
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_studio_controls.rs</FILE> - <DESC>Render themed descriptor-derived studio controls</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
