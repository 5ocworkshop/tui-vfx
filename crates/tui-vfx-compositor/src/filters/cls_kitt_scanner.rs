// <FILE>tui-vfx-compositor/src/filters/cls_kitt_scanner.rs</FILE>
// <DESC>Horizontal ping-pong scanner effect like KITT from Knight Rider</DESC>
// <VERS>VERSION: 1.7.0</VERS>
// <WCTX>Add boost_separator_bg for continuous powerline backgrounds</WCTX>
// <CLOG>Add boost_separator_bg toggle for powerlines with non-terminal backgrounds</CLOG>

use crate::traits::filter::Filter;
use crate::types::cls_filter_spec::ApplyTo;
use crate::utils::is_powerline_separator;
use tui_vfx_types::{Cell, Color};

/// Horizontal ping-pong scanner effect (Larson scanner / KITT scanner).
///
/// Creates a vertical band of brightness that sweeps horizontally left-to-right,
/// then right-to-left in a continuous ping-pong pattern. The effect applies to
/// the full height of the content, creating a dramatic scanning column.
///
/// # Usage
///
/// Apply to interactive elements for a retro sci-fi hover effect:
/// - Classic "KITT" scanner aesthetic
/// - Full-height scanning column
/// - Attention-grabbing hover state
/// - Rhythmic visual feedback
///
/// # Powerline Compatibility
///
/// For powerline-style segments, enable `powerline_mode` which:
/// - Boosts background on regular text cells (segment bg glows)
/// - Boosts fg only on separator glyphs (default - safe for end separators)
///
/// If your powerline has a continuous background that extends beyond the last
/// separator (e.g., connects to a right-side powerline), also enable
/// `boost_separator_bg` to boost both fg and bg on separators.
///
/// This prevents double-brightness while keeping separators visually consistent.
pub struct KittScanner {
    /// Brightness boost added to cells under the scanner
    pub boost: u8,
    /// Width of the scanner band (0.0-1.0 of total width)
    pub band_width: f32,
    /// Animation progress (0.0 = inactive, 1.0 = fully active)
    pub progress: f32,
    /// Beats per second for ping-pong cycle (default: 1.0 = 2 beats per cycle)
    pub bps: f32,
    /// Which color component to boost (ignored if powerline_mode is true)
    pub apply_to: ApplyTo,
    /// Smart powerline mode: bg on text, fg only on separators
    pub powerline_mode: bool,
    /// When true AND powerline_mode is true, also boost separator backgrounds.
    /// Use this when your powerline has a continuous background (not terminal bg).
    pub boost_separator_bg: bool,
}

impl Default for KittScanner {
    fn default() -> Self {
        Self {
            boost: 50,
            band_width: 0.15,
            progress: 0.0,
            bps: 1.0, // 1 beat/sec means 2 seconds for full ping-pong
            apply_to: ApplyTo::Both,
            powerline_mode: false,
            boost_separator_bg: false,
        }
    }
}

impl KittScanner {
    /// Create a new KittScanner with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the brightness boost.
    pub fn with_boost(mut self, boost: u8) -> Self {
        self.boost = boost;
        self
    }

    /// Set the band width (0.0-1.0).
    pub fn with_band_width(mut self, width: f32) -> Self {
        self.band_width = width.clamp(0.0, 0.5);
        self
    }

    /// Set the animation progress (0.0-1.0).
    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set the beats per second.
    pub fn with_bps(mut self, bps: f32) -> Self {
        self.bps = bps.max(0.1);
        self
    }

    /// Set which color components to boost.
    pub fn with_apply_to(mut self, apply_to: ApplyTo) -> Self {
        self.apply_to = apply_to;
        self
    }

    /// Enable powerline mode for smart separator handling.
    ///
    /// When enabled:
    /// - Regular text cells: only background is boosted
    /// - Powerline separator glyphs: only fg is boosted (unless boost_separator_bg is set)
    ///
    /// This keeps separators visually consistent with glowing segments.
    pub fn with_powerline_mode(mut self, enabled: bool) -> Self {
        self.powerline_mode = enabled;
        self
    }

    /// Enable background boosting on separator glyphs.
    ///
    /// Only takes effect when `powerline_mode` is also enabled. Use this when
    /// your powerline has a continuous background that extends beyond the last
    /// separator (e.g., the bar connects to a right-side powerline rather than
    /// ending against the terminal background).
    ///
    /// When enabled, separators get both fg and bg boosted (at same rate as text).
    pub fn with_boost_separator_bg(mut self, enabled: bool) -> Self {
        self.boost_separator_bg = enabled;
        self
    }
}

impl Filter for KittScanner {
    fn apply(&self, cell: &mut Cell, x: u16, _y: u16, width: u16, _height: u16, t: f64) {
        if self.progress <= 0.0 || width == 0 {
            return;
        }

        // Ping-pong position using sine wave
        // bps=1.0 means 1 beat per second, so 2 seconds for full cycle
        // sin goes -1 to 1, we map to 0 to 1 for position
        let cycle = (t * self.bps as f64 * std::f64::consts::PI) as f32;
        let ping_pong = (cycle.sin() + 1.0) / 2.0; // 0.0 to 1.0, smooth ping-pong

        // Horizontal position
        let nx = x as f32 / width.max(1) as f32;

        // Distance from scanner position
        let dist = (nx - ping_pong).abs();

        // Apply highlight if within band (applies to all rows for full-height effect)
        if dist < self.band_width {
            let intensity = 1.0 - (dist / self.band_width);
            // Smooth cubic falloff
            let intensity = intensity * intensity * (3.0 - 2.0 * intensity);
            let boost = (intensity * self.boost as f32 * self.progress) as u8;

            // Determine what to boost based on mode
            // In powerline mode, we want consistent brightness:
            // - Regular text: boost bg only (segment background glows)
            // - Separator glyphs: boost fg only by default, or fg+bg if boost_separator_bg
            // Both get the same boost amount so brightness is uniform
            let (boost_fg, boost_bg) = if self.powerline_mode {
                let is_separator = is_powerline_separator(cell.ch);
                if is_separator {
                    // Separator: always boost fg, optionally boost bg
                    (true, self.boost_separator_bg)
                } else {
                    // Regular text: bg only
                    (false, true)
                }
            } else {
                // Standard mode: use apply_to setting
                (
                    matches!(self.apply_to, ApplyTo::Foreground | ApplyTo::Both),
                    matches!(self.apply_to, ApplyTo::Background | ApplyTo::Both),
                )
            };

            // Use consistent boost amount for both fg and bg in powerline mode
            let fg_boost = if self.powerline_mode {
                boost / 2
            } else {
                boost
            };
            let bg_boost = boost / 2;

            // Boost foreground if enabled and not transparent
            if boost_fg && cell.fg.a > 0 {
                let (r, g, b) = cell.fg.to_rgb();
                cell.fg = Color::new(
                    r.saturating_add(fg_boost),
                    g.saturating_add(fg_boost),
                    b.saturating_add(fg_boost),
                    cell.fg.a,
                );
            }

            // Boost background if enabled and not transparent
            if boost_bg && cell.bg.a > 0 {
                let (br, bg, bb) = cell.bg.to_rgb();
                cell.bg = Color::new(
                    br.saturating_add(bg_boost),
                    bg.saturating_add(bg_boost),
                    bb.saturating_add(bg_boost),
                    cell.bg.a,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    fn make_cell() -> Cell {
        Cell::styled(
            'A',
            Color::rgb(100, 100, 100),
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        )
    }

    #[test]
    fn default_values() {
        let filter = KittScanner::default();
        assert_eq!(filter.boost, 50);
        assert_eq!(filter.band_width, 0.15);
        assert_eq!(filter.progress, 0.0);
        assert_eq!(filter.bps, 1.0);
    }

    #[test]
    fn zero_progress_no_change() {
        let filter = KittScanner::new().with_progress(0.0);
        let mut cell = make_cell();
        let original_fg = cell.fg;
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        assert_eq!(cell.fg, original_fg);
    }

    #[test]
    fn full_progress_applies_highlight_at_center() {
        let filter = KittScanner::new().with_progress(1.0);
        let mut cell = make_cell();
        let original_fg = cell.fg;
        // At t=0.0, sin(0)=0, so position is 0.5 (center)
        // Test cell at x=5 in width=10 (center)
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        assert_ne!(cell.fg, original_fg);
    }

    #[test]
    fn affects_all_rows() {
        let filter = KittScanner::new().with_progress(1.0);

        // Middle row (y=1 in height=3)
        let mut mid_cell = make_cell();
        filter.apply(&mut mid_cell, 5, 1, 10, 3, 0.0);
        let mid_changed = mid_cell.fg != Color::rgb(100, 100, 100);

        // Top row (should also be affected now)
        let mut top_cell = make_cell();
        filter.apply(&mut top_cell, 5, 0, 10, 3, 0.0);
        let top_changed = top_cell.fg != Color::rgb(100, 100, 100);

        // Both rows should be affected (full-height effect)
        assert!(mid_changed);
        assert!(top_changed);
    }

    #[test]
    fn ping_pong_at_edges() {
        let filter = KittScanner::new().with_progress(1.0).with_bps(1.0);

        // At t=0, sin(0)=0 → position = 0.5 (center)
        // At t=0.5 (half beat), sin(π/2)=1 → position = 1.0 (right edge)
        // At t=1.0 (one beat), sin(π)=0 → position = 0.5 (center)
        // At t=1.5 (1.5 beats), sin(3π/2)=-1 → position = 0.0 (left edge)

        let mut cell_center = make_cell();
        filter.apply(&mut cell_center, 5, 0, 10, 1, 0.0);

        let mut cell_right = make_cell();
        filter.apply(&mut cell_right, 9, 0, 10, 1, 0.5);

        let mut cell_left = make_cell();
        filter.apply(&mut cell_left, 0, 0, 10, 1, 1.5);

        // All should be highlighted at their respective times
        assert_ne!(cell_center.fg, Color::rgb(100, 100, 100));
        assert_ne!(cell_right.fg, Color::rgb(100, 100, 100));
        assert_ne!(cell_left.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn builder_pattern() {
        let filter = KittScanner::new()
            .with_boost(80)
            .with_band_width(0.25)
            .with_progress(0.75)
            .with_bps(2.0);

        assert_eq!(filter.boost, 80);
        assert_eq!(filter.band_width, 0.25);
        assert_eq!(filter.progress, 0.75);
        assert_eq!(filter.bps, 2.0);
    }

    #[test]
    fn preserves_transparent_fg() {
        let filter = KittScanner::new().with_progress(1.0);
        let mut cell = Cell::styled(
            'A',
            Color::TRANSPARENT, // Transparent fg
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        );
        let original_fg = cell.fg;
        // Apply at center where highlight would occur
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // Transparent fg should remain transparent
        assert_eq!(cell.fg, original_fg);
        assert_eq!(cell.fg.a, 0);
    }

    #[test]
    fn preserves_transparent_bg() {
        let filter = KittScanner::new().with_progress(1.0);
        let mut cell = Cell::styled(
            'A',
            Color::rgb(100, 100, 100),
            Color::TRANSPARENT, // Transparent bg
            Modifiers::NONE,
        );
        let original_bg = cell.bg;
        // Apply at center where highlight would occur
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // Transparent bg should remain transparent
        assert_eq!(cell.bg, original_bg);
        assert_eq!(cell.bg.a, 0);
    }

    #[test]
    fn preserves_alpha_when_boosting() {
        use crate::types::cls_filter_spec::ApplyTo;

        // Use ApplyTo::Both to test both fg and bg boosting
        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_apply_to(ApplyTo::Both);
        // Semi-transparent colors
        let mut cell = Cell::styled(
            'A',
            Color::new(100, 100, 100, 128),
            Color::new(30, 30, 30, 200),
            Modifiers::NONE,
        );
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // Alpha values should be preserved
        assert_eq!(cell.fg.a, 128);
        assert_eq!(cell.bg.a, 200);
        // But colors should be boosted
        assert!(cell.fg.r > 100);
        assert!(cell.bg.r > 30);
    }

    #[test]
    fn apply_to_foreground_only() {
        use crate::types::cls_filter_spec::ApplyTo;

        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_apply_to(ApplyTo::Foreground);
        let mut cell = make_cell();
        let original_bg = cell.bg;
        // Apply at center where highlight would occur
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // Foreground should be boosted
        assert!(cell.fg.r > 100);
        // Background should NOT be boosted (unchanged)
        assert_eq!(cell.bg, original_bg);
    }

    #[test]
    fn apply_to_background_only() {
        use crate::types::cls_filter_spec::ApplyTo;

        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_apply_to(ApplyTo::Background);
        let mut cell = make_cell();
        let original_fg = cell.fg;
        // Apply at center where highlight would occur
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // Foreground should NOT be boosted (unchanged)
        assert_eq!(cell.fg, original_fg);
        // Background should be boosted
        assert!(cell.bg.r > 30);
    }

    #[test]
    fn apply_to_both() {
        use crate::types::cls_filter_spec::ApplyTo;

        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_apply_to(ApplyTo::Both);
        let mut cell = make_cell();
        // Apply at center where highlight would occur
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // Both should be boosted
        assert!(cell.fg.r > 100);
        assert!(cell.bg.r > 30);
    }

    #[test]
    fn default_apply_to_is_both() {
        use crate::types::cls_filter_spec::ApplyTo;

        let filter = KittScanner::default();
        assert_eq!(filter.apply_to, ApplyTo::Both);
    }

    #[test]
    fn powerline_mode_boosts_bg_on_regular_text() {
        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_powerline_mode(true);
        let mut cell = make_cell(); // 'A' is not a powerline separator
        let original_fg = cell.fg;
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // Foreground should NOT be boosted (regular text)
        assert_eq!(cell.fg, original_fg);
        // Background should be boosted
        assert!(cell.bg.r > 30);
    }

    #[test]
    fn powerline_mode_boosts_fg_only_on_separator() {
        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_powerline_mode(true);
        // U+E0B0 is the classic powerline arrow separator
        let mut cell = Cell::styled(
            '\u{E0B0}',
            Color::rgb(100, 100, 100),
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        );
        let original_bg = cell.bg;
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // fg should be boosted for separator
        assert!(cell.fg.r > 100);
        // bg should NOT be boosted (avoid double-brightness and end-separator artifacts)
        assert_eq!(cell.bg, original_bg);
    }

    #[test]
    fn powerline_mode_detects_bubble_separator() {
        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_powerline_mode(true);
        // U+E0B4 is the bubble/rounded separator
        let mut cell = Cell::styled(
            '\u{E0B4}',
            Color::rgb(100, 100, 100),
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        );
        let original_bg = cell.bg;
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        // fg should be boosted for separator
        assert!(cell.fg.r > 100);
        // bg should NOT be boosted
        assert_eq!(cell.bg, original_bg);
    }

    #[test]
    fn powerline_mode_uniform_boost_amount() {
        // Verify that text cells and separator cells get the same boost amount
        let filter = KittScanner::new()
            .with_boost(50)
            .with_progress(1.0)
            .with_powerline_mode(true);

        // Regular text cell
        let mut text_cell = Cell::styled(
            'A',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100), // Same starting color for comparison
            Modifiers::NONE,
        );
        filter.apply(&mut text_cell, 5, 0, 10, 1, 0.0);
        let text_bg_boost = text_cell.bg.r - 100;

        // Separator cell
        let mut sep_cell = Cell::styled(
            '\u{E0B0}',
            Color::rgb(100, 100, 100), // Same starting color
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        filter.apply(&mut sep_cell, 5, 0, 10, 1, 0.0);
        let sep_fg_boost = sep_cell.fg.r - 100;

        // Both should get the same boost amount (boost/2)
        assert_eq!(text_bg_boost, sep_fg_boost);
    }

    #[test]
    fn is_powerline_separator_detection() {
        use crate::utils::is_powerline_separator;

        // Classic powerline arrows
        assert!(is_powerline_separator('\u{E0B0}')); //
        assert!(is_powerline_separator('\u{E0B2}')); //
        // Bubble/rounded
        assert!(is_powerline_separator('\u{E0B4}')); //
        assert!(is_powerline_separator('\u{E0B6}')); //
        // Flame
        assert!(is_powerline_separator('\u{E0C0}')); // pixel
        assert!(is_powerline_separator('\u{E0C8}')); // flame
        // Wave
        assert!(is_powerline_separator('\u{E0D2}')); // wave

        // Non-separators
        assert!(!is_powerline_separator('A'));
        assert!(!is_powerline_separator(' '));
        assert!(!is_powerline_separator('│'));
    }

    #[test]
    fn powerline_mode_boost_separator_bg_enabled() {
        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_powerline_mode(true)
            .with_boost_separator_bg(true);

        // U+E0B0 is the classic powerline arrow separator
        let mut cell = Cell::styled(
            '\u{E0B0}',
            Color::rgb(100, 100, 100),
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        );
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);

        // fg should be boosted
        assert!(cell.fg.r > 100);
        // bg should ALSO be boosted when boost_separator_bg is enabled
        assert!(cell.bg.r > 30);
    }

    #[test]
    fn boost_separator_bg_has_no_effect_without_powerline_mode() {
        // boost_separator_bg should be ignored when powerline_mode is false
        let filter = KittScanner::new()
            .with_progress(1.0)
            .with_powerline_mode(false)
            .with_boost_separator_bg(true)
            .with_apply_to(ApplyTo::Foreground);

        let mut cell = Cell::styled(
            '\u{E0B0}',
            Color::rgb(100, 100, 100),
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        );
        let original_bg = cell.bg;
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);

        // Should follow apply_to (fg only), not boost_separator_bg
        assert!(cell.fg.r > 100);
        assert_eq!(cell.bg, original_bg);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_kitt_scanner.rs</FILE>
// <DESC>Horizontal ping-pong scanner effect like KITT from Knight Rider</DESC>
// <VERS>END OF VERSION: 1.7.0</VERS>
