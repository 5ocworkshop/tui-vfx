# K2.18 Content Backlog Closure Report

Content backlog was closed without reintroducing pseudo-source vocabulary. Thirty-five records are signed duplicate variants covered by representative canonical content evidence; four backend-heavy content records are signed backend holdbacks.

Baseline `contentBacklog` paths: 39. Final generic `contentBacklog` paths: 0. Final disposition spread: {'backendHoldbackSignedOff': 4, 'duplicateVariantSignedOff': 35}.

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
| content/content_glyph_cascade_braille.json | content/content_glyph_cascade_braille.json | content | contentBacklog | backendHoldbackSignedOff | holdback | content.glyphCascade | content.glyphCascade | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for content.glyphCascade | yes | deferToBackendSeam |
| content/content_glyph_cascade_into.json | content/content_glyph_cascade_into.json | content | contentBacklog | backendHoldbackSignedOff | holdback | content.glyphCascade | content.glyphCascade | futurePlayerRenderBackendEvidence | backend/compositor fidelity or descriptor semantics required for content.glyphCascade | yes | deferToBackendSeam |
| content/content_glyph_particles_base_spray.json | content/content_glyph_particles_base_spray.json | content | contentBacklog | backendHoldbackSignedOff | holdback | filter.invert | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity required before visual parity claims | yes | deferToBackendSeam |
| content/content_glyph_particles_options_concurrency.json | content/content_glyph_particles_options_concurrency.json | content | contentBacklog | backendHoldbackSignedOff | holdback | — | — | futurePlayerRenderBackendEvidence | backend/compositor fidelity required before visual parity claims | yes | deferToBackendSeam |
| content/content_odometer_cell_roll_dispersion_edge_in.json | content/content_odometer_cell_roll_dispersion_edge_in.json | content | contentBacklog | duplicateVariantSignedOff | content | content.odometer, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_odometer_cell_roll_left.json | content/content_odometer_cell_roll_left.json | content | contentBacklog | duplicateVariantSignedOff | content | content.odometer, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_odometer_cell_roll_slot_machine.json | content/content_odometer_cell_roll_slot_machine.json | content | contentBacklog | duplicateVariantSignedOff | content | content.odometer, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_odometer_cell_roll_up.json | content/content_odometer_cell_roll_up.json | content | contentBacklog | duplicateVariantSignedOff | content | content.odometer, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_odometer_decimal_preset_carry.json | content/content_odometer_decimal_preset_carry.json | content | contentBacklog | duplicateVariantSignedOff | content | content.odometer | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_odometer_slot_reel.json | content/content_odometer_slot_reel.json | content | contentBacklog | duplicateVariantSignedOff | content | content.odometer | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_board_update.json | content/content_split_flap_board_update.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_cycles.json | content/content_split_flap_cycles.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_digits.json | content/content_split_flap_digits.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_from_message.json | content/content_split_flap_from_message.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_jitter.json | content/content_split_flap_jitter.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_leading_blocks.json | content/content_split_flap_leading_blocks.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_rolling_cards.json | content/content_split_flap_rolling_cards.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_settle_hinge.json | content/content_split_flap_settle_hinge.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_solari_authentic.json | content/content_split_flap_solari_authentic.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_solari_museum.json | content/content_split_flap_solari_museum.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_spring_settle.json | content/content_split_flap_spring_settle.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_tile_2row.json | content/content_split_flap_tile_2row.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_tile_4row.json | content/content_split_flap_tile_4row.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_tile_6row.json | content/content_split_flap_tile_6row.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_tile_8row.json | content/content_split_flap_tile_8row.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_split_flap_tile_board.json | content/content_split_flap_tile_board.json | content | contentBacklog | duplicateVariantSignedOff | content | content.splitFlap, shader.borderSweep | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_braille_6.json | content/content_typewriter_cursor_braille_6.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_braille_8.json | content/content_typewriter_cursor_braille_8.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_braille_flip.json | content/content_typewriter_cursor_braille_flip.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_braille_pulse.json | content/content_typewriter_cursor_braille_pulse.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_grow_in_center.json | content/content_typewriter_cursor_grow_in_center.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_grow_in_down.json | content/content_typewriter_cursor_grow_in_down.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_grow_in_up.json | content/content_typewriter_cursor_grow_in_up.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_scan_bounce.json | content/content_typewriter_cursor_scan_bounce.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_scan_pulse.json | content/content_typewriter_cursor_scan_pulse.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_wake_gap.json | content/content_typewriter_cursor_wake_gap.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_wake_ghost.json | content/content_typewriter_cursor_wake_ghost.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_cursor_wake_tint.json | content/content_typewriter_cursor_wake_tint.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |
| content/content_typewriter_io_filter_shader.json | content/content_typewriter_io_filter_shader.json | content | contentBacklog | duplicateVariantSignedOff | content | content.typewriter, filter.dim, shader.diffusion | — | — | covered by representative canonical v3.1 fixture for the same descriptor family | yes | doNotMigrate |


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
