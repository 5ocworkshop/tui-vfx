// <FILE>tui-vfx-compositor/src/filters/cls_glisten_sweep.rs</FILE>
// <DESC>Diagonal glisten sweep effect for hover states</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>Add boost_separator_bg for continuous powerline backgrounds</WCTX>
// <CLOG>Add boost_separator_bg toggle for powerlines with non-terminal backgrounds</CLOG>

use crate::traits::filter::Filter;
use crate::utils::is_powerline_separator;
use tui_vfx_types::{Cell, Color};

/// Diagonal glisten sweep that moves across content on hover.
///
/// Creates a 45-degree highlight band that sweeps from one corner to another,
/// providing a polished "shine" effect for buttons and interactive elements.
///
/// # Usage
///
/// Apply to buttons or cards for hover feedback:
/// - Animated shine on hover
/// - Polished button appearance
/// - Premium UI feel
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
pub struct GlistenSweep {
    /// Highlight color boost (added to existing colors)
    pub boost: u8,
    /// Width of the glisten band (0.0-1.0, relative to diagonal)
    pub band_width: f32,
    /// Animation progress (0.0 = start, 1.0 = end of sweep)
    pub progress: f32,
    /// Speed multiplier for time-based animation (0 = use progress only)
    pub speed: f32,
    /// Smart powerline mode: bg on text, fg only on separators
    pub powerline_mode: bool,
    /// When true AND powerline_mode is true, also boost separator backgrounds.
    /// Use this when your powerline has a continuous background (not terminal bg).
    pub boost_separator_bg: bool,
}

impl Default for GlistenSweep {
    fn default() -> Self {
        Self {
            boost: 40,
            band_width: 0.2,
            progress: 0.0,
            speed: 0.5,
            powerline_mode: false,
            boost_separator_bg: false,
        }
    }
}

impl GlistenSweep {
    /// Create a new GlistenSweep with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the highlight boost amount.
    pub fn with_boost(mut self, boost: u8) -> Self {
        self.boost = boost;
        self
    }

    /// Set the band width (0.0-1.0).
    pub fn with_band_width(mut self, width: f32) -> Self {
        self.band_width = width.clamp(0.0, 1.0);
        self
    }

    /// Set the animation progress (0.0-1.0).
    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set the animation speed (0 = manual progress only).
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Enable powerline mode for smart separator handling.
    ///
    /// When enabled:
    /// - Regular text cells: only background is boosted
    /// - Powerline separator glyphs: only fg is boosted (unless boost_separator_bg is set)
    ///
    /// Both get the same boost amount so brightness is uniform.
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

impl Filter for GlistenSweep {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64) {
        if self.progress <= 0.0 {
            return;
        }

        // Calculate diagonal position (0.0 = top-left, 1.0 = bottom-right)
        // Using normalized coordinates for 45-degree sweep
        let nx = x as f32 / width.max(1) as f32;
        let ny = y as f32 / height.max(1) as f32;

        // Diagonal position: average of x and y gives 45-degree line
        let diag_pos = (nx + ny) / 2.0;

        // Sweep position based on progress (and optionally time)
        let sweep_pos = if self.speed > 0.0 {
            // Time-based cycling when hovered
            let cycle = (t * self.speed as f64) % 1.0;
            // Only animate when progress > 0.5 (hovering)
            if self.progress > 0.5 {
                cycle as f32
            } else {
                // Fade out - sweep exits
                1.0 + (1.0 - self.progress * 2.0) * 0.5
            }
        } else {
            // Manual progress mode
            self.progress
        };

        // Distance from sweep position
        let dist = (diag_pos - sweep_pos).abs();

        // Apply highlight if within band
        if dist < self.band_width {
            let intensity = 1.0 - (dist / self.band_width);
            // Smooth falloff
            let intensity = intensity * intensity * (3.0 - 2.0 * intensity);
            let boost = (intensity * self.boost as f32 * self.progress) as u8;

            // Determine what to boost based on mode
            // In powerline mode, we want consistent brightness:
            // - Regular text: boost bg only (segment background glows)
            // - Separator glyphs: boost fg only by default, or fg+bg if boost_separator_bg
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
                // Standard mode: boost both
                (true, true)
            };

            // Use consistent boost amount in powerline mode
            let fg_boost = if self.powerline_mode {
                boost / 3
            } else {
                boost
            };
            let bg_boost = boost / 3;

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
        let filter = GlistenSweep::default();
        assert_eq!(filter.boost, 40);
        assert_eq!(filter.band_width, 0.2);
        assert_eq!(filter.progress, 0.0);
        assert_eq!(filter.speed, 0.5);
    }

    #[test]
    fn zero_progress_no_change() {
        let filter = GlistenSweep::new().with_progress(0.0);
        let mut cell = make_cell();
        let original_fg = cell.fg;
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, original_fg);
    }

    #[test]
    fn full_progress_applies_highlight() {
        let filter = GlistenSweep::new().with_progress(1.0).with_speed(0.0); // Manual mode

        let mut cell = make_cell();
        let original_fg = cell.fg;
        // At progress=1.0 with speed=0, sweep is at position 1.0 (bottom-right)
        // Cell at (9,9) in 10x10 has diag_pos ~= 0.9, close to sweep
        filter.apply(&mut cell, 9, 9, 10, 10, 0.0);
        // Should be brightened
        assert_ne!(cell.fg, original_fg);
    }

    #[test]
    fn builder_pattern() {
        let filter = GlistenSweep::new()
            .with_boost(60)
            .with_band_width(0.3)
            .with_progress(0.5)
            .with_speed(1.0);

        assert_eq!(filter.boost, 60);
        assert_eq!(filter.band_width, 0.3);
        assert_eq!(filter.progress, 0.5);
        assert_eq!(filter.speed, 1.0);
    }

    #[test]
    fn preserves_transparent_fg() {
        let filter = GlistenSweep::new().with_progress(1.0).with_speed(0.0);
        let mut cell = Cell::styled(
            'A',
            Color::TRANSPARENT, // Transparent fg
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        );
        let original_fg = cell.fg;
        // Apply at position where highlight would occur
        filter.apply(&mut cell, 9, 9, 10, 10, 0.0);
        // Transparent fg should remain transparent
        assert_eq!(cell.fg, original_fg);
        assert_eq!(cell.fg.a, 0);
    }

    #[test]
    fn preserves_transparent_bg() {
        let filter = GlistenSweep::new().with_progress(1.0).with_speed(0.0);
        let mut cell = Cell::styled(
            'A',
            Color::rgb(100, 100, 100),
            Color::TRANSPARENT, // Transparent bg
            Modifiers::NONE,
        );
        let original_bg = cell.bg;
        // Apply at position where highlight would occur
        filter.apply(&mut cell, 9, 9, 10, 10, 0.0);
        // Transparent bg should remain transparent
        assert_eq!(cell.bg, original_bg);
        assert_eq!(cell.bg.a, 0);
    }

    #[test]
    fn preserves_alpha_when_boosting() {
        let filter = GlistenSweep::new().with_progress(1.0).with_speed(0.0);
        // Semi-transparent colors
        let mut cell = Cell::styled(
            'A',
            Color::new(100, 100, 100, 128),
            Color::new(30, 30, 30, 200),
            Modifiers::NONE,
        );
        filter.apply(&mut cell, 9, 9, 10, 10, 0.0);
        // Alpha values should be preserved
        assert_eq!(cell.fg.a, 128);
        assert_eq!(cell.bg.a, 200);
        // But colors should be boosted
        assert!(cell.fg.r > 100);
    }

    #[test]
    fn powerline_mode_boosts_bg_on_regular_text() {
        let filter = GlistenSweep::new()
            .with_progress(1.0)
            .with_speed(0.0)
            .with_powerline_mode(true);
        let mut cell = make_cell(); // 'A' is not a powerline separator
        let original_fg = cell.fg;
        filter.apply(&mut cell, 9, 9, 10, 10, 0.0);
        // Foreground should NOT be boosted (regular text)
        assert_eq!(cell.fg, original_fg);
        // Background should be boosted
        assert!(cell.bg.r > 30);
    }

    #[test]
    fn powerline_mode_boosts_fg_only_on_separator() {
        let filter = GlistenSweep::new()
            .with_progress(1.0)
            .with_speed(0.0)
            .with_powerline_mode(true);
        // U+E0B0 is the classic powerline arrow separator
        let mut cell = Cell::styled(
            '\u{E0B0}',
            Color::rgb(100, 100, 100),
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        );
        let original_bg = cell.bg;
        filter.apply(&mut cell, 9, 9, 10, 10, 0.0);
        // fg should be boosted for separator
        assert!(cell.fg.r > 100);
        // bg should NOT be boosted
        assert_eq!(cell.bg, original_bg);
    }

    #[test]
    fn powerline_mode_boost_separator_bg_enabled() {
        let filter = GlistenSweep::new()
            .with_progress(1.0)
            .with_speed(0.0)
            .with_powerline_mode(true)
            .with_boost_separator_bg(true);

        // U+E0B0 is the classic powerline arrow separator
        let mut cell = Cell::styled(
            '\u{E0B0}',
            Color::rgb(100, 100, 100),
            Color::rgb(30, 30, 30),
            Modifiers::NONE,
        );
        filter.apply(&mut cell, 9, 9, 10, 10, 0.0);

        // fg should be boosted
        assert!(cell.fg.r > 100);
        // bg should ALSO be boosted when boost_separator_bg is enabled
        assert!(cell.bg.r > 30);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_glisten_sweep.rs</FILE>
// <DESC>Diagonal glisten sweep effect for hover states</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
