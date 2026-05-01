// <FILE>crates/tui-vfx-compositor-next/src/types/cls_bindable_value.rs</FILE> - <DESC>Re-export of tui_vfx_core::bindable::VfxBindableValue under the historical `BindableValue` name. Preserves consumer import paths during the sweep 1.2.A consolidation; resolves sweep 1.7.A by moving the cross-crate home to tui-vfx-core.</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.A + 1.7.A — collapse three parallel hand-rolled Bindable types into one VfxBindable<T, S> in tui-vfx-core, and migrate BindableValue's home from tui-vfx-compositor-next to tui-vfx-core (where both downstream consumers already depend). The hand-rolled body retired to recyclebin/ in this revision.</WCTX>
// <CLOG>0.3.0: BindableValue is now an alias for tui_vfx_core::bindable::VfxBindableValue (= VfxBindable<f32, SignalOrFloat>). Inherent evaluate / static_f32 / Default / From<f32|SignalOrFloat> / serde / ConfigSchema all surface through the generic. Wire format is now: bare number → Literal, {"literal": N} → Literal, {"binding": "k"} → Binding, {"signal": <SignalOrFloat>} → Signal, plus a bare-SignalSpec fallback (e.g. {"type": "sine", ...}). Original 116-LOC body retired to recyclebin/crates/tui-vfx-compositor-next/src/types/cls_bindable_value.rs.</CLOG>

//! # BindableValue (re-export)
//!
//! Historical name for [`tui_vfx_core::bindable::VfxBindableValue`]. The
//! canonical definition, peer tests, and behaviour live in `tui-vfx-core`;
//! this module exists so downstream code that imports from
//! `tui_vfx_compositor_next::types::BindableValue` continues to compile
//! unchanged during the sweep 1.2.A / 1.7.A consolidation.

pub use tui_vfx_core::bindable::VfxBindableValue as BindableValue;

// <FILE>crates/tui-vfx-compositor-next/src/types/cls_bindable_value.rs</FILE> - <DESC>Re-export of tui_vfx_core::bindable::VfxBindableValue under the historical `BindableValue` name. Preserves consumer import paths during the sweep 1.2.A consolidation; resolves sweep 1.7.A by moving the cross-crate home to tui-vfx-core.</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
