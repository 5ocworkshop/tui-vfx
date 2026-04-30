// <FILE>tui-vfx-compositor/src/samplers/cls_ripple.rs</FILE> - <DESC>Ripple sampler with configurable center</DESC>
// <VERS>VERSION: 3.3.0</VERS>
// <WCTX>2026-04-26 packet — migrate sample() return to SamplerOutput so the orchestrator can thread the displacement delta into ctx.resolved_x.</WCTX>
// <CLOG>3.3.0: sample() now returns SamplerOutput; center short-circuit uses passthrough(); displacing branch carries full x/y deltas; out-of-bounds returns no_displacement().</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use crate::types::cls_sampler_spec::RippleCenter;
use mixed_signals::prelude::{Normalized, Remap, Signal, SignalExt, Sine};
use tui_vfx_types::VfxCellContext;

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
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;
        let t = ctx.t as f32;
        // Determine center based on configuration
        let (center_x, center_y) = match self.center {
            RippleCenter::Center => (ctx.width as f32 / 2.0, ctx.height as f32 / 2.0),
            RippleCenter::Point { x, y } => (x as f32, y as f32),
        };

        let dx = dest_x as f32 - center_x;
        let dy = dest_y as f32 - center_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < 0.001 {
            // At the center, no displacement
            return SamplerOutput::passthrough(dest_x, dest_y);
        }

        // Calculate ripple displacement using remapped Sine
        // The wave moves outward over time
        let phase = dist / self.wavelength - t * self.speed;
        let displacement = self.signal.sample(phase.into());

        // Normalize direction vector
        let nx = dx / dist;
        let ny = dy / dist;

        // Apply radial displacement
        let src_x_f = dest_x as f32 + nx * displacement;
        let src_y_f = dest_y as f32 + ny * displacement;

        if src_x_f < 0.0 || src_y_f < 0.0 {
            SamplerOutput::no_displacement()
        } else {
            let src_x = src_x_f.round() as u16;
            let src_y = src_y_f.round() as u16;
            let delta_x = src_x as i32 - dest_x as i32;
            let delta_y = src_y as i32 - dest_y as i32;
            SamplerOutput::displaced(src_x, src_y, delta_x, delta_y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WIDTH: u16 = 20;
    const TEST_HEIGHT: u16 = 20;

    fn ctx_at(x: u16, y: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, TEST_WIDTH, TEST_HEIGHT, 0, 0, t)
    }

    #[test]
    fn test_ripple_default() {
        let ripple = Ripple::default();
        assert_eq!(ripple.amplitude(), 1.5);
        assert_eq!(ripple.wavelength, 4.0);
    }

    #[test]
    fn test_ripple_at_center_no_displacement() {
        let ripple = Ripple::default();
        let result = ripple.sample(&ctx_at(10, 10, 0.0));
        assert_eq!(result.source, Some((10, 10)));
    }

    #[test]
    fn test_ripple_returns_some() {
        let ripple = Ripple::default();
        let result = ripple.sample(&ctx_at(5, 5, 0.5));
        assert!(result.source.is_some());
    }

    #[test]
    fn test_ripple_varies_with_time() {
        let ripple = Ripple::default();
        let r1 = ripple.sample(&ctx_at(15, 10, 0.0));
        let r2 = ripple.sample(&ctx_at(15, 10, 0.25));
        assert!(r1.source.is_some() && r2.source.is_some());
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // Off-center cell at (15, 10) is 5 cells from center (10, 10)
        // At some t, the ripple phase produces a non-zero displacement
        let ripple = Ripple::new(1.5, 4.0, 2.0, RippleCenter::Center);
        let out = ripple.sample(&ctx_at(15, 10, 0.0));
        assert!(out.source.is_some());
        // Deltas must equal src - dest
        let (src_x, src_y) = out.source.unwrap();
        assert_eq!(out.delta_x, src_x as i32 - 15);
        assert_eq!(out.delta_y, src_y as i32 - 10);
        // With amplitude 1.5 and horizontal displacement from center, delta_x should be non-zero
        assert!(out.delta_x != 0 || out.delta_y != 0);
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_ripple.rs</FILE> - <DESC>Ripple sampler with configurable center</DESC>
// <VERS>END OF VERSION: 3.3.0</VERS>
