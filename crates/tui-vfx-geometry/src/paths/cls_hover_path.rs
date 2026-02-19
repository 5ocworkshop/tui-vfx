// <FILE>tui-vfx-geometry/src/paths/cls_hover_path.rs</FILE> - <DESC>Idle floating path</DESC>
// <VERS>VERSION: 1.2.1</VERS>
// <WCTX>mixed-signals v2 bipolar migration</WCTX>
// <CLOG>Add .normalized() to Sine for 0-1 output after mixed-signals v2 bipolar change

use crate::traits::MotionPath;
use crate::types::Position;
use mixed_signals::prelude::{Normalized, Remap, Signal, SignalExt, Sine};

/// Idle floating/hovering path using mixed_signals::Sine with Remap.
///
/// Creates a gentle bobbing motion at the end position.
/// Uses .normalized() to restore 0-1 output, then Remap to scale to desired amplitude range.
pub struct HoverPath {
    /// The remapped sine signal outputting -amplitude to +amplitude
    signal: Remap<Normalized<Sine>>,
    /// Oscillation frequency (stored for sample input calculation)
    frequency: f32,
    /// Stored amplitude for accessor
    amplitude: f32,
}

impl HoverPath {
    /// Create a new HoverPath with given amplitude and frequency.
    ///
    /// # Arguments
    /// * `amplitude` - Maximum vertical displacement in cells
    /// * `frequency` - Oscillation frequency (cycles per unit time)
    pub fn new(amplitude: f32, frequency: f32) -> Self {
        // Create normalized sine (0-1), then remap to bipolar amplitude range
        // Use frequency = 1/(2*PI) so sample(t * freq) follows sin(t * freq) timing
        let base_sine = Sine::new(1.0 / std::f32::consts::TAU, 1.0, 0.0, 0.0).normalized();
        // Remap 0..1 to -amplitude..+amplitude for bidirectional displacement
        let signal = Remap::new(base_sine, 0.0, 1.0, -amplitude, amplitude);
        Self {
            signal,
            frequency,
            amplitude,
        }
    }

    /// Get the amplitude of the hover.
    pub fn amplitude(&self) -> f32 {
        self.amplitude
    }

    /// Get the frequency of the hover.
    pub fn frequency(&self) -> f32 {
        self.frequency
    }
}

impl Default for HoverPath {
    fn default() -> Self {
        Self::new(2.0, 1.0)
    }
}

/// Legacy-compatible struct for deserialization.
#[derive(Clone)]
pub struct HoverPathSpec {
    pub amplitude: f32,
    pub frequency: f32,
}

impl From<HoverPathSpec> for HoverPath {
    fn from(spec: HoverPathSpec) -> Self {
        Self::new(spec.amplitude, spec.frequency)
    }
}

impl MotionPath for HoverPath {
    fn calculate(&self, t: f64, _start: Position, end: Position) -> (f32, f32) {
        let ex = end.x as f32;
        let ey = end.y as f32;
        // Sample at t * frequency to get oscillation
        let offset = self.signal.sample(t * self.frequency as f64);
        (ex, ey + offset)
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_hover_path.rs</FILE> - <DESC>Idle floating path</DESC>
// <VERS>END OF VERSION: 1.2.1</VERS>
