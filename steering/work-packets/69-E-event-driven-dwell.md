<!-- <FILE>steering/work-packets/69-E-event-driven-dwell.md</FILE> - <DESC>Carve-out of Packet 69 minimal slice: add a host-driven dwell terminator (dwell_until_binding + dwell_fallback_ms) to V3 recipes so apps can advance the dwell phase when an event fires, without taking on the full PhaseTerminator design or any of the V3 sampler statelessness questions.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>2026-04-28 carve-out from packet 69 v0.2.0. Aggressive scope reduction: ONE terminator type (binding-only), ONE phase (dwell), ONE schema location (V3 pipeline.timing), as new additive fields (no rename, no deny_unknown_fields blocker). Sidesteps every architectural design hole in §11 by relying on the latch-state living in DirectV3PreviewState, NOT the pure sampler.</WCTX> -->
<!-- <CLOG>0.1.0: initial draft. Scope and file inventory verified by full reads of every cited file in tui-vfx-recipes. Acknowledged short-term hack — buys real usability while the full PhaseTerminator architecture is still being designed.</CLOG> -->

# Packet 69-E — Event-driven dwell (minimal-slice hack)

**Status:** ready to implement.
**Parent:** packet 69 v0.2.0 (minimal slice acknowledged short-term).
**Scope:** V3 only, recipe-side only. No sibling repo changes.
**Independent of:** packet 69-A. Both can ship in parallel.
**Honest framing:** This is a deliberately narrow hack. It delivers ~80% of the user-visible "event flag" capability while sidestepping every architectural design hole the full PhaseTerminator design needs to resolve. When the full design lands later, the wire shape introduced here can be deprecated cleanly via serde aliases — no recipe migration breakage.

---

## 1. What this packet enables

After this packet ships, a V3 recipe can declare that its dwell phase ends when a host-supplied binding fires:

```json
"pipeline": {
  "timing": {
    "enter_ms": 1500,
    "dwell_until_binding": "user_dismissed",
    "dwell_fallback_ms": 60000,
    "exit_ms": 800
  }
}
```

The host app calls `state.set_runtime_params(params)` with `params["user_dismissed"] = Boolean(true)` (or `Integer(1)`, or any non-empty `Text`). On the next frame, the recipe transitions Dwelling → Exiting. If the binding never fires, the dwell phase advances after `dwell_fallback_ms` milliseconds (default 60000 if unset).

This is a real, recipe-authorable replacement for the imperative V2 escape hatch `AnimationManager.dismiss(id, now)`.

---

## 2. What this packet deliberately does NOT do

This is a hack. It buys time for the full design. Documented limitations:

- **Only the dwell phase.** Enter and exit phases stay strictly time-driven.
- **Only the `Binding` terminator type.** No `EffectComplete`, no `AnyOf`/`AllOf` composition. Those need the V3 sampler architecture decision (packet 69 §11 D1).
- **V3 only.** V2 recipes continue to use `AnimationManager.dismiss(id, now)`. Touching V2 means evolving `tui-vfx-geometry::TransitionSpec` — out of scope.
- **Pipeline-only (not motion).** A recipe that authors duration in `motion.enter` is unaffected (motion has no dwell phase, so this is a non-issue for the dwell case specifically).
- **No new top-level enum.** The wire shape uses two new optional flat scalar fields on `V3PipelineTiming`, not a tagged-union `PhaseTerminator` enum. Future packets can add the enum without breaking these recipes.
- **No loop+binding pairing.** Validator rejects: a recipe with `lifecycle.loop = true` AND `dwell_until_binding` set. Latch semantics in a looping recipe are undefined; rejecting at recipe-load time is cleaner than runtime confusion.

---

## 3. Why this is structurally easy (sidesteps every §11 design hole)

| Packet 69 §11 design hole | Why 69-E sidesteps it |
|---|---|
| **D1: V3 sampler statelessness** | The sampler stays pure. Latch state lives in `DirectV3PreviewState.dwell_terminator_fired_at_ms: Option<f64>`. Sampler gains ONE new optional parameter (`dwell_override_ms: Option<u64>`) — when present, it overrides `auto_dismiss_ms` for dwell duration. The CALLER decides when to set this override based on its own latch state. |
| **D2: Motion-vs-pipeline shadowing** | Dwell has no motion-side counterpart. `V3MotionEnvelope` only carries `enter` and `exit` (verified at `cls_v3_motion_envelope.rs:222-230`). Adding `dwell_until_binding` to `pipeline.timing` shadows nothing. |
| **D3: `deny_unknown_fields` blocker** | This packet ADDS new optional fields, never RENAMES existing ones. `deny_unknown_fields` rejects unknown keys; new declared fields are not unknown. Zero alias scheme needed. |
| **D4: V2 / tui-vfx-geometry** | V2 stays time-only. Documented limitation. No third-repo change. |
| **D5: Marquee + EffectComplete** | EffectComplete is not in this packet. Marquee + Binding-on-dwell is fine — Marquee's continuous semantics are independent of dwell-phase-end signal. |
| **D6: Dissolve "complete" semantics** | Same — EffectComplete is not in this packet. |
| **D7: Binding latch semantics** | Decided up front: latched. Once observed truthy during dwell, stays fired for that dwell phase. State lives in `DirectV3PreviewState`, reset on `state.reset()`. |
| **D8: `EffectComplete.which` id space** | Not relevant — no effect-completion terminator in this packet. |

Every design hole that gates the full Phase C is either irrelevant to this slice or solved by the simplest possible answer. That's the entire reason this packet can ship now.

---

## 4. Implementation plan

### 4.1 Schema additions

**File:** `src/v3/authoring/cls_v3_recipe_document.rs` (current v0.2.0)

Add two new optional fields to `V3PipelineTiming` (the struct at line 248-311). Both are flat scalars, both `Option`, both carry a sensible default. The struct's `#[serde(default, deny_unknown_fields)]` (line 217) does NOT block additive fields — only rejects unrecognized keys. New declared keys are recognized.

```rust
pub struct V3PipelineTiming {
    // ... existing fields (enter_ms, exit_ms, etc.) unchanged ...

    /// Optional name of a runtime binding whose truthy value advances
    /// the dwell phase early. Latched: once observed truthy during a
    /// dwell phase, stays fired for that dwell phase.
    ///
    /// Pairs with `dwell_fallback_ms`: if the binding never fires, dwell
    /// ends after `dwell_fallback_ms` milliseconds (default 60000).
    ///
    /// V3-only. Validator rejects pairing with `lifecycle.loop = true`.
    /// See packet 69-E for the full design rationale and the relationship
    /// to the broader PhaseTerminator design (packet 69 v0.2.0).
    #[config(help = "Runtime binding name that advances dwell when truthy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dwell_until_binding: Option<String>,

    /// Defensive cap on dwell duration when `dwell_until_binding` is set.
    /// If the binding never fires, dwell still advances after this many
    /// milliseconds. Default: 60000 (60 seconds).
    ///
    /// Ignored when `dwell_until_binding` is None.
    #[config(help = "Defensive cap on dwell duration in milliseconds when dwell_until_binding is set", min = 1000, max = 600000)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dwell_fallback_ms: Option<u64>,
}
```

These fields propagate through `NormalizedRecipe` (`cls_normalized_recipe.rs:135` carries `Option<V3PipelineTiming>` — same DTO type) and `CompiledRecipePlan` (`cls_compiled_recipe_plan.rs:120-127` — same DTO type) for free. No normalize-stage or compile-stage code changes.

### 4.2 Sampler signature extension

**File:** `src/v3/compile/cls_v3_playback_timing.rs` (current v0.1.1)

Function `sampled_v3_playback_timing_from_elapsed` at line 71-138 today reads dwell duration from `auto_dismiss_ms` (line 88-93):

```rust
let dwell_ms = timing_u64(
    compiled.envelope.lifecycle.as_ref(),
    "auto_dismiss_ms",
    1000,
).max(1000);
```

Add ONE new optional parameter `dwell_override_ms: Option<u64>` and short-circuit the lookup when present:

```rust
pub fn sampled_v3_playback_timing_from_elapsed(
    compiled: &CompiledRecipePlan,
    elapsed: Duration,
    dwell_override_ms: Option<u64>,  // NEW
) -> V3PlaybackTiming {
    // ... existing motion/pipeline_timing lookup for enter_ms / exit_ms ...

    let dwell_ms = dwell_override_ms
        .unwrap_or_else(|| {
            timing_u64(
                compiled.envelope.lifecycle.as_ref(),
                "auto_dismiss_ms",
                1000,
            )
        })
        .max(1000);

    // ... rest unchanged ...
}
```

When `dwell_override_ms` is `None`, behavior is byte-identical to today. When `Some(N)`, the dwell phase ends after N ms instead of `auto_dismiss_ms`.

The sampler stays pure. It does NOT look at `runtime_params`, does NOT track latches, does NOT know about bindings. The caller (DirectV3PreviewState) computes `dwell_override_ms` and hands it in.

Same parameter on `initial_v3_playback_timing` at line 61-68 (initial timing carries no fired-at-yet info, so callers pass `None`).

### 4.3 Latch state in `DirectV3PreviewState`

**File:** `src/preview/cls_direct_v3_preview_state.rs` (current v0.6.0)

Add three pieces:

**A. Cache the binding name and fallback at construction time.** Walk `compiled.pipeline.timing` to read `dwell_until_binding` and `dwell_fallback_ms`. Store on the state:

```rust
pub struct DirectV3PreviewState {
    // ... existing fields ...

    /// Cached from compiled.pipeline.timing.dwell_until_binding for fast
    /// per-frame check. None if the recipe doesn't declare event-driven
    /// dwell.
    dwell_binding_name: Option<String>,
    /// Cached fallback cap. Defaults to 60000 if dwell_until_binding is
    /// set but dwell_fallback_ms is omitted.
    dwell_fallback_ms: u64,
    /// Latch: elapsed_ms at which the binding was first observed truthy
    /// during the current dwell phase. None until fired.
    /// Reset by `reset()` and by phase transitions out of Dwelling.
    dwell_terminator_fired_at_ms: Option<f64>,
}
```

Populate `dwell_binding_name` and `dwell_fallback_ms` in both `from_compiled_with_runtime_overrides` (line 54-93) by reading `compiled.pipeline.timing.as_ref().and_then(|t| t.dwell_until_binding.clone())` etc.

**B. Update `update_from_elapsed`** (line 198-214) to compute the override and reset the latch on phase transitions:

```rust
pub fn update_from_elapsed(&mut self, elapsed: Duration) -> Result<(), RenderCompiledPlanError> {
    if self.paused { return Ok(()); }

    // 1. Compute the dwell override (if event-driven dwell is configured).
    let dwell_override_ms = self.compute_dwell_override_ms(elapsed);

    // 2. Sample timing with the override.
    let timing = sampled_v3_playback_timing_from_elapsed(
        &self.compiled,
        elapsed,
        dwell_override_ms,
    );

    // 3. Reset latch when leaving Dwelling phase (forward or via reset()).
    if self.phase == AnimationPhase::Dwelling
        && timing.phase != AnimationPhase::Dwelling
    {
        self.dwell_terminator_fired_at_ms = None;
    }

    if self.matches_timing(&timing) { return Ok(()); }
    self.phase = timing.phase;
    self.sample_t = timing.sample_t;
    self.loop_t = timing.loop_t;
    self.absolute_t_ms = timing.absolute_t_ms;
    self.rerender()
}

fn compute_dwell_override_ms(&mut self, elapsed: Duration) -> Option<u64> {
    let binding_name = self.dwell_binding_name.as_ref()?;
    // Latch: once fired, stay fired for this dwell phase.
    if self.dwell_terminator_fired_at_ms.is_none()
        && self.phase == AnimationPhase::Dwelling
        && binding_is_truthy(&self.runtime_overrides.runtime_params, binding_name)
    {
        // Compute when the binding fired in dwell-local time.
        // (For the minimal slice: we treat "fired this frame" as "fired at
        //  current dwell elapsed". Sub-frame precision not required.)
        let elapsed_ms = elapsed.as_millis() as f64;
        self.dwell_terminator_fired_at_ms = Some(elapsed_ms);
    }
    // When fired: snap dwell to the elapsed-at-fire (effectively "end
    // dwell now"). Otherwise: cap dwell at fallback_ms.
    if self.dwell_terminator_fired_at_ms.is_some() {
        // Compute dwell_local_ms based on enter_ms and elapsed_at_fire.
        // Simplest: pass a very small dwell_override (1ms) so the next
        // sampler call advances out of dwell immediately.
        Some(1)
    } else {
        Some(self.dwell_fallback_ms)
    }
}
```

**C. Helper: `binding_is_truthy`** in the same file:

```rust
fn binding_is_truthy(params: &ShaderRuntimeParams, key: &str) -> bool {
    use tui_vfx_style::traits::ShaderRuntimeParamValue;
    match params.get(key) {
        Some(ShaderRuntimeParamValue::Boolean(b)) => *b,
        Some(ShaderRuntimeParamValue::Integer(n)) => *n != 0,
        Some(ShaderRuntimeParamValue::Float(f)) => *f != 0.0 && f.is_finite(),
        Some(ShaderRuntimeParamValue::Text(s)) => !s.is_empty(),
        Some(ShaderRuntimeParamValue::Rgb { .. }) => true,  // any present color = truthy
        None => false,
    }
}
```

The truthiness rules use the `ShaderRuntimeParamValue` enum verbatim (verified at `tui-vfx-style/src/traits/cls_shader_context.rs:23-39` — the enum has Integer, Float, Boolean, Text, Rgb variants). No new accessor on `ShaderRuntimeParams` needed.

**D. Update `reset()`** at line 187-195 to clear the latch:

```rust
pub fn reset(&mut self) -> Result<(), RenderCompiledPlanError> {
    self.paused = false;
    self.dwell_terminator_fired_at_ms = None;  // NEW
    let timing = initial_v3_playback_timing(&self.compiled, None);
    // ... rest unchanged ...
}
```

### 4.4 Other sampler call-site updates

The new `dwell_override_ms` parameter propagates to every sampler caller. Verified callers in production code (excluding tests):

| File | Line | Pass for non-69-E callers |
|---|---|---|
| `src/preview/cls_direct_v3_preview_state.rs` | 58, 189 (initial), 205 (sampled) | `None` for initial/reset; computed via `compute_dwell_override_ms` for the live update |

That's literally one production caller — `DirectV3PreviewState`. All other callers in `cls_v3_playback_timing.rs` are inside the file's own test module (lines 195-265). Tests get `None` passed in.

### 4.5 Validator rule

**File:** new `src/v3/validate/col_validate_event_dwell.rs` (or extend an existing validator)

Two rules:

1. **Loop+binding rejection.** If `compiled.envelope.lifecycle.loop == true` AND `pipeline.timing.dwell_until_binding.is_some()`, return `ValidateError::EventDwellInLoopRecipe`. Latch semantics in a looping recipe are undefined.
2. **Undeclared binding warning** (consistent with existing contract validation per Intention 37). If `dwell_until_binding = "X"` but `requires_bindings` doesn't declare X, warn (or error in strict-contracts mode). The existing `requires_bindings` validator at `col_validate_contracts.rs` is the precedent — extend it to also scan `pipeline.timing.dwell_until_binding`.

### 4.6 Tests

In `cls_direct_v3_preview_state.rs`:

```rust
#[test]
fn dwell_until_binding_advances_phase_when_binding_fires() {
    let compiled = load_v3_compiled("recipes/debug_recipes/event_driven_dwell_demo.json");
    let mut state = DirectV3PreviewState::from_compiled(compiled).unwrap();
    // Advance into dwell phase
    state.update_from_elapsed(Duration::from_millis(2000)).unwrap();
    assert_eq!(state.phase, AnimationPhase::Dwelling);
    // Set binding truthy
    let mut params = state.runtime_overrides.runtime_params.clone();
    params.insert("user_dismissed", true);
    state.set_runtime_params(params).unwrap();
    // Next frame: phase should advance
    state.update_from_elapsed(Duration::from_millis(2050)).unwrap();
    assert_eq!(state.phase, AnimationPhase::Exiting);
}

#[test]
fn dwell_until_binding_falls_back_to_fallback_ms_when_binding_never_fires() {
    let compiled = load_v3_compiled("recipes/debug_recipes/event_driven_dwell_demo.json");
    let mut state = DirectV3PreviewState::from_compiled(compiled).unwrap();
    // Advance past enter (1500ms) + dwell_fallback_ms (declared 5000)
    state.update_from_elapsed(Duration::from_millis(7000)).unwrap();
    assert_eq!(state.phase, AnimationPhase::Exiting);
}

#[test]
fn dwell_latch_resets_after_phase_transition() {
    // Fire binding, observe phase advance, reset state, verify next dwell
    // doesn't auto-fire.
}

#[test]
fn dwell_latch_truthiness_accepts_boolean_integer_text_float_rgb() {
    // Five sub-tests: each truthy value type fires the latch.
}
```

In a new validator test file:

```rust
#[test]
fn loop_recipe_with_dwell_until_binding_is_rejected() {
    // Recipe with lifecycle.loop=true AND dwell_until_binding fails validate.
}
```

### 4.7 Demo recipe

**File:** new `recipes/debug_recipes/event_driven_dwell_demo.json`

```json
{
  "schema_version": 3,
  "id": "debug.event_driven_dwell_demo",
  "title": "Event-driven dwell demo",
  "description": "Dwell phase advances when host sets user_dismissed=true.",
  "version": "0.1.0",
  "last_updated": "2026-04-28",
  "requires_bindings": {
    "user_dismissed": {
      "type": "boolean",
      "description": "Set true to advance from dwell to exit.",
      "loopback": false
    }
  },
  "config": {
    "message": "Press a key to dismiss.",
    "pipeline": {
      "timing": {
        "enter_ms": 1500,
        "dwell_until_binding": "user_dismissed",
        "dwell_fallback_ms": 5000,
        "exit_ms": 800
      }
    }
  }
}
```

---

## 5. Verified blast radius

### 5.1 Files changed

| File | Current version | Surgery |
|---|---|---|
| `src/v3/authoring/cls_v3_recipe_document.rs` | 0.2.0 | 2 new fields on `V3PipelineTiming` |
| `src/v3/compile/cls_v3_playback_timing.rs` | 0.1.1 | 1 new optional parameter on 2 functions; dwell_ms branch logic |
| `src/preview/cls_direct_v3_preview_state.rs` | 0.6.0 | 3 new fields; helper function; update_from_elapsed + reset rewrites |
| `src/v3/validate/col_validate_event_dwell.rs` | NEW | Loop+binding rejection + undeclared binding warning |
| `src/v3/validate/mod.rs` | (verify) | Wire in the new validator |
| `src/v3/validate/enum_validate_error.rs` | (verify) | Add `EventDwellInLoopRecipe` variant |
| `recipes/debug_recipes/event_driven_dwell_demo.json` | NEW | Demo recipe |

**Total: 6 production files (1 new) + 1 demo recipe.** Sibling repo: zero changes. tui-vfx-geometry: zero changes. mixed-signals: zero changes.

### 5.2 What does NOT change

- Recipe JSON wire format for any existing recipe. New fields are optional; absence preserves byte-identical behavior.
- Sampler purity. The sampler still takes `(compiled, elapsed, ...)` and returns `V3PlaybackTiming` deterministically. Only behavior change: when `dwell_override_ms` is `Some`, dwell duration uses that instead of `auto_dismiss_ms`.
- V2 path — `lifecycle.rs::tick`, `AnimationManager`, `AnimationProfile`, `TransitionSpec`. All untouched.
- The shader/filter/mask runtime-binding plumbing. This packet uses the existing `ShaderRuntimeParams` channel; no new IPC surface for the host.
- `tui-vfx-content` — zero changes (this is purely a recipe-side schema + scheduler change).

### 5.3 Wire-shape forward-compatibility

When the full `PhaseTerminator` enum lands (packet 69 Phase C):
- `dwell_until_binding: Option<String>` becomes a `#[serde(alias)]` on a synthesized `dwell: Option<PhaseTerminator>` field whose `Binding` arm carries the string.
- `dwell_fallback_ms: Option<u64>` becomes the `fallback_ms` field on the `Binding` variant.
- Existing recipes parse byte-identically through the alias; new recipes use the richer enum form.

This is a one-direction migration. The hack does not paint us into a corner.

---

## 6. What this packet does NOT do

- Does NOT touch enter or exit phase termination. Both stay strictly time-driven.
- Does NOT add `EffectComplete`, `AnyOf`, or `AllOf` terminators. Those need packet 69 §11 D1 resolved.
- Does NOT touch V2. V2 keeps `AnimationManager.dismiss(id, now)` as its escape hatch.
- Does NOT add a typed `PhaseTerminator` enum. New fields are flat scalars.
- Does NOT change the V3 sampler's purity. Sampler gains one optional parameter; latch state lives elsewhere.
- Does NOT depend on packet 69-A. The two packets are structurally independent and can land in either order or simultaneously.
- Does NOT add a `boolean` accessor to `ShaderRuntimeParams`. Truthiness check uses `params.get(name)` and pattern-matches on the `ShaderRuntimeParamValue` enum.

---

## 7. Verification checklist before merge

- [ ] `cargo build -p tui-vfx-recipes` compiles after schema and sampler changes.
- [ ] `cargo test -p tui-vfx-recipes` passes.
- [ ] Round-trip serialization tests prove existing `V3PipelineTiming` recipes parse and serialize identically (no field reordering, no behavior delta when new fields absent).
- [ ] Demo recipe `event_driven_dwell_demo.json` plays in `tui-vfx-trace` / `tui-vfx-horseman` and visibly advances when the binding is set.
- [ ] Validator rejects loop+binding recipe at recipe-load time with a clear error message.
- [ ] Latch correctness test: binding fires once, phase advances, state.reset() called, next dwell does NOT auto-fire.
- [ ] Truthiness test: each of `Boolean(true)`, `Integer(1)`, `Float(0.5)`, `Text("x")`, `Rgb{...}` fires the latch; `Boolean(false)`, `Integer(0)`, `Float(0.0)`, `Text("")`, missing key do NOT.
- [ ] Documentation: the new fields' docstrings link to packet 69-E and packet 69 (the parent design) for context on the broader vocabulary.

---

## 8. Migration path when the full design lands

When packet 69 Phase C ships the full `PhaseTerminator` enum:

1. New `dwell: Option<PhaseTerminator>` field added to `V3PipelineTiming`.
2. `dwell_until_binding` and `dwell_fallback_ms` get `#[serde(alias)]` attributes pointing to the new shape, with a custom deserializer that constructs `PhaseTerminator::Binding { name, fallback_ms }` from the legacy pair.
3. Validator deprecation warning for the legacy fields (suggest migrating to the tagged form).
4. After a release cycle, the legacy fields move behind a feature flag.
5. Recipes using the legacy fields continue to parse byte-identically throughout.

No breaking change. The hack ages out gracefully.

---

<!-- <FILE>steering/work-packets/69-E-event-driven-dwell.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
