// <FILE>crates/tui-vfx-player/src/fnc_apply_authored_loopback_indicator.rs</FILE> - <DESC>Overlay the authored loopback preview indicator</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player parity: preserve the visible preview affordance when authored loopback values drive a sample.</WCTX>
// <CLOG>0.1.0: INIT — add compact top-right [LB] overlay when authored loopbacks fire.</CLOG>

use crate::PlayerStyledGrid;

const AUTHORED_LOOPBACK_INDICATOR: &str = "[LB]";
const INDICATOR_FOREGROUND: &str = "rgba(0,0,0,255)";
const INDICATOR_BACKGROUND: &str = "rgba(255,165,0,255)";
const INDICATOR_ROLE: &str = "AuthoredLoopbackIndicator";

/// Overlay the compact authored-loopback indicator on the top-right of the sampled rows.
///
/// The source player used an internal 4x1 recipe with the message `[LB]` anchored at
/// top-right. The v3.1 player keeps that observable affordance in its local row/styled-cell
/// model so manual preview makes loopback-driven samples visually distinct from host-driven
/// samples.
pub(crate) fn apply_authored_loopback_indicator(
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    fired_keys: &[String],
) {
    if fired_keys.is_empty() || rows.is_empty() {
        return;
    }
    let width = rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0);
    if width == 0 {
        return;
    }
    let indicator_chars = indicator_chars_for_width(width);
    let start_x = width.saturating_sub(indicator_chars.len());
    overlay_row_text(&mut rows[0], width, start_x, &indicator_chars);
    for (offset, glyph) in indicator_chars.iter().enumerate() {
        styled_grid.set_cell_glyph_and_style(
            start_x + offset,
            0,
            &glyph.to_string(),
            INDICATOR_FOREGROUND,
            INDICATOR_BACKGROUND,
            vec![],
            Some(INDICATOR_ROLE.to_string()),
        );
    }
}

fn indicator_chars_for_width(width: usize) -> Vec<char> {
    AUTHORED_LOOPBACK_INDICATOR
        .chars()
        .take(width.max(1))
        .collect()
}

fn overlay_row_text(row: &mut String, width: usize, start_x: usize, indicator_chars: &[char]) {
    let mut chars = row.chars().collect::<Vec<_>>();
    chars.resize(width, ' ');
    for (offset, glyph) in indicator_chars.iter().enumerate() {
        if let Some(cell) = chars.get_mut(start_x + offset) {
            *cell = *glyph;
        }
    }
    *row = chars.into_iter().collect();
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_authored_loopback_indicator.rs</FILE> - <DESC>authored loopback indicator</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
