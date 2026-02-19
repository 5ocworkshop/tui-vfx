// <FILE>tui-vfx-style/tests/models/test_cls_easing_type.rs</FILE> - <DESC>Tests for EasingType</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-16T20:53:16Z</VERS>
// <WCTX>Turn 3 Implementation</WCTX>
// <CLOG>Initial test</CLOG>

use tui_vfx_style::models::EasingType;
#[test]
fn test_default_is_linear() {
    assert_eq!(EasingType::default(), EasingType::Linear);
}

// <FILE>tui-vfx-style/tests/models/test_cls_easing_type.rs</FILE> - <DESC>Tests for EasingType</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-16T20:53:16Z</VERS>
