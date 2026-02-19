// <FILE>tui-vfx-compositor/src/utils/fnc_is_powerline_separator.rs</FILE>
// <DESC>Detect powerline separator glyphs for smart filter handling</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Extract powerline detection for filter reuse</WCTX>
// <CLOG>Initial creation - covers U+E0B0-E0DF range</CLOG>

/// Check if a character is a powerline separator glyph.
///
/// Powerline separators use both fg and bg colors to create overlap effects.
/// Filters that boost brightness need special handling to avoid double-brightness
/// on these glyphs.
///
/// # Covered Ranges
///
/// - U+E0B0-U+E0BF: Powerline symbols (arrows, rounded)
/// - U+E0C0-U+E0CF: Powerline extra symbols (pixels, flames)
/// - U+E0D0-U+E0DF: Powerline extra symbols (waves, etc.)
///
/// # Example
///
/// ```
/// use tui_vfx_compositor::utils::is_powerline_separator;
///
/// assert!(is_powerline_separator('\u{E0B0}'));  // Classic arrow
/// assert!(is_powerline_separator('\u{E0B4}'));  // Bubble/rounded
/// assert!(!is_powerline_separator('A'));        // Regular text
/// ```
#[inline]
pub fn is_powerline_separator(ch: char) -> bool {
    let code = ch as u32;
    (0xE0B0..=0xE0DF).contains(&code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_classic_arrows() {
        assert!(is_powerline_separator('\u{E0B0}')); //
        assert!(is_powerline_separator('\u{E0B1}')); //
        assert!(is_powerline_separator('\u{E0B2}')); //
        assert!(is_powerline_separator('\u{E0B3}')); //
    }

    #[test]
    fn detects_bubble_rounded() {
        assert!(is_powerline_separator('\u{E0B4}')); //
        assert!(is_powerline_separator('\u{E0B5}')); //
        assert!(is_powerline_separator('\u{E0B6}')); //
        assert!(is_powerline_separator('\u{E0B7}')); //
    }

    #[test]
    fn detects_pixel_style() {
        assert!(is_powerline_separator('\u{E0C0}')); // pixel
        assert!(is_powerline_separator('\u{E0C1}'));
        assert!(is_powerline_separator('\u{E0C2}'));
        assert!(is_powerline_separator('\u{E0C3}'));
    }

    #[test]
    fn detects_flame_style() {
        assert!(is_powerline_separator('\u{E0C8}')); // flame
        assert!(is_powerline_separator('\u{E0C9}'));
        assert!(is_powerline_separator('\u{E0CA}'));
        assert!(is_powerline_separator('\u{E0CB}'));
    }

    #[test]
    fn detects_ice_style() {
        assert!(is_powerline_separator('\u{E0CC}')); // ice
        assert!(is_powerline_separator('\u{E0CD}'));
        assert!(is_powerline_separator('\u{E0CE}'));
        assert!(is_powerline_separator('\u{E0CF}'));
    }

    #[test]
    fn detects_wave_style() {
        assert!(is_powerline_separator('\u{E0D2}')); // wave
        assert!(is_powerline_separator('\u{E0D4}'));
    }

    #[test]
    fn rejects_regular_characters() {
        assert!(!is_powerline_separator('A'));
        assert!(!is_powerline_separator(' '));
        assert!(!is_powerline_separator('│'));
        assert!(!is_powerline_separator('→'));
        assert!(!is_powerline_separator('█'));
    }

    #[test]
    fn rejects_adjacent_ranges() {
        // Just before the range
        assert!(!is_powerline_separator('\u{E0AF}'));
        // Just after the range
        assert!(!is_powerline_separator('\u{E0E0}'));
    }
}

// <FILE>tui-vfx-compositor/src/utils/fnc_is_powerline_separator.rs</FILE>
// <DESC>Detect powerline separator glyphs for smart filter handling</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
