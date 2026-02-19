// <FILE>tui-vfx-style/tests/models/test_cls_gradient.rs</FILE> - <DESC>Unit tests for Gradient</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-16T21:01:47Z</VERS>
// <WCTX>Turn 7 Audit Resolution</WCTX>
// <CLOG>Initial tests</CLOG>

use tui_vfx_style::models::Gradient;
use tui_vfx_types::Color;
#[test]
fn test_gradient_sampling() {
    let g = Gradient::new(vec![
        (0.0, Color::BLACK),
        (1.0, Color::WHITE), // (255,255,255)
    ]);
    // t=0.0 -> Black
    assert_eq!(g.sample(0.0), Color::BLACK);
    // t=1.0 -> White
    assert_eq!(g.sample(1.0), Color::WHITE);
    // t=0.5 -> Gray (127,127,127)
    assert_eq!(g.sample(0.5), Color::rgb(127, 127, 127));
}
#[test]
fn test_gradient_clamping() {
    let g = Gradient::new(vec![(0.0, Color::BLACK), (1.0, Color::WHITE)]);
    // t=-0.5 -> Black
    assert_eq!(g.sample(-0.5), Color::BLACK);
    // t=1.5 -> White
    assert_eq!(g.sample(1.5), Color::WHITE);
}

// <FILE>tui-vfx-style/tests/models/test_cls_gradient.rs</FILE> - <DESC>Unit tests for Gradient</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-16T21:01:47Z</VERS>
