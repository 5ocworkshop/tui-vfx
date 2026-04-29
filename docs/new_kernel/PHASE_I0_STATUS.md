<!-- <FILE>docs/new_kernel/PHASE_I0_STATUS.md</FILE> - <DESC>Phase I0 time lifecycle trigger contract implementation status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase I0 wrap: summarize lifecycle, clock, trigger, predicate, schema, and vocabulary contracts.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase I0 status for architect handoff.</CLOG> -->

# Phase I0 Status — Time / Lifecycle / Trigger Contract

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: I0 — Time / Lifecycle / Trigger Contract

## Summary

Phase I0 implements the lifecycle/time/trigger layer requested in `ARCH-RESP-TO-PHASE_H1.md`.

Current answer: **`tui-vfx-contract` can now describe recipe-level time, lifecycle phases, dwell policy, and trigger-driven lifecycle transitions as strict canonical contract vocabulary. `RecipeDocument.lifecycle: Option<LifecycleSpec>` attaches lifecycle semantics to the canonical recipe root. Schemas are generated and checked in, rustdoc descriptions are enforced, and validation proves typed predicates, explicit latch/reset semantics, max-duration dwell caps, and graph-value rejection for recipe-level lifecycle triggers.**

No runtime trigger engine, player, store, binding execution, migration/lowering, scene visibility gate execution, or effect-local schedule unification was added.

## Implemented contract APIs

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
RecipeDocument.lifecycle
```

New validation errors were added to `DescriptorValidationError` for invalid durations, clock-period misuse, duplicate/missing/out-of-order lifecycle phases, dwell timing on non-dwell phases, recipe-level graph value source rejection, and predicate kind mismatches.

## New checked schema roots

```text
schemas/v3.1/contract/duration.schema.json
schemas/v3.1/contract/clock.schema.json
schemas/v3.1/contract/dwell-policy.schema.json
schemas/v3.1/contract/trigger.schema.json
schemas/v3.1/contract/value-predicate.schema.json
schemas/v3.1/contract/phase.schema.json
schemas/v3.1/contract/lifecycle.schema.json
```

`recipe.schema.json` was regenerated to include optional `lifecycle`.

## Tests added/updated

```text
crates/tui-vfx-contract/tests/test_lifecycle_contract.rs
crates/tui-vfx-contract/tests/test_schema_generation.rs
crates/tui-vfx-contract/tests/test_recipe_document_contract.rs
```

Coverage includes:

- `lifecycle_spec_accepts_fixed_enter_dwell_exit`
- `dwell_until_bool_signal_condition_is_representable`
- `truthy_loopback_case_maps_to_level_condition_not_edge_only`
- `dwell_policy_can_express_max_duration_cap`
- `trigger_latch_policy_is_explicit`
- `trigger_reset_boundary_is_explicit`
- `typed_predicates_reject_wrong_value_kinds`
- `recipe_document_can_include_lifecycle`
- `lifecycle_trigger_rejects_graph_value_sources`
- `recipe_schema_generation_is_current`
- `vocabulary_mentions_trigger_gate_loopback_and_effect_schedule_distinctions`

## Vocabulary and evidence artifacts

Updated:

```text
docs/VOCABULARY.md
```

Created:

```text
docs/new_kernel/I0_EVENT_DWELL_EVIDENCE_NOTES.md
```

Vocabulary now explicitly distinguishes `Clock`, `Lifecycle`, `Phase`, `Trigger`, `Gate`, `Binding / ValueSource`, `Loopback`, and `Effect-local schedule`. It also records the required guardrails:

```text
Trigger ≠ Gate
Trigger ≠ Binding
Trigger ≠ Loopback
Trigger ≠ ValueSource
Lifecycle trigger ≠ GlyphTimelineTriggerSpec
```

## Deliberately not added

```text
runtime player
trigger engine
ParameterStore / SignalStore
binding execution
loopback execution
migration / lowering
template expansion
source authoring syntax
studio manifest
phase graph execution
scene visibility gate execution
effect-local schedule unification
real ports
legacy aliases such as dwell_until_binding or dwell_fallback_ms
```

## Verification status

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

## Worktree note

The following pre-existing unrelated files remain outside Phase I0 scope and should not be staged into the I0 commit:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

<!-- <FILE>docs/new_kernel/PHASE_I0_STATUS.md</FILE> - <DESC>Phase I0 time lifecycle trigger contract implementation status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
