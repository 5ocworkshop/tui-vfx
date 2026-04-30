<!-- <FILE>docs/new_kernel/K2_22_NATIVE_COVERAGE_AUDIT.md</FILE> - <DESC>K2.22 native compositor coverage audit</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Native coverage audit: classify every non-deprecated v3.1 debug recipe under source-isolated native compositor playback.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — update coverage after simple filter native lowerer expansion.
0.1.0: INIT — record full debug_recipes native coverage baseline and implementation order.</CLOG> -->

# K2.22 native compositor coverage audit

## Successful audit results

Command:

```bash
cd /usr/projects/tui-vfx && ./scripts/k222_native_coverage_audit.sh
```

Result root: `/tmp/k222-native-coverage-results/`.

Summary:

- 144 non-deprecated v3.1 debug recipes audited.
- 61 recipes currently render through source-isolated native compositor mode without fallback.
- 83 recipes fall back in `auto` mode because at least one graph node is not yet natively lowered.
- 0 recipes hard-error during audit execution.
- Native filter expansion added `filter.invert`, `filter.greyscale`, `filter.fadeToCanvas`, `filter.crt`, and cleanly representable `filter.vignette` cases.
- Top remaining unsupported effect blockers: `content.splitFlap` (5), `content.odometer` (4), `content.cellMotion` (3), `shader.revealWipe` (3).

## Blockers by family

| Family | Recipes blocked |
| --- | ---: |
| content | 33 |
| filter | 12 |
| mask | 14 |
| style | 12 |
| shader | 10 |
| sampler | 8 |

## Artifact map

- `/tmp/k222-native-coverage-results/native_coverage_summary.json`
- `/tmp/k222-native-coverage-results/native_coverage_records.json`
- `/tmp/k222-native-coverage-results/native_coverage_table.txt`
- `/tmp/k222-native-coverage-results/rendered/` — per-recipe backend JSON artifacts.

## Representative commands

Native pass smoke test:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_tint.json --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format json
```

Representative current blockers:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/content/content_dissolve.json --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format json
```

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_pattern_fill.json --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format json
```

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_blinds.json --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format json
```

## Implementation order

The audit converts the broad blocker into concrete implementation lanes:

1. **Content effects** — highest count, 33 blocked recipes. This is the largest public-demo unblocker and should be handled as its own focused packet because content transformations may need a different lowering strategy than compositor filters/masks/samplers.
2. **Filter effects** — reduced from 20 to 12 blocked recipes by the simple-filter native expansion; remaining filter blockers need richer widgets/semantics such as pattern fill, matrix rain, and indicator geometry.
3. **Masks** — 14 blocked recipes.
4. **Styles** — 12 blocked recipes.
5. **Shaders** — 10 blocked recipes.
6. **Samplers** — 8 blocked recipes.

## Remaining limits

This audit intentionally runs unsupported recipes in `auto` mode so the harness can collect full-corpus evidence without stopping at the first unsupported native node. Acceptance for individual native implementations remains stricter: `--composition-mode native --fail-on-fallback` must pass with `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, and `nativeSourceIsolated=true`.

<!-- <FILE>docs/new_kernel/K2_22_NATIVE_COVERAGE_AUDIT.md</FILE> - <DESC>K2.22 native compositor coverage audit</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
