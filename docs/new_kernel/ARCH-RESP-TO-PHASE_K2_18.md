# ARCH-RESP-TO-PHASE_K2_18.md

## Review verdict

**ACCEPT, but K2.18 must be a blocker-closure sprint.**

K2.17 was productive: it raised the canonical corpus from **88 to 142 fixtures**, expanded descriptor-backed effects from **45 to 75**, added `implementation-readiness`, added `control-catalog`, repaired the source/content vocabulary, and kept every core fixture gate green.

But the user concern is valid: the remaining blocker counts are still large enough that another broad “tranche” would risk becoming wheel-spinning.

Current blocker state from K2.17:

```text
validate-recipe:                 142/142 valid
render-recipe:                   142/142 rendered, 0 unsupported, 0 errors
render-frame:                    142/142 rendered, 0 unsupported, 0 errors
fixture-qc:                      pass
primitive-field-coverage:        894/894 handled
primitive-adapter-gap:           75/75 rendered, 0 unresolved
schema-readiness:                canDeclareSchemaReady=true
explicitOwnerDecisionNeeded:     0

migration mapping:
  canonicalExists:               97
  descriptorDecisionNeeded:      76
  sourceDecisionNeeded:          40
  blockedByFieldCoverage:        8

implementation-readiness:
  canonicalExists:               159
  contentBacklog:                39
  sourceBacklog:                 1
  descriptorBacklog:             84
  graphRuntimeBacklog:           83
  sceneRuntimeBacklog:           16
```

The remaining work is not schema decision work. It is implementation, evidence, and final disposition work.

The next packet must focus on **closing blockers**, not expanding surface area opportunistically.

---

# Phase K2.18 — Blocker Closure Sprint

## Executive goal

K2.18 should aggressively close the remaining concrete blocker queues:

```text
descriptorDecisionNeeded:      76  -> target <= 20, preferred 0 except signed holdbacks
contentBacklog:                39  -> target <= 10, preferred 0 except signed holdbacks
sourceBacklog:                  1  -> target 0 or signed oracle/source holdback
blockedByFieldCoverage:         8  -> target 0
graphRuntimeBacklog:           83  -> target <= 15, preferred exact closure/disposition of all 83
sceneRuntimeBacklog:           16  -> target 0
explicitOwnerDecisionNeeded:    0  -> must remain 0 unless a real owner decision is unavoidable
```

This packet should not merely add fixtures. It should **resolve each blocker path** into one of these outcomes:

```text
canonicalExists
descriptorBacklogResolved
contentBacklogResolved
sourceBacklogResolved
graphRuntimeResolved
sceneRuntimeResolved
backendHoldbackSignedOff
guiHumanReviewHoldbackSignedOff
oracleOnlySignedOff
duplicateVariantSignedOff
deprecatedLegacySignedOff
explicitOwnerDecisionNeeded
```

If a blocker cannot be implemented honestly, it must be converted into a precise signed holdback with path-level justification. Generic `descriptorBacklog`, `contentBacklog`, `graphRuntimeBacklog`, and `sceneRuntimeBacklog` should not remain vague buckets.

---

## Rolling context for implementer

Completed before K2.18:

```text
K2.13:
  Schema decision readiness approved.
  canDeclareSchemaReady=true.
  Source/content, graph I/O, runtime dynamism, scene/layer, templates, and studio-control boundaries accepted.

K2.14:
  Descriptor/adapter tranche 1.
  Canonical fixtures 27 -> 57.

K2.15:
  Player graph topology/value-bus execution begins.
  Canonical fixtures 57 -> 67.

K2.16:
  PlayerRenderIrReport and render-ir CLI.
  Canonical fixtures 67 -> 88.

K2.17:
  Major descriptor/source/content burn-down.
  Canonical fixtures 88 -> 142.
  Descriptor-backed effects 45 -> 75.
  implementation-readiness CLI.
  control-catalog CLI.
  scene visibility binding support.
  player backend seam.
```

Rules that remain locked:

```text
Legacy debug_recipes/ is read-only evidence.
Canonical fixtures go only under recipes/v3.1/debug_recipes/.
Schema readiness is closed; do not reopen it.
Templates are mandatory compile-time composition, not runtime inheritance.
Scene/element/layer support is core v3.1 work.
Player evidence is honest evidence, not visual parity.
UI consumes player evidence and must not construct compositor internals.
Compositor/backend work belongs behind the player backend seam.
```

---

# Work model: 10 parallel lanes

```text
A. Blocker ledger and counter reconciliation
B. Field-coverage blocker closure
C. Remaining content backlog closure
D. Source backlog and source fidelity closure
E. Filter/mask/sampler descriptor closure
F. Shader/style descriptor closure
G. Graph runtime backlog closure
H. Scene runtime backlog closure
I. Holdback signoff and backend boundary
J. QA, docs, schema/API sync, review/de-slop
```

Each lane must return a lane memo with:

```text
- exact blocker paths assigned
- implemented paths
- held-back paths
- canonical fixtures added
- descriptors/adapters changed
- counters before/after for that lane
- tests/gates run
- remaining blockers, if any, with exact reason
```

---

# Lane A — Blocker ledger and counter reconciliation

## Objective

Create a single authoritative blocker ledger for K2.18.

K2.17 now has multiple useful counters, but the raw reports still disagree in ways that can confuse decisions:

```text
migration canonicalExists:          97
implementation canonicalExists:     159

migration descriptorDecisionNeeded: 76
implementation descriptorBacklog:   84

migration sourceDecisionNeeded:     40
implementation contentBacklog:      39
implementation sourceBacklog:        1
```

K2.18 must reconcile these into a path-level ledger.

## Required command/report

Either extend `implementation-readiness` or add a subcommand/report mode:

```bash
cargo run -q -p tui-vfx-player-cli -- implementation-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --include-blockers \
  --json
```

If `--include-blockers` already exists under another flag, use the existing flag. Do not add a redundant command unless needed.

## Required ledger fields

Each blocker record must include:

```text
legacyPath
legacyFamily
legacyRecipeId
canonicalPath
canonicalExists
rawMigrationStatus
implementationDisposition
blockerKind
schemaBlocking
implementationBlocking
recommendedAction
assignedLane
requiredDescriptors[]
missingDescriptors[]
requiredContentDescriptors[]
requiredSources[]
missingSources[]
requiredPlayerAdapters[]
requiredRuntimeFeatures[]
fieldCoverageIssues[]
holdbackReason
holdbackSignedOff
confidence
notes[]
```

## Required queue summaries

Produce queues for:

```text
blockedByFieldCoverage
contentBacklog
sourceBacklog
descriptorBacklog
graphRuntimeBacklog
sceneRuntimeBacklog
backendHoldback
guiHumanReviewHoldback
oracleOnly
duplicateVariant
deprecatedLegacy
```

## Acceptance

```text
- Every remaining K2.17 blocker path is assigned to exactly one K2.18 lane.
- No blocker path is hidden in generic ownerAudit.
- content.* vocabulary remains corrected; no durable pseudo-source names such as source.typewriterText.
- The report has a final before/after table after all other lanes finish.
```

## Deliverables

```text
docs/new_kernel/K2_18_BLOCKER_LEDGER_REPORT.md
docs/new_kernel/K2_18_BASELINE_AND_FINAL_COUNTERS.md
```

---

# Lane B — Field-coverage blocker closure

## Objective

The K2.17 migration report still shows:

```text
blockedByFieldCoverage: 8
```

This must go to zero.

These are the highest-priority blockers because they mean fields are authored but not fully descriptor/player-handled.

## Required first step

Extract and list the exact 8 paths before implementing anything:

```bash
cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json \
  > /tmp/k218-migration-mapping.json
```

Then use a script or existing report parser to emit the 8 `blockedByFieldCoverage` records.

## Required disposition

For each of the 8 records, choose exactly one:

```text
implemented:
  descriptor field exists,
  canonical fixture exists,
  adapter consumes field,
  field coverage passes.

backendHoldbackSignedOff:
  field requires backend/subcell/compositor fidelity,
  not player-evidence appropriate,
  signed as backend holdback.

oracleOnlySignedOff:
  field belongs to oracle/deprecated/offline evidence.
```

Do **not** leave any record in `blockedByFieldCoverage`.

## Acceptance

```text
blockedByFieldCoverage: 8 -> 0
primitive-field-coverage remains 0 unhandled
fixture-qc remains pass
no fields are marked handled without actual adapter consumption
```

## Deliverable

```text
docs/new_kernel/K2_18_FIELD_COVERAGE_CLOSURE_REPORT.md
```

---

# Lane C — Remaining content backlog closure

## Objective

Reduce or close:

```text
contentBacklog: 39
```

K2.17 fixed the source/content vocabulary. K2.18 should now clear the remaining content backlog paths.

## Required first step

Extract the exact 39 content backlog paths from `implementation-readiness --include-blockers`.

Classify them by descriptor family:

```text
content.typewriter
content.splitFlap
content.odometer
content.cellMotion
content.glyphCascade
content.glyphParticles
content.redact
content.numeric
content.mirror
content.slideShift
content.dissolve
content.glitchShift
content.scrambleGlitchShift
```

## Implementation strategy

### High-confidence closures

Prioritize remaining variants of descriptors already accepted and adapted:

```text
content.typewriter cursor variants
content.splitFlap variants
content.odometer variants
content.cellMotion variants
content.glyphCascade variants
content.glyphParticles bounded variants
```

These should become canonical fixtures if descriptors/adapters can consume all authored fields.

### Holdbacks

Only hold back content paths when they require:

```text
backend/subcell renderer
unimplemented source asset system
unbounded particle engine
visual parity that player evidence cannot honestly approximate
```

## Minimum target

```text
contentBacklog: 39 -> <= 10
```

## Preferred target

```text
contentBacklog: 39 -> 0
```

If not zero, every remaining content path must be explicitly listed with a holdback reason.

## Fixture targets

Add canonical fixtures for as many of these as support permits:

```text
content/content_split_flap_ambient_board.json
content/content_split_flap_arrivals_board.json
content/content_split_flap_authentic_timing.json
content/content_split_flap_board_update.json
content/content_split_flap_cycles.json
content/content_split_flap_digits.json
content/content_split_flap_from_message.json
content/content_split_flap_jitter.json
content/content_split_flap_leading_blocks.json
content/content_split_flap_rolling_cards.json
content/content_split_flap_settle_hinge.json
content/content_split_flap_solari_authentic.json
content/content_split_flap_solari_museum.json
content/content_split_flap_spring_settle.json
content/content_split_flap_tile_2row.json
content/content_split_flap_tile_4row.json
content/content_split_flap_tile_6row.json
content/content_split_flap_tile_8row.json
content/content_split_flap_tile_board.json

content/content_odometer_3x3_count_bindable.json
content/content_odometer_cell_roll_diagonal.json
content/content_odometer_cell_roll_dispersion_edge_in.json
content/content_odometer_cell_roll_down.json
content/content_odometer_cell_roll_left.json
content/content_odometer_cell_roll_slot_machine.json
content/content_odometer_decimal_preset_carry.json
content/content_odometer_slot_reel.json

content/content_typewriter_cursor_braille_2.json
content/content_typewriter_cursor_braille_4.json
content/content_typewriter_cursor_braille_6.json
content/content_typewriter_cursor_braille_8.json
content/content_typewriter_cursor_braille_flip.json
content/content_typewriter_cursor_braille_pulse.json
content/content_typewriter_cursor_caret.json
content/content_typewriter_cursor_full.json
content/content_typewriter_cursor_grow_in_center.json
content/content_typewriter_cursor_grow_in_down.json
content/content_typewriter_cursor_grow_in_up.json
content/content_typewriter_cursor_scan_bounce.json
content/content_typewriter_cursor_scan_pulse.json
content/content_typewriter_cursor_wake_gap.json
content/content_typewriter_cursor_wake_ghost.json
content/content_typewriter_cursor_wake_tint.json

content/content_cell_motion_middle_out.json
content/content_cell_motion_root_border_fixed.json
content/content_cell_motion_slice.json
content/content_typewriter_io_filter_shader.json
```

Use the exact blocker ledger to avoid duplicate fixture work.

## Acceptance

```text
- contentBacklog <= 10, preferred 0.
- Every added fixture validates/renders/QCs.
- Content descriptors consume all authored fields.
- Timeline/diff smoke proves motion for animated content.
- No content records are mislabeled as source records.
```

## Deliverable

```text
docs/new_kernel/K2_18_CONTENT_BACKLOG_CLOSURE_REPORT.md
```

---

# Lane D — Source backlog and source fidelity closure

## Objective

Reduce or sign off:

```text
sourceBacklog: 1
```

K2.17 says the one true source backlog remains after content/source vocabulary repair. This is likely command-capture/source-material work, but do not assume; extract the exact path.

## Required work

For the one source backlog path:

```text
- identify exact legacy path;
- decide whether it is sourceDescriptor, oracleOnly, or offlineAuthoringArtifact;
- implement only if runtime-safe and descriptor-backed;
- otherwise sign it off as oracleOnly or offline authoring holdback.
```

## Source fidelity hardening

Keep the K2.17 source progress, but do not expand into uncontrolled work:

```text
source.ansi:
  bounded SGR support only; no full VTE.

source.image:
  resolver seam and fallback grid; no full rasterization unless already scoped and deterministic.

source.procedural:
  bounded registry only; no plugin system.
```

## Acceptance

```text
sourceBacklog: 1 -> 0 or signed holdback
sourceDecisionNeeded decreases accordingly in primary disposition report
no command execution
no full VTE/rasterization scope creep
```

## Deliverable

```text
docs/new_kernel/K2_18_SOURCE_BACKLOG_CLOSURE_REPORT.md
```

---

# Lane E — Filter/mask/sampler descriptor closure

## Objective

Reduce the remaining descriptor backlog by closing low- and medium-risk filter/mask/sampler records.

K2.17 already added many. K2.18 should focus on remaining paths, not duplicate existing coverage.

## Required first step

From the blocker ledger, extract remaining descriptor backlog paths in these families:

```text
filters
masks
samplers
```

Group by descriptor id.

Likely remaining candidates include some of:

```text
filter.animatedGlyphRamp
filter.brailleDust
filter.charsetNoise
filter.colorBridgedShade
filter.glistenSweep
filter.glyphStyle
filter.interlaceCurtain
filter.motionBlur
filter.rigidShake
filter.shadeScanner
filter.subCellShake
filter.subcellLight

mask.centerWipeFade
mask.wipeFadeLeftRight
mask.materialize variants
mask.corner/wipe variants not already mapped

sampler records, if any remain after K2.17
```

Do not assume this list is complete. Use the ledger.

## Implementation policy

Implement descriptors/adapters when all authored fields can be consumed.

Move to backend holdback if the effect depends on:

```text
subcell renderer
terminal raster/light propagation
backend-only visual fidelity
compositor-specific behavior
```

## Minimum target

Close at least:

```text
20 descriptor backlog records
```

Preferred:

```text
all remaining filter/mask/sampler descriptor backlog records
```

## Acceptance

```text
- Descriptor backlog decreases materially.
- All authored fields are handled.
- Field coverage remains 0 unhandled.
- Adapter gap remains 0 unresolved.
- Backend-heavy records are signed off, not left generic.
```

## Deliverable

```text
docs/new_kernel/K2_18_FILTER_MASK_SAMPLER_CLOSURE_REPORT.md
```

---

# Lane F — Shader/style descriptor closure

## Objective

Close the remaining shader/style descriptor backlog or move backend-heavy records into signed holdbacks.

## Required first step

Extract exact remaining shader/style descriptor backlog paths.

Likely remaining groups include some of:

```text
shader.affordanceWake
shader.bevel
shader.chromaticEdge
shader.coloredOverlay
shader.concealedLight
shader.cursor
shader.edgeSheen
shader.glow
shader.orbit
shader.pulseWave
shader.reflect
shader.stochasticSparkle
shader.terminalFire
shader.terminalWater
shader.tracePath
shader.tracePropagation

style.colorShift
style.glitch
style.rainbow
style.rigidShakeStyle
style.spatialEffect
```

Do not assume. Use the blocker ledger.

## Implementation policy

### Implement now when possible

Prioritize styled-cell player-evidence descriptors:

```text
shader.coloredOverlay
shader.glow
shader.pulseWave
shader.reflect
shader.affordanceWake
shader.concealedLight
shader.edgeSheen
shader.bevel
shader.cursor

style.colorShift
style.glitch
style.rainbow
style.rigidShakeStyle
style.spatialEffect
```

### Hold back deliberately

Move to backend holdback when honest support needs backend/compositor/subcell fidelity:

```text
shader.terminalFire
shader.terminalWater
shader.tracePath
shader.tracePropagation
shader.orbit
shader.stochasticSparkle
shader.chromaticEdge
shader.subCellShake
```

This is not failure if the holdback is exact and signed.

## Minimum target

Close or sign off at least:

```text
30 shader/style descriptor backlog records
```

Preferred:

```text
no generic shader/style descriptorBacklog remains
```

## Acceptance

```text
- Descriptor backlog decreases materially.
- Backend-heavy shader records are signed backendHoldback with exact reason.
- Style records use accepted scope vocabulary only.
- No generic predicate registry is introduced.
- Field coverage remains 0 unhandled.
```

## Deliverable

```text
docs/new_kernel/K2_18_SHADER_STYLE_CLOSURE_REPORT.md
```

---

# Lane G — Graph runtime backlog closure

## Objective

Resolve:

```text
graphRuntimeBacklog: 83
```

This is one of the largest remaining blockers. K2.15/K2.16/K2.17 added graph topology/value-bus support, so the remaining graph backlog should now be reducible.

## Required first step

Extract all 83 graph runtime backlog paths from `implementation-readiness`.

Classify them into:

```text
alreadyRepresentableByPlayerGraph
needsCanonicalGraphFixture
needsEffectOutputPublication
needsGraphValueKindDiagnostic
needsParallelMergePolicy
needsSceneLocalGraphRuntime
needsBackendHoldback
needsGuiHumanReviewHoldback
oracleOnly
```

## Implementation priorities

### 1. Already representable

If a path is now covered by topology-first execution, sequence, parallel, graph values, or conflict diagnostics, update mapping/disposition and add canonical fixture only where useful.

### 2. Effect output publication

K2.15 still noted:

```text
Effect-output publication remains unsupported unless an adapter implements a real effect output; current graph value publication is input re-emission.
```

K2.18 should implement a small descriptor-driven output registry:

```text
node output source can publish:
  resolved input field
  adapter-produced scalar
  adapter-produced bool
  adapter-produced color
  adapter-produced sampled field summary if appropriate
```

Do not invent hidden graph values. Outputs must be declared.

### 3. Kind diagnostics

Harden runtime graph diagnostics:

```text
missingGraphValue
graphValueKindMismatch
graphValueMergeConflict
graphValueUnsupportedSource
graphValueCycleRejected if applicable
```

### 4. Parallel merge policy

Ensure deterministic behavior for:

```text
lastWriter
errorOnConflict
warnOnConflict
```

If only last-writer exists, add warning/error modes if already accepted in schema; otherwise document holdback.

## Minimum target

```text
graphRuntimeBacklog: 83 -> <= 20
```

## Preferred target

```text
graphRuntimeBacklog: 83 -> 0 or signed holdbacks
```

## Canonical fixtures

Add targeted graph fixtures for:

```text
sequence effect-output publication
parallel graph-value merge
parallel graph-value conflict warning
parallel graph-value conflict error
graph-value kind mismatch
missing graph value without fallback
scene-local graph value usage
content -> graph effect value propagation if supported
```

## Acceptance

```text
- Graph runtime backlog decreases materially.
- New diagnostics are structured and deterministic.
- render-ir exposes graph values and graph diagnostics.
- graph.order fallback remains green.
- No hidden runtime inheritance/templates.
```

## Deliverable

```text
docs/new_kernel/K2_18_GRAPH_RUNTIME_CLOSURE_REPORT.md
```

---

# Lane H — Scene runtime backlog closure

## Objective

Resolve:

```text
sceneRuntimeBacklog: 16
```

K2.17 added scene/layer visibility support. K2.18 should close the remaining scene runtime backlog.

## Required first step

Extract all 16 scene runtime backlog paths and classify:

```text
visibilityRuntime
layerLocalPipeline
transparentBlendClear
elementDiagnostics
localCoordinates
sceneSourceIntegration
backendHoldback
guiHumanReviewHoldback
```

## Required runtime work

Implement or harden:

```text
layer-local pipeline diagnostics include element/layer id
transparent-empty policy is deterministic and reflected in render IR
explicit clear policy if already in accepted DTOs; otherwise diagnose unsupported clear
local coordinate scope tests for rect/role/channel scopes
scene source integration for ANSI/image/procedural/text/card layers
layer visibility false/true is represented in render-ir
scene merge conflicts produce element-attributed warnings
```

## Minimum target

```text
sceneRuntimeBacklog: 16 -> 0
```

If a scene path is backend-only, sign it as backendHoldback and remove it from generic sceneRuntimeBacklog.

## Canonical fixtures

Add fixtures for:

```text
scene layer visibility false
scene layer visibility true through binding override
scene layer local rect scope
scene layer transparent skip preserving lower content
scene element merge conflict warning
scene source ANSI layer with styled cells if supported
scene procedural layer visibility
scene image fallback layer provenance
```

## Acceptance

```text
sceneRuntimeBacklog: 0
render-ir includes element/layer provenance and diagnostics
fixture-qc remains pass
no role tag is overloaded as element identity
```

## Deliverable

```text
docs/new_kernel/K2_18_SCENE_RUNTIME_CLOSURE_REPORT.md
```

---

# Lane I — Holdback signoff and backend boundary

## Objective

Prevent backend, GUI, oracle, duplicate, and deprecated items from resurfacing as generic blockers.

## Current known holdbacks

```text
backendHoldback
guiHumanReviewHoldback
oracleOnly
duplicateVariant
deprecatedLegacy
```

## Required work

For every holdback path, emit:

```text
legacyPath
family
holdbackDisposition
reason
whyNotSchemaBlocking
whyNotDescriptorBlocking if applicable
futureEvidenceRequired
ownerSignoffStatus
```

## Backend-specific policy

Backend holdbacks should include:

```text
shadows/*
subcell_shapes/*
terminal fire/water if not implemented honestly
subcell light/shake if backend dependent
trace/orbit effects if compositor/backend dependent
```

The report must restate:

```text
Render IR -> PlayerRenderBackend -> future compositor backend adapter.
UI must not construct compositor internals.
```

## GUI conflict policy

For GUI human-review records:

```text
- keep them held back unless deterministic visual conflict diagnostics now make them implementable;
- include screenshot/review requirements for future packet;
- do not mutate schema to satisfy review-only conflicts.
```

## Oracle/deprecated policy

For oracle-only and deprecated records:

```text
- no runtime command execution;
- no canonical fixture unless owner reactivates;
- may be used as offline evidence only.
```

## Acceptance

```text
- No backend/gui/oracle/duplicate/deprecated item remains in generic descriptor/content/graph/scene backlog.
- Holdback register is path-level, not cluster-only.
```

## Deliverable

```text
docs/new_kernel/K2_18_HOLDBACK_SIGNOFF_REGISTER.md
```

---

# Lane J — QA, docs, schema/API sync, review/de-slop

## Objective

Keep the corpus green and keep documentation honest.

## Required docs

Create:

```text
docs/new_kernel/K2_18_BLOCKER_LEDGER_REPORT.md
docs/new_kernel/K2_18_BASELINE_AND_FINAL_COUNTERS.md
docs/new_kernel/K2_18_FIELD_COVERAGE_CLOSURE_REPORT.md
docs/new_kernel/K2_18_CONTENT_BACKLOG_CLOSURE_REPORT.md
docs/new_kernel/K2_18_SOURCE_BACKLOG_CLOSURE_REPORT.md
docs/new_kernel/K2_18_FILTER_MASK_SAMPLER_CLOSURE_REPORT.md
docs/new_kernel/K2_18_SHADER_STYLE_CLOSURE_REPORT.md
docs/new_kernel/K2_18_GRAPH_RUNTIME_CLOSURE_REPORT.md
docs/new_kernel/K2_18_SCENE_RUNTIME_CLOSURE_REPORT.md
docs/new_kernel/K2_18_HOLDBACK_SIGNOFF_REGISTER.md
docs/new_kernel/K2_18_SCHEMA_API_DOCS_GATE.md
docs/new_kernel/PHASE_K2_18_BLOCKER_CLOSURE_STATUS_MEMO_TO_ARCHITECT.md
docs/new_kernel/PHASE_K2_18_REVIEW_AND_DESLOP_REPORT.md
```

Update if touched:

```text
docs/VOCABULARY.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/INDEX.md
```

## Documentation quality requirements

For every touched file:

```text
- rustdoc for public/semi-public APIs;
- schemars descriptions if contract DTOs change;
- no transient phase labels in durable schema/API names;
- no stale counters;
- no generic ownerAudit hiding real blockers;
- no “visual parity” claims unless backend/oracle evidence exists.
```

## Acceptance

```text
- All required docs exist.
- Final status memo includes exact remaining blocker paths if any remain.
- Review/de-slop report records independent review findings and fixes.
```

---

# Canonical fixture target

K2.18 should add fewer opportunistic fixtures than K2.17 but more blocker-targeted fixtures.

Target:

```text
40–70 new canonical fixtures or signed holdback resolutions
```

Minimum acceptable:

```text
25 canonical fixtures plus path-level closure/signoff of remaining blockers
```

A fixture may be added only if:

```text
descriptor exists or is added;
all authored fields are descriptor-covered;
all authored fields are player-handled or explicitly diagnosed;
validate-recipe passes;
render-recipe passes;
render-frame passes;
fixture-qc passes;
primitive-field-coverage remains 0 unhandled;
primitive-adapter-gap remains 0 unresolved;
expected_visual metadata exists;
legacy root is untouched.
```

---

# Required tests

## Ledger/report tests

```text
implementation-readiness --include-blockers emits exact blocker paths
no content record is emitted as source.typewriterText/source.odometer/source.splitFlapText
blockedByFieldCoverage records can be listed path-by-path
holdback paths are path-level, not cluster-only
```

## Field coverage tests

```text
all 8 field coverage blockers close or are signed holdbacks
new descriptor fields are consumed by adapters
field coverage remains 0 unhandled
```

## Content tests

```text
remaining splitFlap variants consume speed/cascade/cycles/tile fields
remaining odometer variants consume direction/travel/tile/from-message fields
typewriter cursor variants consume cursor fields or are held back
cellMotion consumes route/stagger/affect/from/to fields
glyph particle variants consume emitter fields or are held back
```

## Descriptor tests

```text
new filter descriptors consume authored inputs
new mask descriptors consume authored inputs
new sampler descriptors consume authored inputs
new shader/style descriptors consume authored inputs
backend-heavy descriptors are signed holdbacks, not fake adapters
```

## Graph runtime tests

```text
effect output publication works for at least one adapter-produced value
sequence consumes published adapter value
parallel join exposes published value after join
sibling branches cannot see unpublished sibling values
graphValueKindMismatch emits deterministic diagnostic
missingGraphValue without fallback emits deterministic diagnostic
merge conflict emits configured warning/error
```

## Scene runtime tests

```text
sceneRuntimeBacklog paths are covered or signed off
layer visibility true/false works
layer local scope uses local coordinates
transparent skip preserves lower content
element/layer merge conflict diagnostics include element/layer id
render-ir includes scene/layer provenance and visibility result
```

## Regression tests

```text
graph.order fallback remains green
render-ir CLI remains green
control-catalog remains green
fixture-qc remains pass
schema-readiness remains canDeclareSchemaReady=true
legacy root mutation check remains clean
```

---

# Acceptance criteria

## Required

```text
- blockedByFieldCoverage goes to 0.
- sourceBacklog goes to 0 or signed holdback.
- contentBacklog decreases from 39 to <=10, or every remaining path has a signed exact holdback.
- descriptorDecisionNeeded decreases from 76 to <=20, or every remaining path has a signed exact holdback.
- descriptorBacklog decreases from 84 to <=20, or every remaining path has a signed exact holdback.
- graphRuntimeBacklog decreases from 83 to <=20, or every remaining path has a signed exact holdback.
- sceneRuntimeBacklog goes to 0.
- explicitOwnerDecisionNeeded remains 0 unless truly unavoidable.
- validate-recipe passes full canonical corpus.
- render-recipe passes full canonical corpus.
- render-frame passes full canonical corpus.
- fixture-qc passes full canonical corpus.
- primitive-field-coverage remains 0 unhandled.
- primitive-adapter-gap remains 0 unresolved.
- schema-readiness remains canDeclareSchemaReady=true.
- implementation-readiness emits a path-level blocker ledger.
- legacy debug_recipes root remains unmodified.
```

## Preferred

```text
- descriptorDecisionNeeded reaches 0 except backend/oracle/deprecated/duplicate signed holdbacks.
- contentBacklog reaches 0 except signed holdbacks.
- graphRuntimeBacklog reaches 0 except signed holdbacks.
- canonicalExists exceeds 130 in migration mapping.
- canonical v3.1 fixtures exceed 180.
- backend/gui/oracle/duplicate/deprecated holdbacks are path-level signed.
```

## Explicit stop conditions

Stop and report rather than forcing if:

```text
descriptor closure requires fake field handling;
content effects are mislabeled as source descriptors;
graph runtime requires hidden/unresolved graph values;
scene runtime requires unresolved template semantics;
backend-heavy shaders require compositor fidelity;
ANSI requires full VTE scope;
image source requires full rasterization;
procedural source requires plugins or command execution;
UI begins constructing compositor internals;
control catalog infers behavior from raw recipe internals instead of descriptor metadata.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

## Format and lint

```bash
cargo fmt \
  --package tui-vfx-player \
  --package tui-vfx-player-cli \
  --package tui-vfx-player-ui \
  --package tui-vfx-contract \
  --package tui-vfx-contract-cli \
  -- --check

cargo clippy \
  -p tui-vfx-player \
  -p tui-vfx-player-cli \
  -p tui-vfx-player-ui \
  -p tui-vfx-contract \
  -p tui-vfx-contract-cli \
  --all-targets --all-features -- -D warnings
```

## Tests

```bash
cargo nextest run -p tui-vfx-player --no-fail-fast
cargo nextest run -p tui-vfx-player-cli --no-fail-fast
cargo nextest run -p tui-vfx-player-ui --no-fail-fast
cargo nextest run -p tui-vfx-contract --no-fail-fast
cargo nextest run -p tui-vfx-contract-cli --no-fail-fast
cargo nextest run --workspace --no-fail-fast
```

Fallback if nextest is unavailable:

```bash
cargo test --workspace
```

## Corpus gates

```bash
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-ir \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json"

cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --include-offenders \
  --json

cargo run -q -p tui-vfx-player-cli -- implementation-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --include-blockers \
  --json

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json

cargo run -q -p tui-vfx-player-cli -- control-catalog \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json
```

## Docs and schema gates

```bash
cargo test -p tui-vfx-contract --test test_schema_generation

UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract \
  --test test_schema_generation \
  checked_in_contract_schemas_are_current \
  -- --exact

cargo xtask docs generate
cargo xtask docs check
cargo xtask docs api
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema
```

## Cleanliness

```bash
git diff --check

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes

rg -n '"/usr/projects/tui-vfx-recipes|/usr/projects/tui-vfx-recipes' \
  crates/tui-vfx-player \
  crates/tui-vfx-player-cli \
  crates/tui-vfx-player-ui \
  crates/tui-vfx-contract \
  crates/tui-vfx-contract-cli \
  docs/new_kernel
```

The legacy root mutation check must be clean.

---

# Expected final memo

Return:

```text
docs/new_kernel/PHASE_K2_18_BLOCKER_CLOSURE_STATUS_MEMO_TO_ARCHITECT.md
```

It must include:

```text
- executive summary
- before/after counters
- exact blocker closure table
- path-level remaining blocker list, if any
- field coverage closure result
- content backlog closure result
- source backlog closure result
- descriptor backlog closure result
- graph runtime backlog closure result
- scene runtime backlog closure result
- holdback signoff register summary
- canonical fixture additions
- descriptor additions
- adapter additions
- report/schema/API/docs changes
- verification matrix
- legacy root mutation status
- review/de-slop findings
- recommended next packet
```

The final memo must not merely say “remaining blockers exist.” It must list the exact remaining paths and why each one was not closed.

---

# Architectural guidance for K2.18 decisions

Use these decisions to avoid stalling.

## 1. Content effects are not sources

If a legacy file transforms text over time, treat it as:

```text
content.<effect>
```

not:

```text
source.<effect>Text
```

The source is usually `source.text`, `source.card`, or a scene layer source. The content descriptor is the moving part.

## 2. Bounded player evidence is acceptable if honest

For player-only evidence, it is acceptable to implement deterministic approximations for:

```text
glitch
scan
cascade
particle
wipe
fade
pulse
highlight
shape
style
```

as long as the report says:

```text
deterministic player evidence, not visual parity
```

## 3. Backend-heavy effects should become signed holdbacks

Do not waste time forcing backend-heavy effects into player text/styled-cell approximations if they require:

```text
subcell renderer
real compositor layering
procedural fire/water field fidelity
image rasterization
shadow alpha compositing
trace/path geometry fidelity
```

Move them into path-level `backendHoldbackSignedOff`.

## 4. Graph runtime backlog should shrink now

We have enough graph execution infrastructure to resolve many graph backlog records. If a graph record still cannot be resolved, it should name the exact missing runtime feature:

```text
effect-output publication
kind mismatch diagnostics
parallel merge policy
scene-local graph runtime
backend-held graph behavior
```

## 5. Scene runtime backlog should close

K2.17 added scene visibility. K2.18 should finish scene runtime disposition. Remaining scene blockers should not survive as generic `sceneRuntimeBacklog`.

## 6. Holdbacks are allowed, but they must be exact

A holdback is acceptable when it has:

```text
path
reason
future evidence required
why not schema-blocking
why not implementation-ready
signoff status
```

A generic holdback cluster is not enough.

---

# What we should be able to close after K2.18

If K2.18 lands successfully, we should be able to close:

```text
schema readiness: already closed
source/content vocabulary ambiguity: closed
field coverage blockers: closed
true source backlog: closed or signed oracle/source holdback
scene runtime generic backlog: closed
most descriptor implementation backlog: closed or signed backend holdback
most graph runtime generic backlog: closed or reduced to exact missing features
```

After that, the project should be in a much cleaner state:

```text
not “we have many blockers”
but “we have canonical fixtures plus a small signed list of backend/visual/oracle holdbacks”
```

That is the threshold we need before moving confidently into:

```text
backend/compositor adapter prototype
GUI visual review workflow
studio control panel pilot
template compiler implementation
release gate hardening
```
