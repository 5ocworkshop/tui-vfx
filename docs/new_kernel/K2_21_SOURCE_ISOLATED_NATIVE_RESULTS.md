<!-- <FILE>docs/new_kernel/K2_21_SOURCE_ISOLATED_NATIVE_RESULTS.md</FILE> - <DESC>K2.21 source-isolated native compositor results</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Source-isolated native playback: record user-visible commands, artifacts, and remaining limits.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — record boolean, enum, integer, color, and source-control studio evidence.
0.1.0: INIT — capture source-only native substrate, timeline, and descriptor studio evidence.</CLOG> -->

# K2.21 source-isolated native compositor results

## Successful results

Harness:

```bash
cd /usr/projects/tui-vfx && ./scripts/k221_source_isolated_native_demo.sh
```

Result root: `/tmp/k221-source-native-results/`.

Summary:

- 5 native compositor renders proved `sourceRenderMode=sourceOnly` and `nativeSourceIsolated=true`.
- 0 native fallbacks occurred under `--fail-on-fallback`.
- `irResolved` compatibility still reports `sourceRenderMode=postEffectIr` and `playerIrAlreadyResolved`.
- 3 native timelines changed backend hashes: `mask.wipe`, `sampler.sineWave`, and `shader.borderSweep`.
- Studio evidence now covers number, color, integer, boolean, and enum/control-family mutations. Number, color, integer, and boolean changed output; the enum case is accepted with an explicit no-visual-change diagnostic for that sampled frame.

## User-runnable commands

Animated native timeline with color:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend-timeline --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format ansi --samples 3 --no-clear
```

Descriptor-driven studio control snapshot:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-ui -- --descriptor-pack descriptors/v3.1/packs/primitive.json --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_pill_button_progress_binding.json --backend compositor --composition-mode native --fail-on-fallback --studio --script 'set effect:filter.pillButton:effectNode:activeColor=#ff0000; render; quit' --no-clear
```

## Artifact map

- `/tmp/k221-source-native-results/source_isolation_summary.json`
- `/tmp/k221-source-native-results/native_pass_fail_table.txt`
- `/tmp/k221-source-native-results/native_timeline_hashes.json`
- `/tmp/k221-source-native-results/studio_control_mutations.json`
- `/tmp/k221-source-native-results/studio_source_integer_mutation.json`
- `/tmp/k221-source-native-results/studio_boolean_mutation.json`
- `/tmp/k221-source-native-results/studio_enum_mutation_no_visual_change.json`
- `/tmp/k221-source-native-results/studio_descriptor_control_ui.txt`
- `/tmp/k221-source-native-results/native_border_sweep_timeline.ansi`

## Remaining limits

- Source-only IR is now available for native/auto compositor requests, but source-local element pipeline semantics still need a wider audit before claiming full public-demo completeness.
- Descriptor controls now cover recipe-aware effect inputs and source inputs in the UI/CLI surfaces. Rich gradient editing remains follow-up work beyond the bounded text/color/number/integer/boolean/enum evidence here.
- Native lowerer coverage is still bounded to the currently mapped compositor-native effect families.

<!-- <FILE>docs/new_kernel/K2_21_SOURCE_ISOLATED_NATIVE_RESULTS.md</FILE> - <DESC>K2.21 source-isolated native compositor results</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
