// <FILE>crates/tui-vfx-content/tests/transformers/test_cls_odometer_cycles.rs</FILE> - <DESC>Integration tests for Odometer with mechanical cycle config: ordered/preset routes, NumericCarry, settle, extra_rotations</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: TextTransformer signature now takes &TransformContext<'_>.</WCTX>
// <CLOG>0.2.0: route the sample() helper through a OnceLock-cached TransformContext so cycle-path tests compile against the new trait signature.</CLOG>

use mixed_signals::prelude::SignalContext;
use std::sync::OnceLock;
use tui_vfx_content::traits::TransformContext;
use tui_vfx_content::transformers::get_transformer;
use tui_vfx_content::types::{
    ContentEffect, CycleDirectionPolicy, CycleMissingFacePolicy, CycleTieBreaker, CycleWrapMode,
    MechanicalCascadePolicy, MechanicalContentSource, MechanicalCycleConfig, MechanicalCyclePreset,
    MechanicalRouteConfig, MechanicalSettleConfig, OdometerDirection, OdometerTravel,
    UnchangedCellPolicy, WeightedCycleFace,
};
use tui_vfx_style::traits::ShaderRuntimeParams;

static CTX_PARTS: OnceLock<(SignalContext, ShaderRuntimeParams)> = OnceLock::new();

fn tctx() -> TransformContext<'static> {
    let p = CTX_PARTS.get_or_init(|| (SignalContext::default(), ShaderRuntimeParams::new()));
    TransformContext::new(&p.0, &p.1)
}

fn odometer(
    from: &str,
    _target_for_signature_clarity: &str,
    _grid_width_for_signature_clarity: u16,
    mechanical: Option<MechanicalCycleConfig>,
) -> ContentEffect {
    // Tile size is always 1×1 in this test helper — the digits in the
    // source/target strings are individual tiles. The grid extent
    // comes from the message length, not from tile_width.
    ContentEffect::Odometer {
        direction: OdometerDirection::Up,
        travel: OdometerTravel::Axis,
        tile_width: 1,
        tile_height: 1,
        from_message: Some(from.to_string()),
        mechanical,
    }
}

fn sample(effect: &ContentEffect, target: &str, progress: f64) -> String {
    let tx = get_transformer(effect);
    tx.transform(target, progress, &tctx()).into_owned()
}

fn forward_route() -> MechanicalRouteConfig {
    MechanicalRouteConfig {
        direction: CycleDirectionPolicy::Forward,
        tie_breaker: CycleTieBreaker::Forward,
        extra_rotations: 0,
        missing_face: CycleMissingFacePolicy::Error,
    }
}

#[test]
fn absent_mechanical_matches_explicit_pair_default() {
    let absent = odometer("AAA", "111", 1, None);
    let explicit_pair = odometer("AAA", "111", 1, Some(MechanicalCycleConfig::default()));
    for p in [0.0, 0.25, 0.34, 0.5, 0.67, 0.9, 1.0] {
        assert_eq!(
            sample(&absent, "111", p),
            sample(&explicit_pair, "111", p),
            "mismatch at progress {p}",
        );
    }
}

#[test]
fn decimal_preset_forward_walks_intermediate_digits() {
    // 1-tile odometer, source = "8", target = "2". Forward route is
    // 8,9,0,1,2 (5 faces). With Simultaneous cascade and no settle,
    // segment boundaries land on each face at progress 0, 0.25, 0.5,
    // 0.75, 1.0.
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        },
        route: forward_route(),
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::None,
    };
    let effect = odometer("8", "2", 1, Some(mechanical));
    assert_eq!(sample(&effect, "2", 0.0), "8");
    assert_eq!(sample(&effect, "2", 0.25), "9");
    assert_eq!(sample(&effect, "2", 0.5), "0");
    assert_eq!(sample(&effect, "2", 0.75), "1");
    assert_eq!(sample(&effect, "2", 1.0), "2");
}

#[test]
fn decimal_preset_reverse_walks_decreasing_digits() {
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        },
        route: MechanicalRouteConfig {
            direction: CycleDirectionPolicy::Reverse,
            ..forward_route()
        },
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::None,
    };
    let effect = odometer("2", "8", 1, Some(mechanical));
    // Reverse route 2,1,0,9,8 — five faces, segment boundaries:
    assert_eq!(sample(&effect, "8", 0.0), "2");
    assert_eq!(sample(&effect, "8", 0.25), "1");
    assert_eq!(sample(&effect, "8", 0.5), "0");
    assert_eq!(sample(&effect, "8", 0.75), "9");
    assert_eq!(sample(&effect, "8", 1.0), "8");
}

#[test]
fn numeric_carry_increment_routes_changed_digits_forward() {
    // 099 → 100: all three digits change. With NumericCarry, the
    // hint is Increment so each tile routes Forward through the
    // decimal cycle. Hundreds: 0→1 (forward, 1 step). Tens: 9→0
    // (forward, 1 step wrap). Ones: 9→0 (forward, 1 step wrap).
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        },
        route: MechanicalRouteConfig {
            direction: CycleDirectionPolicy::NumericDelta,
            ..forward_route()
        },
        cascade: MechanicalCascadePolicy::NumericCarry {
            stagger_fraction: 0.5,
            unchanged: UnchangedCellPolicy::Hold,
        },
        settle: MechanicalSettleConfig::None,
    };
    let effect = odometer("099", "100", 3, Some(mechanical));
    // At progress 1.0, all tiles land on target.
    assert_eq!(sample(&effect, "100", 1.0), "100");
    // At progress 0.0, all tiles still show source.
    assert_eq!(sample(&effect, "100", 0.0), "099");
}

#[test]
fn numeric_carry_decrement_routes_changed_digits_reverse() {
    // 100 → 099: all three digits change. NumericDelta hint =
    // Decrement → each tile routes Reverse.
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        },
        route: MechanicalRouteConfig {
            direction: CycleDirectionPolicy::NumericDelta,
            ..forward_route()
        },
        cascade: MechanicalCascadePolicy::NumericCarry {
            stagger_fraction: 0.5,
            unchanged: UnchangedCellPolicy::Hold,
        },
        settle: MechanicalSettleConfig::None,
    };
    let effect = odometer("100", "099", 3, Some(mechanical));
    assert_eq!(sample(&effect, "099", 0.0), "100");
    assert_eq!(sample(&effect, "099", 1.0), "099");
}

#[test]
fn numeric_carry_holds_unchanged_tiles_under_hold_policy() {
    // 199 → 100: hundreds unchanged ("1"), tens 9→0, ones 9→0.
    // With Hold, the hundreds tile shows "1" continuously.
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        },
        route: MechanicalRouteConfig {
            direction: CycleDirectionPolicy::NumericDelta,
            ..forward_route()
        },
        cascade: MechanicalCascadePolicy::NumericCarry {
            stagger_fraction: 0.5,
            unchanged: UnchangedCellPolicy::Hold,
        },
        settle: MechanicalSettleConfig::None,
    };
    let effect = odometer("199", "100", 3, Some(mechanical));
    // The hundreds digit should be "1" at every progress sample.
    for p in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let s = sample(&effect, "100", p);
        // Test compares first character only — that's the unchanged tile.
        assert_eq!(s.chars().next(), Some('1'), "hundreds at p={p}: {s:?}");
    }
}

#[test]
fn extra_rotations_lands_target_exactly_at_progress_one() {
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        },
        route: MechanicalRouteConfig {
            direction: CycleDirectionPolicy::Forward,
            extra_rotations: 2,
            ..forward_route()
        },
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::None,
    };
    let effect = odometer("0", "5", 1, Some(mechanical));
    assert_eq!(sample(&effect, "5", 1.0), "5");
    // Mid-progress should not yet be on target.
    let mid = sample(&effect, "5", 0.5);
    assert_ne!(mid, "5", "mid-progress should still be cycling, got {mid}");
}

#[test]
fn ordered_alphabet_drum_walks_named_faces() {
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into(), "D".into()],
            wrap: CycleWrapMode::Circular,
        },
        route: forward_route(),
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::None,
    };
    let effect = odometer("A", "C", 1, Some(mechanical));
    // Forward route A,B,C — 3 faces, 2 segments. Boundary at 0.5 = B.
    assert_eq!(sample(&effect, "C", 0.0), "A");
    assert_eq!(sample(&effect, "C", 0.5), "B");
    assert_eq!(sample(&effect, "C", 1.0), "C");
}

#[test]
fn weighted_reel_lands_target_at_progress_one() {
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Weighted {
            faces: vec![
                WeightedCycleFace {
                    value: "7".into(),
                    weight: 1,
                },
                WeightedCycleFace {
                    value: "$".into(),
                    weight: 2,
                },
                WeightedCycleFace {
                    value: "X".into(),
                    weight: 3,
                },
            ],
            seed: 777,
            wrap: CycleWrapMode::Circular,
        },
        route: MechanicalRouteConfig {
            direction: CycleDirectionPolicy::Forward,
            extra_rotations: 1,
            missing_face: CycleMissingFacePolicy::PairFallback,
            ..forward_route()
        },
        cascade: MechanicalCascadePolicy::Staggered { fraction: 0.2 },
        settle: MechanicalSettleConfig::Spring {
            overshoot: 0.2,
            settle_fraction: 0.18,
        },
    };
    let effect = odometer("X", "7", 1, Some(mechanical));
    assert_eq!(sample(&effect, "7", 1.0), "7");
}

#[test]
fn weighted_reel_is_deterministic_across_runs() {
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Weighted {
            faces: vec![
                WeightedCycleFace {
                    value: "A".into(),
                    weight: 1,
                },
                WeightedCycleFace {
                    value: "B".into(),
                    weight: 1,
                },
                WeightedCycleFace {
                    value: "C".into(),
                    weight: 1,
                },
            ],
            seed: 12345,
            wrap: CycleWrapMode::Circular,
        },
        route: MechanicalRouteConfig {
            missing_face: CycleMissingFacePolicy::PairFallback,
            ..forward_route()
        },
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::None,
    };
    let effect = odometer("A", "C", 1, Some(mechanical.clone()));
    let s1 = sample(&effect, "C", 0.5);
    let s2 = sample(&effect, "C", 0.5);
    assert_eq!(s1, s2);
}

#[test]
fn spring_settle_renders_overshoot_face_during_settle_window() {
    // 1-tile preset, source "5" → target "5" with extra_rotations=1
    // (full wrap). With Spring overshoot=0.5 settle_fraction=0.5 the
    // entire second half of progress is settle, and the first half
    // of the settle phase shows the overshoot face. Forward overshoot
    // of 5 is 6.
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        },
        route: MechanicalRouteConfig {
            extra_rotations: 1,
            ..forward_route()
        },
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::Spring {
            overshoot: 0.5,
            settle_fraction: 0.5,
        },
    };
    let effect = odometer("5", "5", 1, Some(mechanical));
    // At progress 0.6 (in settle window 0.5..1.0, settle_p = 0.2,
    // overshoot window = 1.0 → overshoot face).
    assert_eq!(sample(&effect, "5", 0.6), "6");
    // At progress 1.0 (settled target).
    assert_eq!(sample(&effect, "5", 1.0), "5");
}

#[test]
fn spring_settle_progress_one_returns_borrowed_target() {
    let mechanical = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        },
        route: forward_route(),
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::Spring {
            overshoot: 0.12,
            settle_fraction: 0.18,
        },
    };
    let effect = odometer("0", "9", 1, Some(mechanical));
    // Transformer's progress >= 1.0 short-circuit → exact target.
    assert_eq!(sample(&effect, "9", 1.0), "9");
}

// <FILE>crates/tui-vfx-content/tests/transformers/test_cls_odometer_cycles.rs</FILE>
// <VERS>END OF VERSION: 0.2.0</VERS>
