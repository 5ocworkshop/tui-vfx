// <FILE>crates/tui-vfx-compost/src/primitive/tr_effect_primitive.rs</FILE> - <DESC>Rust-owned effect primitive descriptor trait</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>EffectPrimitive owns the primitive catalog instance descriptor while tui-vfx-contract continues to own the DTO/schema shape.</WCTX>
// <CLOG>0.1.0: INIT — add descriptor trait with explicit input and output associated types.</CLOG>

use tui_vfx_contract::EffectDescriptor;

use super::{PrimitiveInputs, PrimitiveOutputs};

/// Rust source-of-truth declaration for one v3.1 effect descriptor.
pub trait EffectPrimitive {
    /// Effect input declaration type.
    type Inputs: PrimitiveInputs;
    /// Effect output declaration type.
    type Outputs: PrimitiveOutputs;

    /// Build the canonical v3.1 effect descriptor for this primitive.
    fn descriptor() -> EffectDescriptor;
}

// <FILE>crates/tui-vfx-compost/src/primitive/tr_effect_primitive.rs</FILE> - <DESC>Rust-owned effect primitive descriptor trait</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
