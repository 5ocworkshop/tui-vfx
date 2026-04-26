// <FILE>tui-vfx-style/src/utils/fnc_blend_colors.rs</FILE> - <DESC>Color interpolation logic — RGB, HSL, and HCT (perceptually uniform via mcu-hct)</DESC>
// <VERS>VERSION: 0.3.0 - 2026-04-26</VERS>
// <WCTX>TTE effects port — extend blend_colors with the Hct arm so ColorSpace::Hct produces perceptually-uniform gradient interpolation. RGB and HSL arms unchanged.</WCTX>
// <CLOG>0.3.0: add Hct interpolation arm — converts both endpoints to HCT via mcu-hct, lerps tone/chroma linearly, lerps hue along shortest 360° path, converts back. RGB/HSL paths unchanged.</CLOG>

use crate::models::ColorSpace;
use mcu_hct::Hct;
use mcu_utils::color::{argb_from_rgb, blue_from_argb, green_from_argb, red_from_argb};
use tui_vfx_types::Color;
/// Blends two colors based on progress `t` and the specified color space.
///
/// Returns `c1` if `t <= 0.0` and `c2` if `t >= 1.0`.
pub fn blend_colors(c1: Color, c2: Color, t: f32, space: ColorSpace) -> Color {
    if t <= 0.0 {
        return c1;
    }
    if t >= 1.0 {
        return c2;
    }
    // With tui_vfx_types::Color, we can directly access RGB components
    let (r1, g1, b1) = (c1.r, c1.g, c1.b);
    let (r2, g2, b2) = (c2.r, c2.g, c2.b);

    match space {
        ColorSpace::Rgb => {
            let r = lerp(r1 as f32, r2 as f32, t) as u8;
            let g = lerp(g1 as f32, g2 as f32, t) as u8;
            let b = lerp(b1 as f32, b2 as f32, t) as u8;
            Color::rgb(r, g, b)
        }
        ColorSpace::Hsl => {
            let (h1, s1, l1) = rgb_to_hsl(r1, g1, b1);
            let (h2, s2, l2) = rgb_to_hsl(r2, g2, b2);
            // Hue wrapping: shortest path
            let d = h2 - h1;
            let delta = if d > 180.0 {
                d - 360.0
            } else if d < -180.0 {
                d + 360.0
            } else {
                d
            };
            let h = (h1 + delta * t).rem_euclid(360.0);
            let s = lerp(s1, s2, t);
            let l = lerp(l1, l2, t);
            let (r, g, b) = hsl_to_rgb(h, s, l);
            Color::rgb(r, g, b)
        }
        ColorSpace::Hct => {
            let h1 = Hct::from_int(argb_from_rgb(r1, g1, b1));
            let h2 = Hct::from_int(argb_from_rgb(r2, g2, b2));
            let (hue, chroma, tone) = lerp_hct(h1, h2, t as f64);
            let argb = Hct::from(hue, chroma, tone).to_int();
            Color::rgb(red_from_argb(argb), green_from_argb(argb), blue_from_argb(argb))
        }
    }
}
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
#[inline]
fn lerp_f64(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
/// Helper to convert colors to (r,g,b) tuple.
/// With tui_vfx_types::Color, this is always possible as RGB values are directly stored.
/// Returns Some for non-transparent colors, None for fully transparent.
pub fn to_rgb_tuple(c: Color) -> Option<(u8, u8, u8)> {
    if c.a == 0 {
        // Fully transparent - return None for backward compatibility
        None
    } else {
        Some((c.r, c.g, c.b))
    }
}
// Minimal HSL impl to avoid dependencies
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if max == min {
        return (0.0, 0.0, l); // Achromatic
    }
    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = if max == r {
        (g - b) / d + (if g < b { 6.0 } else { 0.0 })
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    (h * 60.0, s, l)
}
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    if s == 0.0 {
        let v = (l * 255.0) as u8;
        return (v, v, v);
    }
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let r = hue_to_rgb(p, q, h / 360.0 + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h / 360.0);
    let b = hue_to_rgb(p, q, h / 360.0 - 1.0 / 3.0);
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}
#[inline]
fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

/// Lerp two HCT colors. Hue takes the shortest 360° path; chroma and tone
/// lerp linearly. The hue rotation matches the HSL arm's shortest-path
/// behavior so authors switching between `ColorSpace::Hsl` and `Hct`
/// don't see direction-of-rotation changes on the same gradient.
fn lerp_hct(a: Hct, b: Hct, t: f64) -> (f64, f64, f64) {
    let h1 = a.hue();
    let h2 = b.hue();
    let d = h2 - h1;
    let delta = if d > 180.0 {
        d - 360.0
    } else if d < -180.0 {
        d + 360.0
    } else {
        d
    };
    let hue = (h1 + delta * t).rem_euclid(360.0);
    let chroma = lerp_f64(a.chroma(), b.chroma(), t);
    let tone = lerp_f64(a.tone(), b.tone(), t);
    (hue, chroma, tone)
}

// <FILE>tui-vfx-style/src/utils/fnc_blend_colors.rs</FILE>
// <VERS>END OF VERSION: 0.3.0 - 2026-04-26</VERS>
