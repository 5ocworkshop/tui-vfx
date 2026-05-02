// <FILE>crates/tui-vfx-compost/src/primitive/tr_primitive_outputs.rs</FILE> - <DESC>Effect primitive output descriptor trait</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>PrimitiveOutputs is a placeholder for future graph-output capture; Phase 0/1 support empty outputs only.</WCTX>
// <CLOG>0.1.0: INIT — add output spec trait implemented by NoOutputs.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{EffectOutputId, EffectOutputSpec};

/// Declares descriptor-local graph outputs published by an effect primitive.
pub trait PrimitiveOutputs {
    /// Return output specs keyed by canonical output id.
    fn output_specs() -> BTreeMap<EffectOutputId, EffectOutputSpec>;
}

// <FILE>crates/tui-vfx-compost/src/primitive/tr_primitive_outputs.rs</FILE> - <DESC>Effect primitive output descriptor trait</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
