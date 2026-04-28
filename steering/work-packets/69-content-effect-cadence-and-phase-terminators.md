<!-- <FILE>steering/work-packets/69-content-effect-cadence-and-phase-terminators.md</FILE> - <DESC>Design proposal: bindable parity for content effects + a unified PhaseTerminator concept that lets phases end on a duration, on effect-completion, or on a host-supplied binding. Top-down design grounded in end-to-end reads of the content-effect family, the bindable infrastructure, the V2 lifecycle scheduler, and the V3 playback timing surface.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>2026-04-28 add top-level "Model & rationale" section (§A) capturing the high-level mental shift, side-by-side ANSI diagrams of today vs proposed, gain/loss summary, blast-radius summary, and the five clarifying-question decision blocks that frame the entire packet. This material was discussed during the design conversation and belongs in the durable doc, not just the chat transcript. Also add §14 Carve-out packets pointer linking to packets 69-A and 69-E.</WCTX> -->
<!-- <CLOG>0.3.0: MINOR — add §A Model & rationale (with side-by-side ANSI diagrams of today vs proposed phase-end model, gains/losses, blast-radius summary, and 5 framing decisions); add §14 pointer to carve-out packets 69-A (bindable parity) and 69-E (event-driven dwell). No removed content. Existing §0–§13 unchanged.
0.2.0: MINOR — verification pass adds §0 corrections summary, §11 outstanding decisions, §12 verified-shape addenda, and §13 file inventory. Inline corrections: stale `apply_with_context` reference (now `apply_with_runtime`); per-effect `is_complete` table corrected with verified reads; Marquee continuous-class semantics flagged; Dissolve "complete" semantic question raised; Phase A surgery reduced (trait change already shipped in v3.0.0); V3 sampler statelessness identified as central design hole; motion-vs-pipeline duration shadowing identified; V2 duration source identified as TransitionSpec.duration_ms in tui-vfx-geometry, not JSON.
0.1.0: initial design draft after a 16-file end-to-end read pass.</CLOG> -->

# Packet 69 — Content-effect cadence + phase terminators

**Status:** design proposal, not yet ratified. Implementation deferred until §11 decisions are locked.
**Scope:** cross-repo. Most code lives in `/usr/projects/tui-vfx`; recipe-side caller updates land in `/usr/projects/tui-vfx-recipes`; **V2 duration source lives in `/usr/projects/tui-vfx-geometry`** (newly identified in 0.2.0).
**Author orientation:** Every load-bearing claim in this document is verified by an `ofpf-*` query or a full end-to-end read of the cited file. File paths are `<crate>/src/<module>/<file>:<line>` form. Line numbers reflect 0.2.0 verification pass; when an old citation is amended, the new line range is given inline.
**Carve-out packets shipping in parallel:**
- **packet 69-A** (`steering/work-packets/69-A-content-effect-bindable-parity.md`) — bindable parity for content effects. No scheduler changes.
- **packet 69-E** (`steering/work-packets/69-E-event-driven-dwell.md`) — minimal-slice event-driven dwell. V3 only, additive fields, sidesteps every §11 design hole.

See §14 for sequencing and the relationship to the full design.

---

## §A. Model & rationale (top-level framing)

Read this section first. Everything that follows in §0 onward is detail layered on top of the model described here. This section was extracted from the design conversation and pinned into the durable doc so the rationale doesn't drift away from the mechanics.

### A.1 The mental shift

**Today:** A phase is a duration. The runtime divides elapsed time by that duration. When the result hits 1.0, the phase advances. That is the entire state machine.

**Proposed:** A phase has a *terminator* — a question the runtime asks every frame: "are we done?" The default terminator is "have N milliseconds elapsed?" — which is exactly what we have today, just expressed differently. But other terminators become available. The recipe author picks which question to ask.

That is the entire conceptual shift. Time becomes one of several ways to answer "are we done?" — not the only way. Everything else (which terminators exist, where they live in the schema, how composition works, how the V3 sampler integrates) is mechanical detail flowing from this primitive.

### A.2 Side-by-side: today vs proposed

```
┌─────────────────────────────────────────────────────────────────┐
│                       TODAY: Time-Only                          │
└─────────────────────────────────────────────────────────────────┘

   Recipe declares fixed durations per phase:
   ┌──────────────┬──────────────┬──────────────┐
   │  enter_ms:   │  dwell_ms:   │  exit_ms:    │
   │     6000     │     4000     │     800      │
   └──────┬───────┴──────┬───────┴──────┬───────┘
          ▼              ▼              ▼
   ┌────────────┐  ┌──────────┐  ┌──────────┐
   │  ENTERING  │─▶│ DWELLING │─▶│ EXITING  │─▶ DONE
   └─────┬──────┘  └────┬─────┘  └────┬─────┘
         │              │             │
   each frame:    each frame:    each frame:
   elapsed/6000   elapsed/4000   elapsed/800
   ≥ 1.0? advance ≥ 1.0? advance ≥ 1.0? advance

   Host has ONE escape hatch (V2 only):
     AnimationManager.dismiss(id, now)
   It's an imperative Rust call. Not authorable in JSON.
   No way to say "wait for the user" inside a recipe.

┌─────────────────────────────────────────────────────────────────┐
│             PROPOSED: Phase ends on a Terminator                │
└─────────────────────────────────────────────────────────────────┘

   Recipe declares a terminator per phase:
   ┌──────────────────┬──────────────────┬──────────────┐
   │ enter:           │ dwell:           │ exit:        │
   │   any_of [       │   until_binding  │    ms 800    │
   │     ms 6000,     │     "dismissed"  │              │
   │     until_typed  │   fallback_ms    │              │
   │   ]              │     60000        │              │
   └──────┬───────────┴────────┬─────────┴──────┬───────┘
          ▼                    ▼                ▼
   ┌────────────┐        ┌──────────┐     ┌──────────┐
   │  ENTERING  │───────▶│ DWELLING │────▶│ EXITING  │─▶ DONE
   └─────┬──────┘        └────┬─────┘     └────┬─────┘
         │                    │                │
   each frame ask       each frame ask    each frame ask
   the terminator:      the terminator:   the terminator:
   "are we done?"       "are we done?"    "are we done?"

   Terminator vocabulary the runtime understands:
   ┌────────────────────────────────────────────────────┐
   │ Duration{ms}        "have N ms elapsed?"        ◀── what we have
   │ EffectComplete      "did the effect finish?"    ◀── new
   │ Binding{name}       "is this host flag true?"   ◀── new
   │ AnyOf[...]          "did any inner fire?"       ◀── new (OR)
   │ AllOf[...]          "did all inner fire?"       ◀── new (AND)
   └────────────────────────────────────────────────────┘

   Host signals through the same runtime-params channel
   that already exists for shader bindings. No new IPC.
```

### A.3 What we gain

- Recipes can say "stay until the user dismisses" without baking a guessed timeout into the JSON.
- Recipes can say "type at WOPR pace, end when typing is done" without precomputing duration from message length.
- Recipes can compose: "at least 5s, no more than 30s, end early if the host signals."
- The imperative `AnimationManager.dismiss(...)` escape hatch becomes a recipe-level contract that V3 honors natively.
- Same vocabulary across enter/dwell/exit. Symmetric, learnable, one mental model.

### A.4 What we lose

- **Pure determinism.** Time-only recipes are perfectly reproducible; event-driven ones depend on host behavior. Mitigated by requiring `fallback_ms` on Binding terminators (validator-enforced).
- **A small surface-area cost.** Every phase now answers a question rather than computes a fraction. Negligible runtime cost, slightly more validator surface.
- **V2 stays time-only initially** (documented limitation). Full V2 parity costs a third-repo change to `tui-vfx-geometry::TransitionSpec`. See §11 D4.

### A.5 Blast-radius summary (top-level)

| Surface | Impact |
|---|---|
| Existing recipes | Zero. Legacy `enter_ms` / `exit_ms` still parse via serde aliases. Bit-identical behavior. |
| V3 schema | Two structs gain optional fields (`V3PipelineTiming`, `V3MotionPhaseSpec`). Both already strict — needs alias scheme (§11 D3). |
| V3 sampler | One pure function gains a `runtime_params` parameter. ~7 caller sites pass it through. |
| V2 path | Unchanged for the minimal slice (packet 69-E). Full parity defers to a future cross-repo packet (§11 D4). |
| Apps | Optional new API surface. The plumbing already exists for shaders; we extend the same channel. |
| Validator | Three new rules: required-fallback warnings, Marquee + EffectComplete rejection, time-vs-cadence drift catching. |
| Recipes ecosystem | Opt-in. Authors who don't touch terminators see no change. |

The single most expensive thing in the whole packet is the architectural decision in §11 D1: how does the V3 sampler ask the transformer "are you done?" without a scheduler. That is the only place where "do it wrong and we eat the cost twice" applies. Everything else is mechanical.

### A.6 Five framing decisions (with recommended answers and defenses)

These are the five top-level decisions that shape the packet. Detailed decisions D1–D9 in §11 are the implementation-level ones; A.6 is the project-level framing.

**A.6.1 — Do we ship the minimal slice first, or the full vocabulary up front?**
*Recommendation: minimal slice first (packet 69-E).* Binding-on-dwell delivers the exact "event flag" capability and is the only terminator that's structurally simple — it's stateless from the sampler's perspective, doesn't need a scheduler, and doesn't touch motion-shadowing or transformer-completion design holes. It can ship in days. The full vocabulary needs a real architectural decision about how the V3 sampler talks to transformers, which deserves to land deliberately rather than under time pressure. Shipping the minimal slice first also gives a real-world signal about what authors actually compose.

**A.6.2 — Does V2 grow this feature, or stay time-only forever?**
*Recommendation: V2 stays time-only.* V2 is the legacy path. New host-driven workflows should be on V3. Cross-repo trait evolution (`tui-vfx-geometry::TransitionSpec`) is a high cost for a feature whose audience would migrate to V3 anyway. V2 still has its imperative `AnimationManager.dismiss(...)` escape hatch for hosts that need it. Document the limitation honestly.

**A.6.3 — When an event terminator has no fallback timeout, do we error or warn?**
*Recommendation: warn by default; error in strict-contracts mode.* Same pattern as the existing signal-catalog gate (Intention 25). Lets debug recipes and prototypes ship without ceremony, but production recipes (which run under strict contracts) cannot accidentally hang. The warning is loud enough to catch real mistakes; the error mode is available for environments that demand it.

**A.6.4 — When time AND event both apply (composite), does the longer or shorter win?**
*Recommendation: explicit composition operators only. No implicit precedence.* `AnyOf` (first-to-fire wins, OR) and `AllOf` (last-to-fire wins, AND) make the author's intent legible from the JSON. Implicit precedence rules become tribal knowledge the author has to memorize; explicit operators do not. The runtime honors the operator literally — no special cases.

**A.6.5 — For `Binding`, is the value latched (event semantics) or live (state semantics)?**
*Recommendation: latched.* "The user dismissed the dialog" should not un-dismiss the dialog if the host's state briefly drops back to false on the next frame. Latch-on-first-true matches how authors will mentally model events. Live semantics are a foot-gun: a single-frame host glitch undoes the transition, and the recipe would un-advance — which violates phase ordering invariants the rest of the runtime assumes. Detailed restatement at §11 D7.

---

## 0. Verification-pass corrections (added 0.2.0)

The 0.1.0 draft was grounded in a 16-file read pass but missed several load-bearing details. A second pass that read **every** file end-to-end (no excerpts, no projections) surfaced the following corrections. Each is expanded in its own section below; this is the index.

| # | What 0.1.0 said | What's actually true | Section |
|---|---|---|---|
| 1 | `apply_with_context` is part of the apply family | Removed in `fnc_apply_content_effect.rs` v2.0.0, replaced by `apply_with_runtime`. The 0.1.0 reference repeated a stale doc comment in `cls_content_effect.rs:131-140` | §8 Risk #6 (revised) |
| 2 | The TextTransformer trait change is part of Phase A | The trait already takes `&TransformContext<'_>` (`text_transformer.rs:13-26`, v3.0.0). All 15 transformers already accept it. Phase A is just field-type swaps + one caller TODO closure | §6 Phase A (revised) |
| 3 | Per-effect `is_complete` table was projected for 10 of 15 | Now verified by full reads of all 15 transformer bodies. Several entries change | §12.1 |
| 4 | `Marquee` overrides `is_complete = false` | Marquee has no `progress >= 1.0` early return at all (`cls_marquee.rs:33-78`). It is genuinely continuous. The validator must reject `EffectComplete` paired with continuous-class effects, not document a hang | §11 D5 |
| 5 | `Dissolve` should use the default `is_complete = progress >= 1.0` | At `progress >= 1.0` Dissolve returns *fully dissolved* (target text gone) (`cls_dissolve.rs:225-230`). "Complete" semantically means "fully obscured", not "settled on target" — needs a project decision | §11 D6 |
| 6 | The Phase C terminator change is local to `V3PipelineTiming` | The V3 sampler reads `motion.enter.duration_ms` first and only falls back to `pipeline.timing.enter_ms` (`cls_v3_playback_timing.rs:75-87`). Motion-bearing recipes silently shadow any pipeline-side terminator | §11 D2, §12.3 |
| 7 | Lenient parse handles legacy `enter_ms` shape | Both `V3PipelineTiming` (`cls_v3_recipe_document.rs:217`) and `V3MotionPhaseSpec` (`cls_v3_motion_envelope.rs:153`) are `#[serde(default, deny_unknown_fields)]`. Field renames need explicit `#[serde(alias = "enter_ms")]` or an untagged enum at the type level | §11 D3, §12.4 |
| 8 | V2 grows the new shape via the `lifecycle.auto_dismiss_ms` precedence rule | V2's enter/exit duration comes from `AnimationProfile.enter: TransitionSpec` (`types/animation_profile.rs:13-66`), and `TransitionSpec.duration_ms` lives in **tui-vfx-geometry**. There is no JSON-shaped V2 timing field to evolve. Phase C touching V2 means evolving `TransitionSpec` in a third repo | §11 D4, §12.5 |
| 9 | The "scheduler polls `is_complete` once per frame after rendering" | V3 has no scheduler. `sampled_v3_playback_timing_from_elapsed` is a pure function called per-frame from the renderer/preview state. It receives only `(compiled, elapsed)` — no place to thread an effect-completion answer between frames. This is the central architectural decision the design must lock first | §11 D1, §12.2 |
| 10 | `MARKETING.md` is in `tui-vfx-recipes` | `MARKETING.md` is in `/usr/projects/tui-vfx/steering/MARKETING.md`. `tui-vfx-recipes/steering/` only has `INTENTIONS.md` and `work-packets/`. Phase D copy lands in the sibling | §6 Phase D (revised) |
| 11 | `EffectComplete { which: Option<String> }` references content-effect ids | No content-effect variant carries an `id` field today (verified read of all 15 in `cls_content_effect.rs:168-542`). `which` would address an id space that doesn't yet exist. Either rename to `step_id` to match the only existing id space (CompiledStep), or define the v0 spec as "no `which` resolves to `RaContentConfig::from(config.content)`" | §11 D8 |
| 12 | The compiled path consumes `V3PipelineTiming` only via its DTO | `CompiledPipelinePlan.timing: Option<V3PipelineTiming>` (`cls_compiled_recipe_plan.rs:120-127`). Same type as authoring → propagates schema changes for free. Win the 0.1.0 didn't claim | §12.6 |

---

## 1. Problem framing (unchanged from 0.1.0)

Two coupled problems surfaced during packet 68:

### 1.1 Content-effect rate is implicitly tied to phase duration

Today, every rate-bearing content effect (`Typewriter`, `Scramble`, `Redact`, `Dissolve`, `Morph`, `GlyphCascade`, `Marquee`, `SplitFlap`, `Odometer`, etc.) takes its reveal/scroll/resolve pace from a single number: `progress` in 0–1, computed as `phase_elapsed / phase_duration`. The transformer renders "fully revealed at progress >= 1.0", so the typewriter (or scramble, or dissolve) reaches its end-state exactly when the phase ends — **never sooner, never later**. Recipe authors who want "1983 WOPR steady typing at ~80 ms/char" have only one lever: pre-compute `enter_duration_ms = char_count × desired_ms_per_char` and re-author per recipe. This is the authoring tax Intention 24 is meant to design away.

**Verified at:**
- `crates/tui-vfx-content/src/transformers/cls_typewriter.rs:36-89` — `transform()` body computes `base_threshold = (i+1)/total` against `progress`. No rate parameter; reveal completes when `progress >= 1.0`.
- `crates/tui-vfx-content/src/transformers/cls_scramble.rs:43-88` — same pattern: `if progress >= 1.0 return target` (line 50).
- `crates/tui-vfx-content/src/transformers/cls_redact.rs:23-48` — same pattern (line 30).
- `crates/tui-vfx-content/src/transformers/cls_glitch_shift.rs:51-79` — window-based: completes when `progress >= glitch_end`.
- `crates/tui-vfx-content/src/transformers/cls_marquee.rs:33-79` — continuous: never "completes" in the traditional sense, scrolls perpetually based on `speed` parameter. **Re-verified 0.2.0:** Marquee has no `progress >= 1.0` early return; even at progress=1.0 it returns a windowed slice computed from `progress * speed`.

### 1.2 Phase scheduling is purely time-driven on both paths

Neither V2's `LifecycleState` nor V3's `V3PlaybackTiming` supports any non-time signal for phase advancement. The V2 path has an imperative escape hatch (`AnimationManager.dismiss(id, now)`) for host-driven Dwelling→Exiting transitions, but it's an out-of-band Rust call — there is no recipe-level shape that says "advance when this binding fires" or "advance when the typewriter finishes."

**Verified at:**
- `tui-vfx-recipes/src/state/lifecycle.rs:84-126` — `tick()` body. Phase transitions fire on `ts.progress() >= 1.0` (Entering→Dwelling at line 92, Dwelling→Exiting at line 108, Exiting→Finished at line 121). No binding hook.
- `tui-vfx-recipes/src/state/lifecycle.rs:66-77` — `dismiss(now)` is the V2 host-driven escape hatch. Sets phase to Exiting; not exposed at recipe level.
- `tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs:71-138` — `sampled_v3_playback_timing_from_elapsed()` derives phase from `enter_ms`, `dwell_ms = max(1000, auto_dismiss_ms)`, `exit_ms`. **0.2.0 amendment:** the function is a **pure function**, not a scheduler; it is called per-frame from outside (e.g. `DirectV3PreviewState::update_from_elapsed` at `cls_direct_v3_preview_state.rs:198-214`). No back-channel for effect completion exists in the signature.
- `tui-vfx-recipes/src/manager/mod.rs:230` — `AnimationManager.dismiss(AnimationId, Instant)` is the public V2 host API; V3 has no equivalent.

### 1.3 Content effects are the one major effect family that's not bindable

The `VfxBindable<T, S>` family (`VfxBindableValue`, `VfxBindableU16`, `VfxBindableString`) is wired into shader, filter, sampler, mask, and style models in `tui-vfx-style` and `tui-vfx-compositor`. It's **not** wired into any content-effect field — `Typewriter::speed_variance`, `Scramble::resolve_pace`, `Marquee::speed`, `SplitFlap::speed/cascade/cycles`, etc. all use `SignalOrFloat` (literal + signal, no binding discriminant). An app cannot supply a runtime value for any content-effect parameter today.

**Verified at:**
- `crates/tui-vfx-core/src/bindable/cls_bindable.rs:167-179` — `VfxBindable<T, S>` envelope: `Literal(T) | Binding(String) | Signal(S)`.
- `crates/tui-vfx-core/src/bindable/cls_bindable.rs:381` — `VfxBindableValue = VfxBindable<f32, SignalOrFloat>`.
- `crates/tui-vfx-content/src/types/cls_content_effect.rs:178-185` — `Typewriter { speed_variance: SignalOrFloat, cursor: Option<TypewriterCursor> }`. No bindable fields.
- All 15 enum variants verified 0.2.0: rate-bearing variants are Typewriter, Scramble, GlitchShift, ScrambleGlitchShift, SplitFlap, Marquee. None expose `VfxBindableValue` today.

### 1.4 Runtime params already plumb to the trait surface but the SSOT caller passes empty

The `TextTransformer` trait already takes `&TransformContext` carrying `runtime_params: &ShaderRuntimeParams`. The SSOT call site for both V2 and V3 (`tui-vfx-recipes/src/preview/fnc_resolve_content_text.rs`) constructs the context but with **`ShaderRuntimeParams::new()` (empty)**, with a TODO marker. Effectively: the TRAIT supports runtime params; the CALLER doesn't pass any.

**Verified at:**
- `crates/tui-vfx-content/src/traits/cls_transform_context.rs:18-30` — `TransformContext { signal_ctx, runtime_params }`. Already there since v1.0.0 of the type.
- `crates/tui-vfx-content/src/traits/text_transformer.rs:13-26` — `transform(&self, target, progress, ctx: &TransformContext)`. Trait is already at v3.0.0; the BREAKING signature change shipped in slice 6.6.
- `tui-vfx-recipes/src/preview/fnc_resolve_content_text.rs:42-45` — literal TODO: `"thread a real &ShaderRuntimeParams through resolve_content_text callers so host-supplied bindings reach transformers. For now, empty params preserve existing behavior (no host bindings supplied)."` File is at v0.2.0; the TODO is tagged `slice-6.6-followup`.
- `crates/tui-vfx-style/src/traits/cls_shader_context.rs:260-270` — `ShaderRuntimeParams` implements `RuntimeParamsRead` from `tui-vfx-core::bindable`. The bridge between the bindable lookup and the runtime-params store **already exists**.
- **0.2.0 finding:** one transformer already reads `ctx.runtime_params` end-to-end — `cls_odometer.rs:88` forwards it to `resolve_mechanical_cycle_with_context` so binding-form font references resolve. So the wiring is proven in production — Phase A only needs to extend the same pattern to the rate-bearing fields.

---

## 2. Verified current-state summary (0.2.0 corrections inline)

| Surface | Where it lives | Bindable? | Runtime-params threaded? | Notes |
|---|---|---|---|---|
| Filter / Mask / Sampler / Shader specs | `tui-vfx-compositor`, `tui-vfx-style` | **Yes** (`VfxBindableValue`, `VfxBindableU16`) | **Yes** (via `ShaderContext.runtime_params`) | Host can supply per-frame values today |
| Style models (BindableString, BindableU16) | `tui-vfx-style/src/models/` | **Yes** (re-exports of `VfxBindable<T>`) | **Yes** | Same as above |
| Content-effect parameters | `tui-vfx-content/src/types/cls_content_effect.rs` | **No** — `SignalOrFloat` only | **Trait surface yes, caller no** (the TODO) — except Odometer fonts which already work | Host has no way in for rate-bearing fields |
| V2 phase scheduler (`LifecycleState`) | `tui-vfx-recipes/src/state/lifecycle.rs` | n/a | n/a | Phase transitions purely time-driven; durations come from `Animated::profile().enter.duration_ms` (a `TransitionSpec` field in `tui-vfx-geometry`); `dismiss(now)` is imperative-only escape hatch |
| V2 duration shape | `tui-vfx-geometry::TransitionSpec.duration_ms: u64` | n/a | n/a | **0.2.0 finding.** Not a JSON-authoring field on the recipe side at all; it's a Rust trait return value driving the V2 lifecycle |
| V3 phase scheduler — **does not exist** | n/a | n/a | n/a | **0.2.0 correction.** V3 is stateless: `sampled_v3_playback_timing_from_elapsed(compiled, elapsed) -> V3PlaybackTiming` is a pure function called once per frame from the renderer/preview state. The "scheduler" is the caller (e.g. `DirectV3PreviewState`) |
| V3 phase duration sources (in precedence order) | (1) `compiled.envelope.motion.enter.duration_ms`<br>(2) `compiled.pipeline.timing.enter_ms`<br>(3) `1000ms` default | n/a | n/a | **0.2.0 finding.** Two coexisting authoring shapes; motion shadows pipeline. `cls_v3_playback_timing.rs:75-87` is the lookup. `V3MotionPhaseSpec.duration_ms: u64` (`cls_v3_motion_envelope.rs:155-158`) is the motion-side; `V3PipelineTiming.enter_ms: Option<u64>` (`cls_v3_recipe_document.rs:248-311`) is the pipeline-side. Both are `deny_unknown_fields` |
| V3 dwell duration source | `compiled.envelope.lifecycle.auto_dismiss_ms` (default 1000ms, max(1000)) | n/a | n/a | `cls_v3_playback_timing.rs:88-93`. Read out of an opaque `serde_json::Value` lifecycle block |
| V3 caller chain (terminator polling target) | `DirectV3PreviewState::update_from_elapsed` (`cls_direct_v3_preview_state.rs:198-214`) → `sampled_v3_playback_timing_from_elapsed` → `rerender` → eventually `cls_v3_source_surface::resolve_source_text:688` → `resolve_content_text:21` | n/a | n/a | The state struct carries `phase, sample_t, loop_t, absolute_t_ms` but **no transformer state** between frames |
| Effect-completion signal back to scheduler | n/a | n/a | n/a | **Does not exist.** No path on V2 or V3 today |
| Content-effect SSOT renderer | `tui-vfx-recipes/src/preview/fnc_resolve_content_text.rs` | n/a | One TODO closes the gap | Used by both V2 (`fnc_render_preview_item.rs:45-53`) and V3 (`cls_v3_source_surface.rs:703-711`) paths |

**The asymmetry:** every effect family in the system supports host-driven runtime parameters EXCEPT the rate-bearing content-effect fields. The plumbing is 80% there — the trait surface accepts it; the bridge type implements `RuntimeParamsRead`; the schema family `VfxBindable<T,S>` exists; one transformer already reads `runtime_params` in production. Three small things are missing: (a) the field type swap in `ContentEffect` variants from `SignalOrFloat` → `VfxBindableValue`, (b) a one-line caller change in `fnc_resolve_content_text.rs`, (c) per-transformer code that reads bindings via the `ctx.runtime_params` already in scope.

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
    /// `None` means "the canonical content effect for this phase" (defined as
    /// the effect parsed from `RaContentConfig::from(config.content)` until a
    /// typed multi-effect surface exists). See §11 D8 — `which` is forward-
    /// compatible with a future id space; v0 spec must say None is the only
    /// supported value.
    EffectComplete {
        which: Option<String>,
        /// Defensive cap so a hung effect does not stall the recipe forever.
        /// REQUIRED when `which` is None and the canonical effect is in the
        /// continuous class (Marquee). Validator-rejected otherwise — see
        /// §11 D5.
        fallback_ms: Option<u64>,
    },

    /// Phase ends when the named binding latches true (or non-zero).
    /// Latch semantics: once `true` is observed, the terminator stays fired
    /// for the duration of the phase. See §11 D7.
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

The legacy flat `enter_ms: u64` shape continues to deserialize via an explicit `#[serde(alias = "enter_ms")]` on the new `enter` field (NOT via lenient unknown-field handling — both `V3PipelineTiming` and `V3MotionPhaseSpec` are `deny_unknown_fields`). See §11 D3 for the alias scheme.

---

## 4. Recommendations for each open question (0.2.0 amendments inline)

### Q1: Multi-effect "complete" semantics

**Recommendation: `EffectComplete { which: Option<String> }` with a `which: None`-only v0.**

When `which: None` (the v0-supported case): the phase ends when the canonical content effect — defined precisely as **the effect parsed via `RaContentConfig::from(config.content)`** in `cls_v3_source_surface.rs:698` — reports complete. This handles 100% of recipes today (single effect per phase).

When `which: Some(id)`: **rejected by validator in v0**. The id space doesn't exist yet (no `ContentEffect` variant carries an `id` field, verified in `cls_content_effect.rs:168-542`). Reserve the wire shape now so multi-effect composition can land later without renaming. See §11 D8 for the field-name decision (`which` vs `step_id` vs `effect_id`).

**Grounded in:** Today the V3 path has only one content effect per phase, surfaced via the typed `V3ContentConfig.extra: BTreeMap<String, Value>` flatten map (`cls_v3_recipe_document.rs:151-152`) and parsed at source-surface time. Multi-effect composition is a future shape; the design accommodates it without retrofit.

### Q2: What does "complete" mean per content effect?

**Recommendation: add `fn is_complete(&self, progress: f64, ctx: &TransformContext<'_>) -> bool` to `TextTransformer` with a default of `progress >= 1.0`. Effects with non-standard semantics override.**

Per-variant table — **0.2.0 verified by full reads of all 15 transformer bodies** (replaces the projected table in 0.1.0):

| Effect | At `progress >= 1.0` returns | Default `progress >= 1.0` correct? | Override needed |
|---|---|---|---|
| Typewriter | `Cow::Borrowed(target)` (`cls_typewriter.rs:46-48`) | **Yes** | None |
| Scramble | `Cow::Borrowed(target)` (`cls_scramble.rs:50-52`) | **Yes** | None |
| Redact | `Cow::Borrowed(target)` (`cls_redact.rs:30-32`) | **Yes** | None |
| GlitchShift | window-based; `Borrowed(target)` outside window (`cls_glitch_shift.rs:70-77`) | **No** | Override returning `progress >= self.glitch_end.evaluate(...)` |
| ScrambleGlitchShift | inherits both (`cls_scramble_glitch_shift.rs:71-126`) | **No** | Override returning `progress >= self.glitch_end.evaluate(...)` AND `progress >= 1.0` for the scramble part — semantic question: which dominates? |
| GlyphCascade | per-cell `local >= 1.0` only when global `progress=1.0` (`cls_glyph_cascade.rs:131-180`) | **Yes** in practice | None |
| SplitFlap | `Cow::Borrowed(target)` (`cls_split_flap.rs:423-425`) | **Yes** | None |
| Odometer | `Cow::Borrowed(target)` (`cls_odometer.rs:77-79`) | **Yes** | None |
| Numeric | `Cow::Borrowed(target)` (`cls_numeric.rs:33-35`) | **Yes** | Override `is_complete(_) = true` (no real animation) |
| Marquee | **NO `>=1.0` early return** — continues windowed slice (`cls_marquee.rs:33-78`) | **Continuous class** — needs special treatment, NOT an override returning `false` (which causes guaranteed hang). See §11 D5 | Validator rejection when paired with `EffectComplete`; runtime falls through to `Duration` only |
| SlideShift | clamps progress; final col at `progress=1.0` (`cls_slide_shift.rs:55-95`) | **Yes** | None |
| Mirror | `Cow::Borrowed(target)` (`cls_mirror.rs:59-61`) | **Yes** | None |
| Dissolve | **fully dissolved** at `>=1.0` (target text gone) (`cls_dissolve.rs:225-230`) | Semantically inverted — "complete" = obscured, not settled. See §11 D6 for the project decision | Project-level decision needed before override |
| Morph | `Cow::Borrowed(target)` (`cls_morph.rs:259-262`) | **Yes** | None |
| WrapIndicator | full wrap at `>=1.0` (`cls_wrap_indicator.rs:84-95`) | **Yes** | None |

**Verified taxonomy (correcting 0.1.0):** 12 effects work under the default. **GlitchShift** + **ScrambleGlitchShift** complete earlier than progress=1.0 by their own semantics. **Marquee** is the sole continuous-class effect — it needs validator handling, not a `false` override. **Dissolve** has an inverted notion of "complete" — needs a project decision (§11 D6).

The trait change is at `crates/tui-vfx-content/src/traits/text_transformer.rs` — additive (new method with default), not a breaking signature change.

### Q3: Composition of terminators

Recommendation unchanged from 0.1.0: ship `AnyOf` and `AllOf` from day one. Composition is the single biggest reason this design will outlast naïve point fixes. A timeout-on-binding pattern is just `AnyOf([Binding, Duration])` — no special-casing needed. The validator can warn when a `Binding`-only terminator has no fallback (Intention 25), but the runtime supports the compose-it-yourself pattern uniformly.

### Q4: Does V3PipelineTiming have a place for this?

**Recommendation amended 0.2.0: replace the flat scalar shape with the tagged-union shape, AND simultaneously update `V3MotionPhaseSpec` so the motion-vs-pipeline shadowing doesn't silently bypass the new terminator.** Pre-1.0 wire format is allowed to evolve (Intention 10); aliases preserve byte-identical legacy parsing.

Current shape (verified at `tui-vfx-recipes/src/v3/authoring/cls_v3_recipe_document.rs:248-311`, `#[serde(default, deny_unknown_fields)]`):
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
#[serde(default, deny_unknown_fields)]
pub struct V3PipelineTiming {
    #[serde(alias = "enter_ms", deserialize_with = "deserialize_phase_terminator_or_ms")]
    pub enter: Option<PhaseTerminator>,
    pub enter_offset_ms: Option<u64>,    // unchanged — offset is always time
    #[serde(alias = "exit_ms", deserialize_with = "deserialize_phase_terminator_or_ms")]
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

Same surgery on `V3MotionPhaseSpec` (`cls_v3_motion_envelope.rs:152-217`):
```rust
#[serde(default, deny_unknown_fields)]
pub struct V3MotionPhaseSpec {
    #[serde(alias = "duration_ms", deserialize_with = "deserialize_phase_terminator_or_ms")]
    pub terminator: Option<PhaseTerminator>,
    // ... other fields unchanged
}
```

The custom deserializer handles the legacy bare-number form (`{"enter_ms": 6000}` → `enter: Some(Duration{ms: Literal(6000)})`). New recipes use the tagged form. Without the alias-plus-custom-deserializer combination, `deny_unknown_fields` rejects the legacy field; without the alias the field rename is breaking on day one.

**Grounded in:**
- `cls_v3_recipe_document.rs:217` — `#[serde(default, deny_unknown_fields)]`. The `deny_unknown_fields` attribute is critical; lenient unknown-field acceptance does not exist on this surface.
- `cls_v3_motion_envelope.rs:153` — same attribute. The motion-side rename has the same blocker.
- `cls_v3_playback_timing.rs:75-87` — current consumer reads `motion.enter.duration_ms` first, then `pipeline_timing.enter_ms`. The `compiled_motion()` lookup at line 75 takes precedence; pipeline-only changes don't affect motion-bearing recipes. **The terminator field MUST land in both places, or precedence MUST be explicitly inverted (with documented behavior change).**
- `cls_compiled_recipe_plan.rs:120-127` — `CompiledPipelinePlan.timing: Option<V3PipelineTiming>` carries the same DTO type; schema changes propagate through normalize → compile for free. Same for `compiled.envelope.motion: Option<V3MotionEnvelope>` (line 83).

### Q5: Cross-phase coordination (dwell + auto_dismiss interaction)

**Recommendation: when `pipeline.timing.dwell` is set, it takes precedence over `lifecycle.auto_dismiss_ms`. When unset, fall back to the legacy `auto_dismiss_ms` reading.** This is fully back-compatible. Authors who want the new shape opt in by declaring `dwell: { ... }`; existing recipes unchanged.

The validator (Intention 25) can emit a warning when both are present at recipe-load time — likely an authoring mistake, but the runtime resolves it deterministically (new field wins).

**0.2.0 amendment for V2:** the V2 path's enter/exit duration comes from `AnimationProfile.enter: TransitionSpec` (a Rust trait return value, not a JSON authoring field). Extending V2 to honor `PhaseTerminator` requires evolving `TransitionSpec` in **tui-vfx-geometry** — a third repo the 0.1.0 packet didn't touch. See §11 D4 for whether V2 grows the new shape at all in this packet, or stays time-only forever and becomes a documented V3-only feature.

**Grounded in:**
- `cls_v3_playback_timing.rs:88-93` — `dwell_ms = max(1000, auto_dismiss_ms)`. The new `dwell` field's `expected_duration_ms()` (for `Duration` terminator) replaces this calculation.
- `tui-vfx-recipes/src/state/lifecycle.rs:53-65` — V2's `enter_duration()` reads `item.profile().enter.duration_ms`; `effective_display_duration` reads `auto_dismiss` exclusively. The `Animated::profile()` trait returns an `AnimationProfile` whose enter/exit are `TransitionSpec` (in tui-vfx-geometry).

### Q6: Effect-completion signal source

**Recommendation overhauled 0.2.0: pure-function path. The phase scheduler/sampler instantiates the transformer itself when a terminator is `EffectComplete`, calls `is_complete(progress, ctx)`, and discards the transformer.** The renderer also instantiates a transformer for actual rendering — the duplication cost is real but the alternative (state plumbing across pure-function frame boundaries) is structurally worse. See §11 D1 for the full architectural decision.

The polling logic is **co-located with the timing sampler**, not the renderer:

```rust
pub fn sampled_v3_playback_timing_from_elapsed(
    compiled: &CompiledRecipePlan,
    elapsed: Duration,
    runtime_params: &ShaderRuntimeParams,  // NEW — needed for Binding terminator + bindable Duration
) -> V3PlaybackTiming {
    // ... existing time math ...
    // For phases whose terminator is EffectComplete: instantiate the transformer
    // from compiled.envelope.content, compute the would-be progress at this
    // elapsed time, ask is_complete, and advance the phase if true.
    // For Binding terminator: look up the binding in runtime_params.
}
```

This is a **breaking signature change** to a function with 7 callers (the V3 preview state, a few diag examples, the validator tools, etc.). Acceptable per Intention 10 pre-1.0; called out here so it doesn't surprise.

For the V2 path, the same logic lives in `lifecycle.rs::tick`. V2 is single-binary (no cross-frame state plumbing required) — it can either instantiate the transformer or carry one in the LifecycleState struct. See §11 D4 for the V2 decision.

**Grounded in:**
- `fnc_resolve_content_text.rs:21-61` — single call site for both V2 and V3 paths; transformer is instantiated inside via `get_transformer(effect)`. Cost is ~1 small allocation per render.
- `cls_v3_playback_timing.rs:71-138` — current sampler signature `(compiled, elapsed) -> V3PlaybackTiming`. Adding `runtime_params` is a strict superset; callers pass through the existing `runtime_overrides.runtime_params` they already hold.
- `cls_direct_v3_preview_state.rs:198-214` — the chief V3 caller already holds runtime_params in `self.runtime_overrides.runtime_params`. Threading through is one parameter.

---

## 5. Worked examples (unchanged from 0.1.0)

### Example A — Recipe: V2-style time-bounded enter (back-compat)

```json
{
  "schema_version": 3,
  "id": "wargames.shall_we_play",
  "config": {
    "pipeline": {
      "timing": { "enter_ms": 6000 }     // legacy flat shape, alias-resolved
    }
  }
}
```
Effect: identical to today. Enter is 6000ms; phase progresses by elapsed/duration. The `enter_ms` alias resolves to `enter: Some(Duration{ms: Literal(6000)})`.

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
Effect: dwell ends as soon as the host sets `user_dismissed` to true (latched), OR after 60 s — whichever comes first.

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

### Example E — Cadence vs phase duration tension

When `cadence_ms × char_count > enter_ms`, the typewriter is still mid-reveal at phase end. Three resolution modes the runtime supports:

1. **Time wins (default if `enter` is `Duration`):** snap the typewriter to fully revealed; phase ends on time. Cadence becomes a target rate, not a guarantee.
2. **Effect wins (when `enter` is `EffectComplete`):** phase auto-extends to fit the cadence. The recipe author opts into this by choosing `EffectComplete` terminator.
3. **Author chose explicitly (composite):** `enter: { all_of: [{ms: 6000}, {until_effect_complete: true}] }` means "wait for both — at least 6 s AND until typing finishes." The longer of the two wins.

The runtime never silently truncates — the recipe-author's terminator choice is the source of truth.

---

## 6. Phased implementation plan (0.2.0 corrections)

### Phase A — Bindable parity for content effects (sibling repo, schema-additive)

**0.2.0 surgery scope is smaller than 0.1.0 claimed** — the trait signature change shipped in `text_transformer.rs` v3.0.0, and all 15 transformers already accept `&TransformContext<'_>`. Phase A is purely:

Files modified in `/usr/projects/tui-vfx/`:
- `crates/tui-vfx-content/src/types/cls_content_effect.rs` — change rate-bearing fields from `SignalOrFloat` to `VfxBindableValue`. Affected variants per the §1 audit: `Typewriter.speed_variance`, `Scramble.resolve_pace`, `GlitchShift.glitch_start/end`, `ScrambleGlitchShift.{resolve_pace, glitch_start, glitch_end}`, `SplitFlap.{speed, cascade, cycles}`, `Marquee.speed`. Lenient deserialize ensures existing recipes keep working — `From<SignalOrFloat>` for `VfxBindableValue` (`cls_bindable.rs:482-489`) collapses Static→Literal automatically; the `BareSignal` repr (`cls_bindable.rs:213-215`) preserves the `{"signal": ...}` shape.
- All affected transformer bodies — one-line edit per: `field.evaluate(t, ctx.signal_ctx)` → `field.evaluate(t, ctx.signal_ctx, ctx.runtime_params)`. Affected: Typewriter, Scramble, GlitchShift, ScrambleGlitchShift, SplitFlap, Marquee.
- `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs` — type-name change in the dispatch arms; same shape. The factory clones the field; the type just becomes `VfxBindableValue`.
- `crates/tui-vfx-content/src/types/fnc_apply_content_effect.rs` — verified 0.2.0 already at v2.0.0. The `apply_with_runtime` entry point is the host-injection path. **No change needed for Phase A** — the file's API already matches the new wiring.

Files modified in `/usr/projects/tui-vfx-recipes/`:
- `src/preview/fnc_resolve_content_text.rs:42-45` — replace the empty `ShaderRuntimeParams::new()` with the host-supplied params. Caller chain needs to thread the params from `fnc_render_preview_item.rs:45-53` (V2) and `cls_v3_source_surface.rs:703-711` (V3). For V3, the params are already in `DirectV3PreviewState.runtime_overrides.runtime_params` (`cls_direct_v3_preview_state.rs:42`). For V2, the source needs identification — likely a new field on `PlaybackPlan`.

Tests:
- One per affected transformer: a recipe with `"speed_variance": { "binding": "k" }` evaluates the binding's value at runtime when the host supplies one.
- A regression suite asserting that bare-number and `{"signal": ...}` shapes keep parsing identically.
- Round-trip test: serialize `VfxBindable::Literal(0.5)` → parse → equals `Literal(0.5)`; same for `Binding` and `Signal`.

### Phase B — Add `cadence_ms` field to rate-driven content effects (sibling repo)

Files modified:
- `crates/tui-vfx-content/src/types/cls_content_effect.rs` — add `cadence_ms: Option<VfxBindableValue>` to `Typewriter`, `Scramble`, `Marquee`, `SplitFlap`, etc. Optional so absence preserves current behavior.
- All affected transformer bodies — when `cadence_ms` is set, re-derive `effective_progress` from elapsed-time-vs-cadence rather than from `progress`. Requires the transformer to know "elapsed ms in current phase," which is already in `ctx.signal_ctx.absolute_t` (verified 0.2.0: `mixed-signals/src/traits/signal.rs:74` defines the field; `cls_v3_playback_timing.rs:54` writes the V3 sampler's `absolute_t_ms` into it).

Validator:
- New rule in `tui-vfx-recipes/src/v3/validate/`: if `cadence_ms` is set and the phase terminator is `Duration`, warn when `cadence_ms × char_count > duration_ms` (Intention 25 — mechanical drift catching).

**Phase B blocker:** the `cadence_ms` overlap with SplitFlap's existing `speed`/`cascade`/`cycles`/`jitter` knobs needs design before code lands. See §11 D9.

### Phase C — Add `PhaseTerminator` + `is_complete()` (cross-repo)

Trait change (sibling):
- `crates/tui-vfx-content/src/traits/text_transformer.rs` — add `fn is_complete(&self, progress: f64, ctx: &TransformContext<'_>) -> bool { progress >= 1.0 }` with default body. Override in `GlitchShift`, `ScrambleGlitchShift`, `Numeric`. **Marquee does NOT override** — see §11 D5 (validator rejects, runtime never invokes).

Schema change (recipes — three places, not one):
1. `tui-vfx-recipes/src/v3/authoring/cls_v3_recipe_document.rs:248-311` — replace `enter_ms`, `exit_ms`, add `dwell` field; type becomes `Option<PhaseTerminator>`. Custom deserializer + `#[serde(alias)]` to handle legacy flat form (see §11 D3).
2. `tui-vfx-recipes/src/v3/compile/cls_v3_motion_envelope.rs:152-217` — same surgery on `V3MotionPhaseSpec.duration_ms`. **Required** to close the motion-vs-pipeline shadowing (§11 D2).
3. New file: `tui-vfx-recipes/src/v3/authoring/cls_phase_terminator.rs` — the enum + serde + ConfigSchema.

Compiled-plan propagation: **no separate work**. `CompiledPipelinePlan.timing: Option<V3PipelineTiming>` (`cls_compiled_recipe_plan.rs:120-127`) and `CompiledEnvelope.motion: Option<V3MotionEnvelope>` (line 83) carry the same DTO types — schema changes propagate through normalize → compile for free. (NormalizedRecipe also carries the same types — `cls_normalized_recipe.rs:135` for pipeline timing, `cls_normalized_recipe.rs:96` for motion envelope.)

Phase scheduler change (V3 — pure-function path):
- `tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs:71-138` — `sampled_v3_playback_timing_from_elapsed` becomes terminator-aware. New parameter `runtime_params: &ShaderRuntimeParams`. For `Duration{ms}` it reads the literal/bound ms; for `EffectComplete` it instantiates the transformer via `get_transformer(canonical_effect)` and calls `is_complete(progress_at_elapsed, ctx)`; for `Binding` it looks up `runtime_params.get_*`. See §11 D1 for the architectural justification.
- `tui-vfx-recipes/src/state/lifecycle.rs:84-126` — V2 `tick()` similarly extended. V2 already holds the lifecycle state across frames so it MAY carry a transformer if instantiation cost matters. See §11 D4.

V2 duration source (third repo):
- **NEW in 0.2.0.** `tui-vfx-geometry::TransitionSpec.duration_ms: u64` is V2's authoring shape. Phase C in V2 means evolving `TransitionSpec` to carry an optional `terminator: Option<PhaseTerminator>` alongside `duration_ms`. See §11 D4 for the explicit decision: V2 grows the new shape (cross-repo cost) OR V2 stays time-only forever (documented limitation).

Runtime params plumbing:
- The `Binding`-terminator's name is looked up in the same `ShaderRuntimeParams` map content effects use. Booleans stored as `Boolean(bool)` per `ShaderRuntimeParamValue` (`cls_shader_context.rs:23-39`). Latch semantics (§11 D7): once true, stays fired for the phase.

### Phase D — Recipe migration + documentation

- Update wargames templates (already done in packet 68 with `enter_ms` bumps) to use the new `cadence_ms` knob where the templates' description ("steady WOPR typing") aligns with a fixed cadence.
- Add a small set of debug recipes demonstrating each terminator variant.
- Authoring guide update: when to choose `Duration` vs `EffectComplete` vs `Binding`.
- **`MARKETING.md` update lands in `/usr/projects/tui-vfx/steering/MARKETING.md`** (corrected 0.2.0; `tui-vfx-recipes/steering/` has no `MARKETING.md`, only `INTENTIONS.md`). Add a sentence about phase termination flexibility (the user-facing benefit).

---

## 7. Cross-repo scope summary (0.2.0 expansion)

| Phase | tui-vfx (sibling) | tui-vfx-recipes | tui-vfx-geometry | mixed-signals | gt-design |
|---|---|---|---|---|---|
| A — Bindable parity | Schema field swaps + transformer body updates (~7 files) | One-line caller TODO closure + caller threading (~3 files) | n/a | n/a | n/a |
| B — `cadence_ms` field | Schema additions + transformer logic (~8 files) | Validator rule (~1 file) | n/a | n/a | n/a |
| C — `PhaseTerminator` + `is_complete` | Trait method + ~4 transformer overrides | Schema (3 places) + sampler signature change + V2 lifecycle | **NEW: `TransitionSpec` evolution if V2 grows the shape (§11 D4)** | n/a | Audit per Intention 41 — likely no edits, but verify zero hits |
| D — Recipe migration | Recipe edits + docs + MARKETING.md update | Recipe edits + docs | n/a | n/a | n/a |

Per Intention 41: every cross-repo change requires a four-repo `ofpf-content` audit before landing. **0.2.0 expands the audit list** — Phase C touches potentially five repos (tui-vfx, tui-vfx-recipes, tui-vfx-geometry, mixed-signals for SignalContext consumption verification, gt-design). The audit must focus on `VfxBindable<f32, SignalOrFloat>`, `SignalOrFloat`, `V3PipelineTiming` field names, `V3MotionPhaseSpec` field names, `TransitionSpec` field names, and any consumer of `enter_ms`/`exit_ms`/`duration_ms` in `gt-design`.

---

## 8. Risks and open items (0.2.0 corrections)

1. **`speed_variance` is a misleading name on a fully-bindable system.** When the field accepts a Binding that supplies an absolute rate, "variance" is wrong. This packet does not rename — but a rename to `cadence_jitter` or similar is worth a future cleanup. (Intention 10: clean-sheet at version boundaries; pre-1.0 still permits.) Recommendation: add `#[serde(alias = "speed_variance")]` to the renamed field in Phase B so the rename ships transparently.

2. **`SplitFlap.speed`, `cascade`, `cycles` interaction with `cadence_ms` is non-trivial.** SplitFlap has multiple internal pacing knobs; introducing `cadence_ms` requires defining how it composes with the existing knobs. Needs design before Phase B touches SplitFlap. See §11 D9.

3. **The `is_complete()` poll has a frame-of-evaluation question.** The boundary semantics need a one-line docstring saying "the phase ends on the first tick where `is_complete(progress_at_elapsed, ctx) == true`." Not a design risk, but worth nailing in the implementation packet.

4. **Mixed-version chains (V2 child extends V3 parent or vice versa) interact with the new shape.** Packet 68 added `TemplateResolutionError::SchemaVersionMismatch` to reject these. Phase C's terminator shape should remain consistent with that gate — no V2 child should be able to inherit a V3-only `until_binding` value through extends.

5. **`fallback_ms` defaults.** If the validator mandates a fallback on every `Binding` and `EffectComplete` terminator, it's defensive. If it only warns, recipe authors might ship recipes that hang. **Recommendation: validator default is "warn"; strict-contracts mode escalates to error. EXCEPT for `EffectComplete` paired with continuous-class effects (Marquee), which is always rejected (§11 D5).**

6. **(0.2.0 REVISED) The `apply` family on `ContentEffect`** lives in `cls_content_effect.rs:124-140` (doc) and `fnc_apply_content_effect.rs:24-97` (impl). Current methods: `apply` (line 48), `apply_to_borrowed` (line 67), `apply_with_runtime` (line 86). **The 0.1.0 reference to `apply_with_context` was stale** — that name was removed in v2.0.0 of `fnc_apply_content_effect.rs`. The doc comment in `cls_content_effect.rs:131-140` still mentions the old name and should be cleaned up in Phase A. The `apply_with_runtime` method already takes `ShaderRuntimeParams` — it is the host-injection path Phase A unlocks; no surgery needed there.

7. **(NEW 0.2.0) The V3 sampler statelessness blocks the `EffectComplete` design until §11 D1 resolves.** No code can land for Phase C until the architectural decision is made.

8. **(NEW 0.2.0) The motion-vs-pipeline duration shadowing means Phase C must touch two schema files, not one.** A pipeline-only terminator silently ignores the motion path's higher-precedence duration. See §11 D2.

9. **(NEW 0.2.0) V2 grows the new shape via a third repo (tui-vfx-geometry).** The 0.1.0 packet treated V2 as a recipe-side change; it is not. See §11 D4.

10. **(NEW 0.2.0) `deny_unknown_fields` blocks lenient back-compat for the field rename.** Both `V3PipelineTiming` and `V3MotionPhaseSpec` are strict. See §11 D3 for the alias scheme.

---

## 9. What this packet does NOT do

- Does not rename `speed_variance` (item 1 above).
- Does not consolidate the V2 `pipeline.enter.duration_ms` shape into the V3 `pipeline.timing.enter_ms` shape — that's a separate cleanup. Packet 68 already shipped lenient parsing for both.
- Does not change the `SignalOrFloat` semantics for non-content-effect surfaces (shaders/filters keep their existing types).
- Does not introduce a new content-effect lifecycle separate from phases (Option 4 from the prior chat thread). Effects stay coupled to phases; the new flexibility is in HOW the phase ends.
- Does not implement multi-effect content composition. The `which: Option<String>` field in `EffectComplete` is reserved for that future, but multi-effect composition is its own design packet. **0.2.0:** v0 validator REJECTS `which: Some(_)`.
- **(NEW 0.2.0)** Does not unify the motion-vs-pipeline duration sources. They remain two coexisting authoring shapes; this packet only ensures `PhaseTerminator` lands on both so neither silently shadows the other.
- **(NEW 0.2.0)** Does not address the `Dissolve` "complete = obscured" semantic question without a project decision (§11 D6).

---

## 10. Suggested next steps (0.2.0 sequencing)

1. **Carve out packet 69-A — Phase A only — and ship it standalone.** Bindable parity + the TODO closure. Scope is now ~7 sibling files + ~3 recipe-side files + tests. Lands cleanly without touching the phase scheduler. Closes the load-bearing gap (host can drive content-effect rates) without taking on any of §11's open decisions.
2. **Lock §11 outstanding decisions D1–D9 before Phase C.** Especially D1 (V3 sampler architecture) and D2 (motion-vs-pipeline shadowing) — these are structural and cannot be deferred mid-implementation.
3. **Defer Phase B until after the SplitFlap cadence design call (§11 D9).**
4. **Defer Phase C until §11 D1 + D2 + D3 + D4 are answered in writing.**
5. **Phase D lands last** — recipe migration + docs + MARKETING.md update.

---

## 11. Outstanding decisions to lock before Phase C (NEW in 0.2.0)

These are the structural decisions Phase C cannot proceed without. Each must be answered in writing — preferably as an amendment to this packet — before any code lands.

### D1. V3 sampler architecture — pure-function vs stateful

**The problem:** V3 has no scheduler. `sampled_v3_playback_timing_from_elapsed(compiled, elapsed)` is a pure function called per-frame. It has no state across frames. To support `EffectComplete`, the sampler needs to know "did the transformer finish at the previous frame?" — but the previous frame's answer has nowhere to live in the current signature.

**Three options:**
- **D1.a Pure-function path (RECOMMENDED).** Sampler instantiates the transformer via `get_transformer(canonical_effect)`, calls `is_complete(progress_at_elapsed, ctx)`, and discards. Renderer also instantiates one for actual rendering. Cost: one extra small allocation per frame per recipe. Benefit: zero new state plumbing; `sampled_v3_playback_timing_from_elapsed` stays nearly pure (just gains a `runtime_params` parameter).
- **D1.b State-channel path.** Add `&PhaseScheduleState` parameter that callers carry across frames. Touches every caller of the sampler (~7 sites). New state struct lives somewhere — probably `DirectV3PreviewState`. Cost: invasive; requires every V3 caller to care about scheduler state.
- **D1.c Stateful object.** Replace `sampled_v3_playback_timing_from_elapsed` with `DirectV3Scheduler::tick(elapsed)`. Most invasive, most honest, hardest to retrofit.

**Recommendation: D1.a.** The transformer-instantiation cost is small (a `Box<dyn TextTransformer>`); the design wins are large (callers don't care; signature stays understandable; pure-function semantics preserved).

**Lock this before any Phase C code lands.**

### D2. Motion-vs-pipeline duration shadowing

**The problem:** `cls_v3_playback_timing.rs:75-87` reads `motion.enter.duration_ms` first, then falls back to `pipeline.timing.enter_ms`. A `PhaseTerminator` placed only on `pipeline.timing.enter` is silently shadowed when motion is present.

**Three options:**
- **D2.a Land terminator on both schemas.** `V3PipelineTiming.enter` AND `V3MotionPhaseSpec.terminator` both become `Option<PhaseTerminator>`. Sampler reads motion's terminator first, then pipeline's. Maintains current precedence; both paths support all terminator variants.
- **D2.b Invert precedence.** Pipeline-side terminator wins over motion-side `duration_ms`. Documented behavior change; existing motion-bearing recipes might shift their phase boundaries.
- **D2.c Land terminator on pipeline only; deprecate motion-side `duration_ms`.** Forces all recipes to migrate timing into pipeline.timing. Scope larger; touches every motion-bearing recipe.

**Recommendation: D2.a.** Smallest behavior delta; preserves the existing precedence; both authoring shapes remain idiomatic.

### D3. `deny_unknown_fields` and the legacy field rename

**The problem:** Both `V3PipelineTiming` and `V3MotionPhaseSpec` are `#[serde(default, deny_unknown_fields)]`. A new field `enter` cannot silently coexist with the old `enter_ms` — the parser rejects whichever isn't declared.

**Option:**
- **D3.a Custom deserializer + `#[serde(alias)]` on the new field.** The new `enter: Option<PhaseTerminator>` field declares `#[serde(alias = "enter_ms")]`, and the type's deserializer accepts both the bare-number form (legacy) and the tagged-union form (new). Round-trip: bare number → `Duration { ms: Literal(n) }`; tagged form → as-authored.

**Recommendation: D3.a — and write a tested round-trip suite proving every legacy recipe still parses byte-identically.** Without this proof Phase C should not ship.

### D4. V2 — does it grow the new shape, or stay time-only?

**The problem:** V2's enter/exit duration comes from `Animated::profile().enter: TransitionSpec.duration_ms` — a Rust trait return value backed by `tui-vfx-geometry::TransitionSpec`. There is no JSON authoring field on the recipe side. Extending V2 to honor `PhaseTerminator` requires evolving `TransitionSpec` in a third repo.

**Three options:**
- **D4.a V2 grows the shape via `tui-vfx-geometry::TransitionSpec` evolution.** Add `terminator: Option<PhaseTerminator>` to `TransitionSpec`; V2 lifecycle.tick consults it before falling back to `duration_ms`. Cross-repo change; needs `PhaseTerminator` to live in a shared crate (probably `tui-vfx-core` or a new shared types crate).
- **D4.b V2 stays time-only forever.** PhaseTerminator is documented as a V3-only feature. V2 recipes that want host-driven dwell continue to use the imperative `AnimationManager.dismiss(id, now)` escape hatch.
- **D4.c V2 grows a recipe-side override.** Add a JSON-authoring field at the V2 recipe envelope level that overrides the trait-supplied duration. Smaller scope but adds a parallel timing surface to V2.

**Recommendation: D4.b in the immediate term.** V2 is the legacy path. New host-driven workflows should use V3. Documenting the limitation is honest; spending Phase C effort on cross-repo trait evolution is not justified by current use cases. Re-evaluate after V3 cutover.

### D5. Marquee + EffectComplete — validator-rejected, not runtime-fallback

**The problem:** Marquee has no `progress >= 1.0` early return (`cls_marquee.rs:33-78`); it scrolls perpetually. Pairing `EffectComplete` with Marquee guarantees a hang to `fallback_ms`. The 0.1.0 recommendation (override `is_complete = false`) makes the hang the documented behavior.

**Recommendation: validator REJECTS the combination.** The recipe author cannot author this combination at all — error at recipe-load time, not at runtime. Marquee belongs to a "continuous class" that should be paired with `Duration` or `Binding` terminators only. Document the class membership in Marquee's docstring.

This generalizes: any future continuous-class effect (perpetual scroll, perpetual oscillation) joins the same validator gate.

### D6. Dissolve — "complete" means obscured, not settled

**The problem:** Dissolve at `progress >= 1.0` returns *fully dissolved* text (target text gone). "Complete" semantically = "fully obscured", not "settled on target". The default `is_complete = progress >= 1.0` would fire when the text is invisible, which may or may not be what `EffectComplete` is meant to mean.

**Three options:**
- **D6.a Dissolve uses the default.** "Complete" means "the dissolve animation finished" regardless of what the user sees. Phase ends when text is fully obscured. Consistent with the trait semantics.
- **D6.b Dissolve overrides `is_complete = false` always.** Dissolve never reports complete; pairing with `EffectComplete` always falls through to `fallback_ms`. Treats dissolve as continuous-class even though it's bounded.
- **D6.c Dissolve doesn't pair with `EffectComplete` at all.** Validator rejects the combination, like Marquee.

**Recommendation: D6.a.** Cleanest semantics; the trait method describes whether the effect's animation is done, not whether the text is visible. Recipe authors who want "phase ends when text is restored" should use a different effect (Morph, Typewriter) for the enter side.

### D7. `Binding` terminator latch semantics

**The problem:** What happens if the host sets `user_dismissed: true` for one tick and then `false` for the next? Three behaviors are defensible:

**Three options:**
- **D7.a Edge-triggered + latched (RECOMMENDED).** Once `true` is observed, the terminator stays fired for the phase. Robust against host glitches.
- **D7.b Level-triggered.** Phase ends only while binding is true. A host glitch that drops the value back to `false` un-dismisses the recipe. Foot-gun.
- **D7.c Edge-triggered + non-latched.** Rare host transient = no-op. Hides bugs.

**Recommendation: D7.a.** Document explicitly in the wire-shape docstring.

### D8. `EffectComplete.which` field name

**The problem:** Today no `ContentEffect` variant has an `id` field. The `which: Option<String>` lookup addresses an id space that doesn't exist. When multi-effect composition arrives, the id space might be `CompiledStep` ids (which DO exist) or new content-effect ids (which don't).

**Three options:**
- **D8.a Keep `which: Option<String>`.** Generic, future-proof, ambiguous.
- **D8.b Rename to `step_id: Option<String>`.** Matches the only existing id space. Honest about what's planned.
- **D8.c Rename to `effect_id: Option<String>`.** Anticipates the future content-effect id space.

**Recommendation: D8.b.** `CompiledStep` ids exist today; multi-effect composition will likely thread through the step tree anyway. If a content-effect-id space arrives later, add `effect_id` as a parallel field; never rename `step_id`.

### D9. `cadence_ms` overlap with SplitFlap's existing knobs

**The problem:** SplitFlap has 4 timing knobs today: `speed`, `cascade`, `cycles`, `jitter` (`cls_content_effect.rs:274-374`). Adding `cadence_ms` requires defining how it composes with each.

**Possible answers:**
- **D9.a `cadence_ms` overrides `speed`.** When `cadence_ms` is set, `speed` is ignored and per-flap timing comes from `cadence_ms`. `cascade` and `jitter` continue to work as variance over the cadence.
- **D9.b `cadence_ms` is additive.** When set, a multiplier on `speed`. Composes by multiplying rates.
- **D9.c `cadence_ms` is exclusive.** Validator rejects coexistence with `speed`.

**Recommendation: defer to a SplitFlap design call.** Phase B should not land cadence_ms on SplitFlap until this is decided. Other rate-bearing effects (Typewriter, Marquee) have a single `speed` or `speed_variance` knob and can adopt `cadence_ms` cleanly without an interaction question.

---

## 12. Verified-shape addenda (NEW in 0.2.0)

This section captures the verified shape of every load-bearing data structure the design touches. Useful as a reference during implementation; sourced from end-to-end reads, not citations.

### 12.1 Per-effect `is_complete` correctness — see §4 Q2 table

(Moved into §4 Q2 above for cohesion.)

### 12.2 V3 sampler — pure-function, no state

```rust
// cls_v3_playback_timing.rs:71-138 (current)
pub fn sampled_v3_playback_timing_from_elapsed(
    compiled: &CompiledRecipePlan,
    elapsed: Duration,
) -> V3PlaybackTiming { ... }

// V3PlaybackTiming carries phase, sample_t, loop_t, absolute_t_ms (line 14-20).
// Returns a SignalContext via .signal_context(width, height, frame, seed)
// (lines 45-57) where absolute_t = absolute_t_ms (line 54). Unit: milliseconds.
```

The chief caller is `DirectV3PreviewState::update_from_elapsed` (`cls_direct_v3_preview_state.rs:198-214`). It carries `phase, sample_t, loop_t, absolute_t_ms, runtime_overrides, snapshot` across frames — but **no transformer or scheduler state**. Adding `runtime_params` to the sampler signature is a one-line caller change since `runtime_overrides.runtime_params` is already in scope.

### 12.3 V3 phase duration sources — three places

```rust
// cls_v3_playback_timing.rs:75-87 (current logic)
let enter_ms = motion
    .as_ref()
    .and_then(|motion| motion.enter.as_ref().map(|phase| phase.duration_ms))   // (1) motion
    .or_else(|| pipeline_timing_u64(compiled, "enter_ms"))                      // (2) pipeline
    .unwrap_or(1000)                                                            // (3) default
    .max(1);
```

Source (1): `V3MotionPhaseSpec.duration_ms: u64` (`cls_v3_motion_envelope.rs:155-158`). Wire shape: `{"motion": {"enter": {"duration_ms": 6000, ...}}}`. Strict deserializer (`#[serde(default, deny_unknown_fields)]`).

Source (2): `V3PipelineTiming.enter_ms: Option<u64>` (`cls_v3_recipe_document.rs:248-258`). Wire shape: `{"pipeline": {"timing": {"enter_ms": 6000}}}`. Strict deserializer.

Source (3): hard-coded 1000ms.

**The motion path takes precedence.** Any pipeline-only PhaseTerminator change is silently shadowed for motion-bearing recipes.

### 12.4 `deny_unknown_fields` strictness — both schemas

```rust
// cls_v3_recipe_document.rs:217
#[serde(default, deny_unknown_fields)]
pub struct V3PipelineTiming { ... }

// cls_v3_motion_envelope.rs:153
#[serde(default, deny_unknown_fields)]
pub struct V3MotionPhaseSpec { ... }
```

Implication: silent unknown-field acceptance does NOT exist on either surface. Renames need explicit `#[serde(alias = "...")]` plus a custom deserializer to handle the legacy bare-number shape. See §11 D3.

### 12.5 V2 duration source — Rust trait, not JSON

```rust
// types/animation_profile.rs:13-66
pub struct AnimationProfile {
    pub enter: TransitionSpec,    // duration_ms lives here
    pub exit: TransitionSpec,
    // ... legacy + style-layer fields ...
    pub loop_period: Option<Duration>,
}

// traits/mod.rs:20-40
pub trait Animated {
    fn profile(&self) -> &AnimationProfile;
    // ... other methods ...
}

// state/lifecycle.rs:53-55
fn enter_duration(&self) -> Duration {
    Duration::from_millis(self.item.profile().enter.duration_ms)
}
```

`TransitionSpec.duration_ms` is in **tui-vfx-geometry** (verified by import at `animation_profile.rs:9`). V2 has no recipe-side authoring field for phase duration; the duration is supplied by whatever implements `Animated`. Phase C touching V2 means cross-repo trait evolution.

### 12.6 Compiled plan — same DTO types as authoring

```rust
// cls_compiled_recipe_plan.rs:120-127
pub struct CompiledPipelinePlan {
    pub timing: Option<V3PipelineTiming>,    // SAME type as authoring DTO
    pub step: Option<CompiledStep>,
}

// line 81-83
pub struct CompiledEnvelope {
    // ...
    pub motion: Option<V3MotionEnvelope>,    // SAME type as authoring DTO
    // ...
}
```

`NormalizedRecipe` also uses the same types (`cls_normalized_recipe.rs:135` for pipeline timing, `cls_normalized_recipe.rs:96` for motion envelope). Schema changes propagate from authoring → normalized → compiled for free. **0.1.0 missed this win.**

### 12.7 Bindable `From<SignalOrFloat>` — collapse semantics

```rust
// cls_bindable.rs:482-489
impl From<SignalOrFloat> for VfxBindable<f32, SignalOrFloat> {
    fn from(value: SignalOrFloat) -> Self {
        match value {
            SignalOrFloat::Static(v) => Self::Literal(v),
            other => Self::Signal(other),
        }
    }
}
```

This is what makes Phase A's field-type swap byte-compatible with existing recipes. `SignalOrFloat::Static(0.0)` → `VfxBindable::Literal(0.0)`; signal expressions stay in `Signal`. The `BareSignal` lenient deserializer (`cls_bindable.rs:213-215`) handles bare object shapes like `{"sine": ...}` by routing them into `Signal` (the historical `{"signal": {...}}` shape also works via the tagged form).

### 12.8 `apply` family — current shape

```rust
// fnc_apply_content_effect.rs (v2.0.0)
impl ContentEffect {
    pub fn apply(&self, target: &str, progress: f64) -> String { ... }                  // line 48
    pub fn apply_to_borrowed<'a>(&self, target: &'a str, progress: f64) -> Cow<'a, str> { ... }  // line 67
    pub fn apply_with_runtime<'a>(                                                       // line 86
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
        runtime_params: &ShaderRuntimeParams,
    ) -> Cow<'a, str> { ... }
}
```

`apply` and `apply_to_borrowed` default-construct empty `SignalContext` and `ShaderRuntimeParams`. `apply_with_runtime` is the host-injection path. **The 0.1.0 reference to `apply_with_context` is stale** — that name was removed in v2.0.0.

---

## 13. File inventory — every file that may be touched (NEW in 0.2.0)

This section lists every file the implementation phases will modify, with verified version + line ranges. Useful for the implementation packet's surgery plan.

### 13.1 tui-vfx (sibling)

| File | Current version | Phase | Surgery |
|---|---|---|---|
| `crates/tui-vfx-content/src/types/cls_content_effect.rs` | 2.15.0 | A | Field-type swap on rate-bearing variants (Typewriter, Scramble, GlitchShift, ScrambleGlitchShift, SplitFlap, Marquee) |
| `crates/tui-vfx-content/src/types/fnc_apply_content_effect.rs` | 2.0.0 | A (cleanup only) | Doc-comment cleanup; `apply_with_runtime` API already correct |
| `crates/tui-vfx-content/src/transformers/cls_typewriter.rs` | 4.0.0 | A | One-line change in `transform()` body for the `evaluate` call |
| `crates/tui-vfx-content/src/transformers/cls_scramble.rs` | 3.1.0 | A | Same |
| `crates/tui-vfx-content/src/transformers/cls_glitch_shift.rs` | 2.2.0 | A, C | Phase A: evaluate call. Phase C: override `is_complete` |
| `crates/tui-vfx-content/src/transformers/cls_scramble_glitch_shift.rs` | 3.2.0 | A, C | Same |
| `crates/tui-vfx-content/src/transformers/cls_marquee.rs` | 2.1.0 | A | Phase A only — Phase C handles via validator (no override) |
| `crates/tui-vfx-content/src/transformers/cls_split_flap.rs` | 3.5.0 | A, B | Phase A: 3 evaluate call sites. Phase B: cadence_ms after §11 D9 |
| `crates/tui-vfx-content/src/transformers/cls_numeric.rs` | 1.1.0 | C | Override `is_complete = true` always |
| `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs` | 3.11.0 | A | Type-name only; constructor calls already clone() the field |
| `crates/tui-vfx-content/src/traits/text_transformer.rs` | 3.0.0 | C | Add `is_complete` default method |
| `crates/tui-vfx-content/src/traits/cls_transform_context.rs` | 1.0.0 | — | No change |
| `crates/tui-vfx-core/src/bindable/cls_bindable.rs` | 0.1.3 | — | No change (the family is already complete for our needs) |
| `crates/tui-vfx-style/src/traits/cls_shader_context.rs` | 2.2.0 | — | No change |
| `steering/MARKETING.md` | (verify) | D | Add phase-termination flexibility line |

### 13.2 tui-vfx-recipes (this repo)

| File | Current version | Phase | Surgery |
|---|---|---|---|
| `src/preview/fnc_resolve_content_text.rs` | 0.2.0 | A | Close the `slice-6.6-followup` TODO; thread `&ShaderRuntimeParams` from caller |
| `src/preview/fnc_render_preview_item.rs` | 2.1.0 | A | V2 caller: thread runtime params into `resolve_content_text` call (line 45-53) |
| `src/v3/compile/cls_v3_source_surface.rs` | (verify) | A | V3 caller: thread runtime params into `resolve_content_text` call (line 703-711) |
| `src/v3/authoring/cls_v3_recipe_document.rs` | 0.2.0 | C | Replace `enter_ms`/`exit_ms` with `enter`/`exit: Option<PhaseTerminator>`; add `dwell` field; alias scheme per §11 D3 |
| `src/v3/compile/cls_v3_motion_envelope.rs` | (verify) | C | Replace `duration_ms` with `terminator: Option<PhaseTerminator>` on `V3MotionPhaseSpec`; alias scheme |
| `src/v3/compile/cls_v3_playback_timing.rs` | 0.1.1 | C | Sampler signature gains `runtime_params`; terminator-aware logic per §11 D1 |
| `src/v3/compile/cls_compiled_recipe_plan.rs` | 0.6.0 | — | No change (carries V3PipelineTiming + V3MotionEnvelope by reference) |
| `src/v3/normalized/cls_normalized_recipe.rs` | 0.5.0 | — | No change (same — carries the same DTOs) |
| `src/v3/authoring/cls_phase_terminator.rs` | (NEW) | C | New file: enum + serde + ConfigSchema |
| `src/state/lifecycle.rs` | 0.3.0 | C (only if D4.a chosen) | V2 tick consults terminator before falling back to duration |
| `src/v3/validate/...` | various | B, C | New rules: cadence_ms × char_count > duration_ms; Marquee + EffectComplete rejection; Binding terminator missing fallback warning |
| `src/preview/cls_direct_v3_preview_state.rs` | 0.6.0 | C | Pass `runtime_overrides.runtime_params` into the new sampler signature |

### 13.3 tui-vfx-geometry (NEW dependency in 0.2.0)

| File | Phase | Surgery |
|---|---|---|
| `tui-vfx-geometry::TransitionSpec` | C (only if D4.a chosen) | Add `terminator: Option<PhaseTerminator>` field |

### 13.4 mixed-signals

| File | Phase | Surgery |
|---|---|---|
| `mixed-signals/src/traits/signal.rs` (`SignalContext.absolute_t`) | — | No change (verified 0.2.0 — field exists at line 74; V3 already writes ms into it) |

### 13.5 gt-design (downstream consumer per Intention 41)

| Phase | Audit |
|---|---|
| C | `ofpf-content` audit for any consumer of `enter_ms`/`exit_ms`/`duration_ms` field names; expected zero hits |

---

---

## 14. Carve-out packets (NEW in 0.3.0)

To deliver real value before §11 design holes are resolved, the work has been carved into two structurally independent packets that ship in parallel:

### 14.1 packet 69-A — Bindable parity for content effects

**File:** `steering/work-packets/69-A-content-effect-bindable-parity.md`

**Delivers:** apps can supply runtime values for any rate-bearing content-effect parameter (`Typewriter.speed_variance`, `Marquee.speed`, `SplitFlap.{speed, cascade, cycles}`, etc.) per frame. Same recipe drives different host states.

**Does NOT deliver event-driven phase advancement.** Phases stay strictly time-driven.

**Scope:** ~13 files (8 sibling, 5 recipes-side). No `tui-vfx-geometry` involvement. No scheduler changes.

**Independent of:** packet 69-E and the full Phase C design.

### 14.2 packet 69-E — Event-driven dwell (minimal slice, deliberately a hack)

**File:** `steering/work-packets/69-E-event-driven-dwell.md`

**Delivers:** V3 recipes can declare `dwell_until_binding: "name"` + `dwell_fallback_ms: N` on `pipeline.timing`. Host sets the binding truthy via `state.set_runtime_params(...)` and the dwell phase advances on the next frame. Latched semantics, validator-rejected loop+binding pairing.

**Does NOT deliver:** enter/exit terminators, EffectComplete, AnyOf/AllOf, V2 parity, or a typed `PhaseTerminator` enum.

**Scope:** 6 production recipe files (1 new) + 1 demo recipe. Sibling repo zero changes. tui-vfx-geometry zero changes.

**Sidesteps every §11 design hole** because the chosen narrow scope happens to dodge each one (D1: latch lives in `DirectV3PreviewState`, sampler stays pure with one new optional `dwell_override_ms` param; D2: dwell has no motion counterpart; D3: additive fields, no rename; D4: V3 only; D5–D8: irrelevant to Binding-only).

**Migrates cleanly** when the full design lands — the two flat fields become serde aliases on the future tagged-union shape. No recipe migration breakage.

### 14.3 What the two carve-outs do NOT cover

- `EffectComplete` terminator (gated on §11 D1 — V3 sampler architecture).
- `AnyOf` / `AllOf` composition (waits for full vocabulary).
- Enter and exit phase terminators (waits for full vocabulary).
- `cadence_ms` field on transformers (packet 69 Phase B; gated on §11 D9 — SplitFlap overlap design).
- V2 phase termination via `tui-vfx-geometry::TransitionSpec` evolution (gated on §11 D4 — recommended deferred indefinitely).
- The motion-vs-pipeline duration shadowing fix (gated on §11 D2 — needed before Phase C, not before 69-E since dwell is unaffected).

### 14.4 Sequencing recommendation

1. **Now:** ship 69-A and 69-E in parallel. Both can land independently. Either order works; both can be in flight simultaneously since they touch disjoint surfaces.
2. **Next:** lock §11 D1–D9 in writing as amendments to this packet.
3. **After:** Phase B (cadence_ms) once D9 (SplitFlap overlap) resolves.
4. **After that:** Phase C (full PhaseTerminator) once D1, D2, D3, D4 resolve.
5. **Phase D (recipe migration + docs):** lands last, including the MARKETING.md update in `/usr/projects/tui-vfx/steering/MARKETING.md`.

---

<!-- <FILE>steering/work-packets/69-content-effect-cadence-and-phase-terminators.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
