// <FILE>crates/tui-vfx-player-ui/src/fnc_render_stats_drawer.rs</FILE> - <DESC>Render the player UI stats drawer</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player UI: keep rapidly changing playback/backend evidence in a compact side drawer.</WCTX>
// <CLOG>0.2.0: MINOR — render Eichler-inspired styled stats drawer lines.</CLOG>

use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{
    PlayerUiApp, cls_player_ui_theme::PlayerUiTheme,
    col_player_ui_stats_drawer::player_ui_stats_drawer_lines,
};

/// Render the right-side stats drawer.
pub(crate) fn render_stats_drawer(app: &PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    let theme = PlayerUiTheme::eichler();
    let lines = player_ui_stats_drawer_lines(app);
    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title(Line::from(Span::styled(
                        " Stats ",
                        theme.drawer_title_style(),
                    )))
                    .borders(Borders::ALL)
                    .border_style(theme.drawer_border_style()),
            )
            .wrap(Wrap { trim: false }),
        area,
    );
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_stats_drawer.rs</FILE> - <DESC>Render the player UI stats drawer</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
