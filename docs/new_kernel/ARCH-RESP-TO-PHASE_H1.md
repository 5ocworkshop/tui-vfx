<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_H1.md</FILE> - <DESC>Architect response to Phase H1 and Phase I0 assignment</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase I0: preserve architect guidance for time lifecycle and trigger contracts.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture I0 assignment and lifecycle/time/trigger guidance.</CLOG> -->

# Architect Response to Phase H1 — Canonical Recipe Document Schema

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Reviewed phase: **H1 — Canonical Recipe Document Schema**
Next recommended phase: **I0 — Time / Lifecycle / Trigger Contract**

## Executive summary

Phase H1 is accepted as the canonical recipe package lock point.

The contract crate can now package the major v3.1 building blocks into a strict canonical recipe document: metadata, assets, source descriptors, source instances, graph, and scenes. `docs/VOCABULARY.md` is also the right standing artifact and should become mandatory maintenance for every future public contract phase.

The next missing contract layer is **time, lifecycle, and trigger semantics**. We should not begin a serious player, migration path, or recipe lowering pass until this is defined, because existing recipes already rely on lifecycle behavior such as enter/dwell/exit, event-driven dwell, fallback dwell caps, looped clocks, and phase-scoped effects.

Important owner direction: **the current trigger/event-driven dwell implementation is evidence, not a constraint.** The v3.1 contract should use the correct model even if it differs from the existing recipe field names or current preview/compiler implementation.

## H1 assessment

H1 successfully established the canonical package shape:

```text
RecipeDocument
    ├── metadata
    ├── assets
    ├── source descriptors
    ├── source instances
    ├── canonical GraphSpec
    └── RecipeScene values
```

That is the right foundation. It gives source-producing surfaces, graph nodes, graph topology, graph values, and scene composition a single strict document container without copying legacy recipe syntax.

The new `docs/VOCABULARY.md` also changes the review standard: future phases must not merely add DTOs and schemas; they must lock terminology.

## Next assignment

```text
Phase I0 — Time / Lifecycle / Trigger Contract
```

### Phase I0 proof question

```text
Can v3.1 describe recipe-level time, lifecycle phases, dwell policy, and
trigger-driven lifecycle transitions as strict canonical contract vocabulary,
without copying legacy recipe field names or implementing a runtime player?
```

Expected answer at completion:

```text
Yes. tui-vfx-contract owns schema-backed lifecycle/time/trigger DTOs.
RecipeDocument can reference lifecycle semantics. The schema is strict,
rustdoc-backed, checked in, and validated by tests. No runtime trigger engine,
player, store, recipe compiler, or legacy migration behavior is implemented.
```

## Why this phase is next

Current recipe evidence includes event-driven dwell and looped clocks:

```text
pipeline.timing.dwell_until_binding
pipeline.timing.dwell_fallback_ms
clock.loop
clock.period_ms
phase-scoped steps
enter/dwell/exit behavior
```

But those fields are current authoring/preview evidence, not the canonical v3.1 model.

If we skip this phase and go straight to recipe lowering or a player, the implementer will have to invent ad hoc timing fields. That creates backtracking risk. Lifecycle should be locked before migration and player work.

## Required conceptual distinctions

Phase I0 should explicitly distinguish these terms in `docs/VOCABULARY.md`.

| Term                      | Meaning                                                | Example                                                  |
| ------------------------- | ------------------------------------------------------ | -------------------------------------------------------- |
| **Clock**                 | Defines the time sample space.                         | monotonic time, loop period, phase-local time            |
| **Lifecycle**             | High-level recipe/object progression.                  | enter → dwell → exit → finished                          |
| **Phase**                 | A named lifecycle interval with timing semantics.      | enter, dwell, exit                                       |
| **Trigger**               | A condition that causes a lifecycle transition/action. | exit dwell when `userDismissed` becomes true             |
| **Gate**                  | A continuously sampled visibility/execution condition. | show/hide a spinner layer while `showSpinner` is true    |
| **Binding / ValueSource** | A way to supply a value.                               | parameter, signal, literal, graph value                  |
| **Loopback**              | Preview/demo value provider when no host value exists. | ramp `demoProgress` in preview                           |
| **Effect-local schedule** | Per-effect activation timing, not lifecycle.           | glyph timeline wavefront, poisson burst, staggered cells |

The biggest risk is conflating these:

```text
Trigger ≠ Gate
Trigger ≠ Binding
Trigger ≠ Loopback
Lifecycle trigger ≠ GlyphTimelineTriggerSpec
```

## Existing implementation is evidence only

The implementer should read the existing event-driven dwell code and recipes, but must not preserve legacy field names by default.

For example, this old shape:

```json
{
  "dwell_until_binding": "user_dismissed",
  "dwell_fallback_ms": 5000
}
```

should lower into a more explicit canonical model, conceptually like:

```json
{
  "dwell": {
    "policy": {
      "kind": "until",
      "condition": {
        "kind": "value",
        "source": {
          "kind": "signal",
          "id": "userDismissed"
        },
        "predicate": {
          "kind": "isTrue"
        }
      },
      "maxDuration": {
        "kind": "milliseconds",
        "value": 5000
      },
      "latch": {
        "kind": "untilPhaseReset"
      }
    }
  }
}
```

This is illustrative only; the implementer should choose final names consistent with existing v3.1 DTO conventions.

## Core design decisions for Phase I0

### 1. Recipe-level lifecycle first

The first lifecycle model should attach at the canonical recipe level unless implementation evidence strongly argues otherwise.

Recommended initial direction:

```text
RecipeDocument.lifecycle: Option<LifecycleSpec>
```

Graph-local or element-local lifecycle overrides can wait until a concrete need appears.

### 2. Dwell fallback should be modeled as a maximum duration

The existing phrase `dwell_fallback_ms` is ambiguous. The observed behavior is closer to:

```text
Dwell until trigger fires, but no longer than max duration.
```

Canonical vocabulary should prefer:

```text
maxDuration
```

rather than a vague `fallback`.

### 3. Use explicit predicates, not magical truthiness

Existing recipes include bool, integer, and text event-driven dwell demos. The canonical model should avoid an untyped “truthy” rule as the only option.

Preferred direction:

```text
ValuePredicate
    ├── isTrue          # bool
    ├── isFalse         # bool
    ├── nonZero         # integer/number/duration if appropriate
    ├── nonEmpty        # string
    ├── equals(value)
    ├── notEquals(value)
    ├── greaterThan(value)
    └── lessThan(value)
```

A convenience `truthy` predicate is acceptable only if its per-kind behavior is documented and tested. Explicit typed predicates are safer.

### 4. Triggers need latch and reset semantics

Current event-driven dwell says the trigger latches once fired. That must not remain implicit.

Recommended vocabulary:

```text
TriggerLatchPolicy
    ├── none
    ├── untilPhaseReset
    └── untilRecipeReset
```

Recommended initial default for old event-driven dwell lowering:

```text
untilPhaseReset
```

### 5. Trigger sources should use the existing value model

Phase F1/F2 already created typed values, parameters, signals, and value sources. Phase I0 should reuse them.

Recommended rule:

```text
TriggerCondition references ValueSource plus ValuePredicate.
```

But contextual validation should restrict inappropriate sources. For recipe-level lifecycle triggers, `GraphValue` should probably be rejected in I0 because graph values are produced during graph execution, not as recipe-level host/lifecycle inputs.

### 6. Host event-like inputs should probably be signals, not parameters

Do not blindly lower every old `requires_bindings` item to `ParameterSpec`.

A rough distinction:

```text
ParameterSpec:
    user-adjustable or recipe-configurable control value

SignalSpec:
    host-provided runtime/event/state signal
```

Examples:

```text
demo_progress       -> probably parameter or preview/demo parameter
wave_speed          -> probably parameter
show_spinner        -> gate signal or bool parameter depending source
user_dismissed      -> likely signal/event-like lifecycle input
```

Phase I0 does not need to settle every migration case, but it should avoid making “binding” the canonical term.

### 7. Gates are deferred or separately named

Scene layer visibility predicates are gates, not triggers.

Example evidence:

```json
"visibility": {
  "predicate": "show_spinner"
}
```

Phase I0 may define a minimal `GateCondition` only if small, but it should not build scene visibility execution. At minimum, `docs/VOCABULARY.md` must define gate vs trigger.

### 8. Effect-local triggers stay separate

Do not merge these into lifecycle triggers:

```text
GlyphTimelineTriggerSpec
TimelineTrigger
WavefrontTriggerConfig
poisson burst schedule
phase offset per cell
```

They are effect-local schedules. They may later reuse value/predicate vocabulary, but Phase I0 should not collapse them into lifecycle triggers.

## Suggested DTO vocabulary

The implementer may refine names, but the phase should likely add a small set close to this:

```text
ClockSpec
ClockMode
LifecycleSpec
LifecyclePhase
PhaseSpec
PhaseTiming
DwellPolicy
TriggerSpec
TriggerCondition
ValuePredicate
TriggerLatchPolicy
TriggerResetBoundary
TriggerAction
TimeoutPolicy / DurationSpec
```

Possible canonical roots:

```text
schemas/v3.1/contract/clock.schema.json
schemas/v3.1/contract/lifecycle.schema.json
schemas/v3.1/contract/phase.schema.json
schemas/v3.1/contract/trigger.schema.json
```

`recipe.schema.json` should be updated if `RecipeDocument` gains lifecycle.

## Required evidence reads

The implementer and any evidence subagent should read these files from `/usr/projects/tui-vfx-recipes`:

```text
recipes/debug_recipes/event_driven_dwell/README.md
recipes/debug_recipes/event_driven_dwell/bool_binding_demo.json
recipes/debug_recipes/event_driven_dwell/bool_binding_truthy_loopback.json
recipes/debug_recipes/event_driven_dwell/integer_binding_demo.json
recipes/debug_recipes/event_driven_dwell/text_binding_demo.json
tests/test_packet_69e_event_driven_dwell.rs
src/v3/validate/col_validate_event_dwell.rs
src/v3/validate/col_validate_contracts.rs
src/v3/validate/fnc_validate_normalized_recipe.rs
src/v3/authoring/cls_v3_recipe_document.rs
src/v3/compile/cls_v3_playback_timing.rs
src/v3/compile/cls_compiled_runtime_overrides.rs
src/preview/cls_direct_v3_preview_state.rs
```

They should also inspect these from `/usr/projects/tui-vfx` only to avoid vocabulary collisions:

```text
crates/tui-vfx-compositor/src/filters/cls_glyph_timeline.rs
crates/tui-vfx-compositor/src/types/cls_filter_spec.rs
crates/tui-vfx-style/src/schedules/fnc_poisson_burst_schedule.rs
crates/tui-vfx-compositor/src/pipeline/cls_composition_playback_timing.rs
```

## Required tests

Phase I0 should include contract tests proving at least:

```text
lifecycle_spec_accepts_fixed_enter_dwell_exit
dwell_until_bool_signal_condition_is_representable
truthy_loopback_case_maps_to_level_condition_not_edge_only
dwell_policy_can_express_max_duration_cap
trigger_latch_policy_is_explicit
trigger_reset_boundary_is_explicit
typed_predicates_reject_wrong_value_kinds
recipe_document_can_include_lifecycle
recipe_schema_generation_is_current
vocabulary_mentions_trigger_gate_loopback_and_effect_schedule_distinctions
```

If the implementer adds `GateCondition`, also require:

```text
gate_condition_is_not_a_trigger_action
visibility_gate_does_not_advance_lifecycle
```

## Required schema/reference behavior

All new public contract-visible types must follow the standing D0+ rule:

```text
Serde + Schemars + rustdoc
strict JSON shape
checked generated schema root where appropriate
schema tests prove freshness and descriptions
```

The schema tests should catch:

```text
undocumented fields
unexpected additionalProperties
stale checked schema files
missing recipe.schema.json update if RecipeDocument changes
```

## Non-goals

Phase I0 must not add:

```text
runtime player
runtime trigger engine
ParameterStore / SignalStore execution
binding execution
loopback execution
recipe migration/lowering
template expansion
source recipe authoring syntax
studio manifest
phase graph execution
scene visibility execution
effect-local schedule unification
real effect ports
legacy alias support
```

## Draft implementer prompt

Use this as the assignment packet.

```text
You are implementing Phase I0 — Time / Lifecycle / Trigger Contract.

Context:
- Phase H1 completed the canonical RecipeDocument schema and docs/VOCABULARY.md.
- Existing event-driven dwell in tui-vfx-recipes is evidence only, not a constraint.
- We want the correct v3.1 model even if it differs from current recipe field names.

Goal:
Define schema-backed contract vocabulary in tui-vfx-contract for recipe-level
time, lifecycle phases, dwell policy, and trigger-driven lifecycle transitions.

Required reads:
- /usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/README.md
- /usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/bool_binding_demo.json
- /usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/bool_binding_truthy_loopback.json
- /usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/integer_binding_demo.json
- /usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/text_binding_demo.json
- /usr/projects/tui-vfx-recipes/tests/test_packet_69e_event_driven_dwell.rs
- /usr/projects/tui-vfx-recipes/src/v3/validate/col_validate_event_dwell.rs
- /usr/projects/tui-vfx-recipes/src/v3/validate/col_validate_contracts.rs
- /usr/projects/tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs
- /usr/projects/tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs
- /usr/projects/tui-vfx/crates/tui-vfx-compositor/src/filters/cls_glyph_timeline.rs
- /usr/projects/tui-vfx/crates/tui-vfx-style/src/schedules/fnc_poisson_burst_schedule.rs

Owner direction:
Do not copy current field names directly. Extract semantics. Current names like
dwell_until_binding and dwell_fallback_ms are legacy evidence. Canonical v3.1
should use explicit concepts such as lifecycle phase, dwell policy, value
condition, predicate, max duration, latch policy, and reset boundary.

Required conceptual distinctions:
- Trigger is a lifecycle transition condition.
- Gate is a continuously sampled visibility/execution condition.
- Binding/ValueSource supplies a value; it is not itself a trigger.
- Loopback is a preview/demo value provider; it is not trigger semantics.
- Effect-local schedules such as glyph timeline triggers are not lifecycle triggers.

Implementation scope:
- Add minimal DTOs to tui-vfx-contract for ClockSpec / LifecycleSpec /
  PhaseSpec / DwellPolicy / TriggerCondition / ValuePredicate /
  TriggerLatchPolicy / reset boundary or equivalent names.
- Reuse the existing ValueSource / Value / ValueKind model.
- Add lifecycle to RecipeDocument if needed.
- Add checked schemas under schemas/v3.1/contract/.
- Update docs/VOCABULARY.md and relevant architecture/checklist docs.
- Add a Phase I0 status memo.

Validation expectations:
- Reject predicate/value-kind mismatches.
- Make latch/reset semantics explicit.
- Represent dwell max duration as a cap, not a vague fallback.
- Reject inappropriate ValueSource variants in recipe-level lifecycle context,
  especially GraphValue unless a clear and tested context exists.

Non-goals:
- Do not implement a player.
- Do not execute triggers.
- Do not add runtime parameter/signal stores.
- Do not execute bindings or loopbacks.
- Do not implement scene visibility gates.
- Do not merge GlyphTimelineTriggerSpec into lifecycle trigger vocabulary.
- Do not build recipe migration/lowering.
- Do not port real effects.

Definition of done:
- New public DTOs have rustdoc, Serde, Schemars, strict JSON shapes.
- Checked schemas are generated and committed.
- Contract tests cover fixed lifecycle, dwell until value condition, max duration,
  latch/reset policy, typed predicates, and RecipeDocument integration.
- docs/VOCABULARY.md defines trigger, gate, lifecycle, phase, clock, binding,
  loopback, and effect-local schedule.
- cargo fmt/clippy/test/schema checks pass for tui-vfx-contract and workspace.
- No forbidden dependency on compositor/style/content/shadow from contract.
```

## Suggested subagent packets

### Evidence subagent

```text
Read the required event-driven dwell recipes, tests, validator, compiler timing,
and preview state files. Produce a concise evidence report answering:

1. What current fields exist?
2. What behavior do they imply?
3. Is the trigger level-based or edge-based?
4. Does it latch?
5. What resets it?
6. What happens if it never fires?
7. What types do bool/integer/text dwell demos imply?
8. Which behavior is stable enough to preserve, and which is just current
   implementation detail?

Do not propose DTOs unless directly asked. Focus on semantics.
```

### Vocabulary/schema subagent

```text
Read docs/VOCABULARY.md, schemas/v3.1/contract/*.schema.json, and the existing
contract DTO naming style. Draft vocabulary additions for clock, lifecycle,
phase, trigger, gate, loopback, binding, ValueSource, and effect-local schedule.
Also identify which new schema roots should exist for Phase I0.

Do not modify code. Return a suggested vocabulary patch and schema-root list.
```

## Expected final status memo

The Phase I0 implementer should return a memo in the established format:

```text
PHASE_I0_STATUS_MEMO_TO_ARCHITECT.md
```

It should include:

```text
Executive summary
Current implementation state
Goal-by-goal status
New DTOs
New schema roots
Vocabulary changes
Evidence read
Key decisions
What deliberately was not added
Verification evidence
Open questions / next assignment request
```

## Bottom line

H1 is the right package lock. The next durable contract gap is lifecycle and trigger semantics.

Phase I0 should stabilize the model around **clock → lifecycle phase → dwell policy → trigger condition/action**, while keeping runtime execution, loopbacks, binding stores, scene gates, and effect-local schedules out of scope.

The owner’s “correct model over existing model” direction should be explicit in the assignment: existing recipes and code are evidence, not constraints.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_H1.md</FILE> - <DESC>Architect response to Phase H1 and Phase I0 assignment</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
