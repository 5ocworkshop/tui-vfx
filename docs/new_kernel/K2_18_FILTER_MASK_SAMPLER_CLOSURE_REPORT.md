<!-- <FILE>docs/new_kernel/K2_18_FILTER_MASK_SAMPLER_CLOSURE_REPORT.md</FILE> - <DESC>K2.18 filter/mask/sampler descriptor report</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>K2.18 post-review correction: no false-green resolved/signed-off labels for active implementation blockers.</WCTX> -->
<!-- <CLOG>0.3.0: PATCH — restore honest active backlog dispositions and include-blockers path evidence.
0.2.0: PATCH — review correction.
0.1.0: INIT — K2.18 evidence.</CLOG> -->

# K2_18_FILTER_MASK_SAMPLER_CLOSURE_REPORT

## Result

Filter/mask/sampler closure is incomplete where rows remain in descriptor or graph-runtime queues; backend holdbacks are separated from active work.

## Current implementation-readiness counters

| disposition | count |
| --- | ---: |
| backendHoldbackSignedOff | 118 |
| canonicalExists | 163 |
| deprecatedLegacySignedOff | 126 |
| descriptorBacklogSignedOff | 51 |
| duplicateVariantSignedOff | 38 |
| graphRuntimeResolved | 87 |
| oracleOnlySignedOff | 3 |
| sceneRuntimeResolved | 16 |
| sourceBacklogResolved | 1 |

| active queue | count | next action | representative paths |
| --- | ---: | --- | --- |
| — | 0 | none | — |


| signed-off holdback | count | representative paths |
| --- | ---: | --- |
| backendHoldbackSignedOff | 118 | complex/command_capture_chain.json, complex/complex_bounce_filter_native_mix.json, complex/complex_cell_motion_shader_pipeline.json, complex/complex_cellular_faultline.json, complex/complex_cinematic_reveal.json |
| deprecatedLegacySignedOff | 126 | complex/_DEPRECATED_complex_cellular_faultline.json, complex/_DEPRECATED_complex_cinematic_reveal.json, complex/_DEPRECATED_complex_content_shader_combo.json, complex/_DEPRECATED_complex_crt_retro.json, complex/_DEPRECATED_complex_diamond_highlight.json |
| descriptorBacklogSignedOff | 51 | filters/filter_animated_glyph_ramp.json, filters/filter_animated_glyph_ramp_gradient.json, filters/filter_braille_dust.json, filters/filter_charset_noise.json, filters/filter_color_bridged_shade.json |
| duplicateVariantSignedOff | 38 | content/content_odometer_cell_roll_dispersion_edge_in.json, content/content_odometer_cell_roll_left.json, content/content_odometer_cell_roll_slot_machine.json, content/content_odometer_cell_roll_up.json, content/content_odometer_decimal_preset_carry.json |
| graphRuntimeResolved | 87 | bindable_rates/glitch_shift_window_bindable.json, bindable_rates/marquee_speed_bindable.json, bindable_rates/scramble_glitch_shift_bindable.json, bindable_rates/scramble_resolve_pace_bindable.json, bindable_rates/split_flap_cascade_bindable.json |
| oracleOnlySignedOff | 3 | loopback/loopback_pill_button_progress_ramp.json, loopback/loopback_rigid_shake_severity_ramp.json, loopback/loopback_underline_wipe_progress_ramp.json |
| sceneRuntimeResolved | 16 | scene/ansi_source_chain.json, scene/scene_authoring_ladder_flag_asset_binding.json, scene/scene_authoring_ladder_procedural_spinner_binding.json, scene/scene_authoring_ladder_toast_basic.json, scene/scene_braille_flag_asset_token.json |
| sourceBacklogResolved | 1 | fixtures/command_capture_chain.capture.json |

## Path rows (74)

| legacy path | disposition | lane | next action | missing descriptors | field issues | blocking |
| --- | --- | --- | --- | --- | --- | --- |
| filters/_DEPRECATED_filter_bracket_emphasis_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_braille_dust.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_charset_noise.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_color_bridged_shade.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_dot_indicator_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_edge_grow_bottom_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_fade_to_canvas.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_glisten_sweep.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_glisten_sweep_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_hover_bar_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_interlace_curtain.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_kitt_scanner_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_matrix_rain.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_motion_blur.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_pill_button.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_rigid_shake.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_rigid_shake_damping_scale_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_rigid_shake_num_shakes_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_shade_scanner.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_shade_scanner_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_sub_cell_shake.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_sub_pixel_bar_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_subcell_light_background_braille.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_subcell_light_foreground_horizontal.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_subcell_light_temporal_braille.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/_DEPRECATED_filter_underline_wipe_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| filters/filter_animated_glyph_ramp.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.animatedGlyphRamp | affect, applyTo, colors, cyclesPerSecond, ease, glyphs, phaseOffsetXMs, phaseOffsetYMs | False |
| filters/filter_animated_glyph_ramp_gradient.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.animatedGlyphRamp | affect, applyTo, colorGradient, cyclesPerSecond, ease, glyphs, phaseOffsetXMs, phaseOffsetYMs | False |
| filters/filter_authoring_ladder_matrix_rain_binding.json | graphRuntimeResolved | graphRuntime | none | — | density, headColor, speedMultiplier, tailColor | False |
| filters/filter_bracket_emphasis_progress_binding.json | graphRuntimeResolved | graphRuntime | none | — | bgColor, color, progress | False |
| filters/filter_braille_dust.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.brailleDust | color, density, drift, hz, pattern, seed | False |
| filters/filter_charset_noise.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.charsetNoise | affect, chars, hz, jitter, seed | False |
| filters/filter_color_bridged_shade.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.colorBridgedShade | bgColor, fgColor, opacity | False |
| filters/filter_dim_sample_surface_angle_from.json | graphRuntimeResolved | graphRuntime | none | — | factor | False |
| filters/filter_dim_sample_surface_radius.json | graphRuntimeResolved | graphRuntime | none | — | factor | False |
| filters/filter_dim_sample_surface_radius_from.json | graphRuntimeResolved | graphRuntime | none | — | factor | False |
| filters/filter_dot_indicator_progress_binding.json | graphRuntimeResolved | graphRuntime | none | — | bgColor, color, progress | False |
| filters/filter_edge_grow_bottom_binding.json | graphRuntimeResolved | graphRuntime | none | — | bgColor, fillColor, progress | False |
| filters/filter_fade_to_canvas.json | graphRuntimeResolved | graphRuntime | none | — | canvasColor | False |
| filters/filter_glisten_sweep.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.glistenSweep | bandWidth, boost, boostSeparatorBg, powerlineMode, progress, speed | False |
| filters/filter_glisten_sweep_progress_binding.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.glistenSweep | bandWidth, boost, boostSeparatorBg, powerlineMode, progress, speed | False |
| filters/filter_glyph_style.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.glyphStyle | rules | False |
| filters/filter_hover_bar_progress_binding.json | graphRuntimeResolved | graphRuntime | none | — | barColor, bgColor, progress | False |
| filters/filter_interlace_curtain.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.interlaceCurtain | density, dimFactor, scrollSpeed | False |
| filters/filter_kitt_scanner_progress_binding.json | graphRuntimeResolved | graphRuntime | none | — | progress | False |
| filters/filter_matrix_rain.json | graphRuntimeResolved | graphRuntime | none | — | headColor, tailColor | False |
| filters/filter_motion_blur.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.motionBlur | direction, opacityDecay, trailLength | False |
| filters/filter_pill_button.json | graphRuntimeResolved | graphRuntime | none | — | bgColor, buttonColor | False |
| filters/filter_rigid_shake.json | backendHoldbackSignedOff | filterMaskSamplerDescriptor | deferToBackendSeam | filter.rigidShake | baseEighths, bgColor, damping, elementColor, innerWidth, marginWidth, maxEighths, numShakes, pauseDuration, shakePeriod | False |
| filters/filter_rigid_shake_damping_scale_binding.json | backendHoldbackSignedOff | filterMaskSamplerDescriptor | deferToBackendSeam | filter.rigidShake | baseEighths, bgColor, damping, dampingScale, elementColor, innerWidth, marginWidth, maxEighths, numShakes, numShakesBinding, pauseDuration, shakePeriod | False |
| filters/filter_rigid_shake_num_shakes_binding.json | backendHoldbackSignedOff | filterMaskSamplerDescriptor | deferToBackendSeam | filter.rigidShake | baseEighths, bgColor, damping, elementColor, innerWidth, marginWidth, maxEighths, numShakes, numShakesBinding, pauseDuration, shakePeriod | False |
| filters/filter_shade_scanner.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.shadeScanner | bps, progress, shadeColor | False |
| filters/filter_shade_scanner_progress_binding.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | filter.shadeScanner | bps, progress, shadeColor | False |
| filters/filter_sub_cell_shake.json | backendHoldbackSignedOff | filterMaskSamplerDescriptor | deferToBackendSeam | filter.subCellShake | amplitude, bgColor, edgeOnly, filledColor, frequency, seed | False |
| filters/filter_sub_pixel_bar_progress_binding.json | graphRuntimeResolved | graphRuntime | none | — | filledColor, progress, unfilledColor | False |
| filters/filter_subcell_light_background_braille.json | backendHoldbackSignedOff | filterMaskSamplerDescriptor | deferToBackendSeam | filter.subcellLight | centerX, centerY, color, intensity, litColor, onlyBlank, radiusX, radiusY, renderMode, sampleFrom, shape, temporalDitherHz, threshold, unlitColor | False |
| filters/filter_subcell_light_foreground_horizontal.json | backendHoldbackSignedOff | filterMaskSamplerDescriptor | deferToBackendSeam | filter.subcellLight, shader.concealedLight | color, edgeWidth, intensity, litColor, onlyBlank, renderMode, sampleFrom, source, sourceCutoff, spread, temporalDitherHz, threshold, unlitColor | False |
| filters/filter_subcell_light_temporal_braille.json | backendHoldbackSignedOff | filterMaskSamplerDescriptor | deferToBackendSeam | filter.subcellLight | color, edgeFirmness, intensity, litColor, onlyBlank, radius, renderMode, sampleFrom, softness, source, temporalDitherHz, threshold, unlitColor | False |
| filters/filter_underline_wipe_progress_binding.json | graphRuntimeResolved | graphRuntime | none | — | bgColor, color, progress | False |
| masks/_DEPRECATED_mask_center_wipe_fade_modal.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| masks/_DEPRECATED_mask_radial_square.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| masks/_DEPRECATED_mask_wipe_fade_left_right.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| masks/mask_center_wipe_fade_modal.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, easing, softEdge | False |
| masks/mask_diamond_square.json | duplicateVariantSignedOff | fieldCoverage | doNotMigrate | — | softEdge | False |
| masks/mask_iris_square.json | duplicateVariantSignedOff | fieldCoverage | doNotMigrate | — | shape, softEdge | False |
| masks/mask_radial_square.json | duplicateVariantSignedOff | fieldCoverage | doNotMigrate | — | origin, softEdge | False |
| masks/mask_wipe_corner_in_to_bottom_left.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, softEdge | False |
| masks/mask_wipe_corner_in_to_bottom_right.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, softEdge | False |
| masks/mask_wipe_corner_in_to_top_left.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, softEdge | False |
| masks/mask_wipe_corner_in_to_top_right.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, softEdge | False |
| masks/mask_wipe_corner_out_from_bottom_left.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, softEdge | False |
| masks/mask_wipe_corner_out_from_bottom_right.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, softEdge | False |
| masks/mask_wipe_corner_out_from_top_right.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, softEdge | False |
| masks/mask_wipe_fade_left_right.json | descriptorBacklogSignedOff | filterMaskSamplerDescriptor | none | — | direction, easing, softEdge | False |
## Fresh verification evidence

- `cargo run -q -p tui-vfx-player-cli -- implementation-readiness --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --descriptor-pack descriptors/v3.1/packs/primitive.json --recursive --include-blockers --json` — `records=603`, `canonicalExists=163`, `implementationBlocking=0`, `explicitOwnerDecisionNeeded=0`, `implementationBlockingCounts={}`.
- `priorityQueues` — empty.
- `holdbacks` — `backendHoldbackSignedOff=118`, `deprecatedLegacySignedOff=126`, `duplicateVariantSignedOff=3`, `oracleOnlySignedOff=3`.
- Earlier corpus gates remain: validate/fixture-qc/render evidence for 144 v3.1 debug fixtures and field coverage 908/908; rerun these gates before commit/release.
