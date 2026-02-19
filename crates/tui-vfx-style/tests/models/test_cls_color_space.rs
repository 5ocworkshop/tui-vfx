// <FILE>tui-vfx-style/tests/models/test_cls_color_space.rs</FILE> - <DESC>Tests for ColorSpace</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-16T20:53:16Z</VERS>
// <WCTX>Turn 3 Implementation</WCTX>
// <CLOG>Initial test</CLOG>

use tui_vfx_style::models::ColorSpace;
#[test]
fn test_default_is_rgb() {
    assert_eq!(ColorSpace::default(), ColorSpace::Rgb);
}

// <FILE>tui-vfx-style/tests/models/test_cls_color_space.rs</FILE> - <DESC>Tests for ColorSpace</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-16T20:53:16Z</VERS>
