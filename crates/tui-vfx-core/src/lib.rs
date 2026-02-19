// <FILE>tui-vfx-core/src/lib.rs</FILE> - <DESC>Crate root for introspection primitives</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-25</VERS>
// <WCTX>Mixed-signals migration Phase 5</WCTX>
// <CLOG>Added ConfigSchema implementations for mixed_signals types</CLOG>

pub mod mixed_signals_schema;
pub mod schema;
pub mod time_spec;

pub use schema::{
    ConfigSchema, FieldMeta, Range, ScalarValue, SchemaField, SchemaNode, SchemaVariant,
};

pub use time_spec::TimeSpec;

// Allow `#[derive(ConfigSchema)]` to resolve via this crate.
pub use tui_vfx_core_macros::ConfigSchema;

// <FILE>tui-vfx-core/src/lib.rs</FILE> - <DESC>Crate root for introspection primitives</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-25</VERS>
