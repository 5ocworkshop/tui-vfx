// <FILE>crates/tui-vfx-compost/src/filters/mod.rs</FILE> - <DESC>Frame filter primitive declarations and runtimes</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase 1 starts descriptor/runtime porting with filter.dim as the smallest frame-filter primitive.</WCTX>
// <CLOG>0.3.0: MINOR — add filter.greyscale primitive export.
// 0.2.0: MINOR — add filter.invert primitive export.
// 0.1.0: INIT — add filter.dim primitive export.</CLOG>

mod cls_channel_target;
mod cls_dim;
mod cls_greyscale;
mod cls_invert;
mod fnc_dim_color;

pub use cls_channel_target::ChannelTarget;
pub use cls_dim::{FilterDim, FilterDimInputs};
pub use cls_greyscale::{FilterGreyscale, FilterGreyscaleInputs, greyscale_luminance};
pub use cls_invert::{FilterInvert, FilterInvertInputs};
pub use fnc_dim_color::dim_color;

// <FILE>crates/tui-vfx-compost/src/filters/mod.rs</FILE> - <DESC>Frame filter primitive declarations and runtimes</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
