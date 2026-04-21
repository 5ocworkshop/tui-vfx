<!-- <FILE>docs/design/tui-vfx-v3-first-slice-checklist.md</FILE> - <DESC>Execution checklist for the first concrete V3 implementation slice.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Tracks the first code-facing work package after schema/catalog/lowering/IR/validator planning. Updated now that the first real V3 scaffolding code has started landing in tui-vfx-recipes.</WCTX> -->
<!-- <CLOG>0.2.0: mark FS-01 and FS-02 as in progress after adding initial V3 authoring-schema and normalized-IR scaffolding module in tui-vfx-recipes.
0.1.0: initial checklist. Seeds the first implementation slice with concrete deliverables and status slots.</CLOG> -->

# tui-vfx V3 first implementation slice checklist

## Status tracker

| ID | Work item | Status | Notes |
|---|---|---|---|
| FS-01 | Authoring schema parse types | IN_PROGRESS | Initial `src/v3::authoring` scaffold added in `tui-vfx-recipes` |
| FS-02 | Normalized IR core types | IN_PROGRESS | Initial `src/v3::normalized` scaffold added in `tui-vfx-recipes` |
| FS-03 | Region-ref resolution | OPEN | |
| FS-04 | `cell_run` / `cell_runs` canonicalization | OPEN | |
| FS-05 | Style normalization pass | OPEN | |
| FS-06 | Hint producer/consumer validation | OPEN | |
| FS-07 | Scene placement normalization | OPEN | |
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
