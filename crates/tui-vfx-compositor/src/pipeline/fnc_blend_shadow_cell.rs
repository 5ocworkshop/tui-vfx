// <FILE>crates/tui-vfx-compositor/src/pipeline/fnc_blend_shadow_cell.rs</FILE> - <DESC>Glyph-overlay shadow blending helper shared by compositor adapters</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Grid-first V3 snapshot composition needs the same glyph-overlay shadow blend used by the compositor pipeline without depending on the private render orchestrator internals.</WCTX>
// <CLOG>0.1.0: extract blend_shadow_cell as a public pipeline helper so non-ratatui snapshot renderers can dispatch ShadowCompositeMode correctly.</CLOG>

//! Glyph-overlay shadow cell blending.
//!
//! This helper implements the [`ShadowCompositeMode::GlyphOverlay`] branch of
//! shadow composition. It is public because adapter and preview paths that
//! composite already-rendered snapshots need the same cell-level behavior as the
//! main compositor pipeline without depending on `orc_render_pipeline` internals.
//!
//! [`ShadowCompositeMode::GlyphOverlay`]: tui_vfx_shadow::ShadowCompositeMode::GlyphOverlay

use tui_vfx_types::Cell;

/// Blend a rendered shadow cell with an existing destination cell.
///
/// Shadow renderers encode the visible shadow in their foreground/background
/// channels. For half-block shadow glyphs, the foreground represents the
/// shadowed portion, so semi-transparent foreground is blended over the
/// destination background rather than the destination foreground. The shadow
/// glyph and modifiers are preserved.
#[inline]
pub fn blend_shadow_cell(shadow_cell: &Cell, dest_cell: &Cell) -> Cell {
    let blended_bg = if shadow_cell.bg.a < 255 && shadow_cell.bg.a > 0 {
        shadow_cell.bg.blend_over(dest_cell.bg)
    } else if shadow_cell.bg.a == 0 {
        dest_cell.bg
    } else {
        shadow_cell.bg
    };

    let blended_fg = if shadow_cell.fg.a < 255 && shadow_cell.fg.a > 0 {
        shadow_cell.fg.blend_over(dest_cell.bg)
    } else if shadow_cell.fg.a == 0 {
        dest_cell.bg
    } else {
        shadow_cell.fg
    };

    Cell::styled(shadow_cell.ch, blended_fg, blended_bg, shadow_cell.mods)
        .with_mod_alpha(shadow_cell.mod_alpha)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Color;

    #[test]
    fn transparent_shadow_channels_preserve_destination_background() {
        let shadow = Cell::new('▐')
            .with_fg(Color::TRANSPARENT)
            .with_bg(Color::TRANSPARENT);
        let dest = Cell::new('A')
            .with_fg(Color::rgb(220, 220, 220))
            .with_bg(Color::rgb(20, 30, 40));

        let blended = blend_shadow_cell(&shadow, &dest);

        assert_eq!(blended.ch, '▐');
        assert_eq!(blended.fg, dest.bg);
        assert_eq!(blended.bg, dest.bg);
    }
}

// <FILE>crates/tui-vfx-compositor/src/pipeline/fnc_blend_shadow_cell.rs</FILE> - <DESC>Glyph-overlay shadow blending helper shared by compositor adapters</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
