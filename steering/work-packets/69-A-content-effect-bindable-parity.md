<!-- <FILE>steering/work-packets/69-A-content-effect-bindable-parity.md</FILE> - <DESC>Carve-out of Packet 69 Phase A: promote rate-bearing content-effect fields from SignalOrFloat to VfxBindableValue and close the recipes-side ShaderRuntimeParams threading TODO so host applications can supply runtime values for content-effect parameters.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>2026-04-28 carve-out from packet 69 v0.2.0. Phase A is structurally independent of the PhaseTerminator design (no scheduler changes, no V3 sampler signature change) so it ships as its own packet to deliver host-driven content-effect rate control without waiting on §11 design decisions.</WCTX> -->
<!-- <CLOG>0.1.0: initial draft. Scope and file inventory verified by full reads of every cited file in tui-vfx-content, tui-vfx-core, tui-vfx-style, and tui-vfx-recipes.</CLOG> -->

# Packet 69-A — Bindable parity for content effects

**Status:** ready to implement.
**Parent:** packet 69 v0.2.0 (Phase A carve-out).
**Scope:** cross-repo. Sibling: `tui-vfx`. Recipe-side: `tui-vfx-recipes`. No `tui-vfx-geometry` involvement.
**Independent of:** packet 69-E and the full PhaseTerminator design. This packet ships standalone.
**Author orientation:** Every load-bearing claim is verified by reading the cited file end-to-end.

---

## 1. What this packet enables

After this packet ships, an application embedding a recipe can set runtime values for any rate-bearing content-effect parameter **per frame**, with no recipe re-authoring. A recipe can be authored once and driven differently by different hosts:

```json
"content": {
  "effect": {
    "type": "typewriter",
    "speed_variance": { "binding": "typing_jitter" }
  }
}
```

The host calls `state.set_runtime_params(params)` with `typing_jitter = 0.2` for slow expository typing or `typing_jitter = 0.0` for steady terminal output, using the same recipe.

This does NOT change phase-advancement behavior. Phases remain time-driven exactly as today. (The event-driven phase advancement lives in packet 69-E, which is independent of this packet.)

---

## 2. What is already in place (verified)

The plumbing is mostly built. This packet is the last mile.

- **The bindable type exists.** `VfxBindableValue = VfxBindable<f32, SignalOrFloat>` at `crates/tui-vfx-core/src/bindable/cls_bindable.rs:381`. Three-arm enum: `Literal | Binding | Signal`.
- **Backwards-compatible upgrade exists.** `From<SignalOrFloat>` for `VfxBindableValue` at `cls_bindable.rs:482-489` collapses `Static(v) → Literal(v)`; signals stay in `Signal`. So recipes that use `SignalOrFloat::Static(0.0)` migrate to `VfxBindable::Literal(0.0)` byte-identically.
- **Lenient deserializer exists.** `VfxBindableRepr` at `cls_bindable.rs:191-216` accepts: bare `T` (number) → `Literal`; `{"literal": T}` → `Literal`; `{"binding": "name"}` → `Binding`; `{"signal": S}` → `Signal`; bare object matching signal shape (e.g. `{"sine": ...}`) → `Signal`. So existing JSON parses unchanged.
- **The trait already takes `runtime_params`.** `TextTransformer::transform(&self, target, progress, ctx: &TransformContext<'_>)` at `crates/tui-vfx-content/src/traits/text_transformer.rs:13-26` (v3.0.0). All 15 transformers already accept this signature.
- **`TransformContext` carries `runtime_params`.** `crates/tui-vfx-content/src/traits/cls_transform_context.rs:18-30` defines `pub runtime_params: &'a ShaderRuntimeParams`.
- **The bindable's `evaluate` method already takes runtime_params.** `cls_bindable.rs:451-467` defines `VfxBindable<f32, SignalOrFloat>::evaluate(&self, loop_t, signal_ctx, runtime_params) -> Option<f32>`. The Binding arm uses `runtime_params.get_f32(key)`.
- **`ShaderRuntimeParams` implements `RuntimeParamsRead`.** `crates/tui-vfx-style/src/traits/cls_shader_context.rs:260-270` provides `get_u16/get_text/get_f32`.
- **One transformer already reads `ctx.runtime_params` end-to-end.** `cls_odometer.rs:88` forwards it to `resolve_mechanical_cycle_with_context` so binding-form font references resolve at runtime. Production proof the wiring works.
- **V2 carries runtime_params on the playback item.** `PreviewItem.runtime_params: ShaderRuntimeParams` at `cls_preview_item.rs:88` (`PlaybackPlan` is a re-export alias at `preview/mod.rs:58`). The `Animated::runtime_params()` method at `cls_preview_item.rs:534-536` returns it.
- **V3 carries runtime_params on the runtime-overrides struct.** `DirectV3PreviewState.runtime_overrides.runtime_params` reachable via `cls_direct_v3_preview_state.rs:42`.

---

## 3. What is missing (the gap this packet closes)

Three small gaps:

1. **The rate-bearing content-effect fields are typed `SignalOrFloat`, not `VfxBindableValue`.** Host bindings cannot be authored. Affected variants in `cls_content_effect.rs`:
   - `Typewriter.speed_variance` (line 181)
   - `Scramble.resolve_pace` (line 198)
   - `GlitchShift.glitch_start, glitch_end` (lines 214, 217)
   - `ScrambleGlitchShift.resolve_pace, glitch_start, glitch_end` (lines 229, 238, 241)
   - `SplitFlap.speed, cascade, cycles` (lines 277, 282, 287)
   - `Marquee.speed` (line 434)

2. **The recipes-side caller passes empty `ShaderRuntimeParams::new()`.** `fnc_resolve_content_text.rs:42-45` literal TODO tagged `slice-6.6-followup`. Even if the field types are upgraded, no host value reaches the transformer until this is closed.

3. **Transformer bodies call `evaluate(progress, ctx.signal_ctx)` (two-arg).** `VfxBindableValue::evaluate` is three-arg `(loop_t, signal_ctx, runtime_params)`. Each call site needs `, ctx.runtime_params` appended.

---

## 4. Implementation plan

### 4.1 sibling repo (`/usr/projects/tui-vfx`) — schema field swap

**File:** `crates/tui-vfx-content/src/types/cls_content_effect.rs` (current v2.15.0)

Change 11 field types (counting tuple-listed fields individually) from `SignalOrFloat` to `VfxBindableValue`. The fields are documented above in §3.1.

The `From<SignalOrFloat>` impl at `cls_bindable.rs:482-489` ensures every existing JSON value parses into `VfxBindableValue` byte-identically. No recipe migration required.

Add `tui_vfx_core::bindable::VfxBindableValue` to the imports.

### 4.2 sibling repo — transformer body updates

Verified via `grep -n "evaluate(progress, ctx.signal_ctx)"` across `crates/tui-vfx-content/src/transformers/*.rs`: **11 evaluate-call sites** in 6 files. Each is a one-line edit appending `, ctx.runtime_params`:

| File | Line | Surgery |
|---|---|---|
| `cls_typewriter.rs` | 58 | `self.speed_variance.evaluate(progress, ctx.signal_ctx, ctx.runtime_params)` |
| `cls_scramble.rs` | 61 | same shape, on `self.resolve_pace` |
| `cls_glitch_shift.rs` | 62 | on `self.glitch_start` |
| `cls_glitch_shift.rs` | 67 | on `self.glitch_end` |
| `cls_scramble_glitch_shift.rs` | 81 | on `self.resolve_pace` |
| `cls_scramble_glitch_shift.rs` | 112 | on `self.glitch_start` |
| `cls_scramble_glitch_shift.rs` | 117 | on `self.glitch_end` |
| `cls_marquee.rs` | 48 | on `self.speed` |
| `cls_split_flap.rs` | 428 | on `self.speed` |
| `cls_split_flap.rs` | 433 | on `self.cascade` |
| `cls_split_flap.rs` | 438 | on `self.cycles` |

The `evaluate` method signature on `VfxBindableValue` (`cls_bindable.rs:456-467`) already takes `(loop_t, signal_ctx, runtime_params)` — three args. Today's transformers call the two-arg `SignalOrFloat::evaluate`. After this packet, they call the three-arg version on the new type. The `runtime_params: &R` is generic over `R: RuntimeParamsRead + ?Sized`, so passing `ctx.runtime_params` (which is `&ShaderRuntimeParams`) compiles directly.

### 4.3 sibling repo — dispatcher

**File:** `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs` (current v3.11.0)

The dispatch arms today clone the field value: `speed_variance.clone()`, `resolve_pace.clone()`, etc. (lines 14-170). Field type changes from `SignalOrFloat` to `VfxBindableValue`, but the constructor call shape stays the same. The transformer struct fields' types must also change to `VfxBindableValue`:

| Transformer struct | Field | File |
|---|---|---|
| `Typewriter` | `speed_variance: VfxBindableValue` | `cls_typewriter.rs:19` |
| `Scramble` | `resolve_pace: VfxBindableValue` | `cls_scramble.rs:22` |
| `GlitchShift` | `glitch_start, glitch_end: VfxBindableValue` | `cls_glitch_shift.rs:18-19` |
| `ScrambleGlitchShift` | `resolve_pace, glitch_start, glitch_end: VfxBindableValue` | `cls_scramble_glitch_shift.rs:24-27` |
| `Marquee` | `speed: VfxBindableValue` | `cls_marquee.rs:16` |
| `SplitFlap` | `speed, cascade, cycles: VfxBindableValue` | `cls_split_flap.rs:120-122` |

Constructors and `Default` impls update accordingly. `SplitFlap::new(speed: VfxBindableValue, cascade: VfxBindableValue)` etc. — callers that pass `SignalOrFloat::Static(...)` get auto-conversion via `From<SignalOrFloat>` so call sites compile unchanged after `.into()` is added at constructor boundaries.

### 4.4 sibling repo — `apply` family doc cleanup

**File:** `crates/tui-vfx-content/src/types/cls_content_effect.rs:131-140`

The doc comment on `ContentEffect` mentions `apply_with_context`. That method was removed in `fnc_apply_content_effect.rs` v2.0.0; the current name is `apply_with_runtime`. Update the doc to reference `apply_with_runtime`. No code change.

### 4.5 recipes-side (`/usr/projects/tui-vfx-recipes`) — close the TODO

**File:** `src/preview/fnc_resolve_content_text.rs` (current v0.2.0)

Replace lines 42-45:
```rust
// TODO(slice-6.6-followup): thread a real &ShaderRuntimeParams through
// resolve_content_text callers so host-supplied bindings reach transformers.
// For now, empty params preserve existing behavior (no host bindings supplied).
let runtime_params = ShaderRuntimeParams::new();
```

with a new `runtime_params: &ShaderRuntimeParams` parameter on the function signature. Function signature becomes:

```rust
pub fn resolve_content_text(
    base_text: &str,
    content_mode: RaContentMode,
    effect: Option<&ContentEffect>,
    phase: AnimationPhase,
    phase_t: f64,
    loop_t: Option<f64>,
    signal_ctx: &SignalContext,
    runtime_params: &ShaderRuntimeParams,  // NEW
) -> String { ... }
```

### 4.6 recipes-side — V2 caller threading

**File:** `src/preview/fnc_render_preview_item.rs` (current v2.1.0)

Two `resolve_content_text(...)` calls via `resolve_message` (line 45-53 and used in both `render_preview_item` line 169-258 and `render_preview_item_inspected` line 286-388).

Add `&item.runtime_params` to the `resolve_content_text` call. `item: &PlaybackPlan` is `&PreviewItem`; `PreviewItem.runtime_params` exists at `cls_preview_item.rs:88`.

`resolve_message` itself (line 37-68) needs a new `runtime_params: &ShaderRuntimeParams` parameter forwarded from its callers. Change is purely additive.

### 4.7 recipes-side — V3 caller threading

**File:** `src/v3/compile/cls_v3_source_surface.rs` (current v0.7.0)

Function `resolve_source_text` at line 688-712 calls `resolve_content_text(...)` at line 703-711. Needs a new `runtime_params: &ShaderRuntimeParams` parameter, forwarded from `build_v3_source_surface` (line 31-179, the public entry point at line 31).

Verified callers of `build_v3_source_surface`: 4 production sites:
- `src/v3/compile/fnc_render_compiled_plan_deterministically.rs:388`
- `src/v3/compile/fnc_render_compiled_plan_deterministically.rs:584`
- `src/v3/compile/fnc_render_compiled_plan_deterministically.rs:659`
- `src/v3/compile/fnc_build_probe_scene_spec_from_compiled_plan.rs:188`

Each already has `overrides: &CompiledV3RuntimeOverrides` in scope; pass `&overrides.runtime_params`. Plus 4 test-site callers in `cls_v3_source_surface.rs` itself (lines 726, 747, 833, 848) — pass `&ShaderRuntimeParams::new()` for tests that don't need bindings.

### 4.8 Tests

For each rate-bearing field, one test asserting that a recipe with `{"binding": "k"}` evaluates the host-supplied value:

```rust
#[test]
fn typewriter_speed_variance_resolves_from_runtime_binding() {
    let mut params = ShaderRuntimeParams::new();
    params.insert("typing_jitter", 0.5_f32);
    let effect = ContentEffect::Typewriter {
        speed_variance: VfxBindableValue::Binding("typing_jitter".to_string()),
        cursor: None,
    };
    // ... render through resolve_content_text and assert variance applied
}
```

A regression suite asserting bare-number and `{"signal": ...}` shapes parse identically to today (round-trip).

---

## 5. Verified blast radius

### 5.1 Files changed

| Repo | File | Current version | Surgery |
|---|---|---|---|
| tui-vfx | `crates/tui-vfx-content/src/types/cls_content_effect.rs` | 2.15.0 | 11 field-type changes (6 enum variants); doc cleanup §4.4 |
| tui-vfx | `crates/tui-vfx-content/src/transformers/cls_typewriter.rs` | 4.0.0 | struct field type change + 1 `evaluate` call |
| tui-vfx | `crates/tui-vfx-content/src/transformers/cls_scramble.rs` | 3.1.0 | same shape |
| tui-vfx | `crates/tui-vfx-content/src/transformers/cls_glitch_shift.rs` | 2.2.0 | 2 field type changes + 2 `evaluate` calls + test imports |
| tui-vfx | `crates/tui-vfx-content/src/transformers/cls_scramble_glitch_shift.rs` | 3.2.0 | 3 field type changes + 3 `evaluate` calls + test imports |
| tui-vfx | `crates/tui-vfx-content/src/transformers/cls_marquee.rs` | 2.1.0 | 1 field + 1 `evaluate` |
| tui-vfx | `crates/tui-vfx-content/src/transformers/cls_split_flap.rs` | 3.5.0 | 3 fields + 3 `evaluate` calls + test imports |
| tui-vfx | `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs` | 3.11.0 | dispatch arm types update (already clone-pass-through) |
| tui-vfx-recipes | `src/preview/fnc_resolve_content_text.rs` | 0.2.0 | new parameter; remove TODO |
| tui-vfx-recipes | `src/preview/fnc_render_preview_item.rs` | 2.1.0 | thread param through 2 resolve_message → resolve_content_text chains |
| tui-vfx-recipes | `src/v3/compile/cls_v3_source_surface.rs` | 0.7.0 | new param on `build_v3_source_surface` and `resolve_source_text` |
| tui-vfx-recipes | `src/v3/compile/fnc_render_compiled_plan_deterministically.rs` | (verify) | 3 `build_v3_source_surface` call sites pass `&overrides.runtime_params` |
| tui-vfx-recipes | `src/v3/compile/fnc_build_probe_scene_spec_from_compiled_plan.rs` | (verify) | 1 `build_v3_source_surface` call site |

**Total: 8 sibling files + 5 recipe files = 13 files.** All changes are additive at the type level (`SignalOrFloat → VfxBindableValue` is a strict superset via `From`) and at the call-site level (one new arg, no removed args).

### 5.2 What does NOT change

- Recipe JSON wire format. Existing recipes parse byte-identically.
- Transformer behavior when the bindable resolves to `Literal` or `Signal`. The new `Binding` arm is the only new behavior path.
- The `apply` family API. `apply` and `apply_to_borrowed` continue to default-construct empty `ShaderRuntimeParams`; `apply_with_runtime` continues to take the host-supplied one.
- V2 phase scheduler, V3 sampler, lifecycle.tick — all untouched.
- `tui-vfx-geometry` — not touched.

### 5.3 Cross-repo sequencing

Sibling repo lands first (schema + transformers + dispatcher + doc cleanup). Recipes-side then lands the caller threading. Recipes-side cannot land before sibling because it depends on the new `VfxBindableValue` field types in `ContentEffect`.

### 5.4 gt-design audit (Intention 41)

Run `ofpf-content "SignalOrFloat" --files-with-matches` against gt-design before sibling lands. Any consumer that reads these fields directly (vs through serde) needs an `.into()` or pattern-match update.

---

## 6. What this packet does NOT do

- Does NOT add `cadence_ms` (a new wire field). That is packet 69 Phase B.
- Does NOT add `is_complete()` to the trait. That is packet 69 Phase C.
- Does NOT add `PhaseTerminator`. That is packets 69-E (minimal Binding-on-dwell) and 69 Phase C (full vocabulary).
- Does NOT change phase-advancement behavior. Phases stay time-driven.
- Does NOT rename `speed_variance` to `cadence_jitter`. Packet 69 Risk #1; deferred.
- Does NOT touch `tui-vfx-geometry::TransitionSpec`. V2 needs no surgery for this packet.
- Does NOT touch the V3 sampler signature (`sampled_v3_playback_timing_from_elapsed`).

---

## 7. Verification checklist before merge

- [ ] `cargo build -p tui-vfx-content` compiles after schema field swaps.
- [ ] `cargo test -p tui-vfx-content` passes — including round-trip serialization tests proving legacy `{"speed_variance": 0.5}` and `{"speed_variance": {"signal": ...}}` shapes parse identically to today.
- [ ] `cargo build -p tui-vfx-recipes` compiles after caller threading.
- [ ] `cargo test -p tui-vfx-recipes` passes — including the new tests asserting `{"binding": "k"}` evaluates the host-supplied value for each affected effect.
- [ ] `ofpf-content "SignalOrFloat" --glob '**/*.rs'` audit on gt-design shows no consumer break.
- [ ] One example recipe in `recipes/debug_recipes/content/` demonstrating `{"binding": "..."}` for at least one rate-bearing field, with a paired unit test that drives the binding.

---

<!-- <FILE>steering/work-packets/69-A-content-effect-bindable-parity.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
