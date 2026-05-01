// <FILE>crates/tui-vfx-compositor-next/src/pipeline/fnc_blend_underlying_shadow_cell.rs</FILE> - <DESC>Destination-preserving alpha shadow blending helper</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Transparent shadows need a mode that preserves destination glyphs like GradeUnderlying while using alpha-aware color blending like GlyphOverlay.</WCTX>
// <CLOG>0.1.0: add blend_underlying_shadow_cell for glyph-preserving alpha-aware shadow composition.</CLOG>

//! Destination-preserving alpha shadow blending.
//!
//! This helper implements the [`ShadowCompositeMode::BlendUnderlying`] branch
//! of shadow composition. It preserves the destination glyph, foreground,
//! modifiers, and modifier alpha, but alpha-blends the rendered shadow color
//! over the destination background. Use it when a recipe wants a translucent
//! shadow that still lets the underlying cell content remain readable.
//!
//! [`ShadowCompositeMode::BlendUnderlying`]: tui_vfx_shadow::ShadowCompositeMode::BlendUnderlying

use tui_vfx_types::{Cell, Color};

/// Blend rendered shadow color onto the destination background while preserving
/// destination glyph and foreground content.
///
/// Shadow renderers may encode coverage in either foreground or background
/// channels depending on style. Background coverage is preferred because solid
/// transparent shadows use background alpha; foreground coverage is used as a
/// fallback for glyph-based shadow styles.
#[inline]
pub fn blend_underlying_shadow_cell(shadow_cell: &Cell, dest_cell: &Cell) -> Cell {
    let shadow_color = shadow_blend_color(shadow_cell);
    if shadow_color.a == 0 {
        return *dest_cell;
    }

    Cell {
        ch: dest_cell.ch,
        fg: dest_cell.fg,
        bg: shadow_color.blend_over(dest_cell.bg),
        mods: dest_cell.mods,
        mod_alpha: dest_cell.mod_alpha,
    }
}

#[inline]
fn shadow_blend_color(shadow_cell: &Cell) -> Color {
    if shadow_cell.bg.a > 0 {
        shadow_cell.bg
    } else if shadow_cell.fg.a > 0 {
        shadow_cell.fg
    } else {
        Color::TRANSPARENT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Color;

    #[test]
    fn blends_shadow_background_but_preserves_destination_content() {
        let shadow = Cell::new(' ')
            .with_fg(Color::TRANSPARENT)
            .with_bg(Color::new(0, 0, 0, 128));
        let dest = Cell::new('▁')
            .with_fg(Color::rgb(48, 220, 220))
            .with_bg(Color::rgb(8, 12, 20));

        let blended = blend_underlying_shadow_cell(&shadow, &dest);

        assert_eq!(blended.ch, dest.ch);
        assert_eq!(blended.fg, dest.fg);
        assert_eq!(blended.mods, dest.mods);
        assert_eq!(blended.mod_alpha, dest.mod_alpha);
        assert_eq!(blended.bg, Color::rgb(4, 6, 10));
    }

    #[test]
    fn transparent_shadow_preserves_destination_cell() {
        let shadow = Cell::new(' ').with_bg(Color::TRANSPARENT);
        let dest = Cell::new('A')
            .with_fg(Color::rgb(200, 210, 220))
            .with_bg(Color::rgb(20, 30, 40));

        assert_eq!(blend_underlying_shadow_cell(&shadow, &dest), dest);
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/pipeline/fnc_blend_underlying_shadow_cell.rs</FILE> - <DESC>Destination-preserving alpha shadow blending helper</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
