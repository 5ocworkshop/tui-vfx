# Architect memo to orchestrator — Phase K2.15 work packet

## Review verdict on K2.14

**ACCEPT.**

K2.14 moved us from schema-decision readiness into real migration evidence. The important counters are now:

```text
canonical v3.1 fixtures:        57
render-recipe:                  57 rendered / 0 unsupported / 0 errors
render-frame:                   57 rendered / 0 unsupported / 0 errors
fixture-qc:                     pass
primitive-field-coverage:       361 used / 361 handled / 0 unhandled
primitive-adapter-gap:          43 rendered / 0 unresolved
schema-readiness:               canDeclareSchemaReady=true
remainingOwnerDecisionCount:    0
```

That is a major close-out of the schema decision loop. The next step is not another schema-readiness packet. The next step is to turn accepted schema decisions into deeper player execution, more canonical fixtures, and descriptor/adapter migration evidence.

The highest-level blocker after K2.14 is:

```text
tui-vfx-player still does not execute full graph topology / value-bus semantics.
```

The next packet should attack that while also continuing descriptor/adaptor migration in parallel.

---

# Phase K2.15 — Graph Execution Integration + Descriptor/Adapter Migration Tranche 2

## Executive goal

Turn the K2.13/K2.14 schema decisions into stronger player evidence and materially reduce the remaining descriptor/source/content backlog.

K2.15 should be a **large, multi-lane implementation packet**. Use up to 10 sub-agents. We are not baby-stepping; we are moving multiple independent lanes forward under clear gates.

The packet has three major outcomes:

```text
1. tui-vfx-player begins consuming graph topology/value-bus semantics, not just graph.order.
2. Another substantial tranche of descriptors/adapters/fixtures migrates into canonical v3.1.
3. The control surfaces show measurable backlog burn-down without reopening schema decisions.
```

---

## Rolling context to include in implementer memo

Completed:

```text
K2.10 corpus-wide migration mapping
K2.11 schema-readiness ledger
K2.12 offender ledger + source.text fixture
K2.13 schema decision burn-down; canDeclareSchemaReady=true
K2.14 descriptor/adapter migration tranche 1; 57 canonical fixtures; 0 unsupported
```

Current durable facts:

```text
Schema readiness for known debug_recipes decisions is approved.
Templates are mandatory compile-time composition, not runtime inheritance.
Scene/element/layer support is core schema, not optional.
Legacy debug_recipes are read-only evidence.
Canonical v3.1 fixtures live under recipes/v3.1/debug_recipes.
Player evidence is honest evidence, not visual parity.
Compositor-backed output remains behind a future player/backend adapter seam.
```

K2.15 is not a schema-decision packet. It is an implementation/evidence packet.

---

# Non-negotiable architectural constraints

## Keep the boundary clean

```text
RecipeDocument v3.1
  -> contract validation
  -> player/runtime evidence
  -> fixture-qc / render-frame / timeline / diff
  -> Ratatui UI consumes player evidence
```

Do not let `tui-vfx-player-ui` construct compositor internals.

Do not mutate compositor internals to fit recipe DTOs.

Do not revive the legacy `tui-vfx-recipes` runtime as authority.

## Legacy root is read-only

The following root is evidence only:

```text
../tui-vfx-recipes/recipes/debug_recipes
```

Do not modify it.

Canonical additions may go only under:

```text
../tui-vfx-recipes/recipes/v3.1/debug_recipes
```

## Schema decisions are closed unless evidence forces a narrow additive correction

Do not reopen broad schema debates. If a lane discovers a truly new schema mismatch, stop that lane and report it as:

```text
explicitOwnerDecisionNeeded
```

Do not silently invent aliases or shove legacy shapes into canonical v3.1.

## Documentation/refactoring requirement

For every file touched:

```text
- look for complexity reduction opportunities,
- improve naming and maintainability,
- add or refresh rustdoc where public or semi-public APIs are touched,
- add schemars/schema descriptions where contract DTOs are touched,
- keep OFPF metadata current where the project style expects it,
- avoid stale phase-specific vocabulary in durable public API names.
```

This is continuous, not optional.

---

# Work model: 10 parallel lanes

```text
A. Metrics/control surface and backlog burn-down accounting
B. Player graph topology/value-bus executor
C. Graph I/O canonical fixtures and proof parity
D. Scene/layer-local pipeline and visibility player evidence
E. Source adapters tranche 2
F. Content adapters tranche 2
G. Filter/mask/sampler descriptors tranche 2
H. Shader/style descriptors tranche 2
I. Backend/holdback policy and visual-review evidence prep
J. Schema/API/docs/studio-control and release gates
```

Each lane should return a concise lane memo and tests. The packet should consolidate them into one architect status memo.

---

# Lane A — Metrics, control surface, and backlog burn-down accounting

## Objective

Make the before/after impact of K2.15 impossible to dispute.

Capture baseline counters before work starts and final counters after work lands.

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

## Deliverables

Create:

```text
docs/new_kernel/K2_15_BASELINE_AND_FINAL_COUNTERS.md
```

Include:

```text
canonical fixture count before/after
rendered/unsupported/errors before/after
field coverage before/after
adapter gap before/after
schema readiness before/after
migration mapping before/after
descriptorBacklog before/after
sourceDescriptor before/after
acceptedSchema before/after
canonicalExists before/after
candidateReady before/after
remaining holdbacks
```

## Acceptance

* Baseline and final counters are recorded.
* No field/adapter gap is hidden by changing report semantics.
* New counters are additive and documented if report fields change.

---

# Lane B — Player graph topology/value-bus executor

## Objective

Make `tui-vfx-player` consume accepted graph topology/value-bus semantics instead of relying only on linear `graph.order`.

This is the highest-leverage packet lane.

## Current gap

K2.14 reported:

```text
tui-vfx-next proves graph I/O semantics,
but tui-vfx-player still applies graph nodes in graph.order.
```

K2.15 should begin closing that gap.

## Accepted semantics to implement

Use K2.13 decisions:

```text
Sequence:
  child N+1 sees mutations and graph values emitted by child N.

Parallel:
  branches read the same input snapshot;
  branch surfaces merge at the join;
  graph values merge at the join;
  conflicts use explicit policy or authored-order default with diagnostics.
```

Accepted I/O shape:

```text
io.outputs:
  hint
  kind
  source

io.inputs:
  input
  hint
  kind
```

## Implementation guidance

Add a player-owned graph execution layer, likely under:

```text
crates/tui-vfx-player/src/
```

Possible new modules:

```text
cls_player_graph_value.rs
cls_player_graph_value_bus.rs
cls_player_graph_execution_context.rs
fnc_execute_graph_step.rs
fnc_execute_graph_sequence.rs
fnc_execute_graph_parallel.rs
fnc_merge_parallel_outputs.rs
fnc_resolve_graph_io.rs
```

Names are suggestions. Keep OFPF size and readable boundaries.

## Behavior

The player should:

```text
1. Prefer graph.topology when present.
2. Fall back to graph.order for old/simple canonical fixtures.
3. Execute leaf nodes through existing adapter dispatch.
4. Allow leaf adapters to read resolved input values from:
   - literal node input
   - parameter
   - signal
   - graph value
   - fallback/default
5. Allow leaf adapters to emit graph values through io.outputs.
6. Carry value kind metadata for scalar/color/gradient/etc.
7. Emit diagnostics when:
   - input hint missing and no fallback exists,
   - kind mismatch occurs,
   - parallel graph-value conflict occurs,
   - parallel surface write conflict occurs,
   - topology references unknown node.
```

## Parallel surface handling

Do not overbuild. For K2.15, acceptable surface conflict behavior is:

```text
- snapshot branch inputs,
- execute each branch independently,
- merge in authored order,
- if two branches write the same channel/cell, later branch wins by default,
- emit deterministic conflict diagnostics.
```

If current styled-grid evidence cannot represent every channel conflict, emit diagnostics and preserve deterministic output.

## Acceptance tests

Add RED tests first.

Required test coverage:

```text
sequence output is consumed by later node
parallel branch output is visible after join
parallel branch output is not visible inside sibling branch
parallel graph-value conflict emits deterministic diagnostic
parallel surface conflict emits deterministic diagnostic
topology fallback to graph.order preserves all existing fixtures
unknown topology node fails with structured diagnostic
```

Existing graph proof tests in `tui-vfx-next` should remain. Do not remove them.

## Acceptance gates

After implementation:

```text
render-recipe over all canonical fixtures still passes
fixture-qc still passes
new graph I/O canonical fixtures render
graph executor tests pass
no old graph.order-only fixtures regress
```

---

# Lane C — Graph I/O canonical fixtures and proof parity

## Objective

Add canonical v3.1 fixtures that prove the new player graph executor, not just `tui-vfx-next`.

## Candidate fixtures

Use existing legacy evidence as inspiration:

```text
complex/complex_filter_to_mask_sourced_output.json
complex/v3_io_parallel_merge_shader.json
complex/complex_nested_parallel_sequences.json
complex/complex_parallel_overlap_conflict_snapshot.json
content/content_typewriter_io_filter_shader.json
scene/scene_layer_visibility_binding_io.json
```

## Add canonical fixtures under

```text
../tui-vfx-recipes/recipes/v3.1/debug_recipes/complex/
```

Suggested fixtures:

```text
complex/graph_io_sequence_filter_to_mask.json
complex/graph_io_parallel_merge_shader.json
complex/graph_nested_parallel_sequences.json
complex/graph_parallel_overlap_conflict_snapshot.json
```

## Requirements

Each fixture must:

```text
- be descriptor-pack-backed,
- validate strictly,
- render through player,
- produce visual-frame evidence,
- show graph I/O behavior through report diagnostics or output,
- keep expected_visual metadata,
- avoid legacy aliases.
```

## Acceptance

* At least 3 new graph I/O fixtures added.
* At least one sequence fixture.
* At least one parallel join fixture.
* At least one conflict diagnostic fixture.
* Player, not only `tui-vfx-next`, proves these behaviors.

---

# Lane D — Scene/layer-local pipeline and visibility evidence

## Objective

Turn accepted scene/element/layer semantics into player evidence.

K2.13 accepted scene semantics. K2.14 only improved bounded source scene evidence. K2.15 should implement more of the player path.

## Target behaviors

The player should support:

```text
scene.layers / scene.elements traversal
z then authoring-order sort
source rendering per layer
placement into global coordinates
visibility predicates from bindings/signals/defaults
surface/base_style application
layer-local pipeline in layer-local coordinates
skip preserves lower content
transparent cells preserve/blend through unless explicit clear policy
diagnostic attribution with layer/element id
```

## Candidate legacy evidence

```text
scene/scene_layer_nested_parallel_sequences.json
scene/scene_layer_visibility_binding_io.json
scene/scene_layer_surface_base_style.json
scene/ansi_source_chain.json
content/content_cell_motion_slice.json
subcell_shapes/fractional_inset_rect_v3.json
subcell_shapes/braille_rounded_rect_v3.json
```

## Deliverables

Possible new/updated player modules:

```text
fnc_render_scene.rs
fnc_render_scene_layer.rs
fnc_render_scene_source.rs
fnc_apply_layer_pipeline.rs
cls_player_scene_diagnostic.rs
```

Refactor existing scene rendering if it is still row-only or too monolithic.

## Canonical fixtures

Add at least 3 canonical scene fixtures:

```text
scene/scene_layer_visibility_binding_io.json
scene/scene_layer_nested_parallel_sequences.json
scene/scene_layer_surface_base_style.json
```

Only add source/procedural/subcell-heavy scene fixtures if the needed source adapter is in place and honest.

## Acceptance

* Visibility binding fixture can be toggled via fallback/binding evidence.
* Layer-local pipeline runs in local coordinates.
* z-order and lower-content preservation are deterministic.
* Diagnostics include layer/element identity.
* Existing source fixtures remain green.

---

# Lane E — Source adapters tranche 2

## Objective

Move source support from placeholder/fallback toward useful bounded adapters.

K2.14 added:

```text
source.ansi
source.image
source.procedural
```

but reported bounded fallback only.

## Scope

Implement/harden:

```text
source.ansi
source.image
source.procedural
source.text
source.card
```

## Guidance

### source.ansi

K2.14 intentionally stripped SGR. K2.15 should either:

```text
- add styled-cell ANSI SGR support for common colors/modifiers, or
- explicitly keep source.ansi as text-only and document it as not visual parity.
```

Preferred: implement a bounded SGR parser if reasonable:

```text
bold
reset
foreground 30-37 / 90-97
background 40-47 / 100-107
basic RGB if already supported by parsing helpers
cursor horizontal move if already simple
```

Stop if ANSI parsing becomes a full VTE project. Do not overbuild.

### source.image

Current fallback is fine, but improve evidence:

```text
missing asset fallback
asset key resolution from binding/default
explicit diagnostic when resolver absent
```

Do not implement actual image rasterization unless already trivial.

### source.procedural

Support a small stock set only:

```text
dots_spinner
subcell_shape_atlas placeholder/fallback if needed
```

Do not turn procedural sources into a hidden runtime command/external plugin system.

## Canonical fixtures

Add or harden fixtures:

```text
sources/source_ansi_sgr_basic.json
sources/source_image_binding_missing_asset.json
sources/source_procedural_dots_spinner_binding.json
```

## Acceptance

* Source descriptors have accurate optional/required inputs.
* Player field coverage includes source inputs actually consumed.
* Source diagnostics are structured.
* No runtime command execution.

---

# Lane F — Content adapters tranche 2

## Objective

Continue migrating content family descriptors/adapters and fixtures.

K2.14 added:

```text
content.typewriter
content.marquee
content.splitFlap
content.wrapIndicator
content.scramble
content.morph
```

K2.15 should expand content coverage substantially but honestly.

## Target content effects

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
```

Use existing legacy evidence:

```text
content/content_odometer.json
content/content_glyph_particles_base_spray.json
content/content_wrap_indicator.json
content/content_cell_motion_slice.json
content/content_typewriter_io_filter_shader.json
```

## Rules

Content effects are not sources.

```text
Source: produces initial semantic surface.
Content effect: transforms/emits content over time inside a source/surface.
Graph effect: runs over a surface after content/source production.
```

Do not overload `source.text` to implement content effects.

## Adapter honesty

Accept deterministic player evidence, not parity.

Examples:

```text
odometer: deterministic tile/roll approximation is okay if documented.
glyphParticles: bounded transient glyph evidence is okay.
redact/scramble/morph: deterministic seeded text transform is okay.
marquee: loop-t based text offset is okay.
```

## Canonical fixtures

Add at least 6 new canonical content fixtures if descriptors/adapters are honest:

```text
content/content_odometer.json
content/content_redact.json
content/content_glyph_cascade.json
content/content_glyph_particles_base_spray.json
content/content_slide_shift.json
content/content_numeric.json
```

Do not add if field coverage cannot honestly pass.

## Acceptance

* New content descriptors include inputs, ranges, allowedValues, units where useful.
* Player handles every authored field or explicitly does not add the fixture.
* Timeline/diff smoke shows non-static content where expected.
* `fixture-qc` remains pass.

---

# Lane G — Filter / mask / sampler descriptor tranche 2

## Objective

Burn down a second batch of descriptorBacklog records in primitive families.

## Candidate filters

From legacy evidence:

```text
filter.crt
filter.matrixRain
filter.patternFill
filter.pillButton
filter.fadeToCanvas
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
```

K2.14 already touched several. K2.15 should either add new ones or deepen variants.

## Candidate masks

```text
mask.pathReveal
mask.materialize
mask.noiseDither
mask.cellular
mask.centerWipeFade
mask.wipe corner/path variants
```

## Candidate samplers

```text
sampler.crt
sampler.crtJitter
sampler.faultLine
sampler.shredder
sampler.radialTwist
```

K2.14 touched some. K2.15 should close remaining high-confidence ones.

## Required discipline

Do not create giant generic descriptors when specific descriptors are clearer.

For example:

```text
Good: mask.pathReveal with path tagged union.
Good: sampler.faultLine with seed/intensity/splitBias.
Risky: one generic geometryFx descriptor that hides semantics.
```

## Canonical fixture target

At least 12 new or hardened fixtures across these families.

## Acceptance

* Primitive field coverage remains 0 unhandled.
* Adapter gap remains 0 unresolved.
* Each new descriptor has documented input semantics.
* Bound inputs use accepted runtime dynamism model, not ad-hoc binding values.

---

# Lane H — Shader / style descriptor tranche 2

## Objective

Continue shader/style descriptor migration with field-coverage honesty.

## Candidate shader compositions

```text
shader.highlighter variants
shader.focusField variants
shader.glistenBand variants
shader.wayfindingNode variants
shader.barberPole
shader.pulseWave
shader.radar
shader.reflect
shader.affordanceWake
shader.concealedLight
shader.edgeSheen
shader.focusedRowGradient
```

## Candidate shader primitives

```text
shader.revealWipe variants
shader.terminalFire modes
shader.terminalWater modes
shader.orbit
shader.neonFlicker
shader.stochasticSparkle
shader.tracePath
shader.tracePropagation
shader.diffusion variants
shader.glow
shader.chromaticEdge
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

## Built-in scope validation

Use K2.13 accepted scopes:

```text
moduloRows
moduloColumns
nonEmpty
outerBand
inner
```

Do not reintroduce generic predicate registry unless a new owner decision is opened.

## Canonical fixture target

At least 10 shader/style fixtures, with a mix of:

```text
static literal input
binding fallback input
gradient/color input
scope variant
timeline/diff visible behavior
```

## Acceptance

* `gradient`, `applyTo`, and `position` remain closed.
* New shader/style fields are either consumed or not added.
* Style scopes are evaluated by player styled-grid evidence.
* No visual parity claims.

---

# Lane I — Backend / holdback policy and visual-review prep

## Objective

Keep backend/gui/oracle holdbacks explicit while preparing the next major phase.

Do not wire compositor yet unless the architect explicitly approves a backend packet.

## Scope

Holdback classes:

```text
backendHoldback: shadows, subcell_shapes, backend-heavy complex mixes
guiHumanReviewHoldback: overlap conflict visual policy fixtures
oracleOnly: command capture, deprecated records, preview-only loopback demos
duplicateVariant: explicit variants
```

## Deliverables

Update:

```text
docs/new_kernel/K2_15_HOLDBACK_REGISTER.md
```

Add:

```text
backend holdback count before/after
candidate backend packet scope
GUI review candidates and exact owner questions
oracle-only records and why runtime must not execute them
duplicate variants and equivalence basis
```

## Backend packet preflight

Create a short preflight section for a later backend adapter packet:

```text
What backend adapter would need to prove
Which fixtures would be first
Which APIs must not be touched
What visual evidence is required
How K1/K3 UI consumes backend evidence without constructing compositor DTOs
```

## Acceptance

* Holdbacks are not counted as schema blockers.
* Holdbacks remain visible and named.
* No compositor/UI boundary violation.

---

# Lane J — Schema/API/docs/studio-control and release gates

## Objective

Keep generated schema/API docs current and advance studio-control readiness without building the full studio.

## Required checks

Use the K2.14 docs gate:

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

If any command is unavailable or flaky, document exact status rather than hand-waving.

## Studio-control catalog preflight

Do not build the full studio UI. But add a CLI/report if bounded and useful:

```text
tui-vfx-player-cli control-catalog
```

or defer with a precise design if too large.

A useful report would consume:

```text
descriptor packs
source descriptors
effect descriptors
graph.parameters
graph.signals
ValueSpec
range
allowedValues
unit
semantic
runtimeMutability
bindable
optional
```

and emit:

```text
v3.1.player.controlCatalog.1
```

Suggested controls:

```text
number/integer + range -> slider/spinbox
enum + allowedValues -> select/radio
boolean -> toggle
color -> color picker/token selector
gradient -> gradient editor placeholder
bindable=true -> binding/source picker affordance
compileTime mutability -> disabled at runtime
```

If this is not implemented, update:

```text
docs/new_kernel/K2_15_STUDIO_CONTROL_CATALOG_PREFLIGHT.md
```

with the exact future report shape.

## Template note

Templates are mandatory but compile-time:

```text
template + slots + overrides + mixins
  -> expanded canonical v3.1 recipe
  -> strict validation
  -> player evidence
```

Do not implement template expansion in K2.15 unless it is already trivially separable. But ensure docs continue to state this boundary.

## Acceptance

* Schema files current.
* Docs/API gates pass or exact failures documented.
* Studio-control path is more concrete than K2.14.
* Public rustdoc/schemars descriptions are refreshed for touched DTOs.

---

# Optional canonical fixture additions

K2.15 may add many fixtures, but only under strict rules.

A canonical fixture may be added only if:

```text
- descriptor exists or is added in this packet,
- descriptor fields are documented,
- player adapter consumes every authored field or honestly reports it,
- primitive-field-coverage remains 0-gap,
- adapter-gap remains 0 unresolved,
- validate-recipe passes,
- render-recipe/render-frame pass,
- fixture-qc passes,
- expected_visual metadata is present,
- the fixture does not rely on legacy aliases,
- the legacy source recipe remains untouched.
```

Expected K2.15 fixture target:

```text
20-35 additional canonical fixtures
```

Do not force this number if graph executor work consumes capacity. Graph executor quality is more important than raw fixture count.

---

# TDD requirements

Start with RED tests for the main new behaviors.

## Required tests

```text
graph sequence value consumption
graph parallel snapshot isolation
graph parallel post-join value visibility
graph graph-value conflict diagnostic
graph surface conflict diagnostic
topology fallback to graph.order
layer visibility binding
layer-local pipeline coordinate behavior
source ANSI bounded styled/text behavior
source image missing asset fallback
source procedural dots spinner
content adapter dynamic/timeline behavior
control/report docs gate if implemented
```

## Regression tests

```text
render-recipe existing canonical corpus
render-frame existing canonical corpus
fixture-qc existing canonical corpus
primitive-field-coverage remains 0-gap
primitive-adapter-gap remains 0 unresolved
schema-readiness remains canDeclareSchemaReady=true
legacy root mutation check
```

---

# Acceptance criteria

## Required

```text
- K2.15 baseline/final metrics are documented.
- tui-vfx-player executes at least sequence topology and one parallel topology path.
- Graph I/O value-bus behavior is proven in player tests, not only tui-vfx-next.
- Existing graph.order fallback behavior remains stable.
- At least 3 canonical graph I/O fixtures are added or hardened.
- At least 3 canonical scene/layer evidence fixtures are added or hardened.
- At least 10 descriptor/adaptor fixture additions or hardenings land across content/source/primitive families.
- validate-recipe passes for canonical v3.1 corpus.
- render-recipe passes for canonical v3.1 corpus.
- render-frame passes for canonical v3.1 corpus.
- fixture-qc passes for canonical v3.1 corpus.
- primitive-field-coverage reports 0 used-but-unhandled fields.
- primitive-adapter-gap reports 0 unresolved.
- schema-readiness remains canDeclareSchemaReady=true.
- no legacy debug_recipes files are modified.
- docs/schema/API gate is run and documented.
```

## Preferred

```text
- 20+ canonical fixtures added/hardened.
- descriptorBacklog decreases materially from 219.
- canonicalExists increases materially from 48.
- player graph executor handles nested sequence/parallel enough for complex fixtures.
- layer-local pipelines support at least filter/style/shader paths.
- source.ansi handles basic SGR styled-cell output, or explicitly documents fallback limits.
- studio control-catalog report is implemented as a small CLI/report surface.
```

## Stop conditions

Stop and report rather than forcing implementation if:

```text
- graph topology requires a hidden legacy runtime dependency,
- player graph execution would require compositor internals,
- field coverage can only pass by marking unconsumed fields as handled,
- source.ansi becomes a full VTE emulator,
- source.image requires real image rasterization beyond missing-asset fallback,
- procedural sources require runtime plugin/command execution,
- template expansion starts leaking into runtime/player,
- scene/layer implementation blurs role identity with element identity,
- a descriptor addition would encode an ambiguous legacy alias,
- visual parity claims are needed to pass tests.
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

The legacy root mutation check must have no modifications.

---

# Expected deliverables

The implementer should return these docs:

```text
docs/new_kernel/K2_15_BASELINE_AND_FINAL_COUNTERS.md
docs/new_kernel/K2_15_GRAPH_EXECUTION_INTEGRATION_REPORT.md
docs/new_kernel/K2_15_GRAPH_IO_FIXTURE_REPORT.md
docs/new_kernel/K2_15_SCENE_LAYER_PLAYER_EVIDENCE_REPORT.md
docs/new_kernel/K2_15_DESCRIPTOR_ADAPTER_TRANCHE_2_REPORT.md
docs/new_kernel/K2_15_SOURCE_CONTENT_TRANCHE_2_REPORT.md
docs/new_kernel/K2_15_HOLDBACK_REGISTER.md
docs/new_kernel/K2_15_SCHEMA_API_DOCS_GATE.md
docs/new_kernel/K2_15_STUDIO_CONTROL_CATALOG_PREFLIGHT.md
docs/new_kernel/PHASE_K2_15_GRAPH_DESCRIPTOR_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md
docs/new_kernel/PHASE_K2_15_REVIEW_AND_DESLOP_REPORT.md
```

The final status memo must include:

```text
- executive summary
- before/after counters
- lane-by-lane result table
- graph executor behavior implemented
- graph I/O fixture evidence
- scene/layer evidence
- descriptor/adaptor additions
- source/content additions
- holdback changes
- schema/API/docs status
- studio-control status
- optional fixture list
- verification matrix
- legacy mutation status
- unresolved risks
- recommended next packet
```

---

# Recommended next packet after K2.15

Depending on K2.15 results, likely next packet should be one of:

```text
K2.16 — Descriptor/Adapter Migration Tranche 3
K2.16 — Scene/Layer/Source Fidelity Tranche
K2.16 — Backend Adapter Preflight for shadows/subcell
K3.0 — Studio control catalog + Ratatui control panel pilot
```

Do not recommend compositor backend wiring unless K2.15 shows descriptor/source/player migration is blocked specifically by backend rendering.

---

# End-state explanation for the architect

## What we are able to close out here

We can close out the **schema-decision phase** for the known `debug_recipes/` migration.

The latest accepted evidence says:

```text
schema-readiness canDeclareSchemaReady=true
unresolvedSchemaBlockers=0
remainingOwnerDecisionCount=0
fieldCoverageBlockedRecords=0
unknownRecords=0
```

That means the outstanding migration work is no longer blocked by vague questions like:

```text
What is a binding?
What is a signal?
Can gradients exist as typed values?
Can sampled spatial fields exist?
Can scenes/layers/elements be represented?
Can graph sequence/parallel/value-bus semantics be represented?
Are templates required?
Where do source/content/effect boundaries sit?
```

Those decisions now have accepted answers.

So we can close:

```text
“v3.1 schema decision readiness for current debug_recipes evidence”
```

with the important qualifier:

```text
This is schema decision readiness, not full migration completion and not visual parity.
```

## How close we are to schema and DTO completion

For the debug recipe corpus, the DTO/model side is now substantially in place.

In place or accepted:

```text
RecipeDocument
DescriptorPack / DescriptorCatalog
SourceDescriptor
EffectDescriptor
ValueKind / Value
GradientSpec / GradientStop
ValueSource including sampledField
optional descriptor/source inputs
Parameter / Signal / GraphValue / Binding distinctions
ScopeSpec built-ins
Scene / Element / Layer model
Graph sequence / parallel / I/O semantics
Lifecycle/motion/easing vocabulary
Template composition boundary
Studio control derivation inputs
```

That means the contract shape is no longer the bottleneck. There may still be **small additive refinements** as later descriptors reveal specific fields, but the major DTO decisions are made.

My estimate:

```text
Schema/DTO decision readiness: very high, effectively closed for current debug_recipes.
Schema/DTO implementation completeness: high, with additive descriptor-driven polish expected.
```

## How close we are to descriptor completion

Descriptor work is not complete.

K2.14 moved descriptor/adaptor evidence forward materially:

```text
descriptor pack expanded from 18 to 45 effects
canonical fixtures moved from 27 to 57
adapter gap rendered 43 / unresolved 0
field coverage 361 / 361 handled
```

But the latest counters still show:

```text
descriptorBacklog=219
sourceDescriptor=remaining backlog
backendHoldback=15
guiHumanReviewHoldback=2
oracleOnly=195
duplicateVariant=3
```

So the descriptor layer is in a good working shape, but not done.

My estimate:

```text
Descriptor system readiness: high.
Descriptor corpus coverage: partial, maybe low-to-mid relative to all 603 legacy records.
Descriptor/adaptor migration: active, with major tranches still needed.
```

## How close we are to player/runtime completion

Player/runtime is now the critical path.

The biggest unresolved implementation gap is:

```text
tui-vfx-player does not yet fully execute graph topology/value-bus semantics.
```

K2.14 added proof evidence in `tui-vfx-next`, but the player still needs to own enough of this to make fixture evidence meaningful.

The next most important player gaps are:

```text
layer-local scene pipelines
visibility predicates
richer source adapters
content effect timelines
source/content fidelity
backend holdbacks for shadow/subcell
```

My estimate:

```text
Player smoke evidence: strong.
Player descriptor adapter evidence: growing quickly.
Player graph/scene runtime fidelity: incomplete and now the highest priority.
```

## What is outstanding and why

### 1. Graph topology/value-bus execution

Outstanding because the proof layer and player evidence are not yet unified.

Why it matters:

```text
Many complex recipes are now schema-ready, but not honestly runnable in player until sequence/parallel/graph-value semantics are player-executed.
```

Recommendation:

```text
Implement player graph executor in K2.15.
```

### 2. Scene/layer-local pipeline execution

Outstanding because scene model is accepted, but the player needs stronger evidence for:

```text
local coordinates
visibility predicates
layer-local pipelines
overlap behavior
element/layer diagnostics
```

Recommendation:

```text
Implement layer-local pipeline and visibility evidence alongside graph executor.
```

### 3. Descriptor/adaptor backlog

Outstanding because 603 legacy debug recipes cover a broad function set. We have migrated the first tranches, not the full catalog.

Recommendation:

```text
Continue descriptor/adaptor migration in batches, grouped by family and blocked only when real semantics are ambiguous.
```

### 4. Source/content fidelity

Outstanding because source/content is now accepted as a model, but real player adapters are still bounded.

Examples:

```text
ANSI styled cells
image asset resolver
procedural source registry
odometer/glyph-particle/content timing
```

Recommendation:

```text
Expand source/content in K2.15 and K2.16, but keep runtime command execution out.
```

### 5. Backend holdbacks

Outstanding because shadow/subcell/compositor-quality rendering belongs behind a backend adapter, not in schema readiness or UI internals.

Recommendation:

```text
Prepare backend packet after graph/scene/player evidence is stronger.
```

### 6. Studio controls

Outstanding because the control catalog depends on descriptors, parameters, signals, and runtime mutability being stable.

We are now close enough to start a **control-catalog report** before building a full studio UI.

Recommendation:

```text
K2.15 or K3.0 should create a descriptor-derived control catalog. Full live UI controls come after graph/value/source player behavior stabilizes.
```

## What I recommend to complete the plan

The fastest professional path is:

```text
1. K2.15: player graph executor + descriptor/adapter migration tranche 2.
2. K2.16: scene/source/content fidelity tranche + remaining descriptor backlog tranche.
3. K2.17: backend adapter preflight or first shadow/subcell backend evidence, depending on backlog.
4. K3.0: studio control catalog and Ratatui control panel pilot.
5. Later: template compiler expansion implementation.
6. Later: visual parity/oracle comparison gates.
```

The next real completion milestone should be:

```text
All non-holdback debug_recipes records are either:
  - canonical v3.1 fixture + fixture-QC passing,
  - descriptorBacklog with exact target descriptor,
  - backendHoldback with exact future backend evidence,
  - guiHumanReviewHoldback with explicit owner question,
  - oracleOnly / duplicateVariant signed off.
```

K2.15 should move a large portion of the first two categories forward and eliminate the current player graph evidence gap.
