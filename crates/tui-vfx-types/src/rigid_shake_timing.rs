// <FILE>crates/tui-vfx-types/src/rigid_shake_timing.rs</FILE> - <DESC>Shared timing calculation for RigidShake effects</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Create shared timing utility for RigidShake filter and style effects</WCTX>
// <CLOG>Initial implementation with configurable parameters and offset calculation</CLOG>

//! Shared timing calculation for RigidShake effects.
//!
//! This module provides the core timing logic for the "ketchup bottle" damped
//! oscillation pattern used by RigidShake filters and style effects.
//!
//! # Usage
//!
//! ```ignore
//! use tui_vfx_types::rigid_shake_timing::{RigidShakeTiming, RigidShakeState};
//!
//! let timing = RigidShakeTiming::default();
//! let state = timing.calculate(elapsed_seconds);
//!
//! if state.is_shifting_right() {
//!     // Apply italic, shift text, etc.
//! }
//! ```

/// Configuration for RigidShake timing calculation.
#[derive(Debug, Clone)]
pub struct RigidShakeTiming {
    /// Duration of one back-and-forth shake in seconds.
    pub shake_period: f32,
    /// Number of shakes before pause (max 8).
    pub num_shakes: u8,
    /// Duration of pause between shake cycles in seconds.
    pub pause_duration: f32,
    /// Maximum extension in eighths of a cell.
    pub max_eighths: u8,
    /// Base extension always visible at rest.
    pub base_eighths: u8,
    /// Amplitude multipliers for each shake (damping curve).
    pub damping: [f32; 8],
}

impl Default for RigidShakeTiming {
    fn default() -> Self {
        Self {
            shake_period: 0.29,
            num_shakes: 4,
            pause_duration: 0.52,
            max_eighths: 12,
            base_eighths: 3,
            damping: [1.0, 0.7, 0.45, 0.25, 0.15, 0.1, 0.05, 0.0],
        }
    }
}

/// Current state of a RigidShake animation.
#[derive(Debug, Clone, Copy)]
pub struct RigidShakeState {
    /// Current offset in eighths of a cell.
    /// Positive = right shift, negative = left shift.
    pub offset_eighths: i16,
    /// Raw oscillation value (-1.0 to 1.0).
    pub raw_offset: f32,
    /// Whether currently in the pause phase.
    pub in_pause: bool,
    /// Current shake number (0-based) or None if in pause.
    pub shake_num: Option<u8>,
}

impl RigidShakeState {
    /// Returns true if the element is shifting right (beyond base).
    #[inline]
    pub fn is_shifting_right(&self) -> bool {
        self.raw_offset > 0.0
    }

    /// Returns true if the element is shifting left.
    #[inline]
    pub fn is_shifting_left(&self) -> bool {
        self.raw_offset < 0.0
    }

    /// Returns true if at rest (pause phase or center of oscillation).
    #[inline]
    pub fn is_at_rest(&self) -> bool {
        self.in_pause || self.raw_offset.abs() < 0.05
    }
}

impl RigidShakeTiming {
    /// Create a new timing configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: set shake period.
    pub fn with_shake_period(mut self, period: f32) -> Self {
        self.shake_period = period;
        self
    }

    /// Builder: set number of shakes.
    pub fn with_num_shakes(mut self, num: u8) -> Self {
        self.num_shakes = num.min(8);
        self
    }

    /// Builder: set pause duration.
    pub fn with_pause_duration(mut self, duration: f32) -> Self {
        self.pause_duration = duration;
        self
    }

    /// Builder: set max eighths.
    pub fn with_max_eighths(mut self, eighths: u8) -> Self {
        self.max_eighths = eighths.min(16);
        self
    }

    /// Builder: set base eighths.
    pub fn with_base_eighths(mut self, eighths: u8) -> Self {
        self.base_eighths = eighths.min(self.max_eighths);
        self
    }

    /// Builder: set damping curve.
    pub fn with_damping(mut self, damping: [f32; 8]) -> Self {
        self.damping = damping;
        self
    }

    /// Calculate the total cycle duration (active shaking + pause).
    #[inline]
    pub fn cycle_duration(&self) -> f32 {
        self.shake_period * self.num_shakes as f32 + self.pause_duration
    }

    /// Calculate the active duration (shaking only, no pause).
    #[inline]
    pub fn active_duration(&self) -> f32 {
        self.shake_period * self.num_shakes as f32
    }

    /// Calculate the current state at time `t` (in seconds).
    ///
    /// This is the core timing function used by both RigidShake filters
    /// and RigidShake style effects to stay in sync.
    pub fn calculate(&self, t: f64) -> RigidShakeState {
        let cycle_duration = self.cycle_duration();
        let active_duration = self.active_duration();
        let cycle_t = (t % cycle_duration as f64) as f32;

        // Calculate raw oscillation offset (-1.0 to 1.0)
        let (raw_offset, in_pause, shake_num) = if cycle_t < active_duration {
            let shake_progress = cycle_t / self.shake_period;
            let shake_idx = shake_progress.floor() as usize;
            let within_shake = shake_progress.fract();

            // Get damping for this shake
            let amplitude = if shake_idx < 8 {
                self.damping[shake_idx]
            } else {
                0.0
            };

            // Sine wave for smooth back-and-forth
            let oscillation = (within_shake * std::f32::consts::TAU).sin();
            (amplitude * oscillation, false, Some(shake_idx as u8))
        } else {
            (0.0, true, None) // Pause phase
        };

        // Scale to eighths with base offset
        let max = self.max_eighths as f32;
        let base = self.base_eighths as f32;

        let offset_eighths = if raw_offset >= 0.0 {
            // Right shift: base + additional (base to max)
            (base + raw_offset * (max - base)).round() as i16
        } else {
            // Left shift: scale from 0 to -max
            (raw_offset * max).round() as i16
        };

        RigidShakeState {
            offset_eighths,
            raw_offset,
            in_pause,
            shake_num,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let timing = RigidShakeTiming::default();
        assert_eq!(timing.shake_period, 0.29);
        assert_eq!(timing.num_shakes, 4);
        assert_eq!(timing.pause_duration, 0.52);
        assert_eq!(timing.max_eighths, 12);
        assert_eq!(timing.base_eighths, 3);
    }

    #[test]
    fn at_rest_during_pause() {
        let timing = RigidShakeTiming::default();
        // During pause (after active duration)
        let state = timing.calculate(1.5);
        assert!(state.in_pause);
        assert!(state.is_at_rest());
        assert_eq!(state.offset_eighths, 3); // base_eighths
    }

    #[test]
    fn shifting_right_at_peak() {
        let timing = RigidShakeTiming::default();
        // At ~25% of first shake period, should be near peak right
        let state = timing.calculate(0.29 * 0.25);
        assert!(state.is_shifting_right());
        assert!(!state.in_pause);
        assert_eq!(state.shake_num, Some(0));
    }

    #[test]
    fn shifting_left_at_trough() {
        let timing = RigidShakeTiming::default();
        // At ~75% of first shake period, should be near peak left
        let state = timing.calculate(0.29 * 0.75);
        assert!(state.is_shifting_left());
        assert!(!state.in_pause);
    }

    #[test]
    fn cycle_wraps() {
        let timing = RigidShakeTiming::default();
        let cycle = timing.cycle_duration() as f64;

        // States at t and t+cycle should be similar
        let state1 = timing.calculate(0.1);
        let state2 = timing.calculate(0.1 + cycle);

        assert_eq!(state1.in_pause, state2.in_pause);
        // Allow small floating point difference
        assert!((state1.raw_offset - state2.raw_offset).abs() < 0.01);
    }
}

// <FILE>crates/tui-vfx-types/src/rigid_shake_timing.rs</FILE> - <DESC>Shared timing calculation for RigidShake effects</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
