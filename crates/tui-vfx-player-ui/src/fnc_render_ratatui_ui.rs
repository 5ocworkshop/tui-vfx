// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_ui.rs</FILE> - <DESC>Render ratatui player UI</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player UI: provide demo.rs-like browser plus preview panes over player frame reports.</WCTX>
// <CLOG>0.1.0: INIT — render fast-fs browser, player snapshot, status, diagnostics, and help.</CLOG>

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use tui_vfx_player::{PlayerRenderBackendOutput, PlayerRenderCell};

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
        PlayerUiFocus::Studio => "studio",
    };
    let text = vec![
        Line::from(vec![
            Span::styled(
                "tui-vfx player ",
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
            "phase={:?} sample_t={:.2} loop_t={} hash={} backend={} mode={} source={} fallback={} native_source={} backend_hash={} non_empty={} elapsed={}ms",
            app.player.phase(),
            app.player.phase_t(),
            app.player
                .loop_t()
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "none".to_string()),
            report.render_hash,
            app.player.last_backend_output.backend,
            app.player.last_backend_output.composition_mode,
            app.player.last_backend_output.source_render_mode,
            app.player.last_backend_output.fallback_used,
            app.player.last_backend_output.native_source_isolated,
            app.player.last_backend_output.backend_hash,
            report.non_empty_cells,
            app.player.elapsed_ms
        )),
    ];
    frame.render_widget(Paragraph::new(text), area);
}

fn render_body(app: &mut PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    if app.player.studio {
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(28),
                Constraint::Percentage(47),
                Constraint::Percentage(25),
            ])
            .split(area);
        render_browser(app, frame, panes[0]);
        render_preview(&app.player, frame, panes[1]);
        render_studio_controls(app, frame, panes[2]);
    } else {
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(34), Constraint::Percentage(66)])
            .split(area);
        render_browser(app, frame, panes[0]);
        render_preview(&app.player, frame, panes[1]);
    }
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
                "status: {:?}  backend={}  {active}  {motion}",
                state.report().status,
                state.last_backend_output.backend
            )),
        ])
        .block(Block::default().title(" Preview ").borders(Borders::ALL)),
        sections[0],
    );
    frame.render_widget(
        Paragraph::new(backend_preview_lines(&state.last_backend_output))
            .block(
                Block::default()
                    .title(" Player snapshot ")
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

fn render_studio_controls(app: &PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
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
                Line::from(format!(
                    "{marker} {} [{}] {} -> {}",
                    control.label, control.control_kind, control.target_kind, current_value
                ))
            })
            .collect()
    };
    frame.render_widget(
        Paragraph::new(lines)
            .block(Block::default().title(title).borders(Borders::ALL))
            .wrap(Wrap { trim: false }),
        area,
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
    let text = match app.focus {
        PlayerUiFocus::Browser => {
            "Tab preview | ↑/↓ j/k move | Enter/Right open | Left parent | R refresh | q quit | ? help"
        }
        PlayerUiFocus::Preview => {
            if app.player.studio {
                "Tab studio | Space pause | r reload | m motion | [ ] phase | ←/→ scrub | t trigger | q quit | ? help"
            } else {
                "Tab browser | Space pause | r reload | m motion | [ ] phase | ←/→ scrub | t trigger | q quit | ? help"
            }
        }
        PlayerUiFocus::Studio => {
            "Tab browser | ↑/↓ j/k select control | Enter/e mutate selected | r reload | q quit | ? help"
        }
    };
    frame.render_widget(
        Paragraph::new(text).block(Block::default().borders(Borders::TOP)),
        area,
    );
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_ui.rs</FILE> - <DESC>Render ratatui player UI</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
