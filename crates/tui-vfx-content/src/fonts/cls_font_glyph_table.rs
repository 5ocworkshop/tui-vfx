// <FILE>crates/tui-vfx-content/src/fonts/cls_font_glyph_table.rs</FILE> - <DESC>Type-erased glyph table that fonts produce per character; extension point for future RSF / non-3x3 fonts</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Slice 6.2 of mechanical circular content cycles plan: pluggable glyph-table contract consumed by Mechanical cycle face expansion. Today the only inhabitant is the embedded Line 3x3 face (Intention 36); future rsf-backed fonts plug in here without changing the consumer.</WCTX>
// <CLOG>0.1.0: introduce FontGlyphTable enum (Line3x3 today), lookup_glyph_rows / cell_width / cell_height accessors, render_text helper that joins per-character glyph rows into a multi-line string. Inline tests for digit lookup, lowercase fold, unknown-char blank fallback, render width assertion.</CLOG>

//! Type-erased glyph table for content effects that paint multi-cell
//! glyphs (Odometer 3x3 digit drums, future SplitFlap multi-cell tiles).
//!
//! Today the only inhabitant is the embedded Line 3x3 table (Intention 36).
//! Future faces — RSF-backed runtime fonts, larger or variable-cell glyph
//! sets — extend this enum or, when the rule of three lands, refactor
//! to a trait.

use super::col_line_3x3_heavy_glyphs::line_3x3_heavy_glyphs;

/// Pluggable glyph table consumed by content-effect face expansion.
///
/// Variants are the registered fonts. The enum is intentionally closed
/// today — `Line3x3` is the only built-in, and future RSF-backed fonts
/// will join as additional variants until enough siblings exist to
/// justify a trait refactor (rule of three per Intention 23).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontGlyphTable {
    /// The canonical Line 3x3 heavy glyph table per Intention 36.
    Line3x3,
}

impl FontGlyphTable {
    /// Look up the glyph rows for `ch`. Returns `None` when the
    /// character is absent from the table; consumers usually fall
    /// back to a blank glyph of the table's cell shape.
    ///
    /// Letter-case: implementations may apply ASCII-uppercase folding
    /// per their convention. Line 3x3 folds 'a'..='z' to 'A'..='Z'.
    pub fn lookup_glyph_rows(&self, ch: char) -> Option<Vec<String>> {
        match self {
            Self::Line3x3 => {
                let key = ch.to_ascii_uppercase();
                line_3x3_heavy_glyphs()
                    .iter()
                    .find(|(c, _)| *c == key || *c == ch)
                    .map(|(_, rows)| rows.iter().map(|r| (*r).to_string()).collect())
            }
        }
    }

    /// Cell width in character cells per glyph row.
    pub fn cell_width(&self) -> u16 {
        match self {
            Self::Line3x3 => 3,
        }
    }

    /// Cell height in character cells per glyph (number of rows).
    pub fn cell_height(&self) -> u16 {
        match self {
            Self::Line3x3 => 3,
        }
    }

    /// Render `text` (one logical line) as `cell_height` rows of
    /// concatenated glyph cells, joined by newlines. Characters absent
    /// from the table render as blank cells.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_vfx_content::fonts::FontGlyphTable;
    ///
    /// let table = FontGlyphTable::Line3x3;
    /// let rendered = table.render_text("12");
    /// assert_eq!(rendered.lines().count(), 3);
    /// for line in rendered.lines() {
    ///     // Two characters at 3 cells each = 6 cells per row.
    ///     assert_eq!(line.chars().count(), 6);
    /// }
    /// ```
    pub fn render_text(&self, text: &str) -> String {
        let cell_h = self.cell_height() as usize;
        let cell_w = self.cell_width() as usize;
        let blank: Vec<String> = (0..cell_h).map(|_| " ".repeat(cell_w)).collect();
        let glyphs: Vec<Vec<String>> = text
            .chars()
            .map(|c| self.lookup_glyph_rows(c).unwrap_or_else(|| blank.clone()))
            .collect();
        let mut out = String::with_capacity(text.chars().count() * cell_w * cell_h + cell_h);
        for row in 0..cell_h {
            if row > 0 {
                out.push('\n');
            }
            for glyph in &glyphs {
                if let Some(line) = glyph.get(row) {
                    out.push_str(line);
                } else {
                    out.push_str(&" ".repeat(cell_w));
                }
            }
        }
        out
    }

    /// Render a single glyph (one character) as a `\n`-joined block.
    /// Convenience for callers that need per-character rendering for
    /// per-tile cycle face expansion.
    pub fn render_glyph(&self, ch: char) -> String {
        let cell_w = self.cell_width() as usize;
        let cell_h = self.cell_height() as usize;
        let rows = self
            .lookup_glyph_rows(ch)
            .unwrap_or_else(|| (0..cell_h).map(|_| " ".repeat(cell_w)).collect());
        rows.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_3x3_cell_dimensions_are_three_by_three() {
        let table = FontGlyphTable::Line3x3;
        assert_eq!(table.cell_width(), 3);
        assert_eq!(table.cell_height(), 3);
    }

    #[test]
    fn line_3x3_zero_resolves_to_box() {
        let glyph = FontGlyphTable::Line3x3.lookup_glyph_rows('0').unwrap();
        assert_eq!(glyph.len(), 3);
        assert_eq!(glyph[0], "┏━┓");
        assert_eq!(glyph[1], "┃ ┃");
        assert_eq!(glyph[2], "┗━┛");
    }

    #[test]
    fn line_3x3_lowercase_letter_folds_to_uppercase() {
        let upper = FontGlyphTable::Line3x3.lookup_glyph_rows('A');
        let lower = FontGlyphTable::Line3x3.lookup_glyph_rows('a');
        assert_eq!(upper, lower);
    }

    #[test]
    fn line_3x3_unknown_char_returns_none() {
        assert!(FontGlyphTable::Line3x3.lookup_glyph_rows('Ω').is_none());
    }

    #[test]
    fn render_text_two_digits_is_six_cells_wide() {
        let rendered = FontGlyphTable::Line3x3.render_text("42");
        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(lines.len(), 3);
        for line in &lines {
            assert_eq!(line.chars().count(), 6);
        }
    }

    #[test]
    fn render_text_unknown_char_renders_blank_glyph() {
        let rendered = FontGlyphTable::Line3x3.render_text("Ω");
        for line in rendered.lines() {
            assert_eq!(line, "   ");
        }
    }

    #[test]
    fn render_glyph_returns_three_newline_joined_rows() {
        let rendered = FontGlyphTable::Line3x3.render_glyph('0');
        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "┏━┓");
        assert_eq!(lines[1], "┃ ┃");
        assert_eq!(lines[2], "┗━┛");
    }

    #[test]
    fn render_glyph_unknown_char_returns_blank() {
        let rendered = FontGlyphTable::Line3x3.render_glyph('Ω');
        for line in rendered.lines() {
            assert_eq!(line, "   ");
        }
    }
}

// <FILE>crates/tui-vfx-content/src/fonts/cls_font_glyph_table.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
