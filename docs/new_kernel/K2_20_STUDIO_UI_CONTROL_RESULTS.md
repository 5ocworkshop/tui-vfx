<!-- <FILE>docs/new_kernel/K2_20_STUDIO_UI_CONTROL_RESULTS.md</FILE> - <DESC>K2.20 generated studio control results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native compositor studio pilot: record generated control panel and before/after hash proof.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture border sweep and pill-button generated control results.</CLOG> -->

# K2.20 studio UI control results

## Successful live control results

The player UI now accepts `--studio` and renders descriptor/recipe-derived signal controls. Script commands can mutate generated controls with `set control=value`.

Evidence:

- `/tmp/k220-native-results/studio_live_border_sweep.txt`
- `/tmp/k220-native-results/studio_live_pill_button.txt`
- `/tmp/k220-native-results/studio_border_sweep_before_after.json`
- `/tmp/k220-native-results/studio_pill_button_before_after.json`

Border sweep result:

- Generated control: `position`
- Signal: `sweepPosition`
- Source: `shader.borderSweep:position`
- Before/after backend hashes changed.
- `changedCells > 0`.
- `fallbackUsed=false` before and after.

Pill button result:

- Generated control: `progress`
- Signal: `pillProgress`
- Source: `filter.pillButton:progress`
- Before/after backend hashes changed.
- `changedCells > 0`.
- `fallbackUsed=false` before and after.

## Studio command

```bash
cargo run -q -p tui-vfx-player-ui -- --descriptor-pack descriptors/v3.1/packs/primitive.json --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json --backend compositor --composition-mode native --fail-on-fallback --studio --script "set position=0.75; render; quit" --no-clear
```

<!-- <FILE>docs/new_kernel/K2_20_STUDIO_UI_CONTROL_RESULTS.md</FILE> - <DESC>K2.20 generated studio control results</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
