// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_ui.rs</FILE> - <DESC>Render ratatui player UI</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: provide demo.rs-like browser plus preview panes over K0 frame reports.</WCTX>
// <CLOG>0.1.0: INIT — render fast-fs browser, K0 snapshot, status, diagnostics, and help.</CLOG>

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::{
    PlayerUiApp, PlayerUiFocus, PlayerUiState, fnc_render_ratatui_help::render_ratatui_help,
};

/// Render the full interactive player UI frame.
pub fn render_ratatui_ui(app: &mut PlayerUiApp, frame: &mut Frame<'_>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(frame.area());
    render_status(app, frame, chunks[0]);
    render_body(app, frame, chunks[1]);
    render_footer(app, frame, chunks[2]);
    if app.player.show_help {
        render_ratatui_help(frame);
    }
}

fn render_status(app: &PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    let report = app.player.report();
    let focus = match app.focus {
        PlayerUiFocus::Browser => "browser",
        PlayerUiFocus::Preview => "preview",
    };
    let text = vec![
        Line::from(vec![
            Span::styled(
                "tui-vfx K1 ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "focus={focus} root={} ",
                app.browser_root.display()
            )),
        ]),
        Line::from(format!(
            "phase={:?} sample_t={:.2} loop_t={} hash={} non_empty={} elapsed={}ms",
            app.player.phase(),
            app.player.phase_t(),
            app.player
                .loop_t()
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "none".to_string()),
            report.render_hash,
            report.non_empty_cells,
            app.player.elapsed_ms
        )),
    ];
    frame.render_widget(Paragraph::new(text), area);
}

fn render_body(app: &mut PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(66)])
        .split(area);
    render_browser(app, frame, panes[0]);
    render_preview(&app.player, frame, panes[1]);
}

fn render_browser(app: &mut PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
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
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
            } else if entry.name.ends_with(".json") {
                Style::default().fg(Color::LightGreen)
            } else {
                Style::default().fg(Color::Gray)
            };
            let prefix = if entry.is_dir { "📁 " } else { "📄 " };
            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(entry.name.clone(), style),
            ]))
        })
        .collect::<Vec<_>>();
    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_symbol("▶ ")
        .highlight_style(Style::default().fg(Color::White).bg(Color::Blue));
    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_preview(state: &PlayerUiState, frame: &mut Frame<'_>, area: Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(5),
        ])
        .split(area);
    let active = if state.paused { "paused" } else { "playing" };
    let motion = if state.motion_disabled {
        "motion-disabled"
    } else {
        "motion"
    };
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(format!("recipe: {}", state.recipe_path.display())),
            Line::from(format!(
                "status: {:?}  {active}  {motion}",
                state.report().status
            )),
        ])
        .block(Block::default().title(" Preview ").borders(Borders::ALL)),
        sections[0],
    );
    frame.render_widget(
        Paragraph::new(state.report().rows.join("\n"))
            .block(
                Block::default()
                    .title(" K0 snapshot ")
                    .borders(Borders::ALL),
            )
            .wrap(Wrap { trim: false }),
        sections[1],
    );
    frame.render_widget(
        Paragraph::new(diagnostic_lines(state)).block(
            Block::default()
                .title(" Diagnostics ")
                .borders(Borders::ALL),
        ),
        sections[2],
    );
}

fn diagnostic_lines(state: &PlayerUiState) -> Vec<Line<'static>> {
    if state.report().errors.is_empty() {
        return vec![Line::from(state.message.clone())];
    }
    state
        .report()
        .errors
        .iter()
        .map(|error| {
            Line::from(format!(
                "{} at {}: {}",
                error.code, error.path, error.message
            ))
        })
        .collect()
}

fn render_footer(app: &PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    let text = match app.focus {
        PlayerUiFocus::Browser => {
            "Tab preview | ↑/↓ j/k move | Enter/Right open | Left parent | R refresh | q quit | ? help"
        }
        PlayerUiFocus::Preview => {
            "Tab browser | Space pause | r reset | m motion | [ ] phase | ←/→ scrub | t trigger | q quit | ? help"
        }
    };
    frame.render_widget(
        Paragraph::new(text).block(Block::default().borders(Borders::TOP)),
        area,
    );
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_ui.rs</FILE> - <DESC>Render ratatui player UI</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
