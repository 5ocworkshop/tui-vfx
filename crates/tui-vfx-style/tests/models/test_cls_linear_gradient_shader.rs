// <FILE>tui-vfx-style/tests/models/test_cls_linear_gradient_shader.rs</FILE> - <DESC>Tests for LinearGradientShader</DESC>
// <VERS>VERSION: 0.2.2 - 2025-12-29</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Use shared ShaderContext helper</CLOG>
use crate::common::make_ctx;

use tui_vfx_style::models::{Gradient, LinearGradientShader};
use tui_vfx_style::traits::StyleShader;
use tui_vfx_types::{Color, Style};
#[test]
fn test_horizontal_gradient() {
    let gradient = Gradient::new(vec![(0.0, Color::BLACK), (1.0, Color::WHITE)]);
    let shader = LinearGradientShader::new(gradient);
    let base = Style::default();
    // Width 10, Height 1
    // Left edge (x=0) -> Black
    let s1 = shader.style_at(&make_ctx(0, 0, 10, 1, 0.0), base);
    assert_eq!(s1.fg, Color::BLACK);
    // Right edge (x=9) -> White
    let s2 = shader.style_at(&make_ctx(9, 0, 10, 1, 0.0), base);
    assert_eq!(s2.fg, Color::WHITE);
    // Middle approx (x=4) -> ~Gray
    let s3 = shader.style_at(&make_ctx(4, 0, 10, 1, 0.0), base);
    // 4/9 = 0.444
    // Black(0) to White(255) * 0.444 = 113
    let fg = s3.fg;
    assert!(
        fg.r > 100 && fg.r < 120,
        "Expected r in 100-120, got {}",
        fg.r
    );
    assert_eq!(fg.r, fg.g);
    assert_eq!(fg.g, fg.b);
}

// <FILE>tui-vfx-style/tests/models/test_cls_linear_gradient_shader.rs</FILE> - <DESC>Tests for LinearGradientShader</DESC>
// <VERS>END OF VERSION: 0.2.2 - 2025-12-29</VERS>
