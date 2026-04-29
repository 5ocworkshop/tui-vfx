<!-- <FILE>docs/new_kernel/PHASE_K2_11_SCHEMA_READINESS_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.11 schema-readiness status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.11 v3.1 schema-readiness ledger, source descriptor pilot, and next blocker sequence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — summarize schema-readiness evidence, source/content decisions, and actionable next packets.</CLOG> -->

# Phase K2.11 Schema-Readiness Status Memo to Architect

## Rolling context

K2.1 through K2.10 built the v3.1 migration evidence path: migration-gap reporting, visual-frame evidence, primitive adapter and field coverage gates, styled-cell output, player UI shell, fixture-QC, simple mask descriptor expansion, migration mapping, and full 603-record corpus mapping. K2.11 converts the K2.10 backlog into a first-class schema-readiness ledger and begins the source descriptor pilot on the v3.1 pathway.

## Executive summary

K2.11 completed the required v3.1 schema-readiness control surface:

- added `tui-vfx-player-cli schema-readiness`, emitting `v3.1.player.schemaReadiness.1`,
- grouped all 603 legacy records into blocker/disposition buckets with counts and representative paths,
- added a bounded `source.text` descriptor pilot to the v3.1 primitive descriptor pack,
- kept runtime command execution out of the player and docs,
- deferred studio control-surface implementation until source/value/binding semantics are stable,
- preserved the legacy debug recipe root as read-only evidence.

## Direct answer: can we declare 100% schema readiness?

**No.** Current evidence says:

```text
totalLegacyRecords:                 603
schemaReadyRecords:                 217
estimatedSchemaReadinessPercent:   36.0
canDeclareSchemaReady:             false
```

The important improvement is that the blockers are now actionable. We are no longer merely saying “there are unaddressed fields and decisions”; the ledger identifies the high-level decision lanes and record counts that must be burned down.

## High-level blockers that are stopping forward progress

These are the real v3.1 blockers to resolve next:

1. **Source/content identity and descriptor policy — 67 blocking records.**
   - The work is not just adding `source.text`; it is deciding where plain text, ANSI text, image-like sources, procedural sources, command-capture artifacts, and content transforms belong in v3.1.
   - Progress path: implement/fixture `source.text`, decide `source.ansi` descriptor versus adapter behavior, keep command capture offline/oracle only, and split content transforms from sources.

2. **Runtime dynamism semantics — 60 blocking records.**
   - Binding, signal, sampled-surface value-source, lifecycle, easing, and motion-route examples are schema/model blockers, not descriptor-pack chores.
   - Progress path: one K2.12 packet should settle binding execution, parameter override, value-source sampling, trigger/dwell boundaries, and motion timing vocabulary.

3. **Scene/source-local pipeline semantics — 12 scene records plus overlap with complex records.**
   - Scene placement exists, but source-local pipeline, ANSI chains, asset/procedural sources, layer visibility, clipping, overflow, and scene-local schedule semantics are not settled.
   - Progress path: a focused scene/source-local pipeline decision packet after source/content and runtime-dynamism decisions have enough shape.

4. **Complex owner-audit bucket — 73 blocking records.**
   - `complex/*` is the biggest catch-all still hiding true blocker types. Some records likely become descriptor composition, some source/content pipeline, some command-capture oracle, some GUI/human-review, and possibly a small schema subset.
   - Progress path: do not migrate these directly. First normalize them into explicit v3.1 blocker kinds.

5. **Primitive field coverage — 4 blocking shader records.**
   - `gradient`, `applyTo`, and `position` are now precise blockers, not vague unsupported fields.
   - Progress path: decide whether these are additive descriptor inputs, adapter support gaps, binding/value-source semantics, or oracle-only records.

6. **Descriptor vocabulary — 151 non-schema migration blockers.**
   - These do not prevent a schema-readiness declaration if explicitly deferred, but they do block significant visible migration progress.
   - Progress path: expand descriptor packs after source/runtime semantics are settled enough to avoid encoding the wrong model.

7. **Non-schema disposition policy — 210 non-blocking records after signoff.**
   - 193 oracle-only, 14 backend-renderer, and 3 duplicate/variant records must be explicitly signed off so they stop reappearing as perceived schema risk.

## Schema-readiness command

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

## Summary counts

| Metric | Count |
|---|---:|
| totalLegacyRecords | 603 |
| schemaReadyRecords | 217 |
| schemaBlockedRecords | 72 |
| sourceBlockedRecords | 67 |
| descriptorBlockedRecords | 151 |
| adapterBlockedRecords | 0 |
| fieldCoverageBlockedRecords | 4 |
| ownerAuditRecords | 280 |
| oracleOnlyRecords | 193 |
| duplicateOrVariantRecords | 3 |
| unknownRecords | 5 |
| estimatedSchemaReadinessPercent | 36.0 |
| canDeclareSchemaReady | false |

## Per-family blocker counts

`ownerAuditRecords` is the raw migration-mapping status count. The blocker ledger further normalizes many of those records into explicit `oracleOnly`, `backendRenderer`, and `duplicateOrVariant` dispositions; the residual schema-readiness-blocking owner-audit bucket is the 73-record `complex` group.


| Family | Records | Blocker counts |
|---|---:|---|
| bindable_rates | 8 | bindingSemantics 8 |
| complex | 83 | oracleOnly 10; ownerAudit 73 |
| content | 111 | oracleOnly 45; sourceDescriptor 66 |
| easings | 29 | motionTimingSemantics 29 |
| event_driven_dwell | 4 | bindingSemantics 3 |
| filters | 98 | descriptorPack 46; oracleOnly 45; valueSourceSemantics 3 |
| fixtures | 1 | sourceDescriptor 1 |
| loopback | 3 | oracleOnly 3 |
| masks | 41 | descriptorPack 15; duplicateOrVariant 3; oracleOnly 15 |
| motion_routes | 5 | motionTimingSemantics 5 |
| other | 2 | oracleOnly 1 |
| samplers | 13 | descriptorPack 5; oracleOnly 6 |
| scene | 19 | bindingSemantics 6; lifecycleSemantics 1; sceneSemantics 12 |
| shaders | 133 | descriptorPack 72; fieldCoverage 4; oracleOnly 54 |
| shadows | 9 | backendRenderer 9 |
| signals | 5 | bindingSemantics 5 |
| styles | 34 | descriptorPack 13; oracleOnly 14; unknown 5 |
| subcell_shapes | 5 | backendRenderer 5 |

## Source/content descriptor pilot results

- `source.text` descriptor added to the primitive descriptor pack.
- `source.text` exposes `text`, `width`, and `height` inputs.
- `source.text` is classified as a `visible` player adapter in inventory evidence.
- `source.card` remains intact and is not overloaded to represent broader content semantics.
- `source.ansi`, `source.image`, procedural sources, and command-capture artifacts are explicitly deferred for decision instead of being forced into the current model.

Inventory evidence:

```text
source.card: descriptorCovered=true, representedByRecipes=true, adapterStatus=visible
source.text: descriptorCovered=true, representedByRecipes=false, adapterStatus=visible
```

## Lifecycle/value-source/schema blocker results

| Blocker | Records | Families | Missing decision | Recommended packet |
|---|---:|---|---|---|
| motionTimingSemantics | 34 | easings, motion_routes | Easing and route timing vocabulary; route-versus-effect boundary | K2.12 lifecycle/signal/binding/value-source schema decision packet |
| bindingSemantics | 22 | bindable_rates, event_driven_dwell, scene, signals | Binding execution, parameter override, and signal source/generator boundary | K2.12 lifecycle/signal/binding/value-source schema decision packet |
| valueSourceSemantics | 3 | filters | Sampled-surface values and signal-shaped descriptor inputs | K2.12 lifecycle/signal/binding/value-source schema decision packet |
| lifecycleSemantics | 1 | scene | Layer I/O filter/shader lifecycle and trigger/dwell boundaries | K2.12 lifecycle/signal/binding/value-source schema decision packet |
| sceneSemantics | 12 | scene | Source-local pipeline, ANSI/asset/procedural source integration, layers | K2.12 scene/source-local pipeline schema decision packet |

## Primitive field-coverage decisions

| Legacy path | Unsupported fields | K2.11 classification | Next action |
|---|---|---|---|
| `shaders/primitives/shader_linear_gradient_diagonal.json` | `gradient` | fieldCoverage, schema-readiness blocking | Decide additive descriptor input versus adapter support. |
| `shaders/primitives/shader_linear_gradient_background_channel.json` | `gradient` | fieldCoverage, schema-readiness blocking | Decide additive descriptor input versus adapter support. |
| `shaders/primitives/shader_linear_gradient_apply_to_both.json` | `applyTo`, `gradient` | fieldCoverage, schema-readiness blocking | Decide whether `applyTo` follows existing filter-style target semantics and how `gradient` maps. |
| `shaders/compositions/shader_border_sweep_position_binding.json` | `position` | fieldCoverage, schema-readiness blocking | Decide descriptor input versus binding/value-source semantics. |

## Owner-audit triage results

- 193 records are now classified as `oracleOnly` disposition candidates.
- 14 records are classified as `backendRenderer` rather than schema blockers.
- 3 mask records are classified as duplicate/variant.
- 73 complex records remain `ownerAudit` because evidence is mixed enough that overclassification would hide schema risk.
- 5 style records remain `unknown`; they are explicitly listed in the ledger and should be reviewed with style descriptor/scope policy.

## Control-surface preflight status

Deferred. A trustworthy control surface depends on source descriptors, value-source semantics, binding execution, lifecycle/motion timing, and field coverage. K2.11 documents the K3.0 trigger in `docs/new_kernel/K2_11_STUDIO_CONTROL_SURFACE_PREFLIGHT.md` instead of inventing premature controls.

## Optional fixture additions

None in this repo packet. `source.text` is descriptor-backed and adapter-visible, but no canonical v3.1 recipe fixture was added to the external recipe repo during this source-tree change.

## Verification matrix

Final verification completed after the briefing-first formal review/de-slop correction:

- `cargo fmt --package tui-vfx-player --package tui-vfx-player-cli --package tui-vfx-player-ui --package tui-vfx-contract-cli -- --check` — PASS.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-player-ui -p tui-vfx-contract-cli --all-targets -- -D warnings` — PASS.
- `cargo nextest run -p tui-vfx-player-cli --no-fail-fast` — PASS, 31 tests.
- `cargo nextest run -p tui-vfx-player --no-fail-fast` — PASS, 18 tests.
- `cargo nextest run -p tui-vfx-contract-cli --no-fail-fast` — PASS, 11 tests.
- Canonical v3.1 report gates `validate-recipe`, `fixture-qc`, `primitive-field-coverage`, and `primitive-adapter-gap` — PASS.
- `migration-mapping-batch --recursive --json` — PASS with 603 records, 18 families, `candidateReady=0`, and `schemaDecisionNeeded=72`.
- `migration-mapping-batch --family masks --json` — PASS with 41 mask records and no candidate-ready overclaim.
- `schema-readiness --recursive --json` — PASS with 603 records, 31 grouped blockers, 36.0% readiness, and `canDeclareSchemaReady=false`.
- `inventory-recipes --recursive --json` — PASS with `source.card` and `source.text` covered; `source.text` adapter status `visible`.
- `git diff --check` — PASS.
- Scoped hard-coded recipe path scan over changed files — PASS.
- Recipe repo mutation check for `recipes/debug_recipes` and `recipes/v3.1/debug_recipes` — PASS, no output.
- Briefing-first AI de-slop review — PASS after reading `.omx/context/k211-subagent-briefing.md`; required wording fix applied here.
- Briefing-first formal code review — PASS after reading `.omx/context/k211-subagent-briefing.md`; no required fixes.

## Legacy root mutation status

The legacy debug recipe root remains read-only evidence. K2.11 code and docs do not require modifying `recipes/debug_recipes`.

## Recommended next packet

Recommended next work should be explicitly sequenced for progress:

1. **K2.12 — source/content descriptor expansion tranche.** Add a v3.1 `source.text` canonical fixture, decide `source.ansi`, keep command-capture offline, and split content transforms from sources.
2. **K2.12 — lifecycle/signal/binding/value-source schema decision packet.** Resolve the largest true schema blocker set: motion timing, binding execution, parameter override, signals, sampled-surface values, and lifecycle boundaries.
3. **K2.12 — owner-audit normalization tranche.** Split the 73 complex records and 5 unknown style records into explicit blocker/disposition buckets.
4. **K2.12 — primitive descriptor/field-coverage closure tranche.** Resolve `gradient`, `applyTo`, and `position` blockers.
5. **K2.13 or later — descriptor-pack expansion.** Burn down descriptor-only filters/masks/samplers/shaders/styles after the schema/source lanes stop shifting.

Do not prioritize compositor backend wiring yet; schema/source/runtime semantics remain the higher-leverage blocker path.

<!-- <FILE>docs/new_kernel/PHASE_K2_11_SCHEMA_READINESS_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.11 schema-readiness status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
