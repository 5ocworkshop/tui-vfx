// <FILE>tui-vfx-geometry/tests/types/test_motion_spec.rs</FILE> - <DESC>Tests for MotionSpec</DESC>
// <VERS>VERSION: 1.2.0 - 2025-12-24</VERS>
// <WCTX>Easing Curves Expansion: Fix test compilation after EasingCurve migration</WCTX>
// <CLOG>Wrapped EasingType values in EasingCurve::Type() for compatibility</CLOG>

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::{
    Anchor, EasingCurve, MotionSpec, PathType, PlacementSpec, Position, SlideDirection,
    SnappingStrategy,
};
use tui_vfx_types::Rect;

fn frame() -> Rect {
    Rect::new(0, 0, 100, 50)
}

fn widget() -> Rect {
    Rect::new(45, 22, 10, 5)
}

// --- Basic structure tests ---

#[test]
fn test_motion_spec_default() {
    let spec = MotionSpec::default();
    assert_eq!(spec.duration_ms, 500);
    assert_eq!(spec.ease, EasingCurve::Type(EasingType::Linear));
    assert_eq!(spec.path, PathType::Linear);
    assert!(spec.from.is_none());
    assert!(spec.via.is_none());
    assert!(spec.to.is_none());
}

#[test]
fn test_motion_spec_with_from() {
    let spec = MotionSpec {
        from: Some(PlacementSpec::Offscreen {
            direction: SlideDirection::FromLeft,
            margin_cells: 1,
        }),
        ..Default::default()
    };
    assert!(spec.from.is_some());
}

#[test]
fn test_motion_spec_with_via() {
    let spec = MotionSpec {
        via: Some(PlacementSpec::FramePermille {
            x_permille: 500,
            y_permille: 100,
        }),
        ..Default::default()
    };
    assert!(spec.via.is_some());
}

#[test]
fn test_motion_spec_with_to() {
    let spec = MotionSpec {
        to: Some(PlacementSpec::Anchor {
            anchor: Anchor::BottomRight,
        }),
        ..Default::default()
    };
    assert!(spec.to.is_some());
}

// --- Resolution tests ---

#[test]
fn test_resolve_from_returns_resolved_position() {
    let spec = MotionSpec {
        from: Some(PlacementSpec::FramePermille {
            x_permille: 0,
            y_permille: 500,
        }),
        ..Default::default()
    };
    let resolved = spec.resolve_from(frame(), widget());
    assert_eq!(resolved, Some(Position::new(0, 25)));
}

#[test]
fn test_resolve_from_none_returns_none() {
    let spec = MotionSpec::default();
    let resolved = spec.resolve_from(frame(), widget());
    assert!(resolved.is_none());
}

#[test]
fn test_resolve_via_returns_resolved_position() {
    let spec = MotionSpec {
        via: Some(PlacementSpec::FramePermille {
            x_permille: 500,
            y_permille: 100,
        }),
        ..Default::default()
    };
    let resolved = spec.resolve_via(frame(), widget());
    // 100 * 500 / 1000 = 50, 50 * 100 / 1000 = 5
    assert_eq!(resolved, Some(Position::new(50, 5)));
}

#[test]
fn test_resolve_to_returns_resolved_position() {
    let spec = MotionSpec {
        to: Some(PlacementSpec::Anchor {
            anchor: Anchor::BottomRight,
        }),
        ..Default::default()
    };
    let resolved = spec.resolve_to(frame(), widget());
    assert_eq!(resolved, Some(Position::new(99, 49)));
}

// --- Path resolution tests ---

#[test]
fn test_resolve_path_linear_unchanged() {
    let spec = MotionSpec {
        path: PathType::Linear,
        ..Default::default()
    };
    let resolved_path = spec.resolve_path(frame(), widget());
    assert_eq!(resolved_path, PathType::Linear);
}

#[test]
fn test_resolve_path_arc_unchanged() {
    let spec = MotionSpec {
        path: PathType::Arc { bulge: 0.5 },
        ..Default::default()
    };
    let resolved_path = spec.resolve_path(frame(), widget());
    assert_eq!(resolved_path, PathType::Arc { bulge: 0.5 });
}

#[test]
fn test_resolve_path_bezier_uses_via_when_present() {
    let spec = MotionSpec {
        path: PathType::Bezier {
            control_x: 0.0,
            control_y: 0.0,
        },
        via: Some(PlacementSpec::FramePermille {
            x_permille: 500,
            y_permille: 100,
        }),
        ..Default::default()
    };
    let resolved_path = spec.resolve_path(frame(), widget());
    // via resolves to (50, 5)
    if let PathType::Bezier {
        control_x,
        control_y,
    } = resolved_path
    {
        assert_eq!(control_x, 50.0);
        assert_eq!(control_y, 5.0);
    } else {
        panic!("Expected Bezier path type");
    }
}

#[test]
fn test_resolve_path_bezier_without_via_keeps_defaults() {
    let spec = MotionSpec {
        path: PathType::Bezier {
            control_x: 42.0,
            control_y: 17.0,
        },
        via: None,
        ..Default::default()
    };
    let resolved_path = spec.resolve_path(frame(), widget());
    if let PathType::Bezier {
        control_x,
        control_y,
    } = resolved_path
    {
        assert_eq!(control_x, 42.0);
        assert_eq!(control_y, 17.0);
    } else {
        panic!("Expected Bezier path type");
    }
}

// --- Full motion spec scenario tests ---

#[test]
fn test_mid_left_to_mid_right_via_top_center() {
    // The canonical use case: arc from mid-left to mid-right via top-center
    let spec = MotionSpec {
        duration_ms: 2000,
        ease: EasingCurve::Type(EasingType::QuadOut),
        path: PathType::Bezier {
            control_x: 0.0,
            control_y: 0.0,
        },
        snap: SnappingStrategy::Round,
        from: Some(PlacementSpec::FramePermille {
            x_permille: 0,
            y_permille: 500,
        }),
        via: Some(PlacementSpec::FramePermille {
            x_permille: 500,
            y_permille: 0,
        }),
        to: Some(PlacementSpec::FramePermille {
            x_permille: 1000,
            y_permille: 500,
        }),
    };

    let from = spec.resolve_from(frame(), widget()).unwrap();
    let via = spec.resolve_via(frame(), widget()).unwrap();
    let to = spec.resolve_to(frame(), widget()).unwrap();

    assert_eq!(from, Position::new(0, 25)); // mid-left
    assert_eq!(via, Position::new(50, 0)); // top-center
    assert_eq!(to, Position::new(99, 25)); // mid-right

    let resolved_path = spec.resolve_path(frame(), widget());
    if let PathType::Bezier {
        control_x,
        control_y,
    } = resolved_path
    {
        assert_eq!(control_x, 50.0);
        assert_eq!(control_y, 0.0);
    } else {
        panic!("Expected Bezier path type");
    }
}

#[test]
fn test_offscreen_slide_with_arc() {
    let spec = MotionSpec {
        duration_ms: 1000,
        ease: EasingCurve::Type(EasingType::QuadIn),
        path: PathType::Arc { bulge: 0.3 },
        snap: SnappingStrategy::Round,
        from: Some(PlacementSpec::Offscreen {
            direction: SlideDirection::FromRight,
            margin_cells: 2,
        }),
        via: None,
        to: Some(PlacementSpec::Anchor {
            anchor: Anchor::BottomRight,
        }),
    };

    let from = spec.resolve_from(frame(), widget()).unwrap();
    let to = spec.resolve_to(frame(), widget()).unwrap();

    // Widget at (45, 22) with size 10x5
    // FromRight with margin 2: x = 100 + 2 = 102
    assert_eq!(from.x, 102);

    // BottomRight anchor
    assert_eq!(to, Position::new(99, 49));
}

// --- Serde tests ---

#[test]
fn test_serde_motion_spec_full() {
    let spec = MotionSpec {
        duration_ms: 1500,
        ease: EasingCurve::Type(EasingType::QuadOut),
        path: PathType::Bezier {
            control_x: 50.0,
            control_y: 10.0,
        },
        snap: SnappingStrategy::Round,
        from: Some(PlacementSpec::FramePermille {
            x_permille: 0,
            y_permille: 500,
        }),
        via: Some(PlacementSpec::FramePermille {
            x_permille: 500,
            y_permille: 100,
        }),
        to: Some(PlacementSpec::Anchor {
            anchor: Anchor::MiddleRight,
        }),
    };

    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MotionSpec = serde_json::from_str(&json).unwrap();

    assert_eq!(spec.duration_ms, parsed.duration_ms);
    assert_eq!(spec.ease, parsed.ease);
    assert_eq!(spec.path, parsed.path);
    assert_eq!(spec.from, parsed.from);
    assert_eq!(spec.via, parsed.via);
    assert_eq!(spec.to, parsed.to);
}

#[test]
fn test_serde_motion_spec_minimal() {
    let spec = MotionSpec::default();
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: MotionSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

// <FILE>tui-vfx-geometry/tests/types/test_motion_spec.rs</FILE> - <DESC>Tests for MotionSpec</DESC>
// <VERS>END OF VERSION: 1.2.0 - 2025-12-24</VERS>
