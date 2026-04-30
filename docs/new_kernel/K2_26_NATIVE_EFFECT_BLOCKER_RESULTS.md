<!-- <FILE>docs/new_kernel/K2_26_NATIVE_EFFECT_BLOCKER_RESULTS.md</FILE> - <DESC>v3.1 native effect blocker closure results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Results for the native shader/filter/mask/sampler blocker closure packet.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record strict-native closure results, counters, model decisions, and remaining blockers.</CLOG> -->

# Native effect blocker closure results

## Result

The packet closed the representable non-deprecated v3.1 `debug_recipes` native fallbacks for the targeted shader/filter/mask/sampler effect set.

Coverage moved from the committed baseline:

```text
recipes=144 nativePasses=81 fallbacks=63 hardErrors=0
```

to:

```text
recipes=144 nativePasses=99 fallbacks=45 hardErrors=0
topUnsupported=style.moduloColumns:2, style.neonFlicker:2, content.dissolve:1, content.glitchShift:1, content.mirror:1, content.numeric:1, content.redact:1, content.scrambleGlitchShift:1
```

That is **18 additional strict-native passes** with zero hard errors.

## Recipes moved to strict native

- `shaders/primitives/shader_reveal_wipe.json`
- `shaders/primitives/shader_reveal_wipe_corner_out_top_left.json`
- `shaders/primitives/shader_reveal_wipe_right_to_left.json`
- `filters/filter_kitt_scanner.json`
- `filters/filter_kitt_scanner_vertical.json`
- `filters/filter_pattern_fill.json`
- `filters/filter_pattern_fill_density_anchors.json`
- `masks/mask_materialize_center.json`
- `masks/mask_materialize_corner.json`
- `masks/mask_materialize_progress.json`
- `masks/mask_noise_dither.json`
- `masks/mask_noise_dither_seed_profile.json`
- `samplers/sampler_faultline.json`
- `samplers/sampler_faultline_offset_positive.json`
- `samplers/sampler_radial_twist_strength_extremes.json`
- `samplers/sampler_radial_twist_v3.json`
- `samplers/sampler_shredder.json`
- `samplers/sampler_shredder_slice_width_stride.json`

## Model decisions integrated

- `shader.revealWipe` now carries authored color into `RevealWipeShader` instead of dropping it.
- `filter.kittScanner` now carries authored scan/trail colors, cell-sized scanner width, cadence, axis, powerline mode, and separator background color through the compositor `FilterSpec` and runtime filter.
- `filter.patternFill` now carries pattern families used by fixtures plus deterministic density gating.
- `mask.materialize` and `mask.materializeCorner` lower through compositor materialization using origin, seed, chunk size, noise, and soft edge.
- `mask.noiseDither` now carries chunk size through `MaskSpec`, prepared masks, and the runtime dither threshold.
- `sampler.faultLine` and `sampler.shredder` now carry fixed offsets for exact fixture lowering while preserving the existing animated defaults when offsets are absent.
- `sampler.radialTwist` now lowers authored strength into the compositor radial twist sampler.

## Guardrails retained

Strict native lowering still rejects unsupported graph shapes instead of silently dropping semantics:

- unknown authored fields,
- graph outputs that require value propagation not represented by the compositor backend slice,
- non-`all` scopes that this strict-native tranche does not yet represent.

The CLI regression suite covers both the successful exact fixture set and unsupported-shape rejection cases.

## Verification evidence

```text
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_renders_compositor_backend_native_exact_effect_blocker_subset_json \
  test_fnc_cli_rejects_native_exact_effect_blocker_subset_unsupported_shapes_json \
  --no-fail-fast
# PASS: 2 tests run, 2 passed
```

```text
./scripts/k222_native_coverage_audit.sh
# PASS: recipes=144 nativePasses=99 fallbacks=45 hardErrors=0
```

```text
cargo fmt --package tui-vfx-style --package tui-vfx-compositor \
  --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli -- --check
cargo clippy -p tui-vfx-style -p tui-vfx-compositor \
  -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --all-targets -- -D warnings
cargo nextest run -p tui-vfx-style -p tui-vfx-compositor \
  -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-fail-fast
# PASS: 1339 tests run, 1339 passed, 0 skipped
```

```text
cargo xtask docs generate
cargo xtask docs check
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema
# PASS: generated docs up to date; API docs valid; configschema audit clean
# Existing unrelated docs warnings remain for GlyphStyle, ScalarFieldGlyph, and Highlighter.
```

## Remaining native blockers

The next blockers are no longer in this packet's exact shader/filter/mask/sampler set. The current audit top list is:

- `style.moduloColumns:2`
- `style.neonFlicker:2`
- `content.dissolve:1`
- `content.glitchShift:1`
- `content.mirror:1`
- `content.numeric:1`
- `content.redact:1`
- `content.scrambleGlitchShift:1`

Recommended next work packet: close the two style blockers first, then continue the content-transform burn-down by deciding compositor-native source-stage semantics for dissolve/glitch/mirror/numeric/redact variants.
