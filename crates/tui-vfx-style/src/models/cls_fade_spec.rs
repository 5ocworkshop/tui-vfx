// <FILE>tui-vfx-style/src/models/cls_fade_spec.rs</FILE> - <DESC>General color-to-color fade with chaining</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>mixed-signals migration: envelope adoption</WCTX>
// <CLOG>Added optional LinearEnvelope for attack/hold/release fade shaping</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::StyleInterpolator;
use crate::utils::fnc_blend_colors::blend_colors;
use mixed_signals::envelopes::LinearEnvelope;
use mixed_signals::traits::Signal;
use serde::{Deserialize, Serialize};
use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_types::{Color, Style};

/// Target color for a fade operation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[derive(Default)]
pub enum FadeTarget {
    /// Fade to/from black (RGB 0,0,0)
    #[default]
    Black,
    /// Fade to/from white (RGB 255,255,255)
    White,
    /// Fade to/from transparent (removes the color)
    Transparent,
    /// Fade to/from the base color (no change - useful in chains)
    Base,
    /// Fade to/from a specific color
    Color {
        #[serde(flatten)]
        color: ColorConfig,
    },
}

impl FadeTarget {
    /// Resolve the target to an actual Color, given the base color.
    pub fn resolve(&self, base: Option<Color>) -> Option<Color> {
        match self {
            FadeTarget::Black => Some(Color::rgb(0, 0, 0)),
            FadeTarget::White => Some(Color::rgb(255, 255, 255)),
            FadeTarget::Transparent => None,
            FadeTarget::Base => base,
            FadeTarget::Color { color } => Some(Color::from(*color)),
        }
    }
}

/// Which color component(s) the fade applies to.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum FadeApplyTo {
    /// Apply fade to foreground color only
    #[serde(alias = "fg")]
    Foreground,
    /// Apply fade to background color only
    #[serde(alias = "bg")]
    Background,
    /// Apply fade to both foreground and background
    #[default]
    Both,
}

/// Optional envelope configuration for fade attack/hold/release shaping.
///
/// When specified, modulates the fade progress with a LinearEnvelope
/// for smoother attack and release phases.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct FadeEnvelope {
    /// Attack phase duration as fraction of total (0..1)
    #[serde(default = "default_attack")]
    pub attack: f32,
    /// Release phase duration as fraction of total (0..1)
    #[serde(default = "default_release")]
    pub release: f32,
}

fn default_attack() -> f32 {
    0.1
}

fn default_release() -> f32 {
    0.1
}

impl FadeEnvelope {
    /// Create a new envelope with specified attack and release.
    pub fn new(attack: f32, release: f32) -> Self {
        Self { attack, release }
    }

    /// Create a symmetric envelope (equal attack and release).
    pub fn symmetric(time: f32) -> Self {
        Self {
            attack: time,
            release: time,
        }
    }

    /// Convert to a mixed_signals LinearEnvelope.
    pub fn to_linear_envelope(&self) -> LinearEnvelope {
        LinearEnvelope::new(self.attack, self.release)
    }
}

impl Default for FadeEnvelope {
    fn default() -> Self {
        Self {
            attack: 0.1,
            release: 0.1,
        }
    }
}

/// A single fade specification: interpolate from one color to another.
///
/// This is the general-purpose fade that can fade between any two colors,
/// not just to/from black. Supports various color spaces for interpolation.
///
/// # Examples
///
/// ```ignore
/// // Fade from red to blue
/// let fade = FadeSpec::color_to_color(
///     ColorConfig::Rgb { r: 255, g: 0, b: 0 },
///     ColorConfig::Rgb { r: 0, g: 0, b: 255 },
/// );
///
/// // Fade in from black
/// let fade_in = FadeSpec::from_black();
///
/// // Fade out to white
/// let fade_out = FadeSpec::to_white();
///
/// // Fade with envelope (soft attack/release)
/// let soft_fade = FadeSpec::from_black()
///     .with_envelope(FadeEnvelope::symmetric(0.2));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct FadeSpec {
    /// Starting color at t=0 (None means use base color)
    #[serde(default)]
    pub from: FadeTarget,
    /// Ending color at t=1 (None means use base color)
    #[serde(default)]
    pub to: FadeTarget,
    /// Which components to apply the fade to
    #[serde(default)]
    pub apply_to: FadeApplyTo,
    /// Easing function for the interpolation
    #[serde(default)]
    pub ease: EasingCurve,
    /// Color space for interpolation (RGB or HSL)
    #[serde(default)]
    pub space: ColorSpace,
    /// Optional envelope for attack/hold/release shaping.
    /// When present, modulates the fade progress with soft attack and release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelope: Option<FadeEnvelope>,
}

impl Default for FadeSpec {
    fn default() -> Self {
        Self {
            from: FadeTarget::Black,
            to: FadeTarget::Base,
            apply_to: FadeApplyTo::Both,
            ease: EasingCurve::Type(EasingType::Linear),
            space: ColorSpace::Rgb,
            envelope: None,
        }
    }
}

impl FadeSpec {
    /// Create a fade from black to base color (fade in).
    pub fn from_black() -> Self {
        Self {
            from: FadeTarget::Black,
            to: FadeTarget::Base,
            ..Default::default()
        }
    }

    /// Create a fade from base color to black (fade out).
    pub fn to_black() -> Self {
        Self {
            from: FadeTarget::Base,
            to: FadeTarget::Black,
            ..Default::default()
        }
    }

    /// Create a fade from white to base color.
    pub fn from_white() -> Self {
        Self {
            from: FadeTarget::White,
            to: FadeTarget::Base,
            ..Default::default()
        }
    }

    /// Create a fade from base color to white.
    pub fn to_white() -> Self {
        Self {
            from: FadeTarget::Base,
            to: FadeTarget::White,
            ..Default::default()
        }
    }

    /// Create a fade between two specific colors.
    pub fn color_to_color(from: ColorConfig, to: ColorConfig) -> Self {
        Self {
            from: FadeTarget::Color { color: from },
            to: FadeTarget::Color { color: to },
            ..Default::default()
        }
    }

    /// Create a fade from a specific color to base.
    pub fn from_color(color: ColorConfig) -> Self {
        Self {
            from: FadeTarget::Color { color },
            to: FadeTarget::Base,
            ..Default::default()
        }
    }

    /// Create a fade from base to a specific color.
    pub fn to_color(color: ColorConfig) -> Self {
        Self {
            from: FadeTarget::Base,
            to: FadeTarget::Color { color },
            ..Default::default()
        }
    }

    /// Set the easing function.
    pub fn with_ease(mut self, ease: EasingCurve) -> Self {
        self.ease = ease;
        self
    }

    /// Set which components to apply to.
    pub fn with_apply_to(mut self, apply_to: FadeApplyTo) -> Self {
        self.apply_to = apply_to;
        self
    }

    /// Set the color space for interpolation.
    pub fn with_space(mut self, space: ColorSpace) -> Self {
        self.space = space;
        self
    }

    /// Set an envelope for attack/hold/release shaping.
    ///
    /// When an envelope is set, the fade progress is modulated by a LinearEnvelope
    /// from mixed-signals. This creates smooth attack and release phases.
    pub fn with_envelope(mut self, envelope: FadeEnvelope) -> Self {
        self.envelope = Some(envelope);
        self
    }

    /// Apply the fade to a single color.
    fn fade_color(&self, base: Color, t: f32) -> Color {
        // For tui_vfx_types::Color, use TRANSPARENT instead of None
        let base_opt = if base == Color::TRANSPARENT {
            None
        } else {
            Some(base)
        };
        let from_color = self.from.resolve(base_opt);
        let to_color = self.to.resolve(base_opt);

        match (from_color, to_color) {
            (Some(from), Some(to)) => blend_colors(from, to, t, self.space),
            (Some(from), None) => {
                // Fading to transparent - blend towards transparent
                if t < 0.5 { from } else { Color::TRANSPARENT }
            }
            (None, Some(to)) => {
                // Fading from transparent
                if t > 0.5 { to } else { Color::TRANSPARENT }
            }
            (None, None) => Color::TRANSPARENT,
        }
    }
}

impl StyleInterpolator for FadeSpec {
    fn calculate(&self, t: f64, base: Style) -> Style {
        let t = t.clamp(0.0, 1.0);
        let eased_t = crate::utils::apply_easing(t, self.ease);

        // Apply envelope modulation if present
        let final_t = if let Some(ref envelope) = self.envelope {
            let linear_env = envelope.to_linear_envelope();
            // Envelope output (0..1) modulates the eased progress
            eased_t * linear_env.sample(t)
        } else {
            eased_t
        };

        let mut result = base;

        if matches!(self.apply_to, FadeApplyTo::Foreground | FadeApplyTo::Both) {
            result.fg = self.fade_color(base.fg, final_t);
        }

        if matches!(self.apply_to, FadeApplyTo::Background | FadeApplyTo::Both) {
            result.bg = self.fade_color(base.bg, final_t);
        }

        result
    }
}

/// A chain of fade specifications applied in sequence.
///
/// Each fade in the chain has a weight that determines what portion
/// of the total time it occupies. The weights are normalized so they
/// sum to 1.0.
///
/// # Example: Fire effect (white → yellow → orange → red → black)
///
/// ```ignore
/// let fire_fade = FadeChain::new(vec![
///     (FadeSpec::color_to_color(white, yellow), 0.2),
///     (FadeSpec::color_to_color(yellow, orange), 0.3),
///     (FadeSpec::color_to_color(orange, red), 0.3),
///     (FadeSpec::color_to_color(red, black), 0.2),
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct FadeChain {
    /// List of (fade_spec, weight) pairs
    pub segments: Vec<FadeSegment>,
}

/// A single segment in a fade chain.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct FadeSegment {
    /// The fade specification for this segment
    pub fade: FadeSpec,
    /// Relative weight of this segment (normalized with other segments)
    #[serde(default = "default_weight")]
    pub weight: f32,
}

fn default_weight() -> f32 {
    1.0
}

impl FadeChain {
    /// Create a new fade chain from segments with weights.
    pub fn new(segments: Vec<(FadeSpec, f32)>) -> Self {
        Self {
            segments: segments
                .into_iter()
                .map(|(fade, weight)| FadeSegment { fade, weight })
                .collect(),
        }
    }

    /// Create a simple two-step fade chain.
    pub fn two_step(first: FadeSpec, second: FadeSpec) -> Self {
        Self::new(vec![(first, 1.0), (second, 1.0)])
    }

    /// Create a fire-style fade chain (white → yellow → orange → red → black).
    pub fn fire() -> Self {
        Self::new(vec![
            (
                FadeSpec::color_to_color(
                    ColorConfig::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    ColorConfig::Rgb {
                        r: 255,
                        g: 255,
                        b: 0,
                    },
                ),
                0.15,
            ),
            (
                FadeSpec::color_to_color(
                    ColorConfig::Rgb {
                        r: 255,
                        g: 255,
                        b: 0,
                    },
                    ColorConfig::Rgb {
                        r: 255,
                        g: 150,
                        b: 0,
                    },
                ),
                0.25,
            ),
            (
                FadeSpec::color_to_color(
                    ColorConfig::Rgb {
                        r: 255,
                        g: 150,
                        b: 0,
                    },
                    ColorConfig::Rgb {
                        r: 255,
                        g: 50,
                        b: 0,
                    },
                ),
                0.30,
            ),
            (
                FadeSpec::color_to_color(
                    ColorConfig::Rgb {
                        r: 255,
                        g: 50,
                        b: 0,
                    },
                    ColorConfig::Rgb { r: 0, g: 0, b: 0 },
                ),
                0.30,
            ),
        ])
    }

    /// Create an ice-style fade chain (white → cyan → blue).
    pub fn ice() -> Self {
        Self::new(vec![
            (
                FadeSpec::color_to_color(
                    ColorConfig::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    ColorConfig::Rgb {
                        r: 200,
                        g: 255,
                        b: 255,
                    },
                ),
                0.3,
            ),
            (
                FadeSpec::color_to_color(
                    ColorConfig::Rgb {
                        r: 200,
                        g: 255,
                        b: 255,
                    },
                    ColorConfig::Rgb {
                        r: 100,
                        g: 200,
                        b: 255,
                    },
                ),
                0.4,
            ),
            (
                FadeSpec::color_to_color(
                    ColorConfig::Rgb {
                        r: 100,
                        g: 200,
                        b: 255,
                    },
                    ColorConfig::Rgb {
                        r: 20,
                        g: 50,
                        b: 150,
                    },
                ),
                0.3,
            ),
        ])
    }

    /// Get the total weight of all segments.
    fn total_weight(&self) -> f32 {
        self.segments.iter().map(|s| s.weight.max(0.0)).sum()
    }

    /// Find which segment and local progress for a given global t.
    fn find_segment(&self, t: f64) -> Option<(&FadeSpec, f64)> {
        if self.segments.is_empty() {
            return None;
        }

        let total = self.total_weight() as f64;
        if total <= 0.0 {
            return None;
        }

        let mut accumulated = 0.0;
        for segment in &self.segments {
            let segment_duration = segment.weight.max(0.0) as f64 / total;
            if t <= accumulated + segment_duration {
                // We're in this segment
                let local_t = if segment_duration > 0.0 {
                    (t - accumulated) / segment_duration
                } else {
                    0.0
                };
                return Some((&segment.fade, local_t.clamp(0.0, 1.0)));
            }
            accumulated += segment_duration;
        }

        // At or past end - use last segment at t=1.0
        self.segments.last().map(|s| (&s.fade, 1.0))
    }
}

impl Default for FadeChain {
    fn default() -> Self {
        Self {
            segments: vec![FadeSegment {
                fade: FadeSpec::from_black(),
                weight: 1.0,
            }],
        }
    }
}

impl StyleInterpolator for FadeChain {
    fn calculate(&self, t: f64, base: Style) -> Style {
        let t = t.clamp(0.0, 1.0);

        match self.find_segment(t) {
            Some((fade, local_t)) => fade.calculate(local_t, base),
            None => base,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fade_spec_from_black() {
        let fade = FadeSpec::from_black();
        let base = Style::fg(Color::rgb(255, 0, 0));

        // At t=0, should be black
        let result = fade.calculate(0.0, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 0));

        // At t=1, should be base (red)
        let result = fade.calculate(1.0, base);
        assert_eq!(result.fg, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_fade_spec_to_black() {
        let fade = FadeSpec::to_black();
        let base = Style::fg(Color::rgb(255, 0, 0));

        // At t=0, should be base (red)
        let result = fade.calculate(0.0, base);
        assert_eq!(result.fg, Color::rgb(255, 0, 0));

        // At t=1, should be black
        let result = fade.calculate(1.0, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_fade_spec_color_to_color() {
        let fade = FadeSpec::color_to_color(
            ColorConfig::Rgb { r: 255, g: 0, b: 0 },
            ColorConfig::Rgb { r: 0, g: 0, b: 255 },
        );
        let base = Style::fg(Color::rgb(128, 128, 128));

        // At t=0, should be red (ignoring base)
        let result = fade.calculate(0.0, base);
        assert_eq!(result.fg, Color::rgb(255, 0, 0));

        // At t=1, should be blue
        let result = fade.calculate(1.0, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 255));

        // At t=0.5, should be purple-ish
        let result = fade.calculate(0.5, base);
        assert!(result.fg.r > 100 && result.fg.r < 150);
        assert!(result.fg.b > 100 && result.fg.b < 150);
    }

    #[test]
    fn test_fade_chain_two_step() {
        let chain = FadeChain::two_step(FadeSpec::from_black(), FadeSpec::to_black());
        let base = Style::fg(Color::rgb(255, 255, 255));

        // At t=0, first fade at 0 = black
        let result = chain.calculate(0.0, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 0));

        // At t=0.5, first fade at 1 = base (white)
        let result = chain.calculate(0.5, base);
        assert_eq!(result.fg, Color::rgb(255, 255, 255));

        // At t=1.0, second fade at 1 = black
        let result = chain.calculate(1.0, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_fade_chain_fire() {
        let chain = FadeChain::fire();
        let base = Style::fg(Color::rgb(128, 128, 128));

        // At t=0, should start white
        let result = chain.calculate(0.0, base);
        assert_eq!(result.fg, Color::rgb(255, 255, 255));

        // At t=1, should end black
        let result = chain.calculate(1.0, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_apply_to_foreground_only() {
        let fade = FadeSpec::to_black().with_apply_to(FadeApplyTo::Foreground);
        let base = Style::fg(Color::rgb(255, 0, 0)).with_bg(Color::rgb(0, 255, 0));

        let result = fade.calculate(1.0, base);

        // Foreground should be black
        assert_eq!(result.fg, Color::rgb(0, 0, 0));
        // Background should be unchanged
        assert_eq!(result.bg, Color::rgb(0, 255, 0));
    }

    #[test]
    fn test_apply_to_background_only() {
        let fade = FadeSpec::to_black().with_apply_to(FadeApplyTo::Background);
        let base = Style::fg(Color::rgb(255, 0, 0)).with_bg(Color::rgb(0, 255, 0));

        let result = fade.calculate(1.0, base);

        // Foreground should be unchanged
        assert_eq!(result.fg, Color::rgb(255, 0, 0));
        // Background should be black
        assert_eq!(result.bg, Color::rgb(0, 0, 0));
    }
}

// <FILE>tui-vfx-style/src/models/cls_fade_spec.rs</FILE> - <DESC>General color-to-color fade with chaining</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
