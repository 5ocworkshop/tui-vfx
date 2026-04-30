<!-- <FILE>docs/new_kernel/K2_20_NATIVE_COMPOSITION_RESULTS.md</FILE> - <DESC>K2.20 native compositor result evidence</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native compositor lowering: record successful native render artifacts and commands.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.20 native compositor harness results.</CLOG> -->

# K2.20 native composition results

## Successful native results

Command:

```bash
./scripts/k220_native_compositor_demo.sh
```

Result root: `/tmp/k220-native-results/`.

Summary from `/tmp/k220-native-results/native_summary.json`:

- Native recipe renders: 13
- Native renders with non-empty `CompositionSpec`: 12
- Fallback count: 0
- Native lowering succeeded count: 13
- Lowered effect families: filter, mask, sampler, shader, style

## Required artifacts

- `/tmp/k220-native-results/native_summary.json`
- `/tmp/k220-native-results/native_pass_fail_table.txt`
- `/tmp/k220-native-results/native_linear_gradient.ansi`
- `/tmp/k220-native-results/native_border_sweep_timeline.json`
- `/tmp/k220-native-results/native_mask_wipe_timeline.json`
- `/tmp/k220-native-results/native_filter_tint.json`
- `/tmp/k220-native-results/studio_live_border_sweep.txt`
- `/tmp/k220-native-results/studio_live_pill_button.txt`
- `/tmp/k220-native-results/studio_before_after.json`

## Pass/fail table

See `/tmp/k220-native-results/native_pass_fail_table.txt`. All required K2.20 recipes plus the pill-button studio control fixture passed native mode with `fallbackUsed=false`.

<!-- <FILE>docs/new_kernel/K2_20_NATIVE_COMPOSITION_RESULTS.md</FILE> - <DESC>K2.20 native compositor result evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
