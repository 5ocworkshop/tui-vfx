// <FILE>tui-vfx-compositor-next/src/traits/mod.rs</FILE>
// <DESC>Traits module — CompositorInspector + InspectionSinkBridge bridge</DESC>
// <VERS>VERSION: 1.6.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — add cls_inspection_sink_bridge module exposing InspectionSinkBridge + TraceFrameContext. CompositorInspector stays here; the bridge is the additive forwarding path into tui-vfx-debug's InspectionSink.</WCTX>
// <CLOG>1.6.0: MINOR additive — publish cls_inspection_sink_bridge alongside the existing pipeline_inspector module.
// 1.5.0: Centralize on pipeline API; expose pipeline_inspector publicly, keep filter/mask/sampler pub(crate).</CLOG>

pub mod cls_inspection_sink_bridge;
pub(crate) mod filter;
pub(crate) mod mask;
pub mod pipeline_inspector;
pub(crate) mod sampler;
pub use tui_vfx_core::ConfigSchema;

// <FILE>tui-vfx-compositor-next/src/traits/mod.rs</FILE>
// <DESC>Traits module</DESC>
// <VERS>END OF VERSION: 1.6.0</VERS>
