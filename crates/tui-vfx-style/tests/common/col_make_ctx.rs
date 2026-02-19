// <FILE>tui-vfx-style/tests/common/col_make_ctx.rs</FILE> - <DESC>Create ShaderContext for integration tests</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Allow time parameter in ShaderContext helper</CLOG>

use tui_vfx_style::traits::ShaderContext;

pub(crate) fn make_ctx(x: u16, y: u16, width: u16, height: u16, t: f64) -> ShaderContext {
    ShaderContext {
        local_x: x,
        local_y: y,
        width,
        height,
        screen_x: 0,
        screen_y: 0,
        t,
        phase: None,
    }
}

// <FILE>tui-vfx-style/tests/common/col_make_ctx.rs</FILE> - <DESC>Create ShaderContext for integration tests</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
