// <FILE>crates/tui-vfx-compositor/src/filters/test_cls_cell_color_intensity_signal.rs</FILE>
// <DESC>Tests for CellColorIntensitySignal — byte-equivalence and boundary anchors</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Slice 6.6 §F.5 — migrate Filter trait to VfxCellContext bundle</WCTX>
// <CLOG>0.1.1: migrate filter.apply call to &VfxCellContext.</CLOG>

use super::CellColorIntensitySignal;
use crate::filters::cls_subcell_light::{LightSampleFrom, SubcellLight};
use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color, Modifiers, VfxCellContext};

fn make_cell(ch: char, fg: Color, bg: Color) -> Cell {
    Cell::styled(ch, fg, bg, Modifiers::NONE)
}

/// Mirror of `SubcellLight::project_intensity` used as reference oracle.
fn legacy_project_intensity(unlit: Color, lit: Color, sampled: Color) -> f32 {
    let base = [unlit.r as f32, unlit.g as f32, unlit.b as f32];
    let lit_f = [lit.r as f32, lit.g as f32, lit.b as f32];
    let sample = [sampled.r as f32, sampled.g as f32, sampled.b as f32];

    let axis = [lit_f[0] - base[0], lit_f[1] - base[1], lit_f[2] - base[2]];
    let axis_len_sq = axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2];
    if axis_len_sq <= f32::EPSILON {
        return 0.0;
    }
    let delta = [
        sample[0] - base[0],
        sample[1] - base[1],
        sample[2] - base[2],
    ];
    let projected = (delta[0] * axis[0] + delta[1] * axis[1] + delta[2] * axis[2]) / axis_len_sq;
    projected.clamp(0.0, 1.0)
}

/// The byte-equivalence anchor: CellColorIntensitySignal::intensity_for must
/// match SubcellLight's internal projection for the same lit/unlit/sample triple.
#[test]
fn test_intensity_for_byte_equivalent_to_subcell_light_project_intensity() {
    let triples: &[(Color, Color, Color)] = &[
        (
            Color::rgb(220, 220, 220),
            Color::rgb(24, 24, 24),
            Color::rgb(110, 110, 110),
        ),
        (
            Color::rgb(255, 0, 0),
            Color::rgb(0, 0, 0),
            Color::rgb(128, 0, 0),
        ),
        (
            Color::rgb(200, 100, 50),
            Color::rgb(10, 10, 10),
            Color::rgb(105, 55, 30),
        ),
    ];

    for &(lit, unlit, sample) in triples {
        let sampler = CellColorIntensitySignal {
            lit,
            unlit,
            sample_from: LightSampleFrom::Background,
        };
        let cell = make_cell(' ', Color::TRANSPARENT, sample);
        let actual = sampler.intensity_for(&cell);
        let expected = legacy_project_intensity(unlit, lit, sample);
        assert!(
            (actual - expected).abs() < f32::EPSILON,
            "mismatch for lit={:?} unlit={:?} sample={:?}: got {}, expected {}",
            lit,
            unlit,
            sample,
            actual,
            expected
        );
    }
}

/// Byte-equivalence also exercised via SubcellLight directly: if SubcellLight
/// with a background sample produces a non-space char at a given cell, the same
/// intensity produced by CellColorIntensitySignal must be > threshold.
#[test]
fn test_intensity_for_consistent_with_subcell_light_apply() {
    let lit = Color::rgb(200, 200, 200);
    let unlit = Color::rgb(20, 20, 20);
    let bg = Color::rgb(110, 110, 110);

    let filter = SubcellLight {
        lit_color: lit,
        unlit_color: unlit,
        threshold: 0.0,
        ..Default::default()
    };
    let mut cell = make_cell(' ', Color::TRANSPARENT, bg);
    filter.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 1, 0, 0, 0.0));
    // SubcellLight wrote a glyph — confirm intensity > 0.
    assert_ne!(cell.ch, ' ', "SubcellLight should have written a glyph");

    let sampler = CellColorIntensitySignal {
        lit,
        unlit,
        sample_from: LightSampleFrom::Background,
    };
    let ref_cell = make_cell(' ', Color::TRANSPARENT, bg);
    let intensity = sampler.intensity_for(&ref_cell);
    assert!(
        intensity > 0.0,
        "CellColorIntensitySignal should report positive intensity"
    );
}

/// Zero-length axis (lit == unlit) must return 0.0.
#[test]
fn test_intensity_for_zero_axis_returns_zero() {
    let sampler = CellColorIntensitySignal {
        lit: Color::rgb(100, 100, 100),
        unlit: Color::rgb(100, 100, 100),
        sample_from: LightSampleFrom::Background,
    };
    let cell = make_cell(' ', Color::TRANSPARENT, Color::rgb(100, 100, 100));
    assert_eq!(sampler.intensity_for(&cell), 0.0);
}

/// Sample far outside the [unlit, lit] axis must be clamped to [0.0, 1.0].
#[test]
fn test_intensity_for_clamps_to_unit_range() {
    // lit is dim, unlit is bright — a sampled white is way past "lit"
    let sampler = CellColorIntensitySignal {
        lit: Color::rgb(50, 50, 50),
        unlit: Color::rgb(0, 0, 0),
        sample_from: LightSampleFrom::Background,
    };
    let cell = make_cell(' ', Color::TRANSPARENT, Color::rgb(255, 255, 255));
    let intensity = sampler.intensity_for(&cell);
    assert!(
        (0.0..=1.0).contains(&intensity),
        "intensity must clamp: got {}",
        intensity
    );
    assert_eq!(intensity, 1.0, "fully beyond lit end → clamped to 1.0");
}

/// A fully-transparent cell (alpha == 0) must return 0.0.
#[test]
fn test_intensity_for_alpha_zero_returns_zero() {
    let sampler = CellColorIntensitySignal {
        lit: Color::rgb(220, 220, 220),
        unlit: Color::rgb(24, 24, 24),
        sample_from: LightSampleFrom::Background,
    };
    // TRANSPARENT has a == 0
    let cell = make_cell(' ', Color::TRANSPARENT, Color::TRANSPARENT);
    assert_eq!(sampler.intensity_for(&cell), 0.0);
}

/// Foreground sampling reads cell.fg; background sampling reads cell.bg.
#[test]
fn test_sample_from_foreground_vs_background() {
    let lit = Color::rgb(220, 220, 220);
    let unlit = Color::rgb(24, 24, 24);
    let bright_fg = Color::rgb(200, 200, 200); // near lit
    let dim_bg = Color::rgb(30, 30, 30); // near unlit

    let cell = make_cell(' ', bright_fg, dim_bg);

    let fg_sampler = CellColorIntensitySignal {
        lit,
        unlit,
        sample_from: LightSampleFrom::Foreground,
    };
    let bg_sampler = CellColorIntensitySignal {
        lit,
        unlit,
        sample_from: LightSampleFrom::Background,
    };

    let fg_intensity = fg_sampler.intensity_for(&cell);
    let bg_intensity = bg_sampler.intensity_for(&cell);

    // Foreground is near lit → high intensity; background is near unlit → low
    assert!(
        fg_intensity > bg_intensity,
        "foreground ({}) should be brighter than background ({})",
        fg_intensity,
        bg_intensity
    );
}

// <FILE>crates/tui-vfx-compositor/src/filters/test_cls_cell_color_intensity_signal.rs</FILE>
// <DESC>Tests for CellColorIntensitySignal — byte-equivalence and boundary anchors</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
