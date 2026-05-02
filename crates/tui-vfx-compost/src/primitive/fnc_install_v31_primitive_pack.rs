// <FILE>crates/tui-vfx-compost/src/primitive/fnc_install_v31_primitive_pack.rs</FILE> - <DESC>Register Rust-owned v3.1 primitive declarations</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase 1 registry installation collects domain-directory primitive ports; codegen will later derive descriptor artifacts from this Rust SSOT.</WCTX>
// <CLOG>0.3.0: MINOR — install sampler.gravity alongside filter.dim and mask.dissolve.
// 0.2.0: MINOR — install mask.dissolve alongside filter.dim.
// 0.1.0: INIT — install the first Rust-owned primitive, filter.dim.</CLOG>

use crate::filters::FilterDim;
use crate::masks::MaskDissolve;
use crate::samplers::SamplerGravity;

use super::{EffectRegistry, PrimitiveRegistryError};

/// Install Rust-owned v3.1 primitive descriptors and runtimes into the registry.
pub fn install_v31_primitive_pack(
    registry: &mut EffectRegistry,
) -> Result<(), PrimitiveRegistryError> {
    registry.install_frame_filter::<FilterDim>()?;
    registry.install_mask::<MaskDissolve>()?;
    registry.install_coordinate_sampler::<SamplerGravity>()?;
    Ok(())
}

// <FILE>crates/tui-vfx-compost/src/primitive/fnc_install_v31_primitive_pack.rs</FILE> - <DESC>Register Rust-owned v3.1 primitive declarations</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
