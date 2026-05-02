// <FILE>crates/tui-vfx-compost/src/primitive/cls_no_outputs.rs</FILE> - <DESC>Empty primitive output marker</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 ports do not publish graph outputs; this marker keeps that decision explicit in primitive types.</WCTX>
// <CLOG>0.1.0: INIT — add NoOutputs implementation for PrimitiveOutputs.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{EffectOutputId, EffectOutputSpec};

use super::PrimitiveOutputs;

/// Marker used by primitives that publish no graph outputs.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NoOutputs;

impl PrimitiveOutputs for NoOutputs {
    fn output_specs() -> BTreeMap<EffectOutputId, EffectOutputSpec> {
        BTreeMap::new()
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_no_outputs.rs</FILE> - <DESC>Empty primitive output marker</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
