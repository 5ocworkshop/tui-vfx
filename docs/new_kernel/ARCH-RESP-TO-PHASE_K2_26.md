<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_26.md</FILE> - <DESC>Self-generated v3.1 native effect blocker closure packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Ralph continuation after K2.25: convert the current top debug_recipes native fallbacks into compositor-native effect lanes.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define the shader/filter/mask/sampler native blocker closure tranche.</CLOG> -->

# Native effect blocker closure packet

## Task statement

Continue the v3.1 player/studio completion loop by turning the current top non-deprecated `debug_recipes` compositor-native fallbacks into strict native passes where existing compositor semantics can represent the authored fields.

This packet follows the successful content-stage closure and targets the highest remaining audit blockers:

- `shader.revealWipe`
- `filter.kittScanner`
- `filter.patternFill`
- `mask.materialize`
- `mask.materializeCorner`
- `mask.noiseDither`
- `sampler.faultLine`
- `sampler.radialTwist`
- `sampler.shredder`

## Desired outcome

- The strict native backend lowers all currently representable fixture shapes for the effects above.
- Unsupported shapes remain explicit fallbacks with actionable diagnostics; do not silently ignore graph outputs, non-all scopes, or fields with no native equivalent.
- `scripts/k222_native_coverage_audit.sh` moves the targeted recipes from fallback to native pass and remains at `hardErrors=0`.
- The player and Studio continue using source-only compositor-native rendering for native mode.
- Documentation and review/de-slop evidence are updated only for impacted files.

## Baseline evidence

Latest committed baseline:

```text
recipes=144 nativePasses=81 fallbacks=63 hardErrors=0
topUnsupported=shader.revealWipe:3, filter.kittScanner:2, filter.patternFill:2, mask.materialize:2, mask.noiseDither:2, sampler.faultLine:2, sampler.radialTwist:2, sampler.shredder:2
```

The explicit blocked recipe set includes:

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

## Constraints

- Scope is v3.1 only.
- Scope is non-deprecated `debug_recipes` only.
- Do not bump the schema version; v3.1 is pre-release and not locked.
- Do not use transient packet shorthand in durable code names or public report vocabulary.
- Use `/usr/projects/tui-vfx-recipes/examples/demo.rs` only as a working GUI/player oracle and compositor usage reference; do not copy its input schema.
- Use nextest for test runs.
- Keep docs, OFPF metadata, rustdocs, and vocabulary synchronized only when the changed files/behavior make them impacted.
- Do not read or provide `steering/ORCHESTRATION.md` to subagents.

## Implementation touchpoints

Primary files:

- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs`
- `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`

Semantic references:

- `crates/tui-vfx-player/src/fnc_apply_shader_primitive.rs`
- `crates/tui-vfx-player/src/fnc_apply_filter_primitive.rs`
- `crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs`
- `crates/tui-vfx-player/src/fnc_apply_distortion_sampler_primitives.rs`
- `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs`
- `crates/tui-vfx-compositor/src/types/cls_mask_spec.rs`
- `crates/tui-vfx-compositor/src/types/cls_sampler_spec.rs`
- `crates/tui-vfx-style/src/models/cls_reveal_wipe_shader.rs`

Verification references:

- `scripts/k222_native_coverage_audit.sh`
- `scripts/k221_source_isolated_native_demo.sh`

## Red / green / refactor plan

1. Add failing strict-native CLI regressions for the target effect set and unsupported-shape guard cases.
2. Run targeted nextest to observe RED.
3. Implement the minimum native lowering surface using existing compositor `FilterSpec`, `MaskSpec`, `SamplerSpec`, and `SpatialShaderType` variants.
4. Run targeted nextest until GREEN.
5. Run the native coverage audit and verify movement from the 81/63/0 baseline.
6. Run format/check/clippy, docs/API gates, formal review, AI de-slop, and post-de-slop regression verification.

## Acceptance criteria

- Targeted strict-native CLI tests pass.
- Native coverage audit shows targeted recipes no longer fall back unless a documented field-shape mismatch remains intentionally unsupported.
- No hard errors are introduced.
- Native metadata remains honest: `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, `nativeSourceIsolated=true` for newly supported recipes.
- Diagnostics remain actionable for unsupported graph outputs, non-all scopes, and unrepresentable fields.
- Results documentation records baseline, final counters, exact blockers closed, and any remaining blockers with a concrete next action.
