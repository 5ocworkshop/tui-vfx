<!-- <FILE>docs/new_kernel/K2_25_REMAINING_CONTENT_STAGE_RESULTS.md</FILE> - <DESC>K2.25 remaining native content-stage results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native content-stage expansion: cell motion, marquee, morph, scramble, and wrap indicator support plus coverage movement.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture remaining content transform native-stage results and audit evidence.</CLOG> -->

# K2.25 remaining content stage results

## Successful results

The remaining representable `content.*` blockers now run through strict compositor-native source-only content stages for current non-deprecated v3.1 debug recipe fields:

- `content.cellMotion` — 3 recipes
- `content.marquee` — 2 recipes
- `content.morph` — 2 recipes
- `content.scramble` — 2 recipes
- `content.wrapIndicator` — 2 recipes

Native evidence required by the regressions:

- `compositionMode=native`
- `fallbackUsed=false`
- `nativeLoweringAttempted=true`
- `nativeLoweringSucceeded=true`
- `sourceRenderMode=sourceOnly`
- `nativeSourceIsolated=true`
- `compositionSpecSummary.contentStages=1`
- `loweredEffectIds` contains the relevant content effect

## No-silent-fallback behavior

Unsupported inputs, graph outputs, and non-`all` scopes remain unsupported. The negative regression requires strict native mode with `--fail-on-fallback` to fail and report `unsupportedNativeEffect`.

## Audit movement

Full native coverage audit after this content-stage tranche:

```text
recipes=144 nativePasses=81 fallbacks=63 hardErrors=0
topUnsupported=shader.revealWipe:3, filter.kittScanner:2, filter.patternFill:2, mask.materialize:2, mask.noiseDither:2, sampler.faultLine:2, sampler.radialTwist:2, sampler.shredder:2
```

This moves the K2.24 baseline from 70 native passes / 74 fallbacks to 81 native passes / 63 fallbacks.

## Verification evidence

- `cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli test_fnc_cli_renders_compositor_backend_native_remaining_content_transforms_json test_fnc_cli_rejects_native_remaining_content_transform_unsupported_shapes_json --no-fail-fast` — PASS, 2/2.
- `./scripts/k222_native_coverage_audit.sh` — PASS, 81 native passes / 63 fallbacks / 0 hard errors.

<!-- <FILE>docs/new_kernel/K2_25_REMAINING_CONTENT_STAGE_RESULTS.md</FILE> - <DESC>K2.25 remaining native content-stage results</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
