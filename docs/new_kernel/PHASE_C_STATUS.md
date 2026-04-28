<!-- <FILE>docs/new_kernel/PHASE_C_STATUS.md</FILE> - <DESC>Status memo for new kernel Phase C ordered pipeline implementation</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>New kernel Phase C verification: clarify descriptor boundary and note strengthened tests.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — clarify no descriptor expansion and incorporate leader test strengthening.</CLOG> -->

# Phase C Status — Ordered Multi-Stage Pipeline Semantics

Date: 2026-04-28

## Implemented

- Added a tiny ordered `SurfacePipeline` abstraction in `tui-vfx-next`.
- Added enum-based `PipelineStage` toy semantics for copy/sample, dim visual-only, explicit role write, and a small glyph-rewrite test helper.
- Defined the Phase C surface flow: each stage reads current, writes next, then next becomes current.
- Preserved Phase A/B scope, sampled-source role, skip, and diagnostic behavior inside pipeline stages.
- Made pipeline diagnostics deterministic and stage-aware with `pipeline.stage[index].name` paths.

## Deliberately not implemented

- No real effect ports.
- No descriptor registry/model expansion; the tiny Phase A `EffectDescriptor` proof DTO remains unchanged.
- No recipes, studio manifest, runtime bindings, trigger engine, or phase graph.
- No legacy compositor/style/content/shadow crate dependencies.
- No physical crate split.

## Acceptance coverage

The Phase C tests in `crates/tui-vfx-next/tests/test_surface_contract.rs` cover:

1. later stages read earlier stage cells;
2. later stages read earlier stage roles;
3. stage order is semantic;
4. visual stages preserve prior roles;
5. skips preserve the current surface;
6. zero-cell diagnostics name the stage;
7. diagnostics are deterministic by stage order;
8. Phase B sampled-source role semantics still hold inside a pipeline.

## Open risks

- The stage model is intentionally toy-sized and enum-based. A later descriptor phase should revisit API shape before generalizing.
- Stage names are caller-provided strings and are embedded in diagnostic paths; later recipe/runtime work may need stricter path segment validation.

<!-- <FILE>docs/new_kernel/PHASE_C_STATUS.md</FILE> - <DESC>Status memo for new kernel Phase C ordered pipeline implementation</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
