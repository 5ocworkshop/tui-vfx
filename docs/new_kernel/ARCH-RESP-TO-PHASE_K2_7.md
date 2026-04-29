## ARCH-RESP-TO-PHASE_K2_7_K2_8.md

### Review verdict

**ACCEPT_WITH_NOTES.**

K2.7/K2.8 lands the right boundary: the clean-room Ratatui UI is above `tui-vfx-player`, `fixture-qc` now gives a composed player gate, and the first represented-family migration batch added clean canonical variants without mutating legacy `debug_recipes/`.

The main note is that the migration workflow now needs to become a **machine-readable, repeatable batch process**, not only a human/doc workflow. The next packet should convert the migration mapping loop into automation and use it to drive one meaningful descriptor/adapter expansion batch.

---

# Phase K2.9 — Migration Mapping Report + Simple Mask Descriptor Expansion

## Rolling context to include in the implementer memo

Completed today:

```text
K2.1 migration-gap
K2.2 visual-frame report
K2.3 primitive adapter burn-down
K2.4 styled-cell substrate foundation
K2.5 styled primitive adapter burn-down
K2.6 GUI PRD, primitive field coverage, migration loop PRD, timeline/diff
K2.7/K2.8 Ratatui UI root polish, fixture-qc, first migration fixture batch
```

Current packet:

```text
K2.9 migration-mapping-batch report
K2.9 simple mask descriptor-design review
K2.9 simple mask canonical fixture/adapters where design is clear
```

Coming next:

```text
K2.10 larger parallel migration loop over remaining debug_recipes families
K3.0 GUI interaction/control surface over player evidence and future manifest data
Later: compositor-backed render adapter behind explicit player/backend seam
```

---

## Executive goal

Build the first **repeatable migration mapping control surface** and use it to move one bounded family forward.

The key discipline remains:

```text
The migration loop should discover schema / descriptor / player gaps.
It must not force every legacy recipe into today’s v3.1 shape.
```

K2.9 should therefore do two things:

1. Add a stable CLI/report surface that classifies legacy debug recipes into migration statuses and recommendations.
2. Use that report to drive a bounded simple-mask expansion only where descriptor semantics are clear.

The preferred expansion target is:

```text
mask.blinds
mask.radial
mask.iris
mask.diamond
```

If any of these cannot be mapped cleanly, do **not** fake support. Mark them with structured `descriptorDecisionNeeded`, `schemaDecisionNeeded`, `adapterDecisionNeeded`, or `ownerAuditNeeded`.

---

## Parallel work model

The implementer should use sub-agents. This packet is designed for up to six parallel lanes.

```text
                           ┌──────────────────────────────┐
                           │ A. migration report surface  │
                           └──────────────┬───────────────┘
                                          │
        ┌─────────────────────────────────┼─────────────────────────────────┐
        │                                 │                                 │
┌───────▼────────┐              ┌─────────▼─────────┐             ┌─────────▼─────────┐
│ B1 masks map   │              │ B2 radial/iris    │             │ B3 shader/style   │
│ blinds/wipe    │              │ diamond map       │             │ adjacent scan     │
└───────┬────────┘              └─────────┬─────────┘             └─────────┬─────────┘
        │                                 │                                 │
        └──────────────────────┬──────────┴─────────────────────────────────┘
                               │
                     ┌─────────▼─────────┐
                     │ C descriptor      │
                     │ decision report   │
                     └─────────┬─────────┘
                               │
              ┌────────────────┴────────────────┐
              │                                 │
      ┌───────▼────────┐              ┌─────────▼─────────┐
      │ D descriptors  │              │ E player adapters │
      │ + fixtures     │              │ + field coverage  │
      └───────┬────────┘              └─────────┬─────────┘
              │                                 │
              └────────────────┬────────────────┘
                               │
                     ┌─────────▼─────────┐
                     │ F QA, docs,       │
                     │ fixture-qc, memo  │
                     └───────────────────┘
```

Lane A can begin immediately. Lanes B1/B2/B3 are read-only and can run in parallel. Lanes D/E must wait for the descriptor decision report from Lane C.

---

# Lane A — `migration-mapping-batch` report surface

## Objective

Add a stable machine-readable report command that applies the migration-loop PRD to a selected legacy family or whole legacy root.

Command shape:

```bash
RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --family masks \
  --json
```

It should also support recursive/all-family mode:

```bash
cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

## Schema

Use:

```text
v3.1.player.migrationMappingBatch.1
```

Top-level fields:

```text
schemaVersion
legacyRoot
v31Root
descriptorPacks
families[]
summary
records[]
recommendationQueue[]
warnings[]
errors[]
```

Each `records[]` entry should include:

```text
legacyPath
legacyFamily
legacyRecipeName
candidateCanonicalPath
canonicalExists
status
recommendation
evidence
requiredDescriptorIds[]
missingDescriptorIds[]
requiredSourceIds[]
missingSourceIds[]
requiredInputFields[]
unsupportedInputFields[]
notes[]
```

Recommended `status` values:

```text
canonicalExists
candidateReady
descriptorDecisionNeeded
schemaDecisionNeeded
adapterDecisionNeeded
sourceDecisionNeeded
ownerAuditNeeded
blockedByUnsupportedSource
blockedByUnsupportedEffect
blockedByFieldCoverage
blockedByAmbiguousLegacyIntent
duplicateOrVariant
notYetClassified
```

Recommended `recommendation` values:

```text
createCanonicalFixture
extendDescriptorPack
addPlayerAdapter
addSourceDescriptor
addFieldHandling
deferForSchemaDecision
deferForDescriptorDecision
deferForOwnerAudit
skipAsDuplicateVariant
useAsOracleOnly
```

## Guardrails

This command is report-only. It must not generate or modify recipes.

It should prefer conservative classification. Unknown or ambiguous fields should become structured recommendations, not guessed descriptors.

---

# Lane B — Read-only migration mapping agents

## Objective

Run sub-agents over legacy recipe families and collect mapping evidence. Legacy recipes are read-only evidence.

Required inspected families for this packet:

```text
masks
```

Optional adjacent scans, if capacity is available:

```text
styles
shaders/primitives
shaders/compositions
```

The adjacent scans are for future queue planning only. Do not expand style/shader descriptors in this packet unless they are directly needed for the simple mask batch.

## Suggested sub-agent split

### B1 — Existing/wipe/checker/dissolve/blinds masks

Inspect legacy mask recipes that look close to already represented mask vocabulary.

Produce:

```text
- likely descriptor id
- required fields
- canonical fixture candidate
- adapter substrate class: textGrid / styledCell / semanticDecision
- confidence
- blockers
```

### B2 — radial / iris / diamond masks

Focus on geometry masks.

Produce:

```text
- whether these are one descriptor with shape enum or separate descriptors
- required center/radius/progress/feather/invert fields
- whether textGrid hiding is honest enough
- which fields are unsafe to add now
```

### B3 — adjacent shader/style scan

Read-only scan for likely collisions with simple mask descriptors.

Produce:

```text
- any legacy recipes that combine simple masks with styles/shaders
- whether new fixtures should include only mask primitives or simple combinations
- any immediate schema pressure discovered
```

## Required output doc

Create:

```text
docs/new_kernel/K2_9_SIMPLE_MASK_MIGRATION_MAPPING_EVIDENCE.md
```

This should be human-readable and grounded in the new `migration-mapping-batch` report.

---

# Lane C — Simple mask descriptor decision report

## Objective

Before changing the descriptor pack, write a decision report.

Create:

```text
docs/new_kernel/K2_9_SIMPLE_MASK_DESCRIPTOR_DECISION_REPORT.md
```

The report must explicitly answer:

```text
1. Which mask descriptors are accepted for K2.9?
2. Which legacy fields are intentionally represented?
3. Which legacy fields are deferred?
4. Which fields would be schema additions vs descriptor additions?
5. Which adapters can honestly render with textGrid/styledCell evidence?
6. Which candidate recipes are duplicates or variants?
7. Which recipes remain oracle-only for now?
```

## Descriptor discipline

Prefer narrowly correct descriptors over broad speculative descriptors.

Do not add inputs just because legacy recipes contain them unless:

```text
- the semantic meaning is clear,
- the player can report/handle the field honestly or field coverage can classify it honestly,
- the descriptor pack can document it without importing legacy naming as canonical vocabulary.
```

If a legacy field is not ready, classify it. Do not silently drop it.

---

# Lane D — Descriptor pack + canonical fixture expansion

## Objective

If Lane C accepts descriptor additions, update the primitive descriptor pack and add canonical fixtures.

Target preferred descriptors:

```text
mask.blinds
mask.radial
mask.iris
mask.diamond
```

Preferred fixture additions, if all four are accepted:

```text
../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_blinds.json
../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_radial.json
../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_iris.json
../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_diamond.json
```

All new fixtures must use pack-provided descriptors:

```json
"descriptorPacks": [{ "id": "v3.1.primitive" }],
"sourceDescriptors": {},
"graph": {
  "effects": {}
}
```

Do not embed standard primitive descriptors in new canonical fixtures.

## Expected count target

If all four fixtures land, the canonical corpus should move from:

```text
22 recipes
```

to:

```text
26 recipes
```

If fewer land, the status memo must explain exactly which descriptors/fixtures were deferred and why.

---

# Lane E — Player adapters + field coverage

## Objective

Add player support only for descriptors accepted in Lane C.

For each accepted mask descriptor:

```text
- add honest adapter support,
- include all authored input fields in primitive-field-coverage,
- ensure visual-frame output changes deterministically,
- ensure fixture-qc remains green.
```

For text-grid masks, hiding/revealing glyph cells is acceptable evidence.

If a descriptor requires color/alpha/feather semantics that cannot be honestly represented, either:

```text
- narrow the descriptor for K2.9,
- classify the field as deferred,
- or leave the descriptor unimplemented and report it as a blocker.
```

Do not fake soft/feathered visual parity through glyph-only evidence unless the report names it as approximation/degraded evidence.

## Field coverage requirement

For all new canonical fixtures:

```text
usedButUnhandledInputFields=0
missingDescriptorInputFields=0
schemaDecisionNeededFields=0
```

If a field is intentionally deferred, it must not be authored in a canonical fixture as though supported.

---

# Lane F — QA, docs, vocabulary, and GUI/compositor boundary

## Required docs

Update:

```text
docs/VOCABULARY.md
```

Add terms if new report concepts are introduced, likely:

```text
MigrationMappingBatchReport
MigrationMappingRecord
DescriptorDecisionReport
```

Add/update:

```text
docs/new_kernel/K2_9_SIMPLE_MASK_MIGRATION_MAPPING_EVIDENCE.md
docs/new_kernel/K2_9_SIMPLE_MASK_DESCRIPTOR_DECISION_REPORT.md
docs/new_kernel/PHASE_K2_9_MIGRATION_MAPPING_AND_SIMPLE_MASK_STATUS_MEMO_TO_ARCHITECT.md
```

## GUI boundary

Do not add compositor integration.

Do not make `tui-vfx-player-ui` construct compositor DTOs.

If the UI is touched at all, limit it to passive smoke support for the expanded recipe root. The UI remains above `tui-vfx-player`.

Future compositor-backed rendering must remain behind an explicit adapter/lowering layer:

```text
RecipeDocument v3.1
  -> contract validation
  -> player/runtime IR
  -> explicit render-backend adapter
  -> compositor-compatible IR / SemanticScene / CompositionSpec
  -> Ratatui display
```

---

# TDD and implementation discipline

Start with RED tests for:

```text
- migration-mapping-batch command unknown / then known
- report schemaVersion
- at least one masks-family record
- accepted descriptor fixture validates with descriptor pack
- accepted descriptor fixture renders
- field coverage sees all authored inputs handled
- fixture-qc remains pass
```

Do not implement first and backfill tests.

If sub-agents produce recommendations that conflict, the main agent should not choose silently. It should record the conflict in the descriptor decision report and defer the descriptor unless there is a clear evidence-backed resolution.

---

# Acceptance criteria

## Required

```text
- migration-mapping-batch command exists.
- It emits schemaVersion=v3.1.player.migrationMappingBatch.1.
- It supports at least --family masks and --recursive.
- It emits per-recipe records with status and recommendation.
- It does not modify legacy recipes.
- Simple mask descriptor decision report exists.
- New descriptors/fixtures are added only for accepted decisions.
- New standard fixtures use descriptor packs, not embedded descriptors.
- validate-recipe passes for the full canonical corpus.
- render-recipe passes for the full canonical corpus.
- render-frame passes for the full canonical corpus.
- primitive-field-coverage has no used-but-unhandled fields.
- primitive-adapter-gap has no unresolved adapters for accepted descriptors.
- fixture-qc overallStatus=pass.
```

## Preferred

```text
- mask.blinds, mask.radial, mask.iris, and mask.diamond all land.
- Canonical corpus increases from 22 to 26.
- migration-mapping-batch identifies additional future mask/style/shader candidates without implementing them.
```

## Explicit stop conditions

Stop and report rather than forcing implementation if:

```text
- a legacy mask family requires new scope algebra,
- a descriptor would need unclear canonical vocabulary,
- player evidence would need to pretend visual parity,
- field coverage can only be made green by marking fields handled without semantics,
- canonical fixtures would need legacy aliases,
- a GUI/compositor dependency is introduced accidentally.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

Run at minimum:

```bash
cargo fmt --package tui-vfx-player --package tui-vfx-player-cli --package tui-vfx-player-ui --package tui-vfx-contract-cli -- --check

cargo clippy -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-player-ui -p tui-vfx-contract-cli --all-targets -- -D warnings

cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test -p tui-vfx-player-ui
cargo test -p tui-vfx-contract-cli
cargo test --workspace

cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --family masks \
  --json

cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

git diff --check

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes
```

Also check no hard-coded local checkout paths were introduced:

```bash
rg -n '"/usr/projects/tui-vfx-recipes|/usr/projects/tui-vfx-recipes' \
  crates/tui-vfx-player \
  crates/tui-vfx-player-cli \
  crates/tui-vfx-player-ui \
  crates/tui-vfx-contract-cli \
  docs/new_kernel
```

The legacy recipe root must remain untouched.

---

# Expected status memo

Return:

```text
docs/new_kernel/PHASE_K2_9_MIGRATION_MAPPING_AND_SIMPLE_MASK_STATUS_MEMO_TO_ARCHITECT.md
```

It must include:

```text
- rolling context
- migration-mapping-batch schema and command shape
- descriptor decision summary
- accepted descriptors
- deferred descriptors/fields with reasons
- new canonical fixture paths
- count changes across validate/render/inventory/field/adapters/fixture-qc
- sub-agent summary table
- verification matrix
- review and de-slop results
- recipe repo mutation status
- recommended next packet
```

The recommended next packet should be one of:

```text
K2.10 — broader parallel migration mapping over remaining debug_recipes
K2.10 — next descriptor expansion family if K2.9 exposes a clear target
K3.0 — GUI controls/manifest-facing interaction if migration tooling is stable enough
```

Do not recommend compositor backend wiring yet unless the GUI/player evidence and migration loop have a clear need for it.
