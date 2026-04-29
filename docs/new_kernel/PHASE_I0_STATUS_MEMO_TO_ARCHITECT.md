<!-- <FILE>docs/new_kernel/PHASE_I0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase I0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase I0 wrap: report lifecycle/time/trigger schema and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase I0 architect memo in the established status-memo style.</CLOG> -->

# Phase I0 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: I0 — Time / Lifecycle / Trigger Contract

## Executive summary

Phase I0 implements the time/lifecycle/trigger contract direction from `ARCH-RESP-TO-PHASE_H1.md`.

Current answer: **yes, v3.1 can now describe recipe-level time, lifecycle phases, dwell policy, and trigger-driven lifecycle transitions as strict canonical contract vocabulary without copying legacy field names or implementing a runtime player.** `tui-vfx-contract` owns `LifecycleSpec`, `ClockSpec`, `PhaseSpec`, `DwellPolicy`, `TriggerSpec`, and `ValuePredicate` DTOs; `RecipeDocument` can carry `lifecycle: Option<LifecycleSpec>`; schema fixtures are checked in; and tests validate the conceptual distinctions you requested.

The phase intentionally stops before runtime playback, trigger execution, signal/parameter stores, binding execution, loopback execution, migration/lowering, scene gates, effect-local schedule unification, source authoring syntax, or real ports.

## Current implementation state

Stable contract crate:

```text
crates/tui-vfx-contract
```

New contract vocabulary:

```text
DurationSpec
ClockMode
ClockSpec
LifecyclePhase
LifecycleSpec
PhaseSpec
PhaseTiming
DwellPolicy
TriggerSpec
TriggerCondition
ValuePredicate
TriggerLatchPolicy
TriggerResetBoundary
TriggerAction
```

Recipe root integration:

```text
RecipeDocument.lifecycle: Option<LifecycleSpec>
```

New checked schema roots:

```text
schemas/v3.1/contract/duration.schema.json
schemas/v3.1/contract/clock.schema.json
schemas/v3.1/contract/dwell-policy.schema.json
schemas/v3.1/contract/trigger.schema.json
schemas/v3.1/contract/value-predicate.schema.json
schemas/v3.1/contract/phase.schema.json
schemas/v3.1/contract/lifecycle.schema.json
```

## Goal-by-goal status against the I0 recommendation

| I0 goal / question | Current status |
|---|---|
| Recipe-level lifecycle first | **Done.** `RecipeDocument.lifecycle: Option<LifecycleSpec>` attaches lifecycle to the canonical recipe root. |
| Clock sample space | **Done.** `ClockSpec` distinguishes monotonic vs looping clocks and validates loop period rules. |
| Enter/dwell/exit lifecycle phases | **Done.** `LifecycleSpec` validates ordered enter, dwell, exit phase declarations. |
| Dwell fallback as max duration | **Done.** Trigger-terminated dwell uses `maxDuration`, not fallback vocabulary. |
| Explicit typed predicates | **Done.** `ValuePredicate` includes boolean, numeric, string/text, comparison, equality, and documented/tested `Truthy`. |
| Explicit latch/reset semantics | **Done.** `TriggerLatchPolicy` and `TriggerResetBoundary` are required trigger fields. |
| Trigger sources reuse existing value model | **Done.** `TriggerCondition` uses `ValueSource` plus `ValuePredicate`. |
| Graph values rejected for recipe lifecycle triggers | **Done.** Recipe-level lifecycle validation rejects `ValueSource::GraphValue`. |
| Host event-like inputs as signals | **Done in examples/tests.** Dwell-until-dismissed is represented as `SignalSpec` + `ValueSource::Signal`. |
| Gates separated | **Done.** Gates are documented as continuously sampled conditions and deferred from I0 implementation. |
| Loopback separated | **Done.** Loopback is documented as preview/demo value provision, not trigger semantics. |
| Effect-local schedules separated | **Done.** Glyph timelines/poisson schedules remain evidence only and are not unified with lifecycle triggers. |

## Evidence grounding

Created:

```text
docs/new_kernel/I0_EVENT_DWELL_EVIDENCE_NOTES.md
```

The existing event-driven dwell implementation, debug recipes, preview timing, compile timing, loopback behavior, and local schedule code were read as evidence only. The notes map legacy/current concepts into canonical homes while explicitly avoiding old field names such as `dwell_until_binding` and `dwell_fallback_ms`.

## Vocabulary artifact

Updated:

```text
docs/VOCABULARY.md
```

The vocabulary now locks the distinctions you requested:

```text
Clock: time sample space
Lifecycle: enter → dwell → exit → finished
Phase: named lifecycle interval
Trigger: condition causing lifecycle action
Gate: continuously sampled visibility/execution condition
Binding / ValueSource: supplies value, not transition semantics
Loopback: preview/demo value provider
Effect-local schedule: per-effect activation timing
```

And the negative rules:

```text
Trigger ≠ Gate
Trigger ≠ Binding
Trigger ≠ Loopback
Trigger ≠ ValueSource
Lifecycle trigger ≠ GlyphTimelineTriggerSpec
```

## Key decisions

### Lifecycle is recipe-level for I0

I0 attaches lifecycle semantics to `RecipeDocument`, not to graph nodes, elements, sources, or templates. More granular lifecycle scopes can wait for evidence from a later phase.

### Dwell caps are maxDuration

The old fallback name was not preserved. The canonical model says: dwell until a trigger fires, but no longer than `maxDuration` when a cap is present.

### Trigger semantics are not hidden in binding/source fields

A `ValueSource` supplies a value. A `TriggerSpec` owns condition, predicate, latch, reset, and action semantics.

### `Truthy` is allowed but not magical-only

Explicit predicates are preferred. `Truthy` exists as a tested convenience for legacy/demo pressure and is documented as a level predicate, not an edge-trigger shape.

## Verification evidence

Verification run for Phase I0:

Final wrap verification passed:

```text
cargo fmt --package tui-vfx-contract -- --check
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current
cargo test -p tui-vfx-contract --test test_lifecycle_contract
cargo test -p tui-vfx-contract --tests
cargo tree -p tui-vfx-contract
forbidden legacy dependency / legacy field grep over contract src and schemas
git diff --check
cargo test --workspace
```

Architect re-review approved after the `maxDuration` schema casing and per-kind `Truthy` documentation/test fixes. Deslop review completed on I0-owned files; no broad refactor was needed.

## Request for next assignment

Please review Phase I0 as the lifecycle/time/trigger contract lock point and advise the next phase.

Likely next work can now choose between runtime-proof work, lowering/migration skeletons, scene gate contracts, demo/loopback contracts, or source/effect porting only after you confirm which dependency should come first.

<!-- <FILE>docs/new_kernel/PHASE_I0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase I0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
