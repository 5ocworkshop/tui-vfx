# K2.18 Scene Runtime Closure Report

All baseline scene runtime backlog records are resolved to `sceneRuntimeResolved`; no scene path remains in a generic runtime backlog.

Baseline `sceneRuntimeBacklog` paths: 16. Final generic `sceneRuntimeBacklog` paths: 0. Final disposition spread: {'sceneRuntimeResolved': 16}.

Evidence date: 2026-04-30

Refreshed report inputs:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
cargo run -q -p tui-vfx-player-cli -- implementation-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive --include-blockers --json > /tmp/k218-doc-impl.json
cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive --json > /tmp/k218-doc-migration.json
```


## Path-level closure table

| legacyPath | canonicalPath | family | baseline queue | final disposition | assigned lane | required descriptors | missing descriptors | required runtime features | holdback reason | signed | recommended action |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| scene/ansi_source_chain.json | scene/ansi_source_chain.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | filter.dim, shader.diffusion | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_authoring_ladder_flag_asset_binding.json | scene/scene_authoring_ladder_flag_asset_binding.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | — | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_authoring_ladder_procedural_spinner_binding.json | scene/scene_authoring_ladder_procedural_spinner_binding.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | — | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_authoring_ladder_toast_basic.json | scene/scene_authoring_ladder_toast_basic.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | — | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_braille_flag_asset_token.json | scene/scene_braille_flag_asset_token.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | — | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_braille_flag_runtime_wave.json | scene/scene_braille_flag_runtime_wave.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | — | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_image_source_bindable.json | scene/scene_image_source_bindable.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | — | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_full_stack.json | scene/scene_layer_full_stack.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | filter.dim, filter.invert | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_io_filter_shader.json | scene/scene_layer_io_filter_shader.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | filter.dim, sampler.spatialSignal, shader.diffusion | sampler.spatialSignal | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_role_scope_pipeline.json | scene/scene_layer_role_scope_pipeline.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | — | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_sequence_sampler_then_style_effect.json | scene/scene_layer_sequence_sampler_then_style_effect.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | sampler.sineWave | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_sequence_style_effect_then_sampler.json | scene/scene_layer_sequence_style_effect_then_sampler.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | sampler.sineWave | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_shadow_nested_parallel_sequences.json | scene/scene_layer_shadow_nested_parallel_sequences.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | filter.greyscale, sampler.sineWave, shader.linearGradient | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_surface_channel_pipeline.json | scene/scene_layer_surface_channel_pipeline.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | filter.dim, filter.invert | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_surface_shadow.json | scene/scene_layer_surface_shadow.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | — | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |
| scene/scene_layer_surface_shadow_pipeline.json | scene/scene_layer_surface_shadow_pipeline.json | scene | sceneRuntimeBacklog | sceneRuntimeResolved | sceneRuntime | filter.invert | — | sceneLocalRuntimeDisposition | generic scene backlog resolved into exact scene-runtime disposition | yes | none |


## Gate evidence

Core gate results from this doc-closure pass:

- validate-recipe: 144/144 valid, 0 invalid.
- render-recipe: 144/144 rendered, 0 unsupported, 0 errors.
- render-frame: 144/144 rendered, 0 unsupported, 0 errors.
- fixture-qc: pass; 144 validated, 144 rendered, 0 unhandled fields, 0 unresolved adapter gaps, timeline smoke True, diff smoke True.
- primitive-field-coverage: 908/908 used fields handled; 0 used-but-unhandled; 0 missing descriptor fields.
- primitive-adapter-gap: 75/75 effects rendered; 0 unsupported; 0 missing descriptors.
- schema-readiness: canDeclareSchemaReady=true; explicitOwnerDecisionNeeded 0; fieldCoverageBlockedRecords 0; adapterBlockedRecords 0.
- implementation-readiness: implementationBlocking 0; explicitOwnerDecisionNeeded 0; generic implementation queues {}.
- control-catalog: 372 controls (16 source, 356 effect).
