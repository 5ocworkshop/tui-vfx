<!-- <FILE>docs/new_kernel/PHASE_K2_1_MIGRATION_GAP_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.1 migration gap status memo to the v3.1 architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase K2.1: summarize the report-only debug recipe migration gap gate.</WCTX> -->
<!-- <CLOG>0.2.0: PATCH — note formal review/de-slop validation and descriptor-pack provenance.</CLOG> -->

# Phase K2.1 Migration Gap Status Memo to the v3.1 Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Packet: `docs/new_kernel/ARCH-RESP-TO-PHASE_K2_1.md`

## Executive summary

K2.1 is complete for the bounded, report-only migration gap packet. K0/player-cli now has a `migration-gap` command that compares the legacy debug recipe corpus against the canonical v3.1 debug recipe corpus using path/family inventory only for legacy recipes and canonical JSON inspection for v3.1 effect ids.

No recipe files were modified. K1 and compositor code were not touched. Existing K2.0 `inventory-recipes` and K0 `render-recipe` behavior remain intact.

## New command shape

```bash
cargo run -p tui-vfx-player-cli -- migration-gap \
  --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes \
  --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json
```

## New schema label

```text
v3.1.player.migrationGap.1
```

Top-level fields:

```text
schemaVersion
legacyRoot
v31Root
descriptorPacks
summary
families
recommendedQueue
```

## Files touched

Player library:

```text
crates/tui-vfx-player/src/lib.rs
crates/tui-vfx-player/src/cls_player_migration_gap_report.rs
crates/tui-vfx-player/src/fnc_collect_debug_recipe_family_inventory.rs
crates/tui-vfx-player/src/fnc_build_migration_gap_report.rs
```

Player CLI/tests:

```text
crates/tui-vfx-player-cli/src/cls_cli_options.rs
crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs
crates/tui-vfx-player-cli/src/fnc_run.rs
crates/tui-vfx-player-cli/src/fnc_print_usage.rs
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
```

Documentation:

```text
docs/new_kernel/PHASE_K2_1_MIGRATION_GAP_STATUS_MEMO_TO_ARCHITECT.md
```

## Roots inspected

```text
legacyRoot=/usr/projects/tui-vfx-recipes/recipes/debug_recipes
v31Root=/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
descriptorPack=descriptors/v3.1/packs/primitive.json
```

## Summary counts

```text
legacyRecipes=603
v31Recipes=16
representedFamilies=8
unrepresentedFamilies=11
partiallyRepresentedFamilies=7
readyFamilies=10
blockedFamilies=9
```

## Family count summary

| Family | Legacy count | v3.1 count | Coverage | Status |
|---|---:|---:|---|---|
| `baseline` | 2 | 1 | `represented` | `migrationCandidateReady` |
| `filters` | 98 | 4 | `partial` | `adapterExpansionReady` |
| `masks` | 41 | 4 | `partial` | `adapterExpansionReady` |
| `samplers` | 13 | 2 | `partial` | `adapterExpansionReady` |
| `shaders/primitives` | 94 | 1 | `partial` | `adapterExpansionReady` |
| `shaders/compositions` | 39 | 1 | `partial` | `adapterExpansionReady` |
| `styles` | 34 | 2 | `partial` | `adapterExpansionReady` |
| `content` | 111 | 0 | `none` | `migrationCandidateReady` |
| `scene` | 19 | 0 | `none` | `migrationCandidateReady` |
| `shadows` | 9 | 0 | `none` | `descriptorDecisionNeeded` |
| `complex` | 83 | 0 | `none` | `adapterExpansionReady` |
| `event_driven_dwell` | 4 | 1 | `partial` | `ownerAuditNeeded` |
| `signals` | 5 | 0 | `none` | `schemaDecisionNeeded` |
| `easings` | 29 | 0 | `none` | `schemaDecisionNeeded` |
| `subcell_shapes` | 5 | 0 | `none` | `descriptorDecisionNeeded` |
| `motion_routes` | 5 | 0 | `none` | `schemaDecisionNeeded` |
| `loopback` | 3 | 0 | `none` | `schemaDecisionNeeded` |
| `bindable_rates` | 8 | 0 | `none` | `ownerAuditNeeded` |
| `fixtures` | 1 | 0 | `none` | `ownerAuditNeeded` |
| `other` | 0 | 0 | `notApplicable` | `notYetClassified` |

## Unrepresented families

`content`, `scene`, `shadows`, `complex`, `signals`, `easings`, `subcell_shapes`, `motion_routes`, `loopback`, `bindable_rates`, `fixtures`

## Partially represented families

`filters`, `masks`, `samplers`, `shaders/primitives`, `shaders/compositions`, `styles`, `event_driven_dwell`

## Recommended migration queue

1. `complex` — create a minimal v3.1 complex fixture. exercise mask + sampler + filter + shader + style/source after K2.0 inventory evidence
2. `primitive-adapters` — clear remaining primitive adapter blockers. reduce the six unsupported K0 primitive ids before broad recipe migration
3. `content` — add a content family pilot. content is legacy-present and v3.1-unrepresented but can start with path/descriptor inventory
4. `scene` — add a scene family pilot. scene coverage is absent and should stay small until scene semantics are confirmed
5. `shadows` — add a shadow family pilot. shadow migration needs descriptor decisions before broad parity claims
6. `complex` — choose complex legacy replacement candidates. legacy complex coverage is large and should follow a minimal canonical fixture
7. `signals/easings/motion_routes` — settle timing and signal semantics. these families need schema-level decisions before mechanical migration
8. `subcell_shapes/loopback/other` — audit advanced families. advanced or ambiguous families need owner review after core coverage improves

## Verification evidence

Required packet checks:

```text
cargo fmt --package tui-vfx-player -- --check                         PASS
cargo fmt --package tui-vfx-player-cli -- --check                     PASS
cargo test -p tui-vfx-player                                         PASS, 4 tests
cargo test -p tui-vfx-player-cli                                     PASS, 9 tests
cargo run -q -p tui-vfx-player-cli -- render-recipe --recursive ...   PASS, total=16 rendered=10 unsupported=6 errors=0
cargo run -q -p tui-vfx-player-cli -- inventory-recipes --recursive ... PASS, totalRecipes=16 rendered=10 unsupported=6 errors=0 descriptorEffectIds=14 representedEffectIds=14 unrepresentedEffectIds=0 unsupportedEffectIds=6
cargo run -q -p tui-vfx-player-cli -- migration-gap ...               PASS, legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7 descriptorPacks=1
```

Additional checks:

```text
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings  PASS
git diff --check                                                                      PASS
```

Captured JSON outputs used for review:

```text
/tmp/tui-vfx-k21-render-report.json
/tmp/tui-vfx-k21-inventory-report.json
/tmp/tui-vfx-k21-migration-gap-report.json
```

Recipe-root modification check:

```text
git -C /usr/projects/tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes
# no output
```

## Acceptance criteria status

| Criterion | Status |
|---|---|
| New migration-gap command exists | Done |
| Command accepts legacy root and v3.1 root | Done |
| Command emits stable JSON | Done |
| Report includes per-family legacy/v3.1 counts | Done |
| Report identifies unrepresented v3.1 families | Done |
| Report recommends a conservative migration queue | Done |
| Existing K2.0 inventory-recipes still works | Done |
| Existing render-recipe still works | Done |
| No recipe files modified | Confirmed |
| Status memo written | Done |


Formal review/de-slop artifact:

```text
docs/new_kernel/PHASE_K2_1_REVIEW_AND_DESLOP_REPORT.md
```

## Recommended next packet

Proceed to K2.2 only after treating this migration gap report as the planning control surface. The next packet should prepare the K0 visual-frame substrate while preserving the current text-row output and K0 CLI regression authority.

<!-- <FILE>docs/new_kernel/PHASE_K2_1_MIGRATION_GAP_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.1 migration gap status memo to the v3.1 architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
