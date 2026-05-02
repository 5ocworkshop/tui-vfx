// <FILE>crates/tui-vfx-compost/src/primitive/fnc_install_v31_primitive_pack.rs</FILE> - <DESC>Register Rust-owned v3.1 primitive declarations</DESC>
// <VERS>VERSION: 0.16.0</VERS>
// <WCTX>Phase 1 registry installation collects domain-directory primitive ports; codegen will later derive descriptor artifacts from this Rust SSOT.</WCTX>
// <CLOG>0.16.0: MINOR — install sampler.crtJitter alongside existing primitive ports.
// 0.15.0: MINOR — install sampler.radialTwist alongside existing primitive ports.
// 0.14.0: MINOR — install sampler.shredder alongside existing primitive ports.
// 0.13.0: MINOR — install sampler.ripple alongside existing primitive ports.
// 0.12.0: MINOR — install sampler.faultLine alongside existing primitive ports.
// 0.11.0: MINOR — install sampler.distortion alongside existing primitive ports.
// 0.10.0: MINOR — install sampler.sineWave alongside existing primitive ports.
// 0.9.0: MINOR — install sampler.pendulum alongside existing primitive ports.
// 0.8.0: MINOR — install sampler.bounce alongside existing primitive ports.
// 0.7.0: MINOR — install mask.checkers alongside existing primitive ports.
// 0.6.0: MINOR — install filter.tint alongside existing primitive ports.
// 0.5.0: MINOR — install filter.greyscale alongside existing primitive ports.
// 0.4.0: MINOR — install filter.invert alongside the existing primitive ports.
// 0.3.0: MINOR — install sampler.gravity alongside filter.dim and mask.dissolve.
// 0.2.0: MINOR — install mask.dissolve alongside filter.dim.
// 0.1.0: INIT — install the first Rust-owned primitive, filter.dim.</CLOG>

use crate::filters::{FilterDim, FilterGreyscale, FilterInvert, FilterTint};
use crate::masks::{MaskCheckers, MaskDissolve};
use crate::samplers::{
    SamplerBounce, SamplerCrtJitter, SamplerDistortion, SamplerFaultLine, SamplerGravity,
    SamplerPendulum, SamplerRadialTwist, SamplerRipple, SamplerShredder, SamplerSineWave,
};

use super::{EffectRegistry, PrimitiveRegistryError};

/// Install Rust-owned v3.1 primitive descriptors and runtimes into the registry.
pub fn install_v31_primitive_pack(
    registry: &mut EffectRegistry,
) -> Result<(), PrimitiveRegistryError> {
    registry.install_frame_filter::<FilterDim>()?;
    registry.install_frame_filter::<FilterGreyscale>()?;
    registry.install_frame_filter::<FilterInvert>()?;
    registry.install_frame_filter::<FilterTint>()?;
    registry.install_mask::<MaskCheckers>()?;
    registry.install_mask::<MaskDissolve>()?;
    registry.install_coordinate_sampler::<SamplerBounce>()?;
    registry.install_coordinate_sampler::<SamplerCrtJitter>()?;
    registry.install_coordinate_sampler::<SamplerDistortion>()?;
    registry.install_coordinate_sampler::<SamplerFaultLine>()?;
    registry.install_coordinate_sampler::<SamplerGravity>()?;
    registry.install_coordinate_sampler::<SamplerPendulum>()?;
    registry.install_coordinate_sampler::<SamplerRadialTwist>()?;
    registry.install_coordinate_sampler::<SamplerRipple>()?;
    registry.install_coordinate_sampler::<SamplerShredder>()?;
    registry.install_coordinate_sampler::<SamplerSineWave>()?;
    Ok(())
}

// <FILE>crates/tui-vfx-compost/src/primitive/fnc_install_v31_primitive_pack.rs</FILE> - <DESC>Register Rust-owned v3.1 primitive declarations</DESC>
// <VERS>END OF VERSION: 0.16.0</VERS>
