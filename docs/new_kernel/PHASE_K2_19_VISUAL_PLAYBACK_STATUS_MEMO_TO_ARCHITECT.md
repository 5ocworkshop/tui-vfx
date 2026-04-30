<!-- <FILE>docs/new_kernel/PHASE_K2_19_VISUAL_PLAYBACK_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.19 status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.19 visible playback: compositor backend and studio-control pilot evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.19 results, commands, artifacts, limits, and verification evidence.</CLOG> -->

# Successful visible results

- `./scripts/k219_visual_demo.sh` generated visible compositor backend artifacts under `/tmp/k219-visual-results/` and printed PASS for every required harness step.
- `play-backend --backend compositor --format ansi` now performs actual bounded playback: it repeatedly samples the recipe, renders through the selected backend, clears/repaints the terminal by default, and sleeps according to `--fps` unless CI disables sleeps with `--sample-ms 0`.
- `/tmp/k219-visual-results/live_color_motion.play.ansi` proves multiple sampled compositor-colored frames with changing backend hashes for a debug style recipe.
- `/tmp/k219-visual-results/live_mask_wipe.play.ansi` proves multiple sampled playback frames for a debug mask recipe; `/tmp/k219-visual-results/live_gradient.play.ansi` proves compositor-colored ANSI playback frames for a static gradient smoke recipe.
- The interactive ratatui player loop now follows the timing shape from `/usr/projects/tui-vfx-recipes/examples/demo.rs`: a 16 ms target frame, elapsed-time advance before draw, draw, then input polling for the remaining frame budget.
- The ratatui preview now renders backend styled cells as ratatui styled spans, so `--backend compositor` is not reduced to plain text in the interactive preview.
- `render-backend --backend compositor --format ansi` emits terminal ANSI color for gradient, tint, and border-sweep fixtures.
- `render-backend --backend compositor --format json` emits backend evidence with `backendHash`, `renderHash`, `styledCells`, `nonDefaultStyledCells`, warnings, errors, diagnostics, and metadata.
- `studio-snapshot --backend compositor --set sweep_progress=0.75` changed the border-sweep backend hash from `3163813609398527370` to `2565555993301103739` with `8` changed cells.
- `studio-snapshot --backend compositor --set demo_progress=1.0` changed the pill-button backend hash from `9807443141935699837` to `5986319716946465575` with `144` changed cells.

# User-runnable commands

```bash
./scripts/k219_visual_demo.sh

cargo run -q -p tui-vfx-player-cli -- play-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format ansi \
  --fps 12 \
  --duration-ms 2000 \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_fade_in_from_canvas.json

cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format ansi \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json

cargo run -q -p tui-vfx-player-cli -- studio-snapshot \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json \
  --set sweep_progress=0.75 \
  --json
```

# Studio control results

| recipe | control | signal | beforeBackendHash | afterBackendHash | changedCells |
| --- | --- | --- | ---: | ---: | ---: |
| `shader_border_sweep_position_binding.json` | `sweep_progress` / `position` | `sweepPosition` | 3163813609398527370 | 2565555993301103739 | 8 |
| `filter_pill_button_progress_binding.json` | `demo_progress` / `progress` | `pillProgress` | 9807443141935699837 | 5986319716946465575 | 144 |

# What works end-to-end

- v3.1 debug recipe -> `RecipePlayer` -> `PlayerRenderIrReport` -> compositor backend adapter -> JSON/text/ANSI backend output.
- CLI `play-backend` repeatedly samples that path over time and paints terminal frames; JSON playback output is available for CI evidence, ANSI playback is available for humans.
- UI `--backend compositor --once` consumes backend output and shows compositor backend hash plus ANSI-colored preview text.
- Interactive UI `--backend compositor` advances time continuously and renders styled backend cells through ratatui spans.
- Backend output stays player-owned at the seam; UI does not construct compositor DTOs.

# What does not work yet

- Direct graph/effect lowering into non-empty compositor `CompositionSpec` remains the central blocker. The current compositor adapter honestly consumes already-resolved player styled IR and emits `playerIrAlreadyResolved`. This was enough to produce visible results, but not enough to claim full compositor-native recipe execution.
- Interactive generated studio forms are not done. The pilot is scripted and descriptor/control-catalog derived.
- No visual parity oracle/screenshot comparator is attached yet.

# Verification matrix

| gate | evidence | result |
| --- | --- | --- |
| Targeted new tests | `cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui ... --no-fail-fast` | PASS, 59/59 |
| Demo harness | `./scripts/k219_visual_demo.sh` | PASS table, 20 artifacts including live playback captures |
| Studio hash mutation | `/tmp/k219-visual-results/*.studio.json` | PASS, hashes changed |
| Full workspace regression | `cargo nextest run --workspace --no-fail-fast` | PASS, 2872/2872 |
| Lint/static gate | targeted `cargo clippy ... -D warnings` | PASS |
| Color-motion playback proof | `/tmp/k219-visual-results/live_color_motion.play.ansi` | PASS, 5 unique backend hashes plus truecolor ANSI |
| Timeline pacing evidence | `render-backend-timeline --sample-ms 250 --format json` | PASS, preserves `sampleMs: 250`, phase samples, and hash variation |

# Files/crates touched

- `crates/tui-vfx-player-backend-compositor/` — new compositor backend adapter crate.
- `crates/tui-vfx-player/` — backend output evidence fields.
- `crates/tui-vfx-player-cli/` — backend render/timeline/studio/playback commands.
- `crates/tui-vfx-player-ui/` — backend selector, one-shot compositor output, interactive timed playback loop, and styled ratatui preview.
- `scripts/k219_visual_demo.sh` — reproducible artifact harness.
- `docs/new_kernel/K2_19_*.md`, `docs/VOCABULARY.md`, `docs/v3.1-feature-contract-checklist.md`, `docs/new_kernel/INDEX.md` — impacted docs.

# Review and de-slop results

Recorded in `PHASE_K2_19_REVIEW_AND_DESLOP_REPORT.md`. The main de-slop target was to keep compositor lowering out of player core and UI, with explicit limitation diagnostics instead of overclaiming full effect lowering.

# Recommended next packet

Focus exclusively on removing the real higher-level blocker: implement direct descriptor/effect lowering into `CompositionSpec` for the same bounded demo corpus. Avoid another report-only packet; require before/after artifacts and visible output for each newly lowered effect family.
