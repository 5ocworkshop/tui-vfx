// <FILE>crates/tui-vfx-compositor/src/pipeline/fnc_grade_shadow_cell.rs</FILE> - <DESC>Destination-preserving color grading for grade-underlying shadow mode</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Color-inert glyph detection for shadow grading replacement</WCTX>
// <CLOG>Replace color-inert glyphs (emoji, PUA) with neutral placeholder during grading</CLOG>

//! Grade-underlying shadow cell helper.
//!
//! Applies color grading to a destination cell based on rendered shadow
//! coverage while preserving the destination's glyph, modifiers, and
//! mod_alpha. This is the compositor-side implementation for
//! [`ShadowCompositeMode::GradeUnderlying`](tui_vfx_shadow::ShadowCompositeMode::GradeUnderlying).

use tui_vfx_shadow::ShadowGradeConfig;
use tui_vfx_types::color_inert::is_color_inert_glyph;
use tui_vfx_types::{Cell, Color};

/// Apply color grading to `dest_cell` based on rendered shadow coverage.
///
/// The `shadow_cell` is used only to derive coverage (from alpha channels).
/// The destination cell's `ch`, `mods`, and `mod_alpha` are preserved.
/// Only foreground and background colors are graded.
///
/// # Algorithm
///
/// 1. Derive coverage from `max(shadow_cell.bg.a, shadow_cell.fg.a) / 255`.
/// 2. If coverage is zero, return `dest_cell` unchanged.
/// 3. Scale each grading strength by coverage.
/// 4. For each color channel (fg and bg): desaturate, then dim, then tint.
/// 5. Preserve alpha when the corresponding preserve flag is set.
#[inline]
pub fn grade_shadow_cell(
    shadow_cell: &Cell,
    dest_cell: &Cell,
    shadow_color: Color,
    grade: &ShadowGradeConfig,
) -> Cell {
    // Step 1: derive coverage from rendered shadow cell alpha
    let raw_coverage = shadow_cell.bg.a.max(shadow_cell.fg.a) as f32 / 255.0;
    let coverage = raw_coverage.clamp(0.0, 1.0);

    // Step 2: zero coverage means no grading
    if coverage == 0.0 {
        return *dest_cell;
    }

    // Step 3: effective strengths scaled by coverage
    let fg_dim = grade.fg_dim_strength * coverage;
    let bg_dim = grade.bg_dim_strength * coverage;
    let fg_desat = grade.fg_desaturate_strength * coverage;
    let bg_desat = grade.bg_desaturate_strength * coverage;
    let fg_tint = grade.fg_tint_strength * coverage;
    let bg_tint = grade.bg_tint_strength * coverage;

    // Step 4: grade foreground (desaturate -> dim -> tint)
    let mut graded_fg = desaturate(dest_cell.fg, fg_desat);
    graded_fg = dim(graded_fg, fg_dim);
    graded_fg = tint(graded_fg, shadow_color, fg_tint);

    // Step 5: grade background (desaturate -> dim -> tint)
    let mut graded_bg = desaturate(dest_cell.bg, bg_desat);
    graded_bg = dim(graded_bg, bg_dim);
    graded_bg = tint(graded_bg, shadow_color, bg_tint);

    // Preserve alpha when flags are set
    if grade.preserve_fg_alpha {
        graded_fg = graded_fg.with_alpha(dest_cell.fg.a);
    }
    if grade.preserve_bg_alpha {
        graded_bg = graded_bg.with_alpha(dest_cell.bg.a);
    }

    // Replace color-inert glyphs with neutral placeholder if configured
    let graded_ch = match grade.replacement_char {
        Some(replacement) if is_color_inert_glyph(dest_cell.ch) => replacement,
        _ => dest_cell.ch,
    };

    // Preserve ch (or replacement), mods, mod_alpha from destination
    Cell {
        ch: graded_ch,
        fg: graded_fg,
        bg: graded_bg,
        mods: dest_cell.mods,
        mod_alpha: dest_cell.mod_alpha,
    }
}

/// Desaturate a color toward BT.601 grey by strength `s` (0.0–1.0).
#[inline]
fn desaturate(color: Color, s: f32) -> Color {
    let grey = (0.299 * color.r as f32 + 0.587 * color.g as f32 + 0.114 * color.b as f32).round();
    let r = (color.r as f32 * (1.0 - s) + grey * s).round() as u8;
    let g = (color.g as f32 * (1.0 - s) + grey * s).round() as u8;
    let b = (color.b as f32 * (1.0 - s) + grey * s).round() as u8;
    Color::new(r, g, b, color.a)
}

/// Dim a color by multiplying RGB by `(1.0 - s)`.
#[inline]
fn dim(color: Color, s: f32) -> Color {
    let factor = 1.0 - s;
    let r = (color.r as f32 * factor).round() as u8;
    let g = (color.g as f32 * factor).round() as u8;
    let b = (color.b as f32 * factor).round() as u8;
    Color::new(r, g, b, color.a)
}

/// Tint a color toward `tint_color` by strength `s` (0.0–1.0).
#[inline]
fn tint(color: Color, tint_color: Color, s: f32) -> Color {
    let r = (color.r as f32 * (1.0 - s) + tint_color.r as f32 * s).round() as u8;
    let g = (color.g as f32 * (1.0 - s) + tint_color.g as f32 * s).round() as u8;
    let b = (color.b as f32 * (1.0 - s) + tint_color.b as f32 * s).round() as u8;
    Color::new(r, g, b, color.a)
}

// <FILE>crates/tui-vfx-compositor/src/pipeline/fnc_grade_shadow_cell.rs</FILE> - <DESC>Destination-preserving color grading for grade-underlying shadow mode</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
