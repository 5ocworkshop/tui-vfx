// <FILE>tui-vfx-content/tests/test_content_effect_apply.rs</FILE> - <DESC>Tests for ContentEffect::apply ergonomic methods</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>feat/content-ergonomics: ContentEffect::apply convenience entry point</WCTX>
// <CLOG>Initial test file covering apply / apply_to_borrowed / apply_with_context</CLOG>

use std::borrow::Cow;

use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use tui_vfx_content::transformers::get_transformer;
use tui_vfx_content::types::{ContentEffect, ScrambleCharset};

fn typewriter() -> ContentEffect {
    ContentEffect::Typewriter {
        speed_variance: SignalOrFloat::Static(0.0),
        cursor: None,
    }
}

fn scramble() -> ContentEffect {
    ContentEffect::Scramble {
        resolve_pace: SignalOrFloat::Static(1.0),
        charset: ScrambleCharset::Alphanumeric,
        seed: 42,
    }
}

#[test]
fn apply_typewriter_progress_zero_returns_empty() {
    let effect = typewriter();
    assert_eq!(effect.apply("Hello", 0.0), "");
}

#[test]
fn apply_typewriter_progress_full_returns_target() {
    let effect = typewriter();
    assert_eq!(effect.apply("Hello", 1.0), "Hello");
}

#[test]
fn apply_typewriter_progress_half_returns_partial_prefix() {
    let effect = typewriter();
    let target = "Hello";
    let result = effect.apply(target, 0.5);
    assert!(!result.is_empty(), "expected non-empty partial reveal");
    assert!(
        result.len() < target.len(),
        "expected partial reveal shorter than target ({} >= {})",
        result.len(),
        target.len()
    );
    assert!(
        target.starts_with(result.as_str()),
        "expected reveal to be a prefix of the target, got {result:?}"
    );
}

#[test]
fn apply_to_borrowed_returns_borrowed_when_transformer_passes_through() {
    // Typewriter at progress 1.0 returns the full target without
    // allocation — preserve the Cow::Borrowed fast path.
    let effect = typewriter();
    let result = effect.apply_to_borrowed("Hello", 1.0);
    assert!(
        matches!(result, Cow::Borrowed(_)),
        "expected Cow::Borrowed at progress 1.0, got Owned"
    );
    assert_eq!(result, "Hello");
}

#[test]
fn apply_matches_explicit_dispatcher_path() {
    // Round-trip equivalence: the new entry point must produce the
    // exact same output as the existing get_transformer + transform
    // path with a default SignalContext.
    let effect = typewriter();
    let target = "tui-vfx";
    let progress = 0.4;

    let via_apply = effect.apply(target, progress);

    let transformer = get_transformer(&effect);
    let ctx = SignalContext::default();
    let via_explicit = transformer.transform(target, progress, &ctx).into_owned();

    assert_eq!(via_apply, via_explicit);
}

#[test]
fn apply_with_context_matches_explicit_path_with_same_context() {
    // Advanced entry point: with a non-default context, output must
    // match the explicit dispatcher + transform call with that same
    // context.
    let effect = scramble();
    let target = "SYSTEM ONLINE";
    let progress = 0.3;
    let ctx = SignalContext {
        frame: 42,
        seed: 99,
        width: 80,
        height: 24,
        phase: None,
        phase_t: None,
        loop_t: None,
        absolute_t: None,
        char_index: None,
    };

    let via_apply = effect
        .apply_with_context(target, progress, &ctx)
        .into_owned();

    let transformer = get_transformer(&effect);
    let via_explicit = transformer.transform(target, progress, &ctx).into_owned();

    assert_eq!(via_apply, via_explicit);
}

#[test]
fn apply_dispatches_through_for_non_typewriter_variants() {
    // Confirm the dispatch path works for at least one other variant.
    // Scramble at progress 1.0 fully resolves to the target.
    let effect = scramble();
    let result = effect.apply("HELLO", 1.0);
    assert_eq!(result, "HELLO");
}

#[test]
fn apply_handles_empty_target() {
    let effect = typewriter();
    assert_eq!(effect.apply("", 0.0), "");
    assert_eq!(effect.apply("", 0.5), "");
    assert_eq!(effect.apply("", 1.0), "");
}

#[test]
fn apply_handles_single_character_target() {
    let effect = typewriter();
    assert_eq!(effect.apply("X", 0.0), "");
    assert_eq!(effect.apply("X", 1.0), "X");
}

// <FILE>tui-vfx-content/tests/test_content_effect_apply.rs</FILE> - <DESC>Tests for ContentEffect::apply ergonomic methods</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
