<!-- <FILE>docs/new_kernel/K2_9_SIMPLE_MASK_MIGRATION_MAPPING_EVIDENCE.md</FILE> - <DESC>K2.9 migration mapping and simple mask evidence</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.9 migration mapping: record report outputs, accepted fixtures, and verification commands.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture migration-mapping-batch and simple mask expansion evidence.</CLOG> -->

# K2.9 Simple Mask Migration Mapping Evidence

## Scope

K2.9 added a read-only player report command and used it to migrate the bounded simple mask set that had clear descriptor semantics:

- `mask.blinds`
- `mask.radial`
- `mask.iris`
- `mask.diamond`

Legacy recipes remain read-only evidence under `../tui-vfx-recipes/recipes/debug_recipes/`.
Canonical v3.1 fixtures live under `../tui-vfx-recipes/recipes/v3.1/debug_recipes/`.

## Report surface

Command:

```bash
RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --family masks \
  --json
```

Stable schema:

```text
v3.1.player.migrationMappingBatch.1
```

Owning symbols:

| Area | Path | Symbol |
|---|---|---|
| report DTOs | `crates/tui-vfx-player/src/cls_player_migration_mapping_batch_report.rs` | `PlayerMigrationMappingBatchReport`, `PlayerMigrationMappingRecord`, `PlayerMigrationMappingBatchSummary`, `PlayerMigrationMappingQueueItem` |
| report builder | `crates/tui-vfx-player/src/fnc_build_migration_mapping_batch_report.rs` | `build_migration_mapping_batch_report` |
| record builder | `crates/tui-vfx-player/src/fnc_build_migration_mapping_record.rs` | `build_migration_mapping_record` |
| legacy mask evidence | `crates/tui-vfx-player/src/fnc_collect_legacy_mask_payloads.rs` | `collect_legacy_mask_payloads`, `required_legacy_mask_descriptors`, `required_legacy_mask_inputs` |
| path collection | `crates/tui-vfx-player/src/fnc_collect_migration_mapping_batch_paths.rs` | `collect_migration_mapping_batch_paths` |
| summary and queue | `crates/tui-vfx-player/src/fnc_summarize_migration_mapping_batch.rs` | `summarize_migration_mapping_records`, `build_migration_mapping_recommendation_queue` |
| CLI runner | `crates/tui-vfx-player-cli/src/fnc_run_migration_mapping_batch.rs` | `run_migration_mapping_batch` |
| CLI dispatch | `crates/tui-vfx-player-cli/src/fnc_run.rs` | `is_known_command`, `dispatch_command` |
| CLI parser | `crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs` | `parse_cli_options` handles `--family` |

## Mask family report evidence

Observed summary after K2.9 fixtures exist:

```json
{
  "schemaVersion": "v3.1.player.migrationMappingBatch.1",
  "summary": {
    "families": 1,
    "records": 41,
    "canonicalExists": 8,
    "candidateReady": 0,
    "descriptorDecisionNeeded": 15,
    "schemaDecisionNeeded": 0,
    "ownerAuditNeeded": 15,
    "duplicateOrVariant": 3
  }
}
```

Square-clarity companions are intentionally classified as variants, not new canonical fixtures:

| Legacy path | Status | Recommendation |
|---|---|---|
| `masks/mask_radial_square.json` | `duplicateOrVariant` | `skipAsDuplicateVariant` |
| `masks/mask_iris_square.json` | `duplicateOrVariant` | `skipAsDuplicateVariant` |
| `masks/mask_diamond_square.json` | `duplicateOrVariant` | `skipAsDuplicateVariant` |

## Added descriptor coverage

Descriptor pack path:

```text
descriptors/v3.1/packs/primitive.json
```

Accepted new descriptor ids:

| Descriptor id | Accepted inputs | Evidence caveat |
|---|---|---|
| `mask.blinds` | `orientation`, `count` | Text-grid band reveal only. |
| `mask.radial` | `origin`, `softEdge` | `origin` is center-only in K2.9. |
| `mask.iris` | `shape`, `softEdge` | `shape` accepts `circle` and `diamond`. |
| `mask.diamond` | `softEdge` | Centered diamond aperture only. |

## Added player adapter coverage

Adapter paths and symbols:

| Effect id | Path | Symbol |
|---|---|---|
| `mask.blinds` | `crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs` | `apply_mask_blinds` |
| `mask.radial` | `crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs` | `apply_mask_radial` |
| `mask.iris` | `crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs` | `apply_mask_iris` |
| `mask.diamond` | `crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs` | `apply_mask_diamond` |

Routing and coverage:

| Path | Symbol/area |
|---|---|
| `crates/tui-vfx-player/src/fnc_apply_graph_effects.rs` | Routes new mask ids to adapter functions. |
| `crates/tui-vfx-player/src/fnc_collect_handled_primitive_inputs.rs` | Declares handled input fields for new masks. |
| `crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs` | Marks new masks as `visible`. |

## Added canonical fixtures

All new fixtures are descriptor-pack-backed and use `source.card`:

| Fixture | Primary effect id |
|---|---|
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_blinds.json` | `mask.blinds` |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_radial.json` | `mask.radial` |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_iris.json` | `mask.iris` |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_diamond.json` | `mask.diamond` |

## Targeted test evidence

RED step captured before implementation:

```text
cargo test -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_reports_migration_mapping_batch_masks_json -- --exact
```

Initial result: command was unknown and usage printed.

GREEN evidence after implementation and de-slop:

```text
cargo test -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_reports_migration_mapping_batch -- --nocapture
```

Result:

```text
2 passed; 0 failed
```

Additional targeted evidence captured before final full verification:

| Command | Result |
|---|---|
| `cargo test -p tui-vfx-contract-cli --test test_fnc_validate_recipe_descriptor_packs` | `5 passed; 0 failed` |
| `validate-recipe --recursive` over v3.1 debug recipes | `total=26`, `valid=26`, `invalid=0` |
| `primitive-field-coverage --recursive` | `usedInputFields=207`, `handledInputFields=207`, gap counts `0` |
| `primitive-adapter-gap --recursive` | `totalEffects=18`, `rendered=18`, gap counts `0` |
| `fixture-qc --recursive` | `totalRecipes=26`, `validated=26`, `rendered=26`, `unsupported=0`, `overallStatus=pass` |

## Deferred observations

- `mask.checkers` has a reported interactive playback issue where only a few characters are visible in one recipe. The existing adapter still passes CLI gates, so this is deferred as a separate visual/player bug investigation.
- `softEdge` for K2.9 masks is coarse text-grid threshold evidence, not alpha feather visual parity.
- Graph application currently iterates all ordered nodes for the sampled frame; K2.9 did not add authored `invert`, `radius`, numeric `feather`, or phase-specific mask fields.

<!-- <FILE>docs/new_kernel/K2_9_SIMPLE_MASK_MIGRATION_MAPPING_EVIDENCE.md</FILE> - <DESC>K2.9 migration mapping and simple mask evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
