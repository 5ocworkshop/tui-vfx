// <FILE>tui-vfx-style/src/models/cls_fade_effect.rs</FILE>
// <DESC>Generalized color-fade primitive (formerly FadeToBlack) plus reusable fade_effect combinator</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>The render path used FadeToBlack and hard-coded Color::BLACK as the fade target, which destroyed non-default canvas colors during enter/exit on design-system recipes; generalize to an arbitrary fade color while preserving backward compatibility</WCTX>
// <CLOG>MINOR: Generalized FadeToBlack to carry a `color: Color` field that defaults to Color::BLACK via #[serde(default)]. The struct name is kept (widely used downstream) and a `FadeToColor` type alias is added to reflect the new semantic. calculate() now blends via blend_colors toward self.color instead of darken() toward black; with color=BLACK it is bit-for-bit identical to the old behavior (darken == blend toward black in RGB space). New constructors fade_in_from(color) / fade_out_to(color) expose the capability. fade_style_to_black renamed to fade_style_to_color and takes an explicit color arg</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

use crate::models::cls_fade_spec::FadeApplyTo;
use crate::models::cls_color_space::ColorSpace;
use crate::traits::StyleInterpolator;
use crate::utils::apply_easing;
use crate::utils::fnc_blend_colors::blend_colors;
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

fn default_fade_color() -> Color {
    Color::BLACK
}

/// Generalized color-fade style interpolator.
///
/// - `In`: `color` → base
/// - `Out`: base → `color`
///
/// The struct is named `FadeToBlack` for historical reasons (and because
/// hundreds of downstream call sites use that name). The `color` field
/// defaults to `Color::BLACK` via serde, so recipes and code that don't
/// specify a color behave **exactly** as before — the old `darken(c, amount)`
/// math and `blend_colors(c, BLACK, amount, Rgb)` produce the same result.
///
/// **Prefer setting `color` to your canvas color for design-system
/// recipes** — otherwise fading in over a non-black canvas produces a
/// visible near-black flash during the first ~30% of the fade as the widget
/// stamps black over the canvas before reaching the base color.
///
/// The type alias [`FadeToColor`] is provided as the semantically correct
/// name for new code; the aliased type is identical.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct FadeToBlack {
    pub direction: FadeDirection,
    pub apply_to: FadeApplyTo,
    pub ease: EasingCurve,
    /// Target color for the fade (`In` fades from this color, `Out` fades to it).
    /// Defaults to `Color::BLACK` for backward compatibility.
    #[serde(default = "default_fade_color")]
    #[config(opaque)]
    pub color: Color,
}

/// Semantically accurate name for [`FadeToBlack`] now that it supports any
/// fade target color. Prefer this name in new code; it's a plain type alias,
/// so `FadeToColor` and `FadeToBlack` are interchangeable at every call site.
pub type FadeToColor = FadeToBlack;

impl Default for FadeToBlack {
    fn default() -> Self {
        Self {
            direction: FadeDirection::In,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
            color: Color::BLACK,
        }
    }
}

impl FadeToBlack {
    /// Fade in from black. Equivalent to `fade_in_from(Color::BLACK)`.
    pub const fn fade_in() -> Self {
        Self {
            direction: FadeDirection::In,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
            color: Color::BLACK,
        }
    }

    /// Fade out to black. Equivalent to `fade_out_to(Color::BLACK)`.
    pub const fn fade_out() -> Self {
        Self {
            direction: FadeDirection::Out,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
            color: Color::BLACK,
        }
    }

    /// Fade in from a specific color (typically the canvas color). Use this
    /// on design-system recipes to avoid the black-flash artifact on enter.
    pub const fn fade_in_from(color: Color) -> Self {
        Self {
            direction: FadeDirection::In,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
            color,
        }
    }

    /// Fade out to a specific color (typically the canvas color). Symmetric
    /// with [`fade_in_from`](Self::fade_in_from) for a clean exit.
    pub const fn fade_out_to(color: Color) -> Self {
        Self {
            direction: FadeDirection::Out,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
            color,
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

    /// Set the fade target color. For `In` the style fades from this color to
    /// base; for `Out` it fades from base to this color.
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl StyleInterpolator for FadeToBlack {
    fn calculate(&self, t: f64, base: Style) -> Style {
        let t = t.clamp(0.0, 1.0);
        let eased_t = apply_easing(t, self.ease);
        // In: at t=0 we want `color`, at t=1 we want base → blend(color, base, eased_t)
        // Out: at t=0 we want base, at t=1 we want `color` → blend(base, color, eased_t)
        fade_style_to_color(base, self.direction, eased_t, self.color, self.apply_to)
    }
}

fn fade_style_to_color(
    style: Style,
    direction: FadeDirection,
    eased_t: f32,
    color: Color,
    apply_to: FadeApplyTo,
) -> Style {
    let mut result = style;

    let fade_color = |base_color: Color| -> Color {
        match direction {
            FadeDirection::In => blend_colors(color, base_color, eased_t, ColorSpace::Rgb),
            FadeDirection::Out => blend_colors(base_color, color, eased_t, ColorSpace::Rgb),
        }
    };

    if matches!(apply_to, FadeApplyTo::Foreground | FadeApplyTo::Both)
        && style.fg != Color::TRANSPARENT
    {
        result.fg = fade_color(style.fg);
    }

    if matches!(apply_to, FadeApplyTo::Background | FadeApplyTo::Both)
        && style.bg != Color::TRANSPARENT
    {
        result.bg = fade_color(style.bg);
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
// <DESC>Generalized color-fade primitive (formerly FadeToBlack) plus reusable fade_effect combinator</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
