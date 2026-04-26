// <FILE>crates/tui-vfx-style/src/models/cls_bindable_u16.rs</FILE> - <DESC>Re-export of tui_vfx_core::bindable::VfxBindableU16 under the historical `BindableU16` name. Preserves consumer import paths during the sweep 1.2.A consolidation.</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.A — collapse three parallel hand-rolled Bindable types into the single VfxBindable<T, S> generic in tui-vfx-core. The hand-rolled body retired to recyclebin/ in this revision.</WCTX>
// <CLOG>0.2.0: BindableU16 is now an alias for tui_vfx_core::bindable::VfxBindableU16 (= VfxBindable<u16, Never>). Inherent evaluate / literal / Default / From<u16> / serde / ConfigSchema all surface through the generic. Original 250-LOC body and inline tests retired to recyclebin/crates/tui-vfx-style/src/models/cls_bindable_u16.rs.</CLOG>

//! # BindableU16 (re-export)
//!
//! Historical name for [`tui_vfx_core::bindable::VfxBindableU16`]. The
//! canonical definition, peer tests, and behaviour live in `tui-vfx-core`;
//! this module exists so downstream code that imports from
//! `tui_vfx_style::models::BindableU16` continues to compile unchanged
//! during the sweep 1.2.A consolidation.

pub use tui_vfx_core::bindable::VfxBindableU16 as BindableU16;

// <FILE>crates/tui-vfx-style/src/models/cls_bindable_u16.rs</FILE> - <DESC>Re-export of tui_vfx_core::bindable::VfxBindableU16 under the historical `BindableU16` name. Preserves consumer import paths during the sweep 1.2.A consolidation.</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
