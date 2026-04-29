# Architect Memo for Orchestrator — K2.13 Schema Settlement and High-Confidence Migration Unblocker Sprint

Date: 2026-04-29
Scope: `/usr/projects/tui-vfx` plus canonical v3.1 fixtures under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes`
Legacy evidence root: `/usr/projects/tui-vfx-recipes/recipes/debug_recipes` remains read-only
Packet intent: stop repeating blocker categories, make architectural decisions, clear low-friction clusters, and establish a credible path to 100% schema readiness.

> **Terminology note:** this extension memo was folded into the K2.13 execution packet after the original K2.12 response. The implementation normalizes this memo's provisional readiness labels into the final report dispositions used throughout code and generated evidence: `acceptedSchema`, `descriptorBacklog`, `adapterBacklog`, `backendHoldback`, `guiHumanReviewHoldback`, `oracleOnly`, `duplicateVariant`, and `explicitOwnerDecisionNeeded`.

---

## 1. Executive decision

K2.12 was useful because it made the blocker ledger mechanical. It is not enough. The next packet must convert the ledger into concrete schema decisions and implementation.

The correct next phase is:

```text
K2.13 — Schema Settlement + High-Confidence Migration Unblocker Sprint
```

This packet should run **6–8 parallel lanes**. It should not baby-step one blocker class at a time. It should make normative decisions, update the schema/descriptor/player/mapping surfaces accordingly, add safe canonical fixtures where justified, and explicitly hold back items that are not appropriate to solve now.

The core rule:

```text
Proceed aggressively on low-friction, schema-compatible items.
Hold back problematic/backend/oracle/human-review items explicitly.
Do not make reports green by hiding semantics.
```

---

## 2. Direct answer: when can we claim 100% schema readiness?

We can claim **100% schema readiness for existing debug recipe migration** when every legacy debug recipe record is in one of these states:

```text
schemaAccepted
canonicalExists
descriptorBacklogAccepted
sourceBacklogAccepted
adapterBacklogAccepted
backendHoldbackAccepted
oracleOnlyAccepted
duplicateOrVariantAccepted
guiHumanReviewAccepted
```

and **zero** records remain in unresolved states such as:

```text
ownerAudit
unknown
schemaDecisionNeeded
sourceDecisionNeeded
sceneSemantics
bindingSemantics
valueSourceSemantics
motionTimingSemantics
lifecycleSemantics
fieldCoverage
blockedByAmbiguousLegacyIntent
notYetClassified
```

This does **not** mean every recipe has been ported. It means the v3.1 schema and descriptor model can honestly classify every legacy debug recipe as either representable, queued for descriptor/adapter work, or explicitly held back for backend/oracle/human-review reasons.

Current K2.12 evidence still says:

```text
schema readiness: NOT YET
offenders: 386
estimated readiness: 36.0%
```

K2.13 should aim to make a real readiness declaration possible by splitting the offender ledger into:

```text
schemaBlockers: must be resolved before schema lock
migrationBacklog: does not block schema lock after owner signoff
acceptedHoldbacks: does not block schema lock after owner signoff
```

If K2.13 completes the decisions below and the ledger proves `schemaBlockers=0`, we can declare:

```text
SCHEMA READINESS: YES, with accepted migration backlog and holdbacks.
```

If not, the status memo must list the exact remaining blockers and why they could not be settled.

---

## 3. Normative architecture decisions for K2.13

The implementer may challenge these only with concrete conflicting evidence from the corpus. Otherwise treat them as the working architectural decisions.

### 3.1 Source vs content vs effect

Do **not** overload `source.card` or `source.text`.

Use this model:

```text
Source:
  Produces an initial local surface.
  Examples: source.text, source.card, source.ansi, source.image, source.procedural.

Content effect:
  Transforms/reveals/emits glyph content before or alongside the graph pipeline.
  Examples: typewriter, scramble, morph, marquee, split_flap, odometer, wrap_indicator, glyph_particles.

Graph effect:
  Runs over a surface using mask/sampler/filter/shader/style semantics.
```

Concrete mapping:

| Legacy examples                                     | K2.13 decision                                                               |
| --------------------------------------------------- | ---------------------------------------------------------------------------- |
| `content_typewriter`, bindable typewriter speed     | `content.typewriter` descriptor/effect, not a source.                        |
| `content_scramble`, `content_morph`                 | Content transform descriptors.                                               |
| `content_marquee`                                   | Content transform with loop clock; not source-level scrolling.               |
| `content_split_flap`, `content_odometer`            | Content transform descriptors with mechanical/tile fields.                   |
| `content_wrap_indicator`                            | Low-friction content transform descriptor.                                   |
| `content_glyph_particles_*`                         | Content emitter descriptor; may be held if emitter lifecycle is too broad.   |
| `ansi_source_chain`                                 | `source.ansi`, then graph pipeline.                                          |
| `scene_image_source_bindable`                       | `source.image` with asset resolver/fallback policy.                          |
| `scene_authoring_ladder_procedural_spinner_binding` | `source.procedural` with source id and params; no runtime command execution. |
| `command_capture_chain`                             | Offline/oracle artifact only. No runtime command execution.                  |

### 3.2 Runtime dynamism

Use a clear split:

```text
Parameter:
  user/studio-adjustable set-and-hold value.

Signal:
  sampled value over time, host event, host progress, lifecycle trigger input, or preview loopback generator.

GraphValue:
  value emitted by an earlier graph step through explicit I/O.
```

Canonical strict v3.1 should not preserve vague legacy `binding` vocabulary forever. Migration tooling may read legacy shapes, but final canonical recipes should lower them into explicit value sources.

Recommended canonical concepts:

```text
ValueSource::Literal
ValueSource::Parameter
ValueSource::Signal
ValueSource::GraphValue
ValueSource::Map
ValueSource::Computed / SurfaceSample   # add only if existing value-source cannot express sample_surface_* cleanly
```

Concrete decisions:

| Pattern                                                       | Decision                                                                                                                   |
| ------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `speed_variance: { "binding": "typing_jitter" }`              | Parameter source if set-and-hold studio control.                                                                           |
| `progress: { "binding": "demo_progress" }` with ramp loopback | Signal source with preview loopback generator.                                                                             |
| Event dwell `dismiss_count` non-zero                          | Signal source + `nonZero` predicate.                                                                                       |
| Event dwell `dismiss_reason` non-empty                        | Signal source + `nonEmpty` predicate.                                                                                      |
| `position: { "binding": "sweep_progress" }`                   | Signal/value source input on `shader.borderSweep.position`.                                                                |
| `factor: { remap(sample_surface_angle_from(...)) }`           | Computed/surface-sample value source or explicit signal expression. Do not hide as plain number.                           |
| Runtime-bound scope cell x/y                                  | Dynamic scope coordinates are allowed only where explicitly typed as numeric value sources with fallback and quantization. |
| Loopback                                                      | Preview fallback generator. Host value wins. Loopback must not become production runtime authority.                        |

### 3.3 Scene / element / layer semantics

Scene support is no longer optional. It must be part of ongoing validation and schema finalization.

Lock this model:

```text
Scene:
  collection of elements/layers that compose into a final surface.

Element/layer:
  stable identity, source, placement, z-order, visibility, local surface, optional local pipeline.

Surface:
  cells + styles + roles; no element identity baked into RoleTag.
```

Rules to implement/document:

```text
- Element identity is not RoleTag.
- Role is semantic class: text, border, shadow, background, icon.
- Element/layer id is instance identity: card, spinner, backdrop, label.
- Layer-local pipelines run in element-local coordinates.
- Root pipeline runs after scene composition, in scene/global coordinates.
- Scene order is z ascending, then authored order for ties.
- Transparent/empty source cells skip by default and preserve lower content.
- Explicit writes overwrite lower content unless write policy says skip/preserve.
- Role propagates from the writing element; skipped cells preserve lower role.
- Visibility predicates evaluate before source render and local pipeline.
- Diagnostics include scene layer id, source id, pipeline step path, and scope.
```

This directly addresses:

```text
scene_layer_nested_parallel_sequences
scene_layer_visibility_binding_io
content_cell_motion_slice
subcell shape scene layers
procedural spinner scene
image source scene
ANSI scene
```

### 3.4 Graph sequence / parallel / I/O semantics

Lock this, because several complex fixtures already depend on it:

```text
Sequence:
  child N+1 reads the surface and graph values produced by child N.

Parallel:
  all branches read the same pre-parallel input snapshot.
  branch outputs merge at the join.
  overlapping cell writes resolve by authored child order unless a stricter merge policy is declared.
  graph-value hint conflicts must be diagnosed or resolved by explicit policy.
```

This supports:

```text
complex_filter_to_mask_sourced_output
v3_io_parallel_merge_shader
complex_nested_parallel_sequences
complex_parallel_overlap_conflict_snapshot
scene_layer_nested_parallel_sequences
scene_layer_visibility_binding_io
```

### 3.5 Scope vocabulary

Accept these scope classes for v3.1 schema readiness:

```text
all
role
channel
rect
rowRange
columnRange
cell
cells
outer
inner
modulo
content/nonEmpty
builtInPredicate
```

Important decision:

```text
Do not introduce an arbitrary open predicate registry yet.
```

Instead:

```text
predicate: interior
```

should canonicalize to a built-in selector equivalent to interior/inner cells, or be represented as:

```text
scope: { kind: "inner", margins: { top: 1, right: 1, bottom: 1, left: 1 } }
```

Custom predicate refs remain holdbacks until a registry is designed.

Dynamic scope coordinates are allowed only as typed numeric value sources with deterministic fallback and quantization.

### 3.6 Template support

Templates are mandatory, but they belong **above this layer**.

Decision:

```text
Templates are compile-time composition.
Runtime never sees inheritance.
Strict canonical v3.1 recipes contain no template references.
```

K2.13 should add this to docs/checklists and schema-roadmap, not implement runtime inheritance.

Required wording:

```text
template + overrides + mixins
  -> deterministic expansion
  -> strict canonical v3.1 recipe
  -> validation/player/runtime
```

Template support must not block debug recipe schema readiness unless a debug recipe explicitly requires template expansion.

### 3.7 Holdback policy

K2.13 must stop counting these as schema blockers after explicit signoff:

| Cluster                      | Disposition                                                                                                                 |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| Shadows                      | Backend holdback unless only descriptor shape is needed.                                                                    |
| Subcell shapes               | Backend/procedural renderer holdback; source.procedural may describe them, but visual proof waits.                          |
| Terminal fire/water          | Descriptor backlog or backend-quality holdback; do not block core schema if fields are representable as shader descriptors. |
| Command capture              | Oracle-only/offline artifact. No runtime command execution.                                                                 |
| GUI visual conflict fixtures | Human-review holdback.                                                                                                      |
| Duplicate variants           | Duplicate/variant accepted.                                                                                                 |

---

## 4. K2.13 work model: eight parallel lanes

The orchestrator should run this as a coordinated sprint with 6–8 agents.

```text
A. Readiness ledger and disposition mechanics
B. Runtime dynamism and value-source model
C. Source/content descriptor and fixture tranche
D. Scene/element/layer and graph-local pipeline semantics
E. Primitive field-coverage closure and low-friction descriptor expansion
F. Holdback/oracle/backend disposition signoff
G. Schema/API/doc generation infrastructure
H. Studio control-manifest preflight
```

Each lane must include refactoring/rustdoc/schemars cleanup for every touched file.

---

# Lane A — Readiness ledger and disposition mechanics

## Objective

Turn `schema-readiness --include-offenders` into a true schema-lock control board.

## Required changes

Add or expose these fields in the report:

```text
schemaBlockingCount
migrationBacklogCount
acceptedHoldbackCount
unresolvedCount
canDeclareSchemaReady
readinessDisposition
blockingDecision
recommendedAction
confidence
```

Each offender row must have:

```text
legacyPath
family
offenderKind
readinessDisposition
blocksSchemaLock: bool
blockingDecision
recommendedAction
representativeCanonicalPath?
confidence
notes[]
```

## Required disposition vocabulary

```text
schemaAccepted
canonicalExists
descriptorBacklogAccepted
sourceBacklogAccepted
adapterBacklogAccepted
backendHoldbackAccepted
oracleOnlyAccepted
duplicateOrVariantAccepted
guiHumanReviewAccepted
schemaDecisionRequired
sourceDecisionRequired
runtimeDecisionRequired
sceneDecisionRequired
fieldCoverageDecisionRequired
notYetClassified
```

## Acceptance

K2.13 must make it possible to distinguish:

```text
cannot lock schema because model is undecided
```

from:

```text
schema can lock; this is descriptor/adapter/backend backlog
```

## Test gates

Add tests proving:

```text
- no offender row has ownerAudit or unknown after normalization,
- every offender row has readinessDisposition,
- canDeclareSchemaReady=false while unresolved schemaDecisionRequired rows exist,
- accepted backend/oracle/duplicate holdbacks do not count as schema blockers,
- descriptorBacklogAccepted does not count as schema blocker once explicitly accepted.
```

---

# Lane B — Runtime dynamism and value-source model

## Objective

Settle bindings, parameters, signals, loopbacks, computed surface values, dynamic scope values, and graph-value I/O.

This is the highest-leverage schema lane.

## Required decisions to implement

### B1. Parameters vs signals

Implement/document:

```text
parameter = set-and-hold studio/user control
signal = sampled/event/time-varying value
graphValue = emitted pipeline value
```

Legacy `requires_bindings` may remain in migration input but should be lowered to canonical parameter/signal declarations.

### B2. Preview loopback

Implement/document:

```text
loopback = preview fallback generator only
host value wins
no production runtime command/source execution
```

Loopback signal examples:

```text
ramp
sine
wave/envelope if already needed by signal fixtures
```

### B3. Dwell trigger predicates

Support canonical trigger predicates:

```text
isTrue
isFalse
truthy
nonZero
nonEmpty
equals
notEquals
greaterThan
lessThan
```

Use these to support:

```text
event_driven_dwell/integer_binding_demo.json
event_driven_dwell/text_binding_demo.json
```

### B4. Bindable effect fields

Descriptor inputs marked bindable must accept `ValueSource`, not only literal values.

Required target examples:

```text
typewriter.speedVariance
filter.pillButton.progress
filter.fadeToCanvas.canvasColor
shader.borderSweep.position
shader.highlighter.speed / direction / blendStrength
shader.focusField.centerX / centerY
shader.glistenBand.direction / blendStrength
shader.wayfindingNode.currentIndex
```

### B5. Dynamic scope fields

Allow dynamic numeric value sources for `cell.x`, `cell.y`, and carefully scoped rect/range values only where explicitly typed.

Rules:

```text
- fallback required
- quantization required for cell coordinates
- clamp to surface bounds
- diagnostics if runtime value is missing or wrong type
```

This supports:

```text
styles/style_cell_position_binding.json
```

### B6. Computed/surface-sample values

Settle `sample_surface_angle_from` and related value-source leaves.

For K2.13, either:

```text
add ComputedValueSource / SurfaceSample source kind
```

or prove existing `Signal` + `Map` can encode it cleanly.

Must support:

```text
filters/filter_dim_sample_surface_angle_from.json
```

without pretending it is a plain numeric factor.

### B7. Graph I/O values

Implement or verify sequence/parallel I/O behavior:

```text
io.outputs.source -> graph value hint
io.inputs.hint -> descriptor input
parallel branch outputs merge at join
later sequence nodes can consume merged hints
```

Required examples:

```text
complex/complex_filter_to_mask_sourced_output.json
complex/v3_io_parallel_merge_shader.json
content/content_typewriter_io_filter_shader.json
scene/scene_layer_visibility_binding_io.json
```

## Acceptance

After this lane, these blockers should be either schema accepted or specifically reduced:

```text
bindingSemantics
valueSourceSemantics
lifecycleSemantics
motionTimingSemantics where binding-related
fieldCoverage position binding
dynamic scope binding
```

## Tests

Add red/green tests using at least:

```text
event_driven_dwell/integer_binding_demo.json
event_driven_dwell/text_binding_demo.json
bindable_rates/typewriter_speed_variance_bindable.json
filters/filter_pill_button_progress_binding.json
filters/filter_fade_to_canvas_canvas_color_binding.json
filters/filter_dim_sample_surface_angle_from.json
shaders/compositions/shader_border_sweep_position_binding.json
styles/style_cell_position_binding.json
complex/v3_io_parallel_merge_shader.json
```

---

# Lane C — Source/content descriptor and fixture tranche

## Objective

Make source/content identity concrete, add low-friction fixtures, and avoid stuffing everything into `source.card`.

## Required source descriptors

Add or finalize:

```text
source.text
source.ansi
source.image
source.procedural
```

### `source.text`

Already added as descriptor. K2.13 should ensure canonical fixture and field coverage remain green.

### `source.ansi`

Decision:

```text
source.ansi accepts ANSI/VTE text as offline-authored data.
It parses into styled cells.
It does not execute terminal lifecycle behavior.
```

Required fixture source:

```text
scene/ansi_source_chain.json
```

### `source.image`

Decision:

```text
source.image references an asset key.
If no asset resolver exists, player emits deterministic fallback cells and diagnostics.
```

Required fixture source:

```text
scene/scene_image_source_bindable.json
```

### `source.procedural`

Decision:

```text
source.procedural references a registered procedural source id plus params.
No shell commands.
No runtime code loading.
Unknown procedural id emits explicit unsupported/fallback diagnostics.
```

Required fixture source:

```text
scene/scene_authoring_ladder_procedural_spinner_binding.json
```

## Required content descriptors

Low-friction content transform descriptors to add or queue as safe:

```text
content.typewriter
content.wrapIndicator
content.scramble
content.morph
content.marquee
```

Medium-risk descriptors to define but possibly not fully visually adapt:

```text
content.splitFlap
content.odometer
content.glyphParticles
```

## Content model

Use this order:

```text
source resolves base text/surface
content transform/emitter applies
scene/local pipeline applies
root graph pipeline applies
```

## Examples to classify

| File                                              | Decision                                                                        |
| ------------------------------------------------- | ------------------------------------------------------------------------------- |
| `content/content_typewriter.json`                 | Low-friction content descriptor + fixture candidate.                            |
| `content/content_wrap_indicator.json`             | Low-friction content descriptor + fixture candidate.                            |
| `content/content_scramble.json`                   | Content descriptor; deterministic seed.                                         |
| `content/content_morph.json`                      | Content descriptor; source string + progression.                                |
| `content/content_marquee.json`                    | Content descriptor using clock.loop.                                            |
| `content/content_split_flap.json`                 | Content descriptor, but fields are richer; migrate after base transform policy. |
| `content/content_odometer.json`                   | Content descriptor with structured travel/tile fields.                          |
| `content/content_glyph_particles_base_spray.json` | Content emitter; may require emitter lifecycle holdback if too broad.           |

## Acceptance

* `sourceDescriptor` offender count materially decreases.
* `source.text`, `source.ansi`, `source.image`, and `source.procedural` are either descriptor-backed or explicitly accepted as source backlog.
* At least 3 low-friction content fixtures are added if all gates pass.
* Command capture remains oracle-only.

---

# Lane D — Scene / element / layer and local pipeline semantics

## Objective

Lock scene support as part of schema finalization.

## Required implementation/doc decisions

Use this scene model:

```text
scene.layers[] or scene.elements[]
  id
  z
  roleTag
  placement
  source
  surface/baseStyle
  visibility
  pipeline
  clip/write policy
```

If both `layers` and `elements` exist in different parts of the codebase, choose a canonical term or define aliases only in migration tooling. Strict v3.1 should prefer one.

## Required semantics

```text
- source resolves into element-local surface
- element-local pipeline runs in element-local coordinates
- root pipeline runs on final scene surface
- z-order then authored order controls overlap
- transparent cells skip by default
- visibility predicate uses runtime value model from Lane B
- local pipeline I/O is local unless explicitly exported
- diagnostics include element/layer id
```

## Required examples

Use these as regression fixtures:

```text
content/content_cell_motion_slice.json
scene/scene_layer_nested_parallel_sequences.json
scene/scene_layer_visibility_binding_io.json
scene/ansi_source_chain.json
scene/scene_image_source_bindable.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
complex/complex_nested_parallel_sequences.json
complex/complex_parallel_overlap_conflict_snapshot.json
```

## Acceptance

* `sceneSemantics` offender rows are reduced or reclassified.
* Scene-local pipeline examples no longer sit in vague schema-decision buckets.
* Multi-element support is explicitly in validation/QC.
* No direct compositor dependency is introduced in the UI.

---

# Lane E — Primitive field closure and low-friction descriptor expansion

## Objective

Close exact field-coverage blockers and add descriptor clusters that are clearly schema-compatible.

## E1. Field coverage blockers

### `shader.linearGradient.gradient`

Decision:

```text
Accept full gradient object with stops[] and color space.
Existing startColor/endColor remain a shorthand or are canonicalized into gradient stops.
```

Descriptor fields:

```text
gradient
angleDeg
intensity
applyTo
```

`applyTo` values:

```text
foreground
background
both
```

Required examples:

```text
shaders/primitives/shader_linear_gradient_apply_to_both.json
shader_linear_gradient_diagonal.json
shader_linear_gradient_background_channel.json
```

### `shader.borderSweep.position`

Decision:

```text
position is optional ValueSource<number> in normalized 0..1 perimeter progress.
If omitted, speed + time drives position.
```

Required example:

```text
shaders/compositions/shader_border_sweep_position_binding.json
```

## E2. Low-friction descriptor expansion candidates

Proceed where input fields are clear and player can honestly report support or explicit limited visual evidence.

### Masks

Add or finalize descriptors:

```text
mask.pathReveal
mask.materialize
mask.noiseDither
```

Use examples:

```text
masks/mask_path_reveal.json
masks/mask_materialize_center.json
masks/mask_noise_dither.json
```

### Samplers

Add or finalize descriptors:

```text
sampler.shredder
sampler.faultLine
sampler.radialTwist
```

Use examples:

```text
samplers/sampler_shredder.json
samplers/sampler_faultline.json
samplers/sampler_radial_twist_v3.json
```

### Filters

Add or finalize descriptors:

```text
filter.crt
filter.patternFill
filter.pillButton
filter.fadeToCanvas
filter.matrixRain
```

Use examples:

```text
filters/filter_crt.json
filters/filter_pattern_fill.json
filters/filter_pill_button_progress_binding.json
filters/filter_fade_to_canvas_canvas_color_binding.json
filters/filter_matrix_rain.json
```

`filter.matrixRain` may be descriptor-only or coarse visual if full procedural field support is too broad. Do not overclaim visual parity.

### Shaders

Add descriptor backlog records, but do not over-implement procedural fire/water unless low-risk:

```text
shader.revealWipe
shader.highlighter
shader.focusField
shader.glistenBand
shader.wayfindingNode
shader.terminalFire
shader.terminalWater
```

Use examples:

```text
shaders/primitives/shader_reveal_wipe_corner_out_top_left.json
shaders/compositions/shader_highlighter_runtime_bindings.json
shaders/compositions/shader_focus_field_center_binding.json
shaders/compositions/shader_glisten_band_direction_blend_binding.json
shaders/compositions/shader_wayfinding_node_current_index_binding.json
shaders/primitives/shader_terminal_fire_v3.json
shaders/primitives/shader_terminal_water_v3.json
```

## Acceptance

* `fieldCoverage` count must drop to zero or be explicitly accepted as backlog with exact decisions.
* Primitive field coverage gate remains zero-gap for canonical v3.1 corpus.
* Descriptor expansion does not encode legacy aliases into strict schema.
* Snake-case legacy names may be read by migration tooling, but strict canonical v3.1 should use the chosen canonical field casing.

---

# Lane F — Holdback / oracle / backend disposition signoff

## Objective

Stop backend/oracle/human-review items from blocking schema readiness.

## Required accepted holdbacks

### Command capture

```text
oracleOnlyAccepted
```

No runtime command execution.

### Shadows

```text
backendHoldbackAccepted
```

Descriptor may exist, but visual fidelity waits for backend renderer.

Representative files:

```text
shadows/shadow_full_cell_transparent_offset.json
shadows/shadow_gradient_soft_layers.json
shadows/shadow_half_block_subcell_texture.json
```

### Subcell shapes

```text
backendHoldbackAccepted or source.procedural backlog
```

Representative files:

```text
subcell_shapes/fractional_inset_rect_v3.json
subcell_shapes/braille_rounded_rect_v3.json
```

### GUI conflict fixtures

```text
guiHumanReviewAccepted
```

Representative file:

```text
complex/complex_parallel_overlap_conflict_snapshot.json
```

But note: if this fixture is primarily proving parallel semantics, it may become schemaAccepted after Lane D locks overlap rules.

### Terminal fire/water

Classify as:

```text
descriptorBacklogAccepted
```

or:

```text
backendHoldbackAccepted
```

Do not leave as schema undecided if the descriptor shape is clear.

## Acceptance

* Backend/oracle/GUI holdbacks are listed in a checked-in decision doc.
* `schema-readiness` stops counting accepted holdbacks as schema blockers.
* Status memo lists all holdback categories and representative paths.

---

# Lane G — Schema/API/doc generation infrastructure

## Objective

Make schema and API documentation generation real, not ad hoc.

## Required tasks

1. Inspect current schema/doc generation infrastructure:

   * `tui-vfx-contract` schemars tests
   * `tui-vfx-contract-cli` export or validation commands
   * existing generated schema artifacts, if any
   * rustdoc comments on touched DTOs

2. Implement or update a first-class command if missing, for example:

```bash
cargo run -q -p tui-vfx-contract-cli -- export-schema --json
```

or whatever command already exists.

3. Generate or refresh docs:

```text
docs/new_kernel/V31_SCHEMA_REFERENCE.md
docs/new_kernel/V31_VALUE_SOURCE_AND_CONTROL_REFERENCE.md
docs/new_kernel/V31_SCENE_ELEMENT_LAYER_REFERENCE.md
docs/new_kernel/V31_TEMPLATE_COMPOSITION_ROADMAP.md
```

4. For every touched Rust DTO:

   * add rustdoc,
   * add schemars descriptions where appropriate,
   * prefer strict schema objects,
   * keep naming and casing explicit.

5. Add a feature checklist section for:

   * scene/element/layer semantics,
   * template composition,
   * runtime controls/studio manifest,
   * backend holdbacks.

## Acceptance

* Schema generation test suite passes.
* Generated docs or generated schema artifacts are reproducible.
* No newly touched public DTO lacks basic rustdoc.
* The status memo says exactly how to regenerate schema/API docs.

---

# Lane H — Studio control-manifest preflight

## Objective

Prepare for dynamic studio controls without building the full studio UI prematurely.

The reach goal is:

```text
load recipe
auto-generate sliders/input boxes
adjust values
see visual effect
```

This depends on runtime value semantics and descriptor metadata. K2.13 should create the control-manifest foundation if Lane B stabilizes the model.

## Proposed report command

If feasible:

```bash
cargo run -q -p tui-vfx-player-cli -- control-manifest \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json <recipe>
```

Schema label:

```text
v3.1.player.controlManifest.1
```

## Manifest fields

```text
recipeId
controls[]
```

Each control:

```text
id
label
sourceKind: parameter | signal | descriptorInput | sourceInput
valueKind
default
range
allowedValues
unit
semantic
runtimeMutability
targetPaths[]
previewLoopback?
```

## Sources of controls

```text
graph.parameters
graph.signals
descriptor inputs where bindable=true
source descriptor inputs where bindable=true
recipe-level declared controls / migrated requires_bindings
```

## Acceptance

This is optional if Lane B is not stable enough. If implemented, it must not invent controls from fields that do not have descriptor/type metadata.

---

## 5. Specific classification guidance from sampled problem recipes

The implementer should use this as the starting point for offender-row classification.

| Recipe                                              | Classification / action                                                                      |
| --------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `content_typewriter`                                | `content.typewriter`; low-friction descriptor/fixture.                                       |
| `content_wrap_indicator`                            | `content.wrapIndicator`; low-friction descriptor/fixture.                                    |
| `content_scramble`                                  | `content.scramble`; deterministic seeded transform.                                          |
| `content_morph`                                     | `content.morph`; source-string transform.                                                    |
| `content_marquee`                                   | `content.marquee`; depends on loop clock.                                                    |
| `content_split_flap`                                | `content.splitFlap`; richer descriptor, migrate after base content model.                    |
| `content_odometer`                                  | `content.odometer`; structured mechanical tile descriptor.                                   |
| `content_glyph_particles_base_spray`                | `content.glyphParticles`; content emitter, possible holdback if emitter lifecycle too broad. |
| `ansi_source_chain`                                 | `source.ansi` + root sequence pipeline.                                                      |
| `scene_image_source_bindable`                       | `source.image`; asset key binding + fallback.                                                |
| `scene_authoring_ladder_procedural_spinner_binding` | `source.procedural`; stock spinner + visibility predicate.                                   |
| `scene_layer_nested_parallel_sequences`             | Scene-local pipeline semantics.                                                              |
| `scene_layer_visibility_binding_io`                 | Scene visibility + local graph I/O + runtime binding.                                        |
| `integer_binding_demo`                              | Dwell trigger signal + nonZero predicate.                                                    |
| `text_binding_demo`                                 | Dwell trigger signal + nonEmpty predicate.                                                   |
| `typewriter_speed_variance_bindable`                | Parameter source for content input.                                                          |
| `single_oscillator_intensity_signal`                | Signal preview loopback; progress binding.                                                   |
| `filter_dim_sample_surface_angle_from`              | Computed/surface-sample value source + map/remap.                                            |
| `ease_bezier_custom`                                | Motion timing schema; cubic bezier accepted.                                                 |
| `motion_figure_eight_infinity`                      | Motion route descriptor; `infinity` alias canonicalizes to figureEight.                      |
| `style_modulo_horizontal_every_third_row`           | Accept modulo scope.                                                                         |
| `style_predicate_interior`                          | Canonicalize to built-in inner/interior scope; no open registry yet.                         |
| `shader_linear_gradient_apply_to_both`              | Add `gradient` and `applyTo` support.                                                        |
| `shader_border_sweep_position_binding`              | Add `position` ValueSource input.                                                            |
| `complex_nested_parallel_sequences`                 | Graph nested sequence/parallel semantics.                                                    |
| `complex_filter_to_mask_sourced_output`             | Graph I/O source output consumed by later mask.                                              |
| `filter_crt`                                        | Add `filter.crt`; step kind disambiguates from sampler CRT.                                  |
| `filter_matrix_rain`                                | Add descriptor; adapter may be coarse/backlog.                                               |
| `filter_pattern_fill`                               | Add descriptor; nested pattern payload.                                                      |
| `filter_pill_button_progress_binding`               | Runtime signal/progress binding.                                                             |
| `filter_fade_to_canvas_canvas_color_binding`        | Runtime color parameter/signal.                                                              |
| `mask_path_reveal`                                  | Add descriptor with path union.                                                              |
| `mask_materialize_center`                           | Add descriptor with origin/noise/chunk/soft edge.                                            |
| `mask_noise_dither`                                 | Add descriptor with matrix enum.                                                             |
| `sampler_shredder`                                  | Add descriptor with stripe/odd/even speeds.                                                  |
| `sampler_faultline`                                 | Add descriptor with seed/intensity/splitBias.                                                |
| `sampler_radial_twist_v3`                           | Add descriptor with twist/center/radiusFloor.                                                |
| `shader_highlighter_runtime_bindings`               | Bindable shader inputs.                                                                      |
| `shader_focus_field_center_binding`                 | Bindable center inputs + clock pulse.                                                        |
| `shader_glisten_band_direction_blend_binding`       | Bindable enum/string direction + blend.                                                      |
| `shader_wayfinding_node_current_index_binding`      | Bindable index + node list.                                                                  |
| `shader_reveal_wipe_corner_out_top_left`            | Descriptor with wipe direction enum/aliases in migration only.                               |
| `shader_terminal_fire_v3`                           | Descriptor backlog or backend-quality holdback.                                              |
| `shader_terminal_water_v3`                          | Descriptor backlog or backend-quality holdback.                                              |
| `style_cell_position_binding`                       | Dynamic scope value source.                                                                  |
| `style_outer_scope_band`                            | Accept outer scope.                                                                          |
| `shadow_full_cell_transparent_offset`               | Backend holdback accepted.                                                                   |
| `shadow_half_block_subcell_texture`                 | Backend holdback accepted.                                                                   |
| `fractional_inset_rect_v3`                          | Source.procedural/subcell backend holdback.                                                  |
| `braille_rounded_rect_v3`                           | Source.procedural/subcell backend holdback.                                                  |
| `loopback_pill_button_progress_ramp`                | Preview loopback signal semantics.                                                           |
| `v3_io_parallel_merge_shader`                       | Parallel graph-value merge semantics.                                                        |
| `complex_parallel_overlap_conflict_snapshot`        | Parallel overlap semantics or GUI review holdback if visual conflict remains.                |

---

## 6. Optional safe canonical fixtures

The implementer may add canonical v3.1 fixtures only if all are true:

```text
- descriptor exists or is added in this packet,
- all authored fields are handled or explicitly documented,
- player evidence is honest,
- primitive-field-coverage remains zero-gap,
- fixture-qc remains pass,
- no legacy root mutation,
- no runtime command execution,
- no visual parity claim beyond player evidence.
```

Recommended safe fixture tranche if gates allow:

```text
sources/source_ansi_basic.json
sources/source_image_fallback.json
sources/source_procedural_spinner.json
content/content_typewriter_basic.json
content/content_wrap_indicator_basic.json
content/content_scramble_basic.json
masks/mask_path_reveal.json
masks/mask_materialize.json
masks/mask_noise_dither.json
samplers/sampler_shredder.json
samplers/sampler_faultline.json
samplers/sampler_radial_twist.json
```

Do not add terminal fire/water, shadows, subcell shapes, or command capture as “passing visual fixtures” unless the evidence is honest and bounded.

---

## 7. Required docs of record

Create or update:

```text
docs/new_kernel/K2_13_SCHEMA_SETTLEMENT_DECISION_REPORT.md
docs/new_kernel/K2_13_RUNTIME_DYNAMISM_MODEL.md
docs/new_kernel/K2_13_SOURCE_CONTENT_MODEL.md
docs/new_kernel/K2_13_SCENE_ELEMENT_LAYER_MODEL.md
docs/new_kernel/K2_13_PRIMITIVE_FIELD_CLOSURE_REPORT.md
docs/new_kernel/K2_13_HOLDBACK_AND_ORACLE_SIGNOFF.md
docs/new_kernel/K2_13_SCHEMA_DOC_GENERATION_REPORT.md
docs/new_kernel/PHASE_K2_13_SCHEMA_SETTLEMENT_STATUS_MEMO_TO_ARCHITECT.md
```

Status memo must include:

```text
- schema readiness declaration: YES/NO
- schemaBlockingCount
- migrationBacklogCount
- acceptedHoldbackCount
- unresolvedCount
- offender kind before/after table
- exact remaining unresolved paths, if any
- source/content decisions
- runtime value decisions
- scene semantics decisions
- graph sequence/parallel/I/O decisions
- field coverage decisions
- optional fixtures added
- holdbacks accepted
- schema/API doc generation workflow
- studio control-manifest readiness
- verification matrix
- legacy root mutation status
- recommended next packet
```

---

## 8. Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

Run formatting and linting:

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

If implemented:

```bash
cargo run -q -p tui-vfx-player-cli -- control-manifest \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json"
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

Legacy root must remain read-only.

---

## 9. Stop conditions

Stop and report rather than forcing implementation if:

```text
- a field can pass coverage only by pretending it has semantics,
- source.ansi would require terminal-runtime behavior rather than parsing authored ANSI data,
- source.image would require real asset loading beyond fallback/resolver seam,
- procedural sources would require runtime code execution,
- computed value sources cannot be represented without a new schema variant,
- dynamic scopes become open-ended unbounded runtime selectors,
- predicate scope requires a generic predicate registry,
- shadows/subcell/terminal fire/water require compositor/backend fidelity,
- templates are implemented as runtime inheritance,
- strict canonical v3.1 would need to preserve legacy aliases.
```

---

## 10. Expected next packet after K2.13

If K2.13 reaches `schemaBlockers=0`, the next packet should be:

```text
K2.14 — Descriptor and Fixture Expansion From Accepted Backlog
```

If K2.13 still has unresolved runtime/scene/source schema blockers, the next packet should be a narrow continuation:

```text
K2.14 — Remaining Schema Blocker Closure
```

Do not prioritize compositor backend wiring until the schema ledger proves the remaining blockers are backend-rendering-specific rather than schema/source/runtime-model issues.
