<!-- <FILE>docs/new_kernel/K2_19_VISUAL_DEMO_COMMANDS.md</FILE> - <DESC>K2.19 user-runnable visual demo commands</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.19 visible playback: compositor backend and studio-control pilot evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.19 results, commands, artifacts, limits, and verification evidence.</CLOG> -->

# K2.19 visual demo commands

## One-command harness

```bash
./scripts/k219_visual_demo.sh
```

The script writes all artifacts under `/tmp/k219-visual-results/` and prints a pass/fail table.

## Actual in-place playback

Use `play-backend` when you want to see the terminal repaint over time rather than inspect a
single frame or a JSON timeline. The harness captures playback with `--no-clear` and
`--sample-ms 0` so CI can prove multiple frames without sleeping; for human playback, omit both
flags:

```bash
cargo run -q -p tui-vfx-player-cli -- play-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format ansi \
  --fps 12 \
  --duration-ms 2000 \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_fade_in_from_canvas.json
```

For a color-oriented compositor smoke check:

```bash
cargo run -q -p tui-vfx-player-cli -- play-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format ansi \
  --fps 12 \
  --duration-ms 2000 \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json
```

The interactive UI also advances time in a `demo.rs`-style 16 ms frame loop:

```bash
cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json \
  --backend compositor
```

## Representative manual commands

```bash
cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format ansi \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json

cargo run -q -p tui-vfx-player-cli -- render-backend-timeline \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format json \
  --samples 5 \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe.json

cargo run -q -p tui-vfx-player-cli -- studio-snapshot \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json \
  --set sweep_progress=0.75 \
  --json

cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json \
  --backend compositor \
  --once \
  --no-clear
```

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
