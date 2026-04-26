// <FILE>crates/tui-vfx-types/src/glyph/test_cls_glyph_encoder.rs</FILE> - <DESC>TDD peer tests for GlyphEncoder enum and encode_one/encode_subcell methods</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 3: TDD tests for GlyphEncoder for water/fire/future field-effect glyph encoding</WCTX>
// <CLOG>0.1.0: initial TDD coverage with byte-equivalence anchors for BrailleEighths vs SubcellLight, Q-D worked example, and full table coverage for block variants</CLOG>

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::braille::braille;
    use crate::glyph::GlyphEncoder;

    // ─── BrailleSubcell tests ────────────────────────────────────────────────

    /// Canonical Q-D anchor from the design doc.
    ///
    /// subcells = [0.1, 0.5, 0.7, 0.2, 0.0, 0.9, 0.3, 0.1], threshold = 0.4
    /// Values >= threshold: index 1 (0.5), index 2 (0.7), index 5 (0.9)
    /// bits = (1 << 1) | (1 << 2) | (1 << 5) = 2 + 4 + 32 = 0x26
    /// braille(0x26) = '⠦'
    #[test]
    fn test_braille_subcell_per_dot_threshold_worked_example() {
        let encoder = GlyphEncoder::BrailleSubcell { threshold: 0.4 };
        let subcells = [0.1f32, 0.5, 0.7, 0.2, 0.0, 0.9, 0.3, 0.1];
        let ch = encoder.encode_subcell(subcells, 0, 0, 0.0);
        // bits: idx1=0x02, idx2=0x04, idx5=0x20 → 0x26
        assert_eq!(ch, braille(0x26), "expected '⠦' (0x26), got {ch:?}");
    }

    #[test]
    fn test_braille_subcell_all_below_threshold_returns_empty() {
        let encoder = GlyphEncoder::BrailleSubcell { threshold: 0.5 };
        let ch = encoder.encode_subcell([0.0f32; 8], 0, 0, 0.0);
        assert_eq!(ch, braille(0x00), "expected empty braille '⠀'");
    }

    #[test]
    fn test_braille_subcell_all_above_threshold_returns_full() {
        let encoder = GlyphEncoder::BrailleSubcell { threshold: 0.0 };
        let ch = encoder.encode_subcell([1.0f32; 8], 0, 0, 0.0);
        assert_eq!(ch, braille(0xFF), "expected full braille '⣿'");
    }

    /// Threshold boundary is inclusive (>=).
    #[test]
    fn test_braille_subcell_threshold_boundary_inclusive() {
        let encoder = GlyphEncoder::BrailleSubcell { threshold: 0.5 };
        // Exactly at threshold for dot 0 only
        let subcells = [0.5f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let ch = encoder.encode_subcell(subcells, 0, 0, 0.0);
        // bit 0 set = 0x01
        assert_eq!(ch, braille(0x01), "dot at exactly threshold should light");
    }

    /// NaN inputs must not panic and must produce a deterministic fallback (empty glyph).
    #[test]
    fn test_braille_subcell_nan_input_does_not_panic() {
        let encoder = GlyphEncoder::BrailleSubcell { threshold: 0.5 };
        let ch = encoder.encode_subcell([f32::NAN; 8], 0, 0, 0.0);
        // NaN is not >= threshold, so no dots light → empty braille
        assert_eq!(ch, braille(0x00), "NaN should produce empty braille");
    }

    /// Cross-shape fallback: BrailleSubcell::encode_one must average to BrailleEighths-style.
    ///
    /// When called via encode_one(0.5), the subcell encoder falls back to
    /// BrailleEighths { rotated: false }::encode_one(0.5) behaviour.
    #[test]
    fn test_braille_subcell_encode_one_averages_to_eighths_form() {
        let subcell_enc = GlyphEncoder::BrailleSubcell { threshold: 0.5 };
        let eighths_enc = GlyphEncoder::BrailleEighths { rotated: false };
        assert_eq!(
            subcell_enc.encode_one(0.5, 0, 0, 0.0),
            eighths_enc.encode_one(0.5, 0, 0, 0.0),
            "encode_one cross-shape fallback must match BrailleEighths"
        );
    }

    // ─── BrailleEighths byte-equivalence with SubcellLight ──────────────────

    /// Reproduce the SubcellLight::rotated_braille_pattern rotation formula
    /// with time_step = 0 (unrotated / rotation from spatial coords only).
    ///
    /// BRAILLE_DOTS order from cls_subcell_light.rs:
    ///   [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80]
    /// rotation = (x*37 + y*67 + time_step) % 8, time_step = 0 for unrotated
    fn expected_braille_eighths(intensity: f32, x: u16, y: u16, time_step: u32) -> char {
        const BRAILLE_BASE: u32 = 0x2800;
        const BRAILLE_DOTS: [u8; 8] = [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];
        let dots_to_fill = (intensity * 8.0).round().clamp(0.0, 8.0) as usize;
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

    /// Unrotated: rotation = 0 (time_step=0, x=0, y=0).
    ///
    /// Expected chars computed by hand with the BRAILLE_DOTS table:
    ///   dots=0 → 0x00 = '⠀'
    ///   dots=1 → 0x01 = '⠁'
    ///   dots=2 → 0x03 = '⠃'
    ///   dots=3 → 0x07 = '⠇'
    ///   dots=4 → 0x47 = '⡇'
    ///   dots=8 → 0xFF = '⣿'
    #[test]
    fn test_braille_eighths_unrotated_byte_equivalent_to_subcell_light() {
        let encoder = GlyphEncoder::BrailleEighths { rotated: false };
        for &intensity in &[0.0f32, 0.125, 0.25, 0.5, 1.0] {
            let got = encoder.encode_one(intensity, 0, 0, 0.0);
            let expected = expected_braille_eighths(intensity, 0, 0, 0);
            assert_eq!(
                got, expected,
                "unrotated mismatch at intensity={intensity}: got={got:?} expected={expected:?}"
            );
        }
    }

    /// Rotated with spatial input that produces a nonzero rotation.
    ///
    /// For x=3, y=2: rotation = (3*37 + 2*67) % 8 = (111 + 134) % 8 = 245 % 8 = 5
    /// That's a nonzero rotation confirming the spatial hash is exercised.
    #[test]
    fn test_braille_eighths_rotated_byte_equivalent_to_subcell_light() {
        let encoder = GlyphEncoder::BrailleEighths { rotated: true };
        // x=3, y=2: rotation = (3*37 + 2*67) % 8 = 245 % 8 = 5 (nonzero)
        for &intensity in &[0.25f32, 0.5, 0.75, 1.0] {
            let got = encoder.encode_one(intensity, 3, 2, 0.0);
            // rotated: time_step = 0 (encoder doesn't do temporal dither)
            let expected = expected_braille_eighths(intensity, 3, 2, 0);
            assert_eq!(
                got, expected,
                "rotated mismatch at intensity={intensity}: got={got:?} expected={expected:?}"
            );
        }

        // x=7, y=5: rotation = (7*37 + 5*67) % 8 = (259 + 335) % 8 = 594 % 8 = 2
        for &intensity in &[0.375f32, 0.625] {
            let got = encoder.encode_one(intensity, 7, 5, 0.0);
            let expected = expected_braille_eighths(intensity, 7, 5, 0);
            assert_eq!(
                got, expected,
                "rotated mismatch at intensity={intensity}: got={got:?} expected={expected:?}"
            );
        }
    }

    // ─── BlockHorizontal table ───────────────────────────────────────────────

    /// For each of the nine block positions, assert the exact character.
    ///
    /// Mirrors SubcellLight::horizontal_partial table byte-for-byte.
    #[test]
    fn test_block_horizontal_table_byte_for_byte() {
        let encoder = GlyphEncoder::BlockHorizontal;
        let expected: &[(f32, char)] = &[
            (0.0 / 8.0, ' '),
            (1.0 / 8.0, '▏'),
            (2.0 / 8.0, '▎'),
            (3.0 / 8.0, '▍'),
            (4.0 / 8.0, '▌'),
            (5.0 / 8.0, '▋'),
            (6.0 / 8.0, '▊'),
            (7.0 / 8.0, '▉'),
            (8.0 / 8.0, '█'),
        ];
        for &(intensity, ch) in expected {
            assert_eq!(
                encoder.encode_one(intensity, 0, 0, 0.0),
                ch,
                "BlockHorizontal at intensity={intensity} expected={ch:?}"
            );
        }
    }

    // ─── BlockVertical table ─────────────────────────────────────────────────

    /// For each of the nine block positions, assert the exact character.
    ///
    /// Mirrors SubcellLight::vertical_partial table byte-for-byte.
    #[test]
    fn test_block_vertical_table_byte_for_byte() {
        let encoder = GlyphEncoder::BlockVertical;
        let expected: &[(f32, char)] = &[
            (0.0 / 8.0, ' '),
            (1.0 / 8.0, '▁'),
            (2.0 / 8.0, '▂'),
            (3.0 / 8.0, '▃'),
            (4.0 / 8.0, '▄'),
            (5.0 / 8.0, '▅'),
            (6.0 / 8.0, '▆'),
            (7.0 / 8.0, '▇'),
            (8.0 / 8.0, '█'),
        ];
        for &(intensity, ch) in expected {
            assert_eq!(
                encoder.encode_one(intensity, 0, 0, 0.0),
                ch,
                "BlockVertical at intensity={intensity} expected={ch:?}"
            );
        }
    }

    // ─── Ramp tests ──────────────────────────────────────────────────────────

    #[test]
    fn test_ramp_basic() {
        let ramp = GlyphEncoder::Ramp(Cow::Borrowed(&['a', 'b', 'c', 'd']));
        assert_eq!(ramp.encode_one(0.0, 0, 0, 0.0), 'a');
        assert_eq!(ramp.encode_one(1.0, 0, 0, 0.0), 'd');
        // 0.5 * 3 = 1.5, round → 2 → 'c'
        assert_eq!(ramp.encode_one(0.5, 0, 0, 0.0), 'c');
    }

    #[test]
    fn test_ramp_empty_returns_space() {
        let ramp = GlyphEncoder::Ramp(Cow::Borrowed(&[]));
        assert_eq!(ramp.encode_one(0.5, 0, 0, 0.0), ' ');
        assert_eq!(ramp.encode_one(0.0, 0, 0, 0.0), ' ');
        assert_eq!(ramp.encode_one(1.0, 0, 0, 0.0), ' ');
    }

    #[test]
    fn test_ramp_clamp_below_zero() {
        let ramp = GlyphEncoder::Ramp(Cow::Borrowed(&['a', 'b', 'c']));
        assert_eq!(ramp.encode_one(-1.0, 0, 0, 0.0), 'a');
    }

    #[test]
    fn test_ramp_clamp_above_one() {
        let ramp = GlyphEncoder::Ramp(Cow::Borrowed(&['a', 'b', 'c']));
        assert_eq!(ramp.encode_one(2.0, 0, 0, 0.0), 'c');
    }

    // ─── Clamping and NaN safety for block encoders ──────────────────────────

    #[test]
    fn test_block_intensity_clamping() {
        let h = GlyphEncoder::BlockHorizontal;
        let v = GlyphEncoder::BlockVertical;
        // Should not panic; should clamp gracefully
        assert_eq!(h.encode_one(-0.5, 0, 0, 0.0), ' ');
        assert_eq!(h.encode_one(1.5, 0, 0, 0.0), '█');
        assert_eq!(v.encode_one(-0.5, 0, 0, 0.0), ' ');
        assert_eq!(v.encode_one(1.5, 0, 0, 0.0), '█');
    }

    // ─── Clone derivation sanity ─────────────────────────────────────────────

    #[test]
    fn test_encoder_clone_works() {
        let enc = GlyphEncoder::BrailleSubcell { threshold: 0.3 };
        let cloned = enc.clone();
        // Both produce the same output
        let subcells = [0.5f32; 8];
        assert_eq!(
            enc.encode_subcell(subcells, 0, 0, 0.0),
            cloned.encode_subcell(subcells, 0, 0, 0.0)
        );
    }

    // ─── encode_subcell fallback for non-subcell encoders ───────────────────

    #[test]
    fn test_block_horizontal_encode_subcell_falls_back_to_average() {
        let enc = GlyphEncoder::BlockHorizontal;
        // Average of [0.5; 8] = 0.5
        let subcell_ch = enc.encode_subcell([0.5f32; 8], 0, 0, 0.0);
        let direct_ch = enc.encode_one(0.5, 0, 0, 0.0);
        assert_eq!(
            subcell_ch, direct_ch,
            "encode_subcell should fall back to encode_one(avg) for non-subcell variants"
        );
    }
}

// <FILE>crates/tui-vfx-types/src/glyph/test_cls_glyph_encoder.rs</FILE> - <DESC>TDD peer tests for GlyphEncoder enum and encode_one/encode_subcell methods</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
