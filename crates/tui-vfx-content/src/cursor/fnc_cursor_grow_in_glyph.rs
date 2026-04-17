// <FILE>tui-vfx-content/src/cursor/fnc_cursor_grow_in_glyph.rs</FILE> - <DESC>Map grow-in progress to glyph+alpha</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: grow-in glyph mapping</WCTX>
// <CLOG>Initial impl</CLOG>

use super::GrowDirection;

const UP_BLOCKS: &[&str] = &["", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
const DOWN_BLOCKS: &[&str] = &["", "▔", "🮂", "🮃", "▀", "🮄", "🮅", "🮆", "█"];
const CENTER_BLOCKS: &[&str] = &["", "▄", "▆", "█"];

/// Maps a base cursor character + grow-in progress (0..1) to the glyph that
/// should render at the current frame, plus the alpha to apply.
///
/// - Block cursor `█` uses the configured direction's 1/8th-block sequence.
/// - Non-block glyphs keep their base character; only alpha animates
///   (see spec §4.2, E4).
/// - Progress is clamped to `0..=1`; out-of-range inputs return endpoint values.
pub fn fnc_cursor_grow_in_glyph(
    base: &str,
    progress: f32,
    direction: GrowDirection,
) -> (String, f32) {
    let p = progress.clamp(0.0, 1.0);

    if base != "█" {
        return (base.to_string(), p);
    }

    let seq: &[&str] = match direction {
        GrowDirection::Up => UP_BLOCKS,
        GrowDirection::Down => DOWN_BLOCKS,
        GrowDirection::Center => CENTER_BLOCKS,
    };
    let idx = ((p * (seq.len() - 1) as f32).round() as usize).min(seq.len() - 1);
    (seq[idx].to_string(), p)
}

// <FILE>tui-vfx-content/src/cursor/fnc_cursor_grow_in_glyph.rs</FILE> - <DESC>Map grow-in progress to glyph+alpha</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
