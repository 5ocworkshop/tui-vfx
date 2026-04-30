<!-- <FILE>docs/new_kernel/K2_27_RESIDUAL_STYLE_CONTENT_RESULTS.md</FILE> - <DESC>v3.1 residual style/content native blocker closure results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Results for closing the residual strict-native style/content blocker tranche.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record strict-native source style/content stage results, counters, and remaining blockers.</CLOG> -->

# Residual style/content native blocker closure results

## Result

The packet closed the representable non-deprecated v3.1 `debug_recipes` native fallbacks for the targeted residual style/content effect set.

Coverage moved from the committed baseline:

```text
recipes=144 nativePasses=99 fallbacks=45 hardErrors=0
topUnsupported=style.moduloColumns:2, style.neonFlicker:2, content.dissolve:1, content.glitchShift:1, content.mirror:1, content.numeric:1, content.redact:1, content.scrambleGlitchShift:1
```

to:

```text
recipes=144 nativePasses=109 fallbacks=35 hardErrors=0
topUnsupported=content.slideShift:1, filter.bracketEmphasis:1, filter.dotIndicator:1, filter.edgeGrow:1, filter.hoverBar:1, filter.matrixRain:1, filter.subPixelBar:1, filter.underlineWipe:1
```

That is **10 additional strict-native passes** with zero hard errors.

## Recipes moved to strict native

- `styles/style_modulo_columns_period.json`
- `styles/style_modulo_vertical_every_fourth_column_offset.json`
- `styles/style_neon_flicker.json`
- `styles/style_neon_flicker_modifier.json`
- `content/content_dissolve.json`
- `content/content_glitch_shift.json`
- `content/content_mirror.json`
- `content/content_numeric.json`
- `content/content_redact.json`
- `content/content_scramble_glitch_shift.json`

## Model decisions integrated

- `style.moduloColumns` lowers to a backend-owned native style stage that applies authored foreground/background colors to the authored modulo-column scope without pretending it is a general compositor shader.
- `style.neonFlicker` lowers to a deterministic native style stage matching the player primitive clock/stability/dimming contract instead of using the stochastic compositor shader with different semantics.
- `content.dissolve`, `content.glitchShift`, `content.mirror`, `content.numeric`, and `content.redact` lower to source-only native content stages that mirror the player primitive behavior for current authored fields.
- `content.scrambleGlitchShift` lowers as the honest two-stage composition of native scramble plus native glitch shift, preserving the authored combined behavior without adding a fake one-off durable field name.
- Native backend metadata now reports both `nativeContentStages` and `nativeStyleStages`, and `compositionSpecSummary` reports `contentStages` and `styleStages` counts.

## Guardrails retained

Strict native lowering still rejects unsupported graph shapes instead of silently dropping semantics:

- unknown authored fields,
- graph outputs that require value propagation not represented by the backend stage,
- unsupported scopes for content stages,
- unsupported scopes for native style stages.

The CLI regression suite covers both successful target fixtures and unsupported-shape rejection cases.

## Verification evidence

```text
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_renders_compositor_backend_native_residual_style_content_blockers_json \
  test_fnc_cli_rejects_native_residual_style_content_blocker_unsupported_shapes_json \
  --no-fail-fast
# PASS: 2 tests run, 2 passed
```

```text
./scripts/k222_native_coverage_audit.sh
# PASS: recipes=144 nativePasses=109 fallbacks=35 hardErrors=0
```

```text
cargo fmt --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli -- --check
cargo clippy -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --all-targets -- -D warnings
# PASS
```

```text
cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-fail-fast
# PASS: 68 tests run, 68 passed, 0 skipped
```

## Remaining native blockers

The current audit top list is now one-off blockers rather than repeated style/content blockers:

- `content.slideShift:1`
- `filter.bracketEmphasis:1`
- `filter.dotIndicator:1`
- `filter.edgeGrow:1`
- `filter.hoverBar:1`
- `filter.matrixRain:1`
- `filter.subPixelBar:1`
- `filter.underlineWipe:1`

Recommended next work packet: continue the blocker burn-down with the remaining one-off content/filter effects, starting with representable filter families and keeping source-stage semantics for content transforms.
