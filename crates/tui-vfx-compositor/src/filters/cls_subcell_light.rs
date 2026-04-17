// <FILE>crates/tui-vfx-compositor/src/filters/cls_subcell_light.rs</FILE>
// <DESC>Sub-cell light filter that renders light fields into partial-block or braille glyphs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Introduce a cell-mutation companion primitive for less-square light rendering on blank shell-owned cells</WCTX>
// <CLOG>Add SubcellLight filter with foreground/background sampling, braille/horizontal/vertical render modes, thresholding, and optional low-rate temporal dither</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubcellLightRenderMode {
    #[default]
    Braille,
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightSampleFrom {
    Foreground,
    #[default]
    Background,
}

/// Render a light field into sub-cell glyphs in blank cells.
///
/// This filter is intended as a companion to style shaders like ConcealedLight,
/// Diffusion, or FocusField. Those shaders shape the light field through color;
/// this filter can then reinterpret the resulting color into a finer sub-cell
/// glyph pattern in blank, shell-owned cells to reduce the perception of
/// square, cell-by-cell light.
pub struct SubcellLight {
    pub lit_color: Color,
    pub unlit_color: Color,
    pub render_mode: SubcellLightRenderMode,
    pub sample_from: LightSampleFrom,
    pub threshold: f32,
    pub temporal_dither_hz: f32,
    pub only_blank: bool,
}

const BRAILLE_BASE: u32 = 0x2800;
const BRAILLE_DOTS: [u8; 8] = [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];

impl Default for SubcellLight {
    fn default() -> Self {
        Self {
            lit_color: Color::rgb(220, 220, 220),
            unlit_color: Color::rgb(24, 24, 24),
            render_mode: SubcellLightRenderMode::Braille,
            sample_from: LightSampleFrom::Background,
            threshold: 0.06,
            temporal_dither_hz: 0.0,
            only_blank: true,
        }
    }
}

impl SubcellLight {
    fn sample_color(&self, cell: &Cell) -> Color {
        match self.sample_from {
            LightSampleFrom::Foreground => cell.fg,
            LightSampleFrom::Background => cell.bg,
        }
    }

    fn project_intensity(&self, sampled: Color) -> f32 {
        let base = [
            self.unlit_color.r as f32,
            self.unlit_color.g as f32,
            self.unlit_color.b as f32,
        ];
        let lit = [
            self.lit_color.r as f32,
            self.lit_color.g as f32,
            self.lit_color.b as f32,
        ];
        let sample = [sampled.r as f32, sampled.g as f32, sampled.b as f32];

        let axis = [lit[0] - base[0], lit[1] - base[1], lit[2] - base[2]];
        let axis_len_sq = axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2];
        if axis_len_sq <= f32::EPSILON {
            return 0.0;
        }

        let sample_delta = [
            sample[0] - base[0],
            sample[1] - base[1],
            sample[2] - base[2],
        ];
        let projected =
            (sample_delta[0] * axis[0] + sample_delta[1] * axis[1] + sample_delta[2] * axis[2])
                / axis_len_sq;
        projected.clamp(0.0, 1.0)
    }

    fn horizontal_partial(sub_index: u8) -> char {
        match sub_index {
            0 => ' ',
            1 => '▏',
            2 => '▎',
            3 => '▍',
            4 => '▌',
            5 => '▋',
            6 => '▊',
            7 => '▉',
            _ => '█',
        }
    }

    fn vertical_partial(sub_index: u8) -> char {
        match sub_index {
            0 => ' ',
            1 => '▁',
            2 => '▂',
            3 => '▃',
            4 => '▄',
            5 => '▅',
            6 => '▆',
            7 => '▇',
            _ => '█',
        }
    }

    fn rotated_braille_pattern(&self, dots_to_fill: usize, x: u16, y: u16, t: f64) -> char {
        let time_step = if self.temporal_dither_hz > 0.0 {
            (t * self.temporal_dither_hz as f64).floor() as u32
        } else {
            0
        };
        let rotation = ((x as u32)
            .wrapping_mul(37)
            .wrapping_add((y as u32).wrapping_mul(67))
            .wrapping_add(time_step))
            % 8;

        let mut pattern = 0_u8;
        for idx in 0..dots_to_fill.min(8) {
            let dot = BRAILLE_DOTS[((idx as u32 + rotation) % 8) as usize];
            pattern |= dot;
        }
        char::from_u32(BRAILLE_BASE + pattern as u32).unwrap_or(' ')
    }
}

impl Filter for SubcellLight {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, _width: u16, _height: u16, t: f64) {
        if self.only_blank && cell.ch != ' ' {
            return;
        }

        let sampled = self.sample_color(cell);
        if sampled.a == 0 {
            return;
        }

        let intensity = self.project_intensity(sampled);
        if intensity <= self.threshold {
            return;
        }

        let eighths = ((intensity * 8.0).round().clamp(0.0, 8.0) as u8).max(1);
        cell.ch = match self.render_mode {
            SubcellLightRenderMode::Braille => self.rotated_braille_pattern(eighths as usize, x, y, t),
            SubcellLightRenderMode::Horizontal => Self::horizontal_partial(eighths),
            SubcellLightRenderMode::Vertical => Self::vertical_partial(eighths),
        };
        cell.fg = self.lit_color;
        cell.bg = self.unlit_color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    fn make_cell(ch: char, fg: Color, bg: Color) -> Cell {
        Cell::styled(ch, fg, bg, Modifiers::NONE)
    }

    #[test]
    fn default_values_are_sensible() {
        let filter = SubcellLight::default();
        assert_eq!(filter.render_mode, SubcellLightRenderMode::Braille);
        assert_eq!(filter.sample_from, LightSampleFrom::Background);
        assert!(filter.only_blank);
    }

    #[test]
    fn only_blank_skips_nonblank_cells() {
        let filter = SubcellLight::default();
        let mut cell = make_cell('X', Color::WHITE, Color::rgb(100, 100, 100));
        let original = cell;
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);
        assert_eq!(cell, original);
    }

    #[test]
    fn background_sample_creates_partial_glyph() {
        let filter = SubcellLight {
            lit_color: Color::rgb(200, 200, 200),
            unlit_color: Color::rgb(20, 20, 20),
            render_mode: SubcellLightRenderMode::Horizontal,
            sample_from: LightSampleFrom::Background,
            threshold: 0.0,
            temporal_dither_hz: 0.0,
            only_blank: true,
        };
        let mut cell = make_cell(' ', Color::TRANSPARENT, Color::rgb(110, 110, 110));
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);
        assert_ne!(cell.ch, ' ');
        assert_eq!(cell.fg, Color::rgb(200, 200, 200));
        assert_eq!(cell.bg, Color::rgb(20, 20, 20));
    }

    #[test]
    fn foreground_sample_works_on_blank_cell() {
        let filter = SubcellLight {
            lit_color: Color::rgb(220, 220, 220),
            unlit_color: Color::rgb(30, 30, 30),
            render_mode: SubcellLightRenderMode::Vertical,
            sample_from: LightSampleFrom::Foreground,
            threshold: 0.0,
            temporal_dither_hz: 0.0,
            only_blank: true,
        };
        let mut cell = make_cell(' ', Color::rgb(140, 140, 140), Color::rgb(30, 30, 30));
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);
        assert_ne!(cell.ch, ' ');
    }

    #[test]
    fn below_threshold_leaves_cell_unchanged() {
        let filter = SubcellLight {
            threshold: 0.5,
            ..Default::default()
        };
        let mut cell = make_cell(' ', Color::TRANSPARENT, Color::rgb(40, 40, 40));
        let original = cell;
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);
        assert_eq!(cell, original);
    }

    #[test]
    fn braille_mode_is_deterministic_without_temporal_dither() {
        let filter = SubcellLight {
            threshold: 0.0,
            temporal_dither_hz: 0.0,
            ..Default::default()
        };
        let mut a = make_cell(' ', Color::TRANSPARENT, Color::rgb(120, 120, 120));
        let mut b = make_cell(' ', Color::TRANSPARENT, Color::rgb(120, 120, 120));
        filter.apply(&mut a, 3, 2, 10, 5, 0.0);
        filter.apply(&mut b, 3, 2, 10, 5, 1.0);
        assert_eq!(a.ch, b.ch);
    }

    #[test]
    fn temporal_dither_changes_braille_pattern_over_time() {
        let filter = SubcellLight {
            threshold: 0.0,
            temporal_dither_hz: 2.0,
            ..Default::default()
        };
        let mut a = make_cell(' ', Color::TRANSPARENT, Color::rgb(120, 120, 120));
        let mut b = make_cell(' ', Color::TRANSPARENT, Color::rgb(120, 120, 120));
        filter.apply(&mut a, 3, 2, 10, 5, 0.0);
        filter.apply(&mut b, 3, 2, 10, 5, 1.0);
        assert_ne!(a.ch, b.ch);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_subcell_light.rs</FILE>
// <DESC>Sub-cell light filter that renders light fields into partial-block or braille glyphs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
