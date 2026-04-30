<!-- <FILE>docs/new_kernel/K2_22_SIMPLE_FILTER_NATIVE_RESULTS.md</FILE> - <DESC>K2.22 simple filter native lowerer results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native lowerer expansion: record source-isolated compositor support for straightforward filter families.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture simple filter native lowerer expansion, coverage movement, and verification evidence.</CLOG> -->

# K2.22 simple filter native results

## Successful results

Native lowerer support now includes these additional filter families:

- `filter.invert`
- `filter.greyscale`
- `filter.fadeToCanvas`
- `filter.crt`
- `filter.vignette` when authored inputs map cleanly to compositor `FilterSpec::Vignette`

The full coverage audit improved from 47 native passes / 97 fallbacks to 61 native passes / 83 fallbacks across 144 non-deprecated v3.1 debug recipes.

Command:

```bash
cd /usr/projects/tui-vfx && ./scripts/k222_native_coverage_audit.sh
```

Current summary:

```text
recipes=144 nativePasses=61 fallbacks=83 hardErrors=0
```

## No-silent-fallback behavior

Non-representable `filter.vignette` fixtures remain unsupported instead of silently dropping semantics. For example, authored color/progress aliases such as `edgeColor` and `applyTo` do not have a direct native `FilterSpec::Vignette` equivalent in the current compositor surface, so those recipes still return `unsupportedNativeEffect` under strict native mode.

## Regression evidence

- `test_fnc_cli_renders_compositor_backend_native_simple_filter_families_json` proves strict native/no-fallback/source-only evidence for invert, greyscale, fade-to-canvas, clean vignette, and CRT.
- Targeted command:

```bash
cd /usr/projects/tui-vfx && cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli test_fnc_cli_renders_compositor_backend_native_simple_filter_families_json --no-fail-fast
```

Result: PASS, 1/1.

## Remaining effect blockers

Top remaining blockers after the simple-filter pass:

- - `content.splitFlap` — 5 recipes
- `content.odometer` — 4 recipes
- `content.cellMotion` — 3 recipes
- `shader.revealWipe` — 3 recipes

The next high-impact implementation lane is content transformation lowering or a native content-transform strategy, because content accounts for 33 blocked debug recipes.

<!-- <FILE>docs/new_kernel/K2_22_SIMPLE_FILTER_NATIVE_RESULTS.md</FILE> - <DESC>K2.22 simple filter native lowerer results</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
