// <FILE>tui-vfx-style/tests/utils/test_fnc_brighten_hct.rs</FILE> - <DESC>Behavioral tests for brighten_hct — hue preservation, monotonicity, perceptual sanity</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>TTE effects port phase 1 — verify the HCT-based brightness operator preserves hue and produces a monotonic darkening curve, replacing the planned HSL byte-equivalence tests.</WCTX>
// <CLOG>0.1.0: initial test suite covering identity-ish at factor=1, full darken at factor=0, hue preservation across primaries and the TTE Beams gradient stops, monotonicity in tone.</CLOG>

use tui_vfx_style::utils::brighten_hct;
use tui_vfx_types::Color;

#[test]
fn factor_zero_returns_black() {
    for &(r, g, b) in &[(255, 0, 0), (123, 87, 200), (40, 42, 54), (0, 209, 255)] {
        let dim = brighten_hct(Color::rgb(r, g, b), 0.0);
        assert_eq!(dim.r, 0, "from ({r},{g},{b})");
        assert_eq!(dim.g, 0, "from ({r},{g},{b})");
        assert_eq!(dim.b, 0, "from ({r},{g},{b})");
    }
}

#[test]
fn factor_one_drifts_within_tolerance() {
    // HCT round-trip is not exact at the byte level due to CAM16 numerical
    // solver convergence. Document the bound rather than asserting equality.
    for &(r, g, b) in &[(255, 0, 0), (40, 42, 54), (138, 0, 138), (0, 209, 255)] {
        let out = brighten_hct(Color::rgb(r, g, b), 1.0);
        let dr = (r as i16 - out.r as i16).abs();
        let dg = (g as i16 - out.g as i16).abs();
        let db = (b as i16 - out.b as i16).abs();
        assert!(dr <= 3, "r drift {r}->{} (delta {dr})", out.r);
        assert!(dg <= 3, "g drift {g}->{} (delta {dg})", out.g);
        assert!(db <= 3, "b drift {b}->{} (delta {db})", out.b);
    }
}

#[test]
fn red_at_thirty_percent_preserves_hue_dominance() {
    // The TTE faded-text case. Red dominance must survive the fade.
    let out = brighten_hct(Color::rgb(255, 0, 0), 0.3);
    assert!(
        out.r > out.g,
        "expected red dominance, got ({}, {}, {})",
        out.r,
        out.g,
        out.b
    );
    assert!(
        out.r > out.b,
        "expected red dominance, got ({}, {}, {})",
        out.r,
        out.g,
        out.b
    );
}

#[test]
fn cyan_stop_preserves_hue_dominance() {
    // 0x00D1FF — TTE Beams mid-gradient stop. Faded cyan must still be cyan-ish
    // (g and b should dominate r), not a desaturated mid-tone.
    let out = brighten_hct(Color::rgb(0x00, 0xD1, 0xFF), 0.3);
    assert!(
        out.g > out.r,
        "cyan g/b dominance, got ({}, {}, {})",
        out.r,
        out.g,
        out.b
    );
    assert!(
        out.b > out.r,
        "cyan g/b dominance, got ({}, {}, {})",
        out.r,
        out.g,
        out.b
    );
}

#[test]
fn magenta_stop_preserves_hue_dominance() {
    // 0x8A008A — TTE Beams magenta stop.
    let out = brighten_hct(Color::rgb(0x8A, 0x00, 0x8A), 0.3);
    assert!(
        out.r > out.g,
        "magenta r/b dominance, got ({}, {}, {})",
        out.r,
        out.g,
        out.b
    );
    assert!(
        out.b > out.g,
        "magenta r/b dominance, got ({}, {}, {})",
        out.r,
        out.g,
        out.b
    );
}

#[test]
fn factor_monotonically_darkens() {
    // For each color, the perceived brightness (use channel sum as a proxy)
    // should decrease monotonically as factor decreases.
    for &(r, g, b) in &[(255, 0, 0), (0, 255, 0), (0, 0, 255), (200, 100, 50)] {
        let c = Color::rgb(r, g, b);
        let f100 = brighten_hct(c, 1.0);
        let f50 = brighten_hct(c, 0.5);
        let f25 = brighten_hct(c, 0.25);
        let f0 = brighten_hct(c, 0.0);
        let sum = |x: Color| x.r as i32 + x.g as i32 + x.b as i32;
        assert!(sum(f100) > sum(f50), "f100 > f50 from ({r},{g},{b})");
        assert!(sum(f50) > sum(f25), "f50 > f25 from ({r},{g},{b})");
        assert!(sum(f25) > sum(f0), "f25 > f0 from ({r},{g},{b})");
    }
}

#[test]
fn alpha_is_preserved() {
    let c = Color {
        r: 100,
        g: 100,
        b: 100,
        a: 128,
    };
    let out = brighten_hct(c, 0.5);
    assert_eq!(out.a, 128);
}

#[test]
fn overflow_factor_does_not_panic() {
    // factor > 1.0 brightens; should clamp at L*=100 ceiling and produce a
    // valid (white-ish) result.
    let out = brighten_hct(Color::rgb(255, 0, 0), 10.0);
    // Just verify it didn't panic and returned something white-ish
    assert!(out.r >= 200);
    assert!(out.g >= 200);
    assert!(out.b >= 200);
}

#[test]
fn black_input_stays_black() {
    let out = brighten_hct(Color::rgb(0, 0, 0), 1.5);
    // Black has tone=0; multiplying by anything gives tone=0; still black.
    assert_eq!(out, Color::rgb(0, 0, 0));
}

#[test]
fn perceptual_uniformity_yellow_vs_blue_at_same_factor() {
    // The HCT win over HSL: saturated yellow at "same brightness factor" as
    // saturated blue should produce *perceptually equivalent* darkening.
    // Verify that both end up with similar HCT tone after the same factor —
    // not that they have the same RGB sum (yellow is brighter than blue at
    // any tone), but that their tone dropped by the same proportion.
    use mcu_hct::Hct;
    use mcu_utils::color::argb_from_rgb;

    let yellow = Color::rgb(255, 255, 0);
    let blue = Color::rgb(0, 0, 255);

    let factor = 0.5;
    let dim_yellow = brighten_hct(yellow, factor);
    let dim_blue = brighten_hct(blue, factor);

    let yellow_tone_in = Hct::from_int(argb_from_rgb(yellow.r, yellow.g, yellow.b)).tone();
    let blue_tone_in = Hct::from_int(argb_from_rgb(blue.r, blue.g, blue.b)).tone();
    let yellow_tone_out =
        Hct::from_int(argb_from_rgb(dim_yellow.r, dim_yellow.g, dim_yellow.b)).tone();
    let blue_tone_out = Hct::from_int(argb_from_rgb(dim_blue.r, dim_blue.g, dim_blue.b)).tone();

    let yellow_ratio = yellow_tone_out / yellow_tone_in;
    let blue_ratio = blue_tone_out / blue_tone_in;
    // Both ratios should be close to `factor`, regardless of the input hue.
    // Allow modest drift for HCT's chroma-ceiling clamping.
    assert!(
        (yellow_ratio - factor).abs() < 0.1,
        "yellow tone ratio {yellow_ratio} should be near {factor}"
    );
    assert!(
        (blue_ratio - factor).abs() < 0.1,
        "blue tone ratio {blue_ratio} should be near {factor}"
    );
}
