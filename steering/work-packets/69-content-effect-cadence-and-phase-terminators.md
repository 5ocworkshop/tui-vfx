<!-- <FILE>steering/work-packets/69-content-effect-cadence-and-phase-terminators.md</FILE> - <DESC>Design proposal: bindable parity for content effects + a unified PhaseTerminator concept that lets phases end on a duration, on effect-completion, or on a host-supplied binding. Top-down design grounded in end-to-end reads of the content-effect family, the bindable infrastructure, the V2 lifecycle scheduler, and the V3 playback timing surface. Implementation deferred until design is ratified.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>2026-04-28 design-only packet — surfaced during packet-68 wargames audit when typewriter "didn't fire" turned out to be a 100ms enter duration making the reveal invisible. Recipe-author bumps to enter durations are a stopgap; the real fix is decoupling effect cadence from phase duration AND giving hosts a way to bind/observe both.</WCTX> -->
<!-- <CLOG>0.1.0: initial design draft after a 16-file end-to-end read pass across tui-vfx-content (15 effect variants + 5 transformer bodies + dispatcher + trait), tui-vfx-core (bindable infrastructure), tui-vfx-style (ShaderRuntimeParams), mixed-signals (SignalOrFloat), and tui-vfx-recipes (V2 lifecycle, V3 playback timing, content-text SSOT).</CLOG> -->

# Packet 69 — Content-effect cadence + phase terminators

**Status:** design proposal, not yet ratified. Implementation deferred.
**Scope:** cross-repo. Most code lives in `/usr/projects/tui-vfx`; recipe-side caller updates land in `/usr/projects/tui-vfx-recipes`.
**Author orientation:** Every load-bearing claim in this document is verified by an `ofpf-*` query or a full end-to-end read of the cited file. File paths are `<crate>/src/<module>/<file>:<line>` form.

---

## 1. Problem framing

Two coupled problems surfaced during packet 68:

### 1.1 Content-effect rate is implicitly tied to phase duration

Today, every rate-bearing content effect (`Typewriter`, `Scramble`, `Redact`, `Dissolve`, `Morph`, `GlyphCascade`, `Marquee`, `SplitFlap`, `Odometer`, etc.) takes its reveal/scroll/resolve pace from a single number: `progress` in 0–1, computed as `phase_elapsed / phase_duration`. The transformer renders "fully revealed at progress >= 1.0", so the typewriter (or scramble, or dissolve) reaches its end-state exactly when the phase ends — **never sooner, never later**. Recipe authors who want "1983 WOPR steady typing at ~80 ms/char" have only one lever: pre-compute `enter_duration_ms = char_count × desired_ms_per_char` and re-author per recipe. This is the authoring tax Intention 24 is meant to design away.

**Verified at:**
- `crates/tui-vfx-content/src/transformers/cls_typewriter.rs:36-89` — `transform()` body computes `base_threshold = (i+1)/total` against `progress`. No rate parameter; reveal completes when `progress >= 1.0`.
- `crates/tui-vfx-content/src/transformers/cls_scramble.rs:43-88` — same pattern: `if progress >= 1.0 return target` (line 50).
- `crates/tui-vfx-content/src/transformers/cls_redact.rs:23-48` — same pattern (line 30).
- `crates/tui-vfx-content/src/transformers/cls_glitch_shift.rs:51-79` — window-based: completes when `progress >= glitch_end`.
- `crates/tui-vfx-content/src/transformers/cls_marquee.rs:33-79` — continuous: never "completes" in the traditional sense, scrolls perpetually based on `speed` parameter.

### 1.2 Phase scheduling is purely time-driven on both paths

Neither V2's `LifecycleState` nor V3's `V3PlaybackTiming` supports any non-time signal for phase advancement. The V2 path has an imperative escape hatch (`AnimationManager.dismiss(id, now)`) for host-driven Dwelling→Exiting transitions, but it's an out-of-band Rust call — there is no recipe-level shape that says "advance when this binding fires" or "advance when the typewriter finishes."

**Verified at:**
- `tui-vfx-recipes/src/state/lifecycle.rs:84-126` — `tick()` body. Phase transitions fire on `ts.progress() >= 1.0` (Entering→Dwelling at line 92, Dwelling→Exiting at line 103, Exiting→Finished at line 120). No binding hook.
- `tui-vfx-recipes/src/state/lifecycle.rs:66-77` — `dismiss(now)` is the V2 host-driven escape hatch. Sets phase to Exiting; not exposed at recipe level.
- `tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs:71-138` — `sampled_v3_playback_timing_from_elapsed()` derives phase from `enter_ms`, `dwell_ms = max(1000, auto_dismiss_ms)`, `exit_ms`. Pure time math. No binding hook.
- `tui-vfx-recipes/src/manager/mod.rs:230` — `AnimationManager.dismiss(AnimationId, Instant)` is the public V2 host API; V3 has no equivalent.

### 1.3 Content effects are the one major effect family that's not bindable

The `VfxBindable<T, S>` family (`VfxBindableValue`, `VfxBindableU16`, `VfxBindableString`) is wired into shader, filter, sampler, mask, and style models in `tui-vfx-style` and `tui-vfx-compositor`. It's **not** wired into any content-effect field — `Typewriter::speed_variance`, `Scramble::resolve_pace`, `Marquee::speed`, `SplitFlap::speed/cascade/cycles`, etc. all use `SignalOrFloat` (literal + signal, no binding discriminant). An app cannot supply a runtime value for any content-effect parameter today.

**Verified at:**
- `crates/tui-vfx-core/src/bindable/cls_bindable.rs:167-179` — `VfxBindable<T, S>` envelope: `Literal(T) | Binding(String) | Signal(S)`.
- `crates/tui-vfx-core/src/bindable/cls_bindable.rs:381` — `VfxBindableValue = VfxBindable<f32, SignalOrFloat>`.
- `crates/tui-vfx-content/src/types/cls_content_effect.rs:178-185` — `Typewriter { speed_variance: SignalOrFloat, cursor: Option<TypewriterCursor> }`. No bindable fields.
- `ofpf-content "VfxBindable" --files-with-matches | grep tui-vfx-content` → **zero hits**. Content effects use `SignalOrFloat` exclusively.

### 1.4 Runtime params already plumb to the trait surface but the SSOT caller passes empty

The `TextTransformer` trait already takes `&TransformContext` carrying `runtime_params: &ShaderRuntimeParams`. The SSOT call site for both V2 and V3 (`tui-vfx-recipes/src/preview/fnc_resolve_content_text.rs`) constructs the context but with **`ShaderRuntimeParams::new()` (empty)**, with a TODO marker. Effectively: the TRAIT supports runtime params; the CALLER doesn't pass any.

**Verified at:**
- `crates/tui-vfx-content/src/traits/cls_transform_context.rs:18-30` — `TransformContext { signal_ctx, runtime_params }`. Already there since v3.0.0 of the trait.
- `crates/tui-vfx-content/src/traits/text_transformer.rs:13-26` — `transform(&self, target, progress, ctx: &TransformContext)`. Already there.
- `tui-vfx-recipes/src/preview/fnc_resolve_content_text.rs:42-49` — literal TODO: `"thread a real &ShaderRuntimeParams through resolve_content_text callers so host-supplied bindings reach transformers. For now, empty params preserve existing behavior (no host bindings supplied)."`
- `crates/tui-vfx-style/src/traits/cls_shader_context.rs:260-270` — `ShaderRuntimeParams` implements `RuntimeParamsRead` from `tui-vfx-core::bindable`. The bridge between the bindable lookup and the runtime-params store **already exists**.

---

## 2. Verified current-state summary

| Surface | Where it lives | Bindable? | Runtime-params threaded? | Notes |
|---|---|---|---|---|
| Filter / Mask / Sampler / Shader specs | `tui-vfx-compositor`, `tui-vfx-style` | **Yes** (`VfxBindableValue`, `VfxBindableU16`) | **Yes** (via `ShaderContext.runtime_params`) | Host can supply per-frame values today |
| Style models (BindableString, BindableU16) | `tui-vfx-style/src/models/` | **Yes** (re-exports of `VfxBindable<T>`) | **Yes** | Same as above |
| Content-effect parameters | `tui-vfx-content/src/types/cls_content_effect.rs` | **No** — `SignalOrFloat` only | **Trait surface yes, caller no** (the TODO) | Host has no way in |
| V2 phase scheduler (`LifecycleState`) | `tui-vfx-recipes/src/state/lifecycle.rs` | n/a | n/a | Phase transitions purely time-driven; `dismiss(now)` is imperative-only escape hatch |
| V3 phase scheduler (`V3PlaybackTiming`) | `tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs` | n/a | n/a | Phase derived from elapsed-ms vs authored `enter_ms` / `dwell_ms` / `exit_ms`; no binding/event hook |
| Effect-completion signal back to scheduler | n/a | n/a | n/a | **Does not exist.** Scheduler has no way to ask "is the typewriter done?" |
| Content-effect SSOT renderer | `tui-vfx-recipes/src/preview/fnc_resolve_content_text.rs` | n/a | One TODO closes the gap | Used by both V2 (`render_preview_item`) and V3 (`cls_v3_source_surface`) paths |

**The asymmetry:** every effect family in the system supports host-driven runtime parameters EXCEPT content effects. The plumbing is 80% there — the trait surface accepts it; the bridge type implements `RuntimeParamsRead`; the schema family `VfxBindable<T,S>` exists. Three small things are missing: (a) the field type swap in `ContentEffect` variants from `SignalOrFloat` → `VfxBindableValue`, (b) a one-line caller change in `fnc_resolve_content_text.rs`, (c) per-transformer code that reads bindings via the `ctx.runtime_params` already in scope.

---

## 3. Design proposal

Two distinct features that compose:

### 3.A. Bindable parity for content effects

Promote rate-bearing content-effect fields from `SignalOrFloat` to `VfxBindableValue`. This is a strict superset: `SignalOrFloat::Static(f)` collapses to `VfxBindable::Literal(f)` automatically (per `From<SignalOrFloat>` at `cls_bindable.rs:482-489`); `SignalOrFloat::Signal(spec)` → `VfxBindable::Signal(spec)`. The new `Binding(String)` variant is what apps supply at runtime.

**Wire format addition** (already supported by `VfxBindableValue`'s lenient deserializer):
```json
"speed_variance": { "binding": "typing_jitter" }
```

**Per-transformer change pattern** (one example, `Typewriter`):
```rust
// Before:
pub speed_variance: SignalOrFloat,
// transform body:
let variance = self.speed_variance.evaluate(progress, ctx.signal_ctx).unwrap_or(0.0);

// After:
pub speed_variance: VfxBindableValue,
// transform body:
let variance = self.speed_variance
    .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
    .unwrap_or(0.0);
```

The trait already has `runtime_params` in scope. The bindable's `evaluate` method (at `cls_bindable.rs:456-467`) already takes `(loop_t, signal_ctx, runtime_params)`. The change inside each transformer is one line.

### 3.B. PhaseTerminator — a unified concept for "when does the phase end?"

Today, `V3PipelineTiming` carries `enter_ms: Option<u64>`, `exit_ms: Option<u64>` — a flat scalar per phase. Replace with a tagged union:

```rust
pub enum PhaseTerminator {
    /// Phase ends after a fixed duration.
    Duration { ms: VfxBindableValue },           // bindable, so host can override

    /// Phase ends when the named content effect's `is_complete()` returns true.
    /// `which` selects the effect by id when multiple effects coexist in a phase;
    /// `None` means "the canonical content effect for this phase" (typically
    /// the one declared in `config.content.effect`).
    EffectComplete {
        which: Option<String>,
        /// Defensive cap so a hung effect does not stall the recipe forever.
        fallback_ms: Option<u64>,
    },

    /// Phase ends when the named binding is true (or non-zero / non-empty).
    /// `fallback_ms` is the defensive cap if the host never sets it.
    Binding {
        name: String,
        fallback_ms: Option<u64>,
    },

    /// Composite: ends when ANY of the inner terminators fires (logical OR).
    AnyOf(Vec<PhaseTerminator>),

    /// Composite: ends when ALL of the inner terminators have fired (logical AND).
    AllOf(Vec<PhaseTerminator>),
}
```

Wire shape (recipe-author surface):

```json
"timing": {
  "enter": { "ms": 6000 },
  "dwell": { "until_effect_complete": true },
  "exit":  { "until_binding": "user_dismissed", "fallback_ms": 30000 },
  "extra_phase_example": {
    "any_of": [
      { "ms": 5000 },
      { "until_binding": "skip" }
    ]
  }
}
```

The legacy flat `enter_ms: u64` shape continues to deserialize (lenient parse — bare number → `PhaseTerminator::Duration { ms: Literal(n) }`), so existing recipes keep working. The new tagged forms unlock the new behavior.

---

## 4. Recommendations for each open question

### Q1: Multi-effect "complete" semantics

**Recommendation: `EffectComplete { which: Option<String> }` with two interpretations.**

When `which: None` (default): the phase ends when the canonical content effect (the one in `config.content.effect`) reports complete. This handles ~95% of recipes — they have one content effect per phase.

When `which: Some(id)`: the phase ends when the named effect (typically a step-tree leaf with an authored id) reports complete. This handles future recipes that compose multiple content effects per phase.

For multi-effect "all of them", the author writes:
```json
{ "all_of": [
    { "until_effect_complete": { "which": "primary_text" } },
    { "until_effect_complete": { "which": "subtitle" } }
] }
```

For "any one of them," same pattern with `any_of`. This avoids a separate `until_all_effects_complete` shorthand — composition does the work.

**Grounded in:** Today the V3 path has only one content effect per phase (the `config.content.effect` field), per the schema at `crates/tui-vfx-content/src/types/cls_content_effect.rs:168-542` and the V3 envelope at `tui-vfx-recipes/src/v3/normalized/cls_normalized_recipe.rs:60-103`. Multi-effect composition is a future shape; the design should accommodate it without retrofit.

### Q2: What does "complete" mean per content effect?

**Recommendation: add `fn is_complete(&self, progress: f64, ctx: &TransformContext<'_>) -> bool` to `TextTransformer` with a default of `progress >= 1.0`. Effects with non-standard semantics override.**

Per-variant survey (verified by reading 5 transformer bodies; pattern projected for the remaining 10):

| Effect | Default `progress >= 1.0` correct? | Notes |
|---|---|---|
| Typewriter | Yes | Rendering body returns full text at `progress >= 1.0` (`cls_typewriter.rs:46`) |
| Scramble | Yes | Same pattern (`cls_scramble.rs:50`) |
| Redact | Yes | Same pattern (`cls_redact.rs:30`) |
| Dissolve | Yes (projected; same family as above) | Read body in implementation phase to confirm |
| Morph | Yes (projected) | Same |
| GlyphCascade | Yes (projected) | Same |
| Numeric | Always (no animation) | Override: `is_complete(_) = true` |
| WrapIndicator | Yes (projected, but trivial) | Same |
| GlitchShift | **No** — completes after `progress >= glitch_end` | Override returning `progress >= self.glitch_end.evaluate(...)`. Verified via `cls_glitch_shift.rs:60-77` (window logic). |
| ScrambleGlitchShift | Same as GlitchShift + Scramble combination | Override |
| Marquee | **Never** in the traditional sense — continuous scroll | Override returning `false` always; pair with `Duration` terminator instead |
| SplitFlap | Yes via `progress >= 1.0`, but the effect has internal "settle" sub-states | Default works; settle-aware authors can use a longer phase or a binding |
| Odometer | Yes via `progress >= 1.0` | Same as SplitFlap |
| SlideShift | Continuous-ish (interpolates start_col → end_col) | Override returning `progress >= 1.0` (final position) |
| Mirror | Toggles state on the boundary; "complete" means flip-back done | Override returning `progress >= 1.0` |

**Grounded in:** verified read of 5 transformer bodies; the remaining 10 will be confirmed during implementation. The trait change is at `crates/tui-vfx-content/src/traits/text_transformer.rs` — additive (new method with default), not a breaking signature change.

### Q3: Composition of terminators

**Recommendation: ship `AnyOf` and `AllOf` from day one.** Composition is the single biggest reason this design will outlast naïve point fixes. A timeout-on-binding pattern is just `AnyOf([Binding, Duration])` — no special-casing needed. The validator can warn when a `Binding`-only terminator has no fallback (Intention 25), but the runtime supports the compose-it-yourself pattern uniformly.

`AllOf` is less common but cheap to ship and unlocks "advance only when both the typewriter finishes AND the host says so" — useful for choreographed multi-content recipes. Skipping it would force authors to fall back to imperative `dismiss()` calls.

**Grounded in:** Intention 24 (earned-place) — composition is concretely useful (timeout-on-binding alone justifies it) and incrementally cheap once `AnyOf` exists. Intention 23 (rule of three) is satisfied by three current uses: timeout-bounded binding, multi-effect synchronization, and validator-only mode-gating.

### Q4: Does V3PipelineTiming have a place for this?

**Recommendation: replace the flat scalar shape with the tagged-union shape. Pre-1.0 wire format is allowed to evolve (Intention 10).**

Current shape (verified at `tui-vfx-recipes/src/v3/authoring/cls_v3_recipe_document.rs:248-311`):
```rust
pub struct V3PipelineTiming {
    pub enter_ms: Option<u64>,
    pub enter_offset_ms: Option<u64>,
    pub exit_ms: Option<u64>,
    pub exit_offset_ms: Option<u64>,
    pub enter_ease: Option<String>,
    pub exit_ease: Option<String>,
}
```

Proposed shape:
```rust
pub struct V3PipelineTiming {
    pub enter: Option<PhaseTerminator>,
    pub enter_offset_ms: Option<u64>,    // unchanged — offset is always time
    pub exit: Option<PhaseTerminator>,
    pub exit_offset_ms: Option<u64>,
    pub enter_ease: Option<String>,
    pub exit_ease: Option<String>,
    /// Optional dwell terminator. Today dwell uses `lifecycle.auto_dismiss_ms`;
    /// the new field gives a unified place for "until effect complete" / "until
    /// binding" on dwell. When present, takes precedence over `auto_dismiss_ms`.
    pub dwell: Option<PhaseTerminator>,
}
```

A lenient deserializer accepts the legacy flat form (`{"enter_ms": 6000}` → `enter: Some(Duration{ms: Literal(6000)})`). New recipes use the tagged form.

**Grounded in:**
- `cls_v3_recipe_document.rs:202` — `pub pipeline: V3Pipeline` — required field, no `#[serde(default)]`. The tagged shape of `enter`/`exit` is the only field-type change; all other fields stay.
- `cls_v3_playback_timing.rs:80-88` — current consumer reads `enter_ms` / `exit_ms` directly. Becomes `enter.as_ref().and_then(|t| t.expected_duration_ms())` or similar — see Phase B in §6 below for the runtime.
- The wargames children at `recipes/wargames/*.json` use a V2-shape `pipeline.enter.duration_ms` that today silently lands in V3Config's `extra` flatten map — this is a separate cleanup. Both shapes can coexist via lenient parsing during the cutover.

### Q5: Cross-phase coordination (dwell + auto_dismiss interaction)

**Recommendation: when `pipeline.timing.dwell` is set, it takes precedence over `lifecycle.auto_dismiss_ms`. When unset, fall back to the legacy `auto_dismiss_ms` reading.** This is fully back-compatible. Authors who want the new shape opt in by declaring `dwell: { ... }`; existing recipes unchanged.

The validator (Intention 25) can emit a warning when both are present at recipe-load time — likely an authoring mistake, but the runtime resolves it deterministically (new field wins).

**Grounded in:**
- `cls_v3_playback_timing.rs:88-93` — `dwell_ms = max(1000, auto_dismiss_ms)`. The new `dwell` field's `expected_duration_ms()` (for `Duration` terminator) replaces this calculation.
- `tui-vfx-recipes/src/state/lifecycle.rs:59-65` — V2's `effective_display_duration` reads `auto_dismiss` exclusively; when V2 grows the new shape, the same precedence rule applies.

### Q6: Effect-completion signal source

**Recommendation: add `is_complete` to `TextTransformer` (Q2). The phase scheduler polls it once per frame after rendering. No back-channel needed; it's a pure function of `(progress, ctx)`.**

The polling location is the SSOT: `fnc_resolve_content_text.rs` already has the transformer in scope at the render call. Right after `transformer.transform(...)`, the scheduler asks `transformer.is_complete(effective_t, &tctx)` and surfaces the answer to the phase scheduler. For the V2 path, the answer rides back in a new `RenderOutcome { text: String, content_effect_complete: bool }` shape (or the simpler "out parameter" pattern via `&mut bool` if the back-compat tax of changing the return type is too high).

**Grounded in:**
- `fnc_resolve_content_text.rs:21-61` — single call site for both V2 and V3 paths. Adding the completion poll there reaches both paths in one place.
- `tui-vfx-recipes/src/state/lifecycle.rs:84-126` — V2's `tick()` runs every frame; adding "if EffectComplete terminator → ask the renderer" fits the existing per-tick model.

---

## 5. Worked examples

### Example A — Recipe: V2-style time-bounded enter (back-compat)

```json
{
  "schema_version": 3,
  "id": "wargames.shall_we_play",
  "config": {
    "pipeline": {
      "timing": { "enter_ms": 6000 }     // legacy flat shape, lenient-parsed
    }
  }
}
```
Effect: identical to today. Enter is 6000ms; phase progresses by elapsed/duration.

### Example B — Recipe: WOPR-pace typing, phase ends when typing finishes

```json
{
  "schema_version": 3,
  "id": "wargames.shall_we_play",
  "config": {
    "content": {
      "effect": {
        "type": "typewriter",
        "cadence_ms": 80,         // <— NEW: ms per char (BindableValue)
        "speed_variance": 0.0
      }
    },
    "pipeline": {
      "timing": {
        "enter": { "until_effect_complete": true, "fallback_ms": 30000 }
      }
    }
  }
}
```
Effect: typewriter reveals at 80 ms/char regardless of message length; enter phase ends when reveal completes (or after 30 s if something stalls). Authoring tax of pre-computing duration disappears.

### Example C — Recipe: host-driven dwell

```json
"pipeline": {
  "timing": {
    "enter": { "ms": 1500 },
    "dwell": {
      "any_of": [
        { "until_binding": "user_dismissed" },
        { "ms": 60000 }
      ]
    },
    "exit": { "ms": 800 }
  }
}
```
Effect: dwell ends as soon as the host sets `user_dismissed` to true, OR after 60 s — whichever comes first.

### Example D — Recipe: app-controlled typing speed

```json
"content": {
  "effect": {
    "type": "typewriter",
    "cadence_ms": { "binding": "typing_speed_ms" },
    "speed_variance": { "binding": "typing_jitter" }
  }
}
```
Effect: app supplies cadence and jitter at runtime via `RuntimeBindings { typing_speed_ms: 80, typing_jitter: 0.2 }`. Same recipe drives slow expository typing OR fast confirmation prompts based on host state.

### Example E — Cadence vs phase duration tension (the Q the user named)

When `cadence_ms × char_count > enter_ms`, the typewriter is still mid-reveal at phase end. Three resolution modes the runtime supports:

1. **Time wins (default if `enter` is `Duration`):** snap the typewriter to fully revealed; phase ends on time. Cadence becomes a target rate, not a guarantee.
2. **Effect wins (when `enter` is `EffectComplete`):** phase auto-extends to fit the cadence. The recipe author opts into this by choosing `EffectComplete` terminator.
3. **Author chose explicitly (composite):** `enter: { all_of: [{ms: 6000}, {until_effect_complete: true}] }` means "wait for both — at least 6 s AND until typing finishes." The longer of the two wins.

The runtime never silently truncates — the recipe-author's terminator choice is the source of truth.

---

## 6. Phased implementation plan

### Phase A — Bindable parity for content effects (sibling repo, schema-additive)

Files modified in `/usr/projects/tui-vfx/`:
- `crates/tui-vfx-content/src/types/cls_content_effect.rs` — change rate-bearing fields from `SignalOrFloat` to `VfxBindableValue`. Affected variants (per the §1 audit): `Typewriter.speed_variance`, `Scramble.resolve_pace`, `GlitchShift.glitch_start/end`, `ScrambleGlitchShift.{resolve_pace, glitch_start, glitch_end}`, `SplitFlap.{speed, cascade, cycles}`, `Marquee.speed`. Lenient deserialize ensures existing recipes (which use bare numbers or `{"signal": ...}` shapes) keep working — `From<SignalOrFloat>` for `VfxBindableValue` already collapses Static→Literal.
- All ~6 transformer files where the changed fields are read — one-line edit per: `field.evaluate(t, ctx.signal_ctx)` → `field.evaluate(t, ctx.signal_ctx, ctx.runtime_params)`.
- `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs` — type-name change in the dispatch arm; same shape.

Files modified in `/usr/projects/tui-vfx-recipes/`:
- `src/preview/fnc_resolve_content_text.rs:42-49` — replace the empty `ShaderRuntimeParams::new()` with the host-supplied params. Caller chain needs to thread the params from `render_preview_item` (V2) and `cls_v3_source_surface::build_v3_source_text` (V3).

Tests:
- One per affected transformer: a recipe with `"speed_variance": { "binding": "k" }` evaluates the binding's value at runtime when the host supplies one.
- A regression suite asserting that bare-number and `{"signal": ...}` shapes keep parsing identically.

### Phase B — Add `cadence_ms` field to rate-driven content effects (sibling repo)

Files modified:
- `crates/tui-vfx-content/src/types/cls_content_effect.rs` — add `cadence_ms: Option<VfxBindableValue>` to `Typewriter`, `Scramble`, `Marquee`, `SplitFlap`, etc. (9 of 15 variants). Optional so absence preserves current behavior.
- All affected transformer bodies — when `cadence_ms` is set, re-derive `effective_progress` from elapsed-time-vs-cadence rather than from `progress`. Requires the transformer to know "elapsed ms in current phase," which is already in `ctx.signal_ctx.absolute_t` (per `cls_v3_playback_timing.rs:54`).

Validator:
- New rule in `tui-vfx-recipes/src/v3/validate/`: if `cadence_ms` is set and the phase terminator is `Duration`, warn when `cadence_ms × char_count > duration_ms` (Intention 25 — mechanical drift catching).

### Phase C — Add `PhaseTerminator` + `is_complete()` (cross-repo)

Trait change (sibling):
- `crates/tui-vfx-content/src/traits/text_transformer.rs` — add `fn is_complete(&self, progress: f64, ctx: &TransformContext<'_>) -> bool { progress >= 1.0 }` with default body. Override in `Marquee`, `GlitchShift`, `ScrambleGlitchShift`, `Numeric`.
- `tui-vfx-recipes/src/preview/fnc_resolve_content_text.rs` — extend the return type (or add an out-parameter) to surface `is_complete`.

Schema change (recipes):
- `tui-vfx-recipes/src/v3/authoring/cls_v3_recipe_document.rs:248-311` — replace `enter_ms`, `exit_ms`, add `dwell` field; type becomes `Option<PhaseTerminator>`. Lenient parse for legacy flat form.
- New file: `tui-vfx-recipes/src/v3/authoring/cls_phase_terminator.rs` — the enum + serde + ConfigSchema.

Phase scheduler change:
- `tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs:71-138` — `sampled_v3_playback_timing_from_elapsed` becomes terminator-aware. `Duration{ms}` keeps current behavior; `EffectComplete` and `Binding` need per-tick polling, so the sampling function gains parameters or a sibling function exists for stateful (poll-driven) progression.
- `tui-vfx-recipes/src/state/lifecycle.rs:84-126` — V2 `tick()` similarly extended.

Runtime params plumbing:
- The `Binding`-terminator's name is looked up in the same `ShaderRuntimeParams` map content effects use. Booleans stored as `Integer(0|1)`; non-zero → fired. (Or add a `Boolean` lookup if explicit typing matters.)

### Phase D — Recipe migration + documentation

- Update wargames templates (already done in packet 68 with `enter_ms` bumps) to use the new `cadence_ms` knob where the templates' description ("steady WOPR typing") aligns with a fixed cadence.
- Add a small set of debug recipes demonstrating each terminator variant.
- Authoring guide update: when to choose `Duration` vs `EffectComplete` vs `Binding`.
- `MARKETING.md` 90-second description: add a sentence about phase termination flexibility (the user-facing benefit).

---

## 7. Cross-repo scope summary

| Phase | tui-vfx (sibling) | tui-vfx-recipes | mixed-signals | gt-design |
|---|---|---|---|---|
| A — Bindable parity | Schema field swaps + transformer body updates | One-line caller TODO closure | n/a | n/a |
| B — `cadence_ms` field | Schema additions + transformer logic | Validator rule | n/a | n/a |
| C — `PhaseTerminator` + `is_complete` | Trait method + transformer overrides | Schema + scheduler + lenient legacy parse | n/a | Audit per Intention 41 — likely no edits, but verify zero hits |
| D — Recipe migration | n/a | Recipe edits + docs | n/a | n/a |

Per Intention 41: every cross-repo change requires a four-repo `ofpf-content` audit before landing. This packet's audit will focus on `VfxBindable<f32, SignalOrFloat>`, `SignalOrFloat`, `V3PipelineTiming` field names, and any consumer of `enter_ms`/`exit_ms` in `gt-design`.

---

## 8. Risks and open items

1. **`speed_variance` is a misleading name on a fully-bindable system.** When the field accepts a Binding that supplies an absolute rate, "variance" is wrong. This packet does not rename — but a rename to `cadence_jitter` or similar is worth a future cleanup. (Intention 10: clean-sheet at version boundaries; pre-1.0 still permits.)

2. **`SplitFlap.speed`, `cascade`, `cycles` interaction with `cadence_ms` is non-trivial.** SplitFlap has multiple internal pacing knobs; introducing `cadence_ms` requires defining how it composes with the existing knobs (probably: `cadence_ms` overrides the default `progress`-derived per-char timing while leaving `cascade` / `jitter` as variance). Needs design before Phase B touches SplitFlap.

3. **The `is_complete()` poll has a frame-of-evaluation question.** The natural place is post-render (the transformer just produced its frame's output, then the framework asks "would your next frame be the same?"). But for boundary determinism — does the phase end at frame N or frame N+1? — the boundary semantics need a one-line docstring saying "the phase ends on the first tick where `is_complete(progress, ctx) == true`." Not a design risk, but worth nailing in the implementation packet.

4. **Mixed-version chains (V2 child extends V3 parent or vice versa) interact with the new shape.** Packet 68 added `TemplateResolutionError::SchemaVersionMismatch` to reject these. Phase C's terminator shape should remain consistent with that gate — no V2 child should be able to inherit a V3-only `until_binding` value through extends.

5. **`fallback_ms` defaults.** If the validator mandates a fallback on every `Binding` and `EffectComplete` terminator, it's defensive. If it only warns, recipe authors might ship recipes that hang. **Recommendation: validator default is "warn"; strict-contracts mode escalates to error.** Per Intention 25 the warn-then-strict pattern is consistent with the signal-catalog gate already in place.

6. **The `apply` family on `ContentEffect`** (mentioned at `cls_content_effect.rs:131-140`) — `ContentEffect::apply()`, `apply_to_borrowed()`, `apply_with_context()` — are convenience methods on the schema type. If they're used outside the transformer dispatch, the `is_complete` change might need to surface there too. Implementation phase to inventory.

---

## 9. What this packet does NOT do

- Does not rename `speed_variance` (item 1 above).
- Does not consolidate the V2 `pipeline.enter.duration_ms` shape into the V3 `pipeline.timing.enter_ms` shape — that's a separate cleanup. Packet 68 already shipped lenient parsing for both.
- Does not change the `SignalOrFloat` semantics for non-content-effect surfaces (shaders/filters keep their existing types).
- Does not introduce a new content-effect lifecycle separate from phases (Option 4 from the prior chat thread). Effects stay coupled to phases; the new flexibility is in HOW the phase ends.
- Does not implement multi-effect content composition. The `which: Option<String>` field in `EffectComplete` is forward-compatible with that future, but multi-effect composition is its own design packet.

---

## 10. Suggested next steps

1. **Ratify or revise this design** — the user's framing in the chat thread is the load-bearing input; this doc is a verification + concretization, not a fait accompli.
2. **Write a smaller scoping packet for Phase A only** — bindable parity for content effects + the TODO closure. That alone is ~3 sibling files + 1 recipe-side file + tests, lands cleanly without touching the phase scheduler.
3. **Defer Phase B/C/D until the V3 wargames cleanup completes.** Packet 68's V2-recipe repointing + the 20 template duration bumps are still uncommitted; landing those first leaves a clean baseline for the cadence work.

<!-- <FILE>steering/work-packets/69-content-effect-cadence-and-phase-terminators.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
