// <FILE>crates/tui-vfx-compost/src/primitive/tr_primitive_inputs.rs</FILE> - <DESC>Effect primitive input descriptor trait</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 keeps descriptor boilerplate explicit while reserving the trait that PrimitiveInputs derive will later implement.</WCTX>
// <CLOG>0.1.0: INIT — add effect input spec trait and empty-input implementation.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{EffectInputId, EffectInputSpec};

/// Declares descriptor-local effect inputs for a Rust-owned primitive.
pub trait PrimitiveInputs {
    /// Return the v3.1 descriptor input specs keyed by canonical input id.
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec>;
}

/// Marker for primitives with no effect inputs.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NoInputs;

impl PrimitiveInputs for NoInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::new()
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/tr_primitive_inputs.rs</FILE> - <DESC>Effect primitive input descriptor trait</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
