<!-- <FILE>docs/new_kernel/K2_19_STUDIO_CONTROL_PILOT_RESULTS.md</FILE> - <DESC>K2.19 studio control pilot results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.19 visible playback: compositor backend and studio-control pilot evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.19 results, commands, artifacts, limits, and verification evidence.</CLOG> -->

# K2.19 studio control pilot results

## What works

- Added `studio-snapshot` to `tui-vfx-player-cli`.
- The command builds the descriptor/recipe-derived control catalog, applies scripted `--set key=value` assignments to signal-backed controls, renders before/after through the selected backend, and reports `beforeBackendHash`, `afterBackendHash`, and `changedCells`.

## Passing studio examples

| recipe | scripted control | signal | beforeBackendHash | afterBackendHash | changedCells | artifact |
| --- | --- | --- | ---: | ---: | ---: | --- |
| `shaders/compositions/shader_border_sweep_position_binding.json` | `sweep_progress=0.75` | `sweepPosition` | 3163813609398527370 | 2565555993301103739 | 8 | `/tmp/k219-visual-results/studio_border_sweep.studio.json` |
| `filters/filter_pill_button_progress_binding.json` | `demo_progress=1.0` | `pillProgress` | 9807443141935699837 | 5986319716946465575 | 144 | `/tmp/k219-visual-results/studio_pill_button.studio.json` |

## Control derivation scope

Controls are still sourced from descriptor/catalog/recipe usage data. The pilot maps user-facing script aliases to recipe signals only at the CLI edge; it does not infer effect behavior from raw recipe internals.
