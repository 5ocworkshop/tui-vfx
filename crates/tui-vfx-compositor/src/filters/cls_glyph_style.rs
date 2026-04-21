// <FILE>tui-vfx-compositor/src/filters/cls_glyph_style.rs</FILE>
// <DESC>GlyphStyle filter — per-glyph-category style overrides via char-membership rules</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Add bg_alternate field to GlyphStyleRule for coordinate-checkerboard bg modulation. Naturally bounded by char-matching — cells outside the rule's char set (padding spaces, border, unmatched chars) are unaffected, so applying alternation to a "letter cells" rule produces a card-grid edge perception only where cards actually live.</WCTX>
// <CLOG>MINOR: GlyphStyleRule grows pub bg_alternate: Option<Color>. When set and (x+y) parity is odd, the cell uses bg_alternate instead of bg; when unset, behavior is identical to v1.0.0. Three new inline tests covering parity application, fall-through, and bound-by-char-match.</CLOG>

//! GlyphStyle filter
//!
//! For each cell, find the first rule whose char set contains `cell.ch`
//! and apply that rule's fg/bg/modifier overrides. Unmatched cells pass
//! through unchanged.
//!
//! # Use cases
//!
//! - **SplitFlap boards**: color hinge blocks (`█▀▔—▁▄`) differently
//!   from letter glyphs and from turned-preview glyphs, producing the
//!   three-tone "board / card / letter" Solari aesthetic.
//! - **GlyphCascade / Scramble / Redact**: any transformer that emits
//!   more than one category of glyph can have each category styled
//!   independently via a single filter.
//!
//! # First-match wins
//!
//! Rules are evaluated in declaration order. Order them from most-specific
//! to least-specific. If no rule matches, the cell is unchanged.

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Single rule in a [`GlyphStyle`] filter: a char-membership set plus
/// optional fg/bg overrides.
///
/// `chars` is a `Vec<char>` for straightforward `contains()` lookup; for
/// the small rule sets typical in Solari-board recipes (<40 chars per
/// rule, <8 rules per filter) linear scan is faster than a hash set and
/// keeps deserialization trivial (`String -> Vec<char>`).
///
/// `bg_alternate` enables a subtle coordinate-checkerboard bg modulation:
/// when set, cells with odd `(x + y)` parity use `bg_alternate` while
/// even-parity cells use `bg`. The alternation is bounded by the rule's
/// char set — cells outside that set (padding spaces, border, unmatched
/// chars) are unaffected — so applying it to a "letter cells" rule
/// produces a card-grid edge perception only where cards actually live.
#[derive(Debug, Clone)]
pub struct GlyphStyleRule {
    /// Characters this rule matches.
    pub chars: Vec<char>,
    /// Override foreground color. `None` leaves fg unchanged.
    pub fg: Option<Color>,
    /// Override background color. `None` leaves bg unchanged.
    pub bg: Option<Color>,
    /// Alternate background for `(x + y) % 2 == 1` cells. `None` disables
    /// alternation (behaves like v1.0.0). When set, even-parity cells use
    /// `bg` (or pass through if `bg` is also `None`), odd-parity cells
    /// use `bg_alternate`. Pick a slightly different shade of the same
    /// color family for a subtle card-edge perception.
    pub bg_alternate: Option<Color>,
}

/// Filter that styles cells based on glyph-content membership.
pub struct GlyphStyle {
    pub rules: Vec<GlyphStyleRule>,
}

impl GlyphStyle {
    /// Create a new GlyphStyle filter from a set of rules (first-match wins).
    pub fn new(rules: Vec<GlyphStyleRule>) -> Self {
        Self { rules }
    }
}

impl Filter for GlyphStyle {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, _w: u16, _h: u16, _t: f64) {
        for rule in &self.rules {
            if rule.chars.contains(&cell.ch) {
                if let Some(fg) = rule.fg {
                    cell.fg = fg;
                }
                // Coordinate-checkerboard bg: even parity uses bg, odd
                // parity uses bg_alternate (or falls back to bg if
                // bg_alternate is unset). Both being None leaves bg
                // untouched — full v1.0.0 compatibility.
                let parity_bg = if (x + y) % 2 == 1 {
                    rule.bg_alternate.or(rule.bg)
                } else {
                    rule.bg
                };
                if let Some(bg) = parity_bg {
                    cell.bg = bg;
                }
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    fn cell_with(ch: char) -> Cell {
        Cell::styled(
            ch,
            Color::rgb(100, 100, 100),
            Color::rgb(20, 20, 20),
            Modifiers::NONE,
        )
    }

    #[test]
    fn no_rules_leaves_cell_unchanged() {
        let filter = GlyphStyle::new(vec![]);
        let mut cell = cell_with('H');
        let before = cell;
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell, before);
    }

    #[test]
    fn unmatched_char_leaves_cell_unchanged() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['█', '▀'],
            fg: Some(Color::rgb(210, 210, 210)),
            bg: None,
            bg_alternate: None,
        }]);
        let mut cell = cell_with('X');
        let before = cell;
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell, before);
    }

    #[test]
    fn matched_char_gets_fg_override() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['█', '▀'],
            fg: Some(Color::rgb(180, 180, 180)),
            bg: None,
            bg_alternate: None,
        }]);
        let mut cell = cell_with('█');
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(180, 180, 180));
        assert_eq!(cell.bg, Color::rgb(20, 20, 20), "bg unchanged");
    }

    #[test]
    fn matched_char_gets_bg_override() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['▀', '▄'],
            fg: None,
            bg: Some(Color::rgb(42, 42, 42)),
            bg_alternate: None,
        }]);
        let mut cell = cell_with('▀');
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(100, 100, 100), "fg unchanged");
        assert_eq!(cell.bg, Color::rgb(42, 42, 42));
    }

    #[test]
    fn first_match_wins_even_with_later_matching_rule() {
        let filter = GlyphStyle::new(vec![
            GlyphStyleRule {
                chars: vec!['H'],
                fg: Some(Color::rgb(255, 0, 0)),
                bg: None,
            bg_alternate: None,
            },
            GlyphStyleRule {
                chars: vec!['H'],
                fg: Some(Color::rgb(0, 255, 0)),
                bg: None,
            bg_alternate: None,
            },
        ]);
        let mut cell = cell_with('H');
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(255, 0, 0), "first rule wins");
    }

    #[test]
    fn three_category_board_mirrors_solari_intent() {
        // Models the production HBF use case: hinge blocks get one style,
        // turned-preview glyphs get another, letters fall through to base.
        let filter = GlyphStyle::new(vec![
            GlyphStyleRule {
                chars: "█▀▔—▁▄▓▒░".chars().collect(),
                fg: Some(Color::rgb(210, 210, 210)),
                bg: Some(Color::rgb(42, 42, 42)),
            bg_alternate: None,
            },
            GlyphStyleRule {
                chars: "ⒶꓭƆꓷƎⅎ⅁ꓘ⅂ԀꓤꓤꞱ∩ΛⅯⅦⅣ".chars().collect(),
                fg: Some(Color::rgb(140, 140, 140)),
                bg: None,
            bg_alternate: None,
            },
        ]);

        let mut hinge = cell_with('█');
        filter.apply(&mut hinge, 0, 0, 10, 10, 0.0);
        assert_eq!(hinge.fg, Color::rgb(210, 210, 210));
        assert_eq!(hinge.bg, Color::rgb(42, 42, 42));

        let mut letter = cell_with('H');
        filter.apply(&mut letter, 0, 0, 10, 10, 0.0);
        assert_eq!(letter.fg, Color::rgb(100, 100, 100), "letter passes through");
        assert_eq!(letter.bg, Color::rgb(20, 20, 20));
    }

    // ---------- bg_alternate: coordinate checkerboard bounded by char match ----------

    #[test]
    fn bg_alternate_applies_on_odd_parity_only() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['A'],
            fg: None,
            bg: Some(Color::rgb(28, 28, 28)),
            bg_alternate: Some(Color::rgb(36, 36, 36)),
        }]);

        // (0,0) → parity even → bg
        let mut even = cell_with('A');
        filter.apply(&mut even, 0, 0, 10, 10, 0.0);
        assert_eq!(even.bg, Color::rgb(28, 28, 28), "even parity uses bg");

        // (1,0) → parity odd → bg_alternate
        let mut odd = cell_with('A');
        filter.apply(&mut odd, 1, 0, 10, 10, 0.0);
        assert_eq!(odd.bg, Color::rgb(36, 36, 36), "odd parity uses bg_alternate");

        // (0,1) → parity odd → bg_alternate
        let mut odd_y = cell_with('A');
        filter.apply(&mut odd_y, 0, 1, 10, 10, 0.0);
        assert_eq!(odd_y.bg, Color::rgb(36, 36, 36), "odd parity (y) uses bg_alternate");

        // (1,1) → parity even → bg
        let mut even_diag = cell_with('A');
        filter.apply(&mut even_diag, 1, 1, 10, 10, 0.0);
        assert_eq!(even_diag.bg, Color::rgb(28, 28, 28), "diagonal even uses bg");
    }

    #[test]
    fn bg_alternate_falls_back_to_bg_when_unset() {
        // bg_alternate=None → all parities use bg (v1.0.0 behavior)
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['A'],
            fg: None,
            bg: Some(Color::rgb(28, 28, 28)),
            bg_alternate: None,
        }]);
        for (x, y) in [(0u16, 0u16), (1, 0), (0, 1), (1, 1), (5, 7)] {
            let mut cell = cell_with('A');
            filter.apply(&mut cell, x, y, 10, 10, 0.0);
            assert_eq!(cell.bg, Color::rgb(28, 28, 28),
                       "cell ({x},{y}) should fall back to bg");
        }
    }

    #[test]
    fn bg_alternate_bounded_by_char_match() {
        // The card-edge effect must NOT apply to cells outside the rule's
        // char set — padding spaces, border, hinge glyphs all stay
        // unchanged regardless of (x,y) parity.
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['A', 'B'],
            fg: None,
            bg: Some(Color::rgb(28, 28, 28)),
            bg_alternate: Some(Color::rgb(36, 36, 36)),
        }]);

        // Space at odd parity — unchanged (no rule matches)
        let mut space = cell_with(' ');
        let before = space;
        filter.apply(&mut space, 1, 0, 10, 10, 0.0);
        assert_eq!(space, before, "unmatched space stays at original bg even at odd parity");

        // Border char at odd parity — unchanged
        let mut border = cell_with('│');
        let before = border;
        filter.apply(&mut border, 1, 0, 10, 10, 0.0);
        assert_eq!(border, before, "unmatched border char stays at original bg");

        // Matched 'B' at odd parity — gets bg_alternate
        let mut matched = cell_with('B');
        filter.apply(&mut matched, 1, 0, 10, 10, 0.0);
        assert_eq!(matched.bg, Color::rgb(36, 36, 36), "matched char gets bg_alternate");
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_glyph_style.rs</FILE>
// <VERS>END OF VERSION: 1.1.0</VERS>
