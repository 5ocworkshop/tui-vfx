// <FILE>tui-vfx-style/src/lib.rs</FILE> - <DESC>Library entry point</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>TTE effects port — declare schedules module as a sibling to models/traits/utils so per-cell trigger schedule generators (TTE Beams cadence, future eased wavefronts) cluster in a discoverable category.</WCTX>
// <CLOG>0.4.0: add `pub mod schedules;` — new home for per-cell trigger-time schedule generators. Composes mixed-signals primitives.</CLOG>

pub mod models;
pub mod schedules;
pub mod traits;
pub mod utils;

// <FILE>tui-vfx-style/src/lib.rs</FILE>
// <VERS>END OF VERSION: 0.4.0</VERS>
