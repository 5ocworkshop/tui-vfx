// <FILE>tui-vfx-style/src/models/cls_style_transition.rs</FILE> - <DESC>Definition of StyleTransition struct</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-24</VERS>
// <WCTX>Easing Curves Expansion: Full Bezier support in tui-style-fx</WCTX>
// <CLOG>BREAKING: Updated to use EasingCurve instead of EasingType for Bezier support</CLOG>

use super::ColorSpace;
use crate::traits::StyleInterpolator;
use crate::utils::{apply_easing, blend_colors};
use serde::{Deserialize, Serialize};
use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_types::{Color, Style};

use crate::models::StyleConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct StyleTransition {
    pub start: Style,
    pub end: Style,
    pub ease: EasingCurve,
    pub color_space: ColorSpace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StyleTransitionSerde {
    start: StyleConfig,
    end: StyleConfig,
    ease: EasingCurve,
    color_space: ColorSpace,
}

impl From<&StyleTransition> for StyleTransitionSerde {
    fn from(value: &StyleTransition) -> Self {
        Self {
            start: StyleConfig::from(value.start),
            end: StyleConfig::from(value.end),
            ease: value.ease,
            color_space: value.color_space,
        }
    }
}

impl From<StyleTransitionSerde> for StyleTransition {
    fn from(value: StyleTransitionSerde) -> Self {
        Self {
            start: Style::from(value.start),
            end: Style::from(value.end),
            ease: value.ease,
            color_space: value.color_space,
        }
    }
}

impl Serialize for StyleTransition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        StyleTransitionSerde::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StyleTransition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = StyleTransitionSerde::deserialize(deserializer)?;
        Ok(Self::from(value))
    }
}
impl StyleTransition {
    pub fn new(start: Style, end: Style) -> Self {
        Self {
            start,
            end,
            ease: EasingCurve::Type(EasingType::Linear),
            color_space: ColorSpace::Rgb,
        }
    }
    pub fn with_ease(mut self, ease: EasingCurve) -> Self {
        self.ease = ease;
        self
    }
    pub fn with_color_space(mut self, space: ColorSpace) -> Self {
        self.color_space = space;
        self
    }
}
impl StyleInterpolator for StyleTransition {
    fn calculate(&self, t: f64, base: Style) -> Style {
        // Apply easing to the normalized time
        let t = t.clamp(0.0, 1.0);
        let eased_t = apply_easing(t, self.ease);
        let mut result = base;
        // Blend FG
        if self.start.fg != Color::TRANSPARENT && self.end.fg != Color::TRANSPARENT {
            result.fg = blend_colors(self.start.fg, self.end.fg, eased_t, self.color_space);
        } else if self.end.fg != Color::TRANSPARENT {
            if base.fg != Color::TRANSPARENT {
                result.fg = blend_colors(base.fg, self.end.fg, eased_t, self.color_space);
            } else {
                result.fg = self.end.fg;
            }
        }
        // Blend BG
        if self.start.bg != Color::TRANSPARENT && self.end.bg != Color::TRANSPARENT {
            result.bg = blend_colors(self.start.bg, self.end.bg, eased_t, self.color_space);
        } else if self.end.bg != Color::TRANSPARENT {
            if base.bg != Color::TRANSPARENT {
                result.bg = blend_colors(base.bg, self.end.bg, eased_t, self.color_space);
            } else {
                result.bg = self.end.bg;
            }
        }
        // Modifiers: For v1, we just take the End modifiers if t > 0.5
        if eased_t > 0.5 {
            result.mods = self.end.mods;
        } else {
            result.mods = self.start.mods;
        }
        result
    }
}

// <FILE>tui-vfx-style/src/models/cls_style_transition.rs</FILE> - <DESC>Definition of StyleTransition struct</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-24</VERS>
