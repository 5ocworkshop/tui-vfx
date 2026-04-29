<!-- <FILE>docs/new_kernel/PHASE_K2_13_SCHEMA_DECISION_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.13 schema decision burn-down status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: final architect memo and blocker statement.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — report schema-readiness declaration, evidence, and forward-progress blockers.</CLOG> -->

# Phase K2.13 Schema Decision Burn-Down Status Memo to Architect

## Declaration

```text
SCHEMA READINESS DECLARATION: APPROVED FOR v3.1 SCHEMA DECISION READINESS
```

This means every active legacy `debug_recipes/` record is now either representable by accepted v3.1 schema, signed off as descriptor/adapter/backend/gui/oracle/duplicate backlog, or listed for exact owner decision. The exact owner-decision list is empty.

This does **not** mean every legacy recipe is visually ported.

## Terminology note

K2.13 and related phase labels are work-packet references only. They must not become product vocabulary, report field names, schema values, variable names, or descriptor ids. The durable public vocabulary from this packet is the disposition set: `acceptedSchema`, `descriptorBacklog`, `adapterBacklog`, `backendHoldback`, `guiHumanReviewHoldback`, `oracleOnly`, `duplicateVariant`, and `explicitOwnerDecisionNeeded`.

## Fresh machine evidence

```text
schemaVersion: v3.1.player.schemaReadiness.1
totalLegacyRecords: 603
offenders: 383
fieldCoverageBlockedRecords: 0
unresolvedSchemaBlockers: 0
explicitOwnerDecisionNeeded: 0
remainingOwnerDecisionCount: 0
canDeclareSchemaReady: true
```

Disposition counts:

| Disposition | Count |
|---|---:|
| `acceptedSchema` | 125 |
| `descriptorBacklog` | 263 |
| `backendHoldback` | 15 |
| `guiHumanReviewHoldback` | 2 |
| `oracleOnly` | 195 |
| `duplicateVariant` | 3 |

Offender kind counts:

| Offender kind | Count |
|---|---:|
| `descriptorPack` | 189 |
| `sourceDescriptor` | 74 |
| `motionTimingSemantics` | 34 |
| `sceneSemantics` | 26 |
| `bindingSemantics` | 22 |
| `backendRenderer` | 15 |
| `valueSourceSemantics` | 13 |
| `contentDescriptor` | 5 |
| `guiHumanReview` | 2 |
| `oracleOnly` | 2 |
| `lifecycleSemantics` | 1 |

## What changed

- The schema-readiness report now has an explicit disposition control surface.
- `shader.linearGradient.gradient`, `shader.linearGradient.applyTo`, and `shader.borderSweep.position` are accepted and player-handled.
- The contract now represents typed gradients, sampled fields, optional descriptor inputs, and the accepted built-in scope vocabulary.
- Source/content, runtime dynamism, scene/layer, motion/scope, complex graph I/O, templates, and studio-control boundaries are documented as decisions rather than generic unresolved fields.

## Higher-level blockers to real forward progress

The forward-progress blockers are no longer “unaddressed fields” or “unknown schema decisions.” They are concrete implementation queues:

1. **Descriptor backlog is the largest blocker.** 263 records are signed off as schema-ready but still need descriptor vocabulary and descriptor-level documentation before fixture migration can move quickly.
2. **Source/content adapters are still thin.** `source.ansi`, `source.image`, `source.procedural`, and content descriptors are schema-accepted, but most do not yet have real player/source adapters or visual evidence.
3. **Graph execution evidence is incomplete.** Sequence/parallel, graph I/O, and graph-value merge semantics are accepted as schema, but need more canonical fixtures and executor/player proof to avoid regressions.
4. **Backend renderer work is intentionally held back.** Shadow/subcell/compositor fidelity needs its own backend packet with visual evidence policy; forcing it into schema readiness would restart the loop.
5. **Human-review conflicts need policy, not schema churn.** The GUI/human-review holdbacks require visual conflict policy and owner signoff in a review lane.
6. **Oracle-only records must stay out of runtime.** Deprecated/capture/oracle evidence should remain raw evidence unless a future offline compiler/export lane consumes it.

## Recommended next packet

Start a descriptor/adapter migration tranche rather than another schema-decision packet. The highest leverage lane is:

```text
Descriptor backlog tranche 1:
  content.typewriter / content.marquee / content.splitFlap
  source.ansi / source.procedural
  shader/style descriptors already covered by player primitives
```

Acceptance should be fixture-plus-player evidence, not another readiness vocabulary pass.

<!-- <FILE>docs/new_kernel/PHASE_K2_13_SCHEMA_DECISION_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.13 schema decision burn-down status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
