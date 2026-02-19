// <FILE>tui-vfx-geometry/tests/test_position_spec.rs</FILE>
// <DESC>PositionSpec frame-relative resolution</DESC>

use tui_vfx_geometry::types::{Position, PositionSpec};
use tui_vfx_types::Rect;

#[test]
fn frame_permille_resolves_inside_frame() {
    let frame = Rect::new(10, 20, 100, 50);
    let p = PositionSpec::FramePermille {
        x_permille: 150,
        y_permille: 500,
    }
    .resolve_in_frame(frame);
    assert_eq!(p, Position::new(25, 45)); // 10 + 15, 20 + 25
}
