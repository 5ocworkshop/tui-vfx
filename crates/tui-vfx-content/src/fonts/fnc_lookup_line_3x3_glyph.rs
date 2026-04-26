// <FILE>crates/tui-vfx-content/src/fonts/fnc_lookup_line_3x3_glyph.rs</FILE> - <DESC>Public char-to-glyph lookup over the canonical Line 3x3 table</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 5 of mechanical circular content cycles plan: AI authors and recipe-tooling code need to synthesize multi-line glyph strings without reaching into the table internals.</WCTX>
// <CLOG>0.1.0: introduce lookup_line_3x3_glyph and render_line_3x3_text helpers, ASCII-uppercased fallback, transparent for absent glyphs.</CLOG>

//! Public char-to-glyph helpers over the canonical Line 3x3 table.
//!
//! Authors and tooling that need to synthesize Odometer / SplitFlap
//! recipe strings programmatically use these helpers rather than
//! reaching into [`super::col_line_3x3_heavy_glyphs::line_3x3_heavy_glyphs`].

use super::col_line_3x3_heavy_glyphs::line_3x3_heavy_glyphs;

/// Look up the 3-row glyph for `ch` in the canonical Line 3x3 table.
///
/// Performs an ASCII-uppercase fold so `'a'..='z'` map to the same
/// glyph as `'A'..='Z'`. Returns `None` for characters not in the
/// table; callers usually fall back to the space glyph (`Some([rows; 3])`
/// where every row is `"   "`) which is at index 0 of the table.
pub fn lookup_line_3x3_glyph(ch: char) -> Option<[&'static str; 3]> {
    let key = ch.to_ascii_uppercase();
    line_3x3_heavy_glyphs()
        .iter()
        .find(|(c, _)| *c == key || (*c == ch && key == ch))
        .map(|(_, rows)| *rows)
}

/// Render a single-line ASCII string into three lines of Line 3x3
/// glyphs, joined with newlines.
///
/// Characters not in the table render as the space glyph (three blank
/// rows of three cells each). Newlines and other structural characters
/// in `text` are passed through unchanged inside their respective rows
/// so callers that need multi-line output handle them at the caller
/// boundary.
///
/// # Examples
///
/// ```
/// use tui_vfx_content::fonts::render_line_3x3_text;
///
/// let glyph_rows = render_line_3x3_text("12");
/// // Result is three rows joined by newlines, where each row has the
/// // glyph rows for "1" then for "2" concatenated cell-by-cell.
/// assert_eq!(glyph_rows.lines().count(), 3);
/// for line in glyph_rows.lines() {
///     assert_eq!(line.chars().count(), 6); // 2 chars * 3 cells each
/// }
/// ```
pub fn render_line_3x3_text(text: &str) -> String {
    let glyphs: Vec<[&'static str; 3]> = text
        .chars()
        .map(|c| lookup_line_3x3_glyph(c).unwrap_or(["   ", "   ", "   "]))
        .collect();
    let mut out = String::with_capacity(glyphs.len() * 3 * 3 + 2);
    for row in 0..3 {
        if row > 0 {
            out.push('\n');
        }
        for glyph in &glyphs {
            out.push_str(glyph[row]);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_zero_is_box() {
        let glyph = lookup_line_3x3_glyph('0').unwrap();
        assert_eq!(glyph[0], "┏━┓");
        assert_eq!(glyph[1], "┃ ┃");
        assert_eq!(glyph[2], "┗━┛");
    }

    #[test]
    fn lowercase_letter_resolves_to_uppercase_glyph() {
        let upper = lookup_line_3x3_glyph('A').unwrap();
        let lower = lookup_line_3x3_glyph('a').unwrap();
        assert_eq!(upper, lower);
    }

    #[test]
    fn unknown_glyph_returns_none() {
        assert!(lookup_line_3x3_glyph('Ω').is_none());
        assert!(lookup_line_3x3_glyph('≈').is_none());
    }

    #[test]
    fn render_two_digit_number_is_six_cells_wide() {
        let rendered = render_line_3x3_text("42");
        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(lines.len(), 3);
        for line in &lines {
            assert_eq!(line.chars().count(), 6);
        }
    }

    #[test]
    fn render_unknown_char_uses_blank_glyph() {
        let rendered = render_line_3x3_text("Ω");
        for line in rendered.lines() {
            assert_eq!(line, "   ");
        }
    }

    #[test]
    fn render_empty_string_is_empty() {
        let rendered = render_line_3x3_text("");
        // Two newlines separating three empty rows.
        assert_eq!(rendered, "\n\n");
    }
}

// <FILE>crates/tui-vfx-content/src/fonts/fnc_lookup_line_3x3_glyph.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
