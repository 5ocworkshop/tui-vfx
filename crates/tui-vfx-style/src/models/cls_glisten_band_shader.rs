// <FILE>tui-vfx-style/src/models/cls_glisten_band_shader.rs</FILE> - <DESC>GlistenBand shader implementation</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>Schema V2.2 standardization</WCTX>
// <CLOG>Renamed GlistenApplyTo variants Fg/Bg to Foreground/Background for consistency</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Direction for the glisten band sweep.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum GlistenDirection {
    /// Sweep from start to end (default)
    #[default]
    Forward,
    /// Sweep from end to start
    Reverse,
    /// Oscillate back and forth
    PingPong,
}

/// Which color channel(s) to apply the glisten effect to.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum GlistenApplyTo {
    /// Apply to foreground only (default)
    #[serde(alias = "fg")]
    #[default]
    Foreground,
    /// Apply to background only
    #[serde(alias = "bg")]
    Background,
    /// Apply to both foreground and background
    Both,
}

/// A moving gradient band that sweeps across the widget at an angle.
///
/// Creates a "shimmer" or "glisten" effect like light reflecting off a surface.
/// Used for premium/gold effects like `golden_marquee`.
///
/// The band moves from one side to the other based on `t` and `speed`,
/// with colors transitioning from `tail` -> `head` -> `tail` within the band.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct GlistenBandShader {
    /// Speed multiplier for band movement (1.0 = normal)
    #[serde(default = "default_speed")]
    pub speed: f32,
    /// Width of the glisten band in cells
    #[serde(default = "default_band_width")]
    pub band_width: u16,
    /// Angle of the band in degrees (0 = horizontal, 90 = vertical)
    #[serde(default = "default_angle")]
    pub angle_deg: f32,
    /// Color at the leading edge of the band (brightest point)
    pub head: ColorConfig,
    /// Color at the trailing edge of the band (fades into base)
    pub tail: ColorConfig,
    /// Direction of the sweep
    #[serde(default)]
    pub direction: GlistenDirection,
    /// Number of times to repeat (0 = continuous loop)
    #[serde(default)]
    pub repeat_count: u8,
    /// Which channel(s) to apply the effect to
    #[serde(default)]
    pub apply_to: GlistenApplyTo,
    /// How strongly the glisten blends with the base (0.0-1.0, default 0.7)
    #[serde(default = "default_blend_strength")]
    pub blend_strength: f32,
}

fn default_speed() -> f32 {
    1.0
}
fn default_band_width() -> u16 {
    6
}
fn default_angle() -> f32 {
    25.0
}
fn default_blend_strength() -> f32 {
    0.7
}

impl Default for GlistenBandShader {
    fn default() -> Self {
        Self {
            speed: 1.0,
            band_width: 6,
            angle_deg: 25.0,
            head: ColorConfig::LightYellow,
            tail: ColorConfig::Rgb {
                r: 200,
                g: 160,
                b: 0,
            },
            direction: GlistenDirection::Forward,
            repeat_count: 0,
            apply_to: GlistenApplyTo::Foreground,
            blend_strength: 0.7,
        }
    }
}

impl StyleShader for GlistenBandShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let t = ctx.t as f32;
        let (x, y, width, height) = (ctx.local_x, ctx.local_y, ctx.width, ctx.height);

        // Convert angle to radians
        let angle_rad = self.angle_deg.to_radians();
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();

        // Project the cell position onto the band's normal vector
        // This gives us the "distance" along the direction the band moves
        let proj = (x as f32) * cos_a + (y as f32) * sin_a;

        // Calculate the maximum projection to normalize
        let max_proj = (width as f32) * cos_a.abs() + (height as f32) * sin_a.abs();

        // Calculate effective t based on direction and repeat_count
        let effective_t = {
            let scaled_t = t * self.speed;
            let cycle_t = if self.repeat_count > 0 {
                // Clamp to repeat_count cycles
                scaled_t.min(self.repeat_count as f32)
            } else {
                scaled_t
            };

            match self.direction {
                GlistenDirection::Forward => cycle_t.fract(),
                GlistenDirection::Reverse => 1.0 - cycle_t.fract(),
                GlistenDirection::PingPong => {
                    // Oscillate: 0->1->0->1...
                    let phase = cycle_t.fract() * 2.0;
                    if phase <= 1.0 { phase } else { 2.0 - phase }
                }
            }
        };

        // Band position moves based on effective_t
        // The band sweeps across the entire range plus its width
        let sweep_range = max_proj + self.band_width as f32;
        let band_center = effective_t * sweep_range - (self.band_width as f32 / 2.0);

        // Calculate distance from the band center
        let dist_from_center = (proj - band_center).abs();
        let half_band = self.band_width as f32 / 2.0;

        // If outside the band, return base style
        if dist_from_center > half_band {
            return base;
        }

        // Calculate intensity within the band (1.0 at center, 0.0 at edges)
        let intensity = 1.0 - (dist_from_center / half_band);

        // Blend from tail -> head based on intensity
        let head_color: Color = self.head.into();
        let tail_color: Color = self.tail.into();

        // Extract RGB from colors (tui_vfx_types::Color is always RGB with alpha)
        let (hr, hg, hb) = (head_color.r, head_color.g, head_color.b);
        let (tr, tg, tb) = (tail_color.r, tail_color.g, tail_color.b);

        // Blend tail -> head based on intensity
        let r = ((tr as f32) + ((hr as f32 - tr as f32) * intensity)) as u8;
        let g = ((tg as f32) + ((hg as f32 - tg as f32) * intensity)) as u8;
        let b = ((tb as f32) + ((hb as f32 - tb as f32) * intensity)) as u8;

        let glisten_color = Color::rgb(r, g, b);
        let blend_amount = intensity * self.blend_strength;

        // Apply the glisten color based on apply_to setting
        let mut style = base;
        match self.apply_to {
            GlistenApplyTo::Foreground => {
                if base.fg != Color::TRANSPARENT {
                    style.fg = blend_colors(base.fg, glisten_color, blend_amount, ColorSpace::Rgb);
                } else {
                    style.fg = glisten_color;
                }
            }
            GlistenApplyTo::Background => {
                if base.bg != Color::TRANSPARENT {
                    style.bg = blend_colors(base.bg, glisten_color, blend_amount, ColorSpace::Rgb);
                } else {
                    style.bg = glisten_color;
                }
            }
            GlistenApplyTo::Both => {
                if base.fg != Color::TRANSPARENT {
                    style.fg = blend_colors(base.fg, glisten_color, blend_amount, ColorSpace::Rgb);
                } else {
                    style.fg = glisten_color;
                }
                if base.bg != Color::TRANSPARENT {
                    style.bg =
                        blend_colors(base.bg, glisten_color, blend_amount * 0.5, ColorSpace::Rgb);
                }
            }
        }

        style
    }
}

// <FILE>tui-vfx-style/src/models/cls_glisten_band_shader.rs</FILE> - <DESC>GlistenBand shader implementation</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-22</VERS>
