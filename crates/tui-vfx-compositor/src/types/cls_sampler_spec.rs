// <FILE>tui-vfx-compositor/src/types/cls_sampler_spec.rs</FILE> - <DESC>SamplerSpec enum with signal-driven parameters</DESC>
// <VERS>VERSION: 2.5.1</VERS>
// <WCTX>Rustfmt normalization for sampler spec docs</WCTX>
// <CLOG>Apply formatting updates after clippy run</CLOG>

//! # Sampler Specifications
//!
//! Samplers distort pixel coordinates before rendering, creating spatial
//! transformation effects. They are applied first in the composition pipeline,
//! before masks, shaders, and filters.
//!
//! ## Available Samplers
//!
//! | Sampler | Description | Best For |
//! |---------|-------------|----------|
//! | [`SamplerSpec::SineWave`] | Sinusoidal wave distortion | Underwater, dream effects |
//! | [`SamplerSpec::Ripple`] | Circular ripple from center | Water, impact effects |
//! | [`SamplerSpec::Shredder`] | Paper shredder strips | Destruction, glitch |
//! | [`SamplerSpec::FaultLine`] | Fault line displacement | Earthquake, error |
//! | [`SamplerSpec::Crt`] | CRT scanlines + curvature | Retro monitor |
//! | [`SamplerSpec::CrtJitter`] | CRT crash/jitter | Failing electronics |
//! | [`SamplerSpec::Bounce`] | Bouncing vertical displacement | Loaders, attention |
//! | [`SamplerSpec::Pendulum`] | Bidirectional swaying motion | Menus, hanging items |
//!
//! ## Signal-Driven Parameters
//!
//! All samplers use [`SignalOrFloat`] for their parameters, enabling
//! both static values and animation-driven effects:
//!
//! ```json
//! { "type": "sine_wave", "amplitude": 2.0 }              // Static
//! { "type": "sine_wave", "amplitude": { "signal": "t" }} // Animated
//! ```
//!
//! ## Performance Considerations
//!
//! Samplers that distort coordinates cause content to be re-sampled,
//! which can affect visual quality. For best results:
//!
//! - Use moderate amplitude values (1-3 cells)
//! - Avoid combining multiple coordinate-distorting effects

use mixed_signals::types::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// Axis for directional effects.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Axis {
    #[default]
    X,
    Y,
}

/// Center point for ripple effects.
/// Accepts either a string "Center" or an object { "x": u16, "y": u16 }
#[derive(Debug, Clone, Copy, PartialEq, tui_vfx_core::ConfigSchema, Default)]
pub enum RippleCenter {
    /// Ripple from the center of the widget
    #[default]
    Center,
    /// Ripple from a specific point
    Point { x: u16, y: u16 },
}

impl Serialize for RippleCenter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RippleCenter::Center => serializer.serialize_str("center"),
            RippleCenter::Point { x, y } => {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct("Point", 2)?;
                s.serialize_field("x", x)?;
                s.serialize_field("y", y)?;
                s.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for RippleCenter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct RippleCenterVisitor;

        impl<'de> Visitor<'de> for RippleCenterVisitor {
            type Value = RippleCenter;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("\"center\" or { \"x\": u16, \"y\": u16 }")
            }

            fn visit_str<E>(self, value: &str) -> Result<RippleCenter, E>
            where
                E: de::Error,
            {
                if value == "center" || value == "Center" {
                    Ok(RippleCenter::Center)
                } else {
                    Err(de::Error::unknown_variant(value, &["center"]))
                }
            }

            fn visit_map<M>(self, mut map: M) -> Result<RippleCenter, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
                Ok(RippleCenter::Point { x, y })
            }
        }

        deserializer.deserialize_any(RippleCenterVisitor)
    }
}

/// Complete sampler specification with all parameters.
///
/// Samplers distort pixel coordinates before rendering, creating spatial
/// transformation effects. They are the first stage in the composition
/// pipeline, applied before masks, shaders, and filters.
///
/// # Categories
///
/// - **Wave Effects**: SineWave, Ripple
/// - **Destruction Effects**: Shredder, FaultLine
/// - **Retro Effects**: Crt, CrtJitter
///
/// # Signal-Driven Parameters
///
/// All parameters use `SignalOrFloat` for static or animated values:
/// ```json
/// { "type": "ripple", "amplitude": 2.0 }                 // Static
/// { "type": "ripple", "amplitude": { "signal": "t" } }   // Animated
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[derive(Default)]
pub enum SamplerSpec {
    /// No coordinate transformation.
    #[default]
    None,

    /// Sinusoidal wave distortion.
    ///
    /// Creates a wavy, undulating effect by applying sine-wave displacement
    /// along one axis. Great for underwater or dream-like effects.
    ///
    /// # Parameters
    ///
    /// - `axis`: Which axis the wave affects (X or Y)
    /// - `amplitude`: Wave height in cells (1-3 recommended)
    /// - `frequency`: Number of wave cycles across the widget
    /// - `speed`: Animation speed (higher = faster wave motion)
    /// - `phase`: Phase offset in radians for timing control
    SineWave {
        /// Which axis the wave affects.
        axis: Axis,

        /// Amplitude of the wave in cells.
        ///
        /// Higher values create more dramatic distortion.
        /// Recommended: 1-3 for subtle, 4+ for dramatic.
        #[serde(default = "default_sine_amplitude")]
        amplitude: SignalOrFloat,

        /// Number of wave cycles across the widget.
        #[serde(default = "default_sine_frequency")]
        frequency: SignalOrFloat,

        /// Speed of wave animation.
        #[serde(default = "default_sine_speed")]
        speed: SignalOrFloat,

        /// Phase offset in radians.
        #[serde(default = "default_sine_phase")]
        phase: SignalOrFloat,
    },

    /// Circular ripple distortion from a center point.
    ///
    /// Creates expanding circular waves like dropping a stone in water.
    /// Great for impact effects or attention-grabbing animations.
    ///
    /// # Parameters
    ///
    /// - `center`: Origin point for the ripples
    /// - `amplitude`: Wave height in cells
    /// - `wavelength`: Distance between ripple peaks
    /// - `speed`: Ripple propagation speed
    Ripple {
        /// Amplitude of the ripple in cells.
        #[serde(default = "default_ripple_amplitude")]
        amplitude: SignalOrFloat,

        /// Distance between ripple peaks in cells.
        #[serde(default = "default_ripple_wavelength")]
        wavelength: SignalOrFloat,

        /// Speed of ripple propagation outward.
        #[serde(default = "default_ripple_speed")]
        speed: SignalOrFloat,

        /// Center point of the ripple.
        ///
        /// Either `"center"` or `{ "x": N, "y": M }`.
        center: RippleCenter,
    },

    /// Paper shredder effect with alternating strips.
    ///
    /// Divides content into vertical strips that slide past each other
    /// at different speeds, like paper going through a shredder.
    /// Great for destruction or transition effects.
    ///
    /// # Parameters
    ///
    /// - `stripe_width`: Width of each strip in cells
    /// - `odd_speed` / `even_speed`: Speeds for alternating strips
    Shredder {
        /// Width of each shredder stripe in cells.
        stripe_width: u16,

        /// Speed of odd-indexed stripes.
        #[serde(default = "default_shredder_odd_speed")]
        odd_speed: SignalOrFloat,

        /// Speed of even-indexed stripes.
        #[serde(default = "default_shredder_even_speed")]
        even_speed: SignalOrFloat,
    },

    /// Fault line displacement effect.
    ///
    /// Creates horizontal fault lines that shift content left/right,
    /// simulating an earthquake or glitch effect.
    ///
    /// # Parameters
    ///
    /// - `seed`: Random seed for reproducible patterns
    /// - `intensity`: Strength of displacement
    /// - `split_bias`: Bias toward upper (negative) or lower (positive) splits
    FaultLine {
        /// Seed for deterministic randomness.
        seed: u64,

        /// Intensity of the fault displacement.
        #[serde(default = "default_faultline_intensity")]
        intensity: SignalOrFloat,

        /// Bias toward upper (-1.0) or lower (+1.0) splits.
        split_bias: f32,
    },

    /// CRT monitor effect with scanlines and curvature.
    ///
    /// Simulates a vintage CRT monitor with scanlines, screen curvature,
    /// and horizontal jitter. Great for retro gaming aesthetics.
    ///
    /// # Parameters
    ///
    /// - `scanline_strength`: Visibility of horizontal scanlines
    /// - `curvature`: Amount of barrel distortion at edges
    /// - `jitter`: Horizontal position jitter
    Crt {
        /// Strength of scanline effect (0.0 = none, 1.0 = maximum).
        #[serde(default = "default_crt_scanline_strength")]
        scanline_strength: SignalOrFloat,

        /// Amount of horizontal jitter.
        #[serde(default = "default_crt_jitter")]
        jitter: SignalOrFloat,

        /// Screen curvature (barrel distortion) amount.
        #[serde(default = "default_crt_curvature")]
        curvature: SignalOrFloat,
    },

    /// CRT crash/jitter effect.
    ///
    /// Simulates a failing CRT with erratic jittering that decays over time.
    /// Great for error states or dramatic glitch effects.
    ///
    /// # Parameters
    ///
    /// - `intensity`: Strength of the jitter effect
    /// - `speed_hz`: Jitter frequency in Hz
    /// - `decay_ms`: Time for effect to decay (in milliseconds)
    CrtJitter {
        /// Intensity of the jitter effect.
        #[serde(default = "default_crtjitter_intensity")]
        intensity: SignalOrFloat,

        /// Jitter frequency in Hz.
        #[serde(default = "default_crtjitter_speed_hz")]
        speed_hz: SignalOrFloat,

        /// Decay time in milliseconds.
        ///
        /// The jitter effect fades out over this duration.
        decay_ms: u64,
    },

    /// Bouncing vertical displacement effect.
    ///
    /// Creates a bouncing ball effect where elements rise and fall with
    /// natural physics-like motion. Elements at different X positions
    /// bounce with phase offsets, creating a wave of bouncing.
    /// Great for loader animations and attention-grabbing effects.
    ///
    /// # Parameters
    ///
    /// - `amplitude`: Bounce height in cells (how high elements rise)
    /// - `speed`: Animation speed (bounces per second)
    /// - `phase_spread`: Phase offset between adjacent columns
    Bounce {
        /// Bounce height in cells.
        ///
        /// Higher values create more dramatic bouncing.
        /// Recommended: 1-3 for loaders.
        #[serde(default = "default_bounce_amplitude")]
        amplitude: SignalOrFloat,

        /// Animation speed multiplier.
        ///
        /// Controls how fast the bounce cycles.
        /// 4.0 = approximately 4 bounces per second.
        #[serde(default = "default_bounce_speed")]
        speed: SignalOrFloat,

        /// Phase offset per column.
        ///
        /// Creates a wave effect where adjacent columns bounce
        /// at slightly different times. 0.5 recommended for loaders.
        #[serde(default = "default_bounce_phase_spread")]
        phase_spread: SignalOrFloat,
    },

    /// Pendulum/swaying displacement effect.
    ///
    /// Creates a continuous swaying effect where elements oscillate back
    /// and forth. Unlike Bounce (which uses abs(sin) for always-positive
    /// displacement), Pendulum uses sin() for true bidirectional motion.
    /// Great for swaying menus, hanging items, or wind-through-grass effects.
    ///
    /// # Parameters
    ///
    /// - `axis`: Which axis the pendulum swings along (X or Y)
    /// - `amplitude`: Swing distance in cells (maximum displacement from center)
    /// - `speed`: Animation speed (affects swing frequency)
    /// - `phase_spread`: Phase offset per position (creates staggered wave effect)
    Pendulum {
        /// Which axis the pendulum swings along.
        ///
        /// X = horizontal swing (phase based on Y position)
        /// Y = vertical swing (phase based on X position)
        #[serde(default)]
        axis: Axis,

        /// Swing amplitude in cells.
        ///
        /// Maximum displacement from center position.
        /// Recommended: 2.0 for gentle sway, 4+ for dramatic.
        #[serde(default = "default_pendulum_amplitude")]
        amplitude: SignalOrFloat,

        /// Animation speed multiplier.
        ///
        /// Controls how fast the pendulum swings.
        /// 2.0 = approximately 2 swings per second.
        #[serde(default = "default_pendulum_speed")]
        speed: SignalOrFloat,

        /// Phase offset per position.
        ///
        /// Creates a wave effect where adjacent rows/columns swing
        /// at slightly different times. 0.3 recommended for gentle wave.
        #[serde(default = "default_pendulum_phase_spread")]
        phase_spread: SignalOrFloat,
    },
}

// Default functions for signal-or-float fields
fn default_sine_amplitude() -> SignalOrFloat {
    SignalOrFloat::Static(1.0)
}

fn default_sine_frequency() -> SignalOrFloat {
    SignalOrFloat::Static(1.0)
}

fn default_sine_speed() -> SignalOrFloat {
    SignalOrFloat::Static(1.0)
}

fn default_sine_phase() -> SignalOrFloat {
    SignalOrFloat::Static(0.0)
}

fn default_ripple_amplitude() -> SignalOrFloat {
    SignalOrFloat::Static(1.0)
}

fn default_ripple_wavelength() -> SignalOrFloat {
    SignalOrFloat::Static(4.0)
}

fn default_ripple_speed() -> SignalOrFloat {
    SignalOrFloat::Static(1.0)
}

fn default_shredder_odd_speed() -> SignalOrFloat {
    SignalOrFloat::Static(2.0)
}

fn default_shredder_even_speed() -> SignalOrFloat {
    SignalOrFloat::Static(2.0)
}

fn default_faultline_intensity() -> SignalOrFloat {
    SignalOrFloat::Static(1.0)
}

fn default_crt_scanline_strength() -> SignalOrFloat {
    SignalOrFloat::Static(0.8)
}

fn default_crt_jitter() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_crt_curvature() -> SignalOrFloat {
    SignalOrFloat::Static(0.1)
}

fn default_crtjitter_intensity() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_crtjitter_speed_hz() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_bounce_amplitude() -> SignalOrFloat {
    SignalOrFloat::Static(2.0)
}

fn default_bounce_speed() -> SignalOrFloat {
    SignalOrFloat::Static(4.0)
}

fn default_bounce_phase_spread() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_pendulum_amplitude() -> SignalOrFloat {
    SignalOrFloat::Static(2.0)
}

fn default_pendulum_speed() -> SignalOrFloat {
    SignalOrFloat::Static(2.0)
}

fn default_pendulum_phase_spread() -> SignalOrFloat {
    SignalOrFloat::Static(0.3)
}

impl SamplerSpec {
    /// Returns the sampler type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            SamplerSpec::None => "None",
            SamplerSpec::SineWave { .. } => "SineWave",
            SamplerSpec::Ripple { .. } => "Ripple",
            SamplerSpec::Shredder { .. } => "Shredder",
            SamplerSpec::FaultLine { .. } => "FaultLine",
            SamplerSpec::Crt { .. } => "Crt",
            SamplerSpec::CrtJitter { .. } => "CrtJitter",
            SamplerSpec::Bounce { .. } => "Bounce",
            SamplerSpec::Pendulum { .. } => "Pendulum",
        }
    }

    /// Returns a brief human-readable description of what this sampler does.
    pub fn terse_description(&self) -> &'static str {
        match self {
            SamplerSpec::None => "No coordinate transformation",
            SamplerSpec::SineWave { .. } => "Sinusoidal wave distortion",
            SamplerSpec::Ripple { .. } => "Circular ripple distortion from center point",
            SamplerSpec::Shredder { .. } => "Paper shredder effect with alternating strips",
            SamplerSpec::FaultLine { .. } => "Fault line displacement effect",
            SamplerSpec::Crt { .. } => "CRT monitor effect with scanlines and curvature",
            SamplerSpec::CrtJitter { .. } => "CRT crash/jitter effect with decay",
            SamplerSpec::Bounce { .. } => "Bouncing vertical displacement for loaders",
            SamplerSpec::Pendulum { .. } => {
                "Bidirectional swaying motion for menus and hanging items"
            }
        }
    }

    /// Returns key parameters of this sampler for documentation purposes.
    pub fn key_parameters(&self) -> Vec<(&'static str, String)> {
        match self {
            SamplerSpec::None => vec![],
            SamplerSpec::SineWave {
                axis,
                amplitude,
                frequency,
                speed,
                phase,
            } => vec![
                ("axis", format!("{:?}", axis)),
                ("amplitude", format!("{:?}", amplitude)),
                ("frequency", format!("{:?}", frequency)),
                ("speed", format!("{:?}", speed)),
                ("phase", format!("{:?}", phase)),
            ],
            SamplerSpec::Ripple {
                amplitude,
                wavelength,
                speed,
                center,
            } => vec![
                ("amplitude", format!("{:?}", amplitude)),
                ("wavelength", format!("{:?}", wavelength)),
                ("speed", format!("{:?}", speed)),
                ("center", format!("{:?}", center)),
            ],
            SamplerSpec::Shredder {
                stripe_width,
                odd_speed,
                even_speed,
            } => vec![
                ("stripe_width", format!("{}", stripe_width)),
                ("odd_speed", format!("{:?}", odd_speed)),
                ("even_speed", format!("{:?}", even_speed)),
            ],
            SamplerSpec::FaultLine {
                seed,
                intensity,
                split_bias,
            } => vec![
                ("seed", format!("{}", seed)),
                ("intensity", format!("{:?}", intensity)),
                ("split_bias", format!("{}", split_bias)),
            ],
            SamplerSpec::Crt {
                scanline_strength,
                jitter,
                curvature,
            } => vec![
                ("scanline_strength", format!("{:?}", scanline_strength)),
                ("jitter", format!("{:?}", jitter)),
                ("curvature", format!("{:?}", curvature)),
            ],
            SamplerSpec::CrtJitter {
                intensity,
                speed_hz,
                decay_ms,
            } => vec![
                ("intensity", format!("{:?}", intensity)),
                ("speed_hz", format!("{:?}", speed_hz)),
                ("decay_ms", format!("{}ms", decay_ms)),
            ],
            SamplerSpec::Bounce {
                amplitude,
                speed,
                phase_spread,
            } => vec![
                ("amplitude", format!("{:?}", amplitude)),
                ("speed", format!("{:?}", speed)),
                ("phase_spread", format!("{:?}", phase_spread)),
            ],
            SamplerSpec::Pendulum {
                axis,
                amplitude,
                speed,
                phase_spread,
            } => vec![
                ("axis", format!("{:?}", axis)),
                ("amplitude", format!("{:?}", amplitude)),
                ("speed", format!("{:?}", speed)),
                ("phase_spread", format!("{:?}", phase_spread)),
            ],
        }
    }
}

// <FILE>tui-vfx-compositor/src/types/cls_sampler_spec.rs</FILE> - <DESC>SamplerSpec enum with signal-driven parameters</DESC>
// <VERS>END OF VERSION: 2.5.1</VERS>
