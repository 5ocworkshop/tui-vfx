// <FILE>crates/tui-vfx-compost/src/samplers/mod.rs</FILE> - <DESC>Native coordinate sampler primitive implementations</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Concrete sampler ports live beside the existing samplers README and mirror tui-vfx-compositor/src/samplers while exposing v3.1 primitive descriptors.</WCTX>
// <CLOG>0.5.0: MINOR — add sampler.distortion native primitive port.
// 0.4.0: MINOR — add sampler.sineWave native primitive port.
// 0.3.0: MINOR — add sampler.pendulum native primitive port.
// 0.2.0: MINOR — add sampler.bounce native primitive port.
// 0.1.0: INIT — add sampler.gravity native primitive port.</CLOG>

mod cls_bounce;
mod cls_distortion;
mod cls_gravity;
mod cls_pendulum;
mod cls_sampler_axis;
mod cls_sine_wave;

pub use cls_bounce::{SamplerBounce, SamplerBounceInputs};
pub use cls_distortion::{SamplerDistortion, SamplerDistortionInputs};
pub use cls_gravity::{SamplerGravity, SamplerGravityInputs};
pub use cls_pendulum::{SamplerPendulum, SamplerPendulumInputs};
pub use cls_sampler_axis::SamplerAxis;
pub use cls_sine_wave::{SamplerSineWave, SamplerSineWaveInputs};

// <FILE>crates/tui-vfx-compost/src/samplers/mod.rs</FILE> - <DESC>Native coordinate sampler primitive implementations</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
