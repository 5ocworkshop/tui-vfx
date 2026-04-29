<!-- <FILE>docs/new_kernel/PHASE_K2_12_SCHEMA_LOCK_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.12 schema-lock status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 v3.1 schema-lock decision sprint and low-friction burn-down.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — summarize offender ledger, source.text fixture, blocker board, and next decisions.</CLOG> -->

# Phase K2.12 Schema-Lock Status Memo to Architect

## Executive summary

K2.12 moved the v3.1 schema-readiness work from a broad blocker ledger to an executable control board:

- added opt-in `schema-readiness --include-offenders` offender rows,
- kept schema version at `v3.1.player.schemaReadiness.1` because v3.1 is pre-release and the report change is additive,
- removed generic `ownerAudit` and `unknown` from offender output by classifying complex/style records,
- added and verified the low-friction canonical `source.text` fixture in the v3.1 recipe pathway,
- documented exact decision tables for source/content, runtime dynamism, complex/style, descriptor expansion, schema/API docs, low-friction burn-down, and schema-lock status.

## Declaration

```text
SCHEMA READINESS DECLARATION: NOT YET
```

This is not a vague “fields remain” answer. The remaining blockers are the specific decision lanes below. Until those lanes are resolved or explicitly held back, a schema-lock declaration would freeze an incomplete model.

## High-level blockers that need owner/architect decisions

1. **Runtime dynamism is the biggest forward-progress blocker.**
   - It spans 22 binding records, 12 value-source records, 34 motion/easing records, 1 lifecycle record, and 9 complex runtime examples.
   - We need one model for binding execution, parameter override timing, signal ownership, sampled-surface value sources, and field-hint sampling.

2. **Source/content identity remains unsettled beyond `source.text`.**
   - The new `source.text` fixture proves plain text source plumbing.
   - Typewriter, split-flap, odometer, marquee, ANSI, image/procedural, and command-capture cases require source/content descriptor decisions instead of being shoved into `source.card` or `source.text`.

3. **Scene-local pipeline semantics block complex sequence/parallel migration.**
   - Fourteen complex records now mechanically classify as `sceneSemantics`.
   - The decision is source-local pipeline ordering, role/scope conflict behavior, and sequence/parallel composition, not merely descriptor presence.

4. **Style scope vocabulary needs explicit acceptance.**
   - The five former unknown style records are now scope vocabulary work: modulo, non-empty/content, outer band, and predicate/ref scope.
   - Predicate interior scope is the riskiest because it implies registry semantics.

5. **Primitive field coverage has exact blockers.**
   - `gradient`, `applyTo`, and `position` must be accepted as descriptor fields, adapter fields, binding/value-source semantics, or holdbacks.
   - K2.12 intentionally did not mark them handled prematurely.

6. **Descriptor expansion can move only after model decisions stop shifting.**
   - The queue has low-friction candidates, but broad expansion before runtime/source decisions risks encoding the wrong vocabulary.

7. **Backend, GUI, oracle, and duplicate/variant dispositions need signoff.**
   - These do not necessarily require schema changes, but they need explicit holdback/owner acceptance so they stop resurfacing as blockers.

## Implemented evidence

```text
schema-readiness --include-offenders: 386 offender rows
complex offender kinds: descriptorPack=38, sceneSemantics=14, valueSourceSemantics=9, sourceDescriptor=8, guiHumanReview=2, backendRenderer=1, oracleOnly=1
unknown offender rows: 0
ownerAudit offender rows: 0
source.text fixture validate/render/fixture-qc: pass
```

Note: top-level `summary.ownerAuditRecords` and `summary.unknownRecords` remain raw migration-status counters. The K2.12 normalization claim applies to `offenders[]` rows emitted with `--include-offenders`.

## Files of record

- `docs/new_kernel/K2_12_SCHEMA_LOCK_DECISION_REPORT.md`
- `docs/new_kernel/K2_12_SOURCE_CONTENT_DECISION_TABLE.md`
- `docs/new_kernel/K2_12_RUNTIME_DYNAMISM_DECISION_MATRIX.md`
- `docs/new_kernel/K2_12_COMPLEX_STYLE_NORMALIZATION_REPORT.md`
- `docs/new_kernel/K2_12_DESCRIPTOR_EXPANSION_QUEUE.md`
- `docs/new_kernel/K2_12_SCHEMA_API_DOC_INFRA_REPORT.md`
- `docs/new_kernel/K2_12_LOW_FRICTION_BURN_DOWN_REPORT.md`
- `docs/new_kernel/PHASE_K2_12_REVIEW_AND_DESLOP_REPORT.md`

## Recommended next packet

Run a focused runtime dynamism decision packet before large descriptor expansion. It provides the shared answer needed for bindable rates, event-driven dwell, signals, sampled-surface value sources, scene bindings, and complex runtime examples.

<!-- <FILE>docs/new_kernel/PHASE_K2_12_SCHEMA_LOCK_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.12 schema-lock status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
