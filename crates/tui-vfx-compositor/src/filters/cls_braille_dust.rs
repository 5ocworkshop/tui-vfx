// <FILE>tui-vfx-compositor/src/filters/cls_braille_dust.rs</FILE> - <DESC>Stochastic braille dust filter for frosted glass texture</DESC>
// <VERS>VERSION: 1.1.1</VERS>
// <WCTX>Clippy cleanup</WCTX>
// <CLOG>Remove field reassignments after Cell::default in tests</CLOG>

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
}

impl Default for BrailleDust {
    fn default() -> Self {
        Self {
            density: 0.03,
            hz: 8.0,
            seed: 42,
            pattern: BraillePattern::SingleDot,
            fg_color: None,
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

    /// Check if a cell is considered empty.
    #[inline]
    fn is_cell_empty(cell: &Cell) -> bool {
        cell.ch.is_whitespace()
    }

    /// Generate deterministic noise for a position and time.
    #[inline]
    fn noise(&self, x: u16, y: u16, t: f64) -> f32 {
        // Convert time to discrete steps based on hz (changes per second)
        let time_component = (t * self.hz as f64).floor() as u64;

        // Pack x,y into a single value with good bit distribution
        let position_hash = (x as u64).wrapping_mul(374761393) ^ (y as u64).wrapping_mul(668265263);

        // Combine with seed and time
        let input = self
            .seed
            .wrapping_add(position_hash)
            .wrapping_add(time_component.wrapping_mul(3935559000370003845));

        fast_random(self.seed, input)
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

        // Generate noise for this position and time
        let noise = self.noise(x, y, t);

        // Check if this cell should have dust (stochastic threshold)
        if noise > (1.0 - self.density) {
            // Use a second noise value to pick the braille character
            let char_noise = self.noise(x.wrapping_add(1000), y.wrapping_add(1000), t);
            cell.ch = self.braille_char(char_noise);

            // Apply foreground color if specified
            if let Some(fg) = self.fg_color {
                cell.fg = fg;
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

        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
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
// <VERS>END OF VERSION: 1.1.1</VERS>
