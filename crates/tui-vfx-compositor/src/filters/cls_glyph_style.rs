// <FILE>tui-vfx-compositor/src/filters/cls_glyph_style.rs</FILE>
// <DESC>GlyphStyle filter — per-glyph-category style overrides via char-membership rules</DESC>
// <VERS>VERSION: 1.2.1</VERS>
// <WCTX>Slice 6.6 §F.5 — migrate Filter trait to VfxCellContext bundle</WCTX>
// <CLOG>1.2.1: migrate apply signature to &VfxCellContext.</CLOG>

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
use tui_vfx_types::{Cell, Color, VfxCellContext};

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
    /// Alternate foreground for `(x + y) % 2 == 1` cells. `None` disables
    /// fg alternation. When set, even-parity cells use `fg` (or leave fg
    /// untouched if `fg` is also `None`), odd-parity cells use
    /// `fg_alternate`. Intended for block/hinge glyphs whose face colour
    /// should match the NEIGHBOR cell's `bg` — pair with `bg_alternate`
    /// and cross-assign (fg = neighbor bg at even, fg_alternate =
    /// neighbor bg at odd) for a subtle depth/shadow read as cells flip.
    pub fg_alternate: Option<Color>,
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
    fn apply(&self, cell: &mut Cell, ctx: &VfxCellContext) {
        let x = ctx.local_x;
        let y = ctx.local_y;
        for rule in &self.rules {
            if rule.chars.contains(&cell.ch) {
                // Coordinate-checkerboard fg: symmetric to bg path below.
                // Even parity uses fg; odd parity uses fg_alternate (or
                // falls back to fg when fg_alternate is None). Both None
                // leaves fg untouched — v1.1.0 compatibility.
                let parity_fg = if (x + y) % 2 == 1 {
                    rule.fg_alternate.or(rule.fg)
                } else {
                    rule.fg
                };
                if let Some(fg) = parity_fg {
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
        filter.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(cell, before);
    }

    #[test]
    fn unmatched_char_leaves_cell_unchanged() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['█', '▀'],
            fg: Some(Color::rgb(210, 210, 210)),
            bg: None,
            bg_alternate: None,
            fg_alternate: None,
        }]);
        let mut cell = cell_with('X');
        let before = cell;
        filter.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(cell, before);
    }

    #[test]
    fn matched_char_gets_fg_override() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['█', '▀'],
            fg: Some(Color::rgb(180, 180, 180)),
            bg: None,
            bg_alternate: None,
            fg_alternate: None,
        }]);
        let mut cell = cell_with('█');
        filter.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
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
            fg_alternate: None,
        }]);
        let mut cell = cell_with('▀');
        filter.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
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
                fg_alternate: None,
            },
            GlyphStyleRule {
                chars: vec!['H'],
                fg: Some(Color::rgb(0, 255, 0)),
                bg: None,
                bg_alternate: None,
                fg_alternate: None,
            },
        ]);
        let mut cell = cell_with('H');
        filter.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
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
                fg_alternate: None,
            },
            GlyphStyleRule {
                chars: "ⒶꓭƆꓷƎⅎ⅁ꓘ⅂ԀꓤꓤꞱ∩ΛⅯⅦⅣ".chars().collect(),
                fg: Some(Color::rgb(140, 140, 140)),
                bg: None,
                bg_alternate: None,
                fg_alternate: None,
            },
        ]);

        let mut hinge = cell_with('█');
        filter.apply(&mut hinge, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(hinge.fg, Color::rgb(210, 210, 210));
        assert_eq!(hinge.bg, Color::rgb(42, 42, 42));

        let mut letter = cell_with('H');
        filter.apply(&mut letter, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(
            letter.fg,
            Color::rgb(100, 100, 100),
            "letter passes through"
        );
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
            fg_alternate: None,
        }]);

        // (0,0) → parity even → bg
        let mut even = cell_with('A');
        filter.apply(&mut even, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(even.bg, Color::rgb(28, 28, 28), "even parity uses bg");

        // (1,0) → parity odd → bg_alternate
        let mut odd = cell_with('A');
        filter.apply(&mut odd, &VfxCellContext::new(1, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(
            odd.bg,
            Color::rgb(36, 36, 36),
            "odd parity uses bg_alternate"
        );

        // (0,1) → parity odd → bg_alternate
        let mut odd_y = cell_with('A');
        filter.apply(&mut odd_y, &VfxCellContext::new(0, 1, 10, 10, 0, 0, 0.0));
        assert_eq!(
            odd_y.bg,
            Color::rgb(36, 36, 36),
            "odd parity (y) uses bg_alternate"
        );

        // (1,1) → parity even → bg
        let mut even_diag = cell_with('A');
        filter.apply(
            &mut even_diag,
            &VfxCellContext::new(1, 1, 10, 10, 0, 0, 0.0),
        );
        assert_eq!(
            even_diag.bg,
            Color::rgb(28, 28, 28),
            "diagonal even uses bg"
        );
    }

    #[test]
    fn bg_alternate_falls_back_to_bg_when_unset() {
        // bg_alternate=None → all parities use bg (v1.0.0 behavior)
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['A'],
            fg: None,
            bg: Some(Color::rgb(28, 28, 28)),
            bg_alternate: None,
            fg_alternate: None,
        }]);
        for (x, y) in [(0u16, 0u16), (1, 0), (0, 1), (1, 1), (5, 7)] {
            let mut cell = cell_with('A');
            filter.apply(&mut cell, &VfxCellContext::new(x, y, 10, 10, 0, 0, 0.0));
            assert_eq!(
                cell.bg,
                Color::rgb(28, 28, 28),
                "cell ({x},{y}) should fall back to bg"
            );
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
            fg_alternate: None,
        }]);

        // Space at odd parity — unchanged (no rule matches)
        let mut space = cell_with(' ');
        let before = space;
        filter.apply(&mut space, &VfxCellContext::new(1, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(
            space, before,
            "unmatched space stays at original bg even at odd parity"
        );

        // Border char at odd parity — unchanged
        let mut border = cell_with('│');
        let before = border;
        filter.apply(&mut border, &VfxCellContext::new(1, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(border, before, "unmatched border char stays at original bg");

        // Matched 'B' at odd parity — gets bg_alternate
        let mut matched = cell_with('B');
        filter.apply(&mut matched, &VfxCellContext::new(1, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(
            matched.bg,
            Color::rgb(36, 36, 36),
            "matched char gets bg_alternate"
        );
    }

    // ---------- fg_alternate: coordinate checkerboard fg modulation ----------

    #[test]
    fn fg_alternate_applies_on_odd_parity_only() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['█'],
            fg: Some(Color::rgb(200, 200, 200)),
            bg: None,
            bg_alternate: None,
            fg_alternate: Some(Color::rgb(120, 120, 120)),
        }]);

        let mut even = cell_with('█');
        filter.apply(&mut even, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(even.fg, Color::rgb(200, 200, 200), "even parity uses fg");

        let mut odd_x = cell_with('█');
        filter.apply(&mut odd_x, &VfxCellContext::new(1, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(
            odd_x.fg,
            Color::rgb(120, 120, 120),
            "odd x uses fg_alternate"
        );

        let mut odd_y = cell_with('█');
        filter.apply(&mut odd_y, &VfxCellContext::new(0, 1, 10, 10, 0, 0, 0.0));
        assert_eq!(
            odd_y.fg,
            Color::rgb(120, 120, 120),
            "odd y uses fg_alternate"
        );

        let mut diag_even = cell_with('█');
        filter.apply(
            &mut diag_even,
            &VfxCellContext::new(1, 1, 10, 10, 0, 0, 0.0),
        );
        assert_eq!(
            diag_even.fg,
            Color::rgb(200, 200, 200),
            "diagonal even uses fg"
        );
    }

    #[test]
    fn fg_alternate_falls_back_to_fg_when_unset() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['█'],
            fg: Some(Color::rgb(200, 200, 200)),
            bg: None,
            bg_alternate: None,
            fg_alternate: None,
        }]);
        for (x, y) in [(0u16, 0u16), (1, 0), (0, 1), (1, 1), (5, 7)] {
            let mut cell = cell_with('█');
            filter.apply(&mut cell, &VfxCellContext::new(x, y, 10, 10, 0, 0, 0.0));
            assert_eq!(
                cell.fg,
                Color::rgb(200, 200, 200),
                "cell ({x},{y}) should fall back to fg"
            );
        }
    }

    #[test]
    fn fg_alternate_bounded_by_char_match() {
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['█'],
            fg: Some(Color::rgb(200, 200, 200)),
            bg: None,
            bg_alternate: None,
            fg_alternate: Some(Color::rgb(120, 120, 120)),
        }]);

        // Unmatched char at odd parity stays unchanged
        let mut space = cell_with(' ');
        let before = space;
        filter.apply(&mut space, &VfxCellContext::new(1, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(
            space, before,
            "unmatched space is not touched by fg_alternate"
        );

        // Matched char at odd parity gets fg_alternate
        let mut block = cell_with('█');
        filter.apply(&mut block, &VfxCellContext::new(1, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(block.fg, Color::rgb(120, 120, 120));
    }

    #[test]
    fn fg_alternate_crossed_with_bg_produces_neighbor_bg_face() {
        // The HBF sparse_update use case: setting fg = bg_alternate's
        // color and fg_alternate = bg's color means the flap's face
        // always carries the NEIGHBOR cell's bg shade (subtle depth).
        let own_bg = Color::rgb(44, 46, 52);
        let own_bg_alt = Color::rgb(66, 58, 46);
        let filter = GlyphStyle::new(vec![GlyphStyleRule {
            chars: vec!['█'],
            fg: Some(own_bg_alt),
            bg: Some(own_bg),
            bg_alternate: Some(own_bg_alt),
            fg_alternate: Some(own_bg),
        }]);

        // Even parity: cell bg = own_bg; neighbor bg = own_bg_alt.
        // Flap fg should equal own_bg_alt (= neighbor bg).
        let mut even = cell_with('█');
        filter.apply(&mut even, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(even.bg, own_bg, "even parity cell bg");
        assert_eq!(even.fg, own_bg_alt, "even parity fg matches neighbor bg");

        // Odd parity: cell bg = own_bg_alt; neighbor bg = own_bg.
        // Flap fg should equal own_bg (= neighbor bg).
        let mut odd = cell_with('█');
        filter.apply(&mut odd, &VfxCellContext::new(1, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(odd.bg, own_bg_alt, "odd parity cell bg");
        assert_eq!(odd.fg, own_bg, "odd parity fg matches neighbor bg");
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_glyph_style.rs</FILE>
// <VERS>END OF VERSION: 1.2.1</VERS>
