// <FILE>tui-vfx-geometry/src/types/cls_shake.rs</FILE> - <DESC>Random position offset for shake effects</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>RNG performance optimization</WCTX>
// <CLOG>Switched to fast_random for ~25x faster shake offset generation</CLOG>

use serde::{Deserialize, Serialize};

/// Shake effect for adding random position offsets.
///
/// Creates trembling, vibration, or instability effects by applying
/// pseudo-random offsets to positions. Useful for:
/// - Alert/warning notifications
/// - Impact feedback
/// - Nervous/unstable UI elements
/// - Glitch aesthetics
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Shake {
    /// No shake - position unchanged
    #[default]
    None,
    /// Uniform shake in all directions
    Uniform {
        /// Maximum offset in cells (applies to both x and y)
        #[serde(default = "default_intensity")]
        intensity: f32,
        /// Shake frequency - higher = faster vibration
        #[serde(default = "default_frequency")]
        frequency: f32,
    },
    /// Horizontal shake only (side-to-side)
    Horizontal {
        #[serde(default = "default_intensity")]
        intensity: f32,
        #[serde(default = "default_frequency")]
        frequency: f32,
    },
    /// Vertical shake only (up-and-down)
    Vertical {
        #[serde(default = "default_intensity")]
        intensity: f32,
        #[serde(default = "default_frequency")]
        frequency: f32,
    },
    /// Decaying shake that reduces over time (impact style)
    Decay {
        /// Starting intensity
        #[serde(default = "default_decay_intensity")]
        intensity: f32,
        /// Decay rate (higher = faster decay)
        #[serde(default = "default_decay_rate")]
        decay: f32,
        #[serde(default = "default_frequency")]
        frequency: f32,
    },
}

fn default_intensity() -> f32 {
    1.0
}
fn default_frequency() -> f32 {
    10.0
}
fn default_decay_intensity() -> f32 {
    2.0
}
fn default_decay_rate() -> f32 {
    3.0
}

/// Offset result from shake calculation.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ShakeOffset {
    pub dx: f32,
    pub dy: f32,
}

impl Shake {
    /// Calculate the shake offset for a given progress and frame.
    ///
    /// * `progress` - Animation progress (0.0 to 1.0)
    /// * `frame` - Frame number for deterministic randomness
    ///
    /// Returns the (dx, dy) offset to apply to position.
    pub fn offset(&self, progress: f32, frame: u64) -> ShakeOffset {
        match self {
            Shake::None => ShakeOffset::default(),

            Shake::Uniform {
                intensity,
                frequency,
            } => {
                let (rx, ry) = random_offset(*frequency, frame);
                ShakeOffset {
                    dx: rx * intensity,
                    dy: ry * intensity,
                }
            }

            Shake::Horizontal {
                intensity,
                frequency,
            } => {
                let (rx, _) = random_offset(*frequency, frame);
                ShakeOffset {
                    dx: rx * intensity,
                    dy: 0.0,
                }
            }

            Shake::Vertical {
                intensity,
                frequency,
            } => {
                let (_, ry) = random_offset(*frequency, frame);
                ShakeOffset {
                    dx: 0.0,
                    dy: ry * intensity,
                }
            }

            Shake::Decay {
                intensity,
                decay,
                frequency,
            } => {
                // Exponential decay based on progress
                let decay_factor = (-decay * progress).exp();
                let current_intensity = intensity * decay_factor;
                let (rx, ry) = random_offset(*frequency, frame);
                ShakeOffset {
                    dx: rx * current_intensity,
                    dy: ry * current_intensity,
                }
            }
        }
    }

    /// Create a uniform shake with given intensity.
    pub fn uniform(intensity: f32) -> Self {
        Self::Uniform {
            intensity,
            frequency: default_frequency(),
        }
    }

    /// Create a horizontal shake with given intensity.
    pub fn horizontal(intensity: f32) -> Self {
        Self::Horizontal {
            intensity,
            frequency: default_frequency(),
        }
    }

    /// Create a vertical shake with given intensity.
    pub fn vertical(intensity: f32) -> Self {
        Self::Vertical {
            intensity,
            frequency: default_frequency(),
        }
    }

    /// Create a decaying shake (impact style).
    pub fn decay(intensity: f32) -> Self {
        Self::Decay {
            intensity,
            decay: default_decay_rate(),
            frequency: default_frequency(),
        }
    }
}

/// Generate pseudo-random offset values using fast_random.
/// Returns values in range -1.0 to 1.0 for both x and y.
/// ~25x faster than ChaCha8-based Rng.
fn random_offset(frequency: f32, frame: u64) -> (f32, f32) {
    use mixed_signals::math::fast_random;
    // Use frequency to determine how often offset changes
    let effective_frame = (frame as f32 * frequency / 60.0) as u64;

    // Generate two independent pseudo-random values with different seeds
    // fast_random returns 0.0..1.0, scale to [-1, 1]
    let rx = fast_random(0xDEAD_u64, effective_frame) * 2.0 - 1.0;
    let ry = fast_random(0xBEEF_u64, effective_frame) * 2.0 - 1.0;

    (rx, ry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_no_offset() {
        let shake = Shake::None;
        let offset = shake.offset(0.5, 10);
        assert!((offset.dx).abs() < 0.001);
        assert!((offset.dy).abs() < 0.001);
    }

    #[test]
    fn test_uniform_bounded() {
        let shake = Shake::Uniform {
            intensity: 2.0,
            frequency: 10.0,
        };
        for frame in 0..100 {
            let offset = shake.offset(0.5, frame);
            assert!(offset.dx.abs() <= 2.0);
            assert!(offset.dy.abs() <= 2.0);
        }
    }

    #[test]
    fn test_horizontal_no_vertical() {
        let shake = Shake::Horizontal {
            intensity: 2.0,
            frequency: 10.0,
        };
        for frame in 0..100 {
            let offset = shake.offset(0.5, frame);
            assert!((offset.dy).abs() < 0.001);
        }
    }

    #[test]
    fn test_vertical_no_horizontal() {
        let shake = Shake::Vertical {
            intensity: 2.0,
            frequency: 10.0,
        };
        for frame in 0..100 {
            let offset = shake.offset(0.5, frame);
            assert!((offset.dx).abs() < 0.001);
        }
    }

    #[test]
    fn test_decay_reduces_over_time() {
        let shake = Shake::Decay {
            intensity: 5.0,
            decay: 3.0,
            frequency: 10.0,
        };
        // At progress=0, should have near-full intensity
        let early_offset = shake.offset(0.0, 10);
        // At progress=1, should have decayed significantly
        let late_offset = shake.offset(1.0, 10);

        // The late offset should be smaller in magnitude
        let early_mag = (early_offset.dx.powi(2) + early_offset.dy.powi(2)).sqrt();
        let late_mag = (late_offset.dx.powi(2) + late_offset.dy.powi(2)).sqrt();

        // With decay=3.0, at t=1.0, factor = e^-3 ≈ 0.05
        assert!(late_mag < early_mag * 0.2);
    }

    #[test]
    fn test_deterministic() {
        let shake = Shake::uniform(1.0);
        let offset1 = shake.offset(0.5, 42);
        let offset2 = shake.offset(0.5, 42);
        assert!((offset1.dx - offset2.dx).abs() < 0.0001);
        assert!((offset1.dy - offset2.dy).abs() < 0.0001);
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_shake.rs</FILE> - <DESC>Random position offset for shake effects</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
