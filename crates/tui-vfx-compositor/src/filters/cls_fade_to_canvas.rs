// <FILE>tui-vfx-compositor/src/filters/cls_fade_to_canvas.rs</FILE> - <DESC>Exit filter that blends cells toward a declared canvas color</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 P0.4 — canvas-aware exit fade replacement for the tint(black, 0.7+) hack</WCTX>
// <CLOG>Initial FadeToCanvas filter: fg/bg blend toward a configured canvas color with caller-controlled strength, avoiding the dark-flash artifact on light canvases</CLOG>

//! # FadeToCanvas filter
//!
//! A drop-in replacement for the `tint(black, 0.7+)` hack that many exit
//! animations use today. The hack darkens cells toward black regardless of
//! what's behind the widget, which flashes a dark band on a light terminal
//! canvas right before the widget disappears.
//!
//! `FadeToCanvas` instead blends toward a *declared* canvas color supplied
//! by the recipe author. Apps know their own terminal background and set
//! `canvas_color` to match, so the exit fades into the backdrop cleanly on
//! light **and** dark canvases. A future patch can lift `canvas_color` to
//! a runtime binding (e.g. `canvas_color_binding: "terminal_bg"`) so themes
//! can update the value live without recompiling; for now the static
//! `ColorConfig` path is sufficient to close the flash bug.
//!
//! Intentional semantic difference from `Tint`:
//!
//! | Filter          | Intended use                              | Field name      |
//! |-----------------|-------------------------------------------|-----------------|
//! | `Tint`          | Static color overlay (hover glow, damage) | `color`         |
//! | `FadeToCanvas`  | Time-driven exit fade into the backdrop   | `canvas_color`  |
//!
//! The blend math is identical — the separation exists so recipe authors
//! don't have to reason about "tint with signal-driven strength" when they
//! actually mean "fade out into the canvas."

use crate::traits::filter::Filter;
use crate::types::cls_filter_spec::ApplyTo;
use tui_vfx_types::{Cell, Color};

/// Blends cell colors toward a declared canvas color at a caller-controlled
/// strength (usually driven from the exit phase's animation progress).
pub struct FadeToCanvas {
    /// The canvas color to fade into. Should be set to the terminal
    /// background the recipe will run against; defaults to black for
    /// drop-in compatibility with the old `tint(black, ...)` hack.
    pub canvas_color: Color,
    /// Fade strength (0.0 = cell untouched, 1.0 = cell fully replaced with
    /// `canvas_color`). Typically resolved per frame by `prepare_filter`
    /// from a signal-driven spec field, then stored concretely here.
    pub strength: f32,
    /// Which color components to fade.
    pub apply_to: ApplyTo,
}

impl Default for FadeToCanvas {
    fn default() -> Self {
        Self {
            canvas_color: Color::rgb(0, 0, 0),
            strength: 0.0,
            apply_to: ApplyTo::Both,
        }
    }
}

impl FadeToCanvas {
    /// Construct a FadeToCanvas with a specific canvas color and strength.
    #[allow(dead_code)]
    pub fn new(canvas_color: Color, strength: f32) -> Self {
        Self {
            canvas_color,
            strength,
            apply_to: ApplyTo::Both,
        }
    }

    /// Linear blend from `base` toward `canvas_color` at `self.strength`.
    /// Uses `round()` to avoid the off-by-one color drift at boundary
    /// strengths that the `Tint` v3.2.0 fix documented.
    fn blend(&self, base: Color) -> Color {
        let s = self.strength.clamp(0.0, 1.0);
        let r = (base.r as f32 * (1.0 - s) + self.canvas_color.r as f32 * s).round() as u8;
        let g = (base.g as f32 * (1.0 - s) + self.canvas_color.g as f32 * s).round() as u8;
        let b = (base.b as f32 * (1.0 - s) + self.canvas_color.b as f32 * s).round() as u8;
        Color::rgb(r, g, b)
    }
}

impl Filter for FadeToCanvas {
    fn apply(&self, cell: &mut Cell, _x: u16, _y: u16, _width: u16, _height: u16, _t: f64) {
        // Strength is pre-resolved by prepare_filter; we ignore the raw `t`
        // passed in by the dispatch enum exactly like `Tint` does.
        match self.apply_to {
            ApplyTo::Foreground => {
                cell.fg = self.blend(cell.fg);
            }
            ApplyTo::Background => {
                cell.bg = self.blend(cell.bg);
            }
            ApplyTo::Both => {
                cell.fg = self.blend(cell.fg);
                cell.bg = self.blend(cell.bg);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    #[test]
    fn zero_strength_leaves_cell_untouched() {
        let filter = FadeToCanvas {
            canvas_color: Color::rgb(255, 255, 255),
            strength: 0.0,
            apply_to: ApplyTo::Both,
        };
        let mut cell = Cell::default();
        cell.fg = Color::rgb(10, 20, 30);
        cell.bg = Color::rgb(40, 50, 60);
        filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
        assert_eq!(cell.fg, Color::rgb(10, 20, 30));
        assert_eq!(cell.bg, Color::rgb(40, 50, 60));
    }

    #[test]
    fn full_strength_replaces_with_canvas_color() {
        let filter = FadeToCanvas {
            canvas_color: Color::rgb(200, 210, 220),
            strength: 1.0,
            apply_to: ApplyTo::Both,
        };
        let mut cell = Cell::default();
        cell.fg = Color::rgb(10, 20, 30);
        cell.bg = Color::rgb(40, 50, 60);
        filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
        assert_eq!(cell.fg, Color::rgb(200, 210, 220));
        assert_eq!(cell.bg, Color::rgb(200, 210, 220));
    }

    #[test]
    fn half_strength_blends_halfway() {
        let filter = FadeToCanvas {
            canvas_color: Color::rgb(200, 200, 200),
            strength: 0.5,
            apply_to: ApplyTo::Foreground,
        };
        let mut cell = Cell::default();
        cell.fg = Color::rgb(0, 0, 0);
        cell.bg = Color::rgb(50, 50, 50);
        filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
        // 0.5 * 200 = 100, matches the blend formula
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
        // Background untouched
        assert_eq!(cell.bg, Color::rgb(50, 50, 50));
    }

    #[test]
    fn light_canvas_does_not_flash_dark() {
        // Regression guard for the bug that motivated P0.4: at mid-exit the
        // cell must NOT be darker than both the start cell and the canvas.
        // The old `tint(black, 0.7)` hack would violate this on any light
        // canvas. FadeToCanvas should not.
        let filter = FadeToCanvas {
            canvas_color: Color::rgb(240, 240, 245), // light terminal bg
            strength: 0.5,
            apply_to: ApplyTo::Background,
        };
        let mut cell = Cell::default();
        cell.fg = Color::rgb(50, 50, 60);
        cell.bg = Color::rgb(180, 180, 190); // widget mid-tone
        filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
        let avg = (cell.bg.r as u16 + cell.bg.g as u16 + cell.bg.b as u16) / 3;
        let widget_avg = (180 + 180 + 190) / 3;
        let canvas_avg = (240 + 240 + 245) / 3;
        assert!(
            avg >= widget_avg.min(canvas_avg),
            "Mid-fade value {} darker than both widget {} and canvas {}",
            avg,
            widget_avg,
            canvas_avg,
        );
    }

    #[test]
    fn apply_to_foreground_leaves_background() {
        let filter = FadeToCanvas {
            canvas_color: Color::rgb(255, 255, 255),
            strength: 1.0,
            apply_to: ApplyTo::Foreground,
        };
        let mut cell = Cell::default();
        cell.fg = Color::rgb(0, 0, 0);
        cell.bg = Color::rgb(100, 100, 100);
        filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
        assert_eq!(cell.fg, Color::rgb(255, 255, 255));
        assert_eq!(cell.bg, Color::rgb(100, 100, 100));
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_fade_to_canvas.rs</FILE> - <DESC>Exit filter that blends cells toward a declared canvas color</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
