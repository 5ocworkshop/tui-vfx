<!-- <FILE>docs/design/tui-vfx-v3-first-slice-checklist.md</FILE> - <DESC>Execution checklist for the first concrete V3 implementation slice.</DESC> -->
<!-- <VERS>VERSION: 0.6.0</VERS> -->
<!-- <WCTX>Tracks the first code-facing work package after schema/catalog/lowering/IR/validator planning. Updated now that FS-01 through FS-08 all have real code footholds or focused passing tests in the new V3 module.</WCTX> -->
<!-- <CLOG>0.6.0: mark FS-08 explicitly in progress and keep the first-slice checklist aligned with the now-documented V3 scaffolding module headers/footers.
0.5.0: mark FS-06/FS-07/FS-08 as in progress after the first hint-validation, scene-normalization, and canonical IR dump helpers landed in `tui-vfx-recipes::v3` with focused passing tests.
0.4.0: mark FS-03/FS-04/FS-05 as in progress after the first normalization helpers landed in `tui-vfx-recipes::v3::normalize` and their focused unit tests passed.
0.3.0: record initial verification signal for FS-01/FS-02 after the V3 scaffold compiled under `cargo check --lib` and the focused parse unit test passed.
0.2.0: mark FS-01 and FS-02 as in progress after adding initial V3 authoring-schema and normalized-IR scaffolding module in tui-vfx-recipes.
0.1.0: initial checklist. Seeds the first implementation slice with concrete deliverables and status slots.</CLOG> -->

# tui-vfx V3 first implementation slice checklist

## Status tracker

| ID | Work item | Status | Notes |
|---|---|---|---|
| FS-01 | Authoring schema parse types | IN_PROGRESS | Initial `src/v3::authoring` scaffold added in `tui-vfx-recipes`; focused parse unit test passes |
| FS-02 | Normalized IR core types | IN_PROGRESS | Initial `src/v3::normalized` scaffold added in `tui-vfx-recipes`; library compile check completed |
| FS-03 | Region-ref resolution | IN_PROGRESS | Initial normalization helper implemented in `tui-vfx-recipes::v3::normalize`; focused unit test passes |
| FS-04 | `cell_run` / `cell_runs` canonicalization | IN_PROGRESS | Initial canonicalization helper implemented in `tui-vfx-recipes::v3::normalize`; focused unit test passes |
| FS-05 | Style normalization pass | IN_PROGRESS | Initial `base_style` → `base_style_override` normalization implemented in `tui-vfx-recipes::v3::normalize`; focused unit test passes |
| FS-06 | Hint producer/consumer validation | IN_PROGRESS | Initial validator in `tui-vfx-recipes::v3::validate`; focused unit tests pass for duplicate/missing hint cases |
| FS-07 | Scene placement normalization | IN_PROGRESS | Initial scene-layer default placement/surface normalization implemented in `tui-vfx-recipes::v3::normalize`; focused unit test passes |
| FS-08 | Canonical IR dump/debug output | OPEN | |

## Minimum first-code definition of done

The first code slice is not done until:

- FS-01 through FS-08 have at least an initial implementation plan or code path
- the resulting normalized IR can be inspected in a deterministic dump form
- later runtime-family work can proceed against normalized IR rather than raw authoring syntax

## First recommended code order

1. parse types
2. normalized IR core types
3. region-ref resolution
4. style normalization
5. hint validation
6. IR dump/debug output
7. scene placement normalization

<!-- <FILE>docs/design/tui-vfx-v3-first-slice-checklist.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
