<!-- <FILE>docs/arch/CLOCKS_AND_TIMING.md</FILE> - <DESC>Architecture note for v3.1 clocks, timing, cadence, and sample-time boundaries</DESC> -->
<!-- <VERS>VERSION: 0.4.1</VERS> -->
<!-- <WCTX>v3.1 transition planning: align lifecycle phases, transition intervals, runtime cadence, and absolute sample time.</WCTX> -->
<!-- <CLOG>0.4.1: PATCH — update clock examples for post-rename field names and distinguish completed naming audit from pending workbench commonality review.</CLOG> -->

# Clocks and Timing

## Status

Draft architecture note.

This document captures the timing vocabulary we should preserve while designing tui-vfx-compost, Primitive Workbench, schema-driven primitive contracts, player evidence, and recipe migration validation.

The core rule is: **do not use `fps` as a catch-all timing concept**. Frame presentation, semantic update cadence, lifecycle progression, loop/sample position, and absolute elapsed time are related but distinct contracts.

## Why This Exists

The v3.1 contract already has lifecycle clocks and durations, and the player already has playback FPS controls. Recent Madeira flag recipes show a third important need: procedural sources sometimes need **absolute elapsed sample time** so animation can advance even when normalized lifecycle coordinates are held constant.

If these concepts are collapsed, we risk either:

- making deterministic validation depend on host/display frame rate;
- making source/effect behavior impossible to sample reproducibly;
- forcing primitives to invent private wall-time conventions;
- or turning recipe schema `fps` into an overloaded field that means different things in different crates.

## Current Timing Concepts

### 1. Lifecycle Clock

Current home:

- `tui_vfx_contract::ClockSpec`
- `tui_vfx_contract::ClockMode`
- `schemas/v3.1/contract/clock.schema.json`
- recipe field: `lifecycle.clock`

Purpose:

The lifecycle clock defines the recipe-level time sample space used for enter/dwell/exit evaluation.

Current modes:

- `monotonic` — time advances through a single lifecycle pass.
- `looping` — time wraps by an explicit `period`.

Conceptual use:

```json
{
  "clock": {
    "clockMode": "looping",
    "period": { "kind": "milliseconds", "value": 60000 }
  }
}
```

What it is not:

- not screen refresh rate;
- not player FPS;
- not an effect-local update schedule;
- not a guarantee that any primitive recomputes at a particular cadence.

### 2. Lifecycle Phase Durations

Current home:

- `tui_vfx_contract::LifecycleSpec`
- `tui_vfx_contract::PhaseSpec`
- `tui_vfx_contract::PhaseTiming`
- `tui_vfx_contract::DwellPolicy`
- `tui_vfx_contract::DurationSpec`
- `schemas/v3.1/contract/lifecycle.schema.json`
- `schemas/v3.1/contract/phase.schema.json`
- `schemas/v3.1/contract/duration.schema.json`

Purpose:

Phase timing describes how long enter, dwell, and exit phases last, or what dwell trigger/cap ends dwell.

Conceptual use:

```json
{
  "phase": "dwell",
  "timing": {
    "kind": "dwell",
    "policy": {
      "kind": "fixed",
      "duration": { "kind": "milliseconds", "value": 5000 }
    }
  }
}
```

What it is not:

- not presentation cadence;
- not a fixed-step simulation rate;
- not a primitive-local throttle.

### 3. Transition Timing

Current home:

- `tui_vfx_contract::TransitionSpec`
- `tui_vfx_contract::TransitionTiming`
- `tui_vfx_contract::TransitionTrack`
- `schemas/v3.1/contract/transition.schema.json`

Purpose:

A transition is a state/surface-change interval. Its `timing` defines the default duration, delay, easing, and stagger inherited by executable tracks unless a track declares its own timing override. Tracks may be visibility, opacity, motion, relation, content, or transient style concerns; persistent generated visuals remain sources or phase-scoped graph nodes.

Relationship to lifecycle:

- recipe lifecycle still owns high-level `enter`, `dwell`, and `exit` progression;
- a transition can declare `activePhases` to say which lifecycle phases it participates in;
- `enter` and `exit` commonly use transitions to introduce or remove subjects;
- `dwell` can still contain transitions for state changes, loops, or externally triggered swaps, but dwell is not replaced by transition vocabulary; continuous visuals such as matrix rain or a waving procedural flag remain dwell graph/source behavior;
- triggers advance lifecycle phases; transition tracks express visual behavior inside the selected interval.

What it is not:

- not presentation cadence;
- not a replacement for lifecycle clock or phase timing;
- not an adapter from canonical v3.1 to a legacy compositor DTO.

Dwell-effect timing pressure:

Persistent effects such as scanners, progress indicators, pulse waves, vignettes,
and coordinate samplers should use graph/effect nodes plus value sources such as
`SignalExpressionSpec`, `ValueSource::PhaseProgress`, or `ValueSource::Clock`.
They should not be reclassified as transitions merely because they are animated.

### 4. Transition Interruption, Reduced Motion, and Variants

Current home:

- `TransitionSpec.interruption`
- `TransitionSpec.reducedMotion`
- `TransitionSpec.variants`

Purpose:

Interruption and reduced-motion policy are part of the transition contract from the start. `interruption` declares what should happen when a transition is superseded before completion. `reducedMotion` declares the transition's accessibility fallback posture. Substitution policies must name a replacement transition, and replacement chains must terminate in a non-substitution policy to avoid recursive fallback loops. `variants` provide a generic engine-level conditional replacement mechanism for reduced-motion requests, capability fallback, or host-selected substitutions without adding app/design-system semantics to tui-vfx.

What it is not:

- not a `gt-design` semantic layer;
- not a Material component policy;
- not runtime fallback to legacy inputs.

### 5. Element-Local Graph Timing

Current home:

- `tui_vfx_contract::RecipeElementGraphTiming`
- recipe scene element graph timing fields such as `enter_ms`, `exit_ms`, `enter_offset_ms`, and `exit_offset_ms`.

Purpose:

Element-local graph timing lets scene elements derive local enter/exit progress from the parent recipe timeline.

Conceptual use:

- stagger an element's enter animation;
- let an element's local `phaseT` start later than the recipe phase;
- preserve source-scene timing offsets during migration.

What it is not:

- not a new clock policy;
- not display FPS;
- not semantic update cadence.

### 6. Player Sample Time

Current home:

- `tui_vfx_player::PlayerSampleRequest`
- fields: `phase_t`, `loop_t`, `absolute_t_ms`
- render evidence/report surfaces that carry sample timing metadata.

Purpose:

A player sample request describes one deterministic sample of a recipe.

Current fields:

- `phase_t` — normalized progress in the requested lifecycle phase.
- `loop_t` — optional normalized loop-local progress.
- `absolute_t_ms` — optional monotonic elapsed sample time in milliseconds.

Conceptual use:

```rust
PlayerSampleRequest {
    phase_t: 0.0,
    loop_t: Some(0.0),
    absolute_t_ms: Some(4_000.0),
    ..PlayerSampleRequest::default()
}
```

What it is not:

- not recipe schema authoring data;
- not player FPS;
- not necessarily wall-clock real time during tests. In deterministic validation, it is an explicit sampled timestamp.

### 7. Runtime Presentation FPS

Current home:

- `tui-vfx-player-cli play-backend --fps`
- `CliOptions.fps`, currently defaulting to `12`.

Purpose:

Playback FPS controls how many frames a player/backend command emits over a duration and how quickly it sleeps between visible frames.

Conceptual use:

```text
tui-vfx-player-cli play-backend --backend compositor --format ansi --fps 24 --duration-ms 4000 --recipe path/to/recipe.json
```

What it is not:

- not canonical recipe semantics;
- not primitive update cadence;
- not schema-owned visual behavior.

### 8. Primitive Frequency / Speed Inputs

Current home:

- primitive descriptor inputs such as `frequency`, `hz`, `glyphChangeHz`, `temporalDitherHz`, `speed`, and related fields.

Purpose:

These fields control a primitive's visual math: wave frequency, shimmer speed, glyph-change rate, temporal dither rate, pulse frequency, and similar behavior.

What they are not:

- not a general source/effect update schedule;
- not screen refresh rate;
- not a replacement for explicit sample time.

## Madeira Flag Timing Lesson

The v3.1 Madeira flag recipes are the concrete reference case for why absolute sample time must remain separate from normalized lifecycle progress.

Relevant recipes:

- `../tui-vfx-recipes/recipes/v3.1/debug_recipes/scene/scene_madeira_flag_runtime_wave.json`
- `../tui-vfx-recipes/recipes/v3.1/debug_recipes/scene/scene_madeira_flag_full_scene.json`
- `../tui-vfx-recipes/recipes/v3.1/debug_recipes/scene/scene_madeira_flag_lambert.json`
- `../tui-vfx-recipes/recipes/v3.1/debug_recipes/scene/scene_madeira_flag_raytrace.json`

Relevant player code:

- `crates/tui-vfx-player/src/fnc_render_procedural_source.rs`
- `crates/tui-vfx-player/src/fnc_apply_preview_loopbacks.rs`

Important behavior:

- `source.procedural` generator `braille_flag_field` uses elapsed seconds for wave motion.
- `source.procedural` generator `ballistic_fireworks` uses elapsed milliseconds for burst cycles.
- Authored preview loopback ramps use elapsed seconds to honor ramp duration.
- The regression `madeira_flag_full_scene_uses_absolute_elapsed_time_for_wave_motion` proves output changes when `absolute_t_ms` changes while `phase_t` and `loop_t` stay fixed.

Conclusion:

Absolute elapsed sample time is not just an implementation detail. It is part of the runtime sample context that timing-sensitive procedural sources and preview/demo signals need.

## Proposed Timing Collection

### A. Lifecycle Clock

Status:

- already exists;
- schema-owned;
- keep as recipe-level lifecycle contract.

Used by:

- recipe lifecycle evaluation;
- phase progression;
- loop period interpretation;
- deterministic player sampling.

Do not use for:

- screen refresh;
- primitive update throttling;
- host playback speed.

### B. Phase Timing

Status:

- already exists;
- schema-owned;
- keep as lifecycle phase contract.

Used by:

- enter/dwell/exit durations;
- dwell trigger policy and maximum dwell duration;
- deriving normalized phase progress.

Do not use for:

- source/effect frame cadence;
- display FPS.

### C. Element-Local Timing Envelope

Status:

- partially exists as `RecipeElementGraphTiming`;
- should remain a local timeline envelope, not a clock.

Used by:

- scene-layer enter/exit offsets;
- staggered local effects;
- deriving element-local `phaseT` from parent sample time.

Potential update:

- document inheritance and fallback rules more clearly as tui-vfx-compost starts consuming element-local timing.

### D. Runtime Sample Time

Status:

- already exists in player request/report data;
- should be treated as part of the explicit runtime sample context at crate boundaries.

Fields:

- `phaseT`
- `loopT`
- `absoluteTimeMs` / `absolute_t_ms`

Used by:

- deterministic player reports;
- procedural source sampling;
- preview loopback signal evaluation;
- migration validation at named sample points;
- tui-vfx-compost requests that need reproducible time input.

Policy:

- Tests and validation should pass this explicitly.
- Continuous procedural animation should prefer `absolute_t_ms` when it needs elapsed time.
- `phaseT` and `loopT` remain normalized coordinates, not substitutes for elapsed time.

### E. Presentation Cadence

Status:

- currently runtime/player-owned via CLI `--fps`;
- proposed to remain runtime-owned by default.

Possible future schema surface:

- optional metadata/profile hint only, if needed.
- preferred name: `targetFrameRateHz` or `presentationRateHz`, not plain `fps`.

Used by:

- live player loops;
- terminal/backend pacing;
- UI display frame budget;
- deciding how many samples to emit over a preview duration.

Do not use for:

- recipe visual semantics;
- primitive math;
- deterministic parity assumptions.

### F. Semantic Update Cadence

Status:

- does not exist as a general schema concept today;
- proposed as an audit candidate, not an automatic schema addition.

Possible future names:

- `updateClock`
- `updateRateHz`
- `updatePeriod`
- `fixedStep`

Used by:

- sources/effects that intentionally recompute at a lower rate than display presentation;
- deterministic stepped procedural behavior;
- palette cycling or noise/glyph updates that should hold between ticks;
- avoiding accidental dependence on player presentation FPS.

Example concept:

```json
{
  "updateClock": {
    "updateClockMode": "fixedStep",
    "updateRateHz": 12
  }
}
```

Open design questions:

1. Which levels may declare it: recipe, scene, element, source, effect, or descriptor default?
2. How does inheritance work?
3. When fixed-step update cadence is lower than presentation cadence, does the renderer hold previous state, interpolate, or resample from quantized time?
4. Is the cadence semantically visible enough to belong in canonical recipe JSON, or should it stay in runtime/studio profile metadata for now?

### G. Primitive Motion Parameters

Status:

- already exists through descriptor-local fields such as `speed`, `frequency`, `hz`, and related names.

Used by:

- visual equations inside a primitive;
- wave cycles;
- pulse rate;
- flicker or dither math;
- glyph-change frequency.

Policy:

- Keep these distinct from update cadence.
- A primitive can have both a semantic frequency and, in the future, an update cadence. For example, a source could animate a wave at 2.4 cycles/sec but update only at 12Hz.

## Recommended Boundary Model

```text
┌────────────────────────────────────────────────────────────┐
│ Recipe Schema                                               │
│ - lifecycle clock                                           │
│ - phase durations / dwell policy                            │
│ - transition timing / interruption / variants               │
│ - element-local timing envelope                             │
│ - primitive/source motion parameters                        │
│ - possible future semantic update cadence                   │
└──────────────────────────────┬─────────────────────────────┘
                               │ canonical recipe + descriptors
                               ▼
┌────────────────────────────────────────────────────────────┐
│ Player / Runtime Sample Context                             │
│ - phaseT                                                     │
│ - loopT                                                      │
│ - absoluteTimeMs                                             │
│ - host signals / runtime overrides                           │
└──────────────────────────────┬─────────────────────────────┘
                               │ deterministic sample request
                               ▼
┌────────────────────────────────────────────────────────────┐
│ Primitive / Source / Compositor Evaluation                   │
│ - evaluate visual math from schema fields                    │
│ - use absolute elapsed time when behavior is continuous       │
│ - use quantized update time only when update cadence says so  │
└──────────────────────────────┬─────────────────────────────┘
                               │ sampled frame/composition
                               ▼
┌────────────────────────────────────────────────────────────┐
│ Presentation / Playback                                     │
│ - target frame rate / CLI fps / UI pacing                    │
│ - terminal draw loop                                         │
│ - not canonical recipe semantics                             │
└────────────────────────────────────────────────────────────┘
```

## Naming Guidance

Avoid:

- `fps` in canonical recipe schema, unless the field is explicitly presentation-only.
- `wallTime` for deterministic recipe sampling; it sounds host-dependent.
- reusing `clock` without a qualifier.

Prefer:

- `lifecycle.clock` for recipe lifecycle sample space.
- `absoluteTimeMs` for explicit elapsed sample timestamp on wire/report surfaces.
- `presentationRateHz` or `targetFrameRateHz` for display/playback intent.
- `updateClock`, `updateRateHz`, or `updatePeriod` for semantic recomputation cadence.
- `phaseT` and `loopT` for normalized sample coordinates.

## Schema-Change Guidance

Do not add a schema field just because the player has an FPS flag.

Before adding timing schema, run a descriptor/workbench commonality review and classify the need. This is distinct from the completed ambiguous field-name audit:

| Need | Preferred home |
| --- | --- |
| Live preview draw rate | Player/runtime option |
| Human-authored target display budget | Optional metadata/profile hint, if accepted |
| Lifecycle duration or loop period | Existing lifecycle clock/phase schema |
| Transition interval, easing, or stagger | `TransitionSpec.timing` or per-track timing override |
| Element enter/exit staggering | Existing element-local timing envelope |
| Continuous procedural animation | Runtime sample context with `absoluteTimeMs` |
| Fixed-step source/effect recomputation | Candidate `updateClock` / `updateRateHz` schema concept |
| Wave/pulse/noise visual frequency | Primitive descriptor input |

## Immediate Plan Impact

1. Add timing/cadence to the descriptor/workbench commonality review only when a concrete primitive/source needs it.
2. Preserve Madeira flag recipes as regression fixtures for absolute elapsed time.
3. Keep player/backend FPS runtime-owned unless we identify a recipe-authored presentation hint use case.
4. If a primitive needs lower update frequency, design `updateClock` vertically with that primitive instead of adding a global schema field speculatively.
5. Primitive Workbench should generate explicit timing/sample-context plumbing instead of letting each primitive invent its own clock assumptions.

<!-- <FILE>docs/arch/CLOCKS_AND_TIMING.md</FILE> - <DESC>Architecture note for v3.1 clocks, timing, cadence, and sample-time boundaries</DESC> -->
<!-- <VERS>END OF VERSION: 0.4.1</VERS> -->
