// <FILE>tui-vfx-compositor/src/types/cls_bindable_value.rs</FILE> - <DESC>Bindable filter-spec value wrapping signal expressions and runtime parameter lookups</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 0 P0.1 — progress_binding infrastructure for filter spec fields</WCTX>
// <CLOG>Add lenient deserialize accepting raw-number, tagged-binding, tagged-signal, and bare SignalOrFloat forms so existing recipe JSON (e.g. "progress": 0.5) keeps working</CLOG>

//! # BindableValue
//!
//! A filter-spec field value that can be one of:
//!
//! - [`BindableValue::Signal`] — a [`SignalOrFloat`] expression (literal or signal-driven)
//! - [`BindableValue::Binding`] — a named runtime parameter looked up at render
//!   time from [`ShaderRuntimeParams`]
//!
//! Used to turn static filter parameters (e.g. `progress: 0.5`) into live widget
//! bindings (e.g. `progress_binding: "scroll_progress"`) without modifying the
//! `mixed-signals` crate upstream. Runtime bindings are a rendering concern;
//! keeping them in a compositor-local wrapper preserves `mixed-signals` as a
//! pure signal-math library.
//!
//! ## Accepted JSON input shapes
//!
//! All four shapes deserialize into a `BindableValue`. Existing recipes that
//! emit a raw number for a progress field keep working without migration.
//!
//! ```json
//! 0.5
//! { "static": 0.5 }
//! { "signal": { "static": 0.5 } }
//! { "binding": "progress_ratio" }
//! ```
//!
//! Serialization always emits the normalized tagged form
//! (`{"signal": ...}` or `{"binding": ...}`).

use mixed_signals::traits::SignalContext;
use mixed_signals::types::SignalOrFloat;
use serde::{Deserialize, Serialize};
use tui_vfx_style::traits::ShaderRuntimeParams;

/// A filter-spec field value resolved at frame-prepare time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "snake_case", from = "BindableValueRepr")]
pub enum BindableValue {
    /// A signal expression or static literal, resolved through `mixed-signals`.
    Signal(SignalOrFloat),
    /// A named runtime parameter, looked up in [`ShaderRuntimeParams`] per frame.
    Binding(String),
}

/// Lenient on-disk representation. Accepts raw numbers, `{"binding": ...}`,
/// `{"signal": ...}`, or a bare `SignalOrFloat` (e.g. `{"static": 0.5}`).
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BindableValueRepr {
    /// `{"binding": "name"}`
    Binding { binding: String },
    /// `{"signal": {...}}`
    Signal { signal: SignalOrFloat },
    /// Bare number (`0.5`) or bare SignalOrFloat (`{"static": 0.5}`).
    Bare(SignalOrFloat),
}

impl From<BindableValueRepr> for BindableValue {
    fn from(repr: BindableValueRepr) -> Self {
        match repr {
            BindableValueRepr::Binding { binding } => BindableValue::Binding(binding),
            BindableValueRepr::Signal { signal } => BindableValue::Signal(signal),
            BindableValueRepr::Bare(signal) => BindableValue::Signal(signal),
        }
    }
}

impl BindableValue {
    /// Evaluate this value against the current frame's signal context and
    /// runtime parameter map. Returns `None` if the binding is missing or the
    /// signal expression fails to build — callers typically `unwrap_or` a
    /// filter-specific default. Signal-build errors are collapsed to `None`
    /// so callers only have to deal with a single "no value" sentinel.
    pub fn evaluate(
        &self,
        loop_t: f64,
        signal_ctx: &SignalContext,
        runtime_params: &ShaderRuntimeParams,
    ) -> Option<f32> {
        match self {
            BindableValue::Signal(value) => value.evaluate(loop_t, signal_ctx).ok(),
            BindableValue::Binding(param) => runtime_params.get_f32(param),
        }
    }

    /// Construct a static-literal bindable value from an `f32`.
    pub fn static_f32(value: f32) -> Self {
        BindableValue::Signal(SignalOrFloat::Static(value))
    }
}

impl From<f32> for BindableValue {
    fn from(value: f32) -> Self {
        BindableValue::static_f32(value)
    }
}

impl From<SignalOrFloat> for BindableValue {
    fn from(value: SignalOrFloat) -> Self {
        BindableValue::Signal(value)
    }
}

impl Default for BindableValue {
    fn default() -> Self {
        BindableValue::static_f32(0.0)
    }
}

// <FILE>tui-vfx-compositor/src/types/cls_bindable_value.rs</FILE> - <DESC>Bindable filter-spec value wrapping signal expressions and runtime parameter lookups</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
