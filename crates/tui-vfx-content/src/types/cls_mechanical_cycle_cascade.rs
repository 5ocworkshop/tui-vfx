// <FILE>tui-vfx-content/src/types/cls_mechanical_cycle_cascade.rs</FILE> - <DESC>Public schema for cycle cascade scheduling and settle behavior</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 1 of mechanical circular content cycles plan: per-tile settle composition with cascade; settle ships fully wired with the runtime in its introducing phase.</WCTX>
// <CLOG>0.2.0: clarify settle is per-tile and composes with cascade; remove parse-but-inert allowance from rustdoc.</CLOG>

use serde::{Deserialize, Serialize};

/// Schedules per-tile progress relative to the overall cycle progress.
///
/// `Simultaneous` reproduces today's whole-grid behavior; the others stagger
/// tiles so the cycle visually progresses across the mechanism.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MechanicalCascadePolicy {
    /// All tiles share the same progress.
    Simultaneous,
    /// Each tile starts later than the previous one by `fraction` of the
    /// total progress window. `fraction` is clamped to `0.0..=0.95`.
    Staggered {
        fraction: f32,
    },
    /// Numeric carry: only digit positions that change between source and
    /// target advance, scheduled from least-significant to most-significant.
    NumericCarry {
        #[serde(default = "default_stagger_fraction")]
        stagger_fraction: f32,
        #[serde(default)]
        unchanged: UnchangedCellPolicy,
    },
    /// Each tile starts at a deterministic random offset derived from `seed`.
    Randomized {
        seed: u64,
        max_delay_fraction: f32,
    },
}

impl Default for MechanicalCascadePolicy {
    fn default() -> Self {
        Self::Simultaneous
    }
}

fn default_stagger_fraction() -> f32 {
    0.35
}

/// What happens to tiles whose source and target faces are equal under
/// `MechanicalCascadePolicy::NumericCarry`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum UnchangedCellPolicy {
    /// The tile holds its existing face for the duration of the cycle.
    #[default]
    Hold,
    /// The tile spins through the cycle but lands back on the same face.
    SpinAndReturn,
}

/// Per-tile settle behavior applied at target arrival.
///
/// Settle composes with `MechanicalCascadePolicy` so each tile gets its
/// own detent in its own time window. With `Simultaneous` cascade the
/// per-tile rule degrades to a whole-cycle settle; with `Staggered` or
/// `NumericCarry` cascade it produces the click-click-click of an
/// odometer ratcheting into place.
///
/// Every variant ships fully wired in the phase that introduces it: there
/// is no "parses but does nothing at runtime" allowance.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MechanicalSettleConfig {
    /// No settle. The tile lands at local progress `1.0` and stops.
    None,
    /// Brief overshoot past the final face then recovery onto it.
    /// `overshoot` is clamped to `0.0..=0.5` and is how far past the final
    /// face the tile travels at peak. `settle_fraction` is clamped to
    /// `0.0..=1.0` and is the fraction of the tile's local progress
    /// devoted to the settle phase; the route advance compresses into
    /// `(1.0 - settle_fraction)`.
    Spring {
        overshoot: f32,
        settle_fraction: f32,
    },
    /// Apply a named easing curve over the final stretch of the tile's
    /// local progress.
    Ease {
        easing: EasingCurveName,
    },
}

impl Default for MechanicalSettleConfig {
    fn default() -> Self {
        Self::None
    }
}

/// Small named easing subset usable by `MechanicalSettleConfig`.
///
/// This is a deliberate, narrow vocabulary chosen to match the shapes a
/// settle motion benefits from. The full easing library lives in the
/// `mixed-signals` crate; broader settle expressiveness is a future
/// extension once it gains `ConfigSchema` support.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum EasingCurveName {
    #[default]
    Linear,
    EaseOut,
    EaseOutBack,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cascade_default_is_simultaneous() {
        assert_eq!(MechanicalCascadePolicy::default(), MechanicalCascadePolicy::Simultaneous);
    }

    #[test]
    fn settle_default_is_none() {
        assert_eq!(MechanicalSettleConfig::default(), MechanicalSettleConfig::None);
    }

    #[test]
    fn unchanged_default_is_hold() {
        assert_eq!(UnchangedCellPolicy::default(), UnchangedCellPolicy::Hold);
    }

    #[test]
    fn easing_default_is_linear() {
        assert_eq!(EasingCurveName::default(), EasingCurveName::Linear);
    }

    #[test]
    fn simultaneous_serde_roundtrip() {
        let json = serde_json::to_string(&MechanicalCascadePolicy::Simultaneous).unwrap();
        assert_eq!(json, r#"{"type":"simultaneous"}"#);
        let back: MechanicalCascadePolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(back, MechanicalCascadePolicy::Simultaneous);
    }

    #[test]
    fn numeric_carry_default_stagger_fraction() {
        let parsed: MechanicalCascadePolicy =
            serde_json::from_str(r#"{"type":"numeric_carry"}"#).unwrap();
        match parsed {
            MechanicalCascadePolicy::NumericCarry { stagger_fraction, unchanged } => {
                assert!((stagger_fraction - 0.35).abs() < f32::EPSILON);
                assert_eq!(unchanged, UnchangedCellPolicy::Hold);
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn spring_settle_serde_roundtrip() {
        let cfg = MechanicalSettleConfig::Spring { overshoot: 0.12, settle_fraction: 0.18 };
        let json = serde_json::to_string(&cfg).unwrap();
        let back: MechanicalSettleConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn unknown_settle_field_rejected() {
        let parsed: Result<MechanicalSettleConfig, _> =
            serde_json::from_str(r#"{"type":"spring","overshoot":0.1,"settle_fraction":0.2,"flair":"yes"}"#);
        assert!(parsed.is_err());
    }
}

// <FILE>tui-vfx-content/src/types/cls_mechanical_cycle_cascade.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
