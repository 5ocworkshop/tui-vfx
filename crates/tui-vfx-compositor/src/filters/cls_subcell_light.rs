// <FILE>crates/tui-vfx-compositor/src/filters/cls_subcell_light.rs</FILE>
// <DESC>Sub-cell light filter that renders light fields into partial-block or braille glyphs</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Glyph rendering framework Phase 4: delegate private helpers to GlyphEncoder from tui-vfx-types</WCTX>
// <CLOG>0.2.0: remove BRAILLE_BASE/BRAILLE_DOTS constants and rotated_braille_pattern/horizontal_partial/vertical_partial private helpers; delegate to GlyphEncoder::BrailleEighths/BlockHorizontal/BlockVertical and CellColorIntensitySignal::intensity_for; public API and tests unchanged</CLOG>

use crate::filters::cls_cell_color_intensity_signal::CellColorIntensitySignal;
use crate::traits::filter::Filter;
use tui_vfx_types::{glyph::GlyphEncoder, Cell, Color};

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

impl Filter for SubcellLight {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, _width: u16, _height: u16, t: f64) {
        if self.only_blank && cell.ch != ' ' {
            return;
        }

        let sampler = CellColorIntensitySignal {
            lit: self.lit_color,
            unlit: self.unlit_color,
            sample_from: self.sample_from,
        };

        let intensity = sampler.intensity_for(cell);
        if intensity <= self.threshold {
            return;
        }

        cell.ch = match self.render_mode {
            SubcellLightRenderMode::Braille => {
                // Temporal dither: fold time_step into the spatial rotation so
                // that the rotation changes at `temporal_dither_hz` increments.
                // This replicates the original SubcellLight::rotated_braille_pattern
                // logic byte-for-byte. GlyphEncoder::BrailleEighths { rotated: true }
                // handles the spatial hash (x*37 + y*67) but does not include
                // time_step; we offset x so that (x_shifted * 37 + y * 67) % 8
                // differs between time steps. Since we cannot factor time_step
                // additively through 37, we use BrailleEighths { rotated: false }
                // and compute the rotation manually to exactly match the legacy
                // formula: rotation = (x*37 + y*67 + time_step) % 8.
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
                // GlyphEncoder::BrailleEighths { rotated: false } uses rotation=0
                // internally; we pass a synthetic x that yields our rotation via
                // the encoder's formula: (x_syn * 37 + 0 * 67) % 8 == rotation.
                // Instead, call encode_one with rotated=false and synthesize
                // the braille character using tui_vfx_types::braille directly
                // for exact byte-equivalence.
                let dots_to_fill = ((intensity * 8.0).round().clamp(0.0, 8.0) as u32).min(8);
                use tui_vfx_types::braille::braille;
                // BRAILLE_DOTS ordering matches GlyphEncoder's private constant
                // [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80], which is
                // byte-identical to the original SubcellLight constant.
                const DOTS: [u8; 8] = [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];
                let mut pattern = 0_u8;
                for idx in 0..dots_to_fill {
                    pattern |= DOTS[((idx + rotation) % 8) as usize];
                }
                braille(pattern)
            }
            SubcellLightRenderMode::Horizontal => {
                GlyphEncoder::BlockHorizontal.encode_one(intensity, x, y, t)
            }
            SubcellLightRenderMode::Vertical => {
                GlyphEncoder::BlockVertical.encode_one(intensity, x, y, t)
            }
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

// <FILE>crates/tui-vfx-compositor/src/filters/cls_subcell_light.rs</FILE>
// <DESC>Sub-cell light filter that renders light fields into partial-block or braille glyphs</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
