<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_9.md</FILE> - <DESC>Architect response defining K2.10 corpus-wide migration mapping packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.10 corpus-wide migration mapping packet source.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add architect response for corpus-wide migration mapping and backlog board.</CLOG> -->

## ARCH-RESP-TO-PHASE_K2_9.md

### Review verdict

**ACCEPT.**

K2.9 landed the right next control surface: `migration-mapping-batch` now exists, masks have a first real per-family mapping report, and the simple mask expansion stayed disciplined. The important behavior is preserved:

```text
legacy debug_recipes/ = read-only evidence
canonical v3.1 fixtures = strict, descriptor-pack-backed artifacts
player adapters = honest evidence, not visual parity claims
```

The next phase should widen the migration loop across the remaining corpus, but stop before bulk migration. We need a broad, structured discovery pass that tells us which recipes are ready, which need descriptor work, which need schema work, and which need owner/architecture review.

---

# Phase K2.10 — Corpus-Wide Migration Mapping + Backlog Board

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
K2.9 migration-mapping-batch report + simple mask descriptor expansion
```

Current packet:

```text
K2.10 corpus-wide recursive migration mapping
K2.10 family-by-family migration backlog board
K2.10 descriptor/schema/player/GUI/backend gap classification
K2.10 no bulk recipe migration unless trivially descriptor-pack-backed and already supported
```

Coming next:

```text
K2.11 targeted descriptor/fixture expansion from the K2.10 backlog board
K3.0 GUI interaction/control surface over player evidence and future manifest data
Later: compositor-backed render adapter behind an explicit player/backend seam
```

---

## Executive goal

Turn `migration-mapping-batch` from a masks-focused report into a **corpus-wide planning authority** for the remaining legacy `debug_recipes/`.

The output should let the main orchestrator and architect answer:

```text
What can migrate now?
What needs descriptor additions?
What needs source descriptors?
What needs schema work?
What needs player adapters?
What needs GUI/human-review evidence?
What must stay oracle-only for now?
```

This packet should **not** attempt to migrate the whole corpus. It should create the structured, parallelizable loop that makes bulk migration safe later.

Core rule:

```text
Discover gaps. Do not hide gaps by forcing legacy recipes into today’s v3.1 shape.
```

---

# Work model: up to six parallel agents

The implementer is encouraged to use sub-agents. The lanes below are intentionally separable.

```text
                                   ┌─────────────────────────────┐
                                   │ A. report infrastructure    │
                                   │ recursive all-family mode   │
                                   └──────────────┬──────────────┘
                                                  │
       ┌──────────────────────────────────────────┼──────────────────────────────────────────┐
       │                                          │                                          │
┌──────▼───────┐                         ┌────────▼────────┐                         ┌───────▼───────┐
│ B. primitive │                         │ C. source/scene │                         │ D. timing /   │
│ families     │                         │ /content        │                         │ lifecycle     │
└──────┬───────┘                         └────────┬────────┘                         └───────┬───────┘
       │                                          │                                          │
       │                                          │                                          │
       │                                ┌─────────▼─────────┐                                │
       │                                │ E. complex /      │                                │
       │                                │ shadows / subcell │                                │
       │                                └─────────┬─────────┘                                │
       │                                          │                                          │
       └──────────────────────────────────────────┼──────────────────────────────────────────┘
                                                  │
                                   ┌──────────────▼──────────────┐
                                   │ F. backlog board, docs,     │
                                   │ GUI/backend boundary, QA    │
                                   └─────────────────────────────┘
```

Lanes B–E are read-only over legacy recipes. Lane A may touch report code. Lane F consolidates.

---

# Lane A — Generalize `migration-mapping-batch`

## Objective

Extend `migration-mapping-batch` so recursive/all-family mode produces useful records for the full legacy debug recipe corpus, not only masks.

Command:

```bash
RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

Family-specific mode must continue to work:

```bash
cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --family filters \
  --json
```

## Report schema

Keep:

```text
v3.1.player.migrationMappingBatch.1
```

Do not churn the schema unless necessary. Extend fields additively only.

The report should include:

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

Each record should retain the K2.9 fields:

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

If useful, add these fields additively:

```text
legacySignals[]
legacyBindings[]
legacySourceKinds[]
legacyEffectFamilies[]
candidateBlockers[]
confidence
```

## Required status vocabulary

Use the existing status set and make it meaningful across all families:

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

## Required recommendation vocabulary

Use:

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

## Hard requirement

Do not make records green by default.

Unknown families should not silently become `candidateReady`. Prefer:

```text
notYetClassified
descriptorDecisionNeeded
schemaDecisionNeeded
sourceDecisionNeeded
ownerAuditNeeded
```

as appropriate.

---

# Lane B — Primitive-family mapping agents

## Scope

Map these legacy families:

```text
filters
masks
samplers
styles
shaders/primitives
shaders/compositions
```

## Objective

Classify the remaining represented and adjacent primitive recipes into actionable buckets.

This lane should answer:

```text
1. Which recipes are already represented by canonical v3.1 fixtures?
2. Which recipes are simple descriptor/fixture expansions?
3. Which need only adapter/input-field handling?
4. Which need descriptor decisions?
5. Which are duplicate variants?
6. Which should remain oracle-only for now?
```

## Expected sub-agent split

### B1 — Filters

Review legacy filter recipes and classify by likely descriptor family.

Potential outcomes:

```text
candidateReady
descriptorDecisionNeeded
adapterDecisionNeeded
duplicateOrVariant
ownerAuditNeeded
```

Do not add new descriptors in this packet unless they are already in the primitive pack and fully handled.

### B2 — Samplers

Review sampler families beyond sine/ripple.

Pay special attention to:

```text
crt
fault-line
shredder
radial-twist
bounce / pendulum / gravity if present
```

Classify as descriptor/player/schema decisions rather than forcing them.

### B3 — Styles + shaders

Review style and shader primitive/composition recipes.

Separate:

```text
color/style-only primitives
role/scope pressure
binding-heavy variants
procedural or source-like shaders
composition/cross-family recipes
```

---

# Lane C — Source, content, and scene mapping agents

## Scope

Map:

```text
content
scene
fixtures
other source-like recipes
```

## Objective

Identify what source descriptors and scene/schema concepts are missing before these families can migrate.

This lane should classify evidence into:

```text
source.card already covered
new source descriptor needed
source-local pipeline needed
scene element/placement decision needed
asset/source resolver needed
content transform descriptor needed
template/profile/source-authoring issue
ownerAuditNeeded
```

## Required output

Produce a doc section that explicitly lists proposed source descriptor candidates, for example:

```text
source.text
source.ansi
source.image
source.procedural.<id>
source.marqueeText
source.commandCaptureArtifact
```

These are **candidates only** unless the lane finds a clear, bounded descriptor that should be proposed for K2.11.

Do not add command execution to runtime/player. Command capture remains offline authoring evidence only.

---

# Lane D — Timing, lifecycle, signal, and binding mapping agents

## Scope

Map:

```text
signals
easings
motion_routes
event_driven_dwell
loopback
bindable_rates
```

## Objective

Classify runtime/lifecycle/schema pressure before broad migration.

This lane must be especially conservative. These families are likely to affect data model discipline.

Classify into:

```text
lifecycleTriggerSupported
needsTriggerExtension
needsSignalValueSource
needsBindingExecutionSemantics
needsParameterContract
needsClockOrTimelineDecision
needsMotionDescriptor
needsLoopbackDemoLayer
ownerAuditNeeded
oracleOnly
```

## Guardrail

Do not introduce demo loopback semantics into canonical v3.1 runtime data.

The lane should preserve the vocabulary distinction:

```text
Trigger ≠ Gate
Trigger ≠ Binding
Trigger ≠ Loopback
Lifecycle trigger ≠ effect-local schedule
```

---

# Lane E — Complex, shadows, subcell, and cross-family mapping agents

## Scope

Map:

```text
complex
shadows
subcell_shapes
advanced compositions
```

## Objective

Identify which recipes are blocked by composition semantics, compositor backend, styled-cell limitations, source/scene gaps, or descriptor gaps.

Expected classifications:

```text
complexCompositionDecisionNeeded
shadowDescriptorNeeded
shadowBackendNeeded
subcellDescriptorNeeded
subcellRendererNeeded
compositorBackendCandidate
schemaDecisionNeeded
ownerAuditNeeded
oracleOnly
```

## Explicit compositor boundary

This lane may document future compositor needs, but must not wire the compositor.

The expected future shape remains:

```text
RecipeDocument v3.1
  -> contract validation
  -> player/runtime IR
  -> explicit render-backend adapter
  -> compositor-compatible IR / SemanticScene / CompositionSpec
  -> Ratatui display
```

Do not mutate `tui-vfx-contract` to fit compositor internals.

Do not make `tui-vfx-player-ui` construct compositor DTOs directly.

---

# Lane F — Backlog board, GUI/backend boundary, QA

## Objective

Consolidate the sub-agent findings into a top-level migration backlog board.

Create:

```text
docs/new_kernel/K2_10_DEBUG_RECIPE_CORPUS_MAPPING_REPORT.md
docs/new_kernel/K2_10_MIGRATION_BACKLOG_BOARD.md
docs/new_kernel/K2_10_RENDER_BACKEND_BOUNDARY_NOTE.md
docs/new_kernel/PHASE_K2_10_CORPUS_MAPPING_STATUS_MEMO_TO_ARCHITECT.md
```

## Backlog board shape

The backlog board should have sections:

```text
Ready now
Descriptor pack expansion candidates
Player adapter candidates
Source descriptor candidates
Schema/model decision candidates
GUI/human-review candidates
Compositor-backend candidates
Owner-audit / oracle-only
Duplicate variants
```

Each item should include:

```text
family
representativeLegacyPaths[]
currentCanonicalCoverage
recommendedNextPacket
blockingDecision
confidence
```

## GUI note

The Ratatui GUI should remain in scope as the human inspection surface, but K2.10 should not add large GUI features.

The GUI note should say:

```text
- tui-vfx-player-ui consumes tui-vfx-player evidence.
- It is useful for human inspection after fixture-qc passes.
- It should eventually display mapping/fixture-QC status, but not in K2.10 unless trivial.
- It must not depend on legacy recipe runtime.
- It must not directly depend on compositor internals.
```

## Backend boundary note

Write the render-backend boundary note now, because it prevents future confusion.

It should explicitly state:

```text
v3.1 DTOs are not compositor DTOs.
The compositor must remain stable.
A future backend adapter lowers validated/player runtime data into compositor-compatible IR.
The GUI should select/consume a player backend, not construct compositor internals itself.
Compositor-backed output is a future backend, not the current migration-mapping authority.
```

---

# Optional safe fixture additions

K2.10 is primarily a mapping/backlog packet.

However, the implementer may add a **small number of canonical fixtures** only if all of these are true:

```text
- the descriptor already exists in descriptors/v3.1/packs/primitive.json,
- the player adapter already handles every authored input,
- primitive-field-coverage remains zero-gap,
- fixture-qc remains pass,
- the fixture is descriptor-pack-backed, not embedded,
- no schema/model decision is required.
```

If any descriptor, schema, or adapter change is needed, record the fixture as a backlog item instead of migrating it in K2.10.

This prevents K2.10 from becoming uncontrolled bulk migration.

---

# TDD requirements

Start with RED tests for:

```text
migration-mapping-batch --recursive emits schemaVersion
migration-mapping-batch --recursive emits records for multiple families
migration-mapping-batch --family filters emits filter records
migration-mapping-batch --family content emits content/source-decision records
migration-mapping-batch does not classify unknown families as candidateReady
backlog board doc is generated/written by the packet, or at minimum checked into docs
```

Also add regression tests for:

```text
legacy debug_recipes root remains unmodified
existing masks-family report still works
fixture-qc still passes canonical corpus
validate/render/frame/field/adapters still pass canonical corpus
```

---

# Acceptance criteria

## Required

```text
- migration-mapping-batch --recursive works over the full legacy debug_recipes root.
- Report schema remains v3.1.player.migrationMappingBatch.1 unless a justified additive bump is documented.
- Report emits records for all known families, not just masks.
- Report summary includes per-status counts across the corpus.
- No legacy recipe files are modified.
- K2_10_DEBUG_RECIPE_CORPUS_MAPPING_REPORT.md exists.
- K2_10_MIGRATION_BACKLOG_BOARD.md exists.
- K2_10_RENDER_BACKEND_BOUNDARY_NOTE.md exists.
- Backlog board names next recommended family packets.
- validate-recipe passes for the canonical corpus.
- fixture-qc passes for the canonical corpus.
- existing K2.9 masks report remains stable.
```

## Preferred

```text
- The report identifies at least 3 high-confidence descriptor-pack expansion candidates.
- The report identifies at least 3 source descriptor candidates.
- The report identifies lifecycle/signal/binding blockers separately instead of lumping them into ownerAuditNeeded.
- Optional safe fixtures may be added only under the strict safe-fixture rules above.
```

## Explicit stop conditions

Stop and report rather than forcing implementation if:

```text
- recursive classification requires guessing old schema intent,
- a sub-agent proposes canonical aliases for legacy fields,
- a source family needs runtime command execution,
- a lifecycle family would blur trigger/gate/binding/loopback vocabulary,
- a complex family needs compositor semantics,
- a fixture would require unimplemented descriptor fields,
- field coverage can only pass by marking unimplemented fields as handled.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

Run:

```bash
cargo fmt \
  --package tui-vfx-player \
  --package tui-vfx-player-cli \
  --package tui-vfx-player-ui \
  --package tui-vfx-contract-cli \
  -- --check

cargo clippy \
  -p tui-vfx-player \
  -p tui-vfx-player-cli \
  -p tui-vfx-player-ui \
  -p tui-vfx-contract-cli \
  --all-targets -- -D warnings

cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test -p tui-vfx-player-ui
cargo test -p tui-vfx-contract-cli
cargo test --workspace
```

Report gates:

```bash
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --family masks \
  --json

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

Cleanliness:

```bash
git diff --check

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes

rg -n '"/usr/projects/tui-vfx-recipes|/usr/projects/tui-vfx-recipes' \
  crates/tui-vfx-player \
  crates/tui-vfx-player-cli \
  crates/tui-vfx-player-ui \
  crates/tui-vfx-contract-cli \
  docs/new_kernel
```

The legacy recipe root must remain untouched.

If canonical fixtures are added under `recipes/v3.1/debug_recipes`, the status memo must clearly list them and explain why they were safe under K2.10’s optional fixture rule.

---

# Expected status memo

Return:

```text
docs/new_kernel/PHASE_K2_10_CORPUS_MAPPING_STATUS_MEMO_TO_ARCHITECT.md
```

The memo must include:

```text
- rolling context
- executive summary
- sub-agent lane table
- recursive migration-mapping-batch command and schema
- total legacy records classified
- per-family status counts
- top recommended backlog items
- descriptor-pack expansion candidates
- source descriptor candidates
- schema/model decision candidates
- GUI/backend boundary summary
- optional fixture additions, if any
- verification matrix
- review and de-slop results
- recipe repo mutation status
- recommended next packet
```

The recommended next packet should be one of:

```text
K2.11 — targeted descriptor expansion from the backlog board
K2.11 — source/content descriptor pilot
K2.11 — lifecycle/signal/binding schema decision packet
K3.0 — GUI controls/manifest-facing interaction, if migration tooling is stable enough
```

Do not recommend compositor backend wiring yet unless the K2.10 backlog clearly shows that player/GUI validation is blocked on backend rendering rather than descriptor/schema/migration work.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_9.md</FILE> - <DESC>Architect response defining K2.10 corpus-wide migration mapping packet</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
