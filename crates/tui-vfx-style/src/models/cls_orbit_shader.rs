// <FILE>tui-vfx-style/src/models/cls_orbit_shader.rs</FILE> - <DESC>Orbiting dot shader implementation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Orbit is authored as visible dots around a center; the shader must make those dots visible even when they land on blank source cells.</WCTX>
// <CLOG>1.1.0: color both foreground and background at orbit positions and reconnect speed as a loop_t multiplier so orbit debug recipes show moving dots around the center. 1.0.1: remove self.speed from positional computation; caller controls sweep rate via loop_t.</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Orbiting dots around the widget center.
///
/// The shader marks dot positions by coloring both foreground and background.
/// That keeps orbit recipes legible on sparse text/card surfaces where the dot
/// often lands on a blank cell: the background color provides the visible
/// terminal "dot" while the foreground color keeps non-blank cells consistent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct OrbitShader {
    /// Animation speed multiplier applied to normalized shader time.
    ///
    /// `1.0` completes one orbit per loop. Higher values complete multiple
    /// revolutions per loop; lower values move more slowly.
    #[config(default = 1.0)]
    pub speed: f32,
    /// Number of dots in the orbit.
    #[config(default = 3)]
    pub dot_count: u8,
    /// Color of the orbiting dots.
    pub color: ColorConfig,
}

impl StyleShader for OrbitShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.dot_count == 0 {
            return base;
        }

        let width = ctx.width as f32;
        let height = ctx.height as f32;
        if width <= 1.0 || height <= 1.0 {
            return base;
        }

        let cx = (width - 1.0) / 2.0;
        let cy = (height - 1.0) / 2.0;
        let radius = cx.min(cy);
        if !radius.is_finite() || radius <= 0.0 {
            return base;
        }

        let mut style = base;
        let speed = if self.speed.is_finite() {
            self.speed.max(0.0)
        } else {
            1.0
        };
        let base_angle = ctx.t as f32 * speed * std::f32::consts::TAU;
        let dot_count = self.dot_count as f32;

        for i in 0..self.dot_count {
            let angle = base_angle + (i as f32) * std::f32::consts::TAU / dot_count;
            let x = (cx + radius * angle.cos()).round() as i32;
            let y = (cy + radius * angle.sin()).round() as i32;

            if x == ctx.local_x as i32 && y == ctx.local_y as i32 {
                let color = Color::from(self.color);
                style.fg = color;
                style.bg = color;
                break;
            }
        }

        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(local_x: u16, local_y: u16, t: f64) -> ShaderContext {
        ShaderContext::new(local_x, local_y, 9, 5, 0, 0, t, None, None)
    }

    fn shader(speed: f32) -> OrbitShader {
        OrbitShader {
            speed,
            dot_count: 1,
            color: ColorConfig::Cyan,
        }
    }

    #[test]
    fn orbit_marks_blank_cells_with_visible_background_dots() {
        let out = shader(1.0).style_at(&ctx_at(6, 2, 0.0), Style::default());

        assert_eq!(out.fg, Color::from(ColorConfig::Cyan));
        assert_eq!(out.bg, Color::from(ColorConfig::Cyan));
    }

    #[test]
    fn orbit_animates_across_time() {
        let orbit = shader(1.0);
        let right_at_start = orbit.style_at(&ctx_at(6, 2, 0.0), Style::default());
        let right_at_quarter = orbit.style_at(&ctx_at(6, 2, 0.25), Style::default());
        let bottom_at_quarter = orbit.style_at(&ctx_at(4, 4, 0.25), Style::default());

        assert_eq!(right_at_start.bg, Color::from(ColorConfig::Cyan));
        assert_ne!(right_at_quarter.bg, Color::from(ColorConfig::Cyan));
        assert_eq!(bottom_at_quarter.bg, Color::from(ColorConfig::Cyan));
    }

    #[test]
    fn orbit_speed_affects_phase() {
        let normal = shader(1.0).style_at(&ctx_at(4, 4, 0.25), Style::default());
        let faster = shader(2.0).style_at(&ctx_at(4, 4, 0.25), Style::default());
        let faster_right = shader(2.0).style_at(&ctx_at(2, 2, 0.25), Style::default());

        assert_eq!(normal.bg, Color::from(ColorConfig::Cyan));
        assert_ne!(faster.bg, Color::from(ColorConfig::Cyan));
        assert_eq!(faster_right.bg, Color::from(ColorConfig::Cyan));
    }
}

// <FILE>tui-vfx-style/src/models/cls_orbit_shader.rs</FILE> - <DESC>Orbiting dot shader implementation</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
