<!-- <FILE>docs/new_kernel/K2_12_SCHEMA_LOCK_DECISION_REPORT.md</FILE> - <DESC>K2.12 schema-lock decision report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 schema lock: convert readiness blockers into a decision board.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record schema-readiness declaration, blocker board, and recommendations.</CLOG> -->

# K2.12 Schema-Lock Decision Report

## Declaration

```text
SCHEMA READINESS DECLARATION: NOT YET
```

K2.12 made the blocker board mechanical and retired the vague `complex ownerAudit` / style `unknown` offender output, but the remaining blockers are still real decisions. Schema lock should wait until the architect/owner explicitly resolves or signs off the source/content, runtime dynamism, scene-local pipeline, scope vocabulary, field-coverage, descriptor-expansion, GUI/backend holdback, and oracle-only disposition lanes below.

## Current machine evidence

```text
schemaVersion: v3.1.player.schemaReadiness.1
totalLegacyRecords: 603
schemaReadyRecords: 217
estimatedSchemaReadinessPercent: 36.0
canDeclareSchemaReady: false
offenders emitted with --include-offenders: 386
```

`summary.ownerAuditRecords` and `summary.unknownRecords` are raw migration-status counters retained for continuity with K2.11. K2.12 normalizes the opt-in offender rows; it does not rename historical summary fields during pre-release v3.1 work.

Offender kind counts:

| Offender kind | Count | Readiness meaning |
|---|---:|---|
| `descriptorPack` | 189 | Descriptor expansion or adapter-honesty queue. |
| `sourceDescriptor` | 74 | Source/content descriptor decisions. |
| `motionTimingSemantics` | 34 | Easing and motion-route schema semantics. |
| `sceneSemantics` | 26 | Scene/source-local pipeline and complex local sequencing. |
| `bindingSemantics` | 22 | Binding, signal, and parameter override execution. |
| `backendRenderer` | 15 | Backend/compositor holdback boundary. |
| `valueSourceSemantics` | 12 | Runtime value-source/signal/field-hint semantics. |
| `contentDescriptor` | 5 | Style scope/content vocabulary decisions. |
| `fieldCoverage` | 4 | `gradient`, `applyTo`, and `position` field dispositions. |
| `guiHumanReview` | 2 | Human visual-conflict holdbacks. |
| `oracleOnly` | 2 | Offline oracle artifacts; non-blocking after signoff. |
| `lifecycleSemantics` | 1 | Lifecycle/trigger boundary. |

## Decisions needed for significant forward progress

1. **Source/content identity policy**
   - Decide how plain text, ANSI text, typewriter/split-flap/odometer/marquee transforms, procedural sources, and command-capture artifacts map to v3.1.
   - K2.12 already added the low-risk `source.text` fixture; the remaining progress requires descriptor ownership, not more generic text plumbing.

2. **Runtime dynamism model**
   - Decide binding execution, parameter override timing, signal generators, sampled-surface value sources, and field-hint semantics.
   - This is the largest schema-level blocker because many apparently independent fields need the same sampling/ownership answer.

3. **Scene-local pipeline semantics**
   - Decide source-local pipeline ordering, sequence/parallel composition, role/scope conflict behavior, and visibility/clip/layer ownership.
   - Complex sequence/parallel fixtures cannot migrate honestly until this model is stable.

4. **Scope vocabulary for styles**
   - Decide modulo, non-empty/content, outer band, and predicate/ref scopes.
   - The highest-risk item is predicate interior scope because it implies a predicate registry rather than a fixed geometric selector.

5. **Field coverage closure**
   - Decide whether `gradient`, `applyTo`, and `position` are descriptor inputs, adapter support gaps, binding/value-source fields, or holdbacks.
   - Do not mark them handled until the destination semantics are exact.

6. **Descriptor expansion queue ownership**
   - Approve a named tranche such as `shader.revealWipe`, `shader.barberPole`, `shader.pulseWave`, `shader.orbit`, and `style.colorShift` only after source/runtime decisions stop shifting the model.

7. **Holdback signoff**
   - Explicitly sign off backend renderer, GUI human-review, oracle-only, and duplicate/variant dispositions so they stop cycling as perceived schema blockers.

## Recommendation

Do not bump schema version during this pre-release v3.1 work. Keep `v3.1.player.schemaReadiness.1` while the offender ledger remains additive, and use the ledger as the control board for the next packet.

The next high-leverage packet should settle the runtime dynamism model first, because it unblocks or clarifies bindable rates, event-driven dwell, signals, sampled-surface filters, scene bindings, and complex runtime fixtures.

<!-- <FILE>docs/new_kernel/K2_12_SCHEMA_LOCK_DECISION_REPORT.md</FILE> - <DESC>K2.12 schema-lock decision report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
