<!-- <FILE>docs/new_kernel/K2_30_RADIAL_WIPE_CORNER_RESULTS.md</FILE> - <DESC>v3.1 radial and wipe-corner native blocker closure results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Results for closing the remaining radial and wipe-corner strict-native debug-recipe mask blockers.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record strict-native radial/wipe-corner source-stage results, counters, and remaining blockers.</CLOG> -->

# Radial and wipe-corner native blocker closure results

## Result

The packet closed the two remaining non-deprecated v3.1 `debug_recipes` mask native fallbacks.

Coverage moved from the committed baseline:

```text
recipes=144 nativePasses=125 fallbacks=19 hardErrors=0
topUnsupported=mask.radial:1, mask.wipeCorner:1, sampler.crt:1, sampler.crtJitter:1, shader.barberPole:1, shader.diffusion:1, shader.focusField:1, shader.glistenBand:1
```

to:

```text
recipes=144 nativePasses=127 fallbacks=17 hardErrors=0
topUnsupported=sampler.crt:1, sampler.crtJitter:1, shader.barberPole:1, shader.diffusion:1, shader.focusField:1, shader.glistenBand:1, shader.highlighter:1, shader.radar:1
```

That is **2 additional strict-native passes** with zero hard errors.

## Recipes moved to strict native

- `masks/mask_radial.json`
- `masks/mask_wipe_corner_out_from_top_left.json`

## Model decisions integrated

- `mask.radial` uses a backend-owned source content stage with player-compatible circle reveal semantics because the current debug fixture and player primitive only preserve center-origin behavior.
- `mask.wipeCorner` uses the existing backend-owned wipe source content stage with corner-direction validation, matching the player adapter's wipe-corner row/glyph behavior.
- Strict native enum handling rejects unsupported `origin` and `direction` values instead of defaulting them silently.
- Native-vs-`irResolved` `rows` and `styledCells` parity is executable for both target recipes at `phase_t=0.35`.

## Guardrails retained

Strict native lowering rejects unsupported graph shapes instead of silently dropping semantics:

- unknown authored fields,
- graph outputs that require value propagation not represented by backend stages,
- unsupported scopes for source content stages,
- unsupported enum values for `origin` and `direction`.

## Verification evidence collected so far

```text
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_renders_compositor_backend_native_radial_wipe_corner_blockers_json \
  test_fnc_cli_rejects_native_radial_wipe_corner_invalid_enum_values_json \
  test_fnc_cli_rejects_native_radial_wipe_corner_unsupported_shapes_json \
  --no-fail-fast
# PASS: 3 tests run, 3 passed, 71 skipped
```

```text
./scripts/k222_native_coverage_audit.sh
# PASS: recipes=144 nativePasses=127 fallbacks=17 hardErrors=0
```

Full post-review verification is recorded in `PHASE_K2_30_REVIEW_AND_DESLOP_REPORT.md`.

## Remaining native blockers

The current audit top list is now sampler/shader-heavy:

- `sampler.crt:1`
- `sampler.crtJitter:1`
- `shader.barberPole:1`
- `shader.diffusion:1`
- `shader.focusField:1`
- `shader.glistenBand:1`
- `shader.highlighter:1`
- `shader.radar:1`

Recommended next work packet: attack `sampler.crt` and `sampler.crtJitter` first, then move to shader/style blockers with explicit direct-spec versus source-stage decisions for each non-isomorphic fixture field set.
