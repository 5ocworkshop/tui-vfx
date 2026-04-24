<!-- <FILE>docs/design/tui-vfx-v3-first-slice-checklist.md</FILE> - <DESC>Execution checklist for the first concrete V3 implementation slice.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Tracks the first code-facing work package after schema/catalog/lowering/IR/validator planning. Updated now that the V3 spine has direct entry points, recipe-layer wrappers, top-level exports, and a separate example path while the legacy viewer remains untouched.</WCTX> -->
<!-- <CLOG>1.0.1: reconcile stale first-slice statuses with as-built parse/normalize, validation, and dump evidence.</CLOG> -->

# tui-vfx V3 first implementation slice checklist

## Status tracker

| ID | Work item | Status | Notes |
|---|---|---|---|
| FS-01 | Authoring schema parse types | COMPLETE_INITIAL | `src/v3/fnc_parse_v3_document.rs`, `src/recipe/fnc_parse_v3.rs`, and `src/v3/authoring/test_authoring.rs` cover the initial parse surface; focused parse tests pass |
| FS-02 | Normalized IR core types | COMPLETE_INITIAL | `src/v3/fnc_normalize_v3_document.rs`, `src/recipe/fnc_load_v3_normalized.rs`, and `src/v3/normalize/test_normalize.rs` cover the initial normalize/load surface; focused normalize/load tests pass |
| FS-03 | Region-ref resolution | COMPLETE_INITIAL | `normalize_region_ref_and_defaults` in `src/v3/normalize/test_normalize.rs` covers region-ref resolution and default phase/scope handling |
| FS-04 | `cell_run` / `cell_runs` canonicalization | COMPLETE_INITIAL | `normalize_cell_runs_into_compact_runs` in `src/v3/normalize/test_normalize.rs` covers compacted cell-run normalization |
| FS-05 | Style normalization pass | COMPLETE_INITIAL | `normalize_base_style_into_base_style_override` in `src/v3/normalize/test_normalize.rs` covers the canonical style rewrite |
| FS-06 | Hint producer/consumer validation | COMPLETE_INITIAL | `src/v3/validate/col_collect_hints.rs` and `src/v3/validate/test_validate_normalized_recipe.rs` cover duplicate/missing hint cases |
| FS-07 | Scene placement normalization | COMPLETE_INITIAL | `normalize_scene_layer_defaults` in `src/v3/normalize/test_normalize.rs` covers default placement and surface normalization |
| FS-08 | Canonical IR dump/debug output | COMPLETE_INITIAL | `src/v3/normalize/fnc_dump_normalized_recipe_pretty.rs` and `examples/v3_play_recipe.rs` expose the normalized dump; follow-on execution-plan work is now the next slice |

## Minimum first-code definition of done

The first code slice is not done until:

- FS-01 through FS-08 have at least an initial implementation plan or code path
- the resulting normalized IR can be inspected in a deterministic dump form
- later runtime-family work can proceed against normalized IR rather than raw authoring syntax

## Follow-on after the first slice

The first-slice follow-on is now explicitly:

- `docs/design/tui-vfx-v3-upgrade-plan/62_compiled_execution_plan.md`
- `docs/design/tui-vfx-v3-compiled-execution-plan.md`

## First recommended code order

1. parse types
2. normalized IR core types
3. region-ref resolution
4. style normalization
5. hint validation
6. IR dump/debug output
7. scene placement normalization

<!-- <FILE>docs/design/tui-vfx-v3-first-slice-checklist.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
