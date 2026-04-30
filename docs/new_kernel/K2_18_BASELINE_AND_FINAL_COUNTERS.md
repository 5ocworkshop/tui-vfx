# K2.18 Baseline and Final Counters

K2.18 closed implementation blocker queues by converting every generic implementation blocker into a canonical mapping, resolved implementation disposition, or signed path-level holdback. Raw migration mapping still exposes schema/readiness audit counters; use the implementation-readiness ledger for implementation closure.

Evidence date: 2026-04-30

Refreshed report inputs:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
cargo run -q -p tui-vfx-player-cli -- implementation-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive --include-blockers --json > /tmp/k218-doc-impl.json
cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive --json > /tmp/k218-doc-migration.json
```


## Before/after counters

| Counter | K2.17 baseline | K2.18 final | Result |
| --- | --- | --- | --- |
| canonical fixtures in active corpus | 142 | 144 | +2 style fixtures |
| implementation records | 603 | 603 | no corpus cardinality change in legacy inventory |
| implementation canonicalExists | 159 | 163 | +4 legacy records now canonical-mapped; +2 physical fixtures |
| implementationBlocking | 223 | 0 | all generic implementation blocker queues closed or signed |
| contentBacklog | 39 | 0 | closed by signed duplicate/backend dispositions |
| sourceBacklog | 1 | 0 | closed by sourceBacklogResolved |
| descriptorBacklog | 84 | 0 | closed by descriptorBacklogResolved/backend/graph/canonical dispositions |
| graphRuntimeBacklog | 83 | 0 | closed by graphRuntimeResolved |
| sceneRuntimeBacklog | 16 | 0 | closed by sceneRuntimeResolved |
| explicitOwnerDecisionNeeded | 0 | 0 | remains zero |
| migration canonicalExists | 97 | 99 | raw migration mapping +2 |
| migration descriptorDecisionNeeded | 76 | 78 | raw mapping inventory remains schema/audit-oriented; implementation ledger signs or resolves exact paths |
| migration sourceDecisionNeeded | 40 | 40 | raw mapping inventory unchanged; implementation source queue is closed |
| migration blockedByFieldCoverage | 8 | 0 | field blocker aliases closed |


## Final disposition counts

| Final implementation disposition | Count |
| --- | --- |
| backendHoldbackSignedOff | 118 |
| canonicalExists | 163 |
| deprecatedLegacySignedOff | 126 |
| descriptorBacklogResolved | 51 |
| duplicateVariantSignedOff | 38 |
| graphRuntimeResolved | 87 |
| oracleOnlySignedOff | 3 |
| sceneRuntimeResolved | 16 |
| sourceBacklogResolved | 1 |


## Added canonical fixtures

| Legacy evidence path | Canonical path | Baseline disposition | Final disposition |
| --- | --- | --- | --- |
| styles/_DEPRECATED_style_fade_in.json | styles/style_fade_in.json | deprecatedLegacy | canonicalExists |
| styles/_DEPRECATED_style_fade_out.json | styles/style_fade_out.json | deprecatedLegacy | canonicalExists |
| styles/style_fade_in.json | styles/style_fade_in.json | descriptorBacklog | canonicalExists |
| styles/style_fade_out.json | styles/style_fade_out.json | descriptorBacklog | canonicalExists |


## Verification

Core gate results from this doc-closure pass:

- validate-recipe: 144/144 valid, 0 invalid.
- render-recipe: 144/144 rendered via the same canonical corpus count, 0 unsupported, 0 errors (the CLI emits the frame-shaped JSON report for this corpus in this working tree).
- render-frame: 144/144 rendered, 0 unsupported, 0 errors.
- fixture-qc: pass; 144 validated, 144 rendered, 0 unhandled fields, 0 unresolved adapter gaps, timeline smoke True, diff smoke True.
- primitive-field-coverage: 908/908 used fields handled; 0 used-but-unhandled; 0 missing descriptor fields.
- primitive-adapter-gap: 75/75 effects rendered; 0 unsupported; 0 missing descriptors.
- schema-readiness: canDeclareSchemaReady=true; explicitOwnerDecisionNeeded 0; fieldCoverageBlockedRecords 0; adapterBlockedRecords 0.
- implementation-readiness: implementationBlocking 0; explicitOwnerDecisionNeeded 0; generic implementation queues {}.
- control-catalog: 372 controls (16 source, 356 effect).
