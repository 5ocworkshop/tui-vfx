// <FILE>crates/tui-vfx-types/src/color_inert.rs</FILE> - <DESC>Detection utility for color-inert glyphs (emoji, PUA, nerd font icons)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Color-inert glyph detection for shadow grading replacement</WCTX>
// <CLOG>Initial creation with is_color_inert_glyph and inline tests</CLOG>

//! Color-inert glyph detection.
//!
//! Some Unicode characters — emoji and nerd font icons in Private Use Area
//! ranges — are rendered by terminal emulators as color bitmap images that
//! **ignore ANSI foreground color attributes**. When shadow grading dims or
//! desaturates these glyphs, the color change is applied but invisible: the
//! glyph stays full brightness while the surrounding background is dimmed.
//!
//! [`is_color_inert_glyph`] detects these characters so callers can replace
//! them with a neutral placeholder during grading.

/// Returns `true` if `ch` is a glyph that typically ignores ANSI fg color
/// in terminal emulators (emoji, PUA/nerd-font icons, variation selectors,
/// ZWJ).
///
/// This intentionally casts a wide net: it is better to replace a colorable
/// glyph than to ship a bright artifact in a dimmed shadow region.
///
/// # Detected ranges
///
/// | Range | What |
/// |-------|------|
/// | U+E000–U+F8FF | BMP Private Use Area (nerd fonts, custom icons) |
/// | U+F0000–U+FFFFD | Supplementary PUA-A |
/// | U+100000–U+10FFFD | Supplementary PUA-B |
/// | U+1F300–U+1FAFF | SMP emoji blocks (pictographs, emoticons, transport, symbols) |
/// | U+1F1E6–U+1F1FF | Regional indicator symbols (flags) |
/// | U+2600–U+27BF | Miscellaneous Symbols + Dingbats (BMP emoji) |
/// | U+2B00–U+2BFF | Misc Symbols and Arrows (some emoji) |
/// | U+FE00–U+FE0F | Variation selectors |
/// | U+200D | Zero Width Joiner |
#[inline]
pub fn is_color_inert_glyph(ch: char) -> bool {
    matches!(ch as u32,
        // BMP Private Use Area (nerd fonts, custom icons)
        0xE000..=0xF8FF |
        // Supplementary PUA-A
        0xF0000..=0xFFFFD |
        // Supplementary PUA-B
        0x100000..=0x10FFFD |
        // SMP emoji blocks (pictographs, emoticons, transport, symbols)
        0x1F300..=0x1FAFF |
        // Regional indicator symbols (flags)
        0x1F1E6..=0x1F1FF |
        // Miscellaneous Symbols + Dingbats
        0x2600..=0x27BF |
        // Misc Symbols and Arrows
        0x2B00..=0x2BFF |
        // Variation selectors
        0xFE00..=0xFE0F |
        // Zero Width Joiner
        0x200D
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_detected() {
        // Common emoji from various SMP blocks
        assert!(is_color_inert_glyph('🌍')); // U+1F30D Earth Globe
        assert!(is_color_inert_glyph('😀')); // U+1F600 Grinning Face
        assert!(is_color_inert_glyph('🚀')); // U+1F680 Rocket
        assert!(is_color_inert_glyph('🧪')); // U+1F9EA Test Tube
    }

    #[test]
    fn pua_detected() {
        // BMP PUA range (nerd font icons live here)
        assert!(is_color_inert_glyph('\u{E000}'));
        assert!(is_color_inert_glyph('\u{F000}')); // Common nerd font range
        assert!(is_color_inert_glyph('\u{F8FF}'));
    }

    #[test]
    fn nerd_font_icons_detected() {
        // Typical nerd font codepoints
        assert!(is_color_inert_glyph('\u{F121}')); // nf-fa-code
        assert!(is_color_inert_glyph('\u{E725}')); // nf-dev-rust
        assert!(is_color_inert_glyph('\u{F015}')); // nf-fa-home
    }

    #[test]
    fn bmp_emoji_detected() {
        // Miscellaneous Symbols range
        assert!(is_color_inert_glyph('☀')); // U+2600 Black Sun
        assert!(is_color_inert_glyph('⚡')); // U+26A1 Lightning
        // Dingbats range
        assert!(is_color_inert_glyph('✂')); // U+2702 Scissors
        assert!(is_color_inert_glyph('❤')); // U+2764 Heavy Heart
    }

    #[test]
    fn regional_indicators_detected() {
        assert!(is_color_inert_glyph('\u{1F1E6}')); // Regional Indicator A
        assert!(is_color_inert_glyph('\u{1F1FF}')); // Regional Indicator Z
    }

    #[test]
    fn variation_selectors_detected() {
        assert!(is_color_inert_glyph('\u{FE00}'));
        assert!(is_color_inert_glyph('\u{FE0F}')); // VS16 (emoji presentation)
    }

    #[test]
    fn zwj_detected() {
        assert!(is_color_inert_glyph('\u{200D}')); // Zero Width Joiner
    }

    // Negative cases: these should NOT be flagged as color-inert

    #[test]
    fn ascii_not_detected() {
        assert!(!is_color_inert_glyph('A'));
        assert!(!is_color_inert_glyph('z'));
        assert!(!is_color_inert_glyph(' '));
        assert!(!is_color_inert_glyph('~'));
    }

    #[test]
    fn box_drawing_not_detected() {
        assert!(!is_color_inert_glyph('─')); // U+2500
        assert!(!is_color_inert_glyph('│')); // U+2502
        assert!(!is_color_inert_glyph('┌')); // U+250C
        assert!(!is_color_inert_glyph('╔')); // U+2554
    }

    #[test]
    fn block_elements_not_detected() {
        assert!(!is_color_inert_glyph('█')); // U+2588 Full Block
        assert!(!is_color_inert_glyph('▄')); // U+2584 Lower Half Block
        assert!(!is_color_inert_glyph('▀')); // U+2580 Upper Half Block
        assert!(!is_color_inert_glyph('░')); // U+2591 Light Shade
    }

    #[test]
    fn braille_not_detected() {
        assert!(!is_color_inert_glyph('⠀')); // U+2800 Blank Braille
        assert!(!is_color_inert_glyph('⣿')); // U+28FF Full Braille
    }

    #[test]
    fn cjk_not_detected() {
        assert!(!is_color_inert_glyph('中')); // U+4E2D
        assert!(!is_color_inert_glyph('日')); // U+65E5
    }

    #[test]
    fn middle_dot_replacement_not_detected() {
        // The replacement char itself must not be flagged
        assert!(!is_color_inert_glyph('\u{00B7}')); // middle dot ·
    }
}

// <FILE>crates/tui-vfx-types/src/color_inert.rs</FILE> - <DESC>Detection utility for color-inert glyphs (emoji, PUA, nerd font icons)</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
