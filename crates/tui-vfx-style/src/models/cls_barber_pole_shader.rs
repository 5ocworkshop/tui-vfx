// <FILE>tui-vfx-style/src/models/cls_barber_pole_shader.rs</FILE> - <DESC>BarberPole shader implementation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>New primitive</WCTX>
// <CLOG>1.1.0: add channel, angle, and gap-color semantics so v3.1 barber-pole recipes can lower without backend style-stage emulation.
// 1.0.0: Initial implementation</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
/// Channels affected by the barber-pole stripe colors.
pub enum BarberPoleApplyTo {
    Foreground,
    #[default]
    Background,
    Both,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct BarberPoleShader {
    #[config(default = 1)]
    pub speed: f32,
    #[config(default = 2)]
    pub stripe_width: u16,
    #[config(default = 2)]
    pub gap_width: u16,
    /// Stripe projection angle in degrees. A literal 0.0 preserves the legacy diagonal `x + y` projection.
    #[serde(default)]
    pub angle_deg: f32,
    /// Color used for stripe cells.
    pub color: ColorConfig,
    /// Optional color used for gap cells; absent gaps leave the base style unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color: Option<ColorConfig>,
    /// Target channel(s) affected by stripe and gap colors.
    #[serde(default)]
    pub apply_to: BarberPoleApplyTo,
}
impl StyleShader for BarberPoleShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let cycle_len = self.stripe_width + self.gap_width;
        if cycle_len == 0 {
            return base;
        }

        let t = ctx.t as f32;

        // Diagonal movement: (x + y)
        // Time offset: t * speed * 10.0 (arbitrary scaling for feel)
        let offset = t * self.speed * 10.0;
        let angle = self.angle_deg.to_radians();
        let projection = if self.angle_deg == 0.0 {
            ctx.local_x as f32 + ctx.local_y as f32
        } else {
            ctx.local_x as f32 * angle.cos() + ctx.local_y as f32 * angle.sin()
        };
        let pos = (projection + offset).rem_euclid(cycle_len as f32);
        let color = if pos < self.stripe_width as f32 {
            Some(Color::from(self.color))
        } else {
            self.background_color.map(Color::from)
        };
        let Some(color) = color else {
            return base;
        };
        let mut style = base;
        match self.apply_to {
            BarberPoleApplyTo::Foreground => style.fg = color,
            BarberPoleApplyTo::Background => style.bg = color,
            BarberPoleApplyTo::Both => {
                style.fg = color;
                style.bg = color;
            }
        }
        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(local_x: u16, local_y: u16) -> ShaderContext {
        ShaderContext::new(local_x, local_y, 8, 4, 0, 0, 0.0, None, None)
    }

    fn base_style() -> Style {
        Style {
            fg: Color::rgb(1, 2, 3),
            bg: Color::rgb(4, 5, 6),
            mods: Default::default(),
        }
    }

    #[test]
    fn stripe_color_targets_requested_channel() {
        let shader = BarberPoleShader {
            speed: 0.0,
            stripe_width: 2,
            gap_width: 2,
            angle_deg: 45.0,
            color: ColorConfig::Red,
            background_color: None,
            apply_to: BarberPoleApplyTo::Foreground,
        };

        let out = shader.style_at(&ctx_at(0, 0), base_style());

        assert_eq!(out.fg, Color::from(ColorConfig::Red));
        assert_eq!(out.bg, base_style().bg);
    }

    #[test]
    fn gap_without_background_color_preserves_base_style() {
        let shader = BarberPoleShader {
            speed: 0.0,
            stripe_width: 1,
            gap_width: 3,
            angle_deg: 45.0,
            color: ColorConfig::Red,
            background_color: None,
            apply_to: BarberPoleApplyTo::Background,
        };

        let out = shader.style_at(&ctx_at(2, 0), base_style());

        assert_eq!(out, base_style());
    }

    #[test]
    fn gap_background_color_can_target_both_channels() {
        let shader = BarberPoleShader {
            speed: 0.0,
            stripe_width: 1,
            gap_width: 3,
            angle_deg: 45.0,
            color: ColorConfig::Red,
            background_color: Some(ColorConfig::Blue),
            apply_to: BarberPoleApplyTo::Both,
        };

        let out = shader.style_at(&ctx_at(2, 0), base_style());

        assert_eq!(out.fg, Color::from(ColorConfig::Blue));
        assert_eq!(out.bg, Color::from(ColorConfig::Blue));
    }

    #[test]
    fn zero_angle_preserves_legacy_diagonal_projection() {
        let shader = BarberPoleShader {
            speed: 0.0,
            stripe_width: 1,
            gap_width: 3,
            angle_deg: 0.0,
            color: ColorConfig::Red,
            background_color: None,
            apply_to: BarberPoleApplyTo::Background,
        };

        let out = shader.style_at(&ctx_at(1, 0), base_style());

        assert_eq!(out, base_style());
    }
}

// <FILE>tui-vfx-style/src/models/cls_barber_pole_shader.rs</FILE> - <DESC>BarberPole shader implementation</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
