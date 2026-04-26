// <FILE>crates/tui-vfx-core/src/bindable/mod.rs</FILE> - <DESC>Module root for the generic VfxBindable<T, S> envelope and the three concrete type aliases (VfxBindableU16, VfxBindableString, VfxBindableValue) downstream consumers depend on.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.A — generalize three parallel hand-rolled Bindable* types into one VfxBindable<T, S>; bundles 1.7.A by hosting BindableValue here so both tui-vfx-style and tui-vfx-compositor (which already depend on tui-vfx-core) consume the same canonical type.</WCTX>
// <CLOG>0.1.0: introduce VfxBindable<T, S = Infallible>, RuntimeParamsRead trait, BindableSignal helper, type aliases.</CLOG>

//! # VfxBindable
//!
//! Generic `Literal | Binding | Signal` envelope for runtime-bound recipe
//! values. Three concrete instantiations are aliased here:
//!
//! - [`VfxBindableU16`] — `VfxBindable<u16>` for cell coordinates.
//! - [`VfxBindableString`] — `VfxBindable<String>` for asset / font / locale
//!   names.
//! - [`VfxBindableValue`] — `VfxBindable<f32, SignalOrFloat>` for filter
//!   parameters that may be literal, runtime-bound, or signal-driven.
//!
//! Non-signal Bindables default `S` to [`std::convert::Infallible`], which
//! makes the `Signal` arm provably unconstructable: `VfxBindableU16::Signal`
//! cannot be matched at runtime because `Infallible` has no inhabitants.
//!
//! See [`VfxBindable`] for the type definition and inherent impls.

mod cls_bindable;

#[cfg(test)]
mod test_cls_bindable;

pub use cls_bindable::{
    BindableSignal, Never, RuntimeParamsRead, VfxBindable, VfxBindableString, VfxBindableU16,
    VfxBindableValue,
};

// <FILE>crates/tui-vfx-core/src/bindable/mod.rs</FILE> - <DESC>Module root for the generic VfxBindable<T, S> envelope and the three concrete type aliases (VfxBindableU16, VfxBindableString, VfxBindableValue) downstream consumers depend on.</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
