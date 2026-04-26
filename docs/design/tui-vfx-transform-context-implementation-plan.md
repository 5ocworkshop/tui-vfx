<!-- <FILE>docs/design/tui-vfx-transform-context-implementation-plan.md</FILE> - <DESC>Implementation plan for the TransformContext bundle change that lets BindableString::Binding font references resolve to host-supplied runtime values; closes Slice 6.6 of the mechanical circular content cycles plan</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Phase 6 of mechanical circular content cycles plan — final outstanding piece (Slice 6.6) is the architectural change that threads ShaderRuntimeParams through TextTransformer; bundle into TransformContext so future context additions don't churn the trait again.</WCTX> -->
<!-- <CLOG>1.0.0: refined to implementation-ready form. Pre-flight grounding section added. Trait file path corrected (text_transformer.rs, not cls_text_transformer.rs). Production-caller plumbing rewritten — callers live in gt-design (recipes/render.rs:119 + text_effects/mod.rs:168), NOT tui-vfx-compositor. Inline #[cfg(test)] sweep in 11 source files surfaced (110+ call sites). Typewriter::transform_with_cursor inherent method addressed. Per-phase shell-ready verification with expected output. Critical findings section added. Code snippets for every non-trivial step.</CLOG> -->

# tui-vfx TransformContext implementation plan

> **Status:** Implementation-ready (v1.0.0). A junior engineer with read access to the repo and `ofpf-*` tooling installed should be able to execute this plan with little oversight.
>
> **Target slice:** Slice 6.6 of `docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md`.
>
> **Scope:** Single coordinated slice touching the `TextTransformer` trait surface, all 15 transformer impls, the `ContentEffect::apply_*` family, every test that calls `.transform()`, and the production caller plumbing in `gt-design`. Plus one recipe migration in `tui-vfx-recipes` and a documentation sweep.

---

## 0. Pre-flight grounding (do this first, before touching code)

These commands give you the orientation you need to proceed. Run them in order. Do **not** start editing until each one returns the expected shape.

### 0.1 Confirm the librarian daemon is healthy

```bash
ofpf-status
```

Expected: `graph_loaded: true`, `is_stale: false`, non-zero `definition_count`. If the daemon is stale or unhealthy, run `ofpf-load` and re-check before proceeding. Stale graph data leads to wrong call-site enumeration, and the trait change blast radius is exactly what you cannot afford to get wrong.

### 0.2 Architectural orientation

```bash
ofpf-orientation --root /usr/projects/tui-vfx
```

This shows hubs, cores, and orchestrators. Confirm `tui-vfx-content` is a leaf-style crate (no compositor dependency on it from src). If `tui-vfx-compositor`'s **src** appears to depend on `tui-vfx-content`, something has changed since this plan was written — stop and re-validate the production-caller mapping in §B before proceeding.

### 0.3 Locate the TextTransformer trait file

```bash
ofpf-defs TextTransformer
```

Expected output (single hit):

```json
{ "def": "trait TextTransformer",
  "loc": "crates/tui-vfx-content/src/traits/text_transformer.rs:12" }
```

**Filename note.** The file is `text_transformer.rs`, **not** `cls_text_transformer.rs`. The v0.1.0 draft of this plan guessed the latter. The OFPF prefix convention (`cls_` for single-cohesive-class files) is not applied to this file because it predates that convention being applied here; do not rename the file as part of this slice (out of scope per the task contract).

### 0.4 Map the trait file's neighborhood

```bash
ofpf-inspect crates/tui-vfx-content/src/traits/text_transformer.rs
```

Expected: `defs.traits: ["TextTransformer:12"]`, `caller_counts.logic: 1` (the `mod.rs` re-export). Role: `unit`. The file is 28 LOC.

### 0.5 Map the trait's blast radius

```bash
ofpf-blast crates/tui-vfx-content/src/traits/text_transformer.rs
```

Expected blast set (33 files at the time this plan was written):

- 1 trait file itself
- 15 transformer impls in `crates/tui-vfx-content/src/transformers/cls_*.rs`
- 1 dispatcher: `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs`
- 1 inherent-method file: `crates/tui-vfx-content/src/types/fnc_apply_content_effect.rs`
- 11 transformer integration tests in `crates/tui-vfx-content/tests/transformers/test_cls_*.rs`
- `crates/tui-vfx-content/src/lib.rs` (rustdoc doctest at lines ~46–53)
- `crates/tui-vfx-content/src/prelude.rs` and re-export shims
- `crates/tui-vfx/src/lib.rs` (top-level workspace re-export at line 147 / 188)
- `crates/tui-vfx-compositor/src/lib.rs` (workspace re-export only — no consuming src)
- `xtask/src/main.rs` (docs generation, no live `.transform()` call)

If your `ofpf-blast` returns substantially more or fewer files, stop and reconcile before starting Phase A. Drift here means new callers landed; redo the mapping.

### 0.6 Enumerate transformer impls

```bash
ls /usr/projects/tui-vfx/crates/tui-vfx-content/src/transformers/
```

Expected: 15 `cls_*.rs` files, 2 `fnc_*.rs` files (`fnc_get_transformer.rs`, `fnc_morph_chars.rs`), and `mod.rs`. The 15 trait impls you must edit:

`cls_dissolve.rs`, `cls_glitch_shift.rs`, `cls_glyph_cascade.rs`, `cls_marquee.rs`, `cls_mirror.rs`, `cls_morph.rs`, `cls_numeric.rs`, `cls_odometer.rs`, `cls_redact.rs`, `cls_scramble.rs`, `cls_scramble_glitch_shift.rs`, `cls_slide_shift.rs`, `cls_split_flap.rs`, `cls_typewriter.rs`, `cls_wrap_indicator.rs`.

### 0.7 Find every `.transform(` call site

```bash
grep -rn "\.transform(" /usr/projects/tui-vfx/crates/tui-vfx-content/src/
grep -rn "\.transform(" /usr/projects/tui-vfx/crates/tui-vfx-content/tests/
```

Expected per-file source counts (from `grep -rc`):

| File | Count |
|---|---|
| `src/transformers/cls_split_flap.rs` | 55 |
| `src/transformers/cls_wrap_indicator.rs` | 16 |
| `src/transformers/cls_morph.rs` | 12 |
| `src/transformers/cls_dissolve.rs` | 8 |
| `src/transformers/cls_glitch_shift.rs` | 6 |
| `src/transformers/cls_mirror.rs` | 5 |
| `src/transformers/cls_scramble_glitch_shift.rs` | 4 |
| `src/transformers/cls_glyph_cascade.rs` | 2 |
| `src/transformers/cls_typewriter.rs` | 1 |
| `src/types/fnc_apply_content_effect.rs` | 1 |
| `src/lib.rs` | 1 (doctest) |

Total: ~111 inline call sites in `src/`, plus ~50 call sites across 11 `tests/transformers/test_cls_*.rs` files.

**Most of these live inside `#[cfg(test)]` modules at the bottom of source files.** They must all migrate. The doctest in `lib.rs` (around line 51) is its own `cargo test --doc` failure mode. Do not skip the doctest sweep.

### 0.8 Map the production callers

```bash
grep -rn -E "(\.apply\(|apply_with_context|apply_to_borrowed|TextTransformer|get_transformer)" /usr/projects/gt-design/crates/
```

Expected production call sites:

- `/usr/projects/gt-design/crates/gtd-ratatui/src/recipes/render.rs:119` — `let transformer = get_transformer(effect);` followed by `transformer.transform(message, effective_t, signal_ctx).to_string()` at line 130. **This is the live render path.** A `SignalContext` is already in scope. A `ShaderRuntimeParams` reference is **not** yet in scope here; threading it in is part of Phase B.
- `/usr/projects/gt-design/crates/gtd-ratatui/src/text_effects/mod.rs:168–172` — the `ApplyTextEffect::apply` impl calls `transformer.transform(text, progress as f64, &ctx).into_owned()` with a `SignalContext::default()`. This facade does not know about runtime params; the migration here is to pass `&ShaderRuntimeParams::new()` for now (matches the prior implicit behavior) and document that consumers wanting host-supplied bindings use the lower-level `get_transformer` path with their own `TransformContext`.

**The compositor's `src/` does NOT consume `tui-vfx-content`.** It is a dev-dep only (used by `tests/cursor_integration.rs`). The v0.1.0 draft's "compositor's V3 content pipeline" framing was wrong on this point — the production wiring is in gt-design. See §10 Critical Findings.

### 0.9 Verify ShaderRuntimeParams import path

```bash
grep -n "pub use\|ShaderRuntimeParams" /usr/projects/tui-vfx/crates/tui-vfx-style/src/traits/mod.rs
```

Expected: `pub use cls_shader_context::{… ShaderRuntimeParams, …};`. The canonical import path for transformer code is:

```rust
use tui_vfx_style::traits::ShaderRuntimeParams;
```

`tui-vfx-content` already depends on `tui-vfx-style` (`Cargo.toml:25`), so no new dep is needed.

### 0.10 Establish the test baseline

```bash
cargo test -p tui-vfx-content --lib 2>&1 | tail -3
cargo test -p tui-vfx-content 2>&1 | tail -3
cargo test --doc -p tui-vfx-content 2>&1 | tail -3
```

At the time this plan was written: lib tests pass at **356**. Capture your local baseline number before starting; the only new tests should be the two Phase C integration tests. Anything else that changes is a regression.

---

## 1. Current situation

### 1.1 What's already shipped

Phase 6 of the mechanical circular content cycles plan (commits `e1de449`, `932ed98`, `b08dfa5`) lifted the font into a bindable surface:

- **`BindableString`** at `crates/tui-vfx-style/src/models/cls_bindable_string.rs` — `Literal(String) | Binding(String)` mirroring `BindableU16`'s shape; lenient bare-string deserialization; `evaluate(&ShaderRuntimeParams) -> Option<&str>`.
- **`FontRegistry` + `FontGlyphTable`** at `crates/tui-vfx-content/src/fonts/` — name → glyph-table mapping with `default_font` sentinel routing per Intention 36. Constructed registries auto-register the embedded Line 3x3 face as the default.
- **`font: Option<BindableString>` field** on `MechanicalContentSource::Preset` plus `resolve_mechanical_cycle_with_context(source, tile, &FontRegistry, &ShaderRuntimeParams)` at `crates/tui-vfx-content/src/mechanical/fnc_resolve_mechanical_cycle.rs:60`. This is the function `cls_odometer.rs` will switch over to in Phase C.
- **Recipe migration** of `recipes/debug_recipes/content/content_odometer_3x3_count_bindable.json` from literal-glyph faces to `Preset { decimal_digits } + font: "line-3x3"` (literal form). Per Intention 38, the recipe carries the `_bindable` filename suffix and the `bindable` metadata tag.

### 1.2 The L2 binding-loopback work just shipped

Sibling Claude landed L2 of the binding-loopback design at commit `7e7e88f` (tui-vfx-recipes) + `66fe546` (tui-vfx). What L2 actually delivers:

- The recipe envelope's existing `requires_bindings` block accepts typed `{ "type": "string", "default": <value-or-signal>, "description": <text> }` declarations.
- The strict-contracts validator gate enforces that every `{"binding": "name"}` reference at any depth in the pipeline has a matching `requires_bindings.<name>` declaration.
- The loopback layer pre-fills `ShaderRuntimeParams` with declared loopback values when the host hasn't supplied them, so recipes are preview-playable per Intention 37.

What L2 does NOT do (sibling's clarification, captured in cycle plan v0.7.0):

- It does not change the `{"binding": "name"}` reference shape. That shape is V3 canon, used everywhere step payloads carry runtime-bound parameters.
- It does not touch host-side asset-resolver concerns (`AssetRegistry` vs. `ImagePool` vs. `AssetMap`).
- It does not gate runtime threading. Whether transformers can actually *read* `ShaderRuntimeParams` is independent of L2.

### 1.3 The remaining gap

`TextTransformer::transform` does not currently receive `&ShaderRuntimeParams`. The actual signature today (verified via `cat crates/tui-vfx-content/src/traits/text_transformer.rs`):

```rust
// crates/tui-vfx-content/src/traits/text_transformer.rs (current, v2.0.0)
use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;

pub trait TextTransformer {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str>;
}
```

Consequence chain:

1. `cls_odometer.rs::transform` (line 60) accepts `_signal_ctx: &SignalContext` (currently underscored — unused) and cannot read host-supplied `ShaderRuntimeParams`.
2. At line 118 it calls the back-compat `resolve_mechanical_cycle(&cfg.source, tile)` entry point, which constructs an empty `ShaderRuntimeParams::new()` internally (see `fnc_resolve_mechanical_cycle.rs:38–40`).
3. `BindableString::Binding(key)` font references evaluate to `None` against the empty map.
4. The `resolve_font_table` helper falls back to the `FontRegistry`'s default per Intention 36's runtime-fallback rule.
5. **Net effect:** a recipe that authors `font: { "binding": "drum_font" }` with `requires_bindings.drum_font: { type: "string", default: "default_font" }` passes validation, gets its loopback populated by L2 into the host's runtime-params map (which the transformer cannot read), and renders identically regardless of host injection.

The literal form (`font: "line-3x3"`) is unaffected because `BindableString::Literal` evaluates without consulting `ShaderRuntimeParams`.

### 1.4 Why a struct, not a bare parameter

Adding a single new parameter would close the loop:

```rust
fn transform<'a>(
    &self,
    target: &'a str,
    progress: f64,
    signal_ctx: &SignalContext,
    runtime_params: &ShaderRuntimeParams,    // ← naive minimal change
) -> Cow<'a, str>;
```

But this is the second time we'd be adding a context-shaped parameter to this trait (the first added `signal_ctx` itself, in v2.0.0). A third — substitutions, a theme-snapshot reference, an asset resolver, anything — forces another sweep across every transformer, every caller, every test. **The buy-once/cry-once approach bundles into a `TransformContext` struct and makes future additions struct-extensions rather than trait churn.** See §2.3 for the rule-of-three justification.

---

## 2. Steering alignment

This change touches the public trait surface of `tui-vfx-content` and ripples through every transformer. The relevant durable framing:

### 2.1 Intention 36 — Line 3x3 default font

The runtime fallback to the registry default *is* the failure mode this slice addresses. Today, the runtime falls back unconditionally because it has no way to read host-supplied font names. Slice 6.6 makes the fallback what Intention 36 actually intended: a *resilience* mechanism for missing assets, not a *substitute* for missing runtime threading.

### 2.2 Intention 37 — Loopback is required

The loopback layer L2 just shipped populates `ShaderRuntimeParams` with declared loopback values. Without Slice 6.6, that population is dead code from the transformer's perspective — the loopback map sits there but the transformer never reads it. Intention 37's preview-playable contract holds today only because `BindableString::Literal` ignores `ShaderRuntimeParams` entirely. Once we want host-supplied values to flow, the threading must exist.

### 2.3 Intention 24 — Library changes earn their place; Intention 23 — rule of three

The honest accounting:

- The trait already has `signal_ctx: &SignalContext` from a previous addition. That's one prior context-shaped parameter (added at trait v2.0.0 — see the file's CLOG: *"BREAKING: Added signal_ctx parameter to transform() method for signal-driven effects"*).
- The current need adds `runtime_params: &ShaderRuntimeParams`. That's two.
- Open known future candidates: `&Substitutions` (load-time host context, currently consumed at recipe-load not transform-time but conceivably needed for late-binding cases), an asset-resolver reference if Phase 7 lifts `image_name` to bindable, and theme-snapshot reads if any transformer ever needs theme-resolved colors. Three plausible additions.

That's the rule-of-three threshold. The bundle earns its place. We are not introducing it speculatively for things we *might* need; we are introducing it because we have two context parameters today and at least one in flight.

### 2.4 No-parse-and-inert rule (`feedback_no_inert_schema.md`)

The `mechanical.source.font` field today *appears* fully wired but for `BindableString::Binding` form it silently degrades to the registry default. By the strict reading of the no-inert rule, this is a partial-wire — the field accepts a binding shape that doesn't actually resolve to host values. Closing this with Slice 6.6 brings the schema field to true full-wire status.

### 2.5 Performance — 60 fps / 16.7 ms budget

Threading a context struct rather than bare parameters has zero runtime cost (the struct is two reference-sized fields, one cache line, `#[derive(Clone, Copy)]`). The architectural choice is performance-neutral.

**Allocation watch.** The current `cls_odometer.rs:118` calls `resolve_mechanical_cycle(&cfg.source, tile)`, which constructs a fresh `ShaderRuntimeParams::new()` internally on every call (currently every per-frame transform). Phase C eliminates that allocation by routing through `resolve_mechanical_cycle_with_context` with the borrowed `ctx.runtime_params`. **The plan must not introduce a new per-cell or per-frame allocation in any transformer.** Specifically: do not construct a `TransformContext` inside any per-cell or per-tile loop — it is constructed once per `transform()` invocation by the caller.

### 2.6 OFPF discipline

`TransformContext` is a leaf data type with simple accessors. It belongs in `crates/tui-vfx-content/src/traits/cls_transform_context.rs` next to the existing `text_transformer.rs`. File size will be well under the `cls_` 200-LOC hard limit (~50 LOC including docs and tests).

---

## 3. Architectural decision

### 3.1 The struct

```rust
// crates/tui-vfx-content/src/traits/cls_transform_context.rs (NEW FILE)
// <FILE>tui-vfx-content/src/traits/cls_transform_context.rs</FILE> - <DESC>Per-call context bundle passed to TextTransformer::transform</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: bundle SignalContext + ShaderRuntimeParams so future additions extend the struct rather than churning the trait.</WCTX>
// <CLOG>1.0.0: introduce TransformContext { signal_ctx, runtime_params } with new() constructor and a test-only default_for_tests() helper.</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_style::traits::ShaderRuntimeParams;

/// Bundle of call-time context passed to every [`TextTransformer`].
///
/// Adding a new context piece in the future (e.g. a `Substitutions` reference,
/// an asset resolver, a theme snapshot) extends this struct without trait
/// churn. Transformers ignore fields they don't need; the struct is `Copy`
/// and zero-cost (two reference-sized fields).
///
/// [`TextTransformer`]: crate::traits::TextTransformer
#[derive(Clone, Copy)]
pub struct TransformContext<'a> {
    /// Per-frame signal evaluation context (frame, seed, phase, normalized
    /// coords, char-index, etc.). Used by signal-driven parameter resolution.
    pub signal_ctx: &'a SignalContext,
    /// Host-supplied runtime parameters. Carries values for `BindableString`
    /// / `BindableU16` / `BindableF32` / `BindableColor` fields whose recipe
    /// shape is `{"binding": "name"}`. Empty map is equivalent to "no host
    /// values supplied"; transformers must degrade gracefully (typically to
    /// a static or asset-default fallback).
    pub runtime_params: &'a ShaderRuntimeParams,
}

impl<'a> TransformContext<'a> {
    /// Construct a context bundle from explicit references. Most callers
    /// use this directly at the transform site; gt-design's render path
    /// already holds both pieces.
    pub fn new(
        signal_ctx: &'a SignalContext,
        runtime_params: &'a ShaderRuntimeParams,
    ) -> Self {
        Self { signal_ctx, runtime_params }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_for_tests_is_constructible() {
        let sig = SignalContext::default();
        let params = ShaderRuntimeParams::new();
        let ctx = TransformContext::new(&sig, &params);
        // Just exercise field reads to confirm the struct shape compiles
        // against the trait callers' expectations.
        assert!(ctx.runtime_params.get("nonexistent").is_none());
        let _ = ctx.signal_ctx;
    }
}

// <FILE>tui-vfx-content/src/traits/cls_transform_context.rs</FILE>
// <VERS>END OF VERSION: 1.0.0</VERS>
```

**Why no `Default` impl on the struct itself.** `TransformContext` borrows; it cannot be `Default` without owning its fields. Tests construct a default by binding two locals (`SignalContext::default()`, `ShaderRuntimeParams::new()`) and calling `TransformContext::new(&sig, &params)`. A test helper (§A.5) collapses the noise.

### 3.2 The trait change

```rust
// crates/tui-vfx-content/src/traits/text_transformer.rs (REPLACED)
// <FILE>tui-vfx-content/src/traits/text_transformer.rs</FILE> - <DESC>TextTransformer trait definition</DESC>
// <VERS>VERSION: 3.0.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: bundle context into TransformContext so the trait absorbs a second context piece (ShaderRuntimeParams) and future ones (substitutions, asset resolver) without further churn.</WCTX>
// <CLOG>3.0.0: BREAKING — replace `signal_ctx: &SignalContext` with `ctx: &TransformContext<'_>`. Per-call context is now bundled. Migration: replace `signal_ctx` reads with `ctx.signal_ctx`; transformers needing runtime params read `ctx.runtime_params`.</CLOG>

use crate::traits::TransformContext;
use std::borrow::Cow;

/// A trait for applying visual effects to text strings.
///
/// Implementors should use `Cow<str>` to return the original string slice
/// if no transformation is needed, avoiding unnecessary allocations.
pub trait TextTransformer {
    /// Transforms the target string based on the current progress (0.0 to 1.0).
    ///
    /// # Arguments
    /// * `target` - The final string to display.
    /// * `progress` - Animation progress from 0.0 (start) to 1.0 (end).
    /// * `ctx` - Per-call context bundle. See [`TransformContext`].
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str>;
}

// <FILE>tui-vfx-content/src/traits/text_transformer.rs</FILE>
// <VERS>END OF VERSION: 3.0.0</VERS>
```

**Lifetime note.** The trait's `'a` is independent of `TransformContext`'s `'_`. The output `Cow<'a, str>` borrows from `target`; the context borrow is irrelevant at the return type. Transformers that don't need `ctx.runtime_params` simply ignore the field.

### 3.3 The `traits/mod.rs` re-export

```rust
// crates/tui-vfx-content/src/traits/mod.rs (EDIT)
// BEFORE
pub mod text_transformer;
pub use text_transformer::TextTransformer;

// AFTER
pub mod cls_transform_context;
pub mod text_transformer;
pub use cls_transform_context::TransformContext;
pub use text_transformer::TextTransformer;
```

Update the `prelude.rs` re-export the same way (search for `TextTransformer` in `crates/tui-vfx-content/src/prelude.rs` — add `TransformContext` alongside it).

---

## 4. Migration impact (concrete file inventory)

| Category | Count | Files |
|---|---|---|
| New file | 1 | `crates/tui-vfx-content/src/traits/cls_transform_context.rs` |
| Trait file | 1 | `crates/tui-vfx-content/src/traits/text_transformer.rs` |
| Trait re-export | 2 | `crates/tui-vfx-content/src/traits/mod.rs`, `crates/tui-vfx-content/src/prelude.rs` |
| Transformer impls | 15 | `crates/tui-vfx-content/src/transformers/cls_*.rs` |
| Transformer inline tests | 11 | the same `cls_*.rs` files (`#[cfg(test)]` blocks) |
| Inherent ContentEffect API | 1 | `crates/tui-vfx-content/src/types/fnc_apply_content_effect.rs` |
| Typewriter inherent method | 1 | `crates/tui-vfx-content/src/transformers/cls_typewriter.rs` (`transform_with_cursor` at line 116) |
| Lib doctest | 1 | `crates/tui-vfx-content/src/lib.rs` (line ~51) |
| Standalone integration tests | ~12 | `crates/tui-vfx-content/tests/transformers/test_cls_*.rs` + `tests/test_content_effect_apply.rs` + `tests/transformers/test_typewriter_transform_with_cursor.rs` |
| Compositor cursor integration test | 1 | `crates/tui-vfx-compositor/tests/cursor_integration.rs` (calls `transform_with_cursor`) |
| Production caller in gt-design | 2 | `gt-design/crates/gtd-ratatui/src/recipes/render.rs:119–130`, `gt-design/crates/gtd-ratatui/src/text_effects/mod.rs:166–184` |
| Recipe migration | 1 | `tui-vfx-recipes/recipes/debug_recipes/content/content_odometer_3x3_count_bindable.json` |
| Documentation | 3+ | `docs/CAPABILITIES_REFERENCE.md`, `docs/templates/api_docs.toml` (Odometer ai_hint), `docs/design/tui-vfx-v3-schema-draft.json` |

**~40 files, ~160 call-site edits.** Most are mechanical (rename `signal_ctx` to `ctx.signal_ctx`).

---

## 5. Phased implementation

The phases are a strict sequence — Phase A must compile clean before Phase B starts, and so on. A failed phase verification is a hard stop; do not paper over with `--allow-dirty`.

### Phase A — TransformContext + trait change + transformer mechanical sweep

**Goal:** the workspace compiles with the new trait shape, all 15 transformer impls migrated, all inline `#[cfg(test)]` blocks updated, the inherent `ContentEffect::apply_*` family updated, and the doctest in `lib.rs` updated.

#### A.1 Add `cls_transform_context.rs`

Write the full file content from §3.1 above. Verify it compiles in isolation:

```bash
cargo build -p tui-vfx-content
```

Expect: a torrent of trait-mismatch errors from the 15 transformer impls (because the trait file hasn't changed yet — the new struct just exists alongside). That is fine. Move to A.2.

Actually, do NOT do A.1 alone. The intermediate state where the struct exists but the trait still uses `signal_ctx` produces noise. Do A.1 + A.2 + A.3 in one editor session and compile after the full sweep. The verification at A.6 is the gate.

#### A.2 Edit `traits/mod.rs` and `prelude.rs`

Add the `cls_transform_context` module and re-export per §3.3. Five lines of edits total.

#### A.3 Edit `traits/text_transformer.rs`

Replace the file content with the §3.2 version. The version bump goes from 2.0.0 to 3.0.0 (BREAKING — it is a public trait signature change).

#### A.4 Sweep all 15 transformer impls

For each transformer in `crates/tui-vfx-content/src/transformers/cls_*.rs`, the trait-impl signature changes the same way. The mechanical pattern (using `cls_marquee.rs:33–39` as the prototypical example):

**Before** (`cls_marquee.rs`):

```rust
impl TextTransformer for Marquee {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        // ...
        let speed = f64::from(
            self.speed
                .evaluate(progress, signal_ctx)
                .unwrap_or(1.0)
                .max(0.0),
        );
        // ...
    }
}
```

**After**:

```rust
impl TextTransformer for Marquee {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        // ...
        let speed = f64::from(
            self.speed
                .evaluate(progress, ctx.signal_ctx)
                .unwrap_or(1.0)
                .max(0.0),
        );
        // ...
    }
}
```

Plus an import update at the top of every transformer file:

**Before**:

```rust
use mixed_signals::prelude::SignalContext;
```

**After**:

```rust
use crate::traits::TransformContext;
```

(In some files `SignalContext` is also used outside the impl — leave it imported alongside.)

**For transformers that ignore `signal_ctx` today** (e.g. Odometer at `cls_odometer.rs:64` uses `_signal_ctx`), the rename becomes `_ctx: &TransformContext<'_>` for now. Phase C upgrades Odometer specifically to read `ctx.runtime_params`.

**Update each file's metadata header.** Bump the patch or minor version (transformer impls: typically MINOR because the parameter type changed; PATCH if the body is a pure rename), and update WCTX/CLOG. Example for `cls_marquee.rs` (current version 1.x.y):

```rust
// <CLOG>x.y+1.0: TextTransformer signature now takes &TransformContext<'_>; reads `ctx.signal_ctx` for speed-signal evaluation.</CLOG>
```

#### A.5 Update inline `#[cfg(test)]` blocks in source files

11 source files have `#[cfg(test)]` modules calling `.transform()` directly. The mechanical pattern (using `cls_dissolve.rs:286` as prototype):

**Before**:

```rust
let result = dissolve.transform("Hello", 0.0, &SignalContext::default());
```

**After (option 1: explicit struct construction)**:

```rust
let sig = SignalContext::default();
let params = ShaderRuntimeParams::new();
let ctx = TransformContext::new(&sig, &params);
let result = dissolve.transform("Hello", 0.0, &ctx);
```

That is verbose for 110+ call sites. Use a test helper instead:

**After (option 2: helper, recommended)**. Add to the top of each file's `#[cfg(test)] mod tests { … }`:

```rust
use crate::traits::TransformContext;
use mixed_signals::prelude::SignalContext;
use tui_vfx_style::traits::ShaderRuntimeParams;

// Test-only helper bound to the test scope. Avoids 100+ verbose
// constructions of an empty TransformContext across the inline test
// modules. Owns the borrows so the returned context lives for the
// expression's evaluation.
fn empty_ctx() -> (SignalContext, ShaderRuntimeParams) {
    (SignalContext::default(), ShaderRuntimeParams::new())
}
```

Then call sites become:

```rust
let (sig, params) = empty_ctx();
let ctx = TransformContext::new(&sig, &params);
let result = dissolve.transform("Hello", 0.0, &ctx);
```

Or, for one-liner sites:

```rust
let (sig, params) = (SignalContext::default(), ShaderRuntimeParams::new());
let result = dissolve.transform("Hello", 0.0, &TransformContext::new(&sig, &params));
```

**Pick one form per file and apply it consistently.** The reviewer should not have to scan multiple styles. The `(sig, params)` tuple form is the lowest-noise option for files with many call sites; a freestanding helper is fine for the largest (`cls_split_flap.rs` at 55 call sites).

**For tests that previously constructed a non-default `SignalContext`** (e.g. for signal-driven tests) — leave the `signal_ctx` construction as-is, just route it through `TransformContext::new(&signal_ctx, &params)`. Do not collapse those.

#### A.6 Update `fnc_apply_content_effect.rs`

The inherent `ContentEffect::apply` family is the public ergonomic surface. The migration:

**Before** (current, lines 22–80):

```rust
impl ContentEffect {
    pub fn apply(&self, target: &str, progress: f64) -> String {
        self.apply_to_borrowed(target, progress).into_owned()
    }

    pub fn apply_to_borrowed<'a>(&self, target: &'a str, progress: f64) -> Cow<'a, str> {
        let ctx = SignalContext::default();
        self.apply_with_context(target, progress, &ctx)
    }

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
```

**After**. The body of `apply_with_context` constructs a `TransformContext` with an empty `ShaderRuntimeParams`. A new `apply_with_runtime` (or extended `apply_with_context`) method is added so consumers that have a `ShaderRuntimeParams` can pass it through:

```rust
use crate::traits::TransformContext;
use tui_vfx_style::traits::ShaderRuntimeParams;

impl ContentEffect {
    pub fn apply(&self, target: &str, progress: f64) -> String {
        self.apply_to_borrowed(target, progress).into_owned()
    }

    pub fn apply_to_borrowed<'a>(&self, target: &'a str, progress: f64) -> Cow<'a, str> {
        let sig = SignalContext::default();
        let params = ShaderRuntimeParams::new();
        let ctx = TransformContext::new(&sig, &params);
        let transformer = get_transformer(self);
        transformer.transform(target, progress, &ctx)
    }

    /// Advanced: apply the effect with a caller-supplied [`SignalContext`].
    /// The runtime-params map defaults to empty. For host-injected binding
    /// resolution (font names, asset names, etc.) use
    /// [`apply_with_runtime`](Self::apply_with_runtime).
    pub fn apply_with_context<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        let params = ShaderRuntimeParams::new();
        let ctx = TransformContext::new(signal_ctx, &params);
        let transformer = get_transformer(self);
        transformer.transform(target, progress, &ctx)
    }

    /// Advanced: apply the effect with a caller-supplied context bundle —
    /// signals **and** runtime parameters. This is the entry point for
    /// host-injected binding resolution.
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
```

**Bump the file's version to 2.0.0** (MINOR additive: new public method, plus internal signature change to the trait). Update the header CLOG accordingly.

The doctests in this file (`apply` example at lines 32–41) do not call `.transform()` directly — they exercise `apply` — so they stay green automatically.

#### A.7 Update `cls_typewriter.rs::transform_with_cursor`

`transform_with_cursor` is an inherent method (not a trait method) at `cls_typewriter.rs:116`. It internally calls `self.transform(target, progress, signal_ctx)` at line 127. Two options:

**Option 1: change `transform_with_cursor` to take a `TransformContext`** (matches the trait shape, ripples to its callers).

**Option 2: keep `transform_with_cursor` taking `signal_ctx`, construct an empty `TransformContext` inside.**

**Choose Option 1.** Rationale: `transform_with_cursor` is part of a slightly different ergonomic surface but its callers (the cursor integration test and external consumers) should see the same context pattern as the trait. Inconsistency here is what surfaces as "why does this method take signal_ctx when transform takes ctx" in code review six months from now.

**Before** (current lines 116–125):

```rust
#[allow(clippy::too_many_arguments)]
pub fn transform_with_cursor<'a>(
    &self,
    target: &'a str,
    progress: f64,
    signal_ctx: &SignalContext,
    cursor: &Cursor,
    state: &mut CursorState,
    now: f64,
    dt: f64,
) -> (Cow<'a, str>, CursorPaintOps) {
    let revealed = self.transform(target, progress, signal_ctx);
    // ...
    fnc_advance_cursor(state, cursor, Some(pos), now, dt, signal_ctx);
    let ops = fnc_render_cursor(state, cursor, now, signal_ctx);
    // ...
}
```

**After**:

```rust
#[allow(clippy::too_many_arguments)]
pub fn transform_with_cursor<'a>(
    &self,
    target: &'a str,
    progress: f64,
    ctx: &TransformContext<'_>,
    cursor: &Cursor,
    state: &mut CursorState,
    now: f64,
    dt: f64,
) -> (Cow<'a, str>, CursorPaintOps) {
    let revealed = self.transform(target, progress, ctx);
    // ...
    fnc_advance_cursor(state, cursor, Some(pos), now, dt, ctx.signal_ctx);
    let ops = fnc_render_cursor(state, cursor, now, ctx.signal_ctx);
    // ...
}
```

**Test sites that call `transform_with_cursor`:**

- `crates/tui-vfx-content/tests/transformers/test_typewriter_transform_with_cursor.rs` — 4–5 sites (`tw.transform_with_cursor("hello", 0.5, &ctx(), &cursor, …)` becomes `tw.transform_with_cursor("hello", 0.5, &TransformContext::new(&sig, &params), &cursor, …)`).
- `crates/tui-vfx-compositor/tests/cursor_integration.rs:41,61,63` — same migration.

#### A.8 Update the lib.rs doctest

`crates/tui-vfx-content/src/lib.rs:51` has a doctest:

```rust
//! let tx = Typewriter::default();
//! let signal_ctx = SignalContext::default();
//! let output = tx.transform("Hello World", 0.5, &signal_ctx);
```

**After**:

```rust
//! let tx = Typewriter::default();
//! let signal_ctx = SignalContext::default();
//! let runtime_params = ShaderRuntimeParams::new();
//! let ctx = TransformContext::new(&signal_ctx, &runtime_params);
//! let output = tx.transform("Hello World", 0.5, &ctx);
```

Also add `use tui_vfx_style::traits::ShaderRuntimeParams;` to the doctest. Confirm `tui-vfx-style` is in the dev-deps as a doctest dep (it should be, via the existing transitive workspace setup; if `cargo test --doc` errors on the import, add it explicitly to `tui-vfx-content/Cargo.toml`'s `[dev-dependencies]`).

#### A.9 Phase A verification

Run, in order, and confirm each passes before moving on:

```bash
cargo build -p tui-vfx-content
```

Expect: clean build, zero warnings introduced by this change.

```bash
cargo test -p tui-vfx-content --lib 2>&1 | tail -3
```

Expect: `test result: ok. 357 passed; 0 failed; 0 ignored;` (356 prior + 1 new in the new TransformContext file). Compare against the baseline you captured at §0.10.

```bash
cargo test -p tui-vfx-content 2>&1 | tail -10
```

Expect: every test binary passes. Lib tests: 357. Each `tests/transformers/test_cls_*.rs` file passes its prior count. `tests/test_content_effect_apply.rs` passes its 3 tests.

```bash
cargo test --doc -p tui-vfx-content 2>&1 | tail -3
```

Expect: doctests pass. The lib.rs doctest (with the updated context construction) is the canary.

```bash
cargo build --workspace
```

Expect: clean. The trait change only affects `tui-vfx-content` directly; the workspace build catches drift in `tui-vfx`'s re-export, `xtask`, and the compositor's dev-dep usage in `cursor_integration.rs`.

```bash
ofpf-blast crates/tui-vfx-content/src/traits/text_transformer.rs
```

Expect: roughly the same blast set as §0.5, plus the new `cls_transform_context.rs` showing up in the trait module's re-export chain. Any unexpected drift means a new caller landed; reconcile.

If any of these fail, **stop and fix before Phase B.** Phase A is the load-bearing phase — the next phases assume the workspace builds.

### Phase B — Production caller plumbing in gt-design

**Goal:** the live render path threads `ShaderRuntimeParams` into `transformer.transform(...)` so host-supplied bindings flow end-to-end.

**Note:** this phase modifies code in the **gt-design** sibling repo (`/usr/projects/gt-design/`), not tui-vfx. Do not commit gt-design changes from a tui-vfx working directory.

#### B.1 Map the call sites

```bash
grep -rn -E "(\.apply\(|apply_with_context|apply_to_borrowed|TextTransformer|get_transformer)" /usr/projects/gt-design/crates/
```

Expected hits:

- `gt-design/crates/gtd-ratatui/src/recipes/render.rs:13` — `use tui_vfx_content::transformers::fnc_get_transformer::get_transformer;`
- `gt-design/crates/gtd-ratatui/src/recipes/render.rs:119` — `let transformer = get_transformer(effect);`
- `gt-design/crates/gtd-ratatui/src/recipes/render.rs:130` — `transformer.transform(message, effective_t, signal_ctx).to_string()`
- `gt-design/crates/gtd-ratatui/src/text_effects/mod.rs:115,118,168` — facade impl path

Read `recipes/render.rs` lines 109–135 (`resolve_recipe_message`) end-to-end before editing.

#### B.2 Edit `recipes/render.rs`

The function `resolve_recipe_message` currently receives `signal_ctx: &SignalContext`. The render-path caller above this function holds (or can hold) a `ShaderRuntimeParams` reference — this is the host-injection map. Add a parameter:

**Before** (lines 109–135):

```rust
pub(crate) fn resolve_recipe_message(
    message: &str,
    effect: Option<&tui_vfx_content::types::ContentEffect>,
    mode: tui_vfx_recipes::recipe_schema::config::RaContentMode,
    t: f64,
    loop_t: Option<f64>,
    signal_ctx: &SignalContext,
    phase: AnimationPhase,
) -> String {
    if let Some(effect) = effect {
        let transformer = get_transformer(effect);
        let effective_t = match mode { /* ... */ };
        transformer
            .transform(message, effective_t, signal_ctx)
            .to_string()
    } else {
        message.to_owned()
    }
}
```

**After**:

```rust
use tui_vfx_content::traits::TransformContext;
use tui_vfx_style::traits::ShaderRuntimeParams;

pub(crate) fn resolve_recipe_message(
    message: &str,
    effect: Option<&tui_vfx_content::types::ContentEffect>,
    mode: tui_vfx_recipes::recipe_schema::config::RaContentMode,
    t: f64,
    loop_t: Option<f64>,
    signal_ctx: &SignalContext,
    runtime_params: &ShaderRuntimeParams,
    phase: AnimationPhase,
) -> String {
    if let Some(effect) = effect {
        let transformer = get_transformer(effect);
        let effective_t = match mode { /* ... unchanged ... */ };
        let ctx = TransformContext::new(signal_ctx, runtime_params);
        transformer
            .transform(message, effective_t, &ctx)
            .to_string()
    } else {
        message.to_owned()
    }
}
```

Now find the callers of `resolve_recipe_message`:

```bash
grep -rn "resolve_recipe_message" /usr/projects/gt-design/crates/
```

For each caller, thread a `&ShaderRuntimeParams`. The natural source is the recipe playback item's resolved runtime-params map — search for `ShaderRuntimeParams` in the gt-design call chain and pass it through. If a caller does not yet have access to runtime params (e.g. legacy code paths), pass `&ShaderRuntimeParams::new()` for now and file a follow-up — that is L3+ territory per the loopback-implementation plan.

#### B.3 Edit `text_effects/mod.rs::ApplyTextEffect`

This is the gtd-ratatui ergonomic facade. It does not currently know about runtime params and most consumers do not care. The migration here is minimal:

**Before** (lines 166–173):

```rust
impl ApplyTextEffect for ContentEffect {
    fn apply(&self, text: &str, progress: f32) -> String {
        let transformer = get_transformer(self);
        let ctx = SignalContext::default();
        transformer
            .transform(text, progress as f64, &ctx)
            .into_owned()
    }
    // ...
}
```

**After**:

```rust
use tui_vfx_content::traits::TransformContext;
use tui_vfx_style::traits::ShaderRuntimeParams;

impl ApplyTextEffect for ContentEffect {
    fn apply(&self, text: &str, progress: f32) -> String {
        let transformer = get_transformer(self);
        let sig = SignalContext::default();
        let params = ShaderRuntimeParams::new();
        let ctx = TransformContext::new(&sig, &params);
        transformer
            .transform(text, progress as f64, &ctx)
            .into_owned()
    }
    // ...
}
```

Update the rustdoc paragraph at lines 126–140 to mention that the facade default-constructs an empty `ShaderRuntimeParams` and that consumers needing host-injected bindings should bypass the facade and call `get_transformer` + `TransformContext::new(&sig, &params)` with their own params.

The `apply_text` and `apply_styled` methods just delegate to `apply`, so no change.

Bump the file's MINOR version (signature/behavior visible in rustdoc changed) and update the header.

#### B.4 Phase B verification

```bash
cd /usr/projects/gt-design && cargo build --workspace
```

Expect: clean build.

```bash
cd /usr/projects/gt-design && cargo test --workspace 2>&1 | tail -10
```

Expect: every test passes. The `test_text_effects_facade.rs` file in gt-design exercises `effect.apply(...)` 30+ times — the facade signature is unchanged, so these stay green automatically. If they fail, the migration introduced a behavior change that should not have happened; revert and re-check.

```bash
cd /usr/projects/tui-vfx && cargo build --workspace
```

Expect: clean. Defensive check that the gt-design changes did not require any further tui-vfx-side migration that was missed.

### Phase C — `cls_odometer.rs` reads ctx.runtime_params

**Goal:** Odometer's `transform` is now reading `ctx.runtime_params` and routing through `resolve_mechanical_cycle_with_context`. Two new integration tests prove the round-trip.

#### C.1 Edit `cls_odometer.rs`

The trait-impl signature already changed in Phase A (Odometer's `_signal_ctx` became `_ctx: &TransformContext<'_>`). Now drop the underscore on `ctx` and route it into the cycle resolver.

**Before** (lines 59–80, post-Phase-A):

```rust
impl TextTransformer for Odometer {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let Some(tile) = MechanicalTile::new(self.tile_width, self.tile_height) else {
            return Cow::Borrowed(target);
        };
        match self.mechanical.as_ref() {
            None => self.roll_legacy_pair(target, progress, tile),
            Some(cfg) if is_legacy_equivalent(cfg) => self.roll_legacy_pair(target, progress, tile),
            Some(cfg) => self.roll_cycle(target, progress, tile, cfg),
        }
    }
}
```

**After**:

```rust
impl TextTransformer for Odometer {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let Some(tile) = MechanicalTile::new(self.tile_width, self.tile_height) else {
            return Cow::Borrowed(target);
        };
        match self.mechanical.as_ref() {
            None => self.roll_legacy_pair(target, progress, tile),
            Some(cfg) if is_legacy_equivalent(cfg) => self.roll_legacy_pair(target, progress, tile),
            Some(cfg) => self.roll_cycle(target, progress, tile, cfg, ctx.runtime_params),
        }
    }
}
```

Then change `roll_cycle`'s signature and switch to the `_with_context` resolver:

**Before** (line 98 + line 118):

```rust
fn roll_cycle<'a>(
    &self,
    target: &'a str,
    progress: f64,
    tile: MechanicalTile,
    cfg: &MechanicalCycleConfig,
) -> Cow<'a, str> {
    // ...
    let Ok(cycle) = resolve_mechanical_cycle(&cfg.source, tile) else {
        return self.roll_legacy_pair(target, progress, tile);
    };
    // ...
}
```

**After**:

```rust
fn roll_cycle<'a>(
    &self,
    target: &'a str,
    progress: f64,
    tile: MechanicalTile,
    cfg: &MechanicalCycleConfig,
    runtime_params: &ShaderRuntimeParams,
) -> Cow<'a, str> {
    // ...
    let registry = FontRegistry::new();  // or accept as param if a registry is plumbed; see note below
    let Ok(cycle) = resolve_mechanical_cycle_with_context(
        &cfg.source,
        tile,
        &registry,
        runtime_params,
    ) else {
        return self.roll_legacy_pair(target, progress, tile);
    };
    // ...
}
```

Update imports at the top of the file:

**Before**:

```rust
use crate::mechanical::{
    blit_tile_grid, extract_tile_text, grid_from_text, grid_to_text, overshoot_face_for,
    paired_grids, resolve_mechanical_cycle, roll_cycle_window, roll_grid_window, route_between,
    settle_sample_for, tile_progress_for, tile_rects, MechanicalSizing, MechanicalSource,
    MechanicalTile, NumericRouteHint, TileScheduleMeta,
};
```

**After**:

```rust
use crate::fonts::FontRegistry;
use crate::mechanical::{
    blit_tile_grid, extract_tile_text, grid_from_text, grid_to_text, overshoot_face_for,
    paired_grids, resolve_mechanical_cycle_with_context, roll_cycle_window, roll_grid_window,
    route_between, settle_sample_for, tile_progress_for, tile_rects, MechanicalSizing,
    MechanicalSource, MechanicalTile, NumericRouteHint, TileScheduleMeta,
};
use tui_vfx_style::traits::ShaderRuntimeParams;
```

The plain `resolve_mechanical_cycle` import is dropped; only the `_with_context` variant is now used from this file.

**Note on `FontRegistry::new()`.** Constructing a fresh `FontRegistry` per `roll_cycle` call is correct for now — the registry's `new()` only registers the embedded Line 3x3 default and is not on a per-cell hot path. If profiling shows this as a real cost, plumb a registry reference through the `Odometer` builder and into `transform`; that is a follow-up, not part of this slice.

Bump the file from 4.0.0 to 4.1.0 (MINOR — internal routing change, no public API change). Update CLOG.

#### C.2 Add two integration tests

In `crates/tui-vfx-content/tests/transformers/test_cls_odometer_cycles.rs` (already exists; current version 0.1.0). Add two tests at the bottom of the file:

```rust
#[test]
fn binding_form_font_resolves_via_runtime_params() {
    use tui_vfx_content::fonts::FontRegistry;
    use tui_vfx_content::traits::TransformContext;
    use tui_vfx_style::models::BindableString;
    use tui_vfx_style::traits::{ShaderRuntimeParams, ShaderRuntimeParamValue};

    // Construct an Odometer with a Preset { decimal_digits } source whose
    // font is a Binding to "drum_font". We expect the runtime-params map
    // to resolve "drum_font" -> "line-3x3", which routes through the font
    // registry's Line 3x3 face. The resulting transform output, at progress
    // 0.5, must NOT be the literal-glyph fallback (which is the
    // ASCII-faces-without-glyph-expansion case).
    let cfg = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: Some(BindableString::Binding("drum_font".to_string())),
        },
        route: forward_route(),
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::None,
    };
    let effect = ContentEffect::Odometer {
        direction: OdometerDirection::Up,
        travel: OdometerTravel::Axis,
        tile_width: 3,
        tile_height: 3,
        from_message: Some("┏━┓\n┃ ┃\n┗━┛".to_string()), // 0 in line-3x3
        mechanical: Some(cfg),
    };

    let mut params = ShaderRuntimeParams::new();
    params.insert(
        "drum_font".to_string(),
        ShaderRuntimeParamValue::Text("line-3x3".to_string()),
    );
    let sig = SignalContext::default();
    let ctx = TransformContext::new(&sig, &params);

    let tx = get_transformer(&effect);
    let target = "╺┓ \n ┃ \n╺┻╸"; // 1 in line-3x3
    let out = tx.transform(target, 0.5, &ctx);

    // Output is mid-roll between line-3x3 "0" and "1" — should be a 3-row,
    // 3-col grid of box-drawing characters, not raw ASCII digits.
    assert!(out.contains('━') || out.contains('┓') || out.contains('┃'),
        "expected line-3x3 box-drawing glyphs in mid-roll output, got {:?}", out);
}

#[test]
fn binding_form_font_falls_back_to_default_when_runtime_param_missing() {
    use tui_vfx_content::traits::TransformContext;
    use tui_vfx_style::models::BindableString;
    use tui_vfx_style::traits::ShaderRuntimeParams;

    // Same effect as the previous test but with an empty runtime-params
    // map. Per Intention 36, a missing binding falls back to the registry
    // default (which is line-3x3). The output should match the
    // host-supplied case byte-for-byte.
    let cfg = MechanicalCycleConfig {
        source: MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: Some(BindableString::Binding("drum_font".to_string())),
        },
        route: forward_route(),
        cascade: MechanicalCascadePolicy::Simultaneous,
        settle: MechanicalSettleConfig::None,
    };
    let effect = ContentEffect::Odometer {
        direction: OdometerDirection::Up,
        travel: OdometerTravel::Axis,
        tile_width: 3,
        tile_height: 3,
        from_message: Some("┏━┓\n┃ ┃\n┗━┛".to_string()),
        mechanical: Some(cfg),
    };

    let params = ShaderRuntimeParams::new(); // empty
    let sig = SignalContext::default();
    let ctx = TransformContext::new(&sig, &params);

    let tx = get_transformer(&effect);
    let out = tx.transform("╺┓ \n ┃ \n╺┻╸", 0.5, &ctx);
    assert!(out.contains('━') || out.contains('┓') || out.contains('┃'),
        "expected default-font fallback to produce line-3x3 glyphs, got {:?}", out);
}
```

The two tests together prove the round-trip: when the host supplies a value, it reaches the transformer; when it does not, the registry default fallback is preserved.

**Note on test brittleness.** These tests assert glyph-presence rather than exact byte equality — exact midpoint output of a roll is sensitive to many parameters (direction, travel, tile size). If the cycle resolver is later refactored to change midpoint composition, these assertions will still hold. Do not tighten to byte equality unless a stable golden master is available.

Bump `test_cls_odometer_cycles.rs` from 0.1.0 to 0.2.0. Update CLOG.

#### C.3 Phase C verification

```bash
cargo build -p tui-vfx-content
cargo test -p tui-vfx-content --lib 2>&1 | tail -3
cargo test -p tui-vfx-content --test test_transformers 2>&1 | tail -3
cargo test -p tui-vfx-content 2>&1 | tail -3
cargo test --doc -p tui-vfx-content 2>&1 | tail -3
```

Expect:
- lib tests: 357 (no change from Phase A; the new tests are integration tests).
- `test_transformers` binary count rises by 2 over the prior baseline.
- workspace clean.

### Phase D — Recipe migration

**Goal:** the bindable odometer recipe declares its binding and references the font via `{"binding": "drum_font"}` shape.

**Note:** this phase modifies code in the **tui-vfx-recipes** sibling repo (`/usr/projects/tui-vfx-recipes/`).

#### D.1 Edit `content_odometer_3x3_count_bindable.json`

**Before** (lines 59–65, current shape):

```json
"mechanical": {
  "source": {
    "type": "preset",
    "preset": "decimal_digits",
    "wrap": "circular",
    "font": "line-3x3"
  },
```

**After**. Add a top-level `requires_bindings` block (sibling at the same nesting level as `config` in the recipe envelope) and convert the font reference to binding shape:

```json
{
  "schema_version": 3,
  "id": "debug.content.odometer_3x3_count",
  // ... existing top-level fields ...
  "requires_bindings": {
    "drum_font": {
      "type": "string",
      "description": "Font name resolved by the player at runtime; defaults to the registered default font when host does not override.",
      "default": "default_font"
    }
  },
  "config": {
    // ... existing config ...
    "content": {
      "mode": "enter_only",
      "effect": {
        "type": "odometer",
        // ...
        "mechanical": {
          "source": {
            "type": "preset",
            "preset": "decimal_digits",
            "wrap": "circular",
            "font": { "binding": "drum_font" }
          },
          // ...
        }
      }
    }
  }
}
```

**Default value rationale.** The default `"default_font"` is the reserved sentinel that `FontRegistry` routes to its currently-registered default (line-3x3 in the embedded shipping case). This keeps the loopback-only behavior identical to the current literal `"line-3x3"`.

Update the recipe's `version` to `0.4.0` and `last_updated` to today's date. Update `metadata.authoring_notes` to remove the sentence about "until sibling's L2 work types the asset-reference shape" — that is now done.

Note the leading lines of the recipe file may carry `#`-prefixed annotation lines (per `tui-vfx-v3-schema-draft.json` convention). Plain JSON recipes do not — leave them off.

#### D.2 Phase D verification

```bash
cd /usr/projects/tui-vfx-recipes && cargo test --test test_debug_recipes_qc 2>&1 | tail -10
```

Expect: every recipe in `recipes/debug_recipes/` validates against the strict-contracts validator. The migrated recipe specifically must:
- Pass `requires_bindings` shape validation (sibling's L2 contract).
- Have its `{"binding": "drum_font"}` reference matched against `requires_bindings.drum_font` (sibling's L2 reference-validity gate).
- Loopback-pre-fill the runtime-params map with `"drum_font" -> Text("default_font")` when the host doesn't override.

If the validator rejects the recipe, surface the exact error message and reconcile against `tui-vfx-recipes/docs/design/tui-vfx-binding-loopback-implementation-plan.md` § L2. Do not work around — the failure indicates the binding-shape contract is being misread.

```bash
cd /usr/projects/tui-vfx-recipes && cargo test --workspace 2>&1 | tail -10
```

Defensive: confirm no other recipes test breaks.

### Phase E — Documentation + cycle plan v0.8.0

**Goal:** the public docs reflect the binding-form authoring shape; the cycle plan marks Slice 6.6 complete.

#### E.1 Update `docs/CAPABILITIES_REFERENCE.md`

Find the Mechanical Cycle Config section (search for `mechanical` heading). Add a "Binding-form authoring" subsection demonstrating the recipe shape from D.1:

```markdown
### Binding-form font references (V3, since Slice 6.6)

The `font` field on `mechanical.source` (Preset variant) accepts both literal
strings and `{"binding": "name"}` references. Binding-form references resolve
through the host-supplied `ShaderRuntimeParams` map at render time, which the
recipe player populates via the recipe envelope's `requires_bindings` block.

Recipe example:

    {
      "requires_bindings": {
        "drum_font": {
          "type": "string",
          "description": "Font name resolved at runtime",
          "default": "default_font"
        }
      },
      "config": {
        "content": {
          "effect": {
            "type": "odometer",
            "mechanical": {
              "source": {
                "type": "preset",
                "preset": "decimal_digits",
                "font": { "binding": "drum_font" }
              }
            }
          }
        }
      }
    }

The reserved `"default_font"` sentinel routes to the registry's currently-
registered default (Line 3x3 in the embedded shipping case). When the host
does not supply a value, the loopback layer pre-fills the binding from
`default`, so the recipe is preview-playable without host wiring per
Intention 37.
```

#### E.2 Update `docs/templates/api_docs.toml`

Search for the Odometer ai_hint block (`[specs.ContentEffect.…]` Odometer section, around line 605–700). Add or update a sentence noting the font field is bindable. Keep the existing ai_hint prose; add one paragraph at the end of the relevant `description` field:

> "The `mechanical.source.font` field on Preset accepts both literal strings (e.g. `\"line-3x3\"`) and binding references (`{\"binding\": \"drum_font\"}`); see CAPABILITIES_REFERENCE for the binding-form authoring shape."

#### E.3 Update `docs/design/tui-vfx-v3-schema-draft.json`

Find the existing `mechanical.source.font` schema annotation (search for `font` in that file). Add a `# Binding-form: { "binding": "name" } resolves through requires_bindings.` annotation line above the field. The schema-draft JSON uses `#`-prefixed annotation lines; do not break existing comment formatting.

#### E.4 Run docs generation

```bash
just docs-all
```

Expect: `cargo xtask docs generate` succeeds, regenerates the capability manifest from rustdoc + `docs/templates/capabilities.toml`, and the updated Odometer ai_hint flows through to the generated `CAPABILITIES_REFERENCE.md` artifact.

```bash
just docs-all-validate
```

Expect: docs-freshness check passes — no rustdoc/template drift. If this fails, the generated artifact and the source-of-truth template are out of sync; reconcile by re-running the generator.

#### E.5 Update the cycle plan

Edit `docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md`:

- Change Slice 6.6's status in the table at line ~20 from `**Deferred (architectural)**` to `**Done**` with the commit hash that closes it.
- Mark Phase 6 fully complete (no remaining deferrals).
- Bump the file's metadata version to 0.8.0.
- Update the CLOG to reflect the Slice 6.6 close.

#### E.6 Phase E verification

```bash
cd /usr/projects/tui-vfx && just docs-all && just docs-all-validate
```

Expect: both pass. If `docs-all-validate` reports drift, re-run `just docs-all` and re-check; drift typically means `xtask` regenerated a slightly different artifact than what was committed.

```bash
cd /usr/projects/tui-vfx && cargo build --workspace && cargo test --workspace 2>&1 | tail -10
```

Expect: workspace-wide green.

```bash
cd /usr/projects/tui-vfx-recipes && cargo test --workspace 2>&1 | tail -10
cd /usr/projects/gt-design && cargo test --workspace 2>&1 | tail -10
```

Expect: green across all three repos. End-to-end gate passed.

---

## 6. Risk inventory (with concrete mitigations)

| Risk | Concrete mitigation | Verification |
|---|---|---|
| **Stale ofpf graph** misreports the trait blast set, leading to a missed transformer impl. | Run `ofpf-status` at the start of Phase A; if `is_stale: true`, run `ofpf-load` before continuing. Re-run `ofpf-blast` after Phase A completes and reconcile against the §0.5 baseline. | `ofpf-blast crates/tui-vfx-content/src/traits/text_transformer.rs` returns the same set as §0.5 plus the new TransformContext file. |
| **Doctest in lib.rs** is missed and `cargo test` summary shows green while doctests fail silently. | Always run `cargo test --doc -p tui-vfx-content` explicitly after Phase A. Mentioned in §A.9 verification. | `cargo test --doc -p tui-vfx-content 2>&1 \| tail -3` shows `0 failed`. |
| **`#[cfg(test)]` blocks in source files** account for ~110 of the ~160 call sites; missing a few produces compile errors that are easy to confuse with the trait change itself. | Use `grep -rc "\.transform(" /usr/projects/tui-vfx/crates/tui-vfx-content/src/` before and after; the after-count should be the same number of call sites, with each one calling `&ctx` instead of `&signal_ctx`. | Per-file count post-Phase-A matches the baseline in §0.7 and `cargo build -p tui-vfx-content` is clean. |
| **Per-frame allocation regression.** A naive implementation could construct a fresh `ShaderRuntimeParams` or `FontRegistry` inside `transform` per call. | Phase C explicitly notes that `FontRegistry::new()` is cheap-but-not-free and is the only allocation-introducing part of the slice; the `TransformContext` struct itself is `Copy` and zero-cost. Do not construct a `TransformContext` inside any per-cell or per-tile loop — it is constructed once per `transform()` invocation. | Spot-check via `ofpf-around crates/tui-vfx-content/src/transformers/cls_odometer.rs "TransformContext::new"` post-Phase-C — there should be zero hits inside `roll_cycle`'s tile loop. |
| **`SignalContext` lifetime / `Send + Sync`** bounds on `Box<dyn TextTransformer>` may interact awkwardly with the `'a` on `TransformContext`. | The trait method takes `ctx: &TransformContext<'_>` (HRTB-style elided lifetime), which is independent of the trait object's bounds. The dispatcher `Box<dyn TextTransformer>` does not change. | Phase A `cargo build -p tui-vfx-content` succeeds without trait-object lifetime warnings. |
| **gt-design caller threading** — the `resolve_recipe_message` chain may not yet have access to `ShaderRuntimeParams` at the relevant call site. | If a caller cannot supply a real `ShaderRuntimeParams`, pass `&ShaderRuntimeParams::new()` explicitly and document the gap with a `// TODO(slice-6.6-followup):` comment. The empty map preserves the prior behavior. | Phase B `cargo test --workspace` passes in gt-design. |
| **Compositor cursor integration test** drift — `cursor_integration.rs` calls `transform_with_cursor` with the old signature. | A.7 explicitly migrates `transform_with_cursor` to take `&TransformContext`; A.7's test-site list includes `cursor_integration.rs:41,61,63`. | `cargo test --workspace` (Phase A.9) passes. |
| **Recipe validator rejection** of the binding-form recipe due to an L2 contract mismatch. | Read `tui-vfx-recipes/docs/design/tui-vfx-binding-loopback-implementation-plan.md` § L2 first to confirm the `requires_bindings` shape. The expected shape: `{"type": "string", "description": "...", "default": "default_font"}`. | Phase D `cargo test --test test_debug_recipes_qc` passes. |
| **Docs-freshness drift** from `just docs-all-validate` — a regenerated artifact does not match the committed one. | After E.4 succeeds, commit the regenerated artifact alongside the manual edits. | E.6 `just docs-all-validate` passes. |

---

## 7. Out of scope for this slice

- Phase 7 schema lift (`image_name: String → BindableString` on `VfxImageSource`). Independently actionable, but its own slice.
- AssetRegistry / ImagePool / AssetMap consolidation. V3 scene-layer composition territory.
- Rocketsplash routing through the Image source variant. V3 scene-layer composition territory.
- Phase 4 (SplitFlap migration). Separate session per cycle plan v0.5.0+.
- Loopback layer L3-L5 (visibility badge, strictness modes, probe + browser integration). Sibling's territory.
- Renaming `text_transformer.rs` to `cls_text_transformer.rs`. Larger naming-convention bikeshed; not scoped here.

---

## 8. Definition of done

- [ ] All 15 transformer impls compile against the new trait signature.
- [ ] `cargo test -p tui-vfx-content --lib` passes with **357** lib tests (356 baseline + 1 new TransformContext test).
- [ ] `cargo test -p tui-vfx-content` passes with two new integration tests in `tests/transformers/test_cls_odometer_cycles.rs` covering binding-form font resolution (host-supplied + loopback-default).
- [ ] `cargo test --doc -p tui-vfx-content` passes; doctests on `ContentEffect::apply_*` and `lib.rs` updated.
- [ ] `cargo build --workspace` clean.
- [ ] `cargo test --workspace` clean across `tui-vfx`, `tui-vfx-recipes`, and `gt-design`.
- [ ] `cargo test --test test_debug_recipes_qc` in `tui-vfx-recipes` passes after the recipe migration.
- [ ] `just docs-all && just docs-all-validate` succeeds; generated docs reflect the binding-form authoring shape.
- [ ] Cycle plan v0.8.0 committed marking Slice 6.6 complete.
- [ ] No regressions in any other transformer's test suite (per-file counts match the baseline captured at §0.10).
- [ ] `ofpf-blast crates/tui-vfx-content/src/traits/text_transformer.rs` returns the expected set (§0.5) plus the new `cls_transform_context.rs` re-export.

---

## 9. Companions

- `docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md` (v0.7.0 → v0.8.0) — the cycle plan; Slice 6.6 is the row this implementation closes.
- `docs/design/tui-vfx-binding-loopback.md` (v0.3.0) — sibling's design proposal; the WHY of bindings.
- `docs/design/tui-vfx-binding-loopback-implementation-plan.md` — sibling's HOW; L2 just shipped at `7e7e88f` (tui-vfx-recipes) + `66fe546` (tui-vfx).
- `steering/INTENTIONS.md` — Intentions 23, 24, 36, 37, 38 directly informed this approach.
- `feedback_no_inert_schema.md` and `feedback_loopback_required.md` in the leader's memory.

---

## 10. Critical findings (drift from the v0.1.0 draft)

The v0.1.0 pre-review draft had several factual gaps and one architectural misframing that this v1.0.0 corrects. Documenting them here so the version history is honest.

1. **Trait file path was guessed.** The v0.1.0 draft said the trait lives at `crates/tui-vfx-content/src/traits/cls_text_transformer.rs` ("or wherever the trait lives — confirm during plan refinement"). The actual file is `text_transformer.rs` — no `cls_` prefix. The `traits/` directory contains exactly two files: `mod.rs` and `text_transformer.rs`. Confirmed via `ofpf-defs TextTransformer`.

2. **Production callers live in gt-design, not tui-vfx-compositor.** The v0.1.0 draft said "compositor production callers: the V3 content pipeline that invokes `TextTransformer::transform`" and instructed the implementer to map them. The compositor's **src** does not consume `tui-vfx-content` at all — it is a dev-dep, used only by `tests/cursor_integration.rs`. The actual production callers are in **gt-design** at `gtd-ratatui/src/recipes/render.rs:119,130` and `gtd-ratatui/src/text_effects/mod.rs:168`. Phase B is rewritten around this fact.

3. **The inline `#[cfg(test)]` sweep in source files is enormous.** The v0.1.0 draft mentioned "every test file that calls `transform()` directly" and the standalone `tests/transformers/test_cls_*.rs` files. It missed the ~110 inline call sites in `#[cfg(test)]` blocks at the bottom of source files (`cls_split_flap.rs` alone has 55). §A.5 makes this explicit and recommends a per-file helper to keep the noise bounded.

4. **`Typewriter::transform_with_cursor` is its own migration.** The v0.1.0 draft did not surface that `cls_typewriter.rs` has an inherent `transform_with_cursor` method (lines 116–148) that calls the trait `transform` internally and has its own callers (the cursor integration test, doc examples). §A.7 addresses it explicitly with the choice to migrate it to take `&TransformContext` rather than introduce inconsistent surfaces.

5. **The lib.rs doctest is a hidden failure mode.** The v0.1.0 draft mentioned "doctests don't surface in `cargo test` summary the same way" but did not pinpoint that `crates/tui-vfx-content/src/lib.rs:51` has a `let output = tx.transform(...)` doctest that breaks with the trait signature change. §A.8 makes this explicit.

6. **The v0.1.0 §6 risk inventory was generic.** Each risk now ties to a concrete verification command. "Production caller plumbing surprise" is now grounded in actual file paths. "Doctest sweep miss" lists the specific file. "Test ergonomics" lists the specific helper recommendation.

7. **`FontRegistry::new()` allocation inside `roll_cycle`** is a real but acceptable cost. The v0.1.0 draft noted "constructing an empty `ShaderRuntimeParams::default()` per call ... is a known tiny allocation" but missed that the Phase C migration introduces a `FontRegistry::new()` per `roll_cycle` invocation. §C.1 calls this out and notes that plumbing a registry reference is a follow-up if profiling shows it as material.

8. **The v0.1.0 draft did not include shell-ready verification commands.** Each phase now ends with copy-pasteable `cargo` / `ofpf-*` / `just` commands plus expected output shape (test counts, exit codes).

9. **The `apply_with_runtime` method is new in v1.0.0.** The v0.1.0 draft did not specify how the inherent `ContentEffect::apply_*` family extends to expose host-supplied runtime params. §A.6 adds `apply_with_runtime` as the third entry point — `apply` and `apply_with_context` keep their current ergonomics; `apply_with_runtime` is the host-injection path.

---

## 11. Open questions (needing human judgment)

These could not be closed from the codebase alone.

1. **Should `transform_with_cursor` migrate to `&TransformContext` (Option 1 in §A.7) or stay on `&SignalContext` and construct a `TransformContext` internally (Option 2)?** The plan picks Option 1 for consistency, but Option 2 minimizes blast radius (fewer callers to migrate). The Option-1 cost is small (3 test sites in compositor + 4–5 in `test_typewriter_transform_with_cursor.rs`); the consistency gain is durable. Confirm before A.7.

2. **Should `apply_with_context` be deprecated in favor of `apply_with_runtime`?** Today's `apply_with_context` only takes `signal_ctx`; `apply_with_runtime` is strictly more capable. Keeping both is the conservative call (back-compat). Deprecating `apply_with_context` (with a `#[deprecated]` attribute pointing at `apply_with_runtime`) is the cleaner long-term call. The plan keeps both for now; flag if you want a deprecation in this slice.

3. **Should the `FontRegistry` reference be plumbed through Odometer's builder?** §C.1 constructs a fresh `FontRegistry::new()` inside `roll_cycle`. This is correct functionally and not on the per-cell hot path, but it is per-`transform`-call. Plumbing a registry reference (held by the caller, e.g. the recipe player) eliminates that allocation. The plan defers this; flag if you want it in scope.

4. **Recipe `default` sentinel — `"default_font"` vs `"line-3x3"`?** §D.1's recipe migration uses `"default_font"` for `requires_bindings.drum_font.default`. The reserved sentinel routes through the registry's currently-registered default (line-3x3 in the shipping case). Using the literal `"line-3x3"` instead would be more explicit but couples the recipe to a specific font name. The plan picks the sentinel for forward-compatibility; flag if you prefer the literal.

<!-- <FILE>docs/design/tui-vfx-transform-context-implementation-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
