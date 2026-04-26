// <FILE>crates/tui-vfx-types/src/glyph/cls_glyph_encoder.rs</FILE> - <DESC>GlyphEncoder enum: closed vocabulary of scalar-to-glyph encoders for field-effect rendering</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Fix stale 4-arg doctest call after encode_one was trimmed to 3-arg signature</WCTX>
// <CLOG>0.2.1: fix doctest in GlyphEncoder struct-level example — drop trailing `0.0` arg that no longer exists after encode_one was trimmed from 4 to 3 parameters.</CLOG>

use std::borrow::Cow;

use crate::braille::braille;

/// Bit order for braille dot fill, mirroring `SubcellLight::BRAILLE_DOTS` exactly.
///
/// The order determines which physical dots light first as intensity increases.
/// Matches the constant at `crates/tui-vfx-compositor/src/filters/cls_subcell_light.rs:43`.
const BRAILLE_DOTS: [u8; 8] = [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];

/// Closed vocabulary of glyph encoders that map a scalar (or eight-scalar)
/// intensity field to a printable terminal `char`.
///
/// Encoders are decoupled from the source of intensity. A consumer pairs an
/// encoder with a `Signal` sampler (typically via `ScalarFieldGlyphFilter` in
/// `tui-vfx-compositor`) to render water/fire/terrain/etc. fields as glyphs.
///
/// # Variants
///
/// - [`GlyphEncoder::BrailleSubcell`] — eight subcell scalars → 256-pattern braille,
///   per-dot threshold (each dot lights independently when its scalar meets the
///   threshold).
/// - [`GlyphEncoder::BrailleEighths`] — single intensity → eighths dot count braille,
///   optionally rotation-permuted by `(x, y)` for spatial hashing. Byte-equivalent
///   to `SubcellLight::rotated_braille_pattern` from `tui-vfx-compositor` when
///   `rotated: true`.
/// - [`GlyphEncoder::BlockHorizontal`] — single intensity → ▏▎▍▌▋▊▉█. Byte-equivalent
///   to `SubcellLight::horizontal_partial`.
/// - [`GlyphEncoder::BlockVertical`] — single intensity → ▁▂▃▄▅▆▇█. Byte-equivalent
///   to `SubcellLight::vertical_partial`.
/// - [`GlyphEncoder::Ramp`] — single intensity → arbitrary char ramp. `chars[0]` is
///   the coldest, `chars[len-1]` the brightest. `Cow<'static, [char]>` keeps the
///   common static-ramp case allocation-free.
///
/// # Example
///
/// ```rust
/// use tui_vfx_types::glyph::GlyphEncoder;
/// let enc = GlyphEncoder::BlockHorizontal;
/// assert_eq!(enc.encode_one(0.5, 0, 0), '▌');
/// ```
#[derive(Debug, Clone)]
pub enum GlyphEncoder {
    /// Eight per-subcell scalars → 256-pattern braille via per-dot threshold.
    ///
    /// Each of the eight braille dots lights independently when its subcell
    /// scalar is `>= threshold`. `encode_one` falls back to averaging the
    /// input into `BrailleEighths`-style behaviour.
    BrailleSubcell { threshold: f32 },

    /// Single intensity → eighths dot-count braille.
    ///
    /// When `rotated: true`, the fill order is permuted by a spatial hash of
    /// `(x, y)` using `(x.wrapping_mul(37) + y.wrapping_mul(67)) % 8`. This
    /// breaks up the visual banding that appears when all cells at the same
    /// intensity fill the same dots.
    ///
    /// When `rotated: false`, `rotation = 0` and dots fill in a fixed order
    /// regardless of cell position.
    BrailleEighths { rotated: bool },

    /// Single intensity → horizontal partial-block characters ▏▎▍▌▋▊▉█.
    BlockHorizontal,

    /// Single intensity → vertical partial-block characters ▁▂▃▄▅▆▇█.
    BlockVertical,

    /// Single intensity → arbitrary char ramp.
    ///
    /// `chars[0]` maps to intensity 0.0; `chars[len-1]` maps to intensity 1.0.
    /// Intensity is clamped to `[0.0, 1.0]` before indexing. An empty ramp
    /// returns `' '` for any intensity.
    Ramp(Cow<'static, [char]>),
}

impl GlyphEncoder {
    /// Encode a single scalar intensity to a glyph.
    ///
    /// Inputs outside `[0.0, 1.0]` are clamped before encoding. NaN intensity
    /// is treated as `0.0` (lowest glyph for all variants).
    ///
    /// `(x, y)` are the cell's grid coordinates; only
    /// `BrailleEighths { rotated: true }` reads them to permute the dot order
    /// across cells. Other variants ignore them.
    ///
    /// Cross-shape fallback: `BrailleSubcell::encode_one` averages the input
    /// scalar across all eight subcell positions and falls back to
    /// `BrailleEighths { rotated: false }` behaviour, so callers can swap
    /// encoders freely.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_vfx_types::glyph::GlyphEncoder;
    /// assert_eq!(GlyphEncoder::BlockVertical.encode_one(0.0, 0, 0), ' ');
    /// assert_eq!(GlyphEncoder::BlockVertical.encode_one(1.0, 0, 0), '█');
    /// ```
    pub fn encode_one(&self, intensity: f32, x: u16, y: u16) -> char {
        let i = if intensity.is_nan() {
            0.0_f32
        } else {
            intensity.clamp(0.0, 1.0)
        };
        match self {
            GlyphEncoder::BrailleSubcell { .. } => {
                // Cross-shape fallback: average → eighths form (unrotated)
                GlyphEncoder::BrailleEighths { rotated: false }.encode_one(i, x, y)
            }
            GlyphEncoder::BrailleEighths { rotated } => braille_eighths(i, x, y, *rotated),
            GlyphEncoder::BlockHorizontal => horizontal_partial_char(i),
            GlyphEncoder::BlockVertical => vertical_partial_char(i),
            GlyphEncoder::Ramp(chars) => ramp_char(chars, i),
        }
    }

    /// Encode eight subcell scalars to a glyph.
    ///
    /// `subcells` is indexed by braille dot number minus one:
    /// `subcells[0]` = dot 1, `subcells[1]` = dot 2, ..., `subcells[7]` = dot 8.
    /// See `tui-vfx-types::braille` for the dot-position diagram.
    ///
    /// For `BrailleSubcell { threshold }`, each dot lights independently when
    /// its subcell scalar is `>= threshold`. NaN subcell scalars are treated as
    /// below threshold (dot stays unlit).
    ///
    /// For non-subcell encoders (`BrailleEighths`, `BlockHorizontal`,
    /// `BlockVertical`, `Ramp`), the eight scalars are averaged and routed
    /// to `encode_one`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_vfx_types::glyph::GlyphEncoder;
    /// use tui_vfx_types::braille::braille;
    /// // All dots above threshold 0.0 → full braille
    /// let enc = GlyphEncoder::BrailleSubcell { threshold: 0.0 };
    /// assert_eq!(enc.encode_subcell([1.0; 8], 0, 0), braille(0xFF));
    /// ```
    pub fn encode_subcell(&self, subcells: [f32; 8], x: u16, y: u16) -> char {
        match self {
            GlyphEncoder::BrailleSubcell { threshold } => {
                let mut bits = 0_u8;
                for (i, &v) in subcells.iter().enumerate() {
                    // NaN is not >= threshold (NaN comparisons always false)
                    if v >= *threshold {
                        bits |= 1 << i;
                    }
                }
                braille(bits)
            }
            _ => {
                // Average the eight scalars and route to encode_one
                let mut sum = 0.0_f32;
                let mut count = 0_u8;
                for &v in &subcells {
                    if !v.is_nan() {
                        sum += v;
                        count += 1;
                    }
                }
                let avg = if count > 0 { sum / count as f32 } else { 0.0 };
                self.encode_one(avg, x, y)
            }
        }
    }
}

/// Braille eighths encoding, byte-equivalent to `SubcellLight::rotated_braille_pattern`.
///
/// When `rotated`, the fill order is permuted by the spatial hash
/// `(x.wrapping_mul(37) + y.wrapping_mul(67)) % 8` with `time_step = 0`
/// (temporal dither lives on the compositor filter, not on the encoder).
#[inline]
fn braille_eighths(intensity: f32, x: u16, y: u16, rotated: bool) -> char {
    let dots_to_fill = (intensity * 8.0).round().clamp(0.0, 8.0) as u32;
    let rotation = if rotated {
        ((x as u32)
            .wrapping_mul(37)
            .wrapping_add((y as u32).wrapping_mul(67)))
            % 8
    } else {
        0
    };
    let mut pattern = 0_u8;
    for idx in 0..dots_to_fill.min(8) {
        let dot = BRAILLE_DOTS[((idx + rotation) % 8) as usize];
        pattern |= dot;
    }
    braille(pattern)
}

/// Horizontal partial-block, byte-equivalent to `SubcellLight::horizontal_partial`.
#[inline]
fn horizontal_partial_char(intensity: f32) -> char {
    let sub = (intensity * 8.0).round().clamp(0.0, 8.0) as u8;
    match sub {
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

/// Vertical partial-block, byte-equivalent to `SubcellLight::vertical_partial`.
#[inline]
fn vertical_partial_char(intensity: f32) -> char {
    let sub = (intensity * 8.0).round().clamp(0.0, 8.0) as u8;
    match sub {
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

/// Ramp character lookup. Intensity is pre-clamped to `[0.0, 1.0]` by the caller.
///
/// Indexing: `chars[(intensity * (len-1)).round() as usize]`.
/// For an empty ramp, returns `' '`.
#[inline]
fn ramp_char(chars: &[char], intensity: f32) -> char {
    if chars.is_empty() {
        return ' ';
    }
    let last = (chars.len() - 1) as f32;
    let idx = (intensity * last).round() as usize;
    chars[idx.min(chars.len() - 1)]
}

#[cfg(test)]
#[path = "test_cls_glyph_encoder.rs"]
mod tests;

// <FILE>crates/tui-vfx-types/src/glyph/cls_glyph_encoder.rs</FILE> - <DESC>GlyphEncoder enum: closed vocabulary of scalar-to-glyph encoders for field-effect rendering</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
