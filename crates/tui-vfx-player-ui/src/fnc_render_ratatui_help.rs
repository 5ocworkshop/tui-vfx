// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_help.rs</FILE> - <DESC>Render ratatui player UI help overlay</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: keep help overlay separate so the main renderer stays small.</WCTX>
// <CLOG>0.1.0: INIT — render centered K1 keybinding help modal.</CLOG>

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Render centered help for the interactive player UI.
pub(crate) fn render_ratatui_help(frame: &mut Frame<'_>) {
    let area = centered_rect(frame.area(), 70, 70);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(vec![
            "Global: q quit, ? help, Tab switch focus".into(),
            "Browser: j/k or arrows move, Enter/Right open, Left parent".into(),
            "Browser: R refresh from disk, . hidden files".into(),
            "Preview: Space pause/resume, r reset, m motion-disabled".into(),
            "Preview: [/] phase, Left/Right scrub sample_t, t trigger dwell signal".into(),
        ])
        .block(Block::default().title(" Help ").borders(Borders::ALL))
        .wrap(Wrap { trim: true }),
        area,
    );
}

fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_help.rs</FILE> - <DESC>Render ratatui player UI help overlay</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
