// <FILE>crates/tui-vfx-player-ui/src/fnc_render_ratatui_help.rs</FILE> - <DESC>Render ratatui player UI help overlay</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>New kernel Phase K1: keep help overlay separate so the main renderer stays small.</WCTX>
// <CLOG>0.3.1: PATCH — document studio slider arrow adjustment.
// 0.3.0: MINOR — document studio mouse click/drag controls.
// 0.2.0: PATCH — list global playback shortcuts, including the black-canvas toggle, so the overlay matches live key handling.
// 0.1.0: INIT — render centered K1 keybinding help modal.</CLOG>

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
            "Global: q quit, ? help, Tab switch focus, s studio, b black bg".into(),
            "Global playback: Space pause/resume, r reload, [/] phase, m motion-disabled, t trigger".into(),
            "Browser: j/k or arrows move, Enter/Right open, Left parent".into(),
            "Browser: R refresh from disk, . hidden files".into(),
            "Preview: Left/Right scrub sample_t".into(),
            "Studio: click selects; drag/click slider adjusts; Left/Right changes slider by 1".into(),
            "Help overlay: Esc closes; Space, r, b, [ ], m, t still execute".into(),
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
// <VERS>END OF VERSION: 0.3.1</VERS>
