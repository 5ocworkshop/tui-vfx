<!-- <FILE>docs/design/tui-vfx-v3-first-slice-checklist.md</FILE> - <DESC>Execution checklist for the first concrete V3 implementation slice.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Tracks the first code-facing work package after schema/catalog/lowering/IR/validator planning. Updated now that the V3 spine has direct entry points, recipe-layer wrappers, top-level exports, and a separate example path while the legacy viewer remains untouched.</WCTX> -->
<!-- <CLOG>1.0.0: record that FS-08 now has an initial implementation path, note the broader V3 entry-surface propagation, and explicitly hand off to the compiled execution-plan follow-on.
0.9.0: record the new recipe-layer V3 wrappers (`parse_v3`, `load_v3`) and their focused passing tests so FS-01/FS-02 reflect the actual reachable API surface.
0.8.0: record the first end-to-end V3 example smoke-test output via `examples/v3_play_recipe.rs`, confirming file load + parse + normalize + dump on a standalone V3 path.
0.7.0: record the new public V3 parse/normalize entry points and their focused passing tests for FS-01/FS-02.
0.6.0: mark FS-08 explicitly in progress and keep the first-slice checklist aligned with the now-documented V3 scaffolding module headers/footers.
0.5.0: mark FS-06/FS-07/FS-08 as in progress after the first hint-validation, scene-normalization, and canonical IR dump helpers landed in `tui-vfx-recipes::v3` with focused passing tests.
0.4.0: mark FS-03/FS-04/FS-05 as in progress after the first normalization helpers landed in `tui-vfx-recipes::v3::normalize` and their focused unit tests passed.
0.3.0: record initial verification signal for FS-01/FS-02 after the V3 scaffold compiled under `cargo check --lib` and the focused parse unit test passed.
0.2.0: mark FS-01 and FS-02 as in progress after adding initial V3 authoring-schema and normalized-IR scaffolding module in tui-vfx-recipes.
0.1.0: initial checklist. Seeds the first implementation slice with concrete deliverables and status slots.</CLOG> -->

# tui-vfx V3 first implementation slice checklist

## Status tracker

| ID | Work item | Status | Notes |
|---|---|---|---|
| FS-01 | Authoring schema parse types | IN_PROGRESS | Initial `src/v3::authoring` scaffold plus public `parse_v3_document`, recipe-layer `parse_v3`, and crate-root/prelude V3 parse exports in `tui-vfx-recipes`; focused parse tests pass |
| FS-02 | Normalized IR core types | IN_PROGRESS | Initial `src/v3::normalized` scaffold plus public `normalize_v3_document`, recipe-layer `load_v3_normalized`, and crate-root/prelude V3 normalized-path exports in `tui-vfx-recipes`; focused normalize/load tests pass |
| FS-03 | Region-ref resolution | IN_PROGRESS | Initial normalization helper implemented in `tui-vfx-recipes::v3::normalize`; focused unit test passes |
| FS-04 | `cell_run` / `cell_runs` canonicalization | IN_PROGRESS | Initial canonicalization helper implemented in `tui-vfx-recipes::v3::normalize`; focused unit test passes |
| FS-05 | Style normalization pass | IN_PROGRESS | Initial `base_style` → `base_style_override` normalization implemented in `tui-vfx-recipes::v3::normalize`; focused unit test passes |
| FS-06 | Hint producer/consumer validation | IN_PROGRESS | Initial validator in `tui-vfx-recipes::v3::validate`; focused unit tests pass for duplicate/missing hint cases |
| FS-07 | Scene placement normalization | IN_PROGRESS | Initial scene-layer default placement/surface normalization implemented in `tui-vfx-recipes::v3::normalize`; focused unit test passes |
| FS-08 | Canonical IR dump/debug output | IN_PROGRESS | `dump_normalized_recipe_pretty` exists, the separate `examples/v3_play_recipe.rs` path prints normalized IR, and the next follow-on is now the compiled execution-plan phase |

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
