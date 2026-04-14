// <FILE>tui-vfx-compositor/src/pipeline/cls_prepare_context.rs</FILE> - <DESC>PrepareContext bundling per-frame values threaded into prepare_filter</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 P0.1 — thread runtime_params through the filter prepare stage</WCTX>
// <CLOG>Introduce PrepareContext with loop_t, signal_ctx, runtime_params reference</CLOG>

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
}

impl<'a> PrepareContext<'a> {
    /// Build a PrepareContext from a loop time and a borrowed runtime-parameter
    /// map. Constructs a fresh `SignalContext::for_loop` internally so the
    /// caller only has to supply frame-level values.
    pub(crate) fn new(loop_t: f64, runtime_params: &'a ShaderRuntimeParams) -> Self {
        Self {
            loop_t,
            signal_ctx: SignalContext::for_loop(loop_t, 0),
            runtime_params,
        }
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepare_context.rs</FILE> - <DESC>PrepareContext bundling per-frame values threaded into prepare_filter</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
