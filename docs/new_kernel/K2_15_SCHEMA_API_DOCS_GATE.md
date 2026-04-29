# K2.15 schema/API/docs gate

## Impact assessment

K2.15 changed player/runtime evidence code, player reports, player CLI fixture-count tests, canonical fixtures, and the pre-release v3.1 `RecipeMetadata.expectedVisual` contract field. Because v3.1 is not locked, this intentionally does not bump a schema/report version.

## Documentation sync

Updated or added impacted docs:

- `K2_15_BASELINE_AND_FINAL_COUNTERS.md`
- `K2_15_GRAPH_EXECUTION_INTEGRATION_REPORT.md`
- `K2_15_GRAPH_IO_FIXTURE_REPORT.md`
- `K2_15_SCENE_LAYER_PLAYER_EVIDENCE_REPORT.md`
- `K2_15_DESCRIPTOR_ADAPTER_TRANCHE_2_REPORT.md`
- `K2_15_SOURCE_CONTENT_TRANCHE_2_REPORT.md`
- `K2_15_HOLDBACK_REGISTER.md`
- `K2_15_STUDIO_CONTROL_CATALOG_PREFLIGHT.md`
- `PHASE_K2_15_GRAPH_DESCRIPTOR_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md`
- `PHASE_K2_15_REVIEW_AND_DESLOP_REPORT.md`
- `docs/new_kernel/INDEX.md`
- `docs/VOCABULARY.md`

`docs/VOCABULARY.md` was updated because `metadata.expectedVisual` is author-facing fixture metadata and player warnings are durable player-report vocabulary. K2.15 also added diagnostic/report codes (`parallelGraphValueConflict`, `parallelSurfaceConflict`, `unknownTopologyNode`, `missingNodeOutputInput`, `unsupportedEffectOutput`) documented in the graph execution and review reports.

## Schema and docs gate evidence

K2.15 added `RecipeMetadata.expectedVisual`, so contract schema freshness was regenerated and checked.

```text
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation checked_in_contract_schemas_are_current -- --exact: pass
cargo test -p tui-vfx-contract --test test_schema_generation checked_in_contract_schemas_are_current -- --exact: pass
cargo xtask docs check: pass with pre-existing warnings for filters.GlyphStyle, filters.ScalarFieldGlyph, and shaders.Highlighter ai_hint params
cargo xtask docs api: pass
cargo xtask docs api-check: pass
cargo xtask docs api-validate: pass
cargo xtask audit configschema: pass
```

Player report serialized schema labels were intentionally not bumped because K2.15 is pre-release v3.1 work and warnings already existed in the report DTO.

