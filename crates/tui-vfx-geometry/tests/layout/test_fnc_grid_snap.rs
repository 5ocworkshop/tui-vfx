// <FILE>tui-vfx-geometry/tests/layout/test_fnc_grid_snap.rs</FILE> - <DESC>Tests for grid snapping</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-16T20:22:32Z</VERS>
// <WCTX>Turn 10 Completeness</WCTX>
// <CLOG>Restored test_floor and test_stochastic_distribution</CLOG>

use tui_vfx_geometry::layout::fnc_grid_snap::snap;
use tui_vfx_geometry::types::SnappingStrategy;
#[test]
fn test_floor() {
    assert_eq!(snap(4.9, SnappingStrategy::Floor), 4);
    assert_eq!(snap(4.1, SnappingStrategy::Floor), 4);
}
#[test]
fn test_round() {
    assert_eq!(snap(4.5, SnappingStrategy::Round), 5);
    assert_eq!(snap(4.4, SnappingStrategy::Round), 4);
}
#[test]
fn test_stochastic_determinism() {
    let strat = SnappingStrategy::Stochastic { seed: 12345 };
    let val = 4.5;
    let res1 = snap(val, strat.clone());
    let res2 = snap(val, strat);
    assert_eq!(res1, res2);
}
#[test]
fn test_stochastic_distribution() {
    let val = 4.5;
    let mut rounds_up = 0;
    let iterations = 1000;
    for i in 0..iterations {
        let strat = SnappingStrategy::Stochastic { seed: i as u64 };
        if snap(val, strat) == 5 {
            rounds_up += 1;
        }
    }
    assert!(
        rounds_up > 400 && rounds_up < 600,
        "Distribution skewed: {}",
        rounds_up
    );
}

#[test]
fn test_non_finite_returns_zero() {
    assert_eq!(snap(f32::NAN, SnappingStrategy::Stochastic { seed: 1 }), 0);
    assert_eq!(
        snap(f32::INFINITY, SnappingStrategy::Stochastic { seed: 2 }),
        0
    );
    assert_eq!(snap(-1.0, SnappingStrategy::Stochastic { seed: 3 }), 0);
}

// <FILE>tui-vfx-geometry/tests/layout/test_fnc_grid_snap.rs</FILE> - <DESC>Tests for grid snapping</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-16T20:22:32Z</VERS>
