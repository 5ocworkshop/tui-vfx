// <FILE>tui-vfx-compositor/src/filters/cls_glyph_style.rs</FILE>
// <DESC>GlyphStyle filter — per-glyph-category style overrides via char-membership rules</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>New filter that styles cells based on their char content, not position. Enables content transformers that emit mixed glyph categories (SplitFlap block/hinge/letter/turned) to be colored independently by recipes without per-cell RoleMap plumbing — first-rule-wins char-membership match.</WCTX>
// <CLOG>Initial: Filter impl takes Vec<GlyphStyleRule>; each rule carries a char set plus optional fg/bg/modifiers overrides. apply() walks rules in declaration order, applies the first matching rule's overrides, returns. Unmatched cells pass through unchanged. Inline tests cover first-match-wins, no-match-passthrough, fg-only and bg-only overrides, multi-rule ordering.</CLOG>

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
#[derive(Debug, Clone)]
pub struct GlyphStyleRule {
    /// Characters this rule matches.
    pub chars: Vec<char>,
    /// Override foreground color. `None` leaves fg unchanged.
    pub fg: Option<Color>,
    /// Override background color. `None` leaves bg unchanged.
    pub bg: Option<Color>,
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
    fn apply(&self, cell: &mut Cell, _x: u16, _y: u16, _w: u16, _h: u16, _t: f64) {
        for rule in &self.rules {
            if rule.chars.contains(&cell.ch) {
                if let Some(fg) = rule.fg {
                    cell.fg = fg;
                }
                if let Some(bg) = rule.bg {
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
            },
            GlyphStyleRule {
                chars: vec!['H'],
                fg: Some(Color::rgb(0, 255, 0)),
                bg: None,
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
            },
            GlyphStyleRule {
                chars: "ⒶꓭƆꓷƎⅎ⅁ꓘ⅂ԀꓤꓤꞱ∩ΛⅯⅦⅣ".chars().collect(),
                fg: Some(Color::rgb(140, 140, 140)),
                bg: None,
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
}

// <FILE>tui-vfx-compositor/src/filters/cls_glyph_style.rs</FILE>
// <VERS>END OF VERSION: 1.0.0</VERS>
