<!-- <FILE>docs/new_kernel/K2_29_VIGNETTE_MASK_RESULTS.md</FILE> - <DESC>v3.1 vignette and mask native blocker closure results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Results for closing the remaining top vignette and mask strict-native debug-recipe blockers.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record strict-native vignette/mask source-stage and mask-spec results, counters, and remaining blockers.</CLOG> -->

# Vignette and mask native blocker closure results

## Result

The packet closed the current top non-deprecated v3.1 `debug_recipes` native fallbacks for the targeted vignette/mask effect set.

Coverage moved from the committed baseline:

```text
recipes=144 nativePasses=117 fallbacks=27 hardErrors=0
topUnsupported=filter.vignette:1, mask.blinds:1, mask.cellular:1, mask.diamond:1, mask.dissolve:1, mask.iris:1, mask.none:1, mask.pathReveal:1
```

to:

```text
recipes=144 nativePasses=125 fallbacks=19 hardErrors=0
topUnsupported=mask.radial:1, mask.wipeCorner:1, sampler.crt:1, sampler.crtJitter:1, shader.barberPole:1, shader.diffusion:1, shader.focusField:1, shader.glistenBand:1
```

That is **8 additional strict-native passes** with zero hard errors.

## Recipes moved to strict native

- `filters/filter_vignette.json`
- `masks/mask_blinds.json`
- `masks/mask_cellular.json`
- `masks/mask_diamond.json`
- `masks/mask_dissolve.json`
- `masks/mask_iris.json`
- `masks/mask_none.json`
- `masks/mask_path_reveal.json`

## Model decisions integrated

- `filter.vignette` uses a backend-owned source style stage when authored `edgeColor` or `applyTo` fields are present, preserving player-visible color and channel semantics with native-vs-`irResolved` `rows` and `styledCells` parity.
- `mask.blinds`, `mask.cellular`, `mask.diamond`, `mask.dissolve`, `mask.iris`, and `mask.pathReveal` use backend-owned source content stages because the direct compositor mask contracts are not yet exact player-visible matches for the current fixtures. Native-vs-`irResolved` `rows` and `styledCells` parity now locks those behaviors.
- `mask.none` lowers to direct compositor `MaskSpec::None`, which is player-visible parity-preserving for the current fixture.
- Strict native enum handling now rejects unsupported `orientation`, `shape`, and `direction` values instead of defaulting them silently.
- Native backend metadata continues to report the honest route through `compositionSpecSummary.masks`, `compositionSpecSummary.contentStages`, and `compositionSpecSummary.styleStages`.

## Guardrails retained

Strict native lowering still rejects unsupported graph shapes instead of silently dropping semantics:

- unknown authored fields,
- graph outputs that require value propagation not represented by backend stages,
- unsupported scopes for source content/style stages,
- unsupported scopes for strict-native mask stages,
- mixed `filter.vignette` field shapes that would combine source-style-only fields with direct compositor-only fields.

The CLI regression suite covers both successful target fixtures and unsupported-shape rejection cases for every target effect.

## Verification evidence collected so far

```text
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_renders_compositor_backend_native_vignette_mask_blockers_json \
  test_fnc_cli_rejects_native_vignette_mask_blocker_invalid_enum_values_json \
  test_fnc_cli_rejects_native_vignette_mask_blocker_unsupported_shapes_json \
  --no-fail-fast
# PASS: 3 tests run, 3 passed, 68 skipped
```

```text
./scripts/k222_native_coverage_audit.sh
# PASS: recipes=144 nativePasses=125 fallbacks=19 hardErrors=0
```

The full post-review verification set is recorded in `PHASE_K2_29_REVIEW_AND_DESLOP_REPORT.md`.

## Remaining native blockers

The current audit top list is now:

- `mask.radial:1`
- `mask.wipeCorner:1`
- `sampler.crt:1`
- `sampler.crtJitter:1`
- `shader.barberPole:1`
- `shader.diffusion:1`
- `shader.focusField:1`
- `shader.glistenBand:1`

Recommended next work packet: close the two remaining mask fallbacks first, then move to the sampler/shader tranche with explicit direct-spec versus source-stage decisions for each non-isomorphic fixture field set.
