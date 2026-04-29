# ARCH-RESP-TO-PHASE_K2_5.md

## Decision

**K2.5 is accepted for planning purposes.** It closes the represented primitive adapter blocker set for the current canonical v3.1 fixture corpus:

```text
rendered=16
unsupported=0
adapter gaps=0
```

The next packet should widen the work unit. We are spending too much review overhead on narrow changes. K2.6 should run multiple non-colliding lanes in parallel and produce both product-direction artifacts and concrete tooling/reporting improvements.

---

# Phase K2.6 Work Packet — GUI Player PRD + Primitive Field Coverage + Migration Mapping Loop

## Rolling context

Completed today:

```text
- K2.1 migration-gap report
- K2.2 visual-frame report
- K2.3 text-grid primitive adapter burn-down
- K2.4 styled-cell substrate foundation
- K2.5 styled/color/role primitive adapter burn-down
```

Current packet:

```text
- K2.6 GUI player PRD and tooling/migration expansion
```

Coming after this packet:

```text
- K2.7 Ratatui GUI player skeleton / first human-facing v3.1 player
- K2.8 iterative recipe migration loop over larger debug-recipe families
- Later: trace, SQLite, QC gates, oracle comparison, visual parity workflow
```

---

## Executive goal

K2.6 should move us from “the current 16 canonical recipes can render through the skeleton player” to “we know how to scale this into a human-facing player and a repeatable migration workflow.”

The packet has three goals:

```text
1. Define the v3.1 Ratatui GUI Player PRD.
2. Add field-level coverage tooling for primitives/capabilities already represented.
3. Define and seed the iterative recipe-by-recipe migration mapping loop.
```

This work is additive. It does **not** replace the established K0/K2 CLI capability. The GUI player sits on top of the CLI/player evidence model and must reuse the same contract-native authority.

---

## Critical framing

The current CLI player is already useful and must remain the regression authority:

```text
render-recipe
render-frame
inventory-recipes
migration-gap
primitive-adapter-gap
```

The Ratatui GUI player is the human-facing layer we need for actual end-to-end review, recipe tuning, visual debugging, and eventual studio work.

The existing legacy GUI at:

```text
../tui-vfx-recipes/examples/demo.rs
```

must be inspected as **oracle inspiration** for UX, layout, controls, reload behavior, help overlays, preview ergonomics, and human workflow. It is not exhaustive and should not be source-ported into the clean-room player, but it is currently the main human interface point for playing legacy recipes and therefore must inform the PRD.

---

# K2.6 Scope

## Lane A — Ratatui GUI Player PRD

Create:

```text
docs/new_kernel/K2_6_RATATUI_GUI_PLAYER_PRD.md
```

The PRD must cover a future Ratatui GUI player built on top of `tui-vfx-player`, not on top of legacy runtime internals.

It must explicitly inspect and cite lessons from:

```text
../tui-vfx-recipes/examples/demo.rs
```

Required PRD sections:

```text
1. Product purpose
2. Non-goals
3. Runtime authority boundary
4. Relationship to existing CLI player
5. Required screens/panes
6. Required controls
7. Recipe-driven UI generation
8. Descriptor/parameter/signal/control model
9. Playback lifecycle controls
10. Debug/evidence panels
11. Migration/parity workflow support
12. Accessibility / keyboard model
13. Expected JSON/report dependencies
14. MVP scope
15. Future studio path
```

### GUI player requirements

The PRD must state that the GUI player:

```text
- Uses ratatui.
- Consumes canonical v3.1 RecipeDocument values and descriptor packs.
- Uses tui-vfx-player frame evidence as the render/evidence substrate.
- Does not infer effect behavior from raw legacy recipes.
- Does not replace CLI commands.
- Can browse, load, reload, preview, pause, scrub, and inspect v3.1 recipes.
- Shows unsupported/migration/adapter diagnostics inline.
- Is designed to evolve toward studio controls.
```

### Required human-facing capabilities

At minimum, capture these as PRD requirements:

```text
- Recipe browser / file picker
- Preview pane
- Recipe metadata pane
- Descriptor/effect/node inventory pane
- Phase controls: enter / dwell / exit
- sampleT scrub
- frame timeline controls
- reload-from-disk
- motion-disabled/freeze mode
- render hash and non-empty cell display
- unsupported/evidence warning display
- descriptor-pack provenance display
- parameter/signal/control list
- future editable controls generated from parameters/signals/descriptors
```

### Explicit `demo.rs` inspiration points

The PRD should record what to borrow conceptually from the legacy demo:

```text
- Browser/preview split layout
- Keyboard-first workflow
- Help modal
- Status strip
- Reload active recipe from disk
- Pause / resume
- Motion-disabled mode
- Phase/sample scrubbing
- Event trigger controls
- Render hash / diagnostics display
- Canvas substrate concept
```

And what not to adopt directly:

```text
- Legacy recipe loading authority
- Legacy fallback paths
- Direct dependency on tui-vfx-recipes runtime crates
- Old schema semantics
- Hard-coded legacy effect inspection
```

---

## Lane B — Primitive Field Coverage Report

Add a field-level coverage report for canonical v3.1 recipes and descriptor packs.

Suggested command:

```bash
cargo run -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

Suggested schema:

```text
v3.1.player.primitiveFieldCoverage.1
```

Top-level report fields:

```text
schemaVersion
root
descriptorPacks
summary
recipes[]
```

Summary fields:

```text
totalRecipes
totalPrimitiveInstances
usedInputFields
handledInputFields
usedButUnhandledInputFields
declaredButUnusedInputFields
missingDescriptorInputFields
schemaDecisionNeededFields
```

Per recipe entry:

```text
recipePath
status
primitiveInstances[]
errors[]
warnings[]
```

Per primitive instance:

```text
kind                  // source | effect
descriptorId
nodeId
sourceInstanceId
domain
usedInputs[]
descriptorInputs[]
adapterHandledInputs[]
usedButUnhandledInputs[]
declaredButUnusedInputs[]
missingDescriptorInputs[]
classification
recommendation
```

### Classification vocabulary

Use stable strings:

```text
usedAndHandled
usedButUnhandled
declaredButUnused
missingDescriptorInput
schemaDecisionNeeded
adapterDecisionNeeded
```

### Acceptance criteria

For the current 16 canonical v3.1 debug fixtures:

```text
usedButUnhandledInputFields=0
missingDescriptorInputFields=0
schemaDecisionNeededFields=0
```

It is acceptable for:

```text
declaredButUnusedInputFields > 0
```

as long as the report names them honestly.

This report is important because `rendered=16` is not enough. We also need to know whether the fields authored in the canonical recipes are actually being consumed by the player adapters.

---

## Lane C — Batch remaining primitive field handling

Using the new field-coverage report, batch out remaining field handling for primitives/capabilities already represented in the canonical fixtures.

Scope is **only** the primitive ids already in the current descriptor pack and canonical fixtures:

```text
source.card
filter.dim
filter.tint
filter.invert
filter.greyscale
mask.none
mask.wipe
mask.checkers
mask.dissolve
sampler.sineWave
sampler.ripple
style.colorFade
style.baseStyleOverride
shader.linearGradient
shader.borderSweep
```

The goal is not to implement every theoretical field in the descriptor pack. The goal is:

```text
Every input field actually used by the 16 canonical fixtures is either handled or explicitly classified.
```

Do not silently ignore a used input. If an input cannot be represented yet, report it as a blocker and explain why.

### Expected field-handling rules

The player should handle, or explicitly report, fields in these categories:

```text
- text/source fields for source.card
- foreground/background color fields
- strength/intensity fields
- applyTo fields
- scope/role hints used by current fixtures
- mask progress/seed/threshold fields
- sampler amplitude/frequency/wavelength/center/speed fields when present
- shader color endpoints/direction fields used by fixtures
- style target color / base style fields used by fixtures
```

### Hard rule

Do not fake support. If a field affects color, style, modifier, or role, the resulting frame must carry styled-cell evidence:

```text
substrate=styledCell
cellSource=styledCells
styleKnown=true
```

---

## Lane D — Initial migration mapping loop

Create a first-version process document for the iterative recipe-by-recipe migration loop.

Create:

```text
docs/new_kernel/K2_6_RECIPE_MIGRATION_LOOP_PRD.md
```

This is not just a narrative doc. It should define the actual repeatable workflow that future agents use to migrate recipes in batches.

It must answer:

```text
- How does an agent pick the next recipe or family?
- What commands must it run first?
- How does it decide whether the recipe maps cleanly?
- How does it report schema gaps?
- How does it report descriptor gaps?
- How does it report player adapter gaps?
- How does it propose new v3.1 fields without mutating strict schema too casually?
- How does it preserve source recipes as read-only evidence?
- What output memo/report should each migration batch produce?
```

### Required migration-loop statuses

Use a stable classification vocabulary:

```text
canonicalReady
descriptorExpansionNeeded
schemaDecisionNeeded
adapterNeeded
sourceDecisionNeeded
semanticReviewNeeded
visualParityPending
blockedByUnknownLegacyIntent
```

### Required recommendation types

```text
addDescriptorInput
addSourceInput
addEffectDescriptor
addSourceDescriptor
addValueKind
addScopeKind
addWritePolicy
addPlayerAdapter
addMigrationRule
manualRewriteRecommended
deferUntilSemanticDecision
```

### Include a reusable agent prompt

The doc should include a compact prompt template that a local migration agent can use for one batch, roughly:

```text
Given:
- legacy source recipe path(s)
- canonical v3.1 descriptor pack(s)
- current v3.1 fixture corpus
- player/validator reports

Produce:
- proposed canonical mapping
- unmapped fields
- required descriptor/schema/player additions
- whether migration can proceed now
- whether visual parity is required before acceptance
```

This is important for the longer-term goal: recipe-by-recipe migration with feedback into schema/descriptor hardening.

---

## Lane E — Frame timeline and diff, limited version

The next CLI evidence improvement should be started now, but keep it bounded.

Either add:

```bash
render-frame --frames N
render-frame --diff-to SAMPLE_T
```

or add separate commands:

```bash
render-timeline
render-frame-diff
```

Pick the cleaner implementation shape.

Required schemas if separate:

```text
v3.1.player.frameTimeline.1
v3.1.player.frameDiff.1
```

Minimum timeline frame fields:

```text
phase
sampleT
loopT
absoluteTimeMs
renderHash
nonEmptyCells
rows
cells
status
unsupportedEffectIds
```

Minimum diff fields:

```text
from
to
changedCells
changedCellCount
hashChanged
nonEmptyDelta
```

This is inspired by `recipe-probe` timeline/diff behavior, but must remain contract-native and player-owned.

### Acceptance criteria

For at least one rendered canonical fixture:

```text
- timeline emits multiple frames
- frame hashes are deterministic
- diff reports changed cells when sampleT differs
- old render-frame output remains unchanged
```

---

# Parallelization guidance

The implementer should use sub-agents or parallel work streams. These lanes do not collide heavily if planned correctly.

## Suggested DAG

```text
                         +-----------------------------+
                         |  K2.5 carry-forward checks  |
                         |  confirm 16 rendered / 0 gap|
                         +--------------+--------------+
                                        |
                                        v
+-----------------------------+   +-----------------------------+   +-----------------------------+
| Lane A                      |   | Lane B                      |   | Lane E                      |
| GUI Player PRD              |   | primitive-field-coverage    |   | timeline / diff             |
| docs only                   |   | CLI/report infrastructure   |   | CLI/report infrastructure   |
+--------------+--------------+   +--------------+--------------+   +--------------+--------------+
               |                                 |                                 |
               |                                 v                                 |
               |                  +-----------------------------+                  |
               |                  | Lane C                      |                  |
               |                  | batch field handling         |                  |
               |                  | adapter updates              |                  |
               |                  +--------------+--------------+                  |
               |                                 |                                 |
               v                                 v                                 v
+---------------------------------------------------------------------------------------------+
| Lane D                                                                                       |
| migration-loop PRD: incorporate GUI/tooling requirements and field-coverage report semantics |
+---------------------------------------------+-----------------------------------------------+
                                              |
                                              v
+---------------------------------------------------------------------------------------------+
| Integration / verification / status memo                                                       |
| docs, vocab, report artifacts, no recipe mutation, all legacy K2 commands still green          |
+---------------------------------------------------------------------------------------------+
```

## Collision notes

Lane A is docs/product work and can proceed immediately.

Lane B and Lane E both touch CLI/report plumbing, so coordinate command parsing and usage text once, but their report builders can be separate.

Lane C should wait for Lane B’s first field-coverage data, but it can begin by auditing current adapter input handling.

Lane D can begin early as a process doc, then update once Lane B defines actual report fields.

---

# Required existing-tooling review inputs

K2.6 should explicitly reuse the source-grounded tooling review already captured in K2.3/K2.5. Do not re-review everything from scratch unless needed.

However, the GUI PRD and tooling workflow must explicitly mention the following inspiration sources and what we borrow from each:

```text
../tui-vfx-recipes/examples/demo.rs
../tui-vfx-recipes/tools/pipeline-validator
../tui-vfx-recipes/tools/recipe-probe
../tui-vfx-recipes/tools/tui-vfx-trace
../tui-vfx-recipes/tools/tui-vfx-horseman
../tui-vfx-recipes/tools/recipe-source-capture
../tui-vfx-recipes/tools/recipe-signals-doc
../tui-vfx-recipes/tools/recipe-validator
```

Expected conclusions:

```text
- demo.rs informs Ratatui UX and human playback flow.
- pipeline-validator informs mode-based CLI and staged reports.
- recipe-probe informs timeline, diff, focus-cell, and causation concepts.
- tui-vfx-trace informs selectors, stage masks, and NDJSON trace streams.
- horseman informs compact corpus summaries.
- recipe-source-capture informs reproducible generated-source artifacts.
- recipe-signals-doc informs generated-doc drift checks.
- recipe-validator remains deprecated and must not become new authority.
```

---

# Required docs and vocabulary updates

Update:

```text
docs/VOCABULARY.md
```

Add or refine terms:

```text
Ratatui GUI Player
PrimitiveFieldCoverageReport
MigrationMappingLoop
FrameTimelineReport
FrameDiffReport
Human Playback Oracle
```

Clarify:

```text
- GUI player does not replace CLI player.
- Legacy demo.rs is UX inspiration, not canonical execution authority.
- Field coverage is not visual parity.
- Timeline/diff evidence is not oracle comparison.
```

Create status memo:

```text
docs/new_kernel/PHASE_K2_6_GUI_PRD_FIELD_COVERAGE_STATUS_MEMO_TO_ARCHITECT.md
```

The memo must include the rolling context list:

```text
Completed today:
- K2.1 migration-gap
- K2.2 visual-frame report
- K2.3 primitive adapter burn-down
- K2.4 styled-cell substrate foundation
- K2.5 styled primitive adapter burn-down

Current packet:
- K2.6 GUI PRD, primitive field coverage, migration mapping loop, initial timeline/diff

Coming next:
- K2.7 Ratatui GUI player skeleton
- K2.8 migration-loop batch over next recipe family
```

---

# Acceptance criteria

K2.6 is complete when:

```text
1. Ratatui GUI Player PRD exists and explicitly reviews demo.rs as UX oracle inspiration.
2. GUI PRD states the GUI is additive to existing CLI player capability.
3. Primitive field coverage report exists with stable schema.
4. Current 16 canonical fixtures have usedButUnhandledInputFields=0.
5. Any declared-but-unused descriptor fields are reported, not hidden.
6. Remaining used primitive input handling is batched out or explicitly classified.
7. Migration-loop PRD exists with statuses, recommendation types, and reusable agent prompt.
8. Initial timeline/diff evidence exists or a justified scoped version exists.
9. Existing commands remain green:
   - render-recipe
   - render-frame
   - inventory-recipes
   - migration-gap
   - primitive-adapter-gap
10. No source recipe files are modified.
11. No legacy recipe tooling crates become dependencies of tui-vfx-player or tui-vfx-player-cli.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO="${RECIPE_REPO:-../tui-vfx-recipes}"
export TMPDIR="${TMPDIR:-/tmp}"
```

Required checks:

```bash
cargo fmt --package tui-vfx-player --package tui-vfx-player-cli -- --check

cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings

cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test --workspace

cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k26-render-report.json"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k26-visual-frame-report.json"

cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k26-inventory-report.json"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k26-primitive-adapter-gap-report.json"

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k26-primitive-field-coverage-report.json"
```

If timeline/diff is implemented:

```bash
cargo run -q -p tui-vfx-player-cli -- render-timeline \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --frames 5 \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json" \
  > "$TMPDIR/tui-vfx-k26-frame-timeline-report.json"

cargo run -q -p tui-vfx-player-cli -- render-frame-diff \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --from-sample-t 0.0 \
  --to-sample-t 1.0 \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json" \
  > "$TMPDIR/tui-vfx-k26-frame-diff-report.json"
```

Diff hygiene:

```bash
git diff --check

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes
```

Hard path check:

```bash
rg -n '"/usr/projects/tui-vfx-recipes' \
  crates/tui-vfx-player/tests \
  crates/tui-vfx-player-cli/tests \
  docs/new_kernel \
  docs/VOCABULARY.md
```

---

# Review guidance

Because this packet is intentionally broader, avoid excessive review ceremony.

Required review artifacts:

```text
- One concise implementation review section in the K2.6 status memo.
- One concise de-slop section in the K2.6 status memo.
```

Do not create multiple redundant review documents unless a blocking issue is found.

Review should focus on:

```text
- Did we preserve existing CLI behavior?
- Did we avoid legacy dependencies?
- Did the GUI PRD accurately treat demo.rs as inspiration, not authority?
- Did field coverage identify used-but-unhandled inputs?
- Did timeline/diff remain deterministic?
- Did all report schemas remain stable and named?
```

---

# Non-goals

Do not build the full GUI player in K2.6.

Do not port legacy `examples/demo.rs`.

Do not mutate source recipes.

Do not claim visual parity.

Do not add schema aliases to make old recipes validate.

Do not introduce legacy tooling crates as dependencies of the clean-room player.

Do not implement SQLite or trace-stage machinery yet unless it falls out naturally from timeline/diff work. Those are later surfaces.

---

# Expected end state after K2.6

After K2.6, we should have:

```text
- A clear Ratatui GUI Player PRD.
- A clear path to K2.7 GUI skeleton.
- Field-level confidence for the currently migrated primitive fixture set.
- A repeatable migration-loop process for future recipe batches.
- Initial timeline/diff evidence to help humans inspect behavior.
- Existing CLI player reports still acting as regression authority.
```

The larger strategy remains:

```text
CLI evidence first.
GUI player on top of CLI/player authority.
Migration loop feeds schema/descriptor/player hardening.
GUI/studio become dynamic from recipe contents, not hand-coded effect assumptions.
```
