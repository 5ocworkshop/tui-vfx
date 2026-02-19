// <FILE>tui-vfx-style/src/models/test_support/col_make_ctx.rs</FILE> - <DESC>Create ShaderContext for style tests</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Add default-time ShaderContext helper</CLOG>

use crate::traits::ShaderContext;

pub(crate) fn make_ctx(x: u16, y: u16, width: u16, height: u16) -> ShaderContext {
    ShaderContext::new(x, y, width, height, 0, 0, 0.0, None)
}

// <FILE>tui-vfx-style/src/models/test_support/col_make_ctx.rs</FILE> - <DESC>Create ShaderContext for style tests</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
