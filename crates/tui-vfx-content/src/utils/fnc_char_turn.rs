// <FILE>tui-vfx-content/src/utils/fnc_char_turn.rs</FILE> - <DESC>Unicode 180° character rotation lookup</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Introduce shared char-turn primitive for upside-down glyph rendering; consumed initially by SplitFlap.flip_preview, reusable by any transformer that wants to preview an inverted glyph (e.g. future Mirror::CharTurn axis, flip-card reveal effects)</WCTX>
// <CLOG>Initial: map A-Z and a-z to the closest Unicode-available 180°-rotated codepoints; map 0,3,6,8,9 among digits; map space plus . ! ? , common punctuation; return None for unmapped codepoints so callers can fall back gracefully (digits 1/2/4/5/7, Q, J and all non-ASCII have no visually honest turned form)</CLOG>

//! # Unicode 180° character rotation
//!
//! Terminals render cells by font lookup, not by pixel transform — so a
//! "rotated character" is only as real as a distinct Unicode codepoint
//! whose glyph resembles the input rotated 180°.
//!
//! [`char_turn`] returns `Some(turned)` for characters with a standard
//! turned counterpart, and `None` for everything else. Callers should
//! always be prepared for `None` and fall back (e.g. keep the original
//! glyph, substitute a block, or skip the effect on that character).
//!
//! ## Coverage
//!
//! * Uppercase A-Z: 24 of 26 mapped (Q and J return `None`).
//! * Lowercase a-z: 26 of 26 mapped via the IPA phonetic block.
//! * Digits: 0, 3, 6, 8, 9 mapped; 1, 2, 4, 5, 7 return `None`.
//! * Space and `. , ! ?` map to their turned forms.
//! * Rotationally symmetric characters (e.g. `H`, `N`, `O`, `S`, `X`, `Z`,
//!   `o`, `s`, `x`, `z`, `0`, `8`) return `Some(self)` — they are their
//!   own 180° rotation, and that is the honest answer.

/// Return the Unicode codepoint whose glyph looks like `c` rotated 180°,
/// or `None` if no standard turned counterpart is available.
///
/// ```
/// use tui_vfx_content::utils::char_turn;
///
/// assert_eq!(char_turn('A'), Some('Ɐ'));
/// assert_eq!(char_turn('a'), Some('ɐ'));
/// assert_eq!(char_turn('H'), Some('H')); // symmetric
/// assert_eq!(char_turn('Q'), None);
/// assert_eq!(char_turn('7'), None);
/// ```
pub fn char_turn(c: char) -> Option<char> {
    let turned = match c {
        // Uppercase A-Z (Q, J intentionally absent)
        'A' => 'Ɐ', 'B' => 'ꓭ', 'C' => 'Ɔ', 'D' => 'ꓷ', 'E' => 'Ǝ',
        'F' => 'Ⅎ', 'G' => '⅁', 'H' => 'H', 'I' => 'I', 'K' => 'ꓘ',
        'L' => '⅂', 'M' => 'W', 'N' => 'N', 'O' => 'O', 'P' => 'Ԁ',
        'R' => 'ꓤ', 'S' => 'S', 'T' => '⊥', 'U' => '∩', 'V' => 'Λ',
        'W' => 'M', 'X' => 'X', 'Y' => '⅄', 'Z' => 'Z',
        // Lowercase a-z (IPA phonetic block provides full coverage)
        'a' => 'ɐ', 'b' => 'q', 'c' => 'ɔ', 'd' => 'p', 'e' => 'ǝ',
        'f' => 'ɟ', 'g' => 'ƃ', 'h' => 'ɥ', 'i' => 'ᴉ', 'j' => 'ɾ',
        'k' => 'ʞ', 'l' => 'l', 'm' => 'ɯ', 'n' => 'u', 'o' => 'o',
        'p' => 'd', 'q' => 'b', 'r' => 'ɹ', 's' => 's', 't' => 'ʇ',
        'u' => 'n', 'v' => 'ʌ', 'w' => 'ʍ', 'x' => 'x', 'y' => 'ʎ',
        'z' => 'z',
        // Digits — 1/2/4/5/7 lack honest turned forms, return None
        '0' => '0', '3' => 'Ɛ', '6' => '9', '8' => '8', '9' => '6',
        // Space + common punctuation
        ' ' => ' ', '.' => '˙', ',' => '\'', '!' => '¡', '?' => '¿',
        _ => return None,
    };
    Some(turned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_has_coverage_except_q_and_j() {
        for c in 'A'..='Z' {
            let turned = char_turn(c);
            if c == 'Q' || c == 'J' {
                assert_eq!(turned, None, "{c} must return None (no honest turned form)");
            } else {
                assert!(turned.is_some(), "{c} must have a turned form");
            }
        }
    }

    #[test]
    fn lowercase_has_full_coverage() {
        for c in 'a'..='z' {
            assert!(char_turn(c).is_some(), "{c} must have a turned form");
        }
    }

    #[test]
    fn symmetric_uppercase_returns_self() {
        for c in ['H', 'I', 'N', 'O', 'S', 'X', 'Z'] {
            assert_eq!(char_turn(c), Some(c), "{c} is its own 180° rotation");
        }
    }

    #[test]
    fn digit_6_and_9_swap() {
        assert_eq!(char_turn('6'), Some('9'));
        assert_eq!(char_turn('9'), Some('6'));
    }

    #[test]
    fn digits_without_honest_turn_return_none() {
        for c in ['1', '2', '4', '5', '7'] {
            assert_eq!(char_turn(c), None, "digit {c} must return None");
        }
    }

    #[test]
    fn unmapped_codepoints_return_none() {
        assert_eq!(char_turn('@'), None);
        assert_eq!(char_turn('ä'), None);
        assert_eq!(char_turn('中'), None);
        assert_eq!(char_turn('🎉'), None);
    }

    #[test]
    fn mw_are_mutually_turned() {
        assert_eq!(char_turn('M'), Some('W'));
        assert_eq!(char_turn('W'), Some('M'));
    }

    #[test]
    fn db_and_pq_are_mutually_turned() {
        assert_eq!(char_turn('b'), Some('q'));
        assert_eq!(char_turn('q'), Some('b'));
        assert_eq!(char_turn('d'), Some('p'));
        assert_eq!(char_turn('p'), Some('d'));
    }
}

// <FILE>tui-vfx-content/src/utils/fnc_char_turn.rs</FILE>
// <VERS>END OF VERSION: 1.0.0</VERS>
