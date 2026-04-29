````markdown
<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_12.md</FILE> - <DESC>Architect response defining K2.13 schema decision and offender burn-down packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 implementer packet: resolve schema-readiness blockers, classify holdbacks, and prepare the studio/schema-doc foundation.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — convert K2.12 offender ledger into a multi-lane decision and implementation packet.</CLOG> -->

# Architect Response to K2.12

## Review verdict

**ACCEPT, with escalation.**

K2.12 did the right preparatory work:

```text
- schema-readiness exists
- --include-offenders exists
- ownerAudit / unknown offender rows are normalized
- source.text is descriptor-backed and fixture-backed
- blocker counts are concrete
````

However, the next packet must not merely restate the blocker board. The next packet must make architectural decisions, implement the low-friction schema/report/descriptor changes, and move every remaining offender into a resolved disposition.

The user’s direction is clear:

```text
Do not keep baby-stepping.
Hold back genuinely problematic items.
Proceed aggressively on low-friction items.
Use 6–8 parallel lanes.
Return exact offender decisions, not generic blocker buckets.
```

---

# Phase K2.13 — Schema Decision and Offender Burn-Down

## Executive goal

Convert the current schema-readiness offender ledger into a schema-lock decision result.

By the end of K2.13, every legacy `debug_recipes/` record must be in one of these states:

```text
acceptedSchema
descriptorBacklog
adapterBacklog
backendHoldback
guiHumanReviewHoldback
oracleOnly
duplicateVariant
explicitOwnerDecisionNeeded
```

The packet may leave descriptor/adapter/backend work for later, but it must not leave vague schema blockers.

---

# Definition of 100% schema readiness

For this project, **100% schema readiness** does **not** mean every legacy recipe has already been ported visually.

It means:

```text
Every legacy debug_recipes record is either:
  - representable by the locked v3.1 schema,
  - blocked only by descriptor/player/adapter implementation,
  - explicitly signed off as backend/gui/oracle/duplicate holdback,
  - or listed as a small exact owner-decision item with path and decision text.
```

The schema-readiness gate should eventually report:

```text
canDeclareSchemaReady: true
```

or, if not true:

```text
canDeclareSchemaReady: false
remainingOwnerDecisionCount: small exact number
remainingOwnerDecisions[]:
  - path
  - family
  - blockerKind
  - exact decision required
```

The next packet is expected to get us to **100% schema-decision readiness**, or to return only a short, exact list of unresolved owner decisions. A broad “not yet” result is no longer sufficient.

---

# Current blocker classes to burn down

K2.12 reports these unresolved offender kinds:

```text
descriptorPack:          189
sourceDescriptor:         74
motionTimingSemantics:    34
sceneSemantics:           26
bindingSemantics:         22
backendRenderer:          15
valueSourceSemantics:     12
contentDescriptor:         5
fieldCoverage:             4
guiHumanReview:            2
oracleOnly:                2
lifecycleSemantics:        1
```

K2.13 must turn those into resolved dispositions.

Descriptor-only backlog may remain after schema lock. Backend/gui/oracle holdbacks may remain after explicit signoff. The remaining schema/model buckets must be resolved or reduced to exact owner decisions.

---

# Non-negotiable architectural decisions

The implementer should use the decisions below unless they discover a concrete contradiction. Any contradiction must be reported with exact recipe paths and a proposed alternative.

## 1. Source and content are separate

Use this distinction:

```text
Source:
  produces an initial semantic surface.

Content effect:
  transforms/emits content over time inside that source/surface.
```

Do not force content effects into `source.card` or `source.text`.

Accepted source families:

```text
source.card
source.text
source.ansi
source.image
source.procedural
```

Accepted content effect descriptor domain:

```text
content.typewriter
content.splitFlap
content.odometer
content.marquee
content.scramble
content.morph
content.redact
content.glyphCascade
content.glyphParticles
```

Command capture remains:

```text
oracleOnly
offline authoring artifact
no runtime command execution
```

## 2. Scene / element / layer support is core schema work

Scene support is not optional future work. It is already required by the reference corpus.

Lock the model as:

```text
Scene
  elements[] / layers[]

Element or Layer
  id
  z / layer
  source
  placement
  visibility
  surface/base_style
  cell_motion
  local pipeline
  clip/overflow policy
```

Element identity and role identity must remain separate:

```text
Role:
  text, border, background, shadow, content

Element:
  ansi_layer, logo, spinner, card, slice_top, visibility_io_card
```

Layer-local pipelines run in layer-local coordinates. Root pipelines run after scene composition in scene/global coordinates unless explicitly declared otherwise.

Composition default:

```text
1. Sort by z, then authoring order.
2. Render each layer source.
3. Apply layer-local content/cell-motion/pipeline.
4. Composite into scene.
5. Skipped cells preserve lower content.
6. Transparent writes blend through / do not clear unless an explicit clear policy exists.
7. Non-transparent writes replace according to write policy.
8. Overlap conflicts emit diagnostics unless explicitly resolved by policy.
```

## 3. Parameter, signal, graph value, and binding are distinct

Use four concepts:

```text
Parameter:
  user/studio-adjustable recipe knob

Signal:
  host/event/time-varying input stream

GraphValue:
  internal value emitted by one graph node and consumed by another

Binding:
  wiring from a parameter/signal/graphValue into an input
```

Binding is not a runtime value class.

This distinction is required for:

```text
typewriter_speed_variance_bindable.json
integer_binding_demo.json
text_binding_demo.json
single_oscillator_intensity_signal.json
filter_dim_sample_surface_angle_from.json
shader_border_sweep_position_binding.json
scene_layer_visibility_binding_io.json
```

## 4. Event-driven dwell uses lifecycle triggers

Do not preserve legacy `pipeline.timing.dwell_until_binding` as canonical runtime schema.

Map to lifecycle dwell policy:

```text
integer dismiss_count:
  trigger source: signal or parameter dismiss_count
  predicate: nonZero

text dismiss_reason:
  trigger source: signal or parameter dismiss_reason
  predicate: nonEmpty

bool userDismissed:
  trigger source: signal userDismissed
  predicate: isTrue
```

Preserve the distinctions:

```text
Trigger ≠ Binding
Trigger ≠ Gate
Trigger ≠ Loopback
Lifecycle trigger ≠ effect-local schedule
```

## 5. Value sources include sampled fields

Descriptor inputs may receive a `ValueSource` when compatible with the declared input kind.

Accepted value-source families:

```text
literal
parameter
signal
graphValue
map/remap
sampledField
```

The sampled-surface examples are not host signals. They are per-cell spatial fields.

Map:

```text
sample_surface_angle_from(x, y)
```

to:

```text
sampledField.surfaceAngleFrom { x, y }
```

Then use `map/remap` for numeric conversion.

## 6. Motion/easing are core lifecycle vocabulary

Easing and motion routes are not effect descriptors.

Accept `EasingSpec`:

```text
linear
quadIn / quadOut / quadInOut
sineIn / sineOut / sineInOut
cubicIn / cubicOut / cubicInOut
expo / circ / back / bounce / elastic
cubicBezier { x1, y1, x2, y2 }
```

Allow Bézier y overshoot. `ease_bezier_custom.json` intentionally uses `y2 = 1.24`.

Accept `MotionRouteSpec`:

```text
linear
figureEight
orbit/helix where explicitly supported
```

Normalize authoring aliases:

```text
infinity -> figureEight
```

Final canonical recipes should not retain aliases.

## 7. Style scopes: accept common built-ins, restrict generic predicates

Accept built-in scope vocabulary:

```text
moduloRows { modulus, remainder }
moduloColumns { modulus, remainder }
nonEmpty
outerBand
inner / interior
```

Do not accept arbitrary predicate registries as an open-ended runtime hook yet.

For:

```text
style_predicate_interior.json
```

canonicalize to built-in:

```text
inner / interior
```

Generic predicate refs remain held back until a predicate registry model is designed.

## 8. Close the four field-coverage blockers

Do not allow these to linger.

### `shader.linearGradient.gradient`

Make `gradient` the canonical input:

```text
gradient:
  stops[]
  space
```

`startColor` / `endColor` may exist only as transitional authoring shorthand or should be migrated out of canonical fixtures.

### `applyTo`

Accept standard channel target:

```text
foreground
background
both
```

Use consistently across filters, shaders, and style effects where applicable.

### `shader.borderSweep.position`

Accept optional:

```text
position: ValueSource<number 0..1>
```

Semantics:

```text
if position exists:
  direct sweep progress

else:
  derive progress from speed and time
```

## 9. Graph I/O and sequence/parallel are schema-level runtime IR

Accept graph I/O as first-class schema semantics.

Sequence:

```text
child N+1 sees mutations and graph values emitted by child N
```

Parallel:

```text
branches read the same input snapshot
branch surfaces merge at the join
graph values merge at the join
conflicts use explicit policy or authored-order default with diagnostics
```

I/O shape:

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

This unblocks:

```text
complex_filter_to_mask_sourced_output.json
content_typewriter_io_filter_shader.json
scene_layer_visibility_binding_io.json
complex_nested_parallel_sequences.json
```

## 10. Shadow and subcell records are holdbacks, not schema blockers

Classify shadows and subcell shapes as:

```text
backendRenderer holdback
```

until a render-backend packet defines:

```text
shadow descriptor
subcell descriptor
backend/compositor adapter
visual evidence policy
```

Do not wire the compositor in this packet.

## 11. Templates are mandatory but compile-time

Template support is mandatory, but it lives above the runtime/player layer.

The model is:

```text
template + slots + overrides + mixins
  -> expanded canonical v3.1 recipe
  -> strict validation
  -> runtime/player evidence
```

Runtime/player must never see unresolved template inheritance.

Final canonical v3.1 runtime recipes contain:

```text
no extends
no unresolved inherited fields
no template refs required for execution
```

K2.13 should document this as mandatory compiler-layer work and ensure schema decisions do not preclude it. Heavy template implementation can be deferred.

## 12. Studio controls are generated from schema/descriptor data

The later studio UI should not need a bespoke control manifest invented from scratch.

Controls should be derivable from:

```text
graph.parameters
graph.signals
source descriptors
effect descriptors
ValueSpec
range
allowedValues
unit
semantic
runtimeMutability
bindable/sourceable status
description
```

K2.13 must ensure the schema/API docs and descriptor data are rich enough to support that future control surface.

---

# Work model: 8 parallel lanes

The orchestrator should run this as a multi-lane implementation packet.

```text
A. schema-readiness control surface
B. source/content descriptor decisions
C. runtime dynamism model
D. scene/element/layer semantics
E. motion/easing/scope vocabulary
F. primitive field-coverage closure
G. graph I/O and complex local-pipeline semantics
H. docs, schemars/API generation, templates, studio preflight
```

Each lane must classify every offender it touches as:

```text
acceptedSchema
descriptorBacklog
adapterBacklog
backendHoldback
guiHumanReviewHoldback
oracleOnly
duplicateVariant
explicitOwnerDecisionNeeded
```

No lane may return generic `ownerAudit`, `unknown`, or vague “needs decision” without exact decision text.

---

# Lane A — Schema-readiness control surface

## Objective

Make `schema-readiness --include-offenders` the authoritative schema-lock gate.

## Required changes

Add or refine additive report fields:

```text
offenders[].disposition
offenders[].schemaBlocking
offenders[].holdbackSignedOff
offenders[].exactDecisionRequired
offenders[].recommendedNextAction
summary.unresolvedSchemaBlockers
summary.signedOffHoldbacks
summary.explicitOwnerDecisionNeeded
summary.canDeclareSchemaReady
```

Keep schema label:

```text
v3.1.player.schemaReadiness.1
```

unless a breaking report change is absolutely necessary.

## Required behavior

The report must distinguish:

```text
schema blocker
descriptor backlog
adapter backlog
backend holdback
oracle-only holdback
duplicate/variant
```

## Tests

Add RED/GREEN tests proving:

```text
- --include-offenders emits disposition for every offender.
- no offender row has generic ownerAudit.
- no offender row has generic unknown.
- canDeclareSchemaReady is false if unresolved schema blockers remain.
- descriptorPack rows do not block schema readiness once marked descriptorBacklog.
- backendRenderer rows do not block schema readiness once marked backendHoldback and signed off.
```

---

# Lane B — Source/content descriptor decisions

## Objective

Resolve source/content identity blockers.

## Required decisions to implement/document

Accept:

```text
source.text
source.ansi
source.image
source.procedural
```

Accept content effect descriptor domain:

```text
content.typewriter
content.splitFlap
content.odometer
content.marquee
content.scramble
content.morph
content.redact
content.glyphCascade
content.glyphParticles
```

Classify command capture as:

```text
oracleOnly
```

## Required mappings

Use these representative files:

```text
content/content_typewriter.json
content/content_split_flap.json
content/content_odometer.json
content/content_typewriter_io_filter_shader.json
scene/ansi_source_chain.json
scene/scene_image_source_bindable.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
fixtures/command_capture_chain.capture.json
```

## Implementation scope

Low-friction descriptor additions are allowed when semantics are clear.

Safe additions:

```text
source.ansi
source.image
source.procedural
```

Content effect descriptors may be added as schema/descriptor definitions even if player adapters remain backlog.

Do not implement runtime command execution.

## Required outputs

Create or update:

```text
docs/new_kernel/K2_13_SOURCE_CONTENT_DECISION_REPORT.md
```

with:

```text
- accepted source descriptors
- accepted content effect descriptors
- per-family offender count changes
- command-capture oracle policy
- remaining exact blockers, if any
```

---

# Lane C — Runtime dynamism model

## Objective

Resolve binding, signal, parameter, graph value, value source, and lifecycle trigger blockers.

## Required decisions to implement/document

Accept the four-part model:

```text
Parameter
Signal
GraphValue
Binding
```

Accept value-source families:

```text
literal
parameter
signal
graphValue
map/remap
sampledField
```

Map event dwell demos to lifecycle triggers:

```text
integer_binding_demo -> predicate nonZero
text_binding_demo -> predicate nonEmpty
bool_binding_demo -> predicate isTrue
```

## Representative files

```text
bindable_rates/typewriter_speed_variance_bindable.json
event_driven_dwell/integer_binding_demo.json
event_driven_dwell/text_binding_demo.json
signals/single_oscillator_intensity_signal.json
filters/filter_dim_sample_surface_angle_from.json
shaders/compositions/shader_border_sweep_position_binding.json
scene/scene_layer_visibility_binding_io.json
```

## Required implementation

At minimum:

```text
- schema-readiness reclassifies these as acceptedSchema / descriptorBacklog / adapterBacklog.
- lifecycle trigger mapping is documented and test-covered.
- sampledField is not misclassified as signal.
- loopback is not treated as canonical runtime inheritance.
```

Signal generators may be classified as:

```text
acceptedSchema
```

only if modeled explicitly as signal-producing runtime/previews nodes with deterministic semantics.

Otherwise classify exact records as:

```text
explicitOwnerDecisionNeeded
```

with the exact unresolved signal-generator question.

## Required outputs

Create:

```text
docs/new_kernel/K2_13_RUNTIME_DYNAMISM_DECISION_REPORT.md
```

---

# Lane D — Scene / element / layer semantics

## Objective

Lock scene support as part of schema readiness.

## Required decisions to implement/document

Accept:

```text
Scene
Element / Layer
Placement
z / authoring order
visibility predicate
surface/base_style
cell_motion
local pipeline
clip/overflow policy
```

Define:

```text
local coordinates
scene/global coordinates
root pipeline order
layer-local pipeline order
overlap behavior
role propagation
skip/transparent behavior
diagnostic attribution
```

## Representative files

```text
content/content_cell_motion_slice.json
scene/ansi_source_chain.json
scene/scene_layer_nested_parallel_sequences.json
scene/scene_layer_visibility_binding_io.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
```

## Required behavior

Scene records should stop being vague `sceneSemantics` blockers if their only issue is now accepted scene/layer semantics.

They may become:

```text
acceptedSchema
descriptorBacklog
adapterBacklog
backendHoldback
explicitOwnerDecisionNeeded
```

but not generic scene blockers.

## Required outputs

Create:

```text
docs/new_kernel/K2_13_SCENE_ELEMENT_LAYER_DECISION_REPORT.md
```

Also update the feature checklist with multi-element scene questions.

---

# Lane E — Motion/easing/scope vocabulary

## Objective

Resolve motion/easing and style-scope blockers.

## Required decisions

Accept `EasingSpec` with named easings and cubic Bézier.

Accept `MotionRouteSpec` with canonical route names.

Normalize aliases:

```text
infinity -> figureEight
```

Accept scope vocabulary:

```text
moduloRows
moduloColumns
nonEmpty
outerBand
inner/interior
```

Restrict arbitrary predicate refs.

## Representative files

```text
easings/ease_bezier_custom.json
motion_routes/motion_figure_eight_infinity.json
styles/style_modulo_horizontal_every_third_row.json
styles/style_predicate_interior.json
```

## Required outputs

Create:

```text
docs/new_kernel/K2_13_MOTION_SCOPE_DECISION_REPORT.md
```

The report must explicitly say:

```text
- which easing/route forms are accepted,
- which aliases normalize away,
- which scope forms are accepted,
- whether predicate refs are restricted or held back.
```

---

# Lane F — Primitive field-coverage closure

## Objective

Close the four exact field-coverage blockers.

## Required decisions

Accept:

```text
shader.linearGradient.gradient
shader.linearGradient.applyTo
shader.borderSweep.position
```

## Representative files

```text
shaders/primitives/shader_linear_gradient_apply_to_both.json
shaders/compositions/shader_border_sweep_position_binding.json
```

## Required implementation

Update descriptor coverage and field-handling so these fields no longer appear as unsupported.

Rules:

```text
gradient is canonical for linear gradient.
applyTo is foreground/background/both.
position is optional ValueSource<number 0..1>.
position overrides speed-derived progress when present.
```

Do not mark fields handled without actual descriptor/adapter/report semantics.

## Required outputs

Create:

```text
docs/new_kernel/K2_13_FIELD_COVERAGE_CLOSURE_REPORT.md
```

---

# Lane G — Graph I/O, sequence/parallel, and complex normalization

## Objective

Resolve complex records that are blocked by local graph semantics rather than true schema gaps.

## Required decisions

Lock graph execution semantics:

```text
Sequence:
  sequential mutation and graph-value propagation

Parallel:
  branch snapshot isolation
  join-time surface merge
  join-time graph-value merge
  conflict diagnostics
```

## Representative files

```text
complex/complex_filter_to_mask_sourced_output.json
complex/complex_nested_parallel_sequences.json
content/content_typewriter_io_filter_shader.json
scene/scene_layer_nested_parallel_sequences.json
scene/scene_layer_visibility_binding_io.json
```

## Required behavior

Complex records must not remain broad owner-audit equivalents.

Classify each touched complex offender as:

```text
acceptedSchema
descriptorBacklog
adapterBacklog
backendHoldback
guiHumanReviewHoldback
oracleOnly
explicitOwnerDecisionNeeded
```

## Required outputs

Create:

```text
docs/new_kernel/K2_13_COMPLEX_GRAPH_IO_DECISION_REPORT.md
```

The report must list every remaining complex offender path that still cannot be resolved and the exact decision required.

---

# Lane H — Docs, schemars/API generation, templates, and studio preflight

## Objective

Put schema/API documentation and future studio control generation on firm ground.

## Required work

Inspect existing schema/API doc infrastructure.

At minimum, confirm or implement:

```text
- JSON schema generation still works.
- generated schema includes descriptions from rustdoc / schemars.
- strict schema tests still pass.
- new/changed types have rustdoc.
- new/changed schema fields have schemars descriptions where appropriate.
```

Create or update docs explaining:

```text
template composition is mandatory but compile-time
final canonical recipe contains no unresolved template refs
studio controls derive from descriptors + graph.parameters/signals
scene support is part of schema readiness
```

## Required docs

Create:

```text
docs/new_kernel/K2_13_SCHEMA_API_DOC_INFRA_REPORT.md
docs/new_kernel/K2_13_TEMPLATE_COMPOSITION_DECISION_NOTE.md
docs/new_kernel/K2_13_STUDIO_CONTROL_SURFACE_PREFLIGHT.md
```

## Studio preflight

Do not build the full studio UI in K2.13.

Instead, implement or document the derivation contract:

```text
RecipeDocument + descriptor packs
  -> parameters/signals/source inputs/effect inputs
  -> typed control metadata
  -> future sliders/input boxes/selects/toggles
```

Controls must derive from:

```text
displayName
description
ValueKind
default
range
allowedValues
unit
semantic
runtimeMutability
bindable/sourceable
```

---

# Low-friction fixture policy

K2.13 may add canonical v3.1 fixtures only when all of these are true:

```text
- descriptor already exists or is accepted in this packet,
- player/report field coverage is honest,
- no schema uncertainty is hidden,
- fixture-qc remains pass,
- legacy root remains untouched,
- the status memo lists the fixture and why it was safe.
```

Good candidates:

```text
source.ansi basic fixture
source.image fallback fixture
event dwell integer/text canonical fixtures
linear gradient applyTo fixture
border sweep position fixture
modulo rows scope fixture
interior scope fixture
```

Bad candidates for this packet:

```text
command capture runtime playback
arbitrary predicate registry fixtures
shadow/subcell parity fixtures
compositor-backed fixtures
native-only complex visual recipes
```

---

# Required testing strategy

Start with RED tests for:

```text
schema-readiness --include-offenders has dispositions for every offender
schema-readiness has no generic ownerAudit offender rows
schema-readiness has no generic unknown offender rows
schema-readiness distinguishes schema blockers from descriptor backlog
schema-readiness distinguishes backend holdbacks from schema blockers
field coverage no longer flags gradient/applyTo/position once implemented
source/content records do not collapse content effects into source.card/source.text
event dwell integer/text classify as lifecycle trigger semantics, not pipeline timing
scene local pipeline records classify under accepted scene semantics
```

Regression tests:

```text
existing render-recipe still passes canonical corpus
render-frame still passes canonical corpus
fixture-qc still passes canonical corpus
primitive-field-coverage still passes canonical corpus
primitive-adapter-gap still passes canonical corpus
migration-mapping-batch recursive still passes
legacy debug_recipes root remains unmodified
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

Run formatting:

```bash
cargo fmt \
  --package tui-vfx-player \
  --package tui-vfx-player-cli \
  --package tui-vfx-player-ui \
  --package tui-vfx-contract \
  --package tui-vfx-contract-cli \
  -- --check
```

Run clippy:

```bash
cargo clippy \
  -p tui-vfx-player \
  -p tui-vfx-player-cli \
  -p tui-vfx-player-ui \
  -p tui-vfx-contract \
  -p tui-vfx-contract-cli \
  --all-targets -- -D warnings
```

Run tests:

```bash
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test -p tui-vfx-player-ui
cargo test -p tui-vfx-contract
cargo test -p tui-vfx-contract-cli
cargo test --workspace
```

Run report gates:

```bash
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

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
  crates/tui-vfx-contract \
  crates/tui-vfx-contract-cli \
  docs/new_kernel
```

The legacy recipe root must remain untouched.

---

# Acceptance criteria

## Required

```text
- schema-readiness --include-offenders emits a disposition for every offender.
- Generic ownerAudit offender rows are gone.
- Generic unknown offender rows are gone.
- Source/content records are classified using the source/content split.
- Scene records are classified using accepted scene/element/layer semantics or exact unresolved decisions.
- Binding/signal/value-source records are classified using Parameter / Signal / GraphValue / Binding.
- Event-driven dwell integer/text map to lifecycle trigger predicates.
- Motion/easing records are classified under accepted motion/easing vocabulary or exact unresolved decisions.
- Style scope records are classified under accepted built-in scopes or exact predicate-registry holdback.
- gradient/applyTo/position field-coverage blockers are closed or exactly held back.
- Complex records no longer hide behind generic owner-audit.
- Backend/gui/oracle/duplicate records have explicit holdback disposition.
- Template composition is documented as mandatory compile-time authoring/compiler work.
- Schema/API doc generation infrastructure is checked and reported.
- Fixture-QC passes canonical corpus.
- Legacy debug_recipes root remains read-only.
```

## Preferred

```text
- canDeclareSchemaReady becomes true.
- If canDeclareSchemaReady remains false, remaining blockers are fewer than 10 exact owner decisions.
- At least one safe source.ansi or source.image fixture is added.
- Event-driven dwell integer/text canonical fixtures are added if lifecycle trigger mapping is accepted.
- Studio control preflight can list controls for at least one canonical recipe from descriptors/parameters/signals.
```

## Explicit stop conditions

Stop and report exact blockers rather than forcing green output if:

```text
- arbitrary predicate refs require a hidden runtime DSL,
- signal loopback cannot be separated from runtime signal generation,
- source.ansi requires terminal lifecycle behavior rather than ANSI data parsing,
- source.image requires unbounded runtime asset execution,
- procedural sources require arbitrary host code execution,
- scene overlap semantics cannot be made deterministic,
- graph parallel merge semantics would hide output conflicts,
- field coverage can pass only by listing fields without semantics,
- template references would remain in canonical runtime recipes,
- compositor wiring becomes necessary to make a schema claim.
```

---

# Required status memo

Return:

```text
docs/new_kernel/PHASE_K2_13_SCHEMA_DECISION_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md
```

The memo must include:

```text
- executive summary
- direct answer: can we declare 100% schema readiness now?
- if yes, exact evidence and remaining non-schema backlog
- if no, exact remaining owner decisions by path
- lane-by-lane implementation summary
- schema-readiness before/after counts
- offender disposition counts
- source/content decisions
- runtime dynamism decisions
- scene/element/layer decisions
- motion/easing/scope decisions
- field-coverage closure results
- complex graph I/O normalization results
- holdback signoff table
- template composition note
- studio control preflight status
- schema/API docs generation status
- optional fixture additions
- verification matrix
- legacy recipe mutation status
- recommended next packet
```

The recommended next packet should be one of:

```text
K2.14 descriptor expansion tranche
K2.14 source/content adapter tranche
K2.14 canonical fixture migration tranche
K3.0 studio control surface pilot
```

Do not recommend compositor backend wiring unless K2.13 proves schema/source/runtime decisions are no longer the gating issue.

---

# Final instruction to orchestrator

This is a decision-and-burn-down packet, not another mapping packet.

The desired outcome is:

```text
schema-readiness reports 100% schema-decision readiness
```

or:

```text
schema-readiness reports a short, exact, owner-actionable list of unresolved decisions.
```

Broad unresolved categories are no longer acceptable.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_12.md</FILE> - <DESC>Architect response defining K2.13 schema decision and offender burn-down packet</DESC> -->

<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->

```
```
