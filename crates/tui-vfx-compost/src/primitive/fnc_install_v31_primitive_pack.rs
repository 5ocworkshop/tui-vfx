// <FILE>crates/tui-vfx-compost/src/primitive/fnc_install_v31_primitive_pack.rs</FILE> - <DESC>Register Rust-owned v3.1 primitive declarations</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 introduces the registry installation entrypoint; bootstrap carry-forward will merge unported descriptors during codegen.</WCTX>
// <CLOG>0.1.0: INIT — install the first Rust-owned primitive, filter.dim.</CLOG>

use super::{EffectRegistry, FilterDim, PrimitiveRegistryError};

/// Install Rust-owned v3.1 primitive descriptors and runtimes into the registry.
pub fn install_v31_primitive_pack(
    registry: &mut EffectRegistry,
) -> Result<(), PrimitiveRegistryError> {
    registry.install_frame_filter::<FilterDim>()
}

// <FILE>crates/tui-vfx-compost/src/primitive/fnc_install_v31_primitive_pack.rs</FILE> - <DESC>Register Rust-owned v3.1 primitive declarations</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
