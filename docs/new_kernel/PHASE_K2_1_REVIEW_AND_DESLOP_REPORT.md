<!-- <FILE>docs/new_kernel/PHASE_K2_1_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>Formal third-party review and de-slop report for K2.1</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase K2.1: document formal review and AI de-slop completion.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture reviewer findings, cleanup passes, fixes, and verification.</CLOG> -->

# Phase K2.1 Formal Review and AI De-slop Report

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Scope: K2.0/K2.1 touched files only

## Standing workflow update

The user requested that formal third-party review and formal AI de-slop become part of the normal workflow going forward. This directive was recorded in:

```text
.omx/project-memory.json
```

## Third-party review summary

### Code/spec/security lane

Reviewer recommendation: **APPROVE**

Findings:

| Severity | Finding | Resolution |
|---|---|---|
| LOW | `migration-gap` accepted and ignored positional/render-only options | Fixed. Added scoped migration-gap option validation and a regression test. |

Evidence reviewed by the third-party lane:

```text
render-recipe remains report-only and intact
inventory-recipes remains report-only and intact
migration-gap is report-only
no recipe roots modified
no K1/compositor files touched
no new dependencies
```

### Architecture/devil's-advocate lane

Architectural status: **WATCH**

Concerns and resolutions:

| Concern | Resolution in de-slop cycle |
|---|---|
| Hardcoded migration policy was becoming too exposed as public API | Narrowed public exports: internal family inventory/classification helpers are no longer re-exported from `tui-vfx-player`. |
| `migration-gap` accepted irrelevant render/inventory options | Added command-specific validation in `fnc_validate_migration_gap_options.rs`. |
| Descriptor-pack validation was not visible in report output | Added `descriptorPacks` provenance to `v3.1.player.migrationGap.1`. |
| OFPF/file-size pressure in new helper files | Split oversized DTO/aggregation/command files into focused OFPF-sized modules. |
| Readiness/status semantics remain provisional planning policy | Left as WATCH; report is still explicitly planning guidance, not semantic parity. |

Final synthesized recommendation after fixes: **APPROVE with WATCH** for provisional migration planning semantics.

## AI SLOP CLEANUP REPORT

Scope: K2.0/K2.1 touched files only.

Behavior lock before cleanup:

```text
cargo test -p tui-vfx-player-cli  PASS, 8 tests before cleanup
```

Cleanup plan executed:

```text
Pass 1: Dead code / irrelevant command-surface cleanup
Pass 2: Duplicate and oversized helper cleanup
Pass 3: Naming / error-handling / public boundary cleanup
Pass 4: Test reinforcement
```

### Passes completed

1. **Dead code / command-surface cleanup**
   - Added `migration-gap` validation to reject positional recipe paths, `--recursive`, frame size options, loop timing, and lifecycle sampling options.
   - Added regression coverage for accidental migration-gap recipe path input.

2. **Duplicate / oversized helper cleanup**
   - Split `cls_player_inventory_report.rs` into focused DTO and aggregation helper files.
   - Split `fnc_build_migration_gap_report.rs` into focused family construction, family ordering, summary, and queue helper files.
   - Split CLI top-level dispatch from render/inventory/migration command runners.
   - Split recipe-inventory id extraction and file-error construction out of `fnc_inventory_recipe_file.rs`.

3. **Naming / boundary cleanup**
   - Kept public exports to report DTOs and top-level command-facing helpers.
   - Kept internal family inventory/classification modules private to `tui-vfx-player`.
   - Added descriptor-pack provenance to the migration-gap report instead of silently validating descriptor packs.

4. **Test reinforcement**
   - CLI regression count increased from 8 to 9 tests.
   - New test: `test_fnc_cli_rejects_migration_gap_recipe_paths`.
   - Existing render, inventory, and migration-gap assertions still pass.

## Files added/changed by formal de-slop

Player library de-slop additions:

```text
crates/tui-vfx-player/src/cls_player_inventory_effect.rs
crates/tui-vfx-player/src/cls_player_inventory_source.rs
crates/tui-vfx-player/src/cls_player_inventory_summary.rs
crates/tui-vfx-player/src/fnc_aggregate_player_inventory_effects.rs
crates/tui-vfx-player/src/fnc_aggregate_player_inventory_sources.rs
crates/tui-vfx-player/src/fnc_classify_debug_recipe_family.rs
crates/tui-vfx-player/src/fnc_collect_descriptor_inventory_ids.rs
crates/tui-vfx-player/src/fnc_collect_migration_gap_family_names.rs
crates/tui-vfx-player/src/fnc_extract_recipe_inventory_ids.rs
crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs
crates/tui-vfx-player/src/fnc_player_inventory_file_error.rs
crates/tui-vfx-player/src/fnc_recommend_migration_queue.rs
crates/tui-vfx-player/src/fnc_summarize_migration_gap_families.rs
crates/tui-vfx-player/src/fnc_summarize_player_inventory.rs
```

Player CLI de-slop additions:

```text
crates/tui-vfx-player-cli/src/fnc_cli_sample_request.rs
crates/tui-vfx-player-cli/src/fnc_collect_cli_recipe_paths.rs
crates/tui-vfx-player-cli/src/fnc_print_render_report.rs
crates/tui-vfx-player-cli/src/fnc_report_root.rs
crates/tui-vfx-player-cli/src/fnc_run_inventory_recipes.rs
crates/tui-vfx-player-cli/src/fnc_run_migration_gap.rs
crates/tui-vfx-player-cli/src/fnc_run_render_recipe.rs
crates/tui-vfx-player-cli/src/fnc_validate_migration_gap_options.rs
```

Existing files materially updated:

```text
crates/tui-vfx-player/src/lib.rs
crates/tui-vfx-player/src/cls_player_inventory_report.rs
crates/tui-vfx-player/src/cls_player_migration_gap_report.rs
crates/tui-vfx-player/src/fnc_build_migration_gap_report.rs
crates/tui-vfx-player/src/fnc_collect_debug_recipe_family_inventory.rs
crates/tui-vfx-player/src/fnc_inventory_recipe_file.rs
crates/tui-vfx-player-cli/src/main.rs
crates/tui-vfx-player-cli/src/fnc_run.rs
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
```

Workflow memory updated:

```text
.omx/project-memory.json
```

## OFPF result

New/touched K2.0/K2.1 helper files are now under hard OFPF limits. Examples:

```text
crates/tui-vfx-player/src/cls_player_inventory_report.rs          63 LOC
crates/tui-vfx-player/src/fnc_inventory_recipe_file.rs            70 LOC
crates/tui-vfx-player/src/fnc_collect_debug_recipe_family_inventory.rs 115 LOC
crates/tui-vfx-player/src/fnc_build_migration_gap_report.rs        51 LOC
crates/tui-vfx-player/src/fnc_build_migration_gap_family.rs       114 LOC
crates/tui-vfx-player-cli/src/fnc_run.rs                           54 LOC
```

Pre-existing non-packet OFPF pressure remains outside this cleanup scope:

```text
crates/tui-vfx-player/src/fnc_load_descriptor_catalog.rs 122 LOC
crates/tui-vfx-player/src/fnc_render_scene.rs            164 LOC
```

## Verification evidence after de-slop

```text
cargo fmt --package tui-vfx-player -- --check                         PASS
cargo fmt --package tui-vfx-player-cli -- --check                     PASS
cargo test -p tui-vfx-player                                         PASS, 4 tests
cargo test -p tui-vfx-player-cli                                     PASS, 9 tests
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings  PASS
cargo run -q -p tui-vfx-player-cli -- render-recipe --recursive ...   PASS, total=16 rendered=10 unsupported=6 errors=0
cargo run -q -p tui-vfx-player-cli -- inventory-recipes --recursive ... PASS, totalRecipes=16 rendered=10 unsupported=6 errors=0 descriptorEffectIds=14 representedEffectIds=14 unrepresentedEffectIds=0 unsupportedEffectIds=6
cargo run -q -p tui-vfx-player-cli -- migration-gap ...               PASS, legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7 descriptorPacks=1
git diff --check                                                      PASS
git -C /usr/projects/tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes  PASS, no output
```

Captured JSON outputs:

```text
/tmp/tui-vfx-review-render-report.json
/tmp/tui-vfx-review-inventory-report.json
/tmp/tui-vfx-review-migration-gap-report.json
```

## Remaining risks

- Migration `status` values remain provisional planning labels. Treat the architecture status as **WATCH** until a future packet either documents the semantics more explicitly or splits readiness from planning status.
- CLI tests still depend on the sibling recipe repo and current corpus counts. This is intentional for these K0 control-surface gates, but it is not hermetic.
- Pre-existing OFPF pressure remains in older player files outside this packet scope.

<!-- <FILE>docs/new_kernel/PHASE_K2_1_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>Formal third-party review and de-slop report for K2.1</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
