// <FILE>tui-vfx-compositor/src/filters/cls_charset_noise.rs</FILE> - <DESC>Non-converging time-varying character replacement filter with vertical gradient</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>feat/content-ergonomics: clean up pre-existing workspace clippy lint</WCTX>
// <CLOG>Use struct-init form in test make_cell helper (clippy::field_reassign_with_default)</CLOG>

use crate::traits::filter::Filter;
use mixed_signals::random::hash_to_index;
use tui_vfx_types::Cell;

/// A single stop in a vertical charset gradient.
#[derive(Debug, Clone)]
pub struct CharsetGradientStop {
    /// Normalized vertical position (0.0 = top, 1.0 = bottom).
    pub at: f32,
    /// Pool of characters at this position.
    pub chars: Vec<char>,
}

/// Controls which cells the filter affects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AffectMode {
    /// Replace all cells (including whitespace).
    All,
    /// Replace only non-whitespace cells (spaces and empty braille ⠀ are skipped).
    #[default]
    NonEmpty,
}

/// Non-converging per-cell character replacement filter.
///
/// Replaces cell characters from a position-aware charset gradient that
/// changes over time. Each 1/hz seconds, every affected cell gets a fresh
/// deterministic pick from its position-appropriate charset pool.
///
/// Including empty characters (like ⠀) in sparse pools causes cells to
/// flicker between visible and invisible, making shape boundaries fluctuate.
///
/// This is a compositor filter — it operates on the rendered cell grid
/// alongside masks, shaders, and other filters. It chains naturally in
/// the filter array (e.g., charset_noise first to mutate characters, then
/// braille_dust to fill gaps, then tint for color).
pub struct CharsetNoise {
    seed: u64,
    hz: f32,
    jitter: f32,
    affect: AffectMode,
    gradient: Vec<CharsetGradientStop>,
}

impl CharsetNoise {
    pub fn new(
        seed: u64,
        hz: f32,
        jitter: f32,
        affect: AffectMode,
        gradient: Vec<CharsetGradientStop>,
    ) -> Self {
        let mut gradient = gradient;
        gradient.sort_by(|a, b| a.at.partial_cmp(&b.at).unwrap_or(std::cmp::Ordering::Equal));
        Self {
            seed,
            hz: hz.max(0.1),
            jitter: jitter.clamp(0.0, 1.0),
            affect,
            gradient,
        }
    }

    /// Per-cell time offset for desynchronized updates (same pattern as braille_dust).
    #[inline]
    fn cell_time_offset(&self, x: u16, y: u16) -> f64 {
        let position_hash = (x as u64).wrapping_mul(374761393) ^ (y as u64).wrapping_mul(668265263);
        let offset_seed = position_hash ^ self.seed.wrapping_mul(2654435761);
        (offset_seed % 1000) as f64 / 1000.0 / self.hz.max(0.1) as f64
    }

    /// Cheap deterministic hash for per-cell variation.
    #[inline]
    fn cell_hash(&self, x: u16, y: u16, time_step: u64) -> u64 {
        let mut h = self
            .seed
            .wrapping_mul(2654435761)
            .wrapping_add(y as u64 * 131)
            .wrapping_add(x as u64 * 997)
            .wrapping_add(time_step.wrapping_mul(7919));
        h ^= h >> 16;
        h = h.wrapping_mul(0x45d9f3b);
        h ^= h >> 16;
        h
    }

    /// Select the charset pool for a given effective vertical position.
    #[inline]
    fn pool_at(&self, effective_pos: f32) -> &[char] {
        if self.gradient.len() == 1 {
            return &self.gradient[0].chars;
        }
        let pos = effective_pos.clamp(0.0, 1.0);
        let mut best_idx = 0;
        let mut best_dist = f32::MAX;
        for (i, stop) in self.gradient.iter().enumerate() {
            let dist = (stop.at - pos).abs();
            if dist < best_dist {
                best_dist = dist;
                best_idx = i;
            }
        }
        &self.gradient[best_idx].chars
    }

    /// Check whether a cell should be affected.
    #[inline]
    fn should_affect(&self, cell: &Cell) -> bool {
        match self.affect {
            AffectMode::All => true,
            AffectMode::NonEmpty => !cell.ch.is_whitespace() && cell.ch != '\u{2800}',
        }
    }
}

impl Filter for CharsetNoise {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, _width: u16, height: u16, t: f64) {
        if self.gradient.is_empty() || !self.should_affect(cell) {
            return;
        }

        // Per-cell desynchronized time step
        let cell_t = t + self.cell_time_offset(x, y);
        let time_step = (cell_t * self.hz as f64).floor() as u64;

        let h = self.cell_hash(x, y, time_step);

        // Vertical position with per-cell jitter
        let v_pos = if height > 1 {
            y as f32 / (height - 1) as f32
        } else {
            0.5
        };
        let jitter_offset = if self.jitter > 0.0 {
            ((h % 101) as f32 / 100.0 - 0.5) * 2.0 * self.jitter
        } else {
            0.0
        };
        let effective_pos = (v_pos + jitter_offset).clamp(0.0, 1.0);

        // Pick character from the gradient pool
        let pool = self.pool_at(effective_pos);
        if pool.is_empty() {
            return;
        }
        let char_idx = hash_to_index(
            self.seed.wrapping_add(time_step),
            x as u64 + y as u64 * 1000,
            pool.len(),
        );
        cell.ch = pool[char_idx];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cell(ch: char) -> Cell {
        Cell {
            ch,
            ..Cell::default()
        }
    }

    fn simple_gradient() -> Vec<CharsetGradientStop> {
        vec![
            CharsetGradientStop {
                at: 0.0,
                chars: vec!['A', 'B'],
            },
            CharsetGradientStop {
                at: 1.0,
                chars: vec!['Y', 'Z'],
            },
        ]
    }

    #[test]
    fn non_empty_cells_get_replaced() {
        let filter = CharsetNoise::new(42, 8.0, 0.0, AffectMode::NonEmpty, simple_gradient());
        let mut cell = make_cell('⣿');
        filter.apply(&mut cell, 0, 0, 10, 10, 500.0);
        assert!(
            cell.ch == 'A' || cell.ch == 'B',
            "Top cell should use first gradient stop, got {:?}",
            cell.ch
        );
    }

    #[test]
    fn empty_cells_skipped() {
        let filter = CharsetNoise::new(42, 8.0, 0.0, AffectMode::NonEmpty, simple_gradient());
        let mut cell = make_cell('⠀');
        filter.apply(&mut cell, 0, 0, 10, 10, 500.0);
        assert_eq!(cell.ch, '⠀', "Empty braille should be skipped");
    }

    #[test]
    fn space_cells_skipped() {
        let filter = CharsetNoise::new(42, 8.0, 0.0, AffectMode::NonEmpty, simple_gradient());
        let mut cell = make_cell(' ');
        filter.apply(&mut cell, 0, 0, 10, 10, 500.0);
        assert_eq!(cell.ch, ' ', "Space should be skipped");
    }

    #[test]
    fn all_mode_replaces_everything() {
        let filter = CharsetNoise::new(42, 8.0, 0.0, AffectMode::All, simple_gradient());
        let mut cell = make_cell(' ');
        filter.apply(&mut cell, 0, 0, 10, 10, 500.0);
        assert!(cell.ch == 'A' || cell.ch == 'B');
    }

    #[test]
    fn gradient_top_vs_bottom() {
        let filter = CharsetNoise::new(42, 8.0, 0.0, AffectMode::NonEmpty, simple_gradient());
        let mut top = make_cell('⣿');
        let mut bottom = make_cell('⣿');
        filter.apply(&mut top, 5, 0, 10, 10, 500.0);
        filter.apply(&mut bottom, 5, 9, 10, 10, 500.0);
        assert!(
            top.ch == 'A' || top.ch == 'B',
            "Top row should use first stop"
        );
        assert!(
            bottom.ch == 'Y' || bottom.ch == 'Z',
            "Bottom row should use last stop"
        );
    }

    #[test]
    fn different_time_different_output() {
        let filter = CharsetNoise::new(
            42,
            8.0,
            0.0,
            AffectMode::NonEmpty,
            vec![CharsetGradientStop {
                at: 0.0,
                chars: "ABCDEFGHIJ".chars().collect(),
            }],
        );
        // Collect outputs across many time values — should not all be identical
        let mut results = std::collections::HashSet::new();
        for t_ms in (0..10000).step_by(200) {
            let mut cell = make_cell('⣿');
            filter.apply(&mut cell, 3, 3, 10, 10, t_ms as f64);
            results.insert(cell.ch);
        }
        assert!(
            results.len() > 1,
            "Different time steps should produce different characters, got {:?}",
            results
        );
    }

    #[test]
    fn deterministic() {
        let filter = CharsetNoise::new(42, 8.0, 0.0, AffectMode::NonEmpty, simple_gradient());
        let mut cell1 = make_cell('⣿');
        let mut cell2 = make_cell('⣿');
        filter.apply(&mut cell1, 3, 3, 10, 10, 500.0);
        filter.apply(&mut cell2, 3, 3, 10, 10, 500.0);
        assert_eq!(cell1.ch, cell2.ch, "Same inputs must produce same output");
    }

    #[test]
    fn empty_gradient_is_noop() {
        let filter = CharsetNoise::new(42, 8.0, 0.0, AffectMode::NonEmpty, vec![]);
        let mut cell = make_cell('⣿');
        filter.apply(&mut cell, 0, 0, 10, 10, 500.0);
        assert_eq!(cell.ch, '⣿');
    }

    #[test]
    fn jitter_creates_variation() {
        let filter = CharsetNoise::new(42, 8.0, 0.5, AffectMode::NonEmpty, simple_gradient());
        let mut results = std::collections::HashSet::new();
        // Sample cells across middle rows — jitter should push some to different stops
        for x in 0..20u16 {
            let mut cell = make_cell('⣿');
            filter.apply(&mut cell, x, 5, 20, 10, 500.0);
            results.insert(cell.ch);
        }
        assert!(
            results.len() > 2,
            "Jitter should produce chars from multiple gradient stops"
        );
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_charset_noise.rs</FILE> - <DESC>Non-converging time-varying character replacement filter with vertical gradient</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
