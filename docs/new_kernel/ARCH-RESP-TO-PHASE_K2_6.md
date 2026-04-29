# ARCH-RESP-TO-PHASE_K2_6.md

## Review verdict

**ACCEPT_WITH_NOTES.**

K2.6 is accepted as the point where the K2 player/tooling stack becomes useful enough to support both human-facing GUI work and structured migration-loop work.

The important result is not only that the current canonical primitive corpus renders. The important result is that we now have enough **contract-native evidence surfaces** to avoid guessing:

```text
validate-recipe
render-recipe
render-frame
render-timeline
render-frame-diff
inventory-recipes
primitive-adapter-gap
primitive-field-coverage
migration-gap
```

The next packet should use those surfaces aggressively, while preserving the core rule:

```text
Migration discovers schema / descriptor / player gaps.
Migration does not force legacy recipes into today’s v3.1 shape.
```

## Rolling context to include in every future packet

Completed today:

```text
- K2.1 migration-gap
- K2.2 visual-frame report
- K2.3 primitive adapter burn-down
- K2.4 styled-cell substrate foundation
- K2.5 styled primitive adapter burn-down
- K2.6 GUI PRD, primitive field coverage, migration loop PRD, timeline/diff
```

Current packet:

```text
- K2.7/K2.8 combined boundary packet:
  Ratatui GUI player skeleton + first parallel debug_recipes migration mapping batch
```

Coming after this packet:

```text
- Broader debug_recipes migration loop, once the first batch proves the process
- GUI/player interaction expansion
- Fixture QC / trace / focused cell analysis
- Compositor wiring investigation for final human playback review
- Non-debug recipe migration after debug_recipes is structurally understood
```

---

# Next assignment

## Phase K2.7/K2.8 — Ratatui GUI Player Skeleton + Parallel Migration Mapping Batch

This is a **multi-lane packet**. Use sub-agents where useful. The lanes are intentionally parallel where they do not collide, but they must stop at clear QA boundaries.

This packet has two major goals:

```text
1. Build the first clean-room Ratatui GUI player skeleton over tui-vfx-player.
2. Run the first structured, parallel migration mapping batch over legacy debug_recipes.
```

The GUI lane gives humans a v3.1 playback surface soon.
The migration lane starts the recipe-by-recipe loop that discovers whether v3.1, descriptors, player adapters, or migration rules need to grow.

Do not let either lane mutate core schema or descriptor semantics casually. Any discovered gap must be reported with a recommendation and reviewed before being treated as canonical.

---

# Non-negotiable architecture rules

## Clean-room authority

The authoritative clean-room tools remain:

```text
tui-vfx-contract-cli     structural validation
tui-vfx-player-cli       canonical player/render/inventory/migration evidence
tui-vfx-player           contract-native player library
```

The Ratatui GUI may call into `tui-vfx-player`. It must not become a new validator.

## Legacy tooling and demo code are oracle inspiration only

Inspect and use this file as UX inspiration:

```text
../tui-vfx-recipes/examples/demo.rs
```

Borrow ideas from it:

```text
- browser / preview layout
- keyboard workflow
- help modal
- status strip
- reload behavior
- pause / resume
- motion-disabled or frozen-sample mode
- phase/sample scrubbing
- render-hash diagnostics
- canvas/substrate concept
```

Do **not** borrow these from it:

```text
- legacy recipe loading authority
- legacy fallback behavior
- old runtime dependencies
- old schema semantics
- direct dependency on tui-vfx-recipes runtime
- hard-coded effect inspection based on old recipe internals
```

## Migration discipline

For migration work:

```text
Legacy source recipes are evidence only.
Legacy debug_recipes must remain read-only.
Canonical v3.1 fixtures may be added only when the mapping is clean.
Unclean mappings become structured recommendations, not forced schema hacks.
```

The goal is to discover:

```text
- clean mappings
- descriptor gaps
- descriptor-input gaps
- source descriptor gaps
- player adapter gaps
- lifecycle/signal/scene semantic gaps
- cases that need architect decision
```

The goal is **not** to make every old file validate by adding aliases or compatibility shortcuts.

---

# Parallel work DAG

Use this as the implementation DAG. Lanes may run concurrently when their dependencies are satisfied.

```text
                                  +-----------------------------+
                                  | A. Context + fixture baseline |
                                  +--------------+--------------+
                                                 |
             +-----------------------------------+-----------------------------------+
             |                                   |                                   |
             v                                   v                                   v
+-----------------------------+     +-----------------------------+     +-----------------------------+
| B. Ratatui GUI skeleton     |     | C. Fixture QC/report runner |     | D. Migration batch planning |
| new clean GUI crate/binary  |     | compose existing reports    |     | choose bounded family slices |
+--------------+--------------+     +--------------+--------------+     +--------------+--------------+
               |                                   |                                   |
               |                                   |                                   v
               |                                   |                    +-----------------------------+
               |                                   |                    | E1/E2/E3. Parallel family   |
               |                                   |                    | migration mapping agents    |
               |                                   |                    +--------------+--------------+
               |                                   |                                   |
               +-------------------+---------------+-----------------------------------+
                                   |
                                   v
                         +-----------------------------+
                         | F. Consolidation + QA gate  |
                         | docs, status memo, reports  |
                         +-----------------------------+
```

Recommended sub-agent split:

```text
Agent 1: Ratatui GUI player skeleton
Agent 2: GUI tests, usage docs, UX review against demo.rs
Agent 3: Fixture-QC/report orchestration command
Agent 4: Migration batch — filters + masks
Agent 5: Migration batch — samplers + styles
Agent 6: Migration batch — shaders/primitives + shaders/compositions, plus consolidation support
```

If an agent finds a schema or descriptor issue, it must record it as a structured gap. It must not silently mutate the schema to make the batch pass.

---

# Lane A — Baseline and context refresh

Before implementation, establish the current baseline.

Run and capture current summaries against the canonical v3.1 debug corpus:

```bash
RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- migration-gap \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json
```

Expected starting posture from K2.6:

```text
canonical v3.1 corpus: 16 recipes
rendered: 16
unsupported: 0
primitive field coverage: 0 unhandled used fields
primitive adapter gap: 0 blockers for represented primitives
legacy debug_recipes: 603
```

Capture report artifacts under `${TMPDIR:-/tmp}` using K2.7/K2.8 names.

---

# Lane B — Ratatui GUI player skeleton

## Goal

Create the first **clean-room Ratatui GUI player** over canonical v3.1 recipes and `tui-vfx-player`.

This is not the final studio. It is the first human-facing playback surface for v3.1 player evidence.

## Preferred crate shape

Create a separate binary crate unless there is a strong implementation reason not to:

```text
crates/tui-vfx-player-tui
```

Expected dependency direction:

```text
tui-vfx-player-tui
  -> tui-vfx-player
  -> tui-vfx-contract
```

Allowed UI/runtime dependencies:

```text
ratatui
crossterm
serde
serde_json
```

Forbidden dependencies:

```text
tui-vfx-recipes runtime
legacy preview manager
legacy recipe fallback loader
old compositor/style/content/shadow crates unless already cleanly used through tui-vfx-player abstractions
```

## Command shape

Minimum command:

```bash
cargo run -p tui-vfx-player-tui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

Also support a direct startup recipe:

```bash
cargo run -p tui-vfx-player-tui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json"
```

Use portable paths. Honor `RECIPE_REPO` in tests where needed.

## Minimum UI

Implement a two-pane Ratatui layout inspired by `../tui-vfx-recipes/examples/demo.rs`:

```text
left:  canonical recipe browser/list
right: recipe/player info + frame preview
```

Minimum displayed information:

```text
recipe path
validation/player status
phase
sampleT
loopT if available
renderHash
nonEmptyCells
substrate
cellSource
styleKnown
descriptor pack ids
warnings/errors
```

Minimum preview behavior:

```text
- Render current visual-frame rows into the preview area.
- Preserve compact text rows as the first display substrate.
- If styled-cell evidence is present, surface styleKnown/substrate status in metadata.
- Do not claim visual parity.
```

## Minimum key bindings

Borrow the workflow spirit from `examples/demo.rs`, but keep the implementation clean:

```text
q        quit
?        help modal
Tab      switch focus if needed
j/k      move browser selection
Enter    load selected canonical recipe
r        reload active recipe from disk
Space    pause/resume timeline if timeline playback is implemented
m        freeze/motion-disabled mode: show stable dwell/sample frame
Left     decrease sampleT
Right    increase sampleT
[        previous phase
]        next phase
c        close active preview
```

If lifecycle trigger support is not implemented in `tui-vfx-player`, do not fake it. You may reserve `t` in help as future trigger support and display a clear “not yet supported” message.

## Minimum tests

Add headless tests for:

```text
- CLI option parsing
- startup recipe path resolution
- descriptor-pack path handling
- recipe-root list loading
- phase/sample state transitions
- help text includes core keys
- no hard-coded /usr/projects paths in tests
```

A full terminal integration test is not required in this packet.

## GUI acceptance criteria

```text
- New Ratatui GUI starts against canonical v3.1 recipe root.
- A canonical recipe can be selected or preloaded.
- Current frame rows are visible.
- Metadata panel shows render hash and visual-frame provenance.
- Phase/sample controls update the displayed frame.
- Reload re-reads recipe JSON.
- No legacy runtime dependency is introduced.
- GUI is clearly documented as player evidence, not visual parity.
```

---

# Lane C — Fixture QC / composed evidence command

## Goal

Add a higher-level clean-room corpus command that composes existing reports into a single QC summary.

This is inspired by legacy `pipeline-validator --debug-recipes-qc` and `tui-vfx-horseman`, but it must remain clean-room and v3.1-only.

## Command shape

Preferred command:

```bash
cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

Schema:

```text
v3.1.player.fixtureQcReport.1
```

## Report purpose

The report should answer, in one artifact:

```text
Can the canonical fixture corpus validate, inventory, render, produce visual frames,
cover primitive fields, and avoid unsupported primitive adapters?
```

## Report shape

Top-level fields:

```text
schemaVersion
root
descriptorPacks
summary
reports
recipes[]
warnings[]
errors[]
```

Summary should include at least:

```text
totalRecipes
validated
validationErrors
rendered
unsupported
playerErrors
visualFrames
fieldCoverageUnhandled
adapterGapUnresolved
timelineSmokePassed
diffSmokePassed
overallStatus
```

`overallStatus` should be one of:

```text
pass
warn
fail
```

## Implementation guidance

Do not shell out to existing commands. Reuse library functions where practical.

Do not duplicate the structural validator’s semantics. If invoking validation internally is too invasive for this packet, the first QC command may be player-side only, but it must clearly state what it did and did not check.

## QC acceptance criteria

```text
- fixture-qc emits schema-labeled JSON.
- It aggregates existing player evidence without changing existing report schemas.
- It fails or warns when a recipe is unsupported, errors, has unhandled primitive fields, or unresolved adapter gaps.
- Existing commands continue to produce unchanged schemas.
```

---

# Lane D — Migration mapping batch planning

## Goal

Turn the K2.6 migration-loop PRD into the first real batch run over legacy debug recipes.

This lane should not try to migrate all 603 recipes. It should prove the process on a meaningful parallel slice.

## Target batch

Start with families that are already represented in canonical v3.1 and already have player evidence:

```text
filters
masks
samplers
styles
shaders/primitives
shaders/compositions
```

These families are large enough to test the process, but close enough to current descriptor/player support to produce useful clean mappings.

Do **not** start with the hardest deferred families in this packet:

```text
content
scene
shadows
complex
signals
easings
motion_routes
loopback
bindable_rates
subcell_shapes
```

Those should be classified at a high level only if encountered, not forced.

## Batch outputs

Create a migration batch report:

```text
docs/new_kernel/K2_8_DEBUG_RECIPE_MIGRATION_BATCH_REPORT.md
```

Optionally also create a machine-readable transient JSON report under `${TMPDIR:-/tmp}`:

```text
${TMPDIR:-/tmp}/tui-vfx-k28-migration-batch-report.json
```

If the implementation adds a stable CLI report for this, use:

```text
v3.1.player.migrationMappingBatch.1
```

But a stable CLI is not mandatory if it would slow the batch. The important thing is the structured classification and recommendations.

## Required per-recipe classification

Every inspected legacy recipe should receive one of these statuses:

```text
canonicalizedCleanly
candidateCleanMapping
needsDescriptorInput
needsDescriptor
needsSourceDescriptor
needsPlayerAdapter
needsSchemaDecision
needsLifecycleSignalDecision
needsSceneDecision
needsHumanSemanticReview
deferAdvancedFamily
doNotMigrateYet
```

Every non-clean status must include a recommendation:

```text
addDescriptorInput
addDescriptor
addSourceDescriptor
addPlayerAdapter
addMigrationRule
addSchemaCapability
deferForLifecycleWork
deferForSceneWork
deferForSourceWork
manualReview
rejectLegacyOnly
```

## Required gap fields

Each record should include:

```text
legacyPath
family
legacyObservedFeatures[]
nearestCanonicalDescriptorIds[]
proposedCanonicalShape
status
recommendation
reason
confidence
wouldRequireSchemaChange
wouldRequireDescriptorChange
wouldRequirePlayerChange
wouldRequireSourceChange
wouldRequireLifecycleDecision
wouldRequireSceneDecision
notes
```

Use `confidence` values:

```text
high
medium
low
```

## Canonical fixture creation policy

Canonical fixtures may be added only when all of these are true:

```text
- The old recipe intent maps cleanly to existing v3.1 concepts.
- The descriptor pack already supports the needed descriptor id, or only needs a narrowly justified input addition.
- The player can render or honestly classify the result.
- No new schema concept is required.
- The fixture validates through tui-vfx-contract-cli.
```

Do not add canonical fixtures for recipes that need semantic decisions.

Suggested cap for this packet:

```text
minimum: 6 new canonical fixtures if clean candidates exist
target: 12-18 new canonical fixtures
hard cap: 24 new canonical fixtures
```

Preserve family paths under:

```text
$RECIPE_REPO/recipes/v3.1/debug_recipes/
```

Legacy recipes under:

```text
$RECIPE_REPO/recipes/debug_recipes/
```

must remain untouched.

---

# Lane E — Parallel family mapping agents

Run family mapping in parallel. Suggested split:

```text
Agent E1: filters + masks
Agent E2: samplers + styles
Agent E3: shaders/primitives + shaders/compositions
```

Each agent should produce a short family report with:

```text
family
legacy count inspected
clean candidates
canonical fixtures proposed/created
blocked cases by blocker type
descriptor input gaps
descriptor gaps
player gaps
schema/semantic gaps
recommended next action
```

## E1 — filters + masks

Focus questions:

```text
- Which legacy filter ids map to existing v3.1 descriptors?
- Which filter fields are already handled by player adapters?
- Which filter fields need descriptor-input additions?
- Which mask families map cleanly to none/wipe/checkers/dissolve?
- Which masks require new mask descriptor decisions?
```

Allowed clean fixture additions:

```text
filters using dim/tint/invert/greyscale with existing or narrowly handled fields
masks using none/wipe/checkers/dissolve
```

Do not add new filter/mask families without a recommendation record.

## E2 — samplers + styles

Focus questions:

```text
- Which sampler recipes map to sineWave/ripple?
- Which sampler fields are missing from descriptor/player handling?
- Which style recipes map to colorFade/baseStyleOverride?
- Which style recipes actually require role semantics not yet modeled?
```

Allowed clean fixture additions:

```text
sampler.sineWave
sampler.ripple
style.colorFade
style.baseStyleOverride
```

Do not fake role semantics. If a recipe needs a role or source role that does not exist, classify it.

## E3 — shaders/primitives + shaders/compositions

Focus questions:

```text
- Which shader primitive recipes map to linearGradient or borderSweep?
- Which shader recipes require new shader descriptors?
- Which compositions are simple graph combinations of existing descriptors?
- Which compositions cross into complex/multi-effect semantics that need separate review?
```

Allowed clean fixture additions:

```text
shader.linearGradient
shader.borderSweep
simple compositions of already-supported primitive descriptors
```

Do not add complex recipes just to improve counts. Complex recipes should remain recommendations unless they truly map cleanly.

---

# Lane F — Consolidation, QA, and status memo

## Required docs

Create:

```text
docs/new_kernel/PHASE_K2_7_K2_8_GUI_AND_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md
```

If the GUI and migration work become too large, separate memos are acceptable:

```text
docs/new_kernel/PHASE_K2_7_RATATUI_GUI_PLAYER_STATUS_MEMO_TO_ARCHITECT.md
docs/new_kernel/PHASE_K2_8_MIGRATION_BATCH_STATUS_MEMO_TO_ARCHITECT.md
```

But the final response should still include a unified rolling context.

## Status memo must include

```text
- Rolling context: completed / current / coming next
- GUI implementation summary
- GUI UX borrowed from demo.rs
- GUI clean-room boundary confirmation
- Migration batch scope
- Families inspected
- Canonical fixtures added, if any
- Recipes classified but not migrated
- Schema/descriptor/player/source/lifecycle/scene gaps discovered
- Report counter changes before/after
- Cross-repo changes, especially recipe fixture additions
- Verification matrix
- Review and de-slop results
- Recommended next packet
```

## Required verification

Run:

```bash
cargo fmt --package tui-vfx-player --package tui-vfx-player-cli -- --check
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
```

If a new GUI crate is added:

```bash
cargo fmt --package tui-vfx-player-tui -- --check
cargo clippy -p tui-vfx-player-tui --all-targets -- -D warnings
cargo test -p tui-vfx-player-tui
```

Run workspace verification:

```bash
cargo test --workspace
git diff --check
```

Run recipe cleanliness checks:

```bash
git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes
```

Expected:

```text
no output
```

For canonical fixture changes, record them explicitly:

```bash
git -C "$RECIPE_REPO" status --short -- recipes/v3.1/debug_recipes
```

Run full canonical corpus evidence after any fixture additions:

```bash
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

If `fixture-qc` is added:

```bash
cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

## Path portability check

Run:

```bash
rg -n '"/usr/projects/tui-vfx-recipes' \
  crates/tui-vfx-player* \
  crates/tui-vfx-player-cli \
  docs/new_kernel
```

Hard-coded absolute recipe repo paths are acceptable only in status memos when explicitly recording observed local evidence. They are not acceptable in tests or reusable commands.

---

# Expected outcomes

By the end of this packet, I expect:

```text
1. A basic Ratatui GUI player exists and can load/render canonical v3.1 recipes.
2. The GUI is explicitly clean-room and does not depend on legacy recipe runtime code.
3. A fixture-QC style report either exists or is clearly deferred with rationale.
4. The first migration mapping batch has been run over represented primitive families.
5. Several clean canonical fixtures may be added, but only when mappings are genuinely clean.
6. Non-clean legacy recipes are classified with structured blockers and recommendations.
7. Any proposed schema/descriptor/player changes are surfaced for review instead of bolted on.
```

The key success condition is not the number of migrated files. The key success condition is that we can now run this loop repeatedly:

```text
inspect legacy evidence
-> propose canonical mapping
-> validate canonical shape
-> render player evidence
-> run timeline/diff/field coverage
-> classify gaps
-> recommend schema/descriptor/player changes
```

That is the process that will get us from a small canonical fixture set to full debug recipe coverage without losing the discipline of the data model.


Yes, include it **now**, but as a **boundary/guardrail**, not as a K2.7 implementation requirement. We are not yet at the point of wiring the GUI to the compositor, but the GUI skeleton must be built so it does not paint us into the wrong architecture.

Paste this addendum into the latest work-packet prompt:

````md
## Addendum — Future compositor wiring boundary

The Ratatui GUI player work must preserve a clean boundary between canonical v3.1 data, player/runtime evidence, GUI presentation, and the existing compositor stack.

Do **not** make the GUI construct compositor DTOs directly.

Do **not** reshape `RecipeDocument` / v3.1 contract DTOs around the compositor’s internal DTOs.

Do **not** modify or break the existing compositor as part of this packet.

The intended future architecture is:

```text
RecipeDocument v3.1
  -> validate through tui-vfx-contract / descriptor catalog
  -> compile or normalize into v3.1 player/runtime IR
  -> lower through an explicit render-backend adapter
  -> existing compositor-compatible IR / CompositionSpec / SemanticScene / buffer output
  -> Ratatui GUI display
````

The GUI should consume player-facing state, frame evidence, diagnostics, controls, lifecycle state, and future manifest data. It should not own lowering to compositor internals.

For this packet, treat compositor wiring as a **future backend lane**. The current GUI skeleton should be backend-ready by depending on clean player abstractions rather than on legacy recipe runtime, V2/V3 preview fallback paths, or compositor internals.

Required architectural notes for the packet:

* `tui-vfx-contract` remains pure DTO/schema/validation and must not depend on compositor crates.
* `tui-vfx-player` remains the contract-native player/evidence layer.
* The Ratatui GUI should sit above `tui-vfx-player`, not beside or beneath it.
* Any future compositor-backed renderer should be an explicit adapter/lowering layer, not an implicit dependency hidden in the GUI.
* Existing compositor behavior is an oracle/backend target, not the owner of v3.1 semantics.
* Unsupported or degraded lowering must eventually report structured diagnostics rather than silently approximating behavior.

Non-goals for this packet:

* No compositor integration.
* No compositor API changes.
* No visual parity claim.
* No legacy fallback authority.
* No direct GUI-to-compositor DTO construction.

The implementation should leave a clear seam where a future backend can replace or augment the current player visual-frame substrate with compositor-backed frame evidence.
```

