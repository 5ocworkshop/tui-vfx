// <FILE>tui-vfx-style/tests/common/mod.rs</FILE> - <DESC>Shared helpers for style integration tests</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Clean up test warnings</WCTX>
// <CLOG>Allow unused shared integration test helpers</CLOG>

mod col_make_ctx;
mod col_make_style;

pub(crate) use col_make_ctx::make_ctx;
#[allow(unused_imports)]
pub(crate) use col_make_style::make_style;

// <FILE>tui-vfx-style/tests/common/mod.rs</FILE> - <DESC>Shared helpers for style integration tests</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
