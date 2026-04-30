# K2.18 Schema and API Docs Gate

K2.18 doc closure did not modify source code, tests, public DTOs, schema files, rustdoc, or API generation outputs. This gate records the current working-tree evidence for implementation changes owned by the implementation lanes.

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


## Commands run in this pass

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

Additional docs/schema commands:

- `cargo test -p tui-vfx-contract --test test_schema_generation`: 21 passed, 0 failed.
- `cargo xtask docs check`: pass with existing descriptor-doc warnings for `filters.ScalarFieldGlyph`, `filters.GlyphStyle`, and `shaders.Highlighter` hint params.
- `cargo xtask docs api-check`: API.md up to date.
- `cargo xtask audit configschema`: pass.

## Impact assessment

- Public durable vocabulary: no new durable public terms added by this doc closure.
- `docs/VOCABULARY.md`: not changed.
- `docs/v3.1-feature-contract-checklist.md`: not changed; existing implementation-readiness/catalog/backend-seam gates already cover this closure.
- `docs/new_kernel/INDEX.md`: updated to index K2.18 closure artifacts.
- Schema/API generated files: not changed by this doc closure.
