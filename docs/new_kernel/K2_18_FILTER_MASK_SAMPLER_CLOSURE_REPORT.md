# K2.18 Filter, Mask, and Sampler Closure Report

Filter/mask/sampler descriptor records were either resolved through descriptor/player evidence or signed as backend holdbacks when styled-cell player evidence would be false visual parity.

Lane record count: 28. Final disposition spread: {'descriptorBacklogResolved': 21, 'backendHoldbackSignedOff': 7}.

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


## Path-level descriptor closure table

| legacyPath | canonicalPath | baseline queue | final disposition | required descriptors | missing descriptors | required player adapters | required runtime features | holdback reason | signed | confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| filters/filter_animated_glyph_ramp.json | filters/filter_animated_glyph_ramp.json | descriptorBacklog | descriptorBacklogResolved | filter.animatedGlyphRamp | filter.animatedGlyphRamp | playerAdapter:filter.animatedGlyphRamp | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_animated_glyph_ramp_gradient.json | filters/filter_animated_glyph_ramp_gradient.json | descriptorBacklog | descriptorBacklogResolved | filter.animatedGlyphRamp | filter.animatedGlyphRamp | playerAdapter:filter.animatedGlyphRamp | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_braille_dust.json | filters/filter_braille_dust.json | descriptorBacklog | descriptorBacklogResolved | filter.brailleDust | filter.brailleDust | playerAdapter:filter.brailleDust | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_charset_noise.json | filters/filter_charset_noise.json | descriptorBacklog | descriptorBacklogResolved | filter.charsetNoise | filter.charsetNoise | playerAdapter:filter.charsetNoise | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_color_bridged_shade.json | filters/filter_color_bridged_shade.json | descriptorBacklog | descriptorBacklogResolved | filter.colorBridgedShade | filter.colorBridgedShade | playerAdapter:filter.colorBridgedShade | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_glisten_sweep.json | filters/filter_glisten_sweep.json | descriptorBacklog | descriptorBacklogResolved | filter.glistenSweep | filter.glistenSweep | playerAdapter:filter.glistenSweep | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_glisten_sweep_progress_binding.json | filters/filter_glisten_sweep_progress_binding.json | descriptorBacklog | descriptorBacklogResolved | filter.glistenSweep | filter.glistenSweep | playerAdapter:filter.glistenSweep | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_glyph_style.json | filters/filter_glyph_style.json | descriptorBacklog | descriptorBacklogResolved | filter.glyphStyle | filter.glyphStyle | playerAdapter:filter.glyphStyle | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_interlace_curtain.json | filters/filter_interlace_curtain.json | descriptorBacklog | descriptorBacklogResolved | filter.interlaceCurtain | filter.interlaceCurtain | playerAdapter:filter.interlaceCurtain | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_motion_blur.json | filters/filter_motion_blur.json | descriptorBacklog | descriptorBacklogResolved | filter.motionBlur | filter.motionBlur | playerAdapter:filter.motionBlur | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_rigid_shake.json | filters/filter_rigid_shake.json | descriptorBacklog | backendHoldbackSignedOff | filter.rigidShake | filter.rigidShake | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for filter.rigidShake | yes | high |
| filters/filter_rigid_shake_damping_scale_binding.json | filters/filter_rigid_shake_damping_scale_binding.json | descriptorBacklog | backendHoldbackSignedOff | filter.rigidShake | filter.rigidShake | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for filter.rigidShake | yes | high |
| filters/filter_rigid_shake_num_shakes_binding.json | filters/filter_rigid_shake_num_shakes_binding.json | descriptorBacklog | backendHoldbackSignedOff | filter.rigidShake | filter.rigidShake | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for filter.rigidShake | yes | high |
| filters/filter_shade_scanner.json | filters/filter_shade_scanner.json | descriptorBacklog | descriptorBacklogResolved | filter.shadeScanner | filter.shadeScanner | playerAdapter:filter.shadeScanner | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_shade_scanner_progress_binding.json | filters/filter_shade_scanner_progress_binding.json | descriptorBacklog | descriptorBacklogResolved | filter.shadeScanner | filter.shadeScanner | playerAdapter:filter.shadeScanner | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| filters/filter_sub_cell_shake.json | filters/filter_sub_cell_shake.json | descriptorBacklog | backendHoldbackSignedOff | filter.subCellShake | filter.subCellShake | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for filter.subCellShake | yes | high |
| filters/filter_subcell_light_background_braille.json | filters/filter_subcell_light_background_braille.json | descriptorBacklog | backendHoldbackSignedOff | filter.subcellLight, shader.focusField | filter.subcellLight | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for filter.subcellLight | yes | high |
| filters/filter_subcell_light_foreground_horizontal.json | filters/filter_subcell_light_foreground_horizontal.json | descriptorBacklog | backendHoldbackSignedOff | filter.subcellLight, shader.concealedLight | filter.subcellLight, shader.concealedLight | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for filter.subcellLight, shader.concealedLight | yes | high |
| filters/filter_subcell_light_temporal_braille.json | filters/filter_subcell_light_temporal_braille.json | descriptorBacklog | backendHoldbackSignedOff | filter.subcellLight, shader.diffusion | filter.subcellLight | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for filter.subcellLight | yes | high |
| masks/mask_center_wipe_fade_modal.json | masks/mask_center_wipe_fade_modal.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe, style.fadeIn, style.fadeOut | — | playerAdapter:mask.wipe, playerAdapter:style.fadeIn, playerAdapter:style.fadeOut | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| masks/mask_wipe_corner_in_to_bottom_left.json | masks/mask_wipe_corner_in_to_bottom_left.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe | — | playerAdapter:mask.wipe | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| masks/mask_wipe_corner_in_to_bottom_right.json | masks/mask_wipe_corner_in_to_bottom_right.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe | — | playerAdapter:mask.wipe | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| masks/mask_wipe_corner_in_to_top_left.json | masks/mask_wipe_corner_in_to_top_left.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe | — | playerAdapter:mask.wipe | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| masks/mask_wipe_corner_in_to_top_right.json | masks/mask_wipe_corner_in_to_top_right.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe | — | playerAdapter:mask.wipe | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| masks/mask_wipe_corner_out_from_bottom_left.json | masks/mask_wipe_corner_out_from_bottom_left.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe | — | playerAdapter:mask.wipe | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| masks/mask_wipe_corner_out_from_bottom_right.json | masks/mask_wipe_corner_out_from_bottom_right.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe | — | playerAdapter:mask.wipe | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| masks/mask_wipe_corner_out_from_top_right.json | masks/mask_wipe_corner_out_from_top_right.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe | — | playerAdapter:mask.wipe | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |
| masks/mask_wipe_fade_left_right.json | masks/mask_wipe_fade_left_right.json | descriptorBacklog | descriptorBacklogResolved | mask.wipe, style.fadeIn, style.fadeOut | — | playerAdapter:mask.wipe, playerAdapter:style.fadeIn, playerAdapter:style.fadeOut | — | descriptor backlog resolved to exact descriptor path disposition | yes | high |


## Acceptance evidence

- Field coverage remains 0 unhandled.
- Adapter gap remains 0 unresolved / 0 missing descriptor.
- Implementation generic descriptorBacklog final count: 0.
