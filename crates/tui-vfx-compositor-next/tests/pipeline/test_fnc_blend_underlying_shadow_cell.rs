// <FILE>crates/tui-vfx-compositor-next/tests/pipeline/test_fnc_blend_underlying_shadow_cell.rs</FILE> - <DESC>Unit tests for destination-preserving alpha shadow blending</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>BlendUnderlying should preserve terminal glyph content while giving dark surfaces a visibly alpha-blended shadow.</WCTX>
// <CLOG>0.1.0: prove BlendUnderlying preserves glyph/fg and blends only background.</CLOG>

use tui_vfx_compositor_next::pipeline::blend_underlying_shadow_cell;
use tui_vfx_types::{Cell, Color};

#[test]
fn blend_underlying_preserves_destination_glyph_and_fg() {
    let shadow = Cell::new(' ')
        .with_fg(Color::TRANSPARENT)
        .with_bg(Color::new(0, 0, 0, 160));
    let dest = Cell::new('▁')
        .with_fg(Color::rgb(48, 220, 220))
        .with_bg(Color::rgb(8, 12, 20));

    let result = blend_underlying_shadow_cell(&shadow, &dest);

    assert_eq!(result.ch, '▁');
    assert_eq!(result.fg, Color::rgb(48, 220, 220));
    assert_eq!(result.bg, Color::rgb(3, 4, 7));
}

#[test]
fn blend_underlying_falls_back_to_shadow_fg_coverage() {
    let shadow = Cell::new('▐')
        .with_fg(Color::new(0, 0, 0, 128))
        .with_bg(Color::TRANSPARENT);
    let dest = Cell::new('A')
        .with_fg(Color::rgb(200, 210, 220))
        .with_bg(Color::rgb(20, 30, 40));

    let result = blend_underlying_shadow_cell(&shadow, &dest);

    assert_eq!(result.ch, 'A');
    assert_eq!(result.fg, dest.fg);
    assert_eq!(result.bg, Color::rgb(10, 15, 20));
}

// <FILE>crates/tui-vfx-compositor-next/tests/pipeline/test_fnc_blend_underlying_shadow_cell.rs</FILE> - <DESC>Unit tests for destination-preserving alpha shadow blending</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
