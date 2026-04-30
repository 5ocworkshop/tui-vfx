<!-- <FILE>docs/new_kernel/K2_19_COMPOSITOR_BACKEND_RESULTS.md</FILE> - <DESC>K2.19 compositor backend result evidence</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.19 visible playback: compositor backend and studio-control pilot evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.19 results, commands, artifacts, limits, and verification evidence.</CLOG> -->

# K2.19 compositor backend results

## What works

- Added `crates/tui-vfx-player-backend-compositor`, a separate adapter crate that consumes `PlayerRenderIrReport`, lowers it to `OwnedGrid`/`RoleMap`/`SemanticScene`, calls `tui_vfx_compositor::pipeline::render_pipeline_with_spec`, and returns `PlayerRenderBackendOutput`.
- Extended backend output JSON with `recipeId`, `recipePath`, `sample`, `renderHash`, `backendHash`, `nonDefaultStyledCells`, forwarded `warnings`/`errors`, diagnostics, and backend metadata.
- Added `play-backend`, which repeatedly samples a recipe, renders through the selected backend, and repaints ANSI/text frames with a real frame delay by default.
- Adapted the interactive UI loop from the `examples/demo.rs` timing pattern: advance time, draw, then poll for input within a 16 ms frame budget.
- Kept dependency direction clean: player core owns IR and the backend trait; the compositor adapter depends on player/types/compositor; UI consumes backend output and does not construct compositor DTOs.

## Demo hashes

| recipe artifact | backendHash | nonDefaultStyledCells |
| --- | ---: | ---: |
| `baseline.json` | 12012130851011687886 | 0 |
| `filter_tint.json` | 11936587110200637702 | 105 |
| `mask_wipe.json` | 170599261833557358 | 0 |
| `mask_checkers.json` | 5259314696568631964 | 0 |
| `gradient_apply.json` | 8800961556078506138 | 200 |
| `gradient_diagonal.json` | 7375087713630939599 | 210 |
| `border_sweep.json` | 3163813609398527370 | 4 |
| `style_modulo.json` | 9128631022341168192 | 72 |

## Result artifacts

Generated under `/tmp/k219-visual-results/` by `./scripts/k219_visual_demo.sh`:

| result | artifact | pass/fail |
| --- | --- | --- |
| compositor baseline | `/tmp/k219-visual-results/baseline.json` | PASS |
| compositor filter tint | `/tmp/k219-visual-results/filter_tint.json` | PASS |
| compositor mask wipe | `/tmp/k219-visual-results/mask_wipe.json` | PASS |
| compositor mask checkers | `/tmp/k219-visual-results/mask_checkers.json` | PASS |
| compositor gradient apply | `/tmp/k219-visual-results/gradient_apply.json` | PASS |
| compositor gradient diagonal | `/tmp/k219-visual-results/gradient_diagonal.json` | PASS |
| compositor border sweep | `/tmp/k219-visual-results/border_sweep.json` | PASS |
| compositor style modulo | `/tmp/k219-visual-results/style_modulo.json` | PASS |
| ANSI gradient | `/tmp/k219-visual-results/gradient_apply.ansi` | PASS |
| ANSI tint | `/tmp/k219-visual-results/filter_tint.ansi` | PASS |
| ANSI border sweep | `/tmp/k219-visual-results/border_sweep.ansi` | PASS |
| live playback mask wipe | `/tmp/k219-visual-results/live_mask_wipe.play.ansi` | PASS |
| live playback color motion | `/tmp/k219-visual-results/live_color_motion.play.ansi` | PASS |
| live playback gradient color | `/tmp/k219-visual-results/live_gradient.play.ansi` | PASS |
| timeline mask checkers | `/tmp/k219-visual-results/mask_checkers.timeline.json` | PASS |
| timeline mask wipe | `/tmp/k219-visual-results/mask_wipe.timeline.json` | PASS |
| studio border sweep slider | `/tmp/k219-visual-results/studio_border_sweep.studio.json` | PASS |
| studio pill progress | `/tmp/k219-visual-results/studio_pill_button.studio.json` | PASS |
| UI compositor once | `/tmp/k219-visual-results/ui_gradient_once.txt` | PASS |
| artifact README | `/tmp/k219-visual-results/README.md` | PASS |
