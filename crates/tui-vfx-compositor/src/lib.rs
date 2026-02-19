// <FILE>crates/tui-vfx-compositor/src/lib.rs</FILE> - <DESC>Library entry point</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Add shared utils module for powerline detection</WCTX>
// <CLOG>Add utils module with is_powerline_separator helper</CLOG>

pub mod context;
pub(crate) mod filters;
pub(crate) mod masks;
pub mod pipeline;
pub(crate) mod samplers;
pub mod traits;
pub mod types;
pub mod utils;
pub mod widgets;

// <FILE>crates/tui-vfx-compositor/src/lib.rs</FILE> - <DESC>Library entry point</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
