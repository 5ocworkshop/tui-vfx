// <FILE>tui-vfx-style/src/models/cls_fade_effect.rs</FILE>
// <DESC>Fade-to-black primitives and a reusable fade_effect combinator</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-24</VERS>
// <WCTX>Easing Curves Expansion: Full Bezier support in tui-style-fx</WCTX>
// <CLOG>BREAKING: Updated to use EasingCurve instead of EasingType for Bezier support</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

use crate::models::cls_fade_spec::FadeApplyTo;
use crate::traits::StyleInterpolator;
use crate::utils::{apply_easing, darken};
use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum FadeDirection {
    #[default]
    In,
    Out,
}

/// Fade-to-black style interpolator.
///
/// - `In`: black → base
/// - `Out`: base → black
///
/// This is intentionally focused on the legacy parity requirement: interpolate colors to black.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct FadeToBlack {
    pub direction: FadeDirection,
    pub apply_to: FadeApplyTo,
    pub ease: EasingCurve,
}

impl Default for FadeToBlack {
    fn default() -> Self {
        Self {
            direction: FadeDirection::In,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
        }
    }
}

impl FadeToBlack {
    pub const fn fade_in() -> Self {
        Self {
            direction: FadeDirection::In,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
        }
    }

    pub const fn fade_out() -> Self {
        Self {
            direction: FadeDirection::Out,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
        }
    }

    pub const fn with_apply_to(mut self, apply_to: FadeApplyTo) -> Self {
        self.apply_to = apply_to;
        self
    }

    pub const fn with_ease(mut self, ease: EasingCurve) -> Self {
        self.ease = ease;
        self
    }
}

impl StyleInterpolator for FadeToBlack {
    fn calculate(&self, t: f64, base: Style) -> Style {
        let t = t.clamp(0.0, 1.0);
        let eased_t = apply_easing(t, self.ease);
        let amount = match self.direction {
            FadeDirection::In => 1.0 - eased_t,
            FadeDirection::Out => eased_t,
        };

        fade_style_to_black(base, amount, self.apply_to)
    }
}

fn fade_style_to_black(style: Style, amount: f32, apply_to: FadeApplyTo) -> Style {
    let mut result = style;

    fn fade_color(c: Color, amount: f32) -> Color {
        darken(c, amount)
    }

    if matches!(apply_to, FadeApplyTo::Foreground | FadeApplyTo::Both)
        && style.fg != Color::TRANSPARENT
    {
        result.fg = fade_color(style.fg, amount);
    }

    if matches!(apply_to, FadeApplyTo::Background | FadeApplyTo::Both)
        && style.bg != Color::TRANSPARENT
    {
        result.bg = fade_color(style.bg, amount);
    }

    result
}

/// A reusable composition wrapper: apply `fade` to the output of `inner`.
///
/// Contract: `FadeEffect.calculate(t, base) == fade.calculate(t, inner.calculate(t, base))`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FadeEffect<I> {
    pub inner: I,
    pub fade: FadeToBlack,
}

impl<I: StyleInterpolator> StyleInterpolator for FadeEffect<I> {
    fn calculate(&self, t: f64, base: Style) -> Style {
        let inner_style = self.inner.calculate(t, base);
        self.fade.calculate(t, inner_style)
    }
}

/// Convenience constructor for the `FadeEffect` combinator.
pub fn fade_effect<I: StyleInterpolator>(inner: I, fade: FadeToBlack) -> FadeEffect<I> {
    FadeEffect { inner, fade }
}

// <FILE>tui-vfx-style/src/models/cls_fade_effect.rs</FILE>
// <DESC>Fade-to-black primitives and a reusable fade_effect combinator</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-24</VERS>
