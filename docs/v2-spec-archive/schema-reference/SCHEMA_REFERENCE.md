<!-- <FILE>docs/schema/SCHEMA_REFERENCE.md</FILE> - <DESC>Hand-maintained reference for the additive scene and continuous recipe blocks</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Sub-plan B Phase B.1 — capture the new scene-bearing and continuous recipe schema blocks in one place until generated docs incorporate the additive types.</WCTX> -->
<!-- <CLOG>0.1.0: initial hand-written reference for scene, continuous, and per-effect clock fields.</CLOG> -->

# Schema Reference

## `RaRecipeConfig.scene`

Optional scene-bearing block with:

- `layers: Vec<RaSceneLayer>`
- `fit_policy: clip | shrink | scroll`
- `default_role: <role shorthand>`

Each `RaSceneLayer` carries `id`, `z`, `placement`, `source`, `role_tag`, `overflow`, and `visibility`.

## `RaPipelineConfig.continuous`

Optional cross-phase effect block mirroring the real pipeline shape:

- `mask: RaMaskConfig`
- `sampler: RaSamplerConfig`
- `filter: RaFilterConfig`
- `styles: Vec<RaStylePipelineConfig>`
- `clock: phase_t | loop_t | absolute_t`

## Per-effect clock overrides

The following existing types now accept `clock: Option<RaClock>`:

- `RaMaskConfig`
- `RaSamplerConfig`
- `RaFilterConfig`
- `RaStylePipelineConfig`

Default semantics:

- phase-local blocks inherit `phase_t`
- `continuous` inherits its own `clock` field
- explicit per-effect overrides win when present

## Validation rules added in B.1

- duplicate scene-layer ids → error
- empty scene layer list → warning
- `overflow = wrap` on non-text sources → error
- empty/whitespace procedural `source_id` → error
- empty phase-visibility set → warning
- empty continuous block → warning
- `clock = phase_t` inside `continuous` → warning

<!-- <FILE>docs/schema/SCHEMA_REFERENCE.md</FILE> - <DESC>Hand-maintained reference for the additive scene and continuous recipe blocks</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
