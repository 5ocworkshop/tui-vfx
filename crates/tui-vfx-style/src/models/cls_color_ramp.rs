// <FILE>tui-vfx-style/src/models/cls_color_ramp.rs</FILE> - <DESC>Multi-stop color gradient interpolation</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T19:15:00Z</VERS>
// <WCTX>Implementing color management gaps</WCTX>
// <CLOG>Initial implementation of ColorRamp with multi-stop gradients</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;

/// A color stop in a gradient ramp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct ColorStop {
    /// Position in the gradient (0.0 to 1.0)
    pub position: f32,
    /// Color at this position
    pub color: ColorConfig,
}

impl ColorStop {
    pub fn new(position: f32, color: ColorConfig) -> Self {
        Self {
            position: if position.is_finite() {
                position.clamp(0.0, 1.0)
            } else {
                0.0
            },
            color,
        }
    }
}

/// Multi-stop color gradient for complex color transitions.
///
/// Used for effects like fire (white→yellow→orange→red→black),
/// heat maps, plasma, and rainbow effects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct ColorRamp {
    /// Color stops sorted by position (0.0 to 1.0)
    pub stops: Vec<ColorStop>,
    /// Color space for interpolation between stops
    #[serde(default)]
    pub space: ColorSpace,
}

impl ColorRamp {
    /// Create a new color ramp with the given stops.
    /// Stops are automatically sorted by position.
    pub fn new(mut stops: Vec<ColorStop>, space: ColorSpace) -> Self {
        stops.sort_by(|a, b| a.position.total_cmp(&b.position));
        Self { stops, space }
    }

    /// Create a simple two-color ramp (equivalent to linear blend).
    pub fn linear(start: ColorConfig, end: ColorConfig) -> Self {
        Self::new(
            vec![ColorStop::new(0.0, start), ColorStop::new(1.0, end)],
            ColorSpace::Rgb,
        )
    }

    /// Create a fire/heat ramp: white → yellow → orange → red → black
    pub fn fire() -> Self {
        Self::new(
            vec![
                ColorStop::new(
                    0.0,
                    ColorConfig::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                ), // white flash
                ColorStop::new(
                    0.2,
                    ColorConfig::Rgb {
                        r: 255,
                        g: 255,
                        b: 0,
                    },
                ), // yellow
                ColorStop::new(
                    0.4,
                    ColorConfig::Rgb {
                        r: 255,
                        g: 150,
                        b: 0,
                    },
                ), // orange
                ColorStop::new(
                    0.6,
                    ColorConfig::Rgb {
                        r: 255,
                        g: 50,
                        b: 0,
                    },
                ), // red-orange
                ColorStop::new(0.8, ColorConfig::Rgb { r: 150, g: 0, b: 0 }), // dark red
                ColorStop::new(1.0, ColorConfig::Rgb { r: 0, g: 0, b: 0 }),   // black/ash
            ],
            ColorSpace::Rgb,
        )
    }

    /// Create a cool/ice ramp: white → cyan → blue → dark blue
    pub fn ice() -> Self {
        Self::new(
            vec![
                ColorStop::new(
                    0.0,
                    ColorConfig::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                ),
                ColorStop::new(
                    0.3,
                    ColorConfig::Rgb {
                        r: 200,
                        g: 255,
                        b: 255,
                    },
                ),
                ColorStop::new(
                    0.6,
                    ColorConfig::Rgb {
                        r: 100,
                        g: 200,
                        b: 255,
                    },
                ),
                ColorStop::new(
                    1.0,
                    ColorConfig::Rgb {
                        r: 20,
                        g: 50,
                        b: 150,
                    },
                ),
            ],
            ColorSpace::Rgb,
        )
    }

    /// Create a rainbow ramp using HSL interpolation.
    pub fn rainbow() -> Self {
        Self::new(
            vec![
                ColorStop::new(0.0, ColorConfig::Rgb { r: 255, g: 0, b: 0 }), // red
                ColorStop::new(
                    0.17,
                    ColorConfig::Rgb {
                        r: 255,
                        g: 165,
                        b: 0,
                    },
                ), // orange
                ColorStop::new(
                    0.33,
                    ColorConfig::Rgb {
                        r: 255,
                        g: 255,
                        b: 0,
                    },
                ), // yellow
                ColorStop::new(0.5, ColorConfig::Rgb { r: 0, g: 255, b: 0 }), // green
                ColorStop::new(0.67, ColorConfig::Rgb { r: 0, g: 0, b: 255 }), // blue
                ColorStop::new(
                    0.83,
                    ColorConfig::Rgb {
                        r: 75,
                        g: 0,
                        b: 130,
                    },
                ), // indigo
                ColorStop::new(
                    1.0,
                    ColorConfig::Rgb {
                        r: 148,
                        g: 0,
                        b: 211,
                    },
                ), // violet
            ],
            ColorSpace::Hsl,
        )
    }

    /// Sample the color ramp at position t (0.0 to 1.0).
    pub fn sample(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);

        if self.stops.is_empty() {
            return Color::TRANSPARENT;
        }

        if self.stops.len() == 1 {
            return Color::from(self.stops[0].color);
        }

        // Find the two stops we're between
        let mut lower_idx = 0;
        let mut upper_idx = self.stops.len() - 1;

        for (i, stop) in self.stops.iter().enumerate() {
            if stop.position <= t {
                lower_idx = i;
            }
            if stop.position >= t && i < upper_idx {
                upper_idx = i;
                break;
            }
        }

        // If we're exactly at or beyond a stop
        if lower_idx == upper_idx {
            return Color::from(self.stops[lower_idx].color);
        }

        let lower = &self.stops[lower_idx];
        let upper = &self.stops[upper_idx];

        // Calculate local t between these two stops
        let range = upper.position - lower.position;
        let local_t = if range > 0.0 {
            (t - lower.position) / range
        } else {
            0.0
        };

        // Interpolate between the two colors
        blend_colors(
            Color::from(lower.color),
            Color::from(upper.color),
            local_t,
            self.space,
        )
    }
}

impl Default for ColorRamp {
    fn default() -> Self {
        Self::linear(ColorConfig::Black, ColorConfig::White)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_endpoints() {
        let ramp = ColorRamp::linear(
            ColorConfig::Rgb { r: 0, g: 0, b: 0 },
            ColorConfig::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        );
        assert_eq!(ramp.sample(0.0), Color::rgb(0, 0, 0));
        assert_eq!(ramp.sample(1.0), Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_sample_midpoint() {
        let ramp = ColorRamp::linear(
            ColorConfig::Rgb { r: 0, g: 0, b: 0 },
            ColorConfig::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        );
        let mid = ramp.sample(0.5);
        // tui_vfx_types::Color is always a struct with r, g, b, a fields
        assert!(mid.r >= 126 && mid.r <= 128);
        assert!(mid.g >= 126 && mid.g <= 128);
        assert!(mid.b >= 126 && mid.b <= 128);
    }

    #[test]
    fn test_multi_stop_ramp() {
        let ramp = ColorRamp::new(
            vec![
                ColorStop::new(0.0, ColorConfig::Rgb { r: 255, g: 0, b: 0 }),
                ColorStop::new(0.5, ColorConfig::Rgb { r: 0, g: 255, b: 0 }),
                ColorStop::new(1.0, ColorConfig::Rgb { r: 0, g: 0, b: 255 }),
            ],
            ColorSpace::Rgb,
        );

        // At 0.0 should be red
        assert_eq!(ramp.sample(0.0), Color::rgb(255, 0, 0));
        // At 0.5 should be green
        assert_eq!(ramp.sample(0.5), Color::rgb(0, 255, 0));
        // At 1.0 should be blue
        assert_eq!(ramp.sample(1.0), Color::rgb(0, 0, 255));

        // At 0.25 should be between red and green
        let quarter = ramp.sample(0.25);
        // tui_vfx_types::Color is always a struct with r, g, b, a fields
        assert!(quarter.r > 100); // Still has red
        assert!(quarter.g > 100); // Has some green
    }

    #[test]
    fn test_fire_ramp() {
        let ramp = ColorRamp::fire();
        // Should start white-ish
        let start = ramp.sample(0.0);
        // tui_vfx_types::Color is always a struct with r, g, b, a fields
        assert_eq!(start.r, 255);
        assert_eq!(start.g, 255);
        assert_eq!(start.b, 255);
        // Should end black
        let end = ramp.sample(1.0);
        assert_eq!(end, Color::rgb(0, 0, 0));
    }
}

// <FILE>tui-vfx-style/src/models/cls_color_ramp.rs</FILE> - <DESC>Multi-stop color gradient interpolation</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T19:15:00Z</VERS>
