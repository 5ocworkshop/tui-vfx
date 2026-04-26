// <FILE>crates/tui-vfx-content/src/fonts/col_line_3x3_heavy_glyphs.rs</FILE> - <DESC>Canonical heavy-weight Line 3x3 glyph table — the project default and runtime-fallback font (Intention 36)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 5 of mechanical circular content cycles plan: embed the Line 3x3 glyph table so Odometer 3x3 digit recipes and the runtime font fallback have one canonical home in tui-vfx-content.</WCTX>
// <CLOG>0.1.0: mirror gtd-components Line 3x3 heavy table byte-for-byte (54 glyphs covering space, digits, A-Z, common punctuation, currency, operators).</CLOG>

//! Heavy 3x3 line glyph table.
//!
//! Each entry is `(char, [row0, row1, row2])` where every row is exactly
//! three character cells. The table is intentionally fixed-cell: silent
//! rescaling would hide font-asset misses. Recipes that lose their
//! declared font and fall back to this table render at native 3x3
//! resolution, padded into whatever tile rectangle the recipe authored
//! around.
//!
//! This is the single canonical home for the Line 3x3 face inside
//! tui-vfx (Intention 36). gt-design's `gtd-components` carries an
//! identical mirror so the family agrees on which strokes draw which
//! glyph; if the two ever drift, this is the source of truth and
//! gt-design re-imports.

/// Returns the canonical heavy-weight Line 3x3 glyph table.
///
/// The slice is sorted in a deliberate order (space first, then
/// digits, then operators that overlap with digit rows like `+ - ^ x :`,
/// then A-Z, then punctuation/currency/symbols). Authors should look up
/// glyphs by character via [`super::lookup_line_3x3_glyph`] rather than
/// by index; the order is part of the table's identity but not its
/// contract.
pub fn line_3x3_heavy_glyphs() -> &'static [(char, [&'static str; 3])] {
    &[
        (' ', ["   ", "   ", "   "]),
        ('0', ["┏━┓", "┃ ┃", "┗━┛"]),
        ('1', ["╺┓ ", " ┃ ", "╺┻╸"]),
        ('2', ["╺━┓", "┏━┛", "┗━╸"]),
        ('3', ["╺━┓", " ━┫", "╺━┛"]),
        ('4', ["╻ ╻", "┗━┫", "  ╹"]),
        ('5', ["┏━╸", "┗━┓", "╺━┛"]),
        ('6', ["┏━╸", "┣━┓", "┗━┛"]),
        ('7', ["╺━┓", "  ┃", "  ╹"]),
        ('8', ["┏━┓", "┣━┫", "┗━┛"]),
        ('9', ["┏━┓", "┗━┫", "╺━┛"]),
        ('+', ["   ", "╺╋╸", "   "]),
        ('-', ["   ", "╺━╸", "   "]),
        ('^', [" ^ ", "   ", "   "]),
        ('x', ["   ", " × ", "   "]),
        (':', ["   ", " : ", "   "]),
        ('A', ["┏━┓", "┣━┫", "╹ ╹"]),
        ('B', ["┏━┓", "┣━┫", "┗━┛"]),
        ('C', ["┏━┓", "┃  ", "┗━┛"]),
        ('D', ["┏━┓", "┃ ┃", "┗━┛"]),
        ('E', ["┏━╸", "┣━ ", "┗━╸"]),
        ('F', ["┏━╸", "┣━ ", "╹  "]),
        ('G', ["┏━┓", "┃┏┓", "┗━┛"]),
        ('H', ["╻ ╻", "┣━┫", "╹ ╹"]),
        ('I', ["╺┳╸", " ┃ ", "╺┻╸"]),
        ('J', ["╺━┓", "  ┃", "┗━┛"]),
        ('K', ["╻┏┛", "┣┫ ", "╹┗┓"]),
        ('L', ["╻  ", "┃  ", "┗━╸"]),
        ('M', ["╻╻╻", "┃┃┃", "╹ ╹"]),
        ('N', ["╻ ╻", "┃╲┃", "╹ ╹"]),
        ('O', ["┏━┓", "┃ ┃", "┗━┛"]),
        ('P', ["┏━┓", "┣━┛", "╹  "]),
        ('Q', ["┏━┓", "┃ ┃", "┗┳┛"]),
        ('R', ["┏━┓", "┣┳┛", "╹┗┓"]),
        ('S', ["┏━╸", "┗━┓", "╺━┛"]),
        ('T', ["╺┳╸", " ┃ ", " ╹ "]),
        ('U', ["╻ ╻", "┃ ┃", "┗━┛"]),
        ('V', ["╻ ╻", "┃ ┃", "┗ ┛"]),
        ('W', ["╻ ╻", "┃┃┃", "┗┻┛"]),
        ('X', ["╲ ╱", " ╳ ", "╱ ╲"]),
        ('Y', ["╻ ╻", "┗┳┛", " ╹ "]),
        ('Z', ["╺━┓", " ╱ ", "┗━╸"]),
        ('$', ["┏╫┓", "┗╫┓", "┗╫┛"]),
        ('£', ["┏━┓", "╋━ ", "┻━╸"]),
        ('€', ["┏━┓", "╋━ ", "┗━┛"]),
        ('(', ["┏╸ ", "┃  ", "┗╸ "]),
        (')', [" ╺┓", "  ┃", " ╺┛"]),
        ('.', ["   ", "   ", " • "]),
        (',', ["   ", "   ", " , "]),
        ('!', [" ┃ ", " ┃ ", " • "]),
        ('/', ["  ╱", " ╱ ", "╱  "]),
        ('?', ["╺━┓", " ┏┛", " • "]),
        ('=', ["   ", "━━━", "━━━"]),
        ('*', ["╲╱ ", " × ", "╱╲ "]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_is_blank_three_by_three() {
        let table = line_3x3_heavy_glyphs();
        let (_, rows) = table.iter().find(|(c, _)| *c == ' ').unwrap();
        for row in rows.iter() {
            assert_eq!(row.chars().count(), 3);
            assert!(row.chars().all(|c| c == ' '));
        }
    }

    #[test]
    fn every_glyph_row_is_exactly_three_chars_wide() {
        for (ch, rows) in line_3x3_heavy_glyphs() {
            for (i, row) in rows.iter().enumerate() {
                assert_eq!(
                    row.chars().count(),
                    3,
                    "glyph {ch:?} row {i} is not 3 chars: {row:?}",
                );
            }
        }
    }

    #[test]
    fn covers_decimal_digits_and_full_uppercase_alphabet() {
        let chars: std::collections::HashSet<char> =
            line_3x3_heavy_glyphs().iter().map(|(c, _)| *c).collect();
        for c in '0'..='9' {
            assert!(chars.contains(&c), "missing digit {c}");
        }
        for c in 'A'..='Z' {
            assert!(chars.contains(&c), "missing letter {c}");
        }
    }
}

// <FILE>crates/tui-vfx-content/src/fonts/col_line_3x3_heavy_glyphs.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
