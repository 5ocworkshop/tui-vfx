// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_theme.rs</FILE> - <DESC>Eichler-inspired player UI theme primitives</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player UI presentation: keep GT-design-inspired colors local without depending on gt-design.</WCTX>
// <CLOG>0.1.0: INIT — define local Eichler palette styles for status and stats surfaces.</CLOG>

use ratatui::style::{Color, Modifier, Style};

/// Local player UI palette inspired by `/usr/projects/gt-design/themes/eichler/colors-dark.json`.
#[derive(Clone, Copy)]
pub(crate) struct PlayerUiTheme {
    primary: Color,
    secondary: Color,
    tertiary: Color,
    neutral: Color,
    surface_high: Color,
}

impl PlayerUiTheme {
    /// Return the local Eichler-inspired dark palette.
    pub(crate) fn eichler() -> Self {
        Self {
            primary: Color::Rgb(80, 220, 205),
            secondary: Color::Rgb(255, 145, 125),
            tertiary: Color::Rgb(255, 225, 80),
            neutral: Color::Rgb(180, 185, 192),
            surface_high: Color::Rgb(68, 84, 98),
        }
    }

    /// Emphasized drawer title style.
    pub(crate) fn drawer_title_style(self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Subtle drawer border style.
    pub(crate) fn drawer_border_style(self) -> Style {
        Style::default().fg(self.surface_high)
    }

    /// Drawer label style for stable metric names.
    pub(crate) fn metric_label_style(self) -> Style {
        Style::default().fg(self.neutral)
    }

    /// Drawer style for healthy native/evidence statuses.
    pub(crate) fn healthy_status_style(self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Drawer style for values that deserve review attention.
    pub(crate) fn attention_status_style(self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    /// Drawer style for hashes and deterministic evidence identifiers.
    pub(crate) fn evidence_style(self) -> Style {
        Style::default().fg(self.tertiary)
    }

    /// Style a boolean status with success/error semantics.
    pub(crate) fn boolean_status_style(self, healthy_when_false: bool, value: bool) -> Style {
        if value != healthy_when_false {
            self.healthy_status_style()
        } else {
            self.attention_status_style()
        }
    }
}

// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_theme.rs</FILE> - <DESC>Eichler-inspired player UI theme primitives</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
