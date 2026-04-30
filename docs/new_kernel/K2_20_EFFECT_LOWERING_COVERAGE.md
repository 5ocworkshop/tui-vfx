<!-- <FILE>docs/new_kernel/K2_20_EFFECT_LOWERING_COVERAGE.md</FILE> - <DESC>K2.20 native effect lowering coverage</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native compositor lowering: document supported bounded effect families and honest warnings.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record native lowerer effect coverage and holdbacks.</CLOG> -->

# K2.20 effect lowering coverage

## Native lowerers implemented

| Effect id | Native compositor target | Evidence artifact |
|---|---|---|
| `filter.tint` | `CompositionSpec.filters` / `FilterSpec::Tint` | `native_filter_tint.json` |
| `filter.dim` | `CompositionSpec.filters` / `FilterSpec::Dim` | `native_filter_dim.json` |
| `filter.pillButton` | `CompositionSpec.filters` / signal-driven `FilterSpec::Tint` | `native_pill_button.json` |
| `mask.wipe` | `CompositionSpec.masks` / `MaskSpec::Wipe` | `native_mask_wipe.json` |
| `mask.checkers` | `CompositionSpec.masks` / `MaskSpec::Checkers` | `native_mask_checkers.json` |
| `sampler.sineWave` | ordered sampler chain / `SamplerSpec::SineWave` | `native_sampler_sinewave.json` |
| `sampler.ripple` | ordered sampler chain / `SamplerSpec::Ripple` | `native_sampler_ripple.json` |
| `shader.linearGradient` | `CompositionSpec.shader_layers` / `SpatialShaderType::LinearGradient` | `native_linear_gradient_apply_to_both.json`, `native_linear_gradient_diagonal.json` |
| `shader.borderSweep` | `CompositionSpec.shader_layers` / `SpatialShaderType::BorderSweep` | `native_border_sweep.json` |
| `style.fadeIn` | progress-derived `FilterSpec::Tint` | `native_style_fade_in.json` |
| `style.fadeOut` | progress-derived `FilterSpec::Tint` | `native_style_fade_out.json` |

## Honest diagnostics

The native lowerer emits `fieldIgnoredWithWarning` when a node-level policy or easing field is acknowledged but not reinterpreted inside the native compositor adapter. It emits `unsupportedNativeEffect` for effects outside the bounded native set, and `requiresIrFallback` in auto mode when fallback is used.

Baseline recipes with no graph nodes render in native mode without fallback, but they do not count as lowered effect coverage or non-empty `CompositionSpec` coverage.

<!-- <FILE>docs/new_kernel/K2_20_EFFECT_LOWERING_COVERAGE.md</FILE> - <DESC>K2.20 native effect lowering coverage</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
