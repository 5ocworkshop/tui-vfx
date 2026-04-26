// <FILE>crates/tui-vfx-core/src/lib.rs</FILE> - <DESC>Crate root for introspection primitives, schema-bridge types, and the canonical VfxBindable envelope.</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.A — host the consolidated VfxBindable<T, S> here so tui-vfx-style and tui-vfx-compositor (both already core dependents) consume one canonical type. Bundles 1.7.A by giving BindableValue a cross-crate home.</WCTX>
// <CLOG>0.3.0: introduce the bindable module (VfxBindable, RuntimeParamsRead, BindableSignal, three type aliases) and re-export its surface from the crate root.</CLOG>

pub mod bindable;
pub mod mixed_signals_schema;
pub mod schema;
pub mod time_spec;

pub use bindable::{
    BindableSignal, RuntimeParamsRead, VfxBindable, VfxBindableString, VfxBindableU16,
    VfxBindableValue,
};

pub use schema::{
    ConfigSchema, FieldMeta, Range, ScalarValue, SchemaField, SchemaNode, SchemaVariant,
};

pub use time_spec::TimeSpec;

// Allow `#[derive(ConfigSchema)]` to resolve via this crate.
pub use tui_vfx_core_macros::ConfigSchema;

// <FILE>crates/tui-vfx-core/src/lib.rs</FILE> - <DESC>Crate root for introspection primitives, schema-bridge types, and the canonical VfxBindable envelope.</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
