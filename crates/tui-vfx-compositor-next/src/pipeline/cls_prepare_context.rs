// <FILE>tui-vfx-compositor-next/src/pipeline/cls_prepare_context.rs</FILE> - <DESC>PrepareContext bundling per-frame values threaded into prepare_filter</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>TTE effects port phase 4b — add canvas width/height so prepare arms that bake per-cell schedules (FilterSpec::GlyphTimeline + PoissonBurst trigger) have the dimensions at construction time.</WCTX>
// <CLOG>0.2.0: add `width: u16, height: u16` fields + update PrepareContext::new signature. Pre-existing production callers in orc_render_pipeline.rs already have width/height in scope.</CLOG>

//! # PrepareContext
//!
//! Bundle of per-frame values that filter-spec conversion needs to resolve
//! signal expressions and runtime-parameter bindings. Passed by reference
//! through `prepare_filters` → `prepare_filter` so each arm can evaluate
//! its dynamic fields without requiring extra arguments.
//!
//! Adding a new context-wide value (e.g. screen width for `x_position_binding`,
//! severity tier for `num_shakes_binding`) means adding a field here rather
//! than expanding the prepare-function signature — the whole Phase 0
//! binding-generalization series reuses this type.

use mixed_signals::traits::SignalContext;
use tui_vfx_style::traits::ShaderRuntimeParams;

/// Per-frame context passed into filter preparation.
///
/// The `signal_ctx` is constructed from `loop_t` at the start of each frame
/// and lives inside this bundle so that `prepare_filter` arms don't each
/// have to rebuild it. `runtime_params` is borrowed from the caller's
/// `CompositionOptions` / `CompositionSpec`, keeping the `ShaderRuntimeParams`
/// map owned by the render request.
pub(crate) struct PrepareContext<'a> {
    pub loop_t: f64,
    pub signal_ctx: SignalContext,
    pub runtime_params: &'a ShaderRuntimeParams,
    /// Canvas width in cells. Used by prepare arms that bake per-cell
    /// schedules at construction time (e.g. `FilterSpec::GlyphTimeline`
    /// with a `PoissonBurst` trigger calls
    /// `tui_vfx_style::schedules::poisson_burst_schedule(width, height, ..)`).
    pub width: u16,
    /// Canvas height in cells. See `width`.
    pub height: u16,
}

impl<'a> PrepareContext<'a> {
    /// Build a PrepareContext from a loop time, runtime-parameter map,
    /// and canvas dimensions. Constructs a fresh `SignalContext::for_loop`
    /// internally so the caller only has to supply frame-level values.
    pub(crate) fn new(
        loop_t: f64,
        runtime_params: &'a ShaderRuntimeParams,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            loop_t,
            signal_ctx: SignalContext::for_loop(loop_t, 0),
            runtime_params,
            width,
            height,
        }
    }
}

// <FILE>tui-vfx-compositor-next/src/pipeline/cls_prepare_context.rs</FILE>
// <VERS>END OF VERSION: 0.2.0</VERS>
