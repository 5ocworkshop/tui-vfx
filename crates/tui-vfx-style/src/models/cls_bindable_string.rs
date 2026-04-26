// <FILE>crates/tui-vfx-style/src/models/cls_bindable_string.rs</FILE> - <DESC>Re-export of tui_vfx_core::bindable::VfxBindableString under the historical `BindableString` name. Preserves consumer import paths during the sweep 1.2.A consolidation.</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.A — collapse three parallel hand-rolled Bindable types into the single VfxBindable<T, S> generic in tui-vfx-core. The hand-rolled body retired to recyclebin/ in this revision.</WCTX>
// <CLOG>0.2.0: BindableString is now an alias for tui_vfx_core::bindable::VfxBindableString (= VfxBindable<String, Never>). Inherent evaluate / literal / binding_key / Default / From<String|&str> / serde / ConfigSchema all surface through the generic. Original 322-LOC body and inline tests retired to recyclebin/crates/tui-vfx-style/src/models/cls_bindable_string.rs.</CLOG>

//! # BindableString (re-export)
//!
//! Historical name for [`tui_vfx_core::bindable::VfxBindableString`]. The
//! canonical definition, peer tests, and behaviour live in `tui-vfx-core`;
//! this module exists so downstream code that imports from
//! `tui_vfx_style::models::BindableString` continues to compile unchanged
//! during the sweep 1.2.A consolidation.

pub use tui_vfx_core::bindable::VfxBindableString as BindableString;

// <FILE>crates/tui-vfx-style/src/models/cls_bindable_string.rs</FILE> - <DESC>Re-export of tui_vfx_core::bindable::VfxBindableString under the historical `BindableString` name. Preserves consumer import paths during the sweep 1.2.A consolidation.</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
