// <FILE>crates/tui-vfx-content/src/mechanical/fnc_apply_settle.rs</FILE> - <DESC>Apply per-tile settle behavior (Spring detent, Ease curve, or None) to local progress</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 3 of mechanical circular content cycles plan: per-tile settle that lets each tile click into its target face like an odometer wheel; composes with cascade so staggered tiles each get their own detent.</WCTX>
// <CLOG>0.1.0: introduce settle_sample_for returning a discriminated Route/Overshoot sample so callers know whether to roll the route or display the overshoot face directly.</CLOG>

use crate::types::{EasingCurveName, MechanicalSettleConfig};

/// What to render at this tile this frame.
///
/// `Route { progress }` tells the caller to walk the route at the
/// given progress (0..=1). `Overshoot` tells the caller to render the
/// overshoot face — one face beyond the route's final face in the
/// route's direction. Callers without an overshoot face available
/// (bounded cycle at an edge) silently fall back to
/// `Route { progress: 1.0 }`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SettleSample {
    Route { progress: f64 },
    Overshoot,
}

/// Map per-tile local progress through the settle config.
///
/// `None` is identity: route progress equals local progress. `Spring`
/// compresses the route into the `1 - settle_fraction` window at the
/// start, then renders the overshoot face for the first
/// `overshoot * settle_fraction` portion of the settle phase, then
/// the target face. `Ease` applies a named easing curve over the
/// whole local progress.
pub(crate) fn settle_sample_for(
    settle: &MechanicalSettleConfig,
    tile_local_progress: f64,
) -> SettleSample {
    let p = tile_local_progress.clamp(0.0, 1.0);
    match settle {
        MechanicalSettleConfig::None => SettleSample::Route { progress: p },
        MechanicalSettleConfig::Spring {
            overshoot,
            settle_fraction,
        } => spring_sample(*overshoot, *settle_fraction, p),
        MechanicalSettleConfig::Ease { easing } => SettleSample::Route {
            progress: apply_easing(*easing, p),
        },
    }
}

fn spring_sample(overshoot: f32, settle_fraction: f32, p: f64) -> SettleSample {
    let sf = (settle_fraction.clamp(0.0, 1.0)) as f64;
    let os = (overshoot.clamp(0.0, 0.5)) as f64;

    // Degenerate: if settle_fraction is zero, settle is effectively
    // disabled; treat as identity.
    if sf <= f64::EPSILON {
        return SettleSample::Route { progress: p };
    }

    let active_end = 1.0 - sf;
    if p < active_end {
        let active_window = active_end.max(f64::EPSILON);
        return SettleSample::Route {
            progress: (p / active_window).min(1.0),
        };
    }

    // Settle phase: p in [active_end, 1.0].
    let settle_p = ((p - active_end) / sf).clamp(0.0, 1.0);
    let overshoot_window = os * 2.0; // os in [0, 0.5] → [0, 1]
    if overshoot_window > 0.0 && settle_p < overshoot_window {
        SettleSample::Overshoot
    } else {
        SettleSample::Route { progress: 1.0 }
    }
}

fn apply_easing(easing: EasingCurveName, p: f64) -> f64 {
    let p = p.clamp(0.0, 1.0);
    match easing {
        EasingCurveName::Linear => p,
        EasingCurveName::EaseOut => 1.0 - (1.0 - p).powi(3),
        EasingCurveName::EaseOutBack => {
            // Snappy back-out: brief overshoot inside [0, 1] envelope.
            // c1 and c3 chosen so the curve peaks slightly above 1
            // mid-way, then settles to 1 at p=1. We then clamp the
            // returned progress to [0, 1] because route sampling is
            // bounded; the visible "back" feel comes from the steep
            // approach into 1.0, not from out-of-range progress.
            let c1: f64 = 1.70158;
            let c3 = c1 + 1.0;
            let v = 1.0 + c3 * (p - 1.0).powi(3) + c1 * (p - 1.0).powi(2);
            v.clamp(0.0, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() < tol, "expected ~{b}, got {a}");
    }

    const F32_F64_TOL: f64 = 1e-6;

    #[test]
    fn none_settle_is_identity() {
        let sample = settle_sample_for(&MechanicalSettleConfig::None, 0.42);
        assert!(matches!(
            sample,
            SettleSample::Route { progress } if (progress - 0.42).abs() < F32_F64_TOL
        ));
    }

    #[test]
    fn none_settle_clamps_progress() {
        match settle_sample_for(&MechanicalSettleConfig::None, -0.5) {
            SettleSample::Route { progress } => assert_eq!(progress, 0.0),
            _ => panic!(),
        }
        match settle_sample_for(&MechanicalSettleConfig::None, 1.5) {
            SettleSample::Route { progress } => assert_eq!(progress, 1.0),
            _ => panic!(),
        }
    }

    #[test]
    fn spring_active_phase_compresses_route_into_active_window() {
        let cfg = MechanicalSettleConfig::Spring {
            overshoot: 0.12,
            settle_fraction: 0.2,
        };
        // At local progress 0.4 (midway through active phase 0..0.8):
        // expected route progress = 0.4 / 0.8 = 0.5.
        match settle_sample_for(&cfg, 0.4) {
            SettleSample::Route { progress } => assert_close(progress, 0.5, F32_F64_TOL),
            other => panic!("expected Route, got {other:?}"),
        }
    }

    #[test]
    fn spring_settle_phase_first_portion_is_overshoot() {
        let cfg = MechanicalSettleConfig::Spring {
            overshoot: 0.5, // maximum overshoot → full settle window is overshoot
            settle_fraction: 0.2,
        };
        // local progress 0.85 → settle_p = (0.85 - 0.8) / 0.2 = 0.25 (in
        // overshoot window 0..1). Expect Overshoot.
        let sample = settle_sample_for(&cfg, 0.85);
        assert_eq!(sample, SettleSample::Overshoot);
    }

    #[test]
    fn spring_settle_target_after_overshoot_window() {
        let cfg = MechanicalSettleConfig::Spring {
            overshoot: 0.12,
            settle_fraction: 0.2,
        };
        // overshoot window = 0.12 * 2 = 0.24 of settle phase.
        // local progress 0.85 → settle_p = 0.25 > 0.24 → target.
        match settle_sample_for(&cfg, 0.85) {
            SettleSample::Route { progress } => assert_close(progress, 1.0, F32_F64_TOL),
            other => panic!("expected Route at target, got {other:?}"),
        }
    }

    #[test]
    fn spring_with_zero_overshoot_never_renders_overshoot() {
        let cfg = MechanicalSettleConfig::Spring {
            overshoot: 0.0,
            settle_fraction: 0.2,
        };
        for p in [0.81, 0.85, 0.9, 0.95, 1.0] {
            match settle_sample_for(&cfg, p) {
                SettleSample::Route { .. } => {}
                SettleSample::Overshoot => panic!("zero overshoot must not render Overshoot"),
            }
        }
    }

    #[test]
    fn spring_with_zero_settle_fraction_is_identity() {
        let cfg = MechanicalSettleConfig::Spring {
            overshoot: 0.5,
            settle_fraction: 0.0,
        };
        match settle_sample_for(&cfg, 0.7) {
            SettleSample::Route { progress } => assert_close(progress, 0.7, F32_F64_TOL),
            _ => panic!(),
        }
    }

    #[test]
    fn spring_lands_target_at_progress_one() {
        let cfg = MechanicalSettleConfig::Spring {
            overshoot: 0.12,
            settle_fraction: 0.2,
        };
        // At local progress 1.0, settle_p = 1.0; overshoot window
        // exhausted; expect target.
        match settle_sample_for(&cfg, 1.0) {
            SettleSample::Route { progress } => assert_close(progress, 1.0, F32_F64_TOL),
            _ => panic!(),
        }
    }

    #[test]
    fn ease_linear_is_identity() {
        let cfg = MechanicalSettleConfig::Ease {
            easing: EasingCurveName::Linear,
        };
        match settle_sample_for(&cfg, 0.42) {
            SettleSample::Route { progress } => assert_close(progress, 0.42, F32_F64_TOL),
            _ => panic!(),
        }
    }

    #[test]
    fn ease_out_is_monotonic_and_lands_at_one() {
        let cfg = MechanicalSettleConfig::Ease {
            easing: EasingCurveName::EaseOut,
        };
        let mut prev = -1.0;
        for step in 0..=10 {
            let p = step as f64 / 10.0;
            match settle_sample_for(&cfg, p) {
                SettleSample::Route { progress } => {
                    assert!(progress >= prev - F32_F64_TOL, "non-monotonic at p={p}");
                    prev = progress;
                }
                _ => panic!(),
            }
        }
        assert_close(prev, 1.0, F32_F64_TOL);
    }

    #[test]
    fn ease_out_back_clamps_to_unit_interval() {
        let cfg = MechanicalSettleConfig::Ease {
            easing: EasingCurveName::EaseOutBack,
        };
        for step in 0..=10 {
            let p = step as f64 / 10.0;
            match settle_sample_for(&cfg, p) {
                SettleSample::Route { progress } => {
                    assert!((0.0..=1.0).contains(&progress));
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn spring_overshoot_clamps_to_half() {
        let cfg = MechanicalSettleConfig::Spring {
            overshoot: 5.0, // intentionally out of range
            settle_fraction: 0.2,
        };
        // Even with overshoot=5.0, the clamp to 0.5 means full settle
        // window is overshoot. At settle_p just below 1, still overshoot.
        let sample = settle_sample_for(&cfg, 0.99);
        assert_eq!(sample, SettleSample::Overshoot);
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_apply_settle.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
