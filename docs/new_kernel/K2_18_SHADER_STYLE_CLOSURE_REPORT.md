<!-- <FILE>docs/new_kernel/K2_18_SHADER_STYLE_CLOSURE_REPORT.md</FILE> - <DESC>K2.18 shader/style descriptor report</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>K2.18 post-review correction: no false-green resolved/signed-off labels for active implementation blockers.</WCTX> -->
<!-- <CLOG>0.3.0: PATCH — restore honest active backlog dispositions and include-blockers path evidence.
0.2.0: PATCH — review correction.
0.1.0: INIT — K2.18 evidence.</CLOG> -->

# K2_18_SHADER_STYLE_CLOSURE_REPORT

## Result

Shader/style closure is incomplete where rows remain in descriptor or graph-runtime queues; backend holdbacks are separated from active work.

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

## Path rows (135)

| legacy path | disposition | lane | next action | missing descriptors | field issues | blocking |
| --- | --- | --- | --- | --- | --- | --- |
| shaders/_DEPRECATED_shader_affordance_wake.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_affordance_wake_progress_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_ambient_occlusion.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_barber_pole.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_bevel.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_border_sweep.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_border_sweep_position_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_chromatic_edge.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_concealed_light.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_concealed_light_both.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_concealed_light_drift.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_concealed_light_foreground.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_cursor.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_diffusion.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_diffusion_both.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_diffusion_breath.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_diffusion_foreground.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_edge_sheen.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_focus_field_both.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_focus_field_center_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_focus_field_ellipse.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_focus_field_foreground.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_focus_field_rect.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_focus_field_rect_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_focused_row_gradient.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_glisten_band.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_glisten_band_direction_blend_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_glisten_band_speed_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_glitch_lines.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_glow.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_apply_to.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_band_mode.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_both_channels.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_center_out.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_reverse_direction.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_row_mask.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_runtime_bindings.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_soft_edge.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_highlighter_vertical_sweep.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_linear_gradient.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_neon_flicker.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_orbit.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_pulse_wave.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_pulse_wave_frequency_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_radar.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_reflect.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_reveal_wipe.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_stochastic_sparkle.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_sub_cell_shake.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_trace_path.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_trace_propagation.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_wayfinding_node.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| shaders/_DEPRECATED_shader_wayfinding_node_current_index_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
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
| styles/_DEPRECATED_style_cell_position_binding.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| styles/_DEPRECATED_style_color_shift.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| styles/_DEPRECATED_style_fade_out_to_canvas.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| styles/_DEPRECATED_style_glitch.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| styles/_DEPRECATED_style_pulse.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| styles/_DEPRECATED_style_rainbow.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| styles/_DEPRECATED_style_rigid_shake_style.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
| styles/_DEPRECATED_style_spatial_effect.json | deprecatedLegacySignedOff | holdback | doNotMigrate | — | — | False |
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
