<!-- <FILE>docs/new_kernel/PHASE_K2_22_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.22 review and de-slop report</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Native coverage and simple filter expansion: track review, de-slop, and verification evidence.</WCTX> -->
<!-- <CLOG>0.3.0: PATCH — record clean re-review pass after docs correction.
0.2.0: PATCH — close review doc inconsistency on representative filter blocker.
0.1.0: INIT — capture K2.22 audit/filter implementation review status and regression evidence.</CLOG> -->

# K2.22 review and de-slop report

## Successful results

- Full native coverage audit script runs and classifies all 144 non-deprecated v3.1 debug recipes.
- Simple filter native expansion initially increased source-isolated native passes from 47 to 55 and reduced fallbacks from 97 to 89; after K2.23 content typewriter support, the same audit now reports 61 native passes and 83 fallbacks.
- Added strict native/no-fallback/source-only regression coverage for `filter.invert`, `filter.greyscale`, `filter.fadeToCanvas`, cleanly representable `filter.vignette`, and `filter.crt`.
- Non-representable vignette inputs remain unsupported instead of being partially lowered while dropping authored semantics.

## AI de-slop actions

Scoped cleanup stayed inside the K2.22 touched lane:

- Added one table-style regression instead of one test per filter family.
- Kept durable public terms (`sourceRenderMode`, `nativeSourceIsolated`, `fallbackUsed`) rather than transient packet shorthand.
- Kept the coverage harness result artifacts under `/tmp/k222-native-coverage-results` so the repo only carries reproducible script/docs, not generated output.

## Verification evidence

- `cargo fmt --package tui-vfx-player --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli --package tui-vfx-player-ui -- --check` — PASS.
- `cargo check -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui` — PASS.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets -- -D warnings` — PASS.
- `cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast` — PASS, 123/123.
- `cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli test_fnc_cli_renders_compositor_backend_native_simple_filter_families_json --no-fail-fast` — PASS, 1/1.
- `./scripts/k222_native_coverage_audit.sh` — PASS; 144 recipes, 61 native passes, 83 fallbacks, 0 hard errors.
- `git diff --check` — PASS.
- `cargo xtask docs check` — PASS with pre-existing warnings for `filters.GlyphStyle`, `filters.ScalarFieldGlyph`, and `shaders.Highlighter` TOML/ai-hint gaps.
- `cargo xtask docs api-check` — PASS.
- `cargo xtask docs api-validate` — PASS.
- `cargo xtask audit configschema` — PASS.

## Formal review status

Formal scoped review requested one documentation correction: the coverage audit still listed `filter_crt` as a representative current blocker after CRT native lowering was implemented. The command was replaced with `filter_pattern_fill`, which remains a current blocker. Re-review passed with zero issues and confirmed `filter_pattern_fill` fails strict native while `filter_crt` passes with `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, and `nativeSourceIsolated=true`. No implementation blockers remained.

## Remaining risks

- Content remains the dominant blocker family; after `content.typewriter` native support, the top current content blockers are `content.splitFlap`, `content.odometer`, and related content transform families. Completing player/studio public-demo coverage now needs a content-transform native strategy, not more scalar filter work.
- Some filter recipes remain blocked because their semantics are not directly representable by current compositor `FilterSpec` variants.
- Interactive studio UI is still script-backed; the next UI packet should add real focus/navigation/edit controls.

<!-- <FILE>docs/new_kernel/PHASE_K2_22_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.22 review and de-slop report</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
