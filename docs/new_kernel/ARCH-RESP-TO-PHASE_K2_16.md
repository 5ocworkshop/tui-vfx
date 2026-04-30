# Architect response to K2.15 and next work packet

## Review verdict

**ACCEPT, with scope caveat.**

K2.15 delivered the most important runtime movement we needed: `tui-vfx-player` is no longer purely `graph.order`-only. It now has topology-first execution, sequence value visibility, parallel snapshot/merge behavior, and deterministic conflict warnings. That materially advances the runtime path.

The caveat is that K2.15 did **not** materially burn down descriptor backlog. It added 10 fixtures and no new primitive descriptors. That was acceptable because the graph executor work was high-value, but the next packet must be much more aggressive about backlog burn-down and source/scene fidelity.

Current state after K2.15:

```text
canonical v3.1 fixtures:          67
validate-recipe:                  67 valid / 0 invalid
render-recipe:                    67 rendered / 0 unsupported / 0 errors
render-frame:                     67 rendered / 0 unsupported / 0 errors
fixture-qc:                       pass
primitive-field-coverage:         422 used / 422 handled / 0 unhandled
primitive-adapter-gap:            43 rendered / 0 unresolved
schema-readiness:                 canDeclareSchemaReady=true
explicitOwnerDecisionNeeded:      0
migration canonicalExists:        51
migration schemaDecisionNeeded:   90
descriptorDecisionNeeded:         113
sourceDecisionNeeded:             61
ownerAuditNeeded:                 280
```

The next packet should not reopen schema readiness. It should convert accepted schema into runtime/player/fixture evidence and aggressively classify or clear the remaining backlog.

---

# Phase K2.16 — Player Render IR, Scene/Source Fidelity, and Backlog Burn-Down

## Executive goal

K2.16 should make the player evidence path mature enough to support the next major migration wave and future backend/studio work.

The packet should deliver four concrete outcomes:

```text
1. Introduce a player-owned render IR that carries rows, styled cells, roles, provenance, graph diagnostics, sample-clock state, and value-bus evidence.

2. Improve scene/layer runtime fidelity: visibility predicates, layer-local pipelines, transparent/skip/clear behavior, and element-attributed diagnostics.

3. Burn down descriptor/source/content backlog with a large tranche of canonical fixtures and honest player adapters.

4. Normalize the remaining owner-audit backlog into exact path-level dispositions so we stop rediscovering the same uncertainty.
```

Use up to **10 parallel sub-agents**. This should be a high-throughput implementation packet, not another planning-only report.

---

## Rolling context to include in implementer memo

Completed and accepted:

```text
K2.13:
  schema decision readiness approved;
  canDeclareSchemaReady=true;
  scene, graph I/O, runtime dynamism, source/content, templates, and studio-control boundaries accepted.

K2.14:
  descriptor/adapter migration tranche 1;
  57 canonical fixtures;
  43 rendered effects;
  0 adapter gaps;
  0 field coverage gaps.

K2.15:
  player graph topology/value-bus execution started;
  67 canonical fixtures;
  graph sequence/parallel/player warnings implemented;
  scene local styled evidence improved;
  source evidence remains bounded.
```

Current durable rules:

```text
Legacy debug_recipes are read-only evidence.
Canonical fixtures live only under recipes/v3.1/debug_recipes.
Templates are mandatory but compile-time only.
Scene/element/layer support is core v3.1 work.
Player evidence is honest evidence, not visual parity.
UI must consume player evidence and must not construct compositor internals.
Compositor/backend work belongs behind a player/backend adapter seam.
```

---

# Non-negotiable constraints

## Legacy root remains read-only

Do not modify:

```text
../tui-vfx-recipes/recipes/debug_recipes
```

Canonical fixtures may be added or modified only under:

```text
../tui-vfx-recipes/recipes/v3.1/debug_recipes
```

## No false green

Do not reduce backlog counters by overclaiming. A field is handled only if an adapter consumes it or a report clearly marks it as held back.

Bad behavior:

```text
marking fields handled because they are listed
treating fallback text evidence as styled ANSI parity
treating missing-asset image fallback as image rasterization
treating procedural source fallback as full plugin support
turning legacy aliases into canonical v3.1 schema
```

## Refactor and documentation discipline

For every touched file:

```text
- reduce complexity where practical;
- improve naming and module boundaries;
- add or update rustdoc for public/semi-public APIs;
- add schemars descriptions when contract DTOs are touched;
- keep OFPF metadata current where applicable;
- avoid K2/K3 phase labels in durable API names, schema values, descriptor ids, or variable names.
```

---

# Work model: 10 parallel lanes

```text
A. Control counters and backlog normalization
B. Player render IR
C. Graph executor hardening
D. Scene/layer runtime fidelity
E. Source fidelity tranche
F. Content descriptor/adapter tranche
G. Filter/mask/sampler descriptor tranche
H. Shader/style descriptor tranche
I. Backend seam and holdback preflight
J. Studio control catalog + schema/API/docs gates
```

Each lane should return a short lane memo. The final orchestrator response should consolidate results into a single architect status memo.

---

# Lane A — Control counters and backlog normalization

## Objective

Stop repeating stale `ownerAuditNeeded=280` without action. Convert it into exact, path-level dispositions and make K2.16’s before/after impact measurable.

## Required baseline commands

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

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

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

## Implementation work

Update `migration-mapping-batch` or add a companion report so each remaining problematic legacy record has an exact disposition:

```text
canonicalExists
candidateReady
descriptorBacklog
sourceBacklog
contentBacklog
adapterBacklog
sceneRuntimeBacklog
graphRuntimeBacklog
backendHoldback
guiHumanReviewHoldback
oracleOnly
duplicateVariant
deprecatedLegacy
explicitOwnerDecisionNeeded
```

Do not hide old counters if they are still useful, but add a disposition-first summary so the architect can see what work is actually left.

## Required outputs

Create:

```text
docs/new_kernel/K2_16_BASELINE_AND_FINAL_COUNTERS.md
docs/new_kernel/K2_16_BACKLOG_NORMALIZATION_REPORT.md
```

The backlog normalization report must include:

```text
per-family disposition counts
top 50 remaining paths by priority
all explicitOwnerDecisionNeeded paths, if any
all holdback paths
all candidateReady paths
all descriptorBacklog paths grouped by descriptor family
all source/content backlog paths grouped by source/content kind
```

## Quantitative target

Aim for:

```text
ownerAuditNeeded raw/mapping count: materially reduced, ideally below 80
explicitOwnerDecisionNeeded: 0, unless a real new decision is found
canonicalExists: 51 -> 75+
descriptorDecisionNeeded: 113 -> below 80 if descriptor lanes succeed
sourceDecisionNeeded: 61 -> below 35 if source lanes succeed
```

If the raw `ownerAuditNeeded` field cannot be reduced because it is historical by design, add a new disposition summary and state that explicitly.

---

# Lane B — Player render IR

## Objective

Introduce a durable, player-owned render IR that becomes the central evidence object for future backend lowering, scene diagnostics, studio controls, and UI display.

K2.15 showed the need clearly:

```text
We need a player-owned render IR before backend/compositor lowering.
```

## Why this matters

Current evidence is split across:

```text
rows
styled cells
warnings
graph values
scene/layer data
sample request state
diagnostics
```

A future backend adapter, studio, and GUI cannot reliably consume a loose set of side channels. The player needs one render IR that can be serialized, inspected, and lowered.

## Suggested internal types

Names are suggestions:

```text
PlayerRenderIr
PlayerRenderSurface
PlayerRenderCell
PlayerRenderRole
PlayerRenderProvenance
PlayerRenderLayer
PlayerRenderElement
PlayerRenderDiagnostic
PlayerRenderGraphValueSnapshot
PlayerRenderClockSample
PlayerRenderWriteEvent
PlayerRenderChannel
PlayerRenderWritePolicy
PlayerRenderConflict
```

## Minimum IR content

The IR should carry:

```text
sample time / phase / normalized progress
final rows
styled cells
role map
cell provenance:
  source id
  element/layer id
  node id
  descriptor id
  scope
  local coordinate
  global coordinate
graph diagnostics
scene diagnostics
source diagnostics
graph value snapshot
write/conflict events
substrate/styleKnown/cellSource metadata
```

## CLI/report surface

Add a report command if feasible:

```bash
cargo run -q -p tui-vfx-player-cli -- render-ir \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json path/to/recipe.json
```

Suggested schema label:

```text
v3.1.player.renderIr.1
```

If a CLI command is too large, implement the IR internally and expose it through `render-frame` additively.

## Acceptance tests

Required tests:

```text
render IR preserves existing rows and renderHash
render IR carries styled cells
render IR carries role/provenance for source.card/source.text
render IR carries graph warnings from parallel conflict fixture
render IR carries graph value snapshot from sequence fixture
render IR carries element/layer provenance for scene fixture
render-frame remains backward-compatible
```

## Acceptance

* Existing `render-frame` output remains compatible.
* New IR does not replace existing reports abruptly.
* IR is player-owned and does not import compositor internals.
* IR is suitable as the input to a future backend adapter.

---

# Lane C — Graph executor hardening

## Objective

Move graph execution from “first useful slice” to “credible player runtime surface.”

K2.15 implemented topology, sequence, parallel, value bus, and warnings. K2.16 should harden missing diagnostics and real output publication.

## Target improvements

Implement or harden:

```text
real effect-output publication where adapters can expose outputs
missing graph value diagnostics when no fallback exists
kind mismatch diagnostics
graph value overwrite policy
parallel branch conflict policy field if already accepted
node id / descriptor id in graph diagnostics
graph-value report in render IR
nested topology regression tests
```

## Candidate fixtures

Harden or add fixtures under:

```text
recipes/v3.1/debug_recipes/complex/
```

Candidate names:

```text
graph_value_missing_input_diagnostic.json
graph_value_kind_mismatch_diagnostic.json
graph_parallel_conflict_last_writer.json
graph_nested_sequence_parallel_value_bus.json
graph_filter_to_mask_sourced_output.json
```

Only add diagnostic fixtures if validation/render reports can represent expected diagnostics cleanly.

## Acceptance

* Missing-value and kind-mismatch behavior is deterministic.
* Graph diagnostics are visible in render-frame/render-ir.
* Existing fixtures stay green.
* No fallback silently hides a missing required graph value.

---

# Lane D — Scene/layer runtime fidelity

## Objective

Implement the next meaningful slice of the accepted scene/element/layer model.

K2.15 status:

```text
local styled-cell evidence works
full visibility predicates incomplete
transparent/clear policy incomplete
full element diagnostics incomplete
```

K2.16 should close these.

## Target behavior

Implement:

```text
visibility predicates from binding/default/loopback
layer skipped when visibility false
z then authoring-order stable sort
layer-local pipeline in local coordinates
transparent write preserves lower content by default
skip preserves lower content
explicit clear policy if already represented; otherwise document as future
element/layer attribution in diagnostics
scene render IR provenance
```

## Candidate fixtures

Add or harden:

```text
scene/scene_layer_visibility_binding_io.json
scene/scene_layer_nested_parallel_sequences.json
scene/scene_layer_surface_base_style.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
scene/ansi_source_chain.json
content/content_cell_motion_slice.json
```

For the visibility fixture, include tests for:

```text
default visible
default hidden if authored
binding override true
binding override false
```

## Acceptance

* Scene visibility predicates work in player sample requests.
* Scene diagnostics include layer/element id.
* Local coordinates are used for layer-local scopes.
* Lower content preservation is tested.
* Render IR records layer provenance.

---

# Lane E — Source fidelity tranche

## Objective

Improve source support beyond fallback-only evidence while staying bounded.

K2.15 source status:

```text
source.ansi: SGR stripped
source.image: missing-asset fallback
source.procedural: tiny bounded generator set
```

## Scope

Work on:

```text
source.ansi
source.image
source.procedural
source.text
source.card
```

## source.ansi

Preferred improvement: bounded SGR-to-styled-cell support.

Support common cases only:

```text
reset
bold
italic if already in style model
foreground 30-37 / 90-97
background 40-47 / 100-107
basic newline
simple cursor-forward if already easy
```

Do **not** implement a full VTE emulator.

If styled ANSI support is too risky, keep text-only but make the descriptor/report explicitly say:

```text
bounded ANSI text evidence, not styled ANSI parity
```

## source.image

Introduce a player-owned asset boundary if feasible:

```text
PlayerAssetResolver
asset id -> pre-rasterized/styled grid or missing-asset diagnostic
```

Do not implement full image rasterization unless it is already trivial. The important architecture is the resolver seam, not pixels.

## source.procedural

Expand bounded registry only:

```text
dots_spinner
subcell_shape_atlas placeholder or minimal shape grid
simple progress bar / simple flag if useful
```

No runtime command execution. No plugins.

## Canonical fixture target

Add or harden at least 5 source fixtures:

```text
sources/source_ansi_sgr_basic.json
sources/source_ansi_multiline_style.json
sources/source_image_binding_missing_asset.json
sources/source_image_resolver_grid_smoke.json
sources/source_procedural_dots_spinner_binding.json
sources/source_procedural_subcell_shape_placeholder.json
```

## Acceptance

* Source field coverage remains 0-gap.
* Source diagnostics are structured.
* Styled-cell evidence improves if ANSI styling is implemented.
* No visual parity claims for incomplete sources.

---

# Lane F — Content descriptor/adapter tranche

## Objective

Burn down content backlog aggressively.

K2.14 added six content fixtures. K2.15 added no new content descriptors. K2.16 should move content forward.

## Target content families

Prioritize:

```text
content.odometer
content.redact
content.glyphCascade
content.glyphParticles
content.slideShift
content.mirror
content.numeric
content.dissolve
content.cellMotion
```

Use legacy files as evidence:

```text
content/content_odometer.json
content/content_glyph_particles_base_spray.json
content/content_cell_motion_slice.json
content/content_wrap_indicator.json
content/content_typewriter_io_filter_shader.json
content/content_scramble.json
content/content_morph.json
```

## Descriptor rules

Content descriptors should distinguish:

```text
content generator
content transform
transient glyph emitter
cell motion content effect
```

Do not turn these into sources unless they truly produce an initial surface independent of content timing.

## Canonical fixture target

Add at least 8 content fixtures if honest adapters can be written:

```text
content/content_odometer.json
content/content_redact.json
content/content_glyph_cascade.json
content/content_glyph_particles_base_spray.json
content/content_slide_shift.json
content/content_mirror.json
content/content_numeric.json
content/content_dissolve.json
```

## Adapter honesty

Accept deterministic approximation, but document it:

```text
odometer: deterministic tile/roll evidence
glyphParticles: bounded transient glyph evidence
redact: deterministic masking evidence
mirror: deterministic text transform
numeric: deterministic number formatting
slideShift: deterministic offset transform
dissolve: deterministic reveal/hide evidence
```

## Acceptance

* Every authored field is consumed or the fixture is not added.
* Timeline/diff smoke shows actual time variation where expected.
* Field coverage remains 0-gap.
* Adapter gap remains 0 unresolved.

---

# Lane G — Filter / mask / sampler descriptor tranche

## Objective

Burn down a meaningful chunk of the remaining descriptor decision backlog.

## Candidate filters

Prioritize high-confidence descriptors:

```text
filter.vignette
filter.bracketEmphasis
filter.dotIndicator
filter.edgeGrow
filter.hoverBar
filter.kittScanner
filter.glistenSweep
filter.underlineWipe
filter.subPixelBar
filter.glyphStyle
filter.colorBridgedShade
filter.motionBlur
```

## Candidate masks

```text
mask.cellular
mask.centerWipeFade
mask.wipeFadeLeftRight
mask.pathReveal variants
mask.materialize corner variant
```

## Candidate samplers

```text
sampler.crt
sampler.crtJitter
sampler.faultLine variants
sampler.shredder variants
sampler.radialTwist variants
```

## Canonical fixture target

Add or harden at least 12 fixtures across filters/masks/samplers.

## Descriptor discipline

Prefer specific descriptors:

```text
filter.kittScanner
filter.underlineWipe
mask.cellular
sampler.crtJitter
```

Avoid generic catch-all descriptors like:

```text
filter.nativeOnlyThing
shader.richEffect
mask.geometryFx
```

## Acceptance

* Descriptor inputs have ranges/allowed values where useful.
* Player adapters consume every authored field.
* Fixture-QC remains pass.
* No visual parity claims beyond deterministic player evidence.

---

# Lane H — Shader / style descriptor tranche

## Objective

Reduce shader/style descriptor backlog and improve styled-cell evidence.

## Candidate shader compositions

```text
shader.barberPole
shader.pulseWave
shader.radar
shader.reflect
shader.affordanceWake
shader.concealedLight
shader.edgeSheen
shader.focusedRowGradient
shader.bevel
shader.diffusion variants
```

## Candidate shader primitives

```text
shader.glow
shader.chromaticEdge
shader.neonFlicker
shader.orbit
shader.stochasticSparkle
shader.tracePath
shader.tracePropagation
shader.terminalFire variants
shader.terminalWater variants
```

## Candidate styles

```text
style.colorShift
style.fadeIn
style.fadeOut
style.fadeInFromCanvas
style.fadeOutToCanvas
style.pulse
style.rainbow
style.glitch
style.italicWindow
style.cellPositionBinding
```

## Built-in scope policy

Use only accepted built-ins:

```text
moduloRows
moduloColumns
nonEmpty
outerBand
inner
role
rect
rowRange
columnRange
all
```

Do not reintroduce generic predicate registries.

## Canonical fixture target

Add or harden at least 10 shader/style fixtures.

Include at least:

```text
one bound numeric input
one enum input
one color input
one accepted built-in scope
one gradient or multi-color input
one timeline/diff-visible behavior
```

## Acceptance

* Style scopes evaluate in player styled-grid evidence.
* Render IR captures style provenance.
* Field coverage remains 0-gap.
* Adapter gap remains 0 unresolved.

---

# Lane I — Backend seam and holdback preflight

## Objective

Prepare backend/compositor work without violating boundaries.

Do **not** wire compositor directly into UI. Do **not** make `tui-vfx-player-ui` construct compositor DTOs.

## Deliverables

Create:

```text
docs/new_kernel/K2_16_BACKEND_ADAPTER_SEAM_PREFLIGHT.md
docs/new_kernel/K2_16_HOLDBACK_REGISTER.md
```

## Backend seam design

Define the intended boundary:

```text
PlayerRenderIr
  -> PlayerRenderBackend trait
  -> Text/styled-cell backend
  -> future compositor backend adapter
  -> UI consumes player output
```

If feasible, add a trait or internal abstraction:

```text
PlayerRenderBackend
PlayerRenderBackendInput
PlayerRenderBackendOutput
```

But do not implement a compositor backend unless the seam is trivial and isolated.

## Holdback classification

Explicitly list:

```text
shadows/*
subcell_shapes/*
shadow/subcell complex mixes
GUI conflict fixtures
oracle/capture/deprecated records
duplicates
```

For each holdback cluster, include:

```text
why it is held back
what future evidence is required
what packet should own it
whether it is schema-blocking
whether it is descriptor-blocking
whether it is backend-blocking
```

## Acceptance

* Backend holdbacks remain non-schema blockers.
* Future backend adapter path is concrete.
* UI/compositor boundary remains clean.

---

# Lane J — Studio control catalog + schema/API/docs gates

## Objective

Move toward the studio reach goal without prematurely building the full studio.

The studio should eventually auto-generate sliders, inputs, selectors, color pickers, and binding controls from recipe/descriptors. K2.16 should create the first machine-readable control catalog if feasible.

## Implement optional CLI

Preferred:

```bash
cargo run -q -p tui-vfx-player-cli -- control-catalog \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json
```

Suggested schema:

```text
v3.1.player.controlCatalog.1
```

## Control catalog inputs

Use:

```text
graph.parameters
graph.signals
source descriptor inputs
effect descriptor inputs
ValueKind
ValueSpec.range
allowedValues
unit
semantic
runtimeMutability
bindable
optional
descriptor id
source/effect domain
```

## Suggested control mappings

```text
number/integer + range       -> slider + numeric input
number/integer no range      -> numeric input
boolean                      -> toggle
enum + allowedValues         -> select/radio
color                        -> color picker/token selector
gradient                     -> gradient-stop editor placeholder
string/text                  -> text input
duration                     -> duration input
binding-capable input        -> binding picker affordance
compile-time-only input      -> disabled at runtime / authoring only
optional input               -> enable/disable checkbox
sampledField ValueSource     -> spatial-field picker placeholder
```

## If CLI is too large

Create:

```text
docs/new_kernel/K2_16_STUDIO_CONTROL_CATALOG_SPEC.md
```

But prefer a small report implementation if possible.

## Template boundary

Refresh docs to state:

```text
templates are mandatory
template composition is compile-time
runtime/player sees expanded canonical v3.1 recipes
no unresolved extends/mixins/slots at runtime
```

Do not implement template expansion in K2.16 unless it is trivially isolated.

## Schema/docs gates

Run or document exact status:

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

## Acceptance

* Control catalog exists or exact spec is checked in.
* Docs/schema gates pass or failures are documented exactly.
* Public DTO docs are refreshed for touched contract surfaces.
* No studio UI is built prematurely.

---

# Optional canonical fixture additions

K2.16 should add fixtures aggressively, but only when honest.

Target:

```text
25–40 additional canonical fixtures
```

Minimum acceptable:

```text
20 additional canonical fixtures
```

Only add a fixture if:

```text
- descriptor exists or is added in this packet,
- descriptor fields are documented,
- player adapter consumes authored fields,
- validate-recipe passes,
- render-recipe passes,
- render-frame passes,
- fixture-qc passes,
- primitive-field-coverage remains 0-gap,
- primitive-adapter-gap remains 0 unresolved,
- expected_visual metadata is present,
- no legacy aliases are introduced,
- legacy source recipe remains untouched.
```

Hold back problematic items instead of forcing them.

---

# TDD requirements

Start with RED tests for new behavior.

Required new tests:

```text
render IR carries rows/styled cells/roles/provenance
render IR carries graph diagnostics and graph value snapshot
render IR carries scene element/layer provenance
graph missing-value diagnostic
graph kind-mismatch diagnostic
visibility predicate true/false
transparent write preserves lower content
skip preserves lower content
layer-local scope uses local coordinates
source ANSI bounded style/text behavior
image resolver/missing-asset diagnostic
procedural bounded source registry
content odometer or equivalent dynamic content timeline
control catalog maps descriptor inputs to controls, if implemented
```

Regression tests:

```text
existing 67 canonical fixtures stay green
graph.order fallback still works
fixture-qc stays pass
field coverage stays 0-gap
adapter gap stays 0 unresolved
schema-readiness stays canDeclareSchemaReady=true
legacy root mutation check stays clean
```

---

# Acceptance criteria

## Required

```text
- Baseline/final counters documented.
- Player render IR exists internally or as a CLI/report surface.
- Render IR carries styled cells, roles, graph diagnostics, and provenance.
- Player graph executor has stronger missing-value/kind diagnostics.
- Scene visibility predicates work for at least one canonical fixture.
- Layer-local pipeline/provenance is represented in render IR.
- At least 20 new canonical fixtures are added/hardened, unless stopped for a documented architectural reason.
- At least one content family beyond K2.14 content set is added or hardened.
- At least one source fidelity improvement lands.
- At least 10 descriptor/adaptor fixture additions land across primitive families.
- Backlog normalization report gives exact path-level dispositions.
- validate-recipe passes for canonical corpus.
- render-recipe passes for canonical corpus.
- render-frame passes for canonical corpus.
- fixture-qc passes for canonical corpus.
- primitive-field-coverage remains 0 unhandled.
- primitive-adapter-gap remains 0 unresolved.
- schema-readiness remains canDeclareSchemaReady=true.
- legacy debug_recipes root remains unmodified.
```

## Preferred

```text
- 25–40 canonical fixtures added/hardened.
- canonicalExists increases from 51 to at least 75.
- descriptorDecisionNeeded falls below 80.
- sourceDecisionNeeded falls below 35.
- ownerAuditNeeded raw or disposition-normalized count falls materially.
- control-catalog CLI/report implemented.
- backend adapter seam trait or design is checked in.
- ANSI SGR basic styled-cell evidence works.
```

## Stop conditions

Stop and report rather than forcing if:

```text
- render IR starts importing compositor internals;
- source.ansi becomes a full VTE emulator;
- source.image requires full rasterization;
- procedural sources require plugin or command execution;
- field coverage can pass only through false handled declarations;
- scene visibility requires unresolved template/runtime inheritance;
- graph diagnostics require changing accepted schema in a broad way;
- descriptors would encode legacy aliases;
- UI starts constructing backend/compositor DTOs directly.
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

If `nextest` is unavailable:

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

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

## New reports if implemented

```bash
cargo run -q -p tui-vfx-player-cli -- render-ir \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json"

cargo run -q -p tui-vfx-player-cli -- control-catalog \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json
```

## Docs/schema gates

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

Legacy root mutation check must show no modifications.

---

# Expected deliverables

Return these docs:

```text
docs/new_kernel/K2_16_BASELINE_AND_FINAL_COUNTERS.md
docs/new_kernel/K2_16_BACKLOG_NORMALIZATION_REPORT.md
docs/new_kernel/K2_16_PLAYER_RENDER_IR_REPORT.md
docs/new_kernel/K2_16_GRAPH_EXECUTOR_HARDENING_REPORT.md
docs/new_kernel/K2_16_SCENE_LAYER_RUNTIME_FIDELITY_REPORT.md
docs/new_kernel/K2_16_SOURCE_FIDELITY_TRANCHE_REPORT.md
docs/new_kernel/K2_16_CONTENT_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md
docs/new_kernel/K2_16_PRIMITIVE_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md
docs/new_kernel/K2_16_SHADER_STYLE_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md
docs/new_kernel/K2_16_BACKEND_ADAPTER_SEAM_PREFLIGHT.md
docs/new_kernel/K2_16_HOLDBACK_REGISTER.md
docs/new_kernel/K2_16_STUDIO_CONTROL_CATALOG_REPORT.md
docs/new_kernel/K2_16_SCHEMA_API_DOCS_GATE.md
docs/new_kernel/PHASE_K2_16_PLAYER_IR_BACKLOG_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md
docs/new_kernel/PHASE_K2_16_REVIEW_AND_DESLOP_REPORT.md
```

The final status memo must include:

```text
- executive summary
- before/after counter table
- lane-by-lane result table
- exact fixture additions
- render IR behavior and schema/report status
- graph executor hardening summary
- scene/layer runtime summary
- source/content improvements
- descriptor/adaptor additions
- backlog normalization results
- holdback register
- studio control catalog status
- schema/API/docs gate status
- verification matrix
- legacy root mutation status
- unresolved risks
- recommended next packet
```

---

# Recommended next packet after K2.16

Depending on results, the next packet should be one of:

```text
K2.17 — Descriptor/Adapter Migration Tranche 3
K2.17 — Backend Adapter Prototype for shadows/subcell
K2.17 — Scene/Source Fidelity Tranche 2
K3.0  — Studio Control Catalog + Ratatui Control Panel Pilot
```

Do not start compositor-backed rendering unless the player render IR and backend adapter seam are in place and the UI boundary remains clean.

---

# What we can close out now

We can close out:

```text
schema decision readiness for the known debug_recipes migration
```

That is now done.

We can also close out the first runtime proof gap:

```text
player graph topology/value-bus evidence has started and is real
```

It is not complete, but it is no longer merely a `tui-vfx-next` proof artifact.

What remains is implementation completion:

```text
player render IR
scene/layer fidelity
source fidelity
descriptor/adaptor burn-down
backend holdbacks
studio control catalog
template compiler implementation later
visual parity/oracle review later
```

The next packet should be judged by whether it reduces the actual backlog counters and creates reusable player infrastructure, not by whether it produces another readiness declaration.
