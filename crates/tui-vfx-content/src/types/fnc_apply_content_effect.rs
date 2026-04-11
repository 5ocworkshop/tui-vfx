// <FILE>tui-vfx-content/src/types/fnc_apply_content_effect.rs</FILE> - <DESC>ContentEffect::apply ergonomic entry point</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>feat/content-ergonomics: ContentEffect::apply convenience entry point</WCTX>
// <CLOG>Initial impl block adding apply / apply_to_borrowed / apply_with_context to ContentEffect</CLOG>

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

use crate::transformers::get_transformer;
use crate::types::ContentEffect;

impl ContentEffect {
    /// Applies this effect to `target` with the given progress and returns
    /// an owned [`String`].
    ///
    /// Convenience wrapper around [`crate::transformers::get_transformer`]
    /// followed by [`crate::traits::TextTransformer::transform`] that
    /// default-constructs the [`SignalContext`]. For the common case where
    /// progress is the only time variable, this collapses three lines of
    /// boilerplate into one call:
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
    /// For advanced use with custom frame / seed / phase / spatial signals,
    /// use [`apply_with_context`](Self::apply_with_context) instead.
    pub fn apply(&self, target: &str, progress: f64) -> String {
        self.apply_to_borrowed(target, progress).into_owned()
    }

    /// Applies this effect to `target` with the given progress and returns
    /// a [`Cow`] that borrows from `target` when the underlying transformer
    /// produced no allocation.
    ///
    /// Preserves the zero-allocation fast path that
    /// [`crate::traits::TextTransformer::transform`] provides. Use this
    /// when you care about avoiding the `Cow::into_owned` call in the
    /// no-op case (for example, Typewriter at progress `1.0` returning the
    /// full target as a borrowed slice).
    pub fn apply_to_borrowed<'a>(&self, target: &'a str, progress: f64) -> Cow<'a, str> {
        let ctx = SignalContext::default();
        self.apply_with_context(target, progress, &ctx)
    }

    /// Applies this effect to `target` with the given progress and a
    /// caller-supplied [`SignalContext`], returning a [`Cow`].
    ///
    /// This is the advanced-use entry point for signal-driven pacing —
    /// dynamic blink rates, breathing cursors, parameters that vary with
    /// procedural noise. Most consumers should prefer [`apply`](Self::apply)
    /// or [`apply_to_borrowed`](Self::apply_to_borrowed), both of which
    /// default-construct the context.
    pub fn apply_with_context<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        let transformer = get_transformer(self);
        transformer.transform(target, progress, signal_ctx)
    }
}

// <FILE>tui-vfx-content/src/types/fnc_apply_content_effect.rs</FILE> - <DESC>ContentEffect::apply ergonomic entry point</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
