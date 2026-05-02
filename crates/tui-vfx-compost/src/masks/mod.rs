// <FILE>crates/tui-vfx-compost/src/masks/mod.rs</FILE> - <DESC>Native mask primitive implementations</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Concrete mask ports live beside the existing masks README and mirror tui-vfx-compositor/src/masks while exposing v3.1 primitive descriptors.</WCTX>
// <CLOG>0.1.0: INIT — add mask.dissolve native primitive port.</CLOG>

mod cls_dissolve;

pub use cls_dissolve::{MaskDissolve, MaskDissolveInputs};

// <FILE>crates/tui-vfx-compost/src/masks/mod.rs</FILE> - <DESC>Native mask primitive implementations</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
