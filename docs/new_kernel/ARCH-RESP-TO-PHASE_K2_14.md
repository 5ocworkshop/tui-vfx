# ARCH-RESP-TO-PHASE_K2_14.md

## Review verdict

**ACCEPT.**

K2.13 is a major pivot point. The schema decision phase for the existing `debug_recipes/` migration is now considered **approved for v3.1 schema decision readiness** based on the reported evidence:

```text
totalLegacyRecords: 603
unresolvedSchemaBlockers: 0
explicitOwnerDecisionNeeded: 0
remainingOwnerDecisionCount: 0
fieldCoverageBlockedRecords: 0
canDeclareSchemaReady: true
```

This does **not** mean the debug corpus is migrated or visually complete. It means the remaining work is now implementation, descriptor, adapter, fixture, backend, GUI-review, oracle, and documentation work — not unresolved schema design.

From this point forward, agents should stop re-litigating the schema unless a concrete contradiction is discovered in implementation. The work now moves to aggressive burn-down.

---

# Phase K2.14 — Descriptor / Adapter / Fixture Migration Tranche 1

## Executive goal

Use the K2.13 schema-readiness declaration to start removing the remaining migration blockers at scale.

The next packet should run **10 parallel lanes** against the outstanding `debug_recipes/` backlog and produce material migration progress:

```text
accepted v3.1 schema
  -> descriptor coverage
  -> player/source/graph adapter evidence
  -> canonical v3.1 fixture migration
  -> fixture-QC / field coverage / adapter gap gates
  -> GUI/human-review or backend holdback signoff where needed
```

This is no longer another decision-ledger packet. It is a descriptor/adapter/fixture/evidence packet.

The core rule:

```text
Proceed aggressively with low-friction and medium-friction clusters.
Hold back only genuinely problematic items, and classify each holdback exactly.
```

---

## Rolling context to include in implementer memo

Completed before this packet:

```text
K2.1   migration-gap
K2.2   visual-frame report
K2.3   primitive adapter burn-down
K2.4   styled-cell substrate
K2.5   styled primitive adapter burn-down
K2.6   GUI PRD, field coverage, timeline/diff
K2.7/8 UI root polish, fixture-QC, first fixture batch
K2.9   mask descriptor expansion
K2.10  corpus-wide mapping and backlog board
K2.11  schema-readiness ledger
K2.12  offender ledger, source.text fixture, complex/style normalization
K2.13  schema decision burn-down
```

Current K2.13 declaration:

```text
SCHEMA READINESS DECLARATION:
  APPROVED FOR v3.1 SCHEMA DECISION READINESS
```

Current remaining disposition counts:

```text
acceptedSchema:             125
descriptorBacklog:          263
backendHoldback:             15
guiHumanReviewHoldback:       2
oracleOnly:                 195
duplicateVariant:             3
```

This packet should reduce `descriptorBacklog` and thin source/content/graph evidence gaps. It should not reopen template/runtime/scene/schema design unless concrete implementation failure proves the accepted decision is inconsistent.

---

## Non-negotiable architectural guardrails

1. **Legacy `recipes/debug_recipes/` remains read-only evidence.**
   Do not modify legacy recipes.

2. **Canonical fixtures may be added only under:**

   ```text
   ../tui-vfx-recipes/recipes/v3.1/debug_recipes/
   ```

3. **Player evidence is honest evidence, not visual parity.**
   Do not claim parity with legacy rendering unless a future oracle comparison proves it.

4. **Descriptor support must be real.**
   Do not mark an input handled unless the descriptor, parser, adapter, field-coverage report, and tests prove it.

5. **Template support is mandatory but compile-time.**
   Runtime/player must never see unresolved template inheritance. No template implementation is required in this packet, but docs and schema tooling must preserve the compile-time template-composition boundary.

6. **Scene support is core v3.1 schema.**
   Scene/element/layer semantics must remain part of ongoing validation and fixture migration. Do not collapse element identity into role identity.

7. **No runtime command execution.**
   Command capture remains offline/oracle-only unless a future offline authoring/export packet is explicitly assigned.

8. **No direct compositor construction in UI.**
   `tui-vfx-player-ui` consumes `tui-vfx-player` evidence. A compositor backend, when it comes, must sit behind a player/backend adapter seam.

9. **No phase labels in durable public vocabulary.**
   K2.x labels are planning labels only. Do not encode them into schema names, descriptor ids, report values, or public API.

10. **Every touched file gets quality review.**
    While touching files, look for refactoring opportunities, reduce complexity, improve readability, update rustdoc, and add/verify schemars details where appropriate.

---

# Work model: 10 parallel lanes

```text
Lane A  Control surface, metrics, and QA coordinator
Lane B  Runtime value/binding/loopback evidence
Lane C  Content descriptors and adapters
Lane D  Source descriptors and adapters
Lane E  Graph execution and I/O evidence
Lane F  Scene / element / layer evidence
Lane G  Filters, masks, and samplers descriptor tranche
Lane H  Shader and style descriptor tranche
Lane I  Backend / GUI-human-review / oracle holdback signoff
Lane J  Schema/API/docs/studio-control infrastructure
```

Each lane must return:

```text
- files touched
- descriptors added/changed
- adapters added/changed
- fixtures added/changed
- tests added
- report deltas
- holdbacks with exact reasons
- refactor/docs/schemars notes
```

---

# Lane A — Control surface, metrics, and QA coordinator

## Objective

Coordinate the migration burn-down and make the before/after impact measurable.

This lane owns report consistency, status memo consolidation, and cross-lane gates.

## Required work

1. Capture baseline before changes:

   ```bash
   export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

   cargo run -q -p tui-vfx-player-cli -- schema-readiness \
     --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
     --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
     --descriptor-pack descriptors/v3.1/packs/primitive.json \
     --recursive \
     --include-offenders \
     --json

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
   ```

2. Capture the same gates after all lanes merge.

3. Ensure the status memo reports:

   ```text
   canonical fixture count before/after
   render-recipe total/rendered/unsupported/errors before/after
   render-frame total/rendered/unsupported/errors before/after
   fixture-qc status before/after
   field coverage before/after
   adapter gap before/after
   schema-readiness canDeclareSchemaReady after
   descriptorBacklog reduction, if report supports it
   explicit holdbacks added/signed off
   ```

4. Ensure no lane silently makes reports green by suppressing fields.

## Acceptance

Required:

```text
- Final status memo includes machine-readable before/after counters.
- schema-readiness remains canDeclareSchemaReady=true.
- fieldCoverageBlockedRecords remains 0.
- primitive-field-coverage usedInputFields == handledInputFields.
- fixture-qc passes for the full canonical v3.1 debug corpus.
```

Preferred:

```text
- Add or improve a migration-progress delta section in existing reports.
- Add a compact “descriptor backlog burn-down” table to docs.
```

---

# Lane B — Runtime value, binding, signal, and loopback evidence

## Objective

K2.13 accepted runtime dynamism semantics. This lane must prove those semantics through player evidence and canonical fixtures.

Focus on:

```text
Parameter
Signal
GraphValue
Binding
ValueSource
loopback
sampledField
enum/string/number/color bindings
```

## Target legacy evidence

Use these as representative inputs:

```text
event_driven_dwell/integer_binding_demo.json
event_driven_dwell/text_binding_demo.json
bindable_rates/typewriter_speed_variance_bindable.json
signals/single_oscillator_intensity_signal.json
filters/filter_dim_sample_surface_angle_from.json
filters/filter_pill_button_progress_binding.json
filters/filter_fade_to_canvas_canvas_color_binding.json
loopback/loopback_pill_button_progress_ramp.json
shaders/compositions/shader_border_sweep_position_binding.json
shaders/compositions/shader_highlighter_runtime_bindings.json
shaders/compositions/shader_focus_field_center_binding.json
shaders/compositions/shader_glisten_band_direction_blend_binding.json
shaders/compositions/shader_wayfinding_node_current_index_binding.json
styles/style_cell_position_binding.json
```

## Required decisions to encode in implementation

Use K2.13 semantics:

```text
Binding is not a value class.
Binding resolves to a typed value source.
Loopback is preview/offline fallback.
Signals are time/sample-dependent producers.
GraphValue is emitted by graph nodes.
sampledField is deterministic per-cell spatial field.
```

## Required work

1. Ensure player/runtime resolution supports at least:

   ```text
   number binding with literal default
   string binding with literal default
   bool binding with literal default
   color binding with literal default
   loopback ramp
   loopback sine
   sampledField number
   binding into descriptor input
   binding into accepted optional field
   binding into scope coordinate only if accepted by current schema
   ```

2. Add focused tests for:

   ```text
   integer dwell terminates on nonzero
   text dwell terminates on nonempty
   loopback ramp changes over timeline samples
   color binding default resolves for fade_to_canvas
   borderSweep.position accepts bound/default value
   sampledField emits deterministic numeric values
   ```

3. Add canonical v3.1 fixtures only where descriptor/adapters are ready.

## Acceptance

Required:

```text
- Runtime-dynamism fixtures validate.
- render-timeline proves loopback/signal values change over time where expected.
- render-frame-diff shows at least one binding/loopback-driven visual delta.
- schema-readiness still has unresolvedSchemaBlockers=0.
```

Preferred:

```text
- Add a compact `runtime-input-evidence` section to fixture-qc or schema-readiness output.
- Migrate at least 4 runtime-dynamism canonical fixtures.
```

Stop condition:

```text
If an implementation would turn loopback into runtime command execution or blur Binding/Signal/GraphValue, stop and report.
```

---

# Lane C — Content descriptors and adapters

## Objective

Start migrating content effects now that source/content split is accepted.

The goal is not to port every content effect in one pass. The goal is to establish real descriptor + adapter + fixture evidence for the first content tranche.

## Target content descriptors

Required first tranche:

```text
content.typewriter
content.marquee
content.splitFlap
```

Preferred additions if low friction:

```text
content.wrapIndicator
content.scramble
content.morph
```

Hold until later unless clearly bounded:

```text
content.odometer
content.glyphParticles
content.glyphCascade
content.redact
content.numeric
content.slideShift
content.dissolve
```

## Representative legacy recipes

```text
content/content_typewriter.json
content/content_marquee.json
content/content_split_flap.json
content/content_wrap_indicator.json
content/content_scramble.json
content/content_morph.json
content/content_odometer.json
content/content_glyph_particles_base_spray.json
content/content_typewriter_io_filter_shader.json
bindable_rates/typewriter_speed_variance_bindable.json
```

## Required work

1. Add descriptor entries for the required content effects.

2. Define input contracts, including:

   ```text
   mode / lifecycle applicability
   speed / speedVariance / width / cascade
   cursor config for typewriter, if supported
   charset / seed / resolve pace for scramble, if implemented
   source/progression/direction for morph, if implemented
   prefix/suffix for wrapIndicator, if implemented
   runtimeMutability and bindable flags
   optional flags for non-required fields
   ```

3. Add player adapters that produce honest visual-frame/styled-cell evidence.

4. Add canonical fixtures under:

   ```text
   ../tui-vfx-recipes/recipes/v3.1/debug_recipes/content/
   ```

5. Keep existing source descriptors separate from content effects.

## Acceptance

Required:

```text
- content.typewriter fixture validates, renders, and appears in fixture-qc.
- content.marquee fixture validates, renders, and timeline/diff shows movement.
- content.splitFlap fixture validates and renders honest evidence, even if visually coarse.
- primitive-field-coverage remains zero-gap.
```

Preferred:

```text
- Add wrapIndicator, scramble, and morph fixtures if adapters can be honest.
- At least 3 content fixtures migrated; preferred 6.
```

Stop condition:

```text
Do not shove content effects into source.card or source.text to get green reports.
```

---

# Lane D — Source descriptors and adapters

## Objective

Turn K2.13 source decisions into actual source descriptor and player adapter evidence.

## Target source families

Required:

```text
source.ansi
source.procedural
source.image fallback path
```

Already exists:

```text
source.card
source.text
```

## Representative legacy recipes

```text
scene/ansi_source_chain.json
scene/scene_image_source_bindable.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
content/content_cell_motion_slice.json
subcell_shapes/fractional_inset_rect_v3.json
subcell_shapes/braille_rounded_rect_v3.json
```

## Required work

1. Confirm descriptors exist for:

   ```text
   source.ansi
   source.image
   source.procedural
   ```

2. Implement or harden source adapters:

   ```text
   source.ansi:
     parse a bounded ANSI/SGR subset into styled cells.
     Unknown ANSI sequences should emit diagnostics, not panic.

   source.image:
     support deterministic missing-asset fallback.
     Do not require a real asset resolver for basic fixture-QC.

   source.procedural:
     implement at least dots_spinner.
     Optionally support subcell_shape_atlas only as backend/procedural holdback unless bounded.
   ```

3. Add canonical fixtures:

   ```text
   sources/source_ansi_basic.json
   sources/source_image_missing_asset_fallback.json
   sources/source_procedural_dots_spinner.json
   ```

   or equivalent canonical paths following the repo’s naming conventions.

4. Preserve command-capture as oracle-only.

## Acceptance

Required:

```text
- source.ansi fixture validates/renders with styled-cell evidence.
- source.image fixture validates/renders deterministic fallback.
- source.procedural dots_spinner fixture validates/renders.
- source descriptors list all used fields.
- field coverage remains zero-gap.
```

Preferred:

```text
- Scene fixture using multiple source types passes fixture-qc.
- Source adapters expose diagnostics for unsupported variants.
```

Stop condition:

```text
Do not implement runtime command execution or network/file shell command capture.
```

---

# Lane E — Graph execution and I/O evidence

## Objective

K2.13 accepted graph I/O and sequence/parallel semantics. This lane must prove them through canonical fixture and player evidence.

Accepted semantics:

```text
Sequence:
  child N+1 sees mutations and graph values emitted by child N.

Parallel:
  branches read the same input snapshot;
  branch surfaces merge at the join;
  graph values merge at the join;
  conflicts use explicit policy or authored-order default with diagnostics.
```

## Representative legacy recipes

```text
complex/complex_filter_to_mask_sourced_output.json
complex/v3_io_parallel_merge_shader.json
complex/complex_nested_parallel_sequences.json
complex/complex_parallel_overlap_conflict_snapshot.json
content/content_typewriter_io_filter_shader.json
scene/scene_layer_visibility_binding_io.json
```

## Required work

1. Add or harden graph executor/player support for:

   ```text
   sequence graph value propagation
   parallel branch snapshot isolation
   graph value merge at parallel join
   graph value input consumption after parallel join
   authored-order conflict diagnostics
   io.outputs.source from payload field
   io.inputs.input into descriptor input
   ```

2. Add canonical v3.1 fixtures proving:

   ```text
   filter -> mask sourced output
   parallel producer -> later shader consumer
   typewriter content -> filter -> shader I/O
   overlap conflict diagnostic fixture, if not held for GUI review
   ```

3. Ensure diagnostics include enough identity:

   ```text
   node/step path
   branch index
   hint name
   input name
   conflict policy
   element/layer id if local pipeline
   ```

## Acceptance

Required:

```text
- At least 3 graph I/O canonical fixtures validate and render.
- render-frame-diff proves downstream effect changes when graph value changes.
- Parallel conflict diagnostics are visible and deterministic.
- schema-readiness remains canDeclareSchemaReady=true.
```

Preferred:

```text
- Add fixture-qc checks for graph I/O presence.
- Add a compact graph-value trace in render-frame or timeline reports.
```

Stop condition:

```text
Do not hide conflicts by silently choosing a branch without diagnostic evidence.
```

---

# Lane F — Scene / element / layer evidence

## Objective

Scene support is now core v3.1 schema. This lane proves practical scene behavior through fixtures and player/source evidence.

## Representative legacy recipes

```text
scene/scene_layer_nested_parallel_sequences.json
scene/scene_layer_visibility_binding_io.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
scene/scene_image_source_bindable.json
scene/ansi_source_chain.json
content/content_cell_motion_slice.json
subcell_shapes/fractional_inset_rect_v3.json
subcell_shapes/braille_rounded_rect_v3.json
```

## Required semantics

Prove:

```text
sort by z then authoring order
element/layer identity separate from role
placement in scene-global coordinates
local pipeline in layer-local coordinates
visibility predicate using binding/default
skipped cells preserve lower content
transparent cells blend/preserve unless explicit clear
diagnostics include layer/element id
```

## Required work

1. Add canonical fixtures for:

   ```text
   scene text layer
   scene ANSI layer
   scene image fallback layer
   scene procedural spinner layer
   scene layer visibility binding
   scene layer local pipeline
   scene nested parallel/sequence local pipeline
   ```

2. Implement/harden player evidence for multi-layer scenes.

3. Ensure scene fixtures pass:

   ```text
   validate-recipe
   render-recipe
   render-frame
   fixture-qc
   ```

## Acceptance

Required:

```text
- At least 4 scene/source fixtures migrated.
- One fixture proves visibility binding.
- One fixture proves z-order/overlap preservation.
- One fixture proves layer-local pipeline.
```

Preferred:

```text
- Cell-level report includes source element/layer identity when available.
- UI can display selected layer/source id in diagnostics, if trivial.
```

Stop condition:

```text
Do not overload role tags as element identifiers.
```

---

# Lane G — Filters, masks, and samplers descriptor tranche

## Objective

Burn down a large portion of descriptor backlog using bounded descriptors and honest player adapters.

## Required target descriptors

Filters:

```text
filter.pillButton
filter.fadeToCanvas
filter.patternFill
```

Masks:

```text
mask.pathReveal
mask.materialize
mask.noiseDither
```

Samplers:

```text
sampler.shredder
sampler.faultLine
sampler.radialTwist
```

Preferred, if bounded:

```text
filter.crt
filter.matrixRain
```

Hold if too broad:

```text
complex native-only filter packs
subcell-light filters that need backend/subcell renderer
```

## Representative recipes

```text
filters/filter_pill_button_progress_binding.json
filters/filter_fade_to_canvas_canvas_color_binding.json
filters/filter_pattern_fill.json
filters/filter_crt.json
filters/filter_matrix_rain.json
masks/mask_path_reveal.json
masks/mask_materialize_center.json
masks/mask_noise_dither.json
samplers/sampler_shredder.json
samplers/sampler_faultline.json
samplers/sampler_radial_twist_v3.json
```

## Required work

1. Add descriptor inputs with types/ranges/defaults/runtimeMutability.

2. Add player adapters with honest approximation where exact renderer parity is not available.

3. Add canonical fixtures under:

   ```text
   recipes/v3.1/debug_recipes/filters/
   recipes/v3.1/debug_recipes/masks/
   recipes/v3.1/debug_recipes/samplers/
   ```

4. Add field coverage declarations for every authored input.

## Acceptance

Required:

```text
- At least 6 fixtures migrated from this lane.
- All migrated fixtures pass fixture-qc.
- primitive-field-coverage remains zero-gap.
- primitive-adapter-gap remains zero unresolved for canonical corpus.
```

Preferred:

```text
- 9 fixtures migrated from this lane.
- render-frame-diff proves time-varying or binding-varying behavior for at least 2 fixtures.
```

Stop condition:

```text
Do not implement terminal_fire/water/shadow/subcell behavior in this lane.
```

---

# Lane H — Shader and style descriptor tranche

## Objective

Use K2.13 field closure and scope decisions to migrate a first substantial tranche of shader/style fixtures.

## Required target descriptors

Shaders:

```text
shader.revealWipe
shader.highlighter
shader.focusField
shader.glistenBand
shader.wayfindingNode
```

Styles/scopes:

```text
style.outerBand
style.moduloRows
style.moduloColumns
style.nonEmpty
style.inner
```

Already accepted field closures to exercise:

```text
shader.linearGradient.gradient
shader.linearGradient.applyTo
shader.borderSweep.position
```

Hold unless clearly bounded:

```text
shader.terminalFire
shader.terminalWater
large procedural weather/fire/water variants
```

## Representative recipes

```text
shaders/primitives/shader_reveal_wipe_corner_out_top_left.json
shaders/primitives/shader_linear_gradient_apply_to_both.json
shaders/compositions/shader_border_sweep_position_binding.json
shaders/compositions/shader_highlighter_runtime_bindings.json
shaders/compositions/shader_focus_field_center_binding.json
shaders/compositions/shader_glisten_band_direction_blend_binding.json
shaders/compositions/shader_wayfinding_node_current_index_binding.json
styles/style_outer_scope_band.json
styles/style_modulo_horizontal_every_third_row.json
styles/style_predicate_interior.json
styles/style_cell_position_binding.json
```

## Required work

1. Add/harden descriptors for shader/style inputs.

2. Add player adapters for bounded shader/style effects.

3. Add canonical fixtures for:

   ```text
   linearGradient gradient/applyTo variants
   borderSweep position binding
   revealWipe corner direction
   highlighter runtime bindings
   focusField center binding
   glistenBand direction/blend binding
   wayfinding current-index binding
   outerBand style scope
   moduloRows style scope
   inner style scope mapped from predicate interior
   ```

4. Preserve the decision that generic predicate registries are not accepted in this packet.

## Acceptance

Required:

```text
- At least 6 fixtures migrated from this lane.
- All shader/style authored fields are handled or explicitly held.
- style_predicate_interior maps to built-in inner, not a generic predicate registry.
```

Preferred:

```text
- 10 fixtures migrated from this lane.
- Runtime-bound shader fixtures show timeline/diff deltas.
```

Stop condition:

```text
Do not add a generic predicate/ref registry to get one fixture green.
```

---

# Lane I — Backend, GUI-human-review, oracle, and duplicate holdback signoff

## Objective

Remove non-schema holdbacks from the “blocker” conversation by making their disposition formal, reviewable, and stable.

## Holdback families

Backend:

```text
shadows/*
subcell_shapes/*
terminal_fire / terminal_water if no bounded player adapter is implemented
```

GUI human review:

```text
complex/complex_parallel_overlap_conflict_snapshot.json
complex/v3_scheduler_overlap_conflict_mixed_family.json
```

Oracle-only:

```text
command capture artifacts
loopback/demo-only records that are not canonical runtime semantics
deprecated legacy fixtures
```

Duplicate/variant:

```text
mask_diamond_square
mask_iris_square
mask_radial_square
```

## Required work

1. Create or update a holdback register doc:

   ```text
   docs/new_kernel/K2_14_HOLDBACK_REGISTER.md
   ```

2. Each holdback entry must include:

   ```text
   legacy path(s)
   disposition
   reason
   why it is not a schema blocker
   future phase/packet
   required evidence for release
   owner signoff status
   ```

3. Add machine-readable holdback data if existing reports support it, or document how current reports expose the disposition.

4. For backend holdbacks, write a concise future backend evidence policy:

   ```text
   shadow evidence
   subcell evidence
   compositor-backed adapter boundary
   GUI display requirements
   no direct UI compositor dependency
   ```

## Acceptance

Required:

```text
- All backendHoldback/guiHumanReviewHoldback/oracleOnly/duplicateVariant classes are represented in the holdback register.
- schema-readiness remains canDeclareSchemaReady=true.
- No holdback appears as explicitOwnerDecisionNeeded unless truly unresolved.
```

Preferred:

```text
- Add a compact holdback summary to schema-readiness or fixture-qc.
```

Stop condition:

```text
Do not implement compositor backend wiring in this packet unless every higher-priority lane is complete and the orchestrator explicitly scopes it.
```

---

# Lane J — Schema/API/docs/studio-control infrastructure

## Objective

Turn the docs/schema/API infrastructure from “reported as present” into a repeatable release-quality gate, and prepare the studio-control path without implementing the full studio.

## Required work

1. Verify existing schema generation workflow.

   The agent must locate and document the current commands for:

   ```text
   generating schemas/v3.1/contract/
   verifying generated schema freshness
   schema generation tests
   rustdoc/schemars propagation
   descriptor docs/export
   ```

2. If no ergonomic command exists, add a minimal one through the appropriate existing tool, likely:

   ```text
   tui-vfx-contract-cli
   xtask
   or documented cargo test/schema-generation workflow
   ```

3. Add or update docs:

   ```text
   docs/new_kernel/K2_14_SCHEMA_API_DOCS_GATE.md
   docs/new_kernel/K2_14_STUDIO_CONTROL_DERIVATION_REPORT.md
   ```

4. Add a minimal studio-control derivation report or command if feasible.

   It should list controls derivable from:

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
   bindable
   optional
   ```

   This is a report/control-catalog only. Do not build full UI controls yet.

5. Confirm template boundary docs remain clear:

   ```text
   templates are mandatory
   templates are compile-time composition
   runtime/player sees expanded canonical v3.1 only
   final canonical recipe has no unresolved template references
   ```

## Acceptance

Required:

```text
- Schema generation workflow is documented and reproducible.
- Generated schemas are checked or regenerated as needed.
- Rustdoc/schemars details are added for all newly touched public DTO fields.
- Studio-control derivation has a concrete report/doc path.
```

Preferred:

```text
- Add a CLI report that emits a JSON control catalog for descriptors.
- Add test coverage that every exported schema object has descriptions where expected.
```

Stop condition:

```text
Do not implement full dynamic studio UI before descriptor/value/source controls are stable.
```

---

# Optional canonical fixture migration targets

The implementer may add more fixtures than the minimum if gates remain green.

Prioritize these low-friction or medium-friction fixtures:

```text
content/content_typewriter.json
content/content_marquee.json
content/content_split_flap.json
content/content_wrap_indicator.json
content/content_scramble.json
content/content_morph.json

scene/ansi_source_chain.json
scene/scene_image_source_bindable.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
scene/scene_layer_visibility_binding_io.json
scene/scene_layer_nested_parallel_sequences.json

filters/filter_pattern_fill.json
filters/filter_pill_button_progress_binding.json
filters/filter_fade_to_canvas_canvas_color_binding.json
filters/filter_crt.json
filters/filter_matrix_rain.json

masks/mask_path_reveal.json
masks/mask_materialize_center.json
masks/mask_noise_dither.json

samplers/sampler_shredder.json
samplers/sampler_faultline.json
samplers/sampler_radial_twist_v3.json

shaders/primitives/shader_linear_gradient_apply_to_both.json
shaders/primitives/shader_reveal_wipe_corner_out_top_left.json
shaders/compositions/shader_border_sweep_position_binding.json
shaders/compositions/shader_highlighter_runtime_bindings.json
shaders/compositions/shader_focus_field_center_binding.json
shaders/compositions/shader_glisten_band_direction_blend_binding.json
shaders/compositions/shader_wayfinding_node_current_index_binding.json

styles/style_outer_scope_band.json
styles/style_modulo_horizontal_every_third_row.json
styles/style_predicate_interior.json
```

Hold or defer these unless clearly bounded:

```text
content/content_odometer.json
content/content_glyph_particles_base_spray.json
shaders/primitives/shader_terminal_fire_v3.json
shaders/primitives/shader_terminal_water_v3.json
shadows/*
subcell_shapes/*
command capture artifacts
complex overlap conflict fixtures needing human review
```

---

# Quantitative targets

The packet should be aggressive but not reckless.

## Required minimum

```text
- Add at least 20 new canonical v3.1 debug fixtures, unless stopped by documented contradictions.
- Add descriptor and adapter support for at least 12 new effect/source/content ids or variants.
- Migrate at least 3 content fixtures.
- Migrate at least 3 source/scene fixtures.
- Migrate at least 6 filter/mask/sampler fixtures.
- Migrate at least 6 shader/style fixtures.
- Add graph I/O proof fixtures or tests for at least 3 graph execution cases.
```

## Preferred

```text
- Add 30+ canonical v3.1 debug fixtures.
- Reduce descriptorBacklog materially in schema-readiness/migration reports.
- Add studio-control derivation JSON report.
- Add graph-value trace evidence in render-frame/timeline.
```

## Quality target

```text
No report may go green by suppressing authored fields.
No canonical fixture may be added with missing descriptor coverage.
No adapter may claim support while ignoring authored input fields.
```

---

# TDD requirements

Start with failing tests or failing fixture/report assertions for each lane.

Required RED/GREEN examples:

```text
- content.typewriter descriptor exists and fixture renders.
- content.marquee timeline changes.
- source.ansi parses styled cells.
- source.image missing-asset fallback renders deterministic output.
- source.procedural dots_spinner renders.
- binding loopback ramp changes render-frame over timeline.
- graph filter->mask output passes through io.outputs/io.inputs.
- parallel merge shader consumes post-parallel graph value.
- scene z-order and visibility binding are visible in frame evidence.
- linearGradient gradient/applyTo fields remain covered.
- borderSweep position binding remains covered.
- modulo/outer/inner scopes select expected cells.
- holdback register includes backend/gui/oracle/duplicate dispositions.
- schema docs generation command is reproducible.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

## Formatting and linting

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

## Tests

```bash
cargo test -p tui-vfx-contract
cargo test -p tui-vfx-contract-cli
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test -p tui-vfx-player-ui
cargo test --workspace
```

If the project uses nextest in this workflow:

```bash
cargo nextest run -p tui-vfx-contract --no-fail-fast
cargo nextest run -p tui-vfx-contract-cli --no-fail-fast
cargo nextest run -p tui-vfx-player --no-fail-fast
cargo nextest run -p tui-vfx-player-cli --no-fail-fast
cargo nextest run -p tui-vfx-player-ui --no-fail-fast
```

## Canonical corpus gates

```bash
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-frame \
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
```

## Migration/schema gates

```bash
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
  --include-offenders \
  --json
```

## Focused report samples

Run at least one timeline and one diff for each migrated class:

```bash
cargo run -q -p tui-vfx-player-cli -- render-timeline \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/content/<new-content-fixture>.json"

cargo run -q -p tui-vfx-player-cli -- render-frame-diff \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/<new-shader-fixture>.json"
```

## Cleanliness

```bash
git diff --check

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes
git -C "$RECIPE_REPO" status --short -- recipes/v3.1/debug_recipes

rg -n '"/usr/projects/tui-vfx-recipes|/usr/projects/tui-vfx-recipes' \
  crates/tui-vfx-player \
  crates/tui-vfx-player-cli \
  crates/tui-vfx-player-ui \
  crates/tui-vfx-contract \
  crates/tui-vfx-contract-cli \
  docs/new_kernel
```

Legacy root must remain unchanged.

---

# Expected docs from this packet

Create or update:

```text
docs/new_kernel/K2_14_DESCRIPTOR_ADAPTER_MIGRATION_REPORT.md
docs/new_kernel/K2_14_CANONICAL_FIXTURE_ADDITIONS.md
docs/new_kernel/K2_14_GRAPH_IO_EVIDENCE_REPORT.md
docs/new_kernel/K2_14_SCENE_SOURCE_EVIDENCE_REPORT.md
docs/new_kernel/K2_14_HOLDBACK_REGISTER.md
docs/new_kernel/K2_14_SCHEMA_API_DOCS_GATE.md
docs/new_kernel/K2_14_STUDIO_CONTROL_DERIVATION_REPORT.md
docs/new_kernel/PHASE_K2_14_DESCRIPTOR_ADAPTER_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md
```

The final status memo must include:

```text
- executive summary
- lane-by-lane table
- before/after metrics
- canonical fixtures added
- descriptors added or changed
- adapters added or changed
- tests added
- report/schema changes
- holdbacks signed off
- field coverage status
- adapter gap status
- fixture-qc status
- schema-readiness status
- schema/API/docs generation status
- studio-control derivation status
- recipe repo mutation status
- unresolved risks
- recommended next packet
```

---

# Recommended next packet after K2.14

The K2.14 status memo should recommend based on evidence, but likely next packets are:

```text
K2.15 — Descriptor / Adapter Migration Tranche 2
  More filters, masks, samplers, shader compositions, and content effects.

K2.15 — Backend Renderer Boundary Packet
  Shadows/subcell/compositor-backed evidence, only if descriptor/source/scene progress is strong.

K3.0 — Studio Control Manifest / Descriptor UI Pilot
  Auto-generated controls from descriptor/schema metadata once enough runtime/binding examples are stable.
```

Do not recommend another schema-decision packet unless a real contradiction is discovered.
