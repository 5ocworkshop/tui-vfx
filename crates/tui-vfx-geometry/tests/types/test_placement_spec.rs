// <FILE>tui-vfx-geometry/tests/types/test_placement_spec.rs</FILE> - <DESC>Tests for PlacementSpec</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-23</VERS>
// <WCTX>Restoring missing effects for recipe compatibility</WCTX>
// <CLOG>Updated Anchor variant to struct form for JSON compatibility</CLOG>

use tui_vfx_geometry::types::{Anchor, PlacementSpec, Position, SlideDirection};
use tui_vfx_types::Rect;

fn frame() -> Rect {
    Rect::new(0, 0, 100, 50)
}

// --- Absolute tests ---

#[test]
fn test_absolute_returns_exact_position() {
    let spec = PlacementSpec::Absolute(Position::new(42, 17));
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(42, 17));
}

#[test]
fn test_absolute_can_be_negative() {
    let spec = PlacementSpec::Absolute(Position::new(-10, -5));
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(-10, -5));
}

// --- FramePermille tests ---

#[test]
fn test_frame_permille_center() {
    let spec = PlacementSpec::FramePermille {
        x_permille: 500,
        y_permille: 500,
    };
    let result = spec.resolve(frame(), None);
    // 100 * 500 / 1000 = 50, 50 * 500 / 1000 = 25
    assert_eq!(result, Position::new(50, 25));
}

#[test]
fn test_frame_permille_top_left() {
    let spec = PlacementSpec::FramePermille {
        x_permille: 0,
        y_permille: 0,
    };
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(0, 0));
}

#[test]
fn test_frame_permille_bottom_right() {
    let spec = PlacementSpec::FramePermille {
        x_permille: 1000,
        y_permille: 1000,
    };
    let result = spec.resolve(frame(), None);
    // Should clamp to last valid cell (99, 49)
    assert_eq!(result, Position::new(99, 49));
}

#[test]
fn test_frame_permille_mid_left() {
    let spec = PlacementSpec::FramePermille {
        x_permille: 0,
        y_permille: 500,
    };
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(0, 25));
}

#[test]
fn test_frame_permille_mid_right() {
    let spec = PlacementSpec::FramePermille {
        x_permille: 1000,
        y_permille: 500,
    };
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(99, 25));
}

// --- Anchor tests ---

#[test]
fn test_anchor_center() {
    let spec = PlacementSpec::Anchor {
        anchor: Anchor::Center,
    };
    let result = spec.resolve(frame(), None);
    // Center of 100x50 is (50, 25)
    assert_eq!(result, Position::new(50, 25));
}

#[test]
fn test_anchor_top_left() {
    let spec = PlacementSpec::Anchor {
        anchor: Anchor::TopLeft,
    };
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(0, 0));
}

#[test]
fn test_anchor_bottom_right() {
    let spec = PlacementSpec::Anchor {
        anchor: Anchor::BottomRight,
    };
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(99, 49));
}

#[test]
fn test_anchor_middle_left() {
    let spec = PlacementSpec::Anchor {
        anchor: Anchor::MiddleLeft,
    };
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(0, 25));
}

#[test]
fn test_anchor_middle_right() {
    let spec = PlacementSpec::Anchor {
        anchor: Anchor::MiddleRight,
    };
    let result = spec.resolve(frame(), None);
    assert_eq!(result, Position::new(99, 25));
}

// --- Offscreen tests ---

#[test]
fn test_offscreen_from_left() {
    let spec = PlacementSpec::Offscreen {
        direction: SlideDirection::FromLeft,
        margin_cells: 1,
    };
    // Widget rect at center, width 10, height 5
    let widget_rect = Some(Rect::new(45, 22, 10, 5));
    let result = spec.resolve(frame(), widget_rect);
    // Should be 1 cell to the left of frame (x = -10 - 1 = -11)
    assert_eq!(result.x, -11);
}

#[test]
fn test_offscreen_from_right() {
    let spec = PlacementSpec::Offscreen {
        direction: SlideDirection::FromRight,
        margin_cells: 2,
    };
    let widget_rect = Some(Rect::new(45, 22, 10, 5));
    let result = spec.resolve(frame(), widget_rect);
    // Should be 2 cells to the right of frame (x = 100 + 2 = 102)
    assert_eq!(result.x, 102);
}

#[test]
fn test_offscreen_from_top() {
    let spec = PlacementSpec::Offscreen {
        direction: SlideDirection::FromTop,
        margin_cells: 1,
    };
    let widget_rect = Some(Rect::new(45, 22, 10, 5));
    let result = spec.resolve(frame(), widget_rect);
    // Should be 1 cell above frame (y = -5 - 1 = -6)
    assert_eq!(result.y, -6);
}

#[test]
fn test_offscreen_from_bottom() {
    let spec = PlacementSpec::Offscreen {
        direction: SlideDirection::FromBottom,
        margin_cells: 0,
    };
    let widget_rect = Some(Rect::new(45, 22, 10, 5));
    let result = spec.resolve(frame(), widget_rect);
    // Should be at bottom of frame (y = 50)
    assert_eq!(result.y, 50);
}

// --- Serde tests ---

#[test]
fn test_serde_absolute() {
    let spec = PlacementSpec::Absolute(Position::new(10, 20));
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: PlacementSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_serde_frame_permille() {
    let spec = PlacementSpec::FramePermille {
        x_permille: 250,
        y_permille: 750,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: PlacementSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_serde_anchor() {
    let spec = PlacementSpec::Anchor {
        anchor: Anchor::BottomCenter,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: PlacementSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_serde_offscreen() {
    let spec = PlacementSpec::Offscreen {
        direction: SlideDirection::FromTopRight,
        margin_cells: 3,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: PlacementSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

// <FILE>tui-vfx-geometry/tests/types/test_placement_spec.rs</FILE> - <DESC>Tests for PlacementSpec</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-23</VERS>
