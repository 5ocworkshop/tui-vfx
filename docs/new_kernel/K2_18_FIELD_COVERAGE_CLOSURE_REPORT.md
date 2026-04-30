# K2.18 Field Coverage Closure Report

Field coverage blockers were reduced from 8 to 0. The active corpus now has 908/908 used fields handled and 0 used-but-unhandled fields.

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


## Exact field coverage blocker closure

| legacyPath | canonicalPath | baseline migration status | final migration status | final implementation disposition | assigned lane | field coverage issues | holdback reason | signed |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| masks/mask_center_wipe_fade_modal.json | masks/mask_center_wipe_fade_modal.json | blockedByFieldCoverage | descriptorDecisionNeeded | descriptorBacklogResolved | filterMaskSamplerDescriptor | direction, easing, softEdge | descriptor backlog resolved to exact descriptor path disposition | yes |
| masks/mask_wipe_fade_left_right.json | masks/mask_wipe_fade_left_right.json | blockedByFieldCoverage | descriptorDecisionNeeded | descriptorBacklogResolved | filterMaskSamplerDescriptor | direction, easing, softEdge | descriptor backlog resolved to exact descriptor path disposition | yes |
| shaders/compositions/shader_barber_pole.json | shaders/compositions/shader_barber_pole.json | blockedByFieldCoverage | schemaDecisionNeeded | graphRuntimeResolved | graphRuntime | color | generic graph backlog resolved into exact runtime feature disposition | yes |
| shaders/primitives/shader_region_compression_scope.json | shaders/primitives/shader_region_compression_scope.json | blockedByFieldCoverage | schemaDecisionNeeded | graphRuntimeResolved | graphRuntime | color | generic graph backlog resolved into exact runtime feature disposition | yes |
| styles/style_fade_in.json | styles/style_fade_in.json | blockedByFieldCoverage | canonicalExists | canonicalExists | blockerLedger | — | — | no |
| styles/style_fade_out.json | styles/style_fade_out.json | blockedByFieldCoverage | canonicalExists | canonicalExists | blockerLedger | — | — | no |
| styles/style_fade_out_to_canvas.json | styles/style_fade_out_to_canvas.json | blockedByFieldCoverage | schemaDecisionNeeded | graphRuntimeResolved | graphRuntime | to | generic graph backlog resolved into exact runtime feature disposition | yes |
| styles/style_pulse.json | styles/style_pulse.json | blockedByFieldCoverage | schemaDecisionNeeded | graphRuntimeResolved | graphRuntime | pulseColor | generic graph backlog resolved into exact runtime feature disposition | yes |


## Acceptance evidence

- `blockedByFieldCoverage`: 8 -> 0.
- `primitive-field-coverage`: 0 unhandled; 0 missing descriptor fields.
- `fixture-qc`: pass.
- Added canonical style aliases: `styles/style_fade_in.json`, `styles/style_fade_out.json`.
