# K2.16 backlog normalization report
## Summary
K2.16 keeps schema readiness separate from implementation readiness. `canDeclareSchemaReady=true` means schema decisions are closed; it does not mean the migration backlog is complete.
## Disposition counts
```json
{
  "acceptedSchema": 169,
  "backendHoldback": 15,
  "descriptorBacklog": 219,
  "duplicateVariant": 3,
  "guiHumanReviewHoldback": 2,
  "oracleOnly": 195
}
```
## Raw mapping counters
```json
{
  "families": 18,
  "records": 603,
  "canonicalExists": 55,
  "candidateReady": 0,
  "descriptorDecisionNeeded": 113,
  "schemaDecisionNeeded": 91,
  "ownerAuditNeeded": 280,
  "adapterDecisionNeeded": 0,
  "sourceDecisionNeeded": 61,
  "blockedByUnsupportedSource": 0,
  "blockedByUnsupportedEffect": 0,
  "blockedByFieldCoverage": 0,
  "blockedByAmbiguousLegacyIntent": 0,
  "duplicateOrVariant": 3,
  "notYetClassified": 0
}
```
## Per-family status counts
| status | family | count |
|---|---:|---:|
| `canonicalExists` | `content` | 6 |
| `canonicalExists` | `event_driven_dwell` | 1 |
| `canonicalExists` | `filters` | 8 |
| `canonicalExists` | `masks` | 11 |
| `canonicalExists` | `other` | 1 |
| `canonicalExists` | `samplers` | 5 |
| `canonicalExists` | `scene` | 2 |
| `canonicalExists` | `shaders` | 14 |
| `canonicalExists` | `styles` | 7 |
| `descriptorDecisionNeeded` | `filters` | 38 |
| `descriptorDecisionNeeded` | `masks` | 12 |
| `descriptorDecisionNeeded` | `samplers` | 2 |
| `descriptorDecisionNeeded` | `shaders` | 48 |
| `descriptorDecisionNeeded` | `styles` | 13 |
| `duplicateOrVariant` | `masks` | 3 |
| `ownerAuditNeeded` | `complex` | 83 |
| `ownerAuditNeeded` | `content` | 45 |
| `ownerAuditNeeded` | `filters` | 45 |
| `ownerAuditNeeded` | `loopback` | 3 |
| `ownerAuditNeeded` | `masks` | 15 |
| `ownerAuditNeeded` | `other` | 1 |
| `ownerAuditNeeded` | `samplers` | 6 |
| `ownerAuditNeeded` | `shaders` | 54 |
| `ownerAuditNeeded` | `shadows` | 9 |
| `ownerAuditNeeded` | `styles` | 14 |
| `ownerAuditNeeded` | `subcell_shapes` | 5 |
| `schemaDecisionNeeded` | `bindable_rates` | 8 |
| `schemaDecisionNeeded` | `easings` | 29 |
| `schemaDecisionNeeded` | `event_driven_dwell` | 3 |
| `schemaDecisionNeeded` | `filters` | 7 |
| `schemaDecisionNeeded` | `motion_routes` | 5 |
| `schemaDecisionNeeded` | `scene` | 17 |
| `schemaDecisionNeeded` | `shaders` | 17 |
| `schemaDecisionNeeded` | `signals` | 5 |
| `sourceDecisionNeeded` | `content` | 60 |
| `sourceDecisionNeeded` | `fixtures` | 1 |

## Top 50 remaining paths by priority
| rank | status | family | legacy path | missing descriptors | missing sources |
|---:|---|---|---|---|---|
| 1 | `schemaDecisionNeeded` | `bindable_rates` | `bindable_rates/glitch_shift_window_bindable.json` | `` | `source.glitchShift` |
| 2 | `schemaDecisionNeeded` | `bindable_rates` | `bindable_rates/marquee_speed_bindable.json` | `` | `source.marqueeText` |
| 3 | `schemaDecisionNeeded` | `bindable_rates` | `bindable_rates/scramble_glitch_shift_bindable.json` | `` | `source.scrambleGlitchShift` |
| 4 | `schemaDecisionNeeded` | `bindable_rates` | `bindable_rates/scramble_resolve_pace_bindable.json` | `` | `source.scramble` |
| 5 | `schemaDecisionNeeded` | `bindable_rates` | `bindable_rates/split_flap_cascade_bindable.json` | `` | `source.splitFlapText` |
| 6 | `schemaDecisionNeeded` | `bindable_rates` | `bindable_rates/split_flap_cycles_bindable.json` | `` | `source.splitFlapText` |
| 7 | `schemaDecisionNeeded` | `bindable_rates` | `bindable_rates/split_flap_speed_bindable.json` | `` | `source.splitFlapText` |
| 8 | `schemaDecisionNeeded` | `bindable_rates` | `bindable_rates/typewriter_speed_variance_bindable.json` | `` | `source.typewriterText` |
| 9 | `schemaDecisionNeeded` | `easings` | `easings/ease_back_in.json` | `` | `` |
| 10 | `schemaDecisionNeeded` | `easings` | `easings/ease_back_in_out.json` | `` | `` |
| 11 | `schemaDecisionNeeded` | `easings` | `easings/ease_back_out.json` | `` | `` |
| 12 | `schemaDecisionNeeded` | `easings` | `easings/ease_bezier.json` | `` | `` |
| 13 | `schemaDecisionNeeded` | `easings` | `easings/ease_bezier_custom.json` | `` | `` |
| 14 | `schemaDecisionNeeded` | `easings` | `easings/ease_bounce_in.json` | `` | `` |
| 15 | `schemaDecisionNeeded` | `easings` | `easings/ease_bounce_in_out.json` | `` | `` |
| 16 | `schemaDecisionNeeded` | `easings` | `easings/ease_bounce_out.json` | `` | `` |
| 17 | `schemaDecisionNeeded` | `easings` | `easings/ease_circ_in.json` | `` | `` |
| 18 | `schemaDecisionNeeded` | `easings` | `easings/ease_circ_in_out.json` | `` | `` |
| 19 | `schemaDecisionNeeded` | `easings` | `easings/ease_circ_out.json` | `` | `` |
| 20 | `schemaDecisionNeeded` | `easings` | `easings/ease_circular_modal.json` | `` | `` |
| 21 | `schemaDecisionNeeded` | `easings` | `easings/ease_cubic_in.json` | `` | `` |
| 22 | `schemaDecisionNeeded` | `easings` | `easings/ease_cubic_in_out.json` | `` | `` |
| 23 | `schemaDecisionNeeded` | `easings` | `easings/ease_cubic_out.json` | `` | `` |
| 24 | `schemaDecisionNeeded` | `easings` | `easings/ease_elastic_in.json` | `` | `` |
| 25 | `schemaDecisionNeeded` | `easings` | `easings/ease_elastic_in_out.json` | `` | `` |
| 26 | `schemaDecisionNeeded` | `easings` | `easings/ease_elastic_out.json` | `` | `` |
| 27 | `schemaDecisionNeeded` | `easings` | `easings/ease_expo_in.json` | `` | `` |
| 28 | `schemaDecisionNeeded` | `easings` | `easings/ease_expo_in_out.json` | `` | `` |
| 29 | `schemaDecisionNeeded` | `easings` | `easings/ease_expo_out.json` | `` | `` |
| 30 | `schemaDecisionNeeded` | `easings` | `easings/ease_expo_snappy.json` | `` | `` |
| 31 | `schemaDecisionNeeded` | `easings` | `easings/ease_linear.json` | `` | `` |
| 32 | `schemaDecisionNeeded` | `easings` | `easings/ease_quad_in.json` | `` | `` |
| 33 | `schemaDecisionNeeded` | `easings` | `easings/ease_quad_in_out.json` | `` | `` |
| 34 | `schemaDecisionNeeded` | `easings` | `easings/ease_quad_out.json` | `` | `` |
| 35 | `schemaDecisionNeeded` | `easings` | `easings/ease_sine_in.json` | `` | `` |
| 36 | `schemaDecisionNeeded` | `easings` | `easings/ease_sine_in_out.json` | `` | `` |
| 37 | `schemaDecisionNeeded` | `easings` | `easings/ease_sine_out.json` | `` | `` |
| 38 | `schemaDecisionNeeded` | `event_driven_dwell` | `event_driven_dwell/bool_binding_truthy_loopback.json` | `` | `` |
| 39 | `schemaDecisionNeeded` | `event_driven_dwell` | `event_driven_dwell/integer_binding_demo.json` | `` | `` |
| 40 | `schemaDecisionNeeded` | `event_driven_dwell` | `event_driven_dwell/text_binding_demo.json` | `` | `` |
| 41 | `schemaDecisionNeeded` | `filters` | `filters/filter_authoring_ladder_matrix_rain_binding.json` | `` | `` |
| 42 | `schemaDecisionNeeded` | `filters` | `filters/filter_dim_sample_surface_angle_from.json` | `` | `` |
| 43 | `schemaDecisionNeeded` | `filters` | `filters/filter_dim_sample_surface_radius.json` | `` | `` |
| 44 | `schemaDecisionNeeded` | `filters` | `filters/filter_dim_sample_surface_radius_from.json` | `` | `` |
| 45 | `schemaDecisionNeeded` | `filters` | `filters/filter_fade_to_canvas.json` | `` | `` |
| 46 | `schemaDecisionNeeded` | `filters` | `filters/filter_matrix_rain.json` | `` | `` |
| 47 | `schemaDecisionNeeded` | `filters` | `filters/filter_pill_button.json` | `` | `` |
| 48 | `schemaDecisionNeeded` | `motion_routes` | `motion_routes/motion_carrier_orbit_helix.json` | `` | `` |
| 49 | `schemaDecisionNeeded` | `motion_routes` | `motion_routes/motion_figure_eight_infinity.json` | `` | `` |
| 50 | `schemaDecisionNeeded` | `motion_routes` | `motion_routes/scene_layer_follow_lag.json` | `` | `` |

## Explicit owner decisions

None. `explicitOwnerDecisionNeeded=0`.

## Candidate-ready paths

None after K2.16 fixture additions; the previous five candidate-ready records now have canonical fixtures.

## Descriptor backlog grouped by descriptor family

### `filter.animatedGlyphRamp` (2)
- `filters/filter_animated_glyph_ramp.json`
- `filters/filter_animated_glyph_ramp_gradient.json`

### `filter.bracketEmphasis` (2)
- `filters/filter_bracket_emphasis.json`
- `filters/filter_bracket_emphasis_progress_binding.json`

### `filter.brailleDust` (1)
- `filters/filter_braille_dust.json`

### `filter.charsetNoise` (1)
- `filters/filter_charset_noise.json`

### `filter.colorBridgedShade` (1)
- `filters/filter_color_bridged_shade.json`

### `filter.dotIndicator` (2)
- `filters/filter_dot_indicator.json`
- `filters/filter_dot_indicator_progress_binding.json`

### `filter.edgeGrow` (2)
- `filters/filter_edge_grow_bottom_binding.json`
- `filters/filter_edge_grow_left.json`

### `filter.glistenSweep` (2)
- `filters/filter_glisten_sweep.json`
- `filters/filter_glisten_sweep_progress_binding.json`

### `filter.glyphStyle` (1)
- `filters/filter_glyph_style.json`

### `filter.hoverBar` (2)
- `filters/filter_hover_bar.json`
- `filters/filter_hover_bar_progress_binding.json`

### `filter.interlaceCurtain` (1)
- `filters/filter_interlace_curtain.json`

### `filter.kittScanner` (3)
- `filters/filter_kitt_scanner.json`
- `filters/filter_kitt_scanner_progress_binding.json`
- `filters/filter_kitt_scanner_vertical.json`

### `filter.motionBlur` (1)
- `filters/filter_motion_blur.json`

### `filter.rigidShake` (3)
- `filters/filter_rigid_shake.json`
- `filters/filter_rigid_shake_damping_scale_binding.json`
- `filters/filter_rigid_shake_num_shakes_binding.json`

### `filter.scalarFieldGlyph` (1)
- `shaders/primitives/shader_terminal_fire_glyph_v3.json`

### `filter.shadeScanner` (2)
- `filters/filter_shade_scanner.json`
- `filters/filter_shade_scanner_progress_binding.json`

### `filter.subCellShake` (1)
- `filters/filter_sub_cell_shake.json`

### `filter.subPixelBar` (2)
- `filters/filter_sub_pixel_bar.json`
- `filters/filter_sub_pixel_bar_progress_binding.json`

### `filter.subcellLight` (1)
- `filters/filter_subcell_light_background_braille.json`

### `filter.subcellLight,shader.concealedLight` (1)
- `filters/filter_subcell_light_foreground_horizontal.json`

### `filter.subcellLight,shader.diffusion` (1)
- `filters/filter_subcell_light_temporal_braille.json`

### `filter.underlineWipe` (2)
- `filters/filter_underline_wipe.json`
- `filters/filter_underline_wipe_progress_binding.json`

### `filter.vignette` (4)
- `filters/filter_vignette.json`
- `filters/filter_vignette_dithered.json`
- `filters/filter_vignette_side_pair.json`
- `filters/filter_vignette_temporal_soften.json`

### `mask.cellular` (1)
- `masks/mask_cellular.json`

### `sampler.crt` (1)
- `samplers/sampler_crt.json`

### `sampler.crtJitter` (1)
- `samplers/sampler_crt_jitter.json`

### `sampler.spatialSignal,style.spatial` (1)
- `shaders/style_field_hint_spatial_shader.json`

### `shader.affordanceWake` (2)
- `shaders/compositions/shader_affordance_wake.json`
- `shaders/compositions/shader_affordance_wake_progress_binding.json`

### `shader.barberPole` (2)
- `shaders/compositions/shader_barber_pole.json`
- `shaders/primitives/shader_region_compression_scope.json`

### `shader.barberPole,shader.fractionalStripeOverlay` (1)
- `shaders/compositions/shader_barber_pole_fractional_third_color.json`

### `shader.bevel` (1)
- `shaders/compositions/shader_bevel.json`

### `shader.chromaticEdge` (1)
- `shaders/primitives/shader_chromatic_edge.json`

### `shader.coloredOverlay` (6)
- `shaders/primitives/shader_ambient_occlusion.json`
- `shaders/primitives/shader_diffusion_breath.json`
- `shaders/primitives/shader_diffusion_center_bg.json`
- `shaders/primitives/shader_diffusion_center_fg.json`
- `shaders/primitives/shader_diffusion_top_left_all.json`
- `shaders/primitives/shader_glow.json`

### `shader.concealedLight` (4)
- `shaders/compositions/shader_concealed_light.json`
- `shaders/compositions/shader_concealed_light_both.json`
- `shaders/compositions/shader_concealed_light_drift.json`
- `shaders/compositions/shader_concealed_light_foreground.json`

### `shader.cursor` (1)
- `shaders/primitives/shader_cursor.json`

### `shader.edgeSheen` (1)
- `shaders/compositions/shader_edge_sheen.json`

### `shader.focusedRowGradient` (3)
- `shaders/compositions/shader_authoring_ladder_focus_row_binding.json`
- `shaders/compositions/shader_focused_row_gradient.json`
- `styles/style_cell_position_binding.json`

### `shader.glitchLines` (1)
- `shaders/primitives/shader_glitch_lines.json`

### `shader.neonFlicker` (1)
- `shaders/primitives/shader_neon_flicker.json`

### `shader.orbit` (1)
- `shaders/primitives/shader_orbit.json`

### `shader.pulseWave` (2)
- `shaders/compositions/shader_pulse_wave.json`
- `shaders/compositions/shader_pulse_wave_frequency_binding.json`

### `shader.radar` (2)
- `shaders/compositions/shader_radar.json`
- `styles/style_spatial_effect.json`

### `shader.radialSpiral` (1)
- `shaders/primitives/shader_radial_spiral_v3.json`

### `shader.reflect` (1)
- `shaders/compositions/shader_reflect.json`

### `shader.stochasticSparkle` (1)
- `shaders/primitives/shader_stochastic_sparkle.json`

### `shader.subCellShake` (1)
- `shaders/primitives/shader_sub_cell_shake.json`

### `shader.terminalFire` (5)
- `shaders/primitives/shader_terminal_fire_campfire_v3.json`
- `shaders/primitives/shader_terminal_fire_candle_v3.json`
- `shaders/primitives/shader_terminal_fire_embers_v3.json`
- `shaders/primitives/shader_terminal_fire_smoke_plume_v3.json`
- `shaders/primitives/shader_terminal_fire_v3.json`

### `shader.terminalWater` (8)
- `shaders/primitives/shader_terminal_water_flow_v3.json`
- `shaders/primitives/shader_terminal_water_glint_v3.json`
- `shaders/primitives/shader_terminal_water_ocean_with_ripples_v3.json`
- `shaders/primitives/shader_terminal_water_rain_v3.json`
- `shaders/primitives/shader_terminal_water_ripple_v3.json`
- `shaders/primitives/shader_terminal_water_v3.json`
- `shaders/primitives/shader_terminal_water_wake_v3.json`
- `shaders/primitives/shader_terminal_water_weather_wake_v3.json`

### `shader.tracePath` (1)
- `shaders/primitives/shader_trace_path.json`

### `shader.tracePropagation` (1)
- `shaders/primitives/shader_trace_propagation.json`

### `style.colorShift` (1)
- `styles/style_color_shift.json`

### `style.fadeIn` (2)
- `styles/style_fade_in.json`
- `styles/style_fade_in_from_canvas.json`

### `style.fadeIn,style.fadeOut` (2)
- `masks/mask_center_wipe_fade_modal.json`
- `masks/mask_wipe_fade_left_right.json`

### `style.fadeOut` (2)
- `styles/style_fade_out.json`
- `styles/style_fade_out_to_canvas.json`

### `style.glitch` (1)
- `styles/style_glitch.json`

### `style.italicWindow` (1)
- `styles/style_italic_window.json`

### `style.neonFlicker` (1)
- `styles/style_neon_flicker.json`

### `style.pulse` (1)
- `styles/style_pulse.json`

### `style.rainbow` (1)
- `styles/style_rainbow.json`

### `style.rigidShakeStyle` (1)
- `styles/style_rigid_shake_style.json`

### `unspecified` (9)
- `masks/mask_materialize_corner.json`
- `masks/mask_wipe_corner_in_to_bottom_left.json`
- `masks/mask_wipe_corner_in_to_bottom_right.json`
- `masks/mask_wipe_corner_in_to_top_left.json`
- `masks/mask_wipe_corner_in_to_top_right.json`
- `masks/mask_wipe_corner_out_from_bottom_left.json`
- `masks/mask_wipe_corner_out_from_bottom_right.json`
- `masks/mask_wipe_corner_out_from_top_left.json`
- `masks/mask_wipe_corner_out_from_top_right.json`

## Source/content backlog grouped by source/content kind

### `source.commandCaptureArtifact` (1)
- `fixtures/command_capture_chain.capture.json`

### `source.dissolve` (1)
- `content/content_dissolve.json`

### `source.glitchShift` (1)
- `content/content_glitch_shift.json`

### `source.glyphCascade` (2)
- `content/content_glyph_cascade_braille.json`
- `content/content_glyph_cascade_into.json`

### `source.mirror` (1)
- `content/content_mirror.json`

### `source.numeric` (1)
- `content/content_numeric.json`

### `source.odometer` (10)
- `content/content_odometer.json`
- `content/content_odometer_3x3_count_bindable.json`
- `content/content_odometer_cell_roll_diagonal.json`
- `content/content_odometer_cell_roll_dispersion_edge_in.json`
- `content/content_odometer_cell_roll_down.json`
- `content/content_odometer_cell_roll_left.json`
- `content/content_odometer_cell_roll_slot_machine.json`
- `content/content_odometer_cell_roll_up.json`
- `content/content_odometer_decimal_preset_carry.json`
- `content/content_odometer_slot_reel.json`

### `source.redact` (1)
- `content/content_redact.json`

### `source.scrambleGlitchShift` (1)
- `content/content_scramble_glitch_shift.json`

### `source.slideShift` (1)
- `content/content_slide_shift.json`

### `source.splitFlapText` (19)
- `content/content_split_flap_ambient_board.json`
- `content/content_split_flap_arrivals_board.json`
- `content/content_split_flap_authentic_timing.json`
- `content/content_split_flap_board_update.json`
- `content/content_split_flap_cycles.json`
- `content/content_split_flap_digits.json`
- `content/content_split_flap_from_message.json`
- `content/content_split_flap_jitter.json`
- `content/content_split_flap_leading_blocks.json`
- `content/content_split_flap_rolling_cards.json`
- `content/content_split_flap_settle_hinge.json`
- `content/content_split_flap_solari_authentic.json`
- `content/content_split_flap_solari_museum.json`
- `content/content_split_flap_spring_settle.json`
- `content/content_split_flap_tile_2row.json`
- `content/content_split_flap_tile_4row.json`
- `content/content_split_flap_tile_6row.json`
- `content/content_split_flap_tile_8row.json`
- `content/content_split_flap_tile_board.json`

### `source.typewriterText` (20)
- `content/content_cell_motion_middle_out.json`
- `content/content_cell_motion_root_border_fixed.json`
- `content/content_cell_motion_slice.json`
- `content/content_typewriter_cursor_braille_2.json`
- `content/content_typewriter_cursor_braille_4.json`
- `content/content_typewriter_cursor_braille_6.json`
- `content/content_typewriter_cursor_braille_8.json`
- `content/content_typewriter_cursor_braille_flip.json`
- `content/content_typewriter_cursor_braille_pulse.json`
- `content/content_typewriter_cursor_caret.json`
- `content/content_typewriter_cursor_full.json`
- `content/content_typewriter_cursor_grow_in_center.json`
- `content/content_typewriter_cursor_grow_in_down.json`
- `content/content_typewriter_cursor_grow_in_up.json`
- `content/content_typewriter_cursor_scan_bounce.json`
- `content/content_typewriter_cursor_scan_pulse.json`
- `content/content_typewriter_cursor_wake_gap.json`
- `content/content_typewriter_cursor_wake_ghost.json`
- `content/content_typewriter_cursor_wake_tint.json`
- `content/content_typewriter_io_filter_shader.json`

### `unspecified` (2)
- `content/content_glyph_particles_base_spray.json`
- `content/content_glyph_particles_options_concurrency.json`

## Holdback paths

Holdbacks remain represented in schema-readiness dispositions (`backendHoldback`, `guiHumanReviewHoldback`, `oracleOnly`) and should not be counted as schema blockers. See `K2_16_HOLDBACK_REGISTER.md`.
