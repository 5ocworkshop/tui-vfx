<!-- <FILE>docs/new_kernel/I0_EVENT_DWELL_EVIDENCE_NOTES.md</FILE> - <DESC>Phase I0 evidence notes for lifecycle, dwell, trigger, and schedule contracts</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>New kernel Phase I0: record event-driven dwell and timing evidence without adopting legacy field names.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — clarify event-dwell recipe evidence root.
0.1.0: INIT — capture lifecycle/time/trigger evidence mapping pressure for Phase I0.</CLOG> -->

# Phase I0 Event-Dwell Evidence Notes

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: I0 — Time / Lifecycle / Trigger Contract

## Evidence read

The following legacy/current materials were read as evidence only. Recipe evidence root:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes
```

Specific files:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/README.md
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/*dwell*.json
src/v3/validate/col_validate_event_dwell.rs
src/v3/validate/col_validate_contracts.rs
src/v3/validate/fnc_validate_normalized_recipe.rs
src/v3/authoring/cls_v3_recipe_document.rs
src/v3/compile/cls_v3_playback_timing.rs
src/v3/compile/cls_compiled_runtime_overrides.rs
src/preview/cls_direct_v3_preview_state.rs
crates/tui-vfx-compositor/src/filters/cls_glyph_timeline.rs
crates/tui-vfx-compositor/src/types/cls_filter_spec.rs
crates/tui-vfx-style/src/schedules/fnc_poisson_burst_schedule.rs
crates/tui-vfx-compositor/src/pipeline/cls_composition_playback_timing.rs
```

## Evidence-to-contract mapping

| Evidence concept | Canonical I0 interpretation |
|---|---|
| `dwell_until_binding` | A recipe-level lifecycle trigger condition using `ValueSource` plus `ValuePredicate`; old field name is not canonical. |
| `dwell_fallback_ms` | `DwellPolicy::Until.maxDuration`; it caps dwell duration rather than naming a vague fallback behavior. |
| bool event dwell | `ValuePredicate::IsTrue` / `IsFalse` against a `SignalSpec` when host/event-like. |
| integer/text demos | Prefer `NonZero` / `NonEmpty`; `Truthy` exists only as documented/tested convenience. |
| trigger latching | `TriggerLatchPolicy` and `TriggerResetBoundary` are explicit fields. |
| loopback demos | Future preview/demo value providers; not trigger semantics. |
| scene visibility predicates | Gates, not triggers; visibility/execution gates remain deferred in I0. |
| glyph timelines / poisson schedules / wavefronts | Effect-local schedules; not recipe lifecycle triggers. |
| graph-local node outputs | Rejected for recipe-level lifecycle triggers in I0 because graph values are produced during graph execution. |

## Locked distinctions

```text
Clock: time sample space.
Lifecycle: enter → dwell → exit → finished progression.
Phase: named lifecycle interval.
Trigger: condition that requests a lifecycle action.
Gate: continuously sampled visibility/execution condition.
Binding / ValueSource: value supplier, not transition semantics.
Loopback: preview/demo value provider, not trigger semantics.
Effect-local schedule: per-effect activation timing, not lifecycle.
```

## Deferrals preserved

I0 intentionally does not add a runtime player, trigger engine, binding execution, signal store, parameter store, loopback executor, migration/lowering, scene visibility gates, phase graph execution, source authoring syntax, effect-local schedule unification, or legacy aliases.

<!-- <FILE>docs/new_kernel/I0_EVENT_DWELL_EVIDENCE_NOTES.md</FILE> - <DESC>Phase I0 evidence notes for lifecycle, dwell, trigger, and schedule contracts</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
