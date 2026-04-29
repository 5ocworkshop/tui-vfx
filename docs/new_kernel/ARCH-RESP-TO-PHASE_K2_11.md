Absolutely. K2.12 should explicitly split the backlog into:

```text
Fast lane:
  low-friction, high-confidence items we can implement or fixture now

Decision lane:
  blockers that need explicit schema/descriptor/source/runtime decisions

Holdback lane:
  problematic or ambiguous items that should be classified, documented, and removed from the critical path until their model is ready
```

That distinction is key. We do **not** need every legacy recipe to be portable immediately in order to reach schema readiness. We need every item to be either:

```text
portable under the schema,
ready for descriptor/player work,
or intentionally held back with a named disposition.
```

Below is the revised packet.

---

# Architect Response to K2.11

## Review verdict

**ACCEPT WITH DIRECTIONAL CORRECTION.**

K2.11 gave us the right control surface: `schema-readiness` now says plainly that we are **not** schema-ready, and it identifies the high-level blocker groups.

However, the next packet must stop producing mostly category summaries. K2.12 must make concrete classifications and decisions. The user is right: we cannot keep circling the same blocker list.

K2.12 should move in three modes at once:

```text
1. Implement low-friction/high-confidence items.
2. Make architecture recommendations for real schema blockers.
3. Hold back problematic items with explicit dispositions so they stop blocking progress.
```

---

# When can we declare 100% schema readiness?

We should define this precisely.

## 100% schema readiness does **not** mean

```text
all 603 legacy recipes are ported,
all visual parity is proven,
all descriptor adapters are complete,
or compositor-backed rendering is wired.
```

## 100% schema readiness means

Every legacy debug recipe is classified into one of these states:

```text
A. already canonical v3.1,
B. directly portable with current schema,
C. portable after descriptor/player/source work, with no schema change needed,
D. held back as oracle-only / duplicate / backend-renderer / owner-policy item,
E. blocked by a named schema decision that has now been resolved.
```

So we can declare schema readiness when:

```text
unknownRecords = 0
notYetClassified = 0
untriagedOwnerAuditRecords = 0
fieldCoverage schema blockers = 0
source/content schema blockers = 0 or explicitly resolved/held back
runtime dynamism schema blockers = 0 or explicitly resolved/held back
scene schema blockers = 0 or explicitly resolved/held back
canDeclareSchemaReady = true
```

Current K2.11 state:

```text
totalLegacyRecords:                603
schemaReadyRecords:                217
estimatedSchemaReadinessPercent:   36.0
canDeclareSchemaReady:             false
```

## Target checkpoint

My current expectation:

```text
K2.12 should make 100% schema readiness assessable and possibly reachable.
K2.13 should be treated as the schema-lock candidate if K2.12 exposes new hard decisions.
```

K2.12 should attempt to move readiness dramatically by clearing low-friction blockers and converting ambiguous blockers into explicit holdbacks or resolved schema decisions.

---

# Phase K2.12 — Schema Lock Decision Sprint + Low-Friction Burn-down

## Executive goal

Convert the K2.11 readiness ledger into a decisive migration control board.

K2.12 must answer:

```text
Which blockers can we clear now?
Which records are safe to hold back?
Which schema decisions must be made before declaring readiness?
Which descriptor/source/player changes are low-friction and should proceed immediately?
Which items are non-schema migration work and should stop blocking schema readiness?
```

This packet should **not** try to port the entire corpus. It should classify and unblock it.

---

## Core strategic rule

```text
Proceed aggressively on low-friction items.
Hold back problematic items explicitly.
Do not force ambiguous legacy intent into the v3.1 schema.
```

A held-back item is not a failure if it has a clear disposition:

```text
oracleOnly
duplicateOrVariant
backendRenderer
guiHumanReview
ownerPolicyHoldback
futureDescriptorCandidate
futureSchemaCandidate
```

---

# Work model: 8 parallel lanes

The orchestrator can run 6–8 pipelines. Use that capacity.

```text
A. Offender ledger + readiness classification
B. Low-friction source/content tranche
C. Runtime dynamism schema decisions
D. Complex + unknown style normalization
E. Primitive field-coverage closure
F. Descriptor expansion triage
G. Schema/API auto-doc infrastructure
H. QA, docs, refactor, final readiness recommendation
```

---

# Lane A — Offender ledger + readiness classification

## Objective

Produce a complete offender ledger, not just grouped counts.

Every outstanding offender must receive a concrete disposition.

## Required command

Extend or use `schema-readiness` to emit full offender records:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json \
  --include-offenders
```

Use a different flag name only if there is already an established CLI convention.

## Required offender fields

Each offender record should include:

```text
legacyPath
family
legacyRecipeName
currentStatus
blockerKind
schemaReadinessBlocking
recommendedDisposition
recommendedNextPacket
confidence
candidateCanonicalPath
canonicalExists
requiredDescriptorIds[]
missingDescriptorIds[]
requiredSourceIds[]
missingSourceIds[]
unsupportedInputFields[]
holdbackReason
notes[]
```

## Required blocker kinds

Use concrete categories:

```text
schemaModel
sourceDescriptor
contentDescriptor
descriptorPack
playerAdapter
fieldCoverage
lifecycleSemantics
bindingSemantics
valueSourceSemantics
sceneSemantics
motionTimingSemantics
guiHumanReview
backendRenderer
ownerPolicyHoldback
duplicateOrVariant
oracleOnly
unknown
```

## Required dispositions

```text
implementNow
addDescriptor
addPlayerAdapter
addCanonicalFixture
deferForSchemaDecision
deferForSourceDecision
deferForDescriptorDecision
deferForBackend
deferForGuiReview
markOracleOnly
markDuplicateVariant
holdBackProblematic
requiresArchitectDecision
```

## Hard requirement

Do not leave broad buckets untouched.

Specifically:

```text
73 complex ownerAudit records must be split into concrete clusters.
5 unknown style records must be individually classified.
67 source/content blockers must be split into source/content/scene/asset/procedural/oracle categories.
72 schema blockers must be split into lifecycle, binding, value-source, motion/easing, signal, scene, or loopback decisions.
4 field-coverage blockers must receive exact recommended fixes or exact deferrals.
```

## Acceptance

```text
- offenders[] covers all non-ready records or clusters.
- unknownRecords is 0, or every unknown path is listed with a reason.
- ownerAudit is no longer used as a vague fallback.
- The report states exactly what remains before canDeclareSchemaReady=true.
```

---

# Lane B — Low-friction source/content tranche

## Objective

Proceed with source/content items that are clear and hold back ambiguous ones.

## Low-friction work to attempt now

### 1. Add a canonical `source.text` fixture

Suggested path:

```text
../tui-vfx-recipes/recipes/v3.1/debug_recipes/sources/source_text_basic.json
```

Only add it if all are true:

```text
source.text descriptor exists,
player adapter handles text/width/height honestly,
validate-recipe passes,
fixture-qc passes,
primitive field/source coverage remains honest,
no schema decision is required.
```

### 2. Decide whether `source.ansi` is low-friction

The lane should assess whether `source.ansi` can be descriptor-only or fixture-backed without inventing runtime semantics.

Possible outcomes:

```text
implementNow
addDescriptorOnly
deferForSourceDecision
holdBackProblematic
```

### 3. Classify image/procedural/command-capture

Do not implement unless clearly bounded.

Expected likely dispositions:

```text
source.image                  -> deferForSourceDecision or backend/asset policy
source.procedural.*            -> deferForSourceDecision
source.commandCaptureArtifact  -> markOracleOnly / offline authoring artifact only
content.typewriter             -> contentDescriptor decision, not source.text overload
content.marquee                -> contentDescriptor or runtime value-source decision
content.cellMotion             -> motion/content boundary decision
```

## Required source/content decision table

Create or update:

```text
docs/new_kernel/K2_12_SOURCE_CONTENT_DECISION_TABLE.md
```

Each row:

```text
candidate
representativeLegacyPaths[]
recordsAffected
decision
schemaImpact
descriptorImpact
playerImpact
holdbackAllowed
blocksSchemaReadiness
confidence
```

## Acceptance

```text
- source.text canonical fixture exists and passes, or is explicitly blocked.
- all 67 sourceBlockedRecords are classified into concrete subcategories.
- command capture remains offline/oracle-only; no runtime command execution.
- source.card is not overloaded to represent all content.
```

---

# Lane C — Runtime dynamism schema decisions

## Objective

Make concrete recommendations for the 60-ish true runtime dynamism blockers.

Scope:

```text
bindable_rates        8
event_driven_dwell    3 blockers
signals               5
easings              29
motion_routes         5
value-source filters  3
scene lifecycle       1
scene binding overlap 6
loopback              3
```

## Required document

Create:

```text
docs/new_kernel/K2_12_RUNTIME_DYNAMISM_DECISION_MATRIX.md
```

## Required distinctions

The matrix must preserve these boundaries:

```text
Trigger ≠ Gate
Trigger ≠ Binding
Trigger ≠ Loopback
Lifecycle trigger ≠ effect-local schedule
Signal value source ≠ signal generator
Binding declaration ≠ binding execution
Parameter override ≠ host signal
Motion route ≠ shader/sampler effect
Easing ≠ lifecycle duration
Loopback demo ≠ canonical runtime contract
```

## Required per-cluster output

```text
cluster
recordCount
representativeLegacyPaths[]
current v3.1 support
recommended decision
schema impact
descriptor impact
player impact
holdback disposition
blocksSchemaReadiness
next packet
confidence
```

## Expected decision posture

This lane may recommend additive schema changes, but should not implement broad runtime semantics unless they are already obviously supported.

Likely dispositions:

```text
event dwell bool trigger       -> already supported / extend predicate carefully
integer/text dwell demos       -> trigger predicate/value-source decision
bindable rates                 -> binding execution/parameter contract decision
signals                        -> signal source/generator boundary decision
easings                        -> motion timing descriptor decision
motion routes                  -> host motion descriptor boundary decision
sampled-surface filter values  -> value-source semantics decision
loopback                       -> demo-layer/oracle-only holdback
```

## Acceptance

```text
- Runtime blockers are no longer one generic schema bucket.
- Each cluster has a concrete recommendation.
- Low-risk decisions are identified for implementation in a later packet.
- Problematic items are held back explicitly.
```

---

# Lane D — Complex + unknown style normalization

## Objective

Break the largest ambiguity bucket.

Scope:

```text
complex: 73 ownerAudit records
styles: 5 unknown records
```

## Required work

Review every complex and unknown style record and classify into concrete clusters.

Allowed complex clusters:

```text
descriptorComposition
sourceContentPipeline
sceneLocalPipeline
runtimeDynamism
backendRenderer
guiHumanReview
oracleOnly
duplicateOrVariant
schemaModel
```

Allowed unknown style clusters:

```text
styleDescriptorNeeded
scopeVocabularyNeeded
bindingValueSourceNeeded
adapterFieldCoverage
oracleOnly
duplicateOrVariant
```

## Required deliverable

Create:

```text
docs/new_kernel/K2_12_COMPLEX_STYLE_NORMALIZATION_REPORT.md
```

Include:

```text
all complex records or clusters
all unknown style records individually
recommended disposition
whether it blocks schema readiness
next packet
confidence
```

## Hard requirement

Do not leave “complex ownerAudit” as a single blocker.

## Acceptance

```text
- complex ownerAudit count is reduced to 0 or replaced by named holdback clusters.
- 5 unknown style records are individually classified.
- Schema-readiness report no longer has vague complex/style unknowns.
```

---

# Lane E — Primitive field-coverage closure

## Objective

Resolve or hold back the 4 precise field-coverage blockers.

Current blockers:

```text
shader_linear_gradient_diagonal.json                 unsupported: gradient
shader_linear_gradient_background_channel.json       unsupported: gradient
shader_linear_gradient_apply_to_both.json            unsupported: gradient, applyTo
shader_border_sweep_position_binding.json           unsupported: position
```

## Required analysis

For each field:

```text
gradient
applyTo
position
```

determine:

```text
is this an additive descriptor input?
is this player adapter support?
is this a value-source/binding issue?
is this legacy-only/oracle-only?
does it block schema readiness?
```

## Low-friction implementation allowed

Proceed if bounded:

```text
applyTo for shader.linearGradient may be safe if it mirrors existing channel/application semantics.
gradient may be safe if it maps cleanly to existing startColor/endColor/colorSpace model or explicit stops.
position for borderSweep should probably be held behind binding/value-source semantics if dynamic.
```

Do not fake field coverage by simply listing the fields as handled.

## Acceptance

```text
- 4 field blockers are either fixed or explicitly held back.
- primitive-field-coverage remains zero-gap for canonical corpus.
- schema-readiness no longer treats these as unresolved vague blockers.
```

---

# Lane F — Descriptor expansion triage

## Objective

Separate descriptor-only migration work from schema readiness.

K2.11 lists 151 descriptor-blocked records. These should not necessarily prevent schema readiness if they need descriptor vocabulary but not schema changes.

## Required work

Create a ranked descriptor expansion queue:

```text
docs/new_kernel/K2_12_DESCRIPTOR_EXPANSION_QUEUE.md
```

Buckets:

```text
lowFrictionNow
needsDescriptorDecision
needsAdapterDecision
needsFieldSemantics
holdBackProblematic
oracleOnly
```

Families:

```text
filters
masks
samplers
styles
shaders
```

## Required per candidate

```text
descriptorCandidate
families
recordsAffected
representativeLegacyPaths[]
requiredInputs[]
knownPlayerSupport
fieldCoverageRisk
schemaImpact
recommendedPacket
confidence
```

## Low-friction descriptor additions allowed

Only proceed if all are true:

```text
descriptor semantics are clear,
inputs are known,
player adapter can handle fields honestly,
canonical fixture can be added safely,
no schema/model decision is needed.
```

Otherwise put it in the queue.

## Acceptance

```text
- 151 descriptor-blocked records are ranked.
- At least 5 low-friction candidates are identified or explicitly rejected with reasons.
- Descriptor-only blockers are separated from true schema blockers.
```

---

# Lane G — Schema/API auto-doc infrastructure

## Objective

Stop relying only on handwritten docs. Determine and strengthen the schema/API documentation generation workflow.

## Required investigation

Find existing workflows for:

```text
schemars JSON schema generation
rustdoc-generated API docs
descriptor pack docs
report schema docs
contract CLI schema output
embedded comments / rustdoc descriptions
```

Search likely areas:

```text
crates/tui-vfx-contract
crates/tui-vfx-contract-cli
docs/new_kernel
xtask
Justfile
tests/test_schema_generation.rs
```

## Required output

Create:

```text
docs/new_kernel/K2_12_SCHEMA_API_DOC_INFRA_REPORT.md
```

It must include:

```text
existing generation commands
missing generation commands
where schemas are emitted
which types have schemars/rustdoc coverage
which report schemas are undocumented
recommended automation path
exact commands to regenerate docs
```

## Implementation allowed

Add or improve the smallest command/workflow that can generate:

```text
canonical v3.1 JSON schema
contract API summary
descriptor pack summary
player report schema summary, if feasible
```

Do not overbuild a doc generator if a good path already exists.

## Acceptance

```text
- Existing schema generation workflow is documented.
- Missing pieces are listed.
- At least one generation command is run or proposed concretely.
- Touched public structs get rustdoc/schemars improvements where appropriate.
```

---

# Lane H — QA, docs, refactor, and final readiness recommendation

## Objective

Consolidate all lane outputs into a clear architectural recommendation.

## Required docs

Create:

```text
docs/new_kernel/K2_12_SCHEMA_LOCK_DECISION_REPORT.md
docs/new_kernel/K2_12_LOW_FRICTION_BURN_DOWN_REPORT.md
docs/new_kernel/PHASE_K2_12_SCHEMA_LOCK_STATUS_MEMO_TO_ARCHITECT.md
```

## Status memo must include

```text
rolling context
executive summary
before/after schema-readiness numbers
low-friction items implemented
problematic items held back
all outstanding blockers by exact cluster
whether 100% schema readiness can now be declared
if not, exact remaining blockers
recommended next packet
verification matrix
refactor/rustdoc/schemars work performed
legacy root mutation status
canonical fixture additions, if any
```

## Required readiness conclusion format

The memo must include one of:

```text
SCHEMA READINESS DECLARATION: APPROVED
```

or:

```text
SCHEMA READINESS DECLARATION: NOT YET
Remaining blockers:
  1. ...
  2. ...
Next packet required:
  ...
```

No vague conclusion.

---

# Optional fixture additions

Allowed in K2.12 only for low-friction items.

Rules:

```text
- descriptor already exists or is added with clear semantics,
- player adapter handles all authored fields,
- primitive-field-coverage remains honest,
- fixture-qc remains pass,
- no schema/model ambiguity,
- no legacy root mutation.
```

Likely safe candidate:

```text
sources/source_text_basic.json
```

Potentially safe only after analysis:

```text
source_ansi_basic.json
linear_gradient_apply_to_channel fixture
simple additional filter/mask descriptor fixture
```

Hold back anything involving:

```text
runtime command execution
image asset resolution
procedural source registry ambiguity
dynamic position binding
loopback runtime behavior
complex full-pipeline parity
shadow/subcell backend rendering
```

---

# TDD requirements

Start with RED tests for:

```text
schema-readiness --include-offenders emits offender records
schema-readiness offender records include blockerKind and recommendedDisposition
complex records are not left as generic ownerAudit
unknown style records are individually listed
source.text fixture validates and appears in inventory if added
field coverage blockers are represented with exact field names
schema/API doc workflow report exists
```

Regression tests:

```text
legacy debug_recipes root remains unmodified
existing fixture-qc still passes canonical corpus
primitive-field-coverage remains honest
primitive-adapter-gap remains green for canonical corpus
migration-mapping-batch --recursive still works
schema-readiness baseline command still works
```

---

# Acceptance criteria

## Required

```text
- Full offender ledger exists or schema-readiness emits equivalent detail.
- Every non-ready record is classified or clustered with a concrete disposition.
- 73 complex records are no longer a vague ownerAudit bucket.
- 5 unknown style records are individually classified.
- 67 source/content blockers are split into concrete categories.
- 72 schema blockers are split into concrete runtime/schema decision clusters.
- 4 field-coverage blockers are fixed or explicitly held back.
- Low-friction source.text work is implemented or blocked with a concrete reason.
- Descriptor-only blockers are separated from schema-readiness blockers.
- Schema/API doc infrastructure is assessed and documented.
- Final memo states whether 100% schema readiness can be declared.
- No legacy recipe files are modified.
```

## Preferred

```text
- source.text canonical fixture added and fixture-qc passing.
- source.ansi decision made, even if deferred.
- readiness percentage improves materially.
- unknownRecords becomes 0.
- untriaged ownerAudit becomes 0.
- at least 5 low-friction descriptor candidates are identified for K2.13.
```

## Explicit holdback acceptance

Problematic items may be held back and should **not** block low-friction progress when they are classified as:

```text
oracleOnly
backendRenderer
guiHumanReview
ownerPolicyHoldback
futureSchemaCandidate
futureDescriptorCandidate
duplicateOrVariant
```

The holdback must include:

```text
reason
record count
representative paths
what would unblock it
whether it blocks schema readiness
```

---

# Stop conditions

Stop and report rather than forcing implementation if:

```text
a fixture requires guessing legacy intent,
a source requires runtime command execution,
a binding decision blurs parameter/signal/trigger semantics,
loopback would become canonical runtime data,
field coverage can only pass by pretending a field is handled,
complex migration needs compositor semantics,
source image/procedural policy is unclear,
schema doc generation would require broad unrelated refactors.
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
  --package tui-vfx-contract-cli \
  -- --check

cargo clippy \
  -p tui-vfx-player \
  -p tui-vfx-player-cli \
  -p tui-vfx-player-ui \
  -p tui-vfx-contract-cli \
  --all-targets -- -D warnings
```

## Tests

```bash
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test -p tui-vfx-player-ui
cargo test -p tui-vfx-contract-cli
cargo test --workspace
```

Use `cargo nextest` where available, but `cargo test` must remain acceptable.

## Report gates

```bash
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- fixture-qc \
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

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json

cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

If implemented:

```bash
cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json \
  --include-offenders
```

## Cleanliness

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

Legacy root must remain untouched.

---

# Expected status memo

Return:

```text
docs/new_kernel/PHASE_K2_12_SCHEMA_LOCK_STATUS_MEMO_TO_ARCHITECT.md
```

The memo must include:

```text
- rolling context
- executive summary
- before/after schema-readiness numbers
- direct statement on 100% schema readiness
- low-friction items completed
- problematic items held back
- exact outstanding blockers
- offender classification summary
- source/content decisions
- runtime dynamism decisions
- complex/style normalization results
- primitive field-coverage decisions
- descriptor expansion queue
- schema/API doc infrastructure status
- verification matrix
- touched-file refactor/rustdoc/schemars notes
- recipe repo mutation status
- recommended next packet
```

Recommended next packet should be one of:

```text
K2.13 — schema-lock candidate and final blocker closure
K2.13 — descriptor expansion tranche from low-friction queue
K2.13 — runtime dynamism additive schema implementation
K3.0 — studio/control-surface preflight, only if schema readiness is declared or very close
```

---

# Architect expectation

K2.12 should not come back saying only:

```text
source/content needs decisions
runtime dynamism needs decisions
complex needs audit
```

It must come back saying:

```text
Here are the exact items.
Here are the decisions we recommend.
Here is what we implemented.
Here is what we held back.
Here is what still blocks schema readiness.
Here is whether we can declare schema readiness now.
```

That is the level of decisiveness required to move toward schema lock, recipe migration, visual confirmation, and eventually the dynamic studio UI.
