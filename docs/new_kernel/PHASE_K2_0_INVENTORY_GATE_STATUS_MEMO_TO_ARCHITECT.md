<!-- <FILE>docs/new_kernel/PHASE_K2_0_INVENTORY_GATE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.0 inventory gate status memo to the v3.1 architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase K2.0: summarize the report-only K0 fixture inventory gate.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document K2.0 inventory command, schema, evidence, and next risks.</CLOG> -->

# Phase K2.0 Inventory Gate Status Memo to the v3.1 Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Packet: `docs/new_kernel/DAY2_PLAN_K2.0_PACKET.md` — first work packet only

## Executive summary

K2.0 is complete for the bounded packet: K0 now has a report-only `inventory-recipes` CLI command that inventories canonical v3.1 debug fixtures, descriptor coverage, source/effect ids, current K0 render status, and unsupported adapter diagnostics without changing render semantics.

The existing `render-recipe` path remains intact and continues to report the expected recursive smoke result:

```text
schemaVersion=v3.1.player.run.1
summary={total:16, rendered:10, unsupported:6, errors:0}
```

The new inventory gate reports:

```text
schemaVersion=v3.1.player.inventory.1
summary={totalRecipes:16, rendered:10, unsupported:6, errors:0, descriptorEffectIds:14, representedEffectIds:14, unrepresentedEffectIds:0, unsupportedEffectIds:6, sourceIds:1}
```

No recipes, K1 files, compositor wiring, or rendering semantics were modified.

## Implemented command

```bash
cargo run -p tui-vfx-player-cli -- inventory-recipes \
  --recursive \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

## Report schema

Top-level schema label:

```text
v3.1.player.inventory.1
```

Top-level fields:

```text
schemaVersion
root
descriptorPacks
summary
recipes
effects
sources
```

Per recipe fields:

```text
path
recipeId
status
sourceIds
effectIds
descriptorCoveredEffectIds
missingDescriptorEffectIds
descriptorCoveredSourceIds
missingDescriptorSourceIds
unsupportedEffectIds
errors
```

Per effect fields:

```text
id
descriptorCovered
representedByRecipes
adapterStatus
recipePaths
```

Per source fields:

```text
id
descriptorCovered
representedByRecipes
adapterStatus
recipePaths
```

## Files changed

Player library:

```text
crates/tui-vfx-player/src/lib.rs
crates/tui-vfx-player/src/cls_player_inventory_recipe.rs
crates/tui-vfx-player/src/cls_player_inventory_report.rs
crates/tui-vfx-player/src/fnc_inventory_recipe_file.rs
crates/tui-vfx-player/src/fnc_inventory_recipe_paths.rs
```

Player CLI/tests:

```text
crates/tui-vfx-player-cli/src/fnc_run.rs
crates/tui-vfx-player-cli/src/fnc_print_usage.rs
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
```

## Key symbols

```text
PlayerInventoryRecipe
PlayerInventoryReport
PlayerInventorySummary
PlayerInventoryEffect
PlayerInventorySource
effect_adapter_status
source_adapter_status
inventory_recipe_file
inventory_recipe_paths
run_inventory_recipes
```

## Current adapter classification from the inventory gate

| Effect id | Adapter status | Represented |
|---|---:|---:|
| `filter.dim` | `noop` | yes |
| `filter.greyscale` | `noop` | yes |
| `filter.invert` | `noop` | yes |
| `filter.tint` | `noop` | yes |
| `mask.checkers` | `visible` | yes |
| `mask.dissolve` | `unsupported` | yes |
| `mask.none` | `noop` | yes |
| `mask.wipe` | `visible` | yes |
| `sampler.ripple` | `unsupported` | yes |
| `sampler.sineWave` | `noop` | yes |
| `shader.borderSweep` | `unsupported` | yes |
| `shader.linearGradient` | `unsupported` | yes |
| `style.baseStyleOverride` | `unsupported` | yes |
| `style.colorFade` | `unsupported` | yes |

Source classification:

| Source id | Adapter status | Represented |
|---|---:|---:|
| `source.card` | `visible` | yes |

## Verification evidence

Required packet checks:

```text
cargo fmt --package tui-vfx-player -- --check                         PASS
cargo fmt --package tui-vfx-player-cli -- --check                     PASS
cargo test -p tui-vfx-player                                         PASS, 4 tests
cargo test -p tui-vfx-player-cli                                     PASS, 6 tests
cargo run -p tui-vfx-player-cli -- render-recipe --recursive ...      PASS, total=16 rendered=10 unsupported=6 errors=0
cargo run -p tui-vfx-player-cli -- inventory-recipes --recursive ...  PASS, totalRecipes=16 rendered=10 unsupported=6 errors=0 descriptorEffectIds=14 representedEffectIds=14 unrepresentedEffectIds=0 unsupportedEffectIds=6
```

Additional static check:

```text
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings  PASS
```

Temporary captured JSON outputs used for review:

```text
/tmp/tui-vfx-k2-render-report.json
/tmp/tui-vfx-k2-inventory-report.json
```

## Acceptance criteria status

| Criterion | Status |
|---|---|
| New report-only K0 inventory CLI exists | Done |
| Existing `render-recipe` behavior remains intact | Done |
| Recursive inventory prints machine-readable JSON | Done |
| Per-recipe source/effect ids are listed | Done |
| Descriptor-covered and missing descriptor ids are listed | Done |
| Unsupported adapter ids are listed | Done |
| Aggregate descriptor coverage is explicit | Done |
| Existing recursive K0 smoke still reports 16/10/6/0 | Done |
| No recipe modifications | Respected |
| No K1 modifications | Respected |
| No compositor wiring | Respected |
| No rendering semantic changes | Respected |

## Remaining risks / next recommendations

- `adapterStatus` is intentionally a K0 inventory classification table, not a dynamic runtime registry. The next adapter work should either keep this table synchronized with real adapters or replace it with a shared adapter registry when compositor-backed adapters land.
- Inventory still samples through K0 text-grid rendering to determine `status`; it does not prove visual compositor parity.
- Per-recipe `errors` includes unsupported adapter diagnostics from the existing K0 render path, so recipes with multiple unsupported nodes may contain multiple diagnostics even when `unsupportedEffectIds` is deduplicated.
- The next packet should use this gate as the control surface before any broad migration: compare old/new debug recipe families and document the migration queue.

<!-- <FILE>docs/new_kernel/PHASE_K2_0_INVENTORY_GATE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.0 inventory gate status memo to the v3.1 architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
