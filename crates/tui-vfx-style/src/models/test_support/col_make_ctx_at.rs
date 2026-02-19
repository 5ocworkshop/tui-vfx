// <FILE>tui-vfx-style/src/models/test_support/col_make_ctx_at.rs</FILE> - <DESC>Create ShaderContext with explicit time</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Add time-aware ShaderContext helper</CLOG>

use crate::traits::ShaderContext;

pub(crate) fn make_ctx_at(x: u16, y: u16, width: u16, height: u16, t: f64) -> ShaderContext {
    ShaderContext::new(x, y, width, height, 0, 0, t, None)
}

// <FILE>tui-vfx-style/src/models/test_support/col_make_ctx_at.rs</FILE> - <DESC>Create ShaderContext with explicit time</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
