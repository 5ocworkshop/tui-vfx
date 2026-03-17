// <FILE>tui-vfx-compositor/src/filters/cls_braille_dust.rs</FILE> - <DESC>Stochastic braille dust filter for frosted glass texture</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>Desynchronize braille_dust particles and add fade envelope</WCTX>
// <CLOG>Per-cell time offset for staggered particles; fade envelope dims fg color smoothly (sin bell curve) for organic fade-in/fade-out</CLOG>

use crate::traits::filter::Filter;
use mixed_signals::math::fast_random;
use tui_vfx_types::braille;
use tui_vfx_types::{Cell, Color};

/// Braille dot pattern options for the dust effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BraillePattern {
    /// Single dots only (⠁ ⠂ ⠄) - most subtle
    #[default]
    SingleDot,
    /// 1-2 vertical dots (⠁ ⠂ ⠄ ⠃ ⠆) - subtle
    OneToTwoDots,
    /// 1-3 vertical dots (⠁ ⠂ ⠄ ⠃ ⠆ ⠇) - moderate
    OneToThreeDots,
    /// 1-4 dots using both columns - more visible
    OneToFourDots,
}

/// Stochastic braille dust filter for frosted glass / film grain texture.
///
/// Places small braille dot patterns in empty cells at random positions,
/// creating a subtle animated "dust motes" or "frosted glass" effect.
/// Only affects cells that contain whitespace, preserving actual content.
///
/// # Example
///
/// ```ignore
/// let dust = BrailleDust::new()
///     .with_density(0.03)      // 3% of empty cells
///     .with_hz(8.0)            // 8 pattern changes per second
///     .with_pattern(BraillePattern::SingleDot);
/// ```
pub struct BrailleDust {
    /// Fraction of empty cells to fill (0.0 - 1.0)
    pub density: f32,
    /// Pattern changes per second (1.0 = once/sec, 8.0 = 8 times/sec)
    pub hz: f32,
    /// Seed for deterministic randomness
    pub seed: u64,
    /// Which braille patterns to use
    pub pattern: BraillePattern,
    /// Optional foreground color for the dust
    pub fg_color: Option<Color>,
    /// Drift in cells per step lifecycle. Positive = downward (gravity), negative = upward (sparks rising).
    /// The hash query shifts by this many rows over each particle's lifetime.
    pub drift: f32,
}

impl Default for BrailleDust {
    fn default() -> Self {
        Self {
            density: 0.03,
            hz: 8.0,
            seed: 42,
            pattern: BraillePattern::SingleDot,
            fg_color: None,
            drift: 0.0,
        }
    }
}

impl BrailleDust {
    /// Create a new braille dust filter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the density (fraction of empty cells to fill).
    pub fn with_density(mut self, density: f32) -> Self {
        self.density = density.clamp(0.0, 1.0);
        self
    }

    /// Set the animation rate in changes per second.
    pub fn with_hz(mut self, hz: f32) -> Self {
        self.hz = hz;
        self
    }

    /// Set the random seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    /// Set the braille pattern type.
    pub fn with_pattern(mut self, pattern: BraillePattern) -> Self {
        self.pattern = pattern;
        self
    }

    /// Set the foreground color for dust particles.
    pub fn with_fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Set drift in cells per step lifecycle.
    /// Positive = downward (gravity), negative = upward (sparks rising).
    pub fn with_drift(mut self, drift: f32) -> Self {
        self.drift = drift;
        self
    }

    /// Check if a cell is considered empty.
    #[inline]
    fn is_cell_empty(cell: &Cell) -> bool {
        cell.ch.is_whitespace() || cell.ch == '\u{2800}'
    }

    /// Per-cell time offset for desynchronized particle updates.
    /// Different cells cross their step boundary at different times,
    /// so particles appear and disappear independently rather than
    /// all flashing in unison.
    #[inline]
    fn cell_time_offset(&self, x: u16, y: u16) -> f64 {
        let position_hash =
            (x as u64).wrapping_mul(374761393) ^ (y as u64).wrapping_mul(668265263);
        let offset_seed = position_hash ^ self.seed.wrapping_mul(2654435761);
        // Offset within one step period (0.0 to 1.0/hz seconds)
        (offset_seed % 1000) as f64 / 1000.0 / self.hz.max(0.1) as f64
    }

    /// Generate deterministic noise for a position and time.
    /// Each cell has a per-cell time offset so particles desynchronize.
    #[inline]
    fn noise(&self, x: u16, y: u16, t: f64) -> f32 {
        // Per-cell time offset desynchronizes step transitions
        let cell_t = t + self.cell_time_offset(x, y);

        // Convert time to discrete steps based on hz (changes per second)
        let time_component = (cell_t * self.hz as f64).floor() as u64;

        // Pack x,y into a single value with good bit distribution
        let position_hash =
            (x as u64).wrapping_mul(374761393) ^ (y as u64).wrapping_mul(668265263);

        // Combine with seed and time
        let input = self
            .seed
            .wrapping_add(position_hash)
            .wrapping_add(time_component.wrapping_mul(3935559000370003845));

        fast_random(self.seed, input)
    }

    /// Returns a fade envelope (0.0→1.0→0.0) for the current position
    /// within its time step. Particles smoothly fade in and out instead
    /// of snapping on/off.
    #[inline]
    fn step_fade(&self, x: u16, y: u16, t: f64) -> f32 {
        let cell_t = t + self.cell_time_offset(x, y);
        let fract = (cell_t * self.hz as f64).fract() as f32;
        // Smooth bell curve: sin(π * fract) — 0 at edges, 1 at center
        (fract * std::f32::consts::PI).sin()
    }

    /// Get a random braille character based on noise value.
    fn braille_char(&self, noise: f32) -> char {
        match self.pattern {
            BraillePattern::SingleDot => {
                // Pick any of the 8 single-dot patterns
                braille::random_with_count(1, noise)
            }
            BraillePattern::OneToTwoDots => {
                // 1-2 dots, weighted toward fewer
                braille::random_up_to_count(2, noise)
            }
            BraillePattern::OneToThreeDots => {
                // 1-3 dots, weighted toward fewer
                braille::random_up_to_count(3, noise)
            }
            BraillePattern::OneToFourDots => {
                // 1-4 dots, weighted toward fewer
                braille::random_up_to_count(4, noise)
            }
        }
    }
}

impl Filter for BrailleDust {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, _width: u16, _height: u16, t: f64) {
        // Only affect empty cells
        if !Self::is_cell_empty(cell) {
            return;
        }

        // Drift: shift the query position over the step lifecycle.
        // The noise is evaluated at the "source" position, which drifts
        // over time. A particle born at row 5 will appear at row 5, then
        // row 6, then row 7 as the step progresses — faking gravity.
        let (query_x, query_y) = if self.drift.abs() > 0.001 {
            let cell_t = t + self.cell_time_offset(x, y);
            let step_progress = (cell_t * self.hz as f64).fract() as f32;
            let drift_offset = (self.drift * step_progress).round() as i32;
            (x, (y as i32 - drift_offset).max(0) as u16)
        } else {
            (x, y)
        };

        // Generate noise at the drifted source position (desynchronized per cell)
        let noise = self.noise(query_x, query_y, t);

        // Check if this cell should have dust (stochastic threshold)
        if noise > (1.0 - self.density) {
            // Fade envelope: 0→1→0 over each step, so particles smoothly appear/disappear
            let fade = self.step_fade(x, y, t);

            // Skip rendering if fade is too low (effectively invisible)
            if fade < 0.05 {
                return;
            }

            // Pick braille character
            let char_noise = self.noise(query_x.wrapping_add(1000), query_y.wrapping_add(1000), t);
            cell.ch = self.braille_char(char_noise);

            // Apply foreground color dimmed by fade envelope for smooth fade-in/fade-out
            if let Some(fg) = self.fg_color {
                cell.fg = Color::rgb(
                    (fg.r as f32 * fade) as u8,
                    (fg.g as f32 * fade) as u8,
                    (fg.b as f32 * fade) as u8,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creates_valid_filter() {
        let dust = BrailleDust::default();
        assert_eq!(dust.density, 0.03);
        assert_eq!(dust.hz, 8.0);
        assert_eq!(dust.pattern, BraillePattern::SingleDot);
    }

    #[test]
    fn test_only_affects_empty_cells() {
        let dust = BrailleDust::new().with_density(1.0); // 100% density

        // Cell with content should not be modified
        let mut cell_with_content = Cell::new('X');
        dust.apply(&mut cell_with_content, 0, 0, 10, 10, 0.5);
        assert_eq!(cell_with_content.ch, 'X');

        // Empty cell should be filled
        let mut empty_cell = Cell::default();
        dust.apply(&mut empty_cell, 0, 0, 10, 10, 0.5);
        assert_ne!(empty_cell.ch, ' ');
    }

    #[test]
    fn test_density_affects_frequency() {
        let low_dust = BrailleDust::new().with_density(0.01);
        let high_dust = BrailleDust::new().with_density(0.20);

        let mut low_count = 0;
        let mut high_count = 0;

        for y in 0..100 {
            for x in 0..100 {
                let mut low_cell = Cell::default();
                low_dust.apply(&mut low_cell, x, y, 100, 100, 0.5);
                if low_cell.ch != ' ' {
                    low_count += 1;
                }

                let mut high_cell = Cell::default();
                high_dust.apply(&mut high_cell, x, y, 100, 100, 0.5);
                if high_cell.ch != ' ' {
                    high_count += 1;
                }
            }
        }

        assert!(
            high_count > low_count * 5,
            "High density ({}) should have much more dust than low density ({})",
            high_count,
            low_count
        );
    }

    #[test]
    fn test_braille_characters_are_valid() {
        let dust = BrailleDust::new().with_density(1.0);

        let mut cell = Cell::default();
        dust.apply(&mut cell, 5, 5, 10, 10, 0.5);

        // Should be a valid braille character
        assert!(
            braille::braille_bits(cell.ch).is_some(),
            "Character {:?} is not a braille character",
            cell.ch
        );
    }

    #[test]
    fn test_different_patterns_produce_different_chars() {
        let patterns = [
            BraillePattern::SingleDot,
            BraillePattern::OneToTwoDots,
            BraillePattern::OneToThreeDots,
            BraillePattern::OneToFourDots,
        ];

        for pattern in patterns {
            let dust = BrailleDust::new().with_density(1.0).with_pattern(pattern);

            let mut cell = Cell::default();
            dust.apply(&mut cell, 0, 0, 10, 10, 0.5);

            // All should produce valid braille
            assert!(
                braille::braille_bits(cell.ch).is_some(),
                "Pattern {:?} produced non-braille character",
                pattern
            );
        }
    }

    #[test]
    fn test_fg_color_applied() {
        let dust = BrailleDust::new()
            .with_density(1.0)
            .with_fg(Color::rgb(100, 100, 100));

        let mut cell = Cell::default().with_fg(Color::WHITE);

        dust.apply(&mut cell, 0, 0, 10, 10, 0.5);

        // Color is dimmed by the fade envelope — verify it's set (non-white)
        // and in the right color family (gray, not the original white)
        assert_ne!(cell.fg, Color::WHITE, "fg should be changed from default");
        assert_eq!(cell.fg.r, cell.fg.g, "gray color should have equal channels");
        assert!(cell.fg.r <= 100, "dimmed color should not exceed base color");
    }

    #[test]
    fn test_animation_changes_over_time() {
        let dust = BrailleDust::new().with_density(1.0).with_hz(1.0);

        let mut chars_t0 = Vec::new();
        let mut chars_t1 = Vec::new();

        for y in 0..10 {
            for x in 0..10 {
                let mut cell = Cell::default();
                dust.apply(&mut cell, x, y, 10, 10, 0.0);
                chars_t0.push(cell.ch);

                let mut cell = Cell::default();
                dust.apply(&mut cell, x, y, 10, 10, 1.0);
                chars_t1.push(cell.ch);
            }
        }

        // At different times, the pattern should be different
        assert_ne!(chars_t0, chars_t1, "Pattern should change over time");
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_braille_dust.rs</FILE> - <DESC>Stochastic braille dust filter for frosted glass texture</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
