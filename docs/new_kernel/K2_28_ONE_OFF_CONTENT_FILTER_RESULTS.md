<!-- <FILE>docs/new_kernel/K2_28_ONE_OFF_CONTENT_FILTER_RESULTS.md</FILE> - <DESC>v3.1 one-off content/filter native blocker closure results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Results for closing one-off strict-native content/filter blockers after residual style/content closure.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record strict-native source content/filter style-stage results, counters, and remaining blockers.</CLOG> -->

# One-off content/filter native blocker closure results

## Result

The packet closed the representable non-deprecated v3.1 `debug_recipes` native fallbacks for the targeted one-off content/filter effect set.

Coverage moved from the committed baseline:

```text
recipes=144 nativePasses=109 fallbacks=35 hardErrors=0
topUnsupported=content.slideShift:1, filter.bracketEmphasis:1, filter.dotIndicator:1, filter.edgeGrow:1, filter.hoverBar:1, filter.matrixRain:1, filter.subPixelBar:1, filter.underlineWipe:1
```

to:

```text
recipes=144 nativePasses=117 fallbacks=27 hardErrors=0
topUnsupported=filter.vignette:1, mask.blinds:1, mask.cellular:1, mask.diamond:1, mask.dissolve:1, mask.iris:1, mask.none:1, mask.pathReveal:1
```

That is **8 additional strict-native passes** with zero hard errors.

## Recipes moved to strict native

- `content/content_slide_shift.json`
- `filters/filter_bracket_emphasis.json`
- `filters/filter_dot_indicator.json`
- `filters/filter_edge_grow_left.json`
- `filters/filter_hover_bar.json`
- `filters/filter_matrix_rain_speed_profile.json`
- `filters/filter_sub_pixel_bar.json`
- `filters/filter_underline_wipe.json`

## Model decisions integrated

- `content.slideShift` lowers to a backend-owned source content stage that matches the player primitive row-shift behavior from authored `startCol`/`endCol` and sampled phase progress.
- The seven targeted filters lower to backend-owned source style stages instead of direct compositor `FilterSpec` variants because the current compositor filter contracts do not preserve the player-visible fixture semantics for `applyTo`, float position, width/thickness, period, or matrix-rain channel intent.
- Native-vs-`irResolved` parity is now an executable regression for this packet: strict-native `rows` and `styledCells` must match `irResolved` output for all eight target recipes at `phase_t=0.35`.
- Native backend metadata continues to report `nativeContentStages`, `nativeStyleStages`, and `compositionSpecSummary.contentStages/styleStages` counts.

## Guardrails retained

Strict native lowering still rejects unsupported graph shapes instead of silently dropping semantics:

- unknown authored fields,
- graph outputs that require value propagation not represented by backend stages,
- unsupported scopes for content stages,
- unsupported scopes for native filter style stages.

The CLI regression suite covers both successful target fixtures and unsupported-shape rejection cases.

## Verification evidence

```text
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_renders_compositor_backend_native_one_off_content_filter_blockers_json \
  test_fnc_cli_rejects_native_one_off_content_filter_blocker_unsupported_shapes_json \
  --no-fail-fast
# PASS: 2 tests run, 2 passed
```

```text
./scripts/k222_native_coverage_audit.sh
# PASS: recipes=144 nativePasses=117 fallbacks=27 hardErrors=0
```

```text
cargo fmt --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli -- --check
cargo check -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli
cargo clippy -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --all-targets -- -D warnings
# PASS
```

```text
cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-fail-fast
# PASS: 70 tests run, 70 passed, 0 skipped
```

```text
cargo xtask docs check
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema
cargo doc -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-deps
# PASS: generated docs/API/configschema/rustdoc gates passed.
# Existing unrelated docs warnings remain for GlyphStyle, ScalarFieldGlyph, and Highlighter.
```

## Remaining native blockers

The current audit top list is now mask-heavy plus one remaining filter blocker:

- `filter.vignette:1`
- `mask.blinds:1`
- `mask.cellular:1`
- `mask.diamond:1`
- `mask.dissolve:1`
- `mask.iris:1`
- `mask.none:1`
- `mask.pathReveal:1`

Recommended next work packet: close the remaining representable vignette/mask blockers, starting with effects that already have nearby compositor mask/filter equivalents and keeping strict unsupported diagnostics for any non-isomorphic field shapes.
