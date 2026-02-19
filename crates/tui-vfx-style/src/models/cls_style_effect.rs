// <FILE>tui-vfx-style/src/models/cls_style_effect.rs</FILE> - <DESC>StyleEffect enum with documentation methods</DESC>
// <VERS>VERSION: 0.15.0</VERS>
// <WCTX>Phase 1 rustdoc enrichment for documentation pipeline</WCTX>
// <CLOG>Add comprehensive module and enum documentation for rustdoc generation</CLOG>

//! # Style Effects
//!
//! Style effects modify the visual appearance of text and colors over time.
//! They operate on [`Style`] values (foreground, background, modifiers) rather
//! than content position or structure.
//!
//! ## Effect Categories
//!
//! | Category | Effects | Description |
//! |----------|---------|-------------|
//! | **Fade** | [`FadeIn`], [`FadeOut`], [`ColorFade`] | Opacity and color transitions |
//! | **Color Cycling** | [`Rainbow`], [`ColorShift`], [`Pulse`] | Continuous color animation |
//! | **Glitch/Noise** | [`Glitch`], [`NeonFlicker`] | Random visual disruption |
//! | **Modifier** | [`ItalicWindow`], [`RigidShakeStyle`] | Text modifier toggling |
//! | **Spatial** | [`Spatial`] | Position-dependent shading |
//!
//! [`FadeIn`]: StyleEffect::FadeIn
//! [`FadeOut`]: StyleEffect::FadeOut
//! [`ColorFade`]: StyleEffect::ColorFade
//! [`Rainbow`]: StyleEffect::Rainbow
//! [`ColorShift`]: StyleEffect::ColorShift
//! [`Pulse`]: StyleEffect::Pulse
//! [`Glitch`]: StyleEffect::Glitch
//! [`NeonFlicker`]: StyleEffect::NeonFlicker
//! [`ItalicWindow`]: StyleEffect::ItalicWindow
//! [`RigidShakeStyle`]: StyleEffect::RigidShakeStyle
//! [`Spatial`]: StyleEffect::Spatial
//!
//! ## Time Parameter
//!
//! All effects receive a normalized time `t` (0.0 to 1.0) that drives the
//! animation. How this maps to real time depends on the compositor's timing
//! configuration.
//!
//! ## Combining with Filters
//!
//! Style effects are often paired with content filters for coordinated
//! visual effects. For example, [`RigidShakeStyle`] should match the timing
//! parameters of a `RigidShake` filter to keep text styling synchronized
//! with positional movement.

use crate::models::{ColorConfig, ColorSpace, FadeApplyTo, FadeSpec, SpatialShaderType};
use crate::traits::{StyleInterpolator, StyleShader};
use crate::utils::{
    blend_style_to_color, blend_style_to_color_in_space, darken, rainbow_style, shift_style_hsl,
};
use serde::{Deserialize, Serialize};
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_types::{Color, RigidShakeTiming, Style};
/// Style effects that modify text appearance over time.
///
/// Each effect implements [`StyleInterpolator`] to compute the style at any
/// point in the animation timeline. Effects operate on a base [`Style`] and
/// return a modified style.
///
/// # Categories
///
/// - **Fade Effects**: Transition opacity/colors (FadeIn, FadeOut, ColorFade)
/// - **Color Effects**: Cycle through colors (Rainbow, ColorShift, Pulse)
/// - **Glitch Effects**: Random visual disruption (Glitch, NeonFlicker)
/// - **Modifier Effects**: Toggle text modifiers (ItalicWindow, RigidShakeStyle)
/// - **Spatial Effects**: Position-dependent shading (Spatial)
///
/// # JSON Configuration
///
/// ```json
/// { "type": "fade_in", "apply_to": "both", "ease": "ease_in_out" }
/// { "type": "rainbow", "speed": 1.5 }
/// { "type": "glitch", "seed": 42, "intensity": 0.3 }
/// ```
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema)]
pub enum StyleEffect {
    /// Fade in from black/transparent.
    ///
    /// Transitions the style from invisible to fully visible over the
    /// animation timeline. The `apply_to` field controls whether foreground,
    /// background, or both are affected.
    FadeIn {
        /// Which style components to fade (foreground, background, or both).
        apply_to: FadeApplyTo,
        /// Easing curve for the fade animation.
        ease: EasingCurve,
    },

    /// Fade out to black/transparent.
    ///
    /// Transitions the style from fully visible to invisible. Opposite of
    /// [`FadeIn`](Self::FadeIn).
    FadeOut {
        /// Which style components to fade.
        apply_to: FadeApplyTo,
        /// Easing curve for the fade animation.
        ease: EasingCurve,
    },

    /// Pulsing color intensity.
    ///
    /// Blends between the base style and a target color using a sine wave,
    /// creating a rhythmic pulsing effect. Great for highlighting or
    /// attention-grabbing animations.
    Pulse {
        /// Pulse frequency (cycles per normalized time unit).
        frequency: f32,
        /// Target color to pulse toward.
        #[config(opaque)]
        color: Color,
    },

    /// Rainbow color cycling.
    ///
    /// Continuously cycles through the color spectrum by rotating the hue.
    /// Creates a vibrant, psychedelic effect suitable for celebrations
    /// or attention-grabbing displays.
    Rainbow {
        /// Speed of hue rotation (degrees per normalized time unit).
        speed: f32,
    },

    /// Random glitch distortion.
    ///
    /// Randomly applies text modifiers (bold, italic, underline, reverse)
    /// based on a noise function. Can optionally force italic during a
    /// specific time window to sync with content shifts.
    Glitch {
        /// Seed for deterministic randomness.
        seed: u64,
        /// Probability of applying a modifier (0.0 to 1.0).
        intensity: f32,
        /// Optional: force italic starting at this normalized time.
        italic_start: Option<f32>,
        /// Optional: force italic ending at this normalized time.
        italic_end: Option<f32>,
    },

    /// Flickering neon/fluorescent tube effect.
    ///
    /// Simulates an unstable light source by randomly dimming colors.
    /// Higher stability values produce steadier light with fewer flickers.
    NeonFlicker {
        /// Stability factor (0.0 = very unstable, 1.0 = perfectly stable).
        stability: f32,
    },

    /// Position-dependent spatial shader.
    ///
    /// Applies a shader that varies based on cell position within the
    /// widget. See [`SpatialShaderType`] for available shader options.
    Spatial {
        /// The spatial shader to apply.
        shader: SpatialShaderType,
    },

    /// Italic modifier during a time window.
    ///
    /// Applies the italic text modifier only when the animation time falls
    /// within the specified window. Useful for synchronizing text styling
    /// with other effects.
    ItalicWindow {
        /// Start of the italic window (0.0 to 1.0).
        start: f32,
        /// End of the italic window (0.0 to 1.0).
        end: f32,
    },
    /// HSL color shift animation.
    ///
    /// Shifts hue, saturation, and lightness values over the animation
    /// timeline. Unlike [`Rainbow`](Self::Rainbow) which cycles continuously,
    /// this transitions toward specific target adjustments.
    ///
    /// Useful for color grading effects or transitioning between color schemes.
    ColorShift {
        /// Hue shift in degrees (-180 to 180).
        hue_shift: f32,
        /// Saturation adjustment (-1.0 to 1.0).
        saturation_shift: f32,
        /// Lightness adjustment (-1.0 to 1.0).
        lightness_shift: f32,
    },

    /// Fade toward a target color.
    ///
    /// More flexible than [`FadeOut`](Self::FadeOut) which always fades to
    /// black. Interpolates in the specified color space for smooth transitions.
    ///
    /// # Color Spaces
    ///
    /// - **RGB**: Linear interpolation (may produce muddy midpoints)
    /// - **HSL**: Perceptually smoother for hue transitions
    /// - **Oklch**: Modern perceptually uniform color space
    ColorFade {
        /// Target color to fade toward.
        #[config(opaque)]
        target: Color,
        /// Color space for interpolation (affects transition smoothness).
        color_space: ColorSpace,
    },

    /// Italic modifier synchronized with RigidShake filter.
    ///
    /// Applies italic text styling when the shake animation moves right,
    /// creating a coordinated visual effect. Match timing parameters with
    /// a `RigidShake` filter for perfect synchronization.
    ///
    /// # Synchronization
    ///
    /// ```json
    /// // Filter and style use matching timing
    /// { "filter": { "type": "rigid_shake", "shake_period": 0.29 } }
    /// { "style_effect": { "type": "rigid_shake_style", "shake_period": 0.29 } }
    /// ```
    RigidShakeStyle {
        /// Duration of one back-and-forth shake in seconds.
        shake_period: f32,
        /// Number of shakes before pause (max 8).
        num_shakes: u8,
        /// Duration of pause between shake cycles in seconds.
        pause_duration: f32,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum StyleEffectSerde {
    FadeIn {
        #[serde(default)]
        apply_to: FadeApplyTo,
        #[serde(default)]
        ease: EasingCurve,
    },
    FadeOut {
        #[serde(default)]
        apply_to: FadeApplyTo,
        #[serde(default)]
        ease: EasingCurve,
    },
    Pulse {
        frequency: f32,
        color: ColorConfig,
    },
    Rainbow {
        speed: f32,
    },
    Glitch {
        seed: u64,
        intensity: f32,
        #[serde(default)]
        italic_start: Option<f32>,
        #[serde(default)]
        italic_end: Option<f32>,
    },
    NeonFlicker {
        stability: f32,
    },
    Spatial {
        shader: SpatialShaderType,
    },
    ItalicWindow {
        start: f32,
        end: f32,
    },
    ColorShift {
        hue_shift: f32,
        saturation_shift: f32,
        lightness_shift: f32,
    },
    ColorFade {
        target: ColorConfig,
        color_space: ColorSpace,
    },
    RigidShakeStyle {
        shake_period: f32,
        num_shakes: u8,
        pause_duration: f32,
    },
}
impl From<&StyleEffect> for StyleEffectSerde {
    fn from(value: &StyleEffect) -> Self {
        match value {
            StyleEffect::FadeIn { apply_to, ease } => Self::FadeIn {
                apply_to: *apply_to,
                ease: *ease,
            },
            StyleEffect::FadeOut { apply_to, ease } => Self::FadeOut {
                apply_to: *apply_to,
                ease: *ease,
            },
            StyleEffect::Pulse { frequency, color } => Self::Pulse {
                frequency: *frequency,
                color: ColorConfig::from(*color),
            },
            StyleEffect::Rainbow { speed } => Self::Rainbow { speed: *speed },
            StyleEffect::Glitch {
                seed,
                intensity,
                italic_start,
                italic_end,
            } => Self::Glitch {
                seed: *seed,
                intensity: *intensity,
                italic_start: *italic_start,
                italic_end: *italic_end,
            },
            StyleEffect::NeonFlicker { stability } => Self::NeonFlicker {
                stability: *stability,
            },
            StyleEffect::Spatial { shader } => Self::Spatial {
                shader: shader.clone(),
            },
            StyleEffect::ItalicWindow { start, end } => Self::ItalicWindow {
                start: *start,
                end: *end,
            },
            StyleEffect::ColorShift {
                hue_shift,
                saturation_shift,
                lightness_shift,
            } => Self::ColorShift {
                hue_shift: *hue_shift,
                saturation_shift: *saturation_shift,
                lightness_shift: *lightness_shift,
            },
            StyleEffect::ColorFade {
                target,
                color_space,
            } => Self::ColorFade {
                target: ColorConfig::from(*target),
                color_space: *color_space,
            },
            StyleEffect::RigidShakeStyle {
                shake_period,
                num_shakes,
                pause_duration,
            } => Self::RigidShakeStyle {
                shake_period: *shake_period,
                num_shakes: *num_shakes,
                pause_duration: *pause_duration,
            },
        }
    }
}
impl From<StyleEffectSerde> for StyleEffect {
    fn from(value: StyleEffectSerde) -> Self {
        match value {
            StyleEffectSerde::FadeIn { apply_to, ease } => Self::FadeIn { apply_to, ease },
            StyleEffectSerde::FadeOut { apply_to, ease } => Self::FadeOut { apply_to, ease },
            StyleEffectSerde::Pulse { frequency, color } => Self::Pulse {
                frequency,
                color: Color::from(color),
            },
            StyleEffectSerde::Rainbow { speed } => Self::Rainbow { speed },
            StyleEffectSerde::Glitch {
                seed,
                intensity,
                italic_start,
                italic_end,
            } => Self::Glitch {
                seed,
                intensity,
                italic_start,
                italic_end,
            },
            StyleEffectSerde::NeonFlicker { stability } => Self::NeonFlicker { stability },
            StyleEffectSerde::Spatial { shader } => Self::Spatial { shader },
            StyleEffectSerde::ItalicWindow { start, end } => Self::ItalicWindow { start, end },
            StyleEffectSerde::ColorShift {
                hue_shift,
                saturation_shift,
                lightness_shift,
            } => Self::ColorShift {
                hue_shift,
                saturation_shift,
                lightness_shift,
            },
            StyleEffectSerde::ColorFade {
                target,
                color_space,
            } => Self::ColorFade {
                target: Color::from(target),
                color_space,
            },
            StyleEffectSerde::RigidShakeStyle {
                shake_period,
                num_shakes,
                pause_duration,
            } => Self::RigidShakeStyle {
                shake_period,
                num_shakes,
                pause_duration,
            },
        }
    }
}
impl Serialize for StyleEffect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        StyleEffectSerde::from(self).serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for StyleEffect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = StyleEffectSerde::deserialize(deserializer)?;
        Ok(Self::from(value))
    }
}
impl PartialEq for StyleEffect {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::FadeIn {
                    apply_to: a1,
                    ease: e1,
                },
                Self::FadeIn {
                    apply_to: a2,
                    ease: e2,
                },
            ) => a1 == a2 && e1 == e2,
            (
                Self::FadeOut {
                    apply_to: a1,
                    ease: e1,
                },
                Self::FadeOut {
                    apply_to: a2,
                    ease: e2,
                },
            ) => a1 == a2 && e1 == e2,
            (
                Self::Pulse {
                    frequency: f1,
                    color: c1,
                },
                Self::Pulse {
                    frequency: f2,
                    color: c2,
                },
            ) => f1 == f2 && c1 == c2,
            (Self::Rainbow { speed: s1 }, Self::Rainbow { speed: s2 }) => s1 == s2,
            (
                Self::Glitch {
                    seed: s1,
                    intensity: i1,
                    italic_start: is1,
                    italic_end: ie1,
                },
                Self::Glitch {
                    seed: s2,
                    intensity: i2,
                    italic_start: is2,
                    italic_end: ie2,
                },
            ) => s1 == s2 && i1 == i2 && is1 == is2 && ie1 == ie2,
            (Self::NeonFlicker { stability: s1 }, Self::NeonFlicker { stability: s2 }) => s1 == s2,
            (Self::Spatial { shader: sh1 }, Self::Spatial { shader: sh2 }) => sh1 == sh2,
            (
                Self::ItalicWindow { start: s1, end: e1 },
                Self::ItalicWindow { start: s2, end: e2 },
            ) => s1 == s2 && e1 == e2,
            (
                Self::ColorShift {
                    hue_shift: h1,
                    saturation_shift: s1,
                    lightness_shift: l1,
                },
                Self::ColorShift {
                    hue_shift: h2,
                    saturation_shift: s2,
                    lightness_shift: l2,
                },
            ) => h1 == h2 && s1 == s2 && l1 == l2,
            (
                Self::ColorFade {
                    target: t1,
                    color_space: cs1,
                },
                Self::ColorFade {
                    target: t2,
                    color_space: cs2,
                },
            ) => t1 == t2 && cs1 == cs2,
            (
                Self::RigidShakeStyle {
                    shake_period: p1,
                    num_shakes: n1,
                    pause_duration: d1,
                },
                Self::RigidShakeStyle {
                    shake_period: p2,
                    num_shakes: n2,
                    pause_duration: d2,
                },
            ) => p1 == p2 && n1 == n2 && d1 == d2,
            _ => false,
        }
    }
}
impl StyleInterpolator for StyleEffect {
    fn calculate(&self, t: f64, base: Style) -> Style {
        let t32 = t as f32;
        match self {
            StyleEffect::FadeIn { apply_to, ease } => FadeSpec::from_black()
                .with_apply_to(*apply_to)
                .with_ease(*ease)
                .calculate(t, base),
            StyleEffect::FadeOut { apply_to, ease } => FadeSpec::to_black()
                .with_apply_to(*apply_to)
                .with_ease(*ease)
                .calculate(t, base),
            StyleEffect::Pulse { frequency, color } => {
                let wave = (t32 * frequency * std::f32::consts::TAU).sin();
                let blend_factor = (wave + 1.0) / 2.0;
                blend_style_to_color(base, *color, blend_factor)
            }
            StyleEffect::Rainbow { speed } => {
                // Rainbow cycles through hues over time
                let hue = (t32 * speed * 360.0).rem_euclid(360.0);
                rainbow_style(base, hue)
            }
            StyleEffect::Glitch {
                seed,
                intensity,
                italic_start,
                italic_end,
            } => {
                // Check if we're in the italic window (deterministic italic)
                let in_italic_window = match (italic_start, italic_end) {
                    (Some(start), Some(end)) => t32 >= *start && t32 < *end,
                    _ => false,
                };

                if in_italic_window {
                    // Force italic during the window (synced with content shift)
                    base.italic()
                } else {
                    // Random glitch effect - 4 modifiers including ITALIC (matches forward branch)
                    let input = *seed as f32 + t32 * 1000.0;
                    let noise = pseudo_random(input as u32);
                    if noise < *intensity {
                        let mod_noise = pseudo_random((input as u32).wrapping_add(7919));
                        let mod_choice = (mod_noise * 4.0) as u32; // 4 choices like forward branch
                        match mod_choice {
                            0 => base.bold(),
                            1 => base.underline(),
                            2 => base.italic(),
                            _ => {
                                let mut new_mods = base.mods;
                                new_mods.reverse = true;
                                base.with_mods(new_mods)
                            }
                        }
                    } else {
                        base
                    }
                }
            }
            StyleEffect::NeonFlicker { stability } => {
                // High frequency noise based on time
                let noise = pseudo_random((t32 * 5000.0) as u32);
                // If noise exceeds stability, we dim the light (simulate power drop)
                if noise > *stability {
                    let dim_amount = (noise - stability) / (1.0 - stability);
                    let mut result = base;
                    if base.fg != Color::TRANSPARENT {
                        result.fg = darken(base.fg, dim_amount * 0.8);
                    }
                    if base.bg != Color::TRANSPARENT {
                        result.bg = darken(base.bg, dim_amount * 0.8);
                    }
                    result
                } else {
                    base
                }
            }
            StyleEffect::Spatial { .. } => base,
            StyleEffect::ItalicWindow { start, end } => {
                if t32 >= *start && t32 < *end {
                    base.italic()
                } else {
                    base
                }
            }
            StyleEffect::ColorShift {
                hue_shift,
                saturation_shift,
                lightness_shift,
            } => {
                // Apply HSL shifts scaled by progress
                let current_hue = hue_shift * t32;
                let current_sat = saturation_shift * t32;
                let current_light = lightness_shift * t32;
                shift_style_hsl(base, current_hue, current_sat, current_light)
            }
            StyleEffect::ColorFade {
                target,
                color_space,
            } => {
                // Fade from base to target over time
                blend_style_to_color_in_space(base, *target, t32, *color_space)
            }
            StyleEffect::RigidShakeStyle {
                shake_period,
                num_shakes,
                pause_duration,
            } => {
                // Reconstruct timing to compute shake phase
                let timing = RigidShakeTiming::new()
                    .with_shake_period(*shake_period)
                    .with_num_shakes(*num_shakes)
                    .with_pause_duration(*pause_duration);
                let state = timing.calculate(t);

                // Apply italic when shifting right (syncs with margin animation)
                if state.is_shifting_right() {
                    base.italic()
                } else {
                    base
                }
            }
        }
    }
}
impl StyleEffect {
    pub fn shader(&self) -> Option<&dyn StyleShader> {
        match self {
            StyleEffect::Spatial { shader } => Some(shader),
            _ => None,
        }
    }

    /// Returns the effect type name.
    pub fn effect_type_name(&self) -> &'static str {
        match self {
            StyleEffect::FadeIn { .. } => "FadeIn",
            StyleEffect::FadeOut { .. } => "FadeOut",
            StyleEffect::Pulse { .. } => "Pulse",
            StyleEffect::Rainbow { .. } => "Rainbow",
            StyleEffect::Glitch { .. } => "Glitch",
            StyleEffect::NeonFlicker { .. } => "NeonFlicker",
            StyleEffect::Spatial { .. } => "Spatial",
            StyleEffect::ItalicWindow { .. } => "ItalicWindow",
            StyleEffect::ColorShift { .. } => "ColorShift",
            StyleEffect::ColorFade { .. } => "ColorFade",
            StyleEffect::RigidShakeStyle { .. } => "RigidShakeStyle",
        }
    }

    /// Returns a brief description of this effect.
    pub fn terse_description(&self) -> &'static str {
        match self {
            StyleEffect::FadeIn { .. } => "Fades in from transparent/black",
            StyleEffect::FadeOut { .. } => "Fades out to transparent/black",
            StyleEffect::Pulse { .. } => "Pulsing color intensity effect",
            StyleEffect::Rainbow { .. } => "Rainbow color cycling effect",
            StyleEffect::Glitch { .. } => "Random glitch distortion with modifiers",
            StyleEffect::NeonFlicker { .. } => "Neon sign flicker effect (temporal)",
            StyleEffect::Spatial { shader } => shader.terse_description(),
            StyleEffect::ItalicWindow { .. } => "Italic modifier during time window",
            StyleEffect::ColorShift { .. } => "HSL color shift animation",
            StyleEffect::ColorFade { .. } => "Color fade toward target",
            StyleEffect::RigidShakeStyle { .. } => "Italic synced with RigidShake filter",
        }
    }

    /// Returns key parameters of this effect for documentation purposes.
    pub fn key_parameters(&self) -> Vec<(&'static str, String)> {
        match self {
            StyleEffect::FadeIn { apply_to, ease } => vec![
                ("apply_to", format!("{:?}", apply_to)),
                ("ease", format!("{:?}", ease)),
            ],
            StyleEffect::FadeOut { apply_to, ease } => vec![
                ("apply_to", format!("{:?}", apply_to)),
                ("ease", format!("{:?}", ease)),
            ],
            StyleEffect::Pulse { frequency, .. } => vec![("frequency", format!("{}", frequency))],
            StyleEffect::Rainbow { speed } => vec![("speed", format!("{}", speed))],
            StyleEffect::Glitch {
                seed, intensity, ..
            } => vec![
                ("seed", format!("{}", seed)),
                ("intensity", format!("{}", intensity)),
            ],
            StyleEffect::NeonFlicker { stability } => vec![("stability", format!("{}", stability))],
            StyleEffect::Spatial { shader } => shader.key_parameters(),
            StyleEffect::ItalicWindow { start, end } => {
                vec![("start", format!("{}", start)), ("end", format!("{}", end))]
            }
            StyleEffect::ColorShift {
                hue_shift,
                saturation_shift,
                lightness_shift,
            } => vec![
                ("hue_shift", format!("{}deg", hue_shift)),
                ("saturation_shift", format!("{}", saturation_shift)),
                ("lightness_shift", format!("{}", lightness_shift)),
            ],
            StyleEffect::ColorFade { color_space, .. } => {
                vec![("color_space", format!("{:?}", color_space))]
            }
            StyleEffect::RigidShakeStyle {
                shake_period,
                num_shakes,
                pause_duration,
            } => vec![
                ("shake_period", format!("{}s", shake_period)),
                ("num_shakes", format!("{}", num_shakes)),
                ("pause_duration", format!("{}s", pause_duration)),
            ],
        }
    }
}
/// Deterministic pseudo-random number generator.
/// Returns a value in 0.0..1.0 based on the input seed.
/// ~25x faster than ChaCha8-based Rng.
#[inline]
fn pseudo_random(input: u32) -> f32 {
    use mixed_signals::math::fast_random;
    fast_random(input as u64, 0)
}

// <FILE>tui-vfx-style/src/models/cls_style_effect.rs</FILE> - <DESC>StyleEffect enum with documentation methods</DESC>
// <VERS>END OF VERSION: 0.15.0</VERS>
