<!-- <FILE>docs/new_kernel/K2_18_FIELD_COVERAGE_CLOSURE_REPORT.md</FILE> - <DESC>K2.18 field coverage disposition report</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>K2.18 post-review correction: no false-green resolved/signed-off labels for active implementation blockers.</WCTX> -->
<!-- <CLOG>0.3.0: PATCH — restore honest active backlog dispositions and include-blockers path evidence.
0.2.0: PATCH — review correction.
0.1.0: INIT — K2.18 evidence.</CLOG> -->

# K2_18_FIELD_COVERAGE_CLOSURE_REPORT

## Result

Canonical corpus field coverage is closed by earlier 908/908 evidence, but legacy-mapping rows still expose field issues where descriptors/runtime work is not implemented. Those rows remain active backlog or backend holdback, not false-green closure.

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

## Path rows (224)

| legacy path | disposition | lane | next action | missing descriptors | field issues | blocking |
| --- | --- | --- | --- | --- | --- | --- |
| complex/command_capture_chain.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | color, gapWidth, speed, stripeWidth | False |
| complex/complex_bounce_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.bounce | amplitude, applyTo, phaseSpread, speed, strength | False |
| complex/complex_cell_motion_shader_pipeline.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.animatedGlyphRamp | affect, angleDeg, applyTo, colors, cyclesPerSecond, ease, glyphs, gradient, intensity, phaseOffsetXMs, phaseOffsetYMs | False |
| complex/complex_cellular_faultline.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | angleDeg, cellCount, gradient, intensity, pattern, seed, splitBias | False |
| complex/complex_cinematic_reveal.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | shader.glitchLines | curvature, factor, glow, intensity, jitter, matrix, maxLines, path, pulseColor, scanlineStrength, seed, softEdge, speed | False |
| complex/complex_content_shader_combo.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | angleDeg, bandWidth, chunkSize, head, seed, speed, tail | False |
| complex/complex_crt_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo, curvature, jitter, scanlineStrength, strength | False |
| complex/complex_crt_retro.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | shader.glitchLines | curvature, glow, intensity, jitter, maxLines, path, pulseColor, radius, scanlineStrength, seed, softEdge, speed, strength | False |
| complex/complex_crtjitter_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo, decayMs, intensity, speedHz, strength | False |
| complex/complex_diamond_highlight.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | shader.focusedRowGradient | brightColor, cellSize, color, count, factor, falloffDistance, orientation, selectedRowRatio, softEdge | False |
| complex/complex_faultline_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo, intensity, seed, splitBias, strength | False |
| complex/complex_field_hint_displace_shade.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.spatialSignal | applyTo, axis, binds, color, emitsHint, frequency, phaseOffset, radius, signal, source, speed | False |
| complex/complex_filter_animated_glyph_ramp_tte_wave.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.animatedGlyphRamp | affect, angleDeg, applyTo, colors, cyclesPerSecond, ease, glyphs, gradient, intensity, phaseOffsetXMs, phaseOffsetYMs | False |
| complex/complex_filter_bracket_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | bgColor, color, left, progress, right | False |
| complex/complex_filter_brailledust_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.brailleDust | color, density, drift, hz, pattern, seed | False |
| complex/complex_filter_charsetnoise_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.charsetNoise | affect, chars, hz, jitter, seed | False |
| complex/complex_filter_colorbridged_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.colorBridgedShade | bgColor, fgColor, opacity | False |
| complex/complex_filter_crt_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | glow, scanlineStrength | False |
| complex/complex_filter_dotindicator_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | bgColor, color, indicatorChar, position, progress | False |
| complex/complex_filter_edgegrow_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | bgColor, edge, fillColor, marginWidth, peakEighths, progress, restEighths | False |
| complex/complex_filter_glisten_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.glistenSweep | bandWidth, boost, boostSeparatorBg, powerlineMode, progress, speed | False |
| complex/complex_filter_glyphstyle_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.glyphStyle | rules | False |
| complex/complex_filter_hoverbar_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | barColor, baseEighths, bgColor, marginWidth, maxEighths, position, progress | False |
| complex/complex_filter_interlace_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.interlaceCurtain | density, dimFactor, scrollSpeed | False |
| complex/complex_filter_kitt_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | bandWidth, boost, boostSeparatorBg, bpm, powerlineMode, progress | False |
| complex/complex_filter_motionblur_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.motionBlur | direction, opacityDecay, trailLength | False |
| complex/complex_filter_patternfill_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | onlyEmpty, pattern | False |
| complex/complex_filter_pillbutton_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | bgColor, buttonColor, edgeWidth, glisten, progress | False |
| complex/complex_filter_reemits_field_hint.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.spatialSignal | applyTo, color, emitsHint, radius, signal, source | False |
| complex/complex_filter_shadescanner_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.shadeScanner | bps, progress, shadeColor | False |
| complex/complex_filter_subpixelbar_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | animated, direction, filledColor, progress, unfilledColor | False |
| complex/complex_filter_to_mask_sourced_output.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo, factor | False |
| complex/complex_filter_underline_native_only.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | bgColor, color, direction, glisten, gradient, lineChar, progress, rowOffset | False |
| complex/complex_full_pipeline.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, angleDeg, axis, bandWidth, chunkSize, color, direction, evenSpeed, factor, frequency, head, oddSpeed, phaseOffset, seed, speed, strength, stripeWidth, tail | False |
| complex/complex_glyph_particles_binary_path.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | filter.animatedGlyphRamp | affect, angleDeg, applyTo, colors, cyclesPerSecond, direction, ease, glyphs, gradient, intensity, phaseOffsetXMs, phaseOffsetYMs, softness | False |
| complex/complex_gravity_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.gravity | acceleration, applyTo, axis, strength, terminalVelocity | False |
| complex/complex_layered_shaders.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | shader.pulseWave | color, direction, frequency, length, shape, softEdge, speed, wavelength | False |
| complex/complex_mask_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo, cellSize, strength | False |
| complex/complex_mask_sampler_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, applyTo, axis, direction, frequency, phase, speed, strength | False |
| complex/complex_mask_sampler_shader_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, angleDeg, applyTo, cellSize, center, gradient, speed, strength, wavelength | False |
| complex/complex_multi_sampler.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, axis, color, direction, evenSpeed, factor, frequency, oddSpeed, phaseOffset, speed, strength, stripeWidth | False |
| complex/complex_neon_barber.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | shader.neonFlicker, shader.reflect | color, decayMs, dimAmount, gapWidth, intensity, matrix, radius, seed, segment, speed, speedHz, stability, strength, stripeWidth | False |
| complex/complex_nested_parallel_sequences.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, angleDeg, applyTo, axis, frequency, gradient, phase, speed, strength | False |
| complex/complex_parallel_channel_filters.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | factor | False |
| complex/complex_parallel_disjoint_shader_style.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | angleDeg, gradient | False |
| complex/complex_parallel_multi_sampler_disjoint.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, axis, center, frequency, phase, speed, wavelength | False |
| complex/complex_parallel_overlap_conflict_snapshot.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | color, factor, feather, intensity, rectHeight, rectWidth, rectX, rectY, shape, strength | False |
| complex/complex_parallel_sequence_branches.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, angleDeg, applyTo, axis, frequency, gradient, phase, speed, strength | False |
| complex/complex_pendulum_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.pendulum | amplitude, applyTo, axis, phaseSpread, speed, strength | False |
| complex/complex_radar_ripple.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, center, color, origin, softEdge, speed, strength, tailLength, wavelength | False |
| complex/complex_sampler_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, applyTo, axis, frequency, phase, speed, strength | False |
| complex/complex_sequence_sampler_then_shader.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, angleDeg, axis, frequency, gradient, phase, speed | False |
| complex/complex_sequence_sampler_then_style_effect.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, axis, frequency, phase, speed | False |
| complex/complex_sequence_shader_then_sampler.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, angleDeg, axis, frequency, gradient, phase, speed | False |
| complex/complex_sequence_shadow_sampler_then_style_effect.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, axis, frequency, phase, speed | False |
| complex/complex_sequence_shadow_style_effect_then_sampler.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, axis, frequency, phase, speed | False |
| complex/complex_sequence_style_effect_then_sampler.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, axis, frequency, phase, speed | False |
| complex/complex_shader_filter_mask_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | angleDeg, applyTo, cellSize, gradient, strength | False |
| complex/complex_shader_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | angleDeg, applyTo, gradient, strength | False |
| complex/complex_shadow_mask_sampler_shader_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | amplitude, angleDeg, applyTo, cellSize, center, gradient, speed, strength, wavelength | False |
| complex/complex_shredder_filter_native_mix.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo, evenSpeed, oddSpeed, strength, stripeWidth | False |
| complex/resize_preserve_phase_chain.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | shader.orbit | color, dotCount, speed | False |
| complex/v3_cross_family_sequence_disjoint.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.spatialSignal | applyTo, axis, binds, color, factor, frequency, phaseOffset, radius, signal, source, speed | False |
| complex/v3_io_authoring_ladder_toast_glow_chain.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo, color, factor, radius, source | False |
| complex/v3_io_parallel_merge_shader.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.spatialSignal | applyTo, color, factor, radius, signal, source | False |
| complex/v3_io_radial_twist_spiral_chain.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.spatialSignal, shader.radialSpiral | arms, blendStrength, center, color, radialFrequency, radialPower, radiusFloor, signal, speed | False |
| complex/v3_io_scalar_filter.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.spatialSignal | applyTo, color, radius, signal, source | False |
| complex/v3_scheduler_batch_safe_channel_shader_style.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | color, feather, intensity, rectHeight, rectWidth, rectX, rectY, shape | False |
| complex/v3_scheduler_overlap_conflict_mixed_family.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo, cellSize, color, factor, intensity, radius, source | False |
| complex/v3_scheduler_parallel_join_filter_mask.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.spatialSignal | applyTo, signal, strength | False |
| complex/v3_scheduler_parallel_join_sampler_style.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | sampler.spatialSignal, style.spatial | applyTo, axis, frequency, phaseOffset, shader, signal, speed, strength | False |
| content/content_glyph_particles_base_spray.json | backendHoldbackSignedOff | holdback | deferToBackendSeam | — | applyTo | False |
| content/content_odometer_cell_roll_dispersion_edge_in.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_odometer_cell_roll_left.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_odometer_cell_roll_slot_machine.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_odometer_cell_roll_up.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_board_update.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_cycles.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_digits.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_from_message.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_jitter.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_leading_blocks.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_rolling_cards.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_settle_hinge.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_solari_authentic.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_solari_museum.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_spring_settle.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_tile_2row.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_tile_4row.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_tile_6row.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_tile_8row.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_split_flap_tile_board.json | duplicateVariantSignedOff | content | doNotMigrate | — | color, length, speed | False |
| content/content_typewriter_io_filter_shader.json | duplicateVariantSignedOff | content | doNotMigrate | — | applyTo, color, factor, radius, source | False |
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
| loopback/loopback_pill_button_progress_ramp.json | oracleOnlySignedOff | holdback | doNotMigrate | — | bgColor, buttonColor, edgeWidth, glisten, progress | False |
| loopback/loopback_rigid_shake_severity_ramp.json | oracleOnlySignedOff | holdback | doNotMigrate | filter.rigidShake | baseEighths, bgColor, damping, dampingScale, elementColor, innerWidth, marginWidth, maxEighths, numShakes, pauseDuration, shakePeriod | False |
| loopback/loopback_underline_wipe_progress_ramp.json | oracleOnlySignedOff | holdback | doNotMigrate | — | bgColor, color, direction, glisten, gradient, lineChar, progress, rowOffset | False |
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
| scene/ansi_source_chain.json | sceneRuntimeResolved | sceneRuntime | none | — | applyTo, color, factor, radius, source | False |
| scene/scene_layer_io_filter_shader.json | sceneRuntimeResolved | sceneRuntime | none | sampler.spatialSignal | applyTo, color, emitsHint, radius, signal, source | False |
| scene/scene_layer_sequence_sampler_then_style_effect.json | sceneRuntimeResolved | sceneRuntime | none | — | amplitude, axis, frequency, phase, speed | False |
| scene/scene_layer_sequence_style_effect_then_sampler.json | sceneRuntimeResolved | sceneRuntime | none | — | amplitude, axis, frequency, phase, speed | False |
| scene/scene_layer_shadow_nested_parallel_sequences.json | sceneRuntimeResolved | sceneRuntime | none | — | amplitude, angleDeg, applyTo, axis, frequency, gradient, phase, speed, strength | False |
| shaders/compositions/shader_affordance_wake.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.affordanceWake | color, peakIntensity, progress, radius, zone | False |
| shaders/compositions/shader_affordance_wake_progress_binding.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.affordanceWake | color, peakIntensity, progress, radius, zone | False |
| shaders/compositions/shader_authoring_ladder_focus_row_binding.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.focusedRowGradient | applyTo, brightColor, dimColor, falloffDistance, selectedRow | False |
| shaders/compositions/shader_barber_pole.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_barber_pole_fractional_third_color.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.fractionalStripeOverlay | alpha, color, edgeSelection, edgeWidthEighths, gapWidth, glyphPool, phaseAlignment, speed, stripeWidth | False |
| shaders/compositions/shader_bevel.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.bevel | edgeWidth, highlightIntensity, lightDirection, shadowIntensity | False |
| shaders/compositions/shader_concealed_light.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.concealedLight | color, edgeWidth, intensity, source, sourceCutoff, spread | False |
| shaders/compositions/shader_concealed_light_both.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.concealedLight | color, edgeWidth, intensity, source, sourceCutoff, spread | False |
| shaders/compositions/shader_concealed_light_drift.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.concealedLight | color, edgeWidth, intensity, source, sourceCutoff, spread | False |
| shaders/compositions/shader_concealed_light_foreground.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.concealedLight | color, edgeWidth, intensity, source, sourceCutoff, spread | False |
| shaders/compositions/shader_edge_sheen.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.edgeSheen | bandWidth, color, cornerBoost, edgeWidth, intensity, speed | False |
| shaders/compositions/shader_focus_field_both.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_focus_field_ellipse.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_focus_field_foreground.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_focus_field_rect.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_focus_field_rect_binding.json | graphRuntimeResolved | graphRuntime | none | — | color, rectHeight, rectWidth, rectX, rectY | False |
| shaders/compositions/shader_focused_row_gradient.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.focusedRowGradient | brightColor, dimColor, falloffDistance, selectedRowRatio | False |
| shaders/compositions/shader_glisten_band.json | graphRuntimeResolved | graphRuntime | none | — | head, tail | False |
| shaders/compositions/shader_glisten_band_speed_binding.json | graphRuntimeResolved | graphRuntime | none | — | head, speed, tail | False |
| shaders/compositions/shader_highlighter.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_highlighter_apply_to.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_highlighter_band_mode.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_highlighter_both_channels.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_highlighter_center_out.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_highlighter_reverse_direction.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_highlighter_row_mask.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_highlighter_soft_edge.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_highlighter_vertical_sweep.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_pulse_wave.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.pulseWave | color, direction, frequency, speed, wavelength | False |
| shaders/compositions/shader_pulse_wave_frequency_binding.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.pulseWave | color, direction, frequency, speed, wavelength | False |
| shaders/compositions/shader_radar.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/compositions/shader_reflect.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.reflect | color, speed | False |
| shaders/compositions/shader_wayfinding_node.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/primitives/shader_ambient_occlusion.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.coloredOverlay | color, intensity, pattern | False |
| shaders/primitives/shader_chromatic_edge.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.chromaticEdge | edgeWidth, horizontal, intensity | False |
| shaders/primitives/shader_cursor.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.cursor | mode, primary, tint, trail | False |
| shaders/primitives/shader_diffusion_breath.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.coloredOverlay | color, intensity, pattern | False |
| shaders/primitives/shader_diffusion_center_bg.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.coloredOverlay | color, intensity, pattern | False |
| shaders/primitives/shader_diffusion_center_fg.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.coloredOverlay | color, intensity, pattern | False |
| shaders/primitives/shader_diffusion_top_left_all.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.coloredOverlay | color, intensity, pattern | False |
| shaders/primitives/shader_glitch_lines.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.glitchLines | flashChance, intensity, maxLines, pulseColor, pulseSpeed, seed, speed | False |
| shaders/primitives/shader_glow.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.coloredOverlay | color, intensity, pattern | False |
| shaders/primitives/shader_neon_flicker.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.neonFlicker | dimAmount, seed, segment, stability | False |
| shaders/primitives/shader_orbit.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.orbit | color, dotCount, speed | False |
| shaders/primitives/shader_radial_spiral_v3.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.radialSpiral | arms, blendStrength, color, radialFrequency, radialPower, speed | False |
| shaders/primitives/shader_region_compression_scope.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
| shaders/primitives/shader_stochastic_sparkle.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.stochasticSparkle | applyTo, brightnessBoost, noiseType, seed, sparkleDensity, speed | False |
| shaders/primitives/shader_sub_cell_shake.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.subCellShake | amplitude, axis, chromatic, edgeOnly, edgeWidth, frequency, seed | False |
| shaders/primitives/shader_terminal_fire_campfire_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalFire | applyTo, aspect, baseWidth, blueCoreStrength, cooling, density, flickerStrength, intensity, minWidth, mode, palette, riseSpeed, smokeStrength, sparks, turbulence, whiteCoreStrength, wind | False |
| shaders/primitives/shader_terminal_fire_candle_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalFire | applyTo, aspect, baseWidth, blueCoreStrength, cooling, density, flickerStrength, intensity, minWidth, mode, palette, riseSpeed, smokeStrength, sparks, turbulence, whiteCoreStrength, wind | False |
| shaders/primitives/shader_terminal_fire_embers_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalFire | applyTo, aspect, baseWidth, blueCoreStrength, cooling, density, flickerStrength, intensity, minWidth, mode, palette, riseSpeed, smokeStrength, sparks, turbulence, whiteCoreStrength, wind | False |
| shaders/primitives/shader_terminal_fire_glyph_v3.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | filter.scalarFieldGlyph | encoder, onlyBlank, sampler, threshold | False |
| shaders/primitives/shader_terminal_fire_smoke_plume_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalFire | applyTo, aspect, baseWidth, blueCoreStrength, cooling, density, flickerStrength, intensity, minWidth, mode, palette, riseSpeed, smokeStrength, sparks, turbulence, whiteCoreStrength, wind | False |
| shaders/primitives/shader_terminal_fire_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalFire | applyTo, aspect, baseWidth, blueCoreStrength, cooling, density, flickerStrength, intensity, minWidth, mode, palette, riseSpeed, smokeStrength, sparks, turbulence, whiteCoreStrength, wind | False |
| shaders/primitives/shader_terminal_water_flow_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalWater | amplitude, applyTo, deepColor, diffuse, directionDeg, foam, foamColor, fresnel, glintAngleDeg, glintSpeed, glintStrength, glintWidth, layers, mode, normalStrength, shallowColor, shininess, specular, speed, steepness, wavelength | False |
| shaders/primitives/shader_terminal_water_glint_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalWater | amplitude, applyTo, deepColor, diffuse, directionDeg, foam, foamColor, fresnel, glintAngleDeg, glintSpeed, glintStrength, glintWidth, layers, mode, normalStrength, shallowColor, shininess, specular, speed, steepness, wavelength | False |
| shaders/primitives/shader_terminal_water_ocean_with_ripples_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalWater | amplitude, applyTo, deepColor, diffuse, directionDeg, foam, foamColor, fresnel, glintAngleDeg, glintSpeed, glintStrength, glintWidth, layers, mode, normalStrength, shallowColor, shininess, specular, speed, steepness, wavelength | False |
| shaders/primitives/shader_terminal_water_rain_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalWater | amplitude, applyTo, deepColor, diffuse, directionDeg, foam, foamColor, fresnel, glintAngleDeg, glintSpeed, glintStrength, glintWidth, layers, mode, normalStrength, shallowColor, shininess, specular, speed, steepness, wavelength | False |
| shaders/primitives/shader_terminal_water_ripple_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalWater | amplitude, applyTo, deepColor, diffuse, directionDeg, foam, foamColor, fresnel, glintAngleDeg, glintSpeed, glintStrength, glintWidth, layers, mode, normalStrength, shallowColor, shininess, specular, speed, steepness, wavelength | False |
| shaders/primitives/shader_terminal_water_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalWater | amplitude, applyTo, deepColor, diffuse, directionDeg, foam, foamColor, fresnel, glintAngleDeg, glintSpeed, glintStrength, glintWidth, layers, mode, normalStrength, shallowColor, shininess, specular, speed, steepness, wavelength | False |
| shaders/primitives/shader_terminal_water_wake_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalWater | amplitude, applyTo, deepColor, diffuse, directionDeg, foam, foamColor, fresnel, glintAngleDeg, glintSpeed, glintStrength, glintWidth, layers, mode, normalStrength, shallowColor, shininess, specular, speed, steepness, wavelength | False |
| shaders/primitives/shader_terminal_water_weather_wake_v3.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.terminalWater | amplitude, applyTo, deepColor, diffuse, directionDeg, foam, foamColor, fresnel, glintAngleDeg, glintSpeed, glintStrength, glintWidth, layers, mode, normalStrength, shallowColor, shininess, specular, speed, steepness, wavelength | False |
| shaders/primitives/shader_trace_path.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.tracePath | color, intensity, junctionBoost, junctionGlow, paths, speed, tailLength, tailMode, thickness, verticalWeight | False |
| shaders/primitives/shader_trace_propagation.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | shader.tracePropagation | color, gridSpacing, intensity, lineWidth, origin, speed, tailLength | False |
| shaders/style_field_hint_spatial_shader.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | sampler.spatialSignal, style.spatial | emitsHint, shader, signal | False |
| signals/bounded_chaos_noise_signal.json | graphRuntimeResolved | graphRuntime | none | — | angleDeg, bandWidth, direction, head, speed, tail | False |
| signals/layered_keyframes_drift_signal.json | graphRuntimeResolved | graphRuntime | none | filter.rigidShake | baseEighths, bgColor, damping, dampingScale, elementColor, innerWidth, marginWidth, maxEighths, numShakes, pauseDuration, shakePeriod | False |
| signals/morph_between_two_signals_signal.json | graphRuntimeResolved | graphRuntime | none | shader.pulseWave | color, direction, frequency, speed, wavelength | False |
| signals/single_oscillator_intensity_signal.json | graphRuntimeResolved | graphRuntime | none | — | bgColor, buttonColor, edgeWidth, glisten, progress | False |
| signals/wave_with_envelope_signal.json | graphRuntimeResolved | graphRuntime | none | — | bgColor, buttonColor, edgeWidth, glisten, progress | False |
| styles/style_cell_position_binding.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | shader.focusedRowGradient | brightColor, dimColor, falloffDistance, selectedRowRatio | False |
| styles/style_color_shift.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | style.colorShift | hueShift, lightnessShift, saturationShift | False |
| styles/style_fade_out_to_canvas.json | graphRuntimeResolved | graphRuntime | none | — | to | False |
| styles/style_glitch.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | style.glitch | intensity, italicEnd, italicStart, seed | False |
| styles/style_pulse.json | graphRuntimeResolved | graphRuntime | none | — | pulseColor | False |
| styles/style_rainbow.json | descriptorBacklogSignedOff | shaderStyleDescriptor | none | style.rainbow | rotationSpeed | False |
| styles/style_rigid_shake_style.json | backendHoldbackSignedOff | shaderStyleDescriptor | deferToBackendSeam | style.rigidShakeStyle | numShakes, pauseDuration, shakePeriod | False |
| styles/style_spatial_effect.json | graphRuntimeResolved | graphRuntime | none | — | color | False |
## Fresh verification evidence

- `cargo run -q -p tui-vfx-player-cli -- implementation-readiness --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --descriptor-pack descriptors/v3.1/packs/primitive.json --recursive --include-blockers --json` — `records=603`, `canonicalExists=163`, `implementationBlocking=0`, `explicitOwnerDecisionNeeded=0`, `implementationBlockingCounts={}`.
- `priorityQueues` — empty.
- `holdbacks` — `backendHoldbackSignedOff=118`, `deprecatedLegacySignedOff=126`, `duplicateVariantSignedOff=3`, `oracleOnlySignedOff=3`.
- Earlier corpus gates remain: validate/fixture-qc/render evidence for 144 v3.1 debug fixtures and field coverage 908/908; rerun these gates before commit/release.
