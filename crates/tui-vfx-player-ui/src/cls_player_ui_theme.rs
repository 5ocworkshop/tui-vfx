// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_theme.rs</FILE> - <DESC>Eichler-inspired player UI theme primitives</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player UI presentation: keep GT-design-inspired colors and surface hierarchy local without depending on gt-design.</WCTX>
// <CLOG>0.2.0: MINOR — expose canvas, panel, border, text, and focus styles for the full player/studio shell.</CLOG>

use ratatui::style::{Color, Modifier, Style};

/// Local player UI palette inspired by `/usr/projects/gt-design/themes/eichler/colors-dark.json`.
#[derive(Clone, Copy)]
pub(crate) struct PlayerUiTheme {
    primary: Color,
    secondary: Color,
    tertiary: Color,
    neutral: Color,
    on_surface: Color,
    on_surface_variant: Color,
    outline: Color,
    canvas: Color,
    surface_low: Color,
    surface: Color,
    surface_high: Color,
    surface_highest: Color,
}

impl PlayerUiTheme {
    /// Return the local Eichler-inspired dark palette.
    pub(crate) fn eichler() -> Self {
        Self {
            primary: Color::Rgb(80, 220, 205),
            secondary: Color::Rgb(255, 145, 125),
            tertiary: Color::Rgb(255, 225, 80),
            neutral: Color::Rgb(180, 185, 192),
            on_surface: Color::Rgb(215, 220, 228),
            on_surface_variant: Color::Rgb(190, 198, 208),
            outline: Color::Rgb(118, 132, 145),
            canvas: Color::Rgb(16, 22, 28),
            surface_low: Color::Rgb(32, 42, 50),
            surface: Color::Rgb(42, 54, 64),
            surface_high: Color::Rgb(68, 84, 98),
            surface_highest: Color::Rgb(68, 84, 98),
        }
    }

    /// Whole-canvas background style.
    pub(crate) fn canvas_style(self) -> Style {
        Style::default().bg(self.canvas).fg(self.on_surface)
    }

    /// Status/footer chrome style.
    pub(crate) fn chrome_style(self) -> Style {
        Style::default().bg(self.canvas).fg(self.on_surface_variant)
    }

    /// Primary panel background style.
    pub(crate) fn panel_style(self) -> Style {
        Style::default().bg(self.surface_low).fg(self.on_surface)
    }

    /// Emphasized preview/studio surface style.
    pub(crate) fn elevated_panel_style(self) -> Style {
        Style::default().bg(self.surface).fg(self.on_surface)
    }

    /// Highest emphasis panel style for dense evidence drawers.
    pub(crate) fn drawer_panel_style(self) -> Style {
        Style::default()
            .bg(self.surface_highest)
            .fg(self.on_surface)
    }

    /// Standard low-contrast border style.
    pub(crate) fn panel_border_style(self) -> Style {
        Style::default().fg(self.outline).bg(self.surface_low)
    }

    /// Border style for focused panes.
    pub(crate) fn focused_border_style(self) -> Style {
        Style::default()
            .fg(self.primary)
            .bg(self.surface)
            .add_modifier(Modifier::BOLD)
    }

    /// Preview/studio border style.
    pub(crate) fn elevated_border_style(self) -> Style {
        Style::default().fg(self.surface_high).bg(self.surface)
    }

    /// High-emphasis title style.
    pub(crate) fn title_style(self) -> Style {
        Style::default()
            .fg(self.primary)
            .bg(self.surface)
            .add_modifier(Modifier::BOLD)
    }

    /// Secondary title style used by browser and quiet panes.
    pub(crate) fn quiet_title_style(self) -> Style {
        Style::default()
            .fg(self.tertiary)
            .bg(self.surface_low)
            .add_modifier(Modifier::BOLD)
    }

    /// Body text style.
    pub(crate) fn body_style(self) -> Style {
        Style::default().fg(self.on_surface).bg(self.surface)
    }

    /// Muted/supporting text style.
    pub(crate) fn muted_style(self) -> Style {
        Style::default()
            .fg(self.on_surface_variant)
            .bg(self.surface)
    }

    /// Accent label style.
    pub(crate) fn label_style(self) -> Style {
        Style::default()
            .fg(self.tertiary)
            .bg(self.surface)
            .add_modifier(Modifier::BOLD)
    }

    /// Browser directory style.
    pub(crate) fn directory_style(self) -> Style {
        Style::default()
            .fg(self.primary)
            .bg(self.surface_low)
            .add_modifier(Modifier::BOLD)
    }

    /// Browser recipe-file style.
    pub(crate) fn recipe_file_style(self) -> Style {
        Style::default().fg(self.on_surface).bg(self.surface_low)
    }

    /// Browser non-recipe file style.
    pub(crate) fn dim_file_style(self) -> Style {
        Style::default().fg(self.neutral).bg(self.surface_low)
    }

    /// Browser selected row style.
    pub(crate) fn selected_row_style(self) -> Style {
        Style::default()
            .fg(Color::Rgb(0, 48, 45))
            .bg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub(crate) fn drawer_title_style(self) -> Style {
        Style::default()
            .fg(self.primary)
            .bg(self.surface_highest)
            .add_modifier(Modifier::BOLD)
    }

    /// Subtle drawer border style.
    pub(crate) fn drawer_border_style(self) -> Style {
        Style::default().fg(self.outline).bg(self.surface_highest)
    }

    /// Drawer label style for stable metric names.
    pub(crate) fn metric_label_style(self) -> Style {
        Style::default().fg(self.neutral).bg(self.surface_highest)
    }

    /// Drawer style for healthy native/evidence statuses.
    pub(crate) fn healthy_status_style(self) -> Style {
        Style::default()
            .fg(self.primary)
            .bg(self.surface_highest)
            .add_modifier(Modifier::BOLD)
    }

    /// Drawer style for values that deserve review attention.
    pub(crate) fn attention_status_style(self) -> Style {
        Style::default()
            .fg(self.secondary)
            .bg(self.surface_highest)
            .add_modifier(Modifier::BOLD)
    }

    /// Drawer style for hashes and deterministic evidence identifiers.
    pub(crate) fn evidence_style(self) -> Style {
        Style::default().fg(self.tertiary).bg(self.surface_highest)
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
// <VERS>END OF VERSION: 0.2.0</VERS>
