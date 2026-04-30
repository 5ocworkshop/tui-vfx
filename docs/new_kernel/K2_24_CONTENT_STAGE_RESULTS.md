<!-- <FILE>docs/new_kernel/K2_24_CONTENT_STAGE_RESULTS.md</FILE> - <DESC>K2.24 split-flap and odometer native content-stage results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native content-stage expansion: split-flap and odometer support plus coverage movement.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture strict native split-flap/odometer results and audit evidence.</CLOG> -->

# K2.24 content stage results

## Successful results

`content.splitFlap` and `content.odometer` now run through strict compositor-native source-only content stages for the current non-deprecated v3.1 debug recipe fields.

Native evidence required by the regressions:

- `compositionMode=native`
- `fallbackUsed=false`
- `nativeLoweringAttempted=true`
- `nativeLoweringSucceeded=true`
- `sourceRenderMode=sourceOnly`
- `nativeSourceIsolated=true`
- `compositionSpecSummary.contentStages=1`
- `loweredEffectIds` contains the relevant content effect

Positive coverage now includes all current debug fixtures for these two effects:

- `content.splitFlap` — 5 recipes
- `content.odometer` — 4 recipes

## No-silent-fallback behavior

Unsupported inputs, graph outputs, and non-`all` scopes remain unsupported for both effects. The negative regression requires strict native mode with `--fail-on-fallback` to fail and report `unsupportedNativeEffect`.

## Audit movement

Full native coverage audit after the content-stage tranche:

```text
recipes=144 nativePasses=70 fallbacks=74 hardErrors=0
topUnsupported=content.cellMotion:3, shader.revealWipe:3, content.marquee:2, content.morph:2, content.scramble:2, content.wrapIndicator:2, filter.kittScanner:2, filter.patternFill:2
```

This moves the K2.23 baseline from 61 native passes / 83 fallbacks to 70 native passes / 74 fallbacks.

## Verification evidence

- `cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli test_fnc_cli_renders_compositor_backend_native_split_flap_and_odometer_json test_fnc_cli_rejects_native_split_flap_and_odometer_unsupported_content_shapes_json --no-fail-fast` — PASS, 2/2.
- `./scripts/k222_native_coverage_audit.sh` — PASS, 70 native passes / 74 fallbacks / 0 hard errors.

<!-- <FILE>docs/new_kernel/K2_24_CONTENT_STAGE_RESULTS.md</FILE> - <DESC>K2.24 split-flap and odometer native content-stage results</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
