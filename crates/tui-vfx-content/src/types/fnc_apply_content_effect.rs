// <FILE>tui-vfx-content/src/types/fnc_apply_content_effect.rs</FILE> - <DESC>ContentEffect::apply ergonomic entry point</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: TextTransformer trait now takes &TransformContext<'_>; the inherent ContentEffect::apply family follows.</WCTX>
// <CLOG>2.0.0: BREAKING — remove apply_with_context; add apply_with_runtime that takes (&SignalContext, &ShaderRuntimeParams) and constructs the TransformContext internally. apply / apply_to_borrowed default both pieces.</CLOG>

//! Ergonomic `apply` entry point on [`ContentEffect`].
//!
//! This module attaches three convenience methods to [`ContentEffect`] that
//! collapse the common "I have a progress value, give me the transformed
//! string" workflow into a single call. The full
//! [`crate::transformers::get_transformer`] followed by
//! [`crate::traits::TextTransformer::transform`] path remains the canonical
//! advanced API; these methods sit on top of it.

use std::borrow::Cow;

use mixed_signals::prelude::SignalContext;
use tui_vfx_style::traits::ShaderRuntimeParams;

use crate::traits::TransformContext;
use crate::transformers::get_transformer;
use crate::types::ContentEffect;

impl ContentEffect {
    /// Applies this effect to `target` with the given progress and returns
    /// an owned [`String`].
    ///
    /// Convenience wrapper around [`crate::transformers::get_transformer`]
    /// followed by [`crate::traits::TextTransformer::transform`] that
    /// default-constructs both halves of the [`TransformContext`] (an empty
    /// [`SignalContext`] and an empty [`ShaderRuntimeParams`]). For the
    /// common case where progress is the only time variable, this collapses
    /// three lines of boilerplate into one call:
    ///
    /// ```
    /// use tui_vfx_content::prelude::*;
    ///
    /// let effect = ContentEffect::Typewriter {
    ///     speed_variance: SignalOrFloat::Static(0.0),
    ///     cursor: None,
    /// };
    /// let revealed = effect.apply("Hello", 1.0);
    /// assert_eq!(revealed, "Hello");
    /// ```
    ///
    /// For advanced use with custom signals or host-supplied runtime
    /// bindings, use [`apply_with_runtime`](Self::apply_with_runtime) instead.
    pub fn apply(&self, target: &str, progress: f64) -> String {
        self.apply_to_borrowed(target, progress).into_owned()
    }

    /// Applies this effect to `target` with the given progress and returns
    /// a [`Cow`] that borrows from `target` when the underlying transformer
    /// produced no allocation.
    ///
    /// Preserves the zero-allocation fast path that
    /// [`crate::traits::TextTransformer::transform`] provides. Use this
    /// when you care about avoiding the [`Cow::into_owned`] call in the
    /// no-op case (for example, Typewriter at progress `1.0` returning the
    /// full target as a borrowed slice).
    ///
    /// Like [`apply`](Self::apply), default-constructs both halves of the
    /// [`TransformContext`]. Hot-path consumers that already hold a real
    /// [`SignalContext`] / [`ShaderRuntimeParams`] should call
    /// [`apply_with_runtime`](Self::apply_with_runtime) directly to avoid
    /// the per-call default construction.
    pub fn apply_to_borrowed<'a>(&self, target: &'a str, progress: f64) -> Cow<'a, str> {
        let sig = SignalContext::default();
        let params = ShaderRuntimeParams::new();
        let ctx = TransformContext::new(&sig, &params);
        let transformer = get_transformer(self);
        transformer.transform(target, progress, &ctx)
    }

    /// Applies this effect to `target` with the given progress and a
    /// caller-supplied [`SignalContext`] and [`ShaderRuntimeParams`],
    /// returning a [`Cow`].
    ///
    /// This is the host-injection entry point for binding-form recipe
    /// fields (font names, asset names, locale tokens, and any other
    /// `BindableString` / `BindableU16` / `BindableF32` / `BindableColor`
    /// references whose resolution depends on host-supplied values).
    /// Most consumers should prefer [`apply`](Self::apply) or
    /// [`apply_to_borrowed`](Self::apply_to_borrowed); both default-construct
    /// both halves of the context.
    pub fn apply_with_runtime<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
        runtime_params: &ShaderRuntimeParams,
    ) -> Cow<'a, str> {
        let ctx = TransformContext::new(signal_ctx, runtime_params);
        let transformer = get_transformer(self);
        transformer.transform(target, progress, &ctx)
    }
}

// <FILE>tui-vfx-content/src/types/fnc_apply_content_effect.rs</FILE> - <DESC>ContentEffect::apply ergonomic entry point</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
