<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_10.md</FILE> - <DESC>Architect response to K2.10 corpus mapping and K2.11 schema-readiness packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.10 closeout: record architect acceptance and next schema-readiness packet.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add K2.10 architect acceptance and K2.11 recommendation.</CLOG> -->

## Current schema-readiness answer

**We are not yet at 100% schema readiness for the existing debug recipe migration.** K2.10 made that visible rather than hiding it.

The clearest current evidence is:

```text
Legacy records mapped:          603
canonicalExists:                 21
candidateReady:                   0
schemaDecisionNeeded:            72
sourceDecisionNeeded:            67
descriptorDecisionNeeded:       151
blockedByFieldCoverage:           4
ownerAuditNeeded:               280
notYetClassified:                 5
```

My current architectural position is:

```text
We can only claim 100% schema readiness after every legacy debug recipe is either:
  1. canonical v3.1 and fixture-QC passing,
  2. ready for canonical migration with no schema/source/model blockers,
  3. explicitly duplicate/variant,
  4. explicitly oracle-only / out-of-runtime-scope,
  5. or explicitly deferred behind a non-schema blocker such as player adapter, descriptor expansion, GUI review, or backend rendering.
```

The important distinction is that **descriptor readiness, adapter readiness, visual readiness, and schema readiness are not the same thing**.

Right now, the known blockers that prevent a 100% schema-readiness claim are:

```text
72 schemaDecisionNeeded records
67 sourceDecisionNeeded records
4 blockedByFieldCoverage records
5 notYetClassified records
280 ownerAuditNeeded records that may hide additional schema/model blockers
```

So the next packet must not just continue migrating. It must produce a **schema-readiness ledger** that lists every outstanding blocker and separates:

```text
true schema/model blockers
descriptor-only blockers
source descriptor blockers
player adapter blockers
field coverage blockers
GUI/human-review blockers
backend/compositor blockers
oracle-only records
duplicates/variants
unknowns
```

My conditional forecast is:

```text
K2.11 should produce the first credible 100% schema-readiness forecast.

If K2.11 successfully turns ownerAuditNeeded / notYetClassified into explicit blocker buckets,
then K2.12–K2.13 can burn down the true schema/source/model blockers.

A 100% schema-readiness declaration is plausible only after:
  - source/content descriptor policy is settled,
  - lifecycle/signal/binding/value-source semantics are settled,
  - scene/source-local pipeline semantics are settled,
  - remaining field-coverage blockers are resolved or explicitly deferred as non-schema,
  - and owner-audit records no longer hide unknown schema decisions.
```

I would not honestly claim “100% schema readiness” before that ledger exists.

---

# Architect response to K2.10

## Review verdict

**ACCEPT.**

K2.10 did the right thing: it widened migration mapping across the full 603-record legacy debug corpus without bulk migration, kept the report schema stable, preserved the legacy root as read-only evidence, and produced the backlog board we needed.

The next phase should be larger than the earlier packets, because we now have enough tooling and the orchestrator can support 6–8 parallel lanes. The packet below explicitly asks the implementer to assess the full situation and list all outstanding blockers as a primary deliverable.

---

# Phase K2.11 — Schema Readiness Ledger + Source/Content Descriptor Pilot

## Executive goal

Turn the K2.10 backlog into a **schema-readiness control surface** and begin the highest-confidence source/content descriptor work.

The packet has two main outcomes:

```text
1. A defensible answer to:
   “What is blocking 100% schema readiness for existing debug recipe migration?”

2. A bounded implementation pilot for source/content descriptors where model fit is already clear,
   without forcing uncertain legacy recipes into today’s v3.1 shape.
```

This packet should not attempt bulk migration. It should classify and reduce blockers.

Core rule:

```text
Do not make the corpus look ready by weakening classifications.
If a blocker is uncertain, name the uncertainty and assign it to the correct lane.
```

---

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
K2.10 corpus-wide migration mapping + backlog board + render backend boundary note
```

Current corpus state:

```text
canonical v3.1 debug fixtures: 26
legacy debug recipe records:   603
candidateReady after K2.10:    0
```

Current known decision backlog:

```text
schemaDecisionNeeded:       72
sourceDecisionNeeded:       67
descriptorDecisionNeeded:  151
blockedByFieldCoverage:      4
ownerAuditNeeded:          280
notYetClassified:            5
```

---

# Work model: 8 parallel lanes

The implementer should use sub-agents where useful.

```text
A. schema-readiness ledger/report
B. source descriptor pilot
C. content/source-family audit
D. lifecycle/signal/binding/value-source audit
E. primitive field-coverage closure
F. owner-audit triage and blocker normalization
G. studio/control-surface preflight
H. docs, refactor, rustdoc/schemars, QA
```

Every lane must follow the standing file-touch rule:

```text
For every touched file, look for refactoring opportunities that reduce complexity,
improve maintainability/readability, and add rustdoc comments and schemars details
where appropriate.
```

Do not create churn for its own sake. But when a touched file exposes unclear DTOs, missing comments, duplicated parsing logic, oversized functions, stale wording, or weak schema annotations, clean it within scope.

---

# Lane A — Schema-readiness ledger and blocker report

## Objective

Create a first-class report that answers:

```text
What blocks 100% schema readiness for existing debug recipe migration?
```

This can be implemented as either:

```text
tui-vfx-player-cli schema-readiness
```

or as an additive mode on `migration-mapping-batch`, if that is cleaner:

```text
tui-vfx-player-cli migration-mapping-batch --schema-readiness
```

Prefer a distinct command if it keeps code clearer.

## Required command shape

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

If the command name is changed, document why.

## Report schema

Use a new schema label:

```text
v3.1.player.schemaReadiness.1
```

Top-level fields should include:

```text
schemaVersion
legacyRoot
v31Root
descriptorPacks
summary
families[]
blockers[]
readinessMilestones[]
warnings[]
errors[]
```

The `summary` must include:

```text
totalLegacyRecords
schemaReadyRecords
schemaBlockedRecords
sourceBlockedRecords
descriptorBlockedRecords
adapterBlockedRecords
fieldCoverageBlockedRecords
ownerAuditRecords
oracleOnlyRecords
duplicateOrVariantRecords
unknownRecords
estimatedSchemaReadinessPercent
canDeclareSchemaReady
```

`canDeclareSchemaReady` must be `false` until there are zero true schema/source/model unknowns.

## Blocker record shape

Each blocker must include:

```text
id
family
representativeLegacyPaths[]
statusFromMigrationMapping
blockerKind
blockingDecision
recommendedNextPacket
confidence
isSchemaReadinessBlocking
notes[]
```

Allowed `blockerKind` values:

```text
schemaModel
sourceDescriptor
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
ownerAudit
duplicateOrVariant
oracleOnly
unknown
```

## Hard requirement

The report must list **all outstanding blockers**, not just top examples.

It may group repeated records, but the grouping must preserve counts and representative paths.

## Acceptance

```text
- Report runs over all 603 legacy records.
- `notYetClassified` is reduced to zero or explicitly justified in blockers[].
- Every K2.10 status bucket maps to a blockerKind.
- The report states whether 100% schema readiness can be declared.
- The report gives a next-packet sequence to reach schema readiness.
```

---

# Lane B — Source descriptor pilot

## Objective

Begin the source/content backlog with the highest-confidence source descriptor work, without touching command execution or forcing broad content semantics.

Start with the simplest safe candidates:

```text
source.text
source.ansi
```

Assess but do not necessarily implement:

```text
source.image
procedural sources
source.commandCaptureArtifact
source.card expansion/versioning
```

## Required assessment

Before implementation, the agent must write or generate a source decision table:

```text
source id candidate
legacy evidence paths
contract type support exists?
descriptor shape clear?
player adapter exists?
field coverage known?
safe to implement in K2.11?
remaining blocker
```

## Implementation guidance

`source.text` is likely the safest because K0 already has a `source.text` adapter path in the player, but the descriptor pack currently only exposes `source.card`.

`source.ansi` may be descriptor-only or adapter-backed depending on existing contract/player support. Do not claim rendered support unless the player actually handles it.

`source.image` should probably remain a candidate unless asset resolution and player evidence are explicit.

`source.commandCaptureArtifact` must remain offline/oracle policy only. No runtime command execution.

## Files likely touched

Likely, but not mandatory:

```text
descriptors/v3.1/packs/primitive.json
crates/tui-vfx-player/src/fnc_render_scene.rs
crates/tui-vfx-player/src/fnc_inventory_recipe_file.rs
crates/tui-vfx-player/src/fnc_inventory_recipe_paths.rs
crates/tui-vfx-player-cli tests
docs/new_kernel/*
```

If contract DTOs are touched, add rustdoc/schemars details.

## Optional canonical fixtures

Add only if all are true:

```text
- descriptor exists,
- player adapter handles every authored source input,
- validation passes,
- fixture-qc passes,
- primitive/source field coverage stays zero-gap,
- no schema decision is required.
```

Likely safe fixture candidate:

```text
recipes/v3.1/debug_recipes/sources/source_text_basic.json
```

Only add it if the recipe repo is in scope for the implementer and the fixture is descriptor-pack-backed.

## Acceptance

```text
- `source.text` is either implemented and fixture-backed, or explicitly blocked with reason.
- `source.ansi` is either implemented, descriptor-only with honest unsupported status, or explicitly blocked.
- Source descriptor candidates are documented with confidence and next action.
- No runtime command execution is introduced.
```

---

# Lane C — Content and scene-source family audit

## Objective

Separate content effects, source descriptors, and scene semantics so we do not blur the model.

The K2.10 report shows:

```text
content: 111 records, sourceDecisionNeeded 66, ownerAuditNeeded 45
scene:    19 records, schemaDecisionNeeded 19
fixtures:  1 record, sourceDecisionNeeded 1
```

This lane must answer:

```text
Which content records are actually source descriptor needs?
Which are content transform/effect descriptor needs?
Which are scene/source-local-pipeline schema needs?
Which are oracle-only?
```

## Required output

Create a section in the K2.11 status memo and preferably a standalone doc:

```text
docs/new_kernel/K2_11_SOURCE_CONTENT_SCENE_DECISION_TABLE.md
```

The table should include:

```text
family
recordCount
subclass
representativeLegacyPaths[]
requiredSourceDescriptor
requiredContentDescriptor
requiredSceneDecision
recommendedNextPacket
confidence
```

## Candidate concepts to assess

```text
source.text
source.ansi
source.image
source.procedural.*
content.typewriter
content.marquee
content.cellMotion
source-local pipeline
scene element placement
scene layer visibility/overflow
asset resolver seam
offline command capture artifact policy
```

## Stop conditions

Stop and classify instead of implementing if:

```text
- a content effect requires runtime schedule semantics,
- scene-local pipeline semantics are unclear,
- procedural source identity is ambiguous,
- asset resolution would require a new runtime resolver,
- command capture would require runtime command execution.
```

---

# Lane D — Lifecycle, signal, binding, and value-source schema audit

## Objective

Audit the 72 `schemaDecisionNeeded` records and produce a blocker ledger for lifecycle/value-source readiness.

This is central to the user’s 100% schema-readiness question.

## Scope

```text
event_driven_dwell
bindable_rates
easings
motion_routes
signals
loopback
value-source-shaped filter records
```

## Required distinctions

Preserve these vocabulary boundaries:

```text
Trigger ≠ Gate
Trigger ≠ Binding
Trigger ≠ Loopback
Lifecycle trigger ≠ effect-local schedule
Signal source ≠ signal generator
Parameter contract ≠ runtime binding execution
Motion route ≠ shader/sampler descriptor unless explicitly modeled that way
```

## Required output

Add a lifecycle/schema blocker table to the schema-readiness report and memo:

```text
blocker
recordCount
families
representativeLegacyPaths[]
current v3.1 support
missing schema/model decision
recommendedNextPacket
confidence
```

## Specific blockers to assess

```text
filter.dim sampled-surface value sources
integer/text event dwell demos
signal generator recipes
bindable rate recipes
easing recipes
motion route recipes
loopback demos
parameter override semantics
binding execution semantics
```

## Implementation rule

Do not add new schema semantics in this lane unless the decision is obvious and bounded.

This lane is primarily an assessment and readiness ledger lane.

---

# Lane E — Primitive field-coverage closure

## Objective

Resolve or precisely classify the four field-coverage blockers from K2.10:

```text
shaders/primitives/shader_linear_gradient_diagonal.json
  unsupported field: gradient

shaders/primitives/shader_linear_gradient_background_channel.json
  unsupported field: gradient

shaders/primitives/shader_linear_gradient_apply_to_both.json
  unsupported fields: applyTo, gradient

shaders/compositions/shader_border_sweep_position_binding.json
  unsupported field: position
```

## Required assessment

For each blocker, decide:

```text
- Can this field map to the existing descriptor contract?
- Does the descriptor need additive inputs?
- Does the player adapter already have enough data to support it?
- Is it actually a binding/value-source semantic blocker?
- Should the legacy record remain oracle-only?
```

## Possible implementation

If safe and bounded:

```text
- add additive descriptor inputs,
- update adapter field handling,
- update primitive-field-coverage semantics,
- add tests.
```

If not safe:

```text
- keep blockedByFieldCoverage,
- assign blockerKind precisely,
- document next packet.
```

## Acceptance

```text
- The four blockers are no longer vague.
- If implemented, primitive-field-coverage remains zero-gap for canonical corpus.
- If deferred, the schema-readiness ledger explains whether each is schema-blocking or adapter/descriptor-blocking.
```

---

# Lane F — Owner-audit triage and blocker normalization

## Objective

Reduce the 280 `ownerAuditNeeded` records into meaningful blocker buckets without overclaiming readiness.

Current broad buckets:

```text
filters ownerAuditNeeded 45
masks ownerAuditNeeded 15
samplers ownerAuditNeeded 6
shaders ownerAuditNeeded 54
styles ownerAuditNeeded 14
content ownerAuditNeeded 45
loopback ownerAuditNeeded 3
complex ownerAuditNeeded 83
shadows ownerAuditNeeded 9
subcell_shapes ownerAuditNeeded 5
other ownerAuditNeeded 1
```

## Required output

For each family, group owner-audit records into:

```text
descriptorPack
schemaModel
sourceDescriptor
playerAdapter
guiHumanReview
backendRenderer
oracleOnly
duplicateOrVariant
unknown
```

## Hard requirement

Do not leave `ownerAuditNeeded` as a catch-all if there is enough evidence to classify it.

But do not fake precision. If the evidence is truly ambiguous, keep `ownerAudit` and explain why.

## Acceptance

```text
- Owner-audit bucket has meaningful subcounts.
- Unknown records are minimized and justified.
- Schema-readiness report reflects owner-audit uncertainty honestly.
```

---

# Lane G — Studio/control-surface preflight

## Objective

Seed the future studio reach goal without blocking migration.

The long-term goal is:

```text
Load a recipe in the UI,
auto-generate sliders/input boxes from recipe + descriptor metadata,
adjust settings dynamically,
and see the visual result.
```

K2.11 should not build the full studio. It should add a **report-only control-surface preflight** if capacity allows.

## Proposed command

```bash
cargo run -q -p tui-vfx-player-cli -- control-surface \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/filters/filter_tint.json"
```

Use another name if clearer, but document it.

## Proposed schema

```text
v3.1.player.controlSurface.1
```

Top-level fields:

```text
schemaVersion
recipePath
recipeId
controls[]
signals[]
parameters[]
warnings[]
errors[]
```

Each control:

```text
controlId
label
targetPath
sourceKind
valueKind
currentValue
defaultValue
range
allowedValues
unit
semantic
runtimeMutability
bindable
uiHint
```

## Source of truth

The control surface must derive from:

```text
RecipeDocument
DescriptorPack / DescriptorCatalog
SourceDescriptor inputs
EffectDescriptor inputs
Graph signals/parameters/bindings
```

No hard-coded UI controls except type-to-widget hints:

```text
number/integer + range -> slider/spinbox candidate
boolean -> checkbox candidate
enum -> select candidate
color -> color picker candidate
text/string -> input candidate
```

## Acceptance

```text
- Report exists or is explicitly deferred.
- It works on at least baseline, filter_tint, mask_wipe, shader_linear_gradient.
- It does not mutate recipes.
- It does not add GUI sliders yet.
- It does not invent runtime binding semantics.
```

This lane is a reach goal. If it threatens schema-readiness work, defer it and document as K3.0 prep.

---

# Lane H — Docs, refactor, rustdoc/schemars, and QA

## Objective

Consolidate the packet into clear docs and keep touched code maintainable.

## Required docs

Create:

```text
docs/new_kernel/K2_11_SCHEMA_READINESS_LEDGER.md
docs/new_kernel/K2_11_SOURCE_CONTENT_SCENE_DECISION_TABLE.md
docs/new_kernel/K2_11_STUDIO_CONTROL_SURFACE_PREFLIGHT.md
docs/new_kernel/PHASE_K2_11_SCHEMA_READINESS_STATUS_MEMO_TO_ARCHITECT.md
```

If the control-surface lane is deferred, the studio doc should say so and explain the next trigger.

## Status memo must include

```text
- executive summary
- explicit answer: can we declare 100% schema readiness?
- current schema-readiness percent and caveats
- full outstanding blocker list or grouped ledger
- per-family blocker counts
- source/content descriptor pilot results
- lifecycle/value-source/schema blocker results
- primitive field-coverage decisions
- owner-audit triage results
- optional fixture additions, if any
- control-surface preflight status
- verification matrix
- legacy root mutation status
- recommended next packet
```

## Refactor and documentation rule

For every touched Rust file:

```text
- reduce duplicated parsing/reporting logic where practical,
- keep functions OFPF-sized where practical,
- add rustdoc for public DTOs/functions,
- add or update schemars annotations/details where the touched type participates in schema generation,
- remove stale wording from docs/tests,
- avoid broad unrelated rewrites.
```

---

# TDD requirements

Start with RED tests or failing snapshots for:

```text
schema-readiness command emits schemaVersion v3.1.player.schemaReadiness.1
schema-readiness command lists blockers for schemaDecisionNeeded records
schema-readiness command does not allow canDeclareSchemaReady=true on current corpus
schema-readiness command maps sourceDecisionNeeded records to sourceDescriptor blockers
schema-readiness command maps field-coverage blockers explicitly
source.text descriptor validation path, if implemented
source descriptor coverage in inventory/mapping reports, if implemented
control-surface report emits controls from descriptor metadata, if implemented
```

Regression tests:

```text
migration-mapping-batch --recursive still works
migration-mapping-batch --family masks still works
fixture-qc still passes canonical corpus
primitive-field-coverage still passes canonical corpus
primitive-adapter-gap still passes canonical corpus
render-recipe/render-frame still pass canonical corpus
legacy debug_recipes root remains unmodified
```

---

# Acceptance criteria

## Required

```text
- A schema-readiness report or equivalent ledger exists.
- The ledger explicitly answers whether 100% schema readiness can be declared.
- All outstanding K2.10 blockers are listed or grouped with counts and representative paths.
- `notYetClassified` is reduced to zero or explicitly justified.
- Owner-audit records are triaged into more specific blocker kinds where possible.
- Source/content/scene decisions are documented.
- Lifecycle/signal/binding/value-source blockers are documented.
- The four field-coverage blockers are resolved or precisely classified.
- No legacy recipe files are modified.
- Existing canonical corpus gates remain green.
- Docs include a clear next-packet recommendation.
```

## Preferred

```text
- `source.text` descriptor is added and fixture-backed if safe.
- `source.ansi` is either implemented or precisely deferred.
- The schema-readiness report includes an estimated readiness percent with caveats.
- The report identifies the minimum packet sequence required before a 100% schema-readiness declaration.
- Control-surface preflight report is implemented or cleanly deferred with a K3.0 plan.
```

## Explicit stop conditions

Stop and report instead of forcing implementation if:

```text
- source descriptor semantics require runtime command execution,
- image/procedural sources require unresolved asset/runtime policy,
- lifecycle work blurs trigger/gate/binding/loopback vocabulary,
- control-surface work requires unapproved runtime mutation semantics,
- field coverage can only pass by pretending unsupported fields are handled,
- owner-audit records cannot be classified without human design judgment,
- broad schema changes would be needed across unrelated families.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

Format and lint:

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

Tests:

```bash
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test -p tui-vfx-player-ui
cargo test -p tui-vfx-contract-cli
cargo test --workspace
```

Core report gates:

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

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --family masks \
  --json
```

New K2.11 gates:

```bash
cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

If control-surface preflight is implemented:

```bash
cargo run -q -p tui-vfx-player-cli -- control-surface \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json"

cargo run -q -p tui-vfx-player-cli -- control-surface \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/filters/filter_tint.json"

cargo run -q -p tui-vfx-player-cli -- control-surface \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/masks/mask_wipe.json"
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

---

# Expected status memo

Return:

```text
docs/new_kernel/PHASE_K2_11_SCHEMA_READINESS_STATUS_MEMO_TO_ARCHITECT.md
```

The memo must include the direct answer:

```text
Can we declare 100% schema readiness for existing recipe migration?
```

Expected answer today is likely:

```text
No, but the blocker ledger now defines the remaining path.
```

If the agent believes the answer is “yes,” they must prove it with:

```text
- zero schema/model/source unknowns,
- zero notYetClassified records,
- all owner-audit records classified as non-schema or oracle-only,
- all source descriptor blockers resolved or explicitly non-schema,
- all field-coverage blockers resolved or explicitly non-schema,
- fixture-qc / validation / field coverage / adapter gap gates green.
```

Recommended next packet should be one of:

```text
K2.12 — lifecycle/signal/binding/value-source schema decision packet
K2.12 — source/content descriptor expansion tranche
K2.12 — primitive descriptor/field-coverage closure tranche
K3.0 — studio control-surface GUI pilot, only if schema/control-surface evidence is stable
```

Do not recommend compositor backend wiring unless K2.11 proves that schema/source/descriptor readiness is no longer the blocking path.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_10.md</FILE> - <DESC>Architect response to K2.10 corpus mapping and K2.11 schema-readiness packet</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
