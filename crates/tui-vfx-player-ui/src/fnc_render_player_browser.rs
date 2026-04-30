// <FILE>crates/tui-vfx-player-ui/src/fnc_render_player_browser.rs</FILE> - <DESC>Render the themed player UI recipe browser pane</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player UI presentation: keep browser-pane rendering out of the main frame orchestrator.</WCTX>
// <CLOG>0.1.0: INIT — render the Eichler-inspired browser pane with focus styling.</CLOG>

use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{PlayerUiApp, PlayerUiFocus, cls_player_ui_theme::PlayerUiTheme};

/// Render the recipe browser pane.
pub(crate) fn render_player_browser(app: &mut PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    let theme = PlayerUiTheme::eichler();
    let title = if app.focus == PlayerUiFocus::Browser {
        " Browser * "
    } else {
        " Browser "
    };
    let items = app
        .browser
        .files()
        .iter()
        .map(|entry| {
            let style = if entry.is_dir {
                theme.directory_style()
            } else if entry.name.ends_with(".json") {
                theme.recipe_file_style()
            } else {
                theme.dim_file_style()
            };
            let prefix = if entry.is_dir { "📁 " } else { "📄 " };
            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(entry.name.clone(), style),
            ]))
        })
        .collect::<Vec<_>>();
    let list = List::new(items)
        .style(theme.panel_style())
        .block(
            Block::default()
                .title(Line::from(Span::styled(title, theme.quiet_title_style())))
                .borders(Borders::ALL)
                .border_style(if app.focus == PlayerUiFocus::Browser {
                    theme.focused_border_style()
                } else {
                    theme.panel_border_style()
                })
                .style(theme.panel_style()),
        )
        .highlight_symbol("▶ ")
        .highlight_style(theme.selected_row_style());
    frame.render_stateful_widget(list, area, &mut app.list_state);
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_player_browser.rs</FILE> - <DESC>Render the themed player UI recipe browser pane</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
