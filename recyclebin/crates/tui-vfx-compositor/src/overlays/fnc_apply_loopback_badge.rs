// <FILE>recyclebin/crates/tui-vfx-compositor/src/overlays/fnc_apply_loopback_badge.rs</FILE> - <DESC>Hardcoded-cell-painter L3 badge function — recycled in favor of recipe-driven badge per Intention 39</DESC>
// <VERS>VERSION: 0.1.0 (recycled)</VERS>
// <WCTX>Loopback Phase L3 first attempt (recycled 2026-04-26). Painted a 4-cell ASCII `[LB]` or 5-cell Nerd Font ` LB ` directly into the top-right of the host grid with hardcoded orange BG / black FG. The user's correction flagged the loss of recipe-author flexibility: every styling decision (orange shade, fade approach, glyph choice, padding) was locked in engine code instead of an editable JSON file. See Intention 39 for the resulting principle.</WCTX>
// <CLOG>0.1.0 (recycled): preserved as the artefact surrounding Intention 39's emergence. The hardcoded BADGE_FULL_BG / BADGE_FADE_BG / BADGE_FG / NF_WARNING constants are exactly what the principle warns against locking into code.</CLOG>

//! Pure overlay function that paints the LB visibility badge into a grid.
//!
//! ABANDONED: superseded by recipe-based architecture per Intention 39.

use tui_vfx_types::{Cell, Color, Grid, OwnedGrid};

use super::cls_loopback_badge_state::LoopbackBadgeState;
use super::enum_loopback_badge_style::LoopbackBadgeStyle;

const BADGE_FULL_BG: Color = Color { r: 255, g: 165, b: 0, a: 255 };
const BADGE_FADE_BG: Color = Color { r: 180, g: 120, b: 0, a: 255 };
const BADGE_FG: Color = Color { r: 0, g: 0, b: 0, a: 255 };
const NF_WARNING: char = '\u{F071}';

pub fn apply_loopback_badge(
    grid: &mut OwnedGrid,
    state: &LoopbackBadgeState,
    style: LoopbackBadgeStyle,
) {
    if !state.is_active() {
        return;
    }
    if grid.height() == 0 {
        return;
    }
    let cells = badge_cells(style.resolve());
    if grid.width() < cells.len() {
        return;
    }
    let start_x = grid.width() - cells.len();
    for (offset, cell) in cells.iter().enumerate() {
        grid.set(start_x + offset, 0, cell.clone());
    }
}

fn badge_cells(style: LoopbackBadgeStyle) -> Vec<Cell> {
    match style {
        LoopbackBadgeStyle::Auto => badge_cells(LoopbackBadgeStyle::Auto.resolve()),
        LoopbackBadgeStyle::Ascii => vec![
            badge_cell('[', BADGE_FADE_BG),
            badge_cell('L', BADGE_FULL_BG),
            badge_cell('B', BADGE_FULL_BG),
            badge_cell(']', BADGE_FADE_BG),
        ],
        LoopbackBadgeStyle::NerdFont => vec![
            badge_cell(' ', BADGE_FADE_BG),
            badge_cell(NF_WARNING, BADGE_FULL_BG),
            badge_cell('L', BADGE_FULL_BG),
            badge_cell('B', BADGE_FULL_BG),
            badge_cell(' ', BADGE_FADE_BG),
        ],
    }
}

fn badge_cell(ch: char, bg: Color) -> Cell {
    Cell {
        ch,
        fg: BADGE_FG,
        bg,
        mods: Default::default(),
        mod_alpha: None,
    }
}

// Tests omitted in recyclebin copy — full TDD suite was authored alongside;
// see git history if you need to reconstruct it. Resurrection of this file
// would require Intention 39's principle to be rescinded, which is unlikely.

// <FILE>recyclebin/crates/tui-vfx-compositor/src/overlays/fnc_apply_loopback_badge.rs</FILE>
// <VERS>END OF VERSION: 0.1.0 (recycled)</VERS>
