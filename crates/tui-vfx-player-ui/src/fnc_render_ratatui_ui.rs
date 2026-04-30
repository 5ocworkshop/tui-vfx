// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_ui.rs</FILE> - <DESC>Render ratatui player UI</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Player UI: provide themed browser, preview, studio, and quiet stats surfaces over player frame reports.</WCTX>
// <CLOG>0.3.0: MINOR — apply Eichler-inspired surfaces and reserve wrapped recipe-summary space.</CLOG>

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use tui_vfx_player::{PlayerRenderBackendOutput, PlayerRenderCell};

use crate::{
    PlayerUiApp, PlayerUiFocus, PlayerUiState,
    cls_player_ui_theme::PlayerUiTheme,
    col_player_ui_recipe_summary::{
        player_ui_recipe_summary_height, player_ui_recipe_summary_lines,
        player_ui_recipe_summary_wrap,
    },
    col_player_ui_stats_drawer::player_ui_stats_drawer_width,
    fnc_render_player_browser::render_player_browser,
    fnc_render_ratatui_help::render_ratatui_help,
    fnc_render_stats_drawer::render_stats_drawer,
    fnc_render_studio_controls::render_studio_controls,
};

/// Render the full interactive player UI frame.
pub fn render_ratatui_ui(app: &mut PlayerUiApp, frame: &mut Frame<'_>) {
    let theme = PlayerUiTheme::eichler();
    frame.render_widget(Block::default().style(theme.canvas_style()), frame.area());
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
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
    let theme = PlayerUiTheme::eichler();
    let focus = match app.focus {
        PlayerUiFocus::Browser => "browser",
        PlayerUiFocus::Preview => "preview",
        PlayerUiFocus::Studio => "studio",
    };
    let drawer_hint = if app.stats_drawer_open {
        "Ctrl+Right close stats"
    } else {
        "Ctrl+Left open stats"
    };
    let text = Line::from(vec![
        Span::styled(
            "tui-vfx player ",
            theme
                .chrome_style()
                .fg(Color::Rgb(80, 220, 205))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(
                "focus={focus} root={} | {drawer_hint}",
                app.browser_root.display()
            ),
            theme.chrome_style(),
        ),
    ]);
    frame.render_widget(Paragraph::new(text).style(theme.chrome_style()), area);
}

fn render_body(app: &mut PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    if app.stats_drawer_open {
        let drawer_width = player_ui_stats_drawer_width(app);
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(drawer_width)])
            .split(area);
        render_main_body(app, frame, panes[0]);
        render_stats_drawer(app, frame, panes[1]);
    } else {
        render_main_body(app, frame, area);
    }
}

fn render_main_body(app: &mut PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    if app.player.studio {
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(28),
                Constraint::Percentage(47),
                Constraint::Percentage(25),
            ])
            .split(area);
        render_player_browser(app, frame, panes[0]);
        render_preview(&app.player, frame, panes[1]);
        render_studio_controls(app, frame, panes[2]);
    } else {
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(34), Constraint::Percentage(66)])
            .split(area);
        render_player_browser(app, frame, panes[0]);
        render_preview(&app.player, frame, panes[1]);
    }
}

fn render_preview(state: &PlayerUiState, frame: &mut Frame<'_>, area: Rect) {
    let theme = PlayerUiTheme::eichler();
    let active = if state.paused { "paused" } else { "playing" };
    let motion = if state.motion_disabled {
        "motion-disabled"
    } else {
        "motion"
    };
    let summary_height = player_ui_recipe_summary_height(state, area.width, area.height);
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(summary_height),
            Constraint::Min(5),
            Constraint::Length(5),
        ])
        .split(area);
    frame.render_widget(
        Paragraph::new(player_ui_recipe_summary_lines(state, active, motion))
            .style(theme.elevated_panel_style())
            .block(
                Block::default()
                    .title(Line::from(Span::styled(" Recipe ", theme.title_style())))
                    .borders(Borders::ALL)
                    .border_style(theme.elevated_border_style())
                    .style(theme.elevated_panel_style()),
            )
            .wrap(player_ui_recipe_summary_wrap()),
        sections[0],
    );
    frame.render_widget(
        Paragraph::new(backend_preview_lines(&state.last_backend_output))
            .block(
                Block::default()
                    .title(Line::from(Span::styled(
                        " Player snapshot ",
                        theme.title_style(),
                    )))
                    .borders(Borders::ALL)
                    .border_style(theme.elevated_border_style())
                    .style(theme.elevated_panel_style()),
            )
            .style(theme.elevated_panel_style())
            .wrap(Wrap { trim: false }),
        sections[1],
    );
    frame.render_widget(
        Paragraph::new(diagnostic_lines(state))
            .block(
                Block::default()
                    .title(Line::from(Span::styled(
                        " Diagnostics ",
                        theme.title_style(),
                    )))
                    .borders(Borders::ALL)
                    .border_style(theme.elevated_border_style())
                    .style(theme.elevated_panel_style()),
            )
            .style(theme.elevated_panel_style()),
        sections[2],
    );
}

fn backend_preview_lines(output: &PlayerRenderBackendOutput) -> Vec<Line<'static>> {
    let width = output
        .rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0)
        .max(
            output
                .styled_cells
                .iter()
                .map(|cell| cell.x + 1)
                .max()
                .unwrap_or(0),
        );
    let height = output.rows.len().max(
        output
            .styled_cells
            .iter()
            .map(|cell| cell.y + 1)
            .max()
            .unwrap_or(0),
    );
    if width == 0 || height == 0 {
        return vec![Line::from("")];
    }
    let mut cells = vec![preview_cell(' '); width * height];
    for (y, row) in output.rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if x < width && y < height {
                cells[y * width + x].glyph = ch;
            }
        }
    }
    for styled_cell in &output.styled_cells {
        if styled_cell.x < width && styled_cell.y < height {
            cells[styled_cell.y * width + styled_cell.x] = PreviewCell::from(styled_cell);
        }
    }
    (0..height)
        .map(|y| {
            let spans = (0..width)
                .map(|x| {
                    let cell = &cells[y * width + x];
                    Span::styled(cell.glyph.to_string(), cell.style)
                })
                .collect::<Vec<_>>();
            Line::from(spans)
        })
        .collect()
}

#[derive(Clone, Copy)]
struct PreviewCell {
    glyph: char,
    style: Style,
}

impl From<&PlayerRenderCell> for PreviewCell {
    fn from(cell: &PlayerRenderCell) -> Self {
        Self {
            glyph: cell.glyph.chars().next().unwrap_or(' '),
            style: style_from_player_cell(cell),
        }
    }
}

fn preview_cell(glyph: char) -> PreviewCell {
    PreviewCell {
        glyph,
        style: Style::default(),
    }
}

fn style_from_player_cell(cell: &PlayerRenderCell) -> Style {
    let mut style = Style::default();
    if let Some(color) = ratatui_color_from_label(&cell.foreground) {
        style = style.fg(color);
    }
    if let Some(color) = ratatui_color_from_label(&cell.background) {
        style = style.bg(color);
    }
    for modifier in &cell.modifiers {
        style = match modifier.as_str() {
            "bold" => style.add_modifier(Modifier::BOLD),
            "dim" => style.add_modifier(Modifier::DIM),
            "italic" => style.add_modifier(Modifier::ITALIC),
            "underline" => style.add_modifier(Modifier::UNDERLINED),
            "reverse" => style.add_modifier(Modifier::REVERSED),
            "strikethrough" => style.add_modifier(Modifier::CROSSED_OUT),
            _ => style,
        };
    }
    style
}

fn ratatui_color_from_label(label: &str) -> Option<Color> {
    let inner = label.strip_prefix("rgba(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(str::trim);
    let r = parts.next()?.parse::<u8>().ok()?;
    let g = parts.next()?.parse::<u8>().ok()?;
    let b = parts.next()?.parse::<u8>().ok()?;
    let a = parts.next()?.parse::<u8>().ok()?;
    if a == 0 || parts.next().is_some() {
        None
    } else {
        Some(Color::Rgb(r, g, b))
    }
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
    let theme = PlayerUiTheme::eichler();
    let text = match app.focus {
        PlayerUiFocus::Browser => {
            "Tab preview | ↑/↓ j/k move | Enter/Right open | Left parent | Ctrl+←/→ stats | q quit | ? help"
        }
        PlayerUiFocus::Preview => {
            if app.player.studio {
                "Tab studio | Space pause | r reload | [ ] phase | ←/→ scrub | Ctrl+←/→ stats | q quit | ? help"
            } else {
                "Tab browser | Space pause | r reload | [ ] phase | ←/→ scrub | Ctrl+←/→ stats | q quit | ? help"
            }
        }
        PlayerUiFocus::Studio => {
            "Tab browser | ↑/↓ j/k select control | Enter/e mutate selected | Ctrl+←/→ stats | q quit | ? help"
        }
    };
    frame.render_widget(
        Paragraph::new(text).style(theme.chrome_style()).block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(theme.panel_border_style())
                .style(theme.chrome_style()),
        ),
        area,
    );
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_ui.rs</FILE> - <DESC>Render ratatui player UI</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
