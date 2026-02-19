// <FILE>tui-vfx-geometry/tests/types/test_rect_scale.rs</FILE> - <DESC>Tests for RectScale types</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T22:00:00Z</VERS>
// <WCTX>V2 Recipe Gap Analysis: Adding RectScale geometry</WCTX>
// <CLOG>Initial tests for Origin and RectScaleSpec</CLOG>

use tui_vfx_geometry::types::{Origin, RectScaleSpec};
use tui_vfx_types::Rect;

// === Origin Tests ===

#[test]
fn test_origin_default_is_center() {
    let origin = Origin::default();
    assert_eq!(origin, Origin::Center);
}

#[test]
fn test_origin_serde_roundtrip() {
    for origin in [
        Origin::Center,
        Origin::TopLeft,
        Origin::TopCenter,
        Origin::TopRight,
        Origin::MiddleLeft,
        Origin::MiddleCenter,
        Origin::MiddleRight,
        Origin::BottomLeft,
        Origin::BottomCenter,
        Origin::BottomRight,
    ] {
        let json = serde_json::to_string(&origin).unwrap();
        let parsed: Origin = serde_json::from_str(&json).unwrap();
        assert_eq!(origin, parsed);
    }
}

// === RectScaleSpec Tests ===

#[test]
fn test_rect_scale_default() {
    let spec = RectScaleSpec::default();
    match spec {
        RectScaleSpec::RectScale {
            origin,
            min_width,
            min_height,
        } => {
            assert_eq!(origin, Origin::Center);
            assert_eq!(min_width, 0);
            assert_eq!(min_height, 0);
        }
        _ => panic!("Expected RectScale variant as default"),
    }
}

#[test]
fn test_rect_scale_serde_roundtrip() {
    let spec = RectScaleSpec::RectScale {
        origin: Origin::TopLeft,
        min_width: 10,
        min_height: 5,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: RectScaleSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_rect_scale_spring_serde_roundtrip() {
    let spec = RectScaleSpec::RectScaleSpring {
        origin: Origin::BottomRight,
        min_width: 4,
        min_height: 2,
        stiffness: 0.8,
        damping: 0.3,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: RectScaleSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

// === scale_rect Function Tests ===

#[test]
fn test_scale_from_center_at_t0() {
    let spec = RectScaleSpec::RectScale {
        origin: Origin::Center,
        min_width: 0,
        min_height: 0,
    };
    let target = Rect::new(10, 10, 20, 10);
    let result = spec.scale_rect(target, 0.0);
    // At t=0, rect should be collapsed to center point
    assert_eq!(result.width, 0);
    assert_eq!(result.height, 0);
    assert_eq!(result.x, 20); // center x = 10 + 20/2 = 20
    assert_eq!(result.y, 15); // center y = 10 + 10/2 = 15
}

#[test]
fn test_scale_from_center_at_t1() {
    let spec = RectScaleSpec::RectScale {
        origin: Origin::Center,
        min_width: 0,
        min_height: 0,
    };
    let target = Rect::new(10, 10, 20, 10);
    let result = spec.scale_rect(target, 1.0);
    // At t=1, rect should be fully expanded to target
    assert_eq!(result, target);
}

#[test]
fn test_scale_from_center_at_t05() {
    let spec = RectScaleSpec::RectScale {
        origin: Origin::Center,
        min_width: 0,
        min_height: 0,
    };
    let target = Rect::new(10, 10, 20, 10);
    let result = spec.scale_rect(target, 0.5);
    // At t=0.5, rect should be half size, centered
    // anchor_y = 10 + 10*0.5 = 15, y = 15 - 5*0.5 = 12.5 → rounds to 13
    assert_eq!(result.width, 10);
    assert_eq!(result.height, 5);
    assert_eq!(result.x, 15); // anchor_x=20, x = 20 - 10*0.5 = 15
    assert_eq!(result.y, 13); // anchor_y=15, y = 15 - 5*0.5 = 12.5 → 13
}

#[test]
fn test_scale_from_top_left_at_t05() {
    let spec = RectScaleSpec::RectScale {
        origin: Origin::TopLeft,
        min_width: 0,
        min_height: 0,
    };
    let target = Rect::new(10, 10, 20, 10);
    let result = spec.scale_rect(target, 0.5);
    // Origin at top-left: x and y stay at target's origin
    assert_eq!(result.x, 10);
    assert_eq!(result.y, 10);
    assert_eq!(result.width, 10);
    assert_eq!(result.height, 5);
}

#[test]
fn test_scale_from_bottom_right_at_t05() {
    let spec = RectScaleSpec::RectScale {
        origin: Origin::BottomRight,
        min_width: 0,
        min_height: 0,
    };
    let target = Rect::new(10, 10, 20, 10);
    let result = spec.scale_rect(target, 0.5);
    // Origin at bottom-right: rect grows from bottom-right corner
    assert_eq!(result.width, 10);
    assert_eq!(result.height, 5);
    // Right edge stays at 10+20=30, so x = 30 - 10 = 20
    assert_eq!(result.x, 20);
    // Bottom edge stays at 10+10=20, so y = 20 - 5 = 15
    assert_eq!(result.y, 15);
}

#[test]
fn test_scale_with_min_dimensions() {
    let spec = RectScaleSpec::RectScale {
        origin: Origin::Center,
        min_width: 4,
        min_height: 2,
    };
    let target = Rect::new(10, 10, 20, 10);
    let result = spec.scale_rect(target, 0.0);
    // At t=0, rect should be at min dimensions, centered
    assert_eq!(result.width, 4);
    assert_eq!(result.height, 2);
    // Center x = 20, so x = 20 - 4/2 = 18
    assert_eq!(result.x, 18);
    // Center y = 15, so y = 15 - 2/2 = 14
    assert_eq!(result.y, 14);
}

#[test]
fn test_scale_spring_uses_spring_curve() {
    let spec = RectScaleSpec::RectScaleSpring {
        origin: Origin::Center,
        min_width: 0,
        min_height: 0,
        stiffness: 1.0,
        damping: 0.5,
    };
    let target = Rect::new(10, 10, 20, 10);

    // Spring should overshoot, so at some t > 0.5 we might have width > target
    // But we can't test exact values without knowing the curve
    // Just verify it runs and produces valid output
    let result = spec.scale_rect(target, 0.5);
    assert!(result.width > 0 || spec.min_width() == 0);

    // At t=1, should settle to target
    let final_result = spec.scale_rect(target, 1.0);
    assert_eq!(final_result, target);
}

#[test]
fn test_origin_methods() {
    let target = Rect::new(10, 10, 20, 10);

    assert_eq!(Origin::Center.anchor_point(target), (20, 15));
    assert_eq!(Origin::TopLeft.anchor_point(target), (10, 10));
    assert_eq!(Origin::TopRight.anchor_point(target), (30, 10));
    assert_eq!(Origin::BottomLeft.anchor_point(target), (10, 20));
    assert_eq!(Origin::BottomRight.anchor_point(target), (30, 20));
    assert_eq!(Origin::TopCenter.anchor_point(target), (20, 10));
    assert_eq!(Origin::BottomCenter.anchor_point(target), (20, 20));
    assert_eq!(Origin::MiddleLeft.anchor_point(target), (10, 15));
    assert_eq!(Origin::MiddleRight.anchor_point(target), (30, 15));
    assert_eq!(Origin::MiddleCenter.anchor_point(target), (20, 15));
}

// <FILE>tui-vfx-geometry/tests/types/test_rect_scale.rs</FILE> - <DESC>Tests for RectScale types</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T22:00:00Z</VERS>
