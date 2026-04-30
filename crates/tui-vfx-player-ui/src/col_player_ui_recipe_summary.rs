// <FILE>crates/tui-vfx-player-ui/src/col_player_ui_recipe_summary.rs</FILE> - <DESC>Recipe summary presentation helpers for the player UI</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player UI presentation: wrap long recipe descriptions in a bounded summary panel.</WCTX>
// <CLOG>0.2.0: MINOR — keep normal preview metadata in a fixed-height summary allowance.
// 0.1.0: INIT — build styled recipe-summary lines and estimate wrapped panel height.</CLOG>

use ratatui::{
    text::{Line, Span},
    widgets::Wrap,
};

use crate::{PlayerUiState, cls_player_ui_theme::PlayerUiTheme};

const COMPACT_RECIPE_SUMMARY_HEIGHT: u16 = 5;
const NORMAL_RECIPE_SUMMARY_HEIGHT: u16 = 9;
const MAXIMUM_RECIPE_SUMMARY_HEIGHT: u16 = 12;
const PLAYBACK_AND_DIAGNOSTICS_ALLOWANCE: u16 = 8;

/// Build the preview summary lines with labels that wrap inside the panel.
pub(crate) fn player_ui_recipe_summary_lines(
    state: &PlayerUiState,
    active: &str,
    motion: &str,
) -> Vec<Line<'static>> {
    let theme = PlayerUiTheme::eichler();
    let metadata = &state.recipe.metadata;
    let title = metadata.title.as_deref().unwrap_or("<untitled>");
    let description = metadata.description.as_deref().unwrap_or("<none>");
    let mut lines = vec![
        labeled_line("title", title, theme),
        labeled_line("description", description, theme),
    ];
    if let Some(expected_visual) = metadata
        .expected_visual
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        lines.push(labeled_line("expected", expected_visual, theme));
    }
    lines.push(Line::from(vec![
        Span::styled("status ", theme.label_style()),
        Span::styled(format!("{:?}", state.report().status), theme.body_style()),
        Span::styled(" · backend ", theme.muted_style()),
        Span::styled(
            state.last_backend_output.backend.to_string(),
            theme.body_style(),
        ),
        Span::styled(format!(" · {active} · {motion}"), theme.muted_style()),
    ]));
    lines
}

/// Estimate enough height for normal wrapped descriptions without starving playback.
pub(crate) fn player_ui_recipe_summary_height(
    state: &PlayerUiState,
    available_width: u16,
    available_height: u16,
) -> u16 {
    if available_height <= 12 {
        return COMPACT_RECIPE_SUMMARY_HEIGHT.min(available_height);
    }
    let inner_width = available_width.saturating_sub(2).max(24) as usize;
    let content_height = recipe_summary_content_rows(state, inner_width).saturating_add(2);
    let desired_height = if content_height <= NORMAL_RECIPE_SUMMARY_HEIGHT as usize {
        NORMAL_RECIPE_SUMMARY_HEIGHT
    } else {
        (content_height as u16).clamp(NORMAL_RECIPE_SUMMARY_HEIGHT, MAXIMUM_RECIPE_SUMMARY_HEIGHT)
    };
    desired_height.min(
        available_height
            .saturating_sub(PLAYBACK_AND_DIAGNOSTICS_ALLOWANCE)
            .max(COMPACT_RECIPE_SUMMARY_HEIGHT),
    )
}

/// Ratatui wrap policy for the summary panel.
pub(crate) fn player_ui_recipe_summary_wrap() -> Wrap {
    Wrap { trim: false }
}

fn recipe_summary_content_rows(state: &PlayerUiState, inner_width: usize) -> usize {
    let metadata = &state.recipe.metadata;
    let title = metadata.title.as_deref().unwrap_or("<untitled>");
    let description = metadata.description.as_deref().unwrap_or("<none>");
    let mut rows = wrapped_rows("title: ", title, inner_width)
        + wrapped_rows("description: ", description, inner_width)
        + 1;
    if let Some(expected_visual) = metadata
        .expected_visual
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        rows += wrapped_rows("expected: ", expected_visual, inner_width);
    }
    rows
}

fn labeled_line(label: &'static str, value: &str, theme: PlayerUiTheme) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label}: "), theme.label_style()),
        Span::styled(value.to_string(), theme.body_style()),
    ])
}

fn wrapped_rows(prefix: &str, value: &str, width: usize) -> usize {
    let mut rows = 1;
    let mut current_width = prefix.chars().count();
    for word in value.split_whitespace() {
        let word_width = word.chars().count();
        let needed = if current_width == 0 {
            word_width
        } else {
            word_width + 1
        };
        if current_width + needed > width {
            rows += 1;
            current_width = word_width;
        } else {
            current_width += needed;
        }
    }
    rows
}

// <FILE>crates/tui-vfx-player-ui/src/col_player_ui_recipe_summary.rs</FILE> - <DESC>Recipe summary presentation helpers for the player UI</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
