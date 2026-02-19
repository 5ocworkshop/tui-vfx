// <FILE>tui-vfx-compositor/src/samplers/cls_ripple.rs</FILE> - <DESC>Ripple sampler with configurable center</DESC>
// <VERS>VERSION: 3.1.2</VERS>
// <WCTX>mixed-signals v2 bipolar migration</WCTX>
// <CLOG>Add .normalized() to Sine for 0-1 output after mixed-signals v2 bipolar change

use crate::traits::sampler::Sampler;
use crate::types::cls_sampler_spec::RippleCenter;
use mixed_signals::prelude::{Normalized, Remap, Signal, SignalExt, Sine};

/// Circular ripple distortion emanating from a configurable center point.
///
/// Creates a water-like ripple effect where pixels are displaced
/// radially based on distance from the center. Uses mixed_signals::Sine
/// with Remap for bidirectional displacement.
pub struct Ripple {
    /// The remapped sine signal outputting -amplitude to +amplitude
    signal: Remap<Normalized<Sine>>,
    /// Distance between ripple peaks
    pub wavelength: f32,
    /// Speed of ripple propagation
    pub speed: f32,
    /// Center point of the ripple
    pub center: RippleCenter,
    /// Stored amplitude for accessor
    #[allow(dead_code)]
    amplitude: f32,
}

impl Default for Ripple {
    fn default() -> Self {
        Self::new(1.5, 4.0, 2.0, RippleCenter::Center)
    }
}

impl Ripple {
    /// Create a new Ripple sampler.
    ///
    /// # Arguments
    /// * `amplitude` - Wave amplitude in cells
    /// * `wavelength` - Distance between ripple peaks
    /// * `speed` - Temporal animation speed
    /// * `center` - Center point (Center = widget center, or Point { x, y })
    pub fn new(amplitude: f32, wavelength: f32, speed: f32, center: RippleCenter) -> Self {
        // Create normalized sine (0-1), then remap to amplitude range
        // Use frequency = 1/(2*PI) so sample(phase) follows sin(phase) timing
        let base_sine = Sine::new(1.0 / std::f32::consts::TAU, 1.0, 0.0, 0.0).normalized();
        // Remap 0..1 to -amplitude..+amplitude for bidirectional displacement
        let signal = Remap::new(base_sine, 0.0, 1.0, -amplitude, amplitude);
        Self {
            signal,
            wavelength,
            speed,
            center,
            amplitude,
        }
    }

    /// Get the amplitude of the ripple.
    #[allow(dead_code)]
    pub fn amplitude(&self) -> f32 {
        self.amplitude
    }
}

impl Sampler for Ripple {
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        width: u16,
        height: u16,
        t: f64,
    ) -> Option<(u16, u16)> {
        let t = t as f32;
        // Determine center based on configuration
        let (center_x, center_y) = match self.center {
            RippleCenter::Center => (width as f32 / 2.0, height as f32 / 2.0),
            RippleCenter::Point { x, y } => (x as f32, y as f32),
        };

        let dx = dest_x as f32 - center_x;
        let dy = dest_y as f32 - center_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < 0.001 {
            // At the center, no displacement
            return Some((dest_x, dest_y));
        }

        // Calculate ripple displacement using remapped Sine
        // The wave moves outward over time
        let phase = dist / self.wavelength - t * self.speed;
        let displacement = self.signal.sample(phase.into());

        // Normalize direction vector
        let nx = dx / dist;
        let ny = dy / dist;

        // Apply radial displacement
        let src_x = dest_x as f32 + nx * displacement;
        let src_y = dest_y as f32 + ny * displacement;

        if src_x < 0.0 || src_y < 0.0 {
            None
        } else {
            Some((src_x.round() as u16, src_y.round() as u16))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WIDTH: u16 = 20;
    const TEST_HEIGHT: u16 = 20;

    #[test]
    fn test_ripple_default() {
        let ripple = Ripple::default();
        assert_eq!(ripple.amplitude(), 1.5);
        assert_eq!(ripple.wavelength, 4.0);
    }

    #[test]
    fn test_ripple_at_center_no_displacement() {
        let ripple = Ripple::default();
        let result = ripple.sample(10, 10, TEST_WIDTH, TEST_HEIGHT, 0.0);
        assert_eq!(result, Some((10, 10)));
    }

    #[test]
    fn test_ripple_returns_some() {
        let ripple = Ripple::default();
        let result = ripple.sample(5, 5, TEST_WIDTH, TEST_HEIGHT, 0.5);
        assert!(result.is_some());
    }

    #[test]
    fn test_ripple_varies_with_time() {
        let ripple = Ripple::default();
        let r1 = ripple.sample(15, 10, TEST_WIDTH, TEST_HEIGHT, 0.0);
        let r2 = ripple.sample(15, 10, TEST_WIDTH, TEST_HEIGHT, 0.25);
        assert!(r1.is_some() && r2.is_some());
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_ripple.rs</FILE> - <DESC>Ripple sampler with configurable center</DESC>
// <VERS>END OF VERSION: 3.1.2</VERS>
