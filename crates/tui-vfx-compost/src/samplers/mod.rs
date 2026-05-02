// <FILE>crates/tui-vfx-compost/src/samplers/mod.rs</FILE> - <DESC>Native coordinate sampler primitive implementations</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Concrete sampler ports live beside the existing samplers README and mirror tui-vfx-compositor/src/samplers while exposing v3.1 primitive descriptors.</WCTX>
// <CLOG>0.1.0: INIT — add sampler.gravity native primitive port.</CLOG>

mod cls_gravity;
mod cls_sampler_axis;

pub use cls_gravity::{SamplerGravity, SamplerGravityInputs};
pub use cls_sampler_axis::SamplerAxis;

// <FILE>crates/tui-vfx-compost/src/samplers/mod.rs</FILE> - <DESC>Native coordinate sampler primitive implementations</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
