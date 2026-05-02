// <FILE>crates/tui-vfx-compost/src/primitive/tr_source_primitive.rs</FILE> - <DESC>Rust-owned source primitive descriptor trait</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sources sit beside effects in the generated primitive descriptor pack but have their own SourceDescriptor contract and materialization runtime.</WCTX>
// <CLOG>0.1.0: INIT — add SourcePrimitive descriptor trait.</CLOG>

use tui_vfx_contract::SourceDescriptor;

/// Rust source-of-truth declaration for one v3.1 source descriptor.
pub trait SourcePrimitive {
    /// Runtime input bundle for this source.
    type Inputs;

    /// Build the canonical v3.1 source descriptor for this primitive.
    fn descriptor() -> SourceDescriptor;
}

// <FILE>crates/tui-vfx-compost/src/primitive/tr_source_primitive.rs</FILE> - <DESC>Rust-owned source primitive descriptor trait</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
