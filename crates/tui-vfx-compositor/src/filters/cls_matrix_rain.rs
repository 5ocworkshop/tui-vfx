// <FILE>tui-vfx-compositor/src/filters/cls_matrix_rain.rs</FILE> - <DESC>Deterministic procedural digital-rain filter</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Slice 6.6 §F.5 — migrate Filter trait to VfxCellContext bundle</WCTX>
// <CLOG>0.1.1: migrate apply signature to &VfxCellContext.</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color, VfxCellContext};

const DEFAULT_MATRIX_GLYPHS: &str = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎ0123456789@#$%&*+=-<>!?";
const DEFAULT_BINARY_GLYPHS: &str = "01";
const DEFAULT_HEX_GLYPHS: &str = "0123456789ABCDEF";
const DEFAULT_ASCII_GLYPHS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Which cells MatrixRain should overwrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatrixRainAffectMode {
    /// Replace every cell in the target region.
    #[default]
    All,
    /// Only overwrite blank/whitespace cells.
    OnlyBlank,
}

/// Preset glyph alphabets for MatrixRain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatrixRainGlyphPreset {
    /// Half-width katakana + digits/symbols; safest Matrix-like default.
    #[default]
    Matrix,
    /// Binary digits only.
    Binary,
    /// Hex characters only.
    Hex,
    /// Uppercase ASCII letters and digits.
    Ascii,
}

/// Rendering behavior for MatrixRain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatrixRainMode {
    /// Column-coherent falling streams (modern interpretation).
    #[default]
    Modern,
    /// Stationary glyph field with illumination waves (classic film-inspired look).
    Classic,
}

/// Deterministic procedural digital-rain filter.
///
/// The filter treats each screen column as an independently-parameterized
/// falling stream chosen from a deterministic hash of `(x, seed)`. Rather
/// than storing mutable column state, it derives stream presence, speed,
/// trail length, head position, and glyph churn directly from `(x, y, t)`,
/// which keeps it compatible with the compositor's stateless per-cell filter
/// API while still reading as coherent columnar rain.
pub struct MatrixRain {
    pub mode: MatrixRainMode,
    pub density: f32,
    pub speed_multiplier: f32,
    pub speed_min: f32,
    pub speed_max: f32,
    pub trail_min: u16,
    pub trail_max: u16,
    pub glyph_change_hz: f32,
    pub head_color: Color,
    pub tail_color: Color,
    pub seed: u64,
    pub affect: MatrixRainAffectMode,
    glyphs: Vec<char>,
}

impl Default for MatrixRain {
    fn default() -> Self {
        Self {
            mode: MatrixRainMode::Modern,
            density: 0.5,
            speed_multiplier: 1.0,
            speed_min: 5.0,
            speed_max: 15.0,
            trail_min: 8,
            trail_max: 20,
            glyph_change_hz: 8.0,
            head_color: Color::rgb(220, 255, 220),
            tail_color: Color::rgb(0, 160, 0),
            seed: 42,
            affect: MatrixRainAffectMode::All,
            glyphs: DEFAULT_MATRIX_GLYPHS.chars().collect(),
        }
    }
}

impl MatrixRain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_density(mut self, density: f32) -> Self {
        self.density = density.clamp(0.0, 1.0);
        self
    }

    pub fn with_mode(mut self, mode: MatrixRainMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_speed_multiplier(mut self, speed_multiplier: f32) -> Self {
        self.speed_multiplier = speed_multiplier.max(0.0);
        self
    }

    pub fn with_speed_range(mut self, min: f32, max: f32) -> Self {
        let min = min.max(0.1);
        let max = max.max(min);
        self.speed_min = min;
        self.speed_max = max;
        self
    }

    pub fn with_trail_range(mut self, min: u16, max: u16) -> Self {
        let min = min.max(1);
        let max = max.max(min);
        self.trail_min = min;
        self.trail_max = max;
        self
    }

    pub fn with_glyph_change_hz(mut self, glyph_change_hz: f32) -> Self {
        self.glyph_change_hz = glyph_change_hz.max(0.1);
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_head_color(mut self, head_color: Color) -> Self {
        self.head_color = head_color;
        self
    }

    pub fn with_tail_color(mut self, tail_color: Color) -> Self {
        self.tail_color = tail_color;
        self
    }

    pub fn with_affect(mut self, affect: MatrixRainAffectMode) -> Self {
        self.affect = affect;
        self
    }

    pub fn with_preset(mut self, preset: MatrixRainGlyphPreset) -> Self {
        self.glyphs = match preset {
            MatrixRainGlyphPreset::Matrix => DEFAULT_MATRIX_GLYPHS,
            MatrixRainGlyphPreset::Binary => DEFAULT_BINARY_GLYPHS,
            MatrixRainGlyphPreset::Hex => DEFAULT_HEX_GLYPHS,
            MatrixRainGlyphPreset::Ascii => DEFAULT_ASCII_GLYPHS,
        }
        .chars()
        .collect();
        self
    }

    pub fn with_custom_glyphs(mut self, glyphs: impl Into<String>) -> Self {
        let glyphs: Vec<char> = glyphs.into().chars().collect();
        if !glyphs.is_empty() {
            self.glyphs = glyphs;
        }
        self
    }

    #[inline]
    fn should_affect(&self, cell: &Cell) -> bool {
        match self.affect {
            MatrixRainAffectMode::All => true,
            MatrixRainAffectMode::OnlyBlank => cell.ch.is_whitespace() || cell.ch == '\u{2800}',
        }
    }

    #[inline]
    fn hash(&self, a: u64, b: u64, c: u64) -> u64 {
        let mut h = self.seed
            ^ a.wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ b.wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
            ^ c.wrapping_mul(0x1656_67B1_9E37_79F9);
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd);
        h ^= h >> 33;
        h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
        h ^= h >> 33;
        h
    }

    #[inline]
    fn hash_unit(&self, a: u64, b: u64, c: u64) -> f32 {
        (self.hash(a, b, c) % 10_000) as f32 / 10_000.0
    }

    #[inline]
    fn column_active(&self, x: u16) -> bool {
        self.hash_unit(x as u64, 0, 0) < self.density
    }

    #[inline]
    fn column_speed(&self, x: u16) -> f32 {
        let span = (self.speed_max - self.speed_min).max(0.0);
        let authored_speed = self.speed_min + (span * self.hash_unit(x as u64, 1, 0));
        authored_speed * self.speed_multiplier.max(0.0)
    }

    #[inline]
    fn column_trail(&self, x: u16) -> u16 {
        let span = self.trail_max.saturating_sub(self.trail_min);
        self.trail_min + ((span as f32) * self.hash_unit(x as u64, 2, 0)).round() as u16
    }

    #[inline]
    fn column_phase(&self, x: u16) -> f32 {
        self.hash_unit(x as u64, 3, 0)
    }

    #[inline]
    fn head_position(&self, x: u16, height: u16, t: f64) -> f32 {
        let trail = self.column_trail(x) as f32;
        let travel = height as f32 + (trail * 2.0) + 2.0;
        let progress = ((self.column_phase(x) + ((t as f32) * self.column_speed(x))) % 1.0).abs();
        progress * travel - trail
    }

    #[inline]
    fn glyph_at(&self, x: u16, y: u16, distance_from_head: u16, t: f64) -> char {
        let step = ((t as f32) * self.glyph_change_hz.max(0.1) * 64.0).floor() as u64;
        let idx = (self.hash(x as u64, y as u64, step + distance_from_head as u64)
            % self.glyphs.len() as u64) as usize;
        self.glyphs[idx]
    }

    #[inline]
    fn classic_glyph_at(&self, x: u16, y: u16, t: f64) -> char {
        // Approximate "some glyphs remain static for 3 frames" by using a
        // globally-aligned time step with a slower cadence than the modern stream churn.
        let step = ((t as f32) * self.glyph_change_hz.max(0.1) * 3.0).floor() as u64;
        let idx = (self.hash(x as u64, y as u64, step) % self.glyphs.len() as u64) as usize;
        self.glyphs[idx]
    }

    #[inline]
    fn lerp_color(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color::rgb(
            ((a.r as f32) + ((b.r as f32 - a.r as f32) * t)).round() as u8,
            ((a.g as f32) + ((b.g as f32 - a.g as f32) * t)).round() as u8,
            ((a.b as f32) + ((b.b as f32 - a.b as f32) * t)).round() as u8,
        )
    }
}

impl Filter for MatrixRain {
    fn apply(&self, cell: &mut Cell, ctx: &VfxCellContext) {
        let x = ctx.local_x;
        let y = ctx.local_y;
        let height = ctx.height;
        let t = ctx.t;
        if self.glyphs.is_empty() || !self.should_affect(cell) || !self.column_active(x) {
            return;
        }

        let trail = self.column_trail(x).max(1);
        let head = self.head_position(x, height.max(1), t);
        let distance = head - y as f32;
        match self.mode {
            MatrixRainMode::Modern => {
                if !(0.0..(trail as f32)).contains(&distance) {
                    return;
                }

                let distance_from_head = distance.floor() as u16;
                let brightness = 1.0 - (distance / trail as f32);
                cell.ch = self.glyph_at(x, y, distance_from_head, t);
                cell.fg = Self::lerp_color(self.tail_color, self.head_color, brightness.powf(0.75));
            }
            MatrixRainMode::Classic => {
                // Fixed grid: glyphs stay in their cells, but only the active string window is visible.
                // Motion is an illumination wave / visible-string front, not literal glyph descent.
                let cycle_distance = distance.rem_euclid(height.max(1) as f32 + trail as f32);
                let visibility_step = ((t as f32) * 2.0).floor() as u64;
                let visible_ratio = 0.35 + 0.35 * self.hash_unit(x as u64, 7, visibility_step);
                let visible_length = (trail as f32 * visible_ratio).max(3.0);
                if cycle_distance >= visible_length {
                    return;
                }
                let brightness = 1.0 - (cycle_distance / visible_length);
                cell.ch = self.classic_glyph_at(x, y, t);
                let highlighted = self.hash_unit(x as u64, 9, 0) < 0.2 && cycle_distance < 1.0;
                let head_color = if highlighted {
                    Self::lerp_color(self.head_color, Color::rgb(255, 255, 255), 0.35)
                } else {
                    self.head_color
                };
                cell.fg = Self::lerp_color(self.tail_color, head_color, brightness.powf(0.75));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blank() -> Cell {
        Cell::default()
    }

    #[test]
    fn density_zero_renders_nothing() {
        let filter = MatrixRain::new().with_density(0.0);
        for x in 0..32 {
            for y in 0..16 {
                let mut cell = blank();
                filter.apply(&mut cell, &VfxCellContext::new(x, y, 32, 16, 0, 0, 0.5));
                assert_eq!(cell.ch, ' ');
            }
        }
    }

    #[test]
    fn deterministic_for_same_inputs() {
        let filter = MatrixRain::new().with_density(1.0).with_seed(7);
        let mut a = blank();
        let mut b = blank();
        filter.apply(&mut a, &VfxCellContext::new(3, 4, 20, 10, 0, 0, 0.42));
        filter.apply(&mut b, &VfxCellContext::new(3, 4, 20, 10, 0, 0, 0.42));
        assert_eq!(a.ch, b.ch);
        assert_eq!(a.fg, b.fg);
    }

    #[test]
    fn time_changes_stream_output() {
        let filter = MatrixRain::new().with_density(1.0).with_seed(9);
        let mut seen = std::collections::HashSet::new();
        for t in [0.1, 0.25, 0.4, 0.6] {
            let mut cell = blank();
            filter.apply(&mut cell, &VfxCellContext::new(2, 4, 20, 10, 0, 0, t));
            seen.insert((cell.ch, cell.fg.g));
        }
        assert!(seen.len() > 1);
    }

    #[test]
    fn speed_multiplier_zero_stops_motion() {
        let filter = MatrixRain::new()
            .with_density(1.0)
            .with_seed(11)
            .with_speed_multiplier(0.0);
        let mut a = blank();
        let mut b = blank();
        filter.apply(&mut a, &VfxCellContext::new(3, 4, 20, 10, 0, 0, 0.1));
        filter.apply(&mut b, &VfxCellContext::new(3, 4, 20, 10, 0, 0, 0.9));
        assert_eq!(a.ch, b.ch);
        assert_eq!(a.fg, b.fg);
    }

    #[test]
    fn classic_mode_leaves_vertical_gaps() {
        let filter = MatrixRain::new()
            .with_density(1.0)
            .with_mode(MatrixRainMode::Classic)
            .with_seed(5);
        let mut rendered = 0usize;
        for y in 0..12 {
            let mut cell = blank();
            filter.apply(&mut cell, &VfxCellContext::new(2, y, 10, 12, 0, 0, 0.5));
            if cell.ch != ' ' {
                rendered += 1;
            }
        }
        assert!(rendered > 0);
        assert!(rendered < 12);
    }

    #[test]
    fn head_is_brighter_than_tail() {
        let filter = MatrixRain::new().with_density(1.0).with_seed(1);
        let x = 0;
        let height = 24;
        let t = 0.5;
        let mut rendered: Vec<(u16, Cell)> = Vec::new();
        for y in 0..height {
            let mut cell = blank();
            filter.apply(&mut cell, &VfxCellContext::new(x, y, 10, height, 0, 0, t));
            if cell.ch != ' ' {
                rendered.push((y, cell));
            }
        }
        assert!(rendered.len() >= 2);
        let tail = &rendered[0].1;
        let head = &rendered[rendered.len() - 1].1;
        assert!(head.fg.g >= tail.fg.g);
    }

    #[test]
    fn only_blank_respects_existing_content() {
        let filter = MatrixRain::new()
            .with_density(1.0)
            .with_affect(MatrixRainAffectMode::OnlyBlank);
        let mut cell = Cell::new('X');
        filter.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.5));
        assert_eq!(cell.ch, 'X');
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_matrix_rain.rs</FILE> - <DESC>Deterministic procedural digital-rain filter</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
