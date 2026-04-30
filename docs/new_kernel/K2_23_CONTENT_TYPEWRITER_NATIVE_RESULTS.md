<!-- <FILE>docs/new_kernel/K2_23_CONTENT_TYPEWRITER_NATIVE_RESULTS.md</FILE> - <DESC>K2.23 content typewriter native results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Content native stage: record first native content transform implementation.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture native content.typewriter stage evidence and coverage movement.</CLOG> -->

# K2.23 content typewriter native results

## Successful results

`content.typewriter` now runs through a dedicated native backend content stage without weakening source-only native semantics.

Strict native evidence for `content/content_typewriter.json` proves:

- `fallbackUsed=false`
- `sourceRenderMode=sourceOnly`
- `nativeSourceIsolated=true`
- `nativeLoweringSucceeded=true`
- `loweredEffectIds=["content.typewriter"]`
- no `unsupportedNativeEffect`
- `compositionSpecSummary.contentStages=1`

The full native coverage audit improved again:

```text
recipes=144 nativePasses=61 fallbacks=83 hardErrors=0
```

Top remaining blockers after typewriter support:

- `content.splitFlap` — 5 recipes
- `content.odometer` — 4 recipes
- `content.cellMotion` — 3 recipes
- `shader.revealWipe` — 3 recipes

## No-silent-fallback behavior

Unsupported typewriter fields, outputs, or non-all scopes remain unsupported rather than being partially lowered. Auto fallback clears native content-stage metadata so fallback artifacts do not imply native source staging.

## Verification evidence

- Positive regression: `test_fnc_cli_renders_compositor_backend_native_content_typewriter_json`.
- Negative regression: `test_fnc_cli_rejects_native_content_typewriter_with_unsupported_input_json`.
- Integrated lane: `cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast` — PASS, 130/130.
- Audit: `./scripts/k222_native_coverage_audit.sh` — PASS, 61 native passes / 83 fallbacks / 0 hard errors.

<!-- <FILE>docs/new_kernel/K2_23_CONTENT_TYPEWRITER_NATIVE_RESULTS.md</FILE> - <DESC>K2.23 content typewriter native results</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
