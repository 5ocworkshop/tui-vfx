<!-- <FILE>docs/new_kernel/K2_11_SCHEMA_READINESS_LEDGER.md</FILE> - <DESC>K2.11 schema-readiness blocker ledger</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.11 schema-readiness ledger for the v3.1 debug recipe migration path.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record schema-readiness command evidence, blocker groups, and next-resolution lanes.</CLOG> -->

# K2.11 Schema-Readiness Ledger

## Scope

- Pathway: v3.1 debug recipe migration only.
- Legacy evidence root: `../tui-vfx-recipes/recipes/debug_recipes`.
- Canonical v3.1 root: `../tui-vfx-recipes/recipes/v3.1/debug_recipes`.
- Descriptor pack: `descriptors/v3.1/packs/primitive.json`.
- Report schema: `v3.1.player.schemaReadiness.1`.
- Legacy root mutation policy: read-only.

## Command

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

## Direct answer

We **cannot** declare 100% schema readiness for existing debug recipe migration yet.

The v3.1 readiness estimate is **36.0%** with `canDeclareSchemaReady=false`. The blocker ledger is now explicit enough to direct forward progress instead of repeating a generic “unaddressed fields/decisions” answer.

## Summary counts

| Metric | Count |
|---|---:|
| totalLegacyRecords | 603 |
| schemaReadyRecords | 217 |
| schemaBlockedRecords | 72 |
| sourceBlockedRecords | 67 |
| descriptorBlockedRecords | 151 |
| adapterBlockedRecords | 0 |
| fieldCoverageBlockedRecords | 4 |
| ownerAuditRecords | 280 |
| oracleOnlyRecords | 193 |
| duplicateOrVariantRecords | 3 |
| unknownRecords | 5 |
| estimatedSchemaReadinessPercent | 36.0 |
| canDeclareSchemaReady | false |

## Blocker kinds by represented record count

| Blocker kind | Records | Schema-readiness blocking? | Resolution lane |
|---|---:|---|---|
| oracleOnly | 193 | No, after owner signoff | Confirm oracle-only/deprecated disposition. |
| descriptorPack | 151 | No, but blocks migration/render coverage | Descriptor-pack expansion tranches. |
| ownerAudit | 73 | Yes | Normalize complex records into descriptor/source/schema/oracle/backend buckets. |
| sourceDescriptor | 67 | Yes | Source/content descriptor expansion. |
| motionTimingSemantics | 34 | Yes | Easing and motion-route schema semantics. |
| bindingSemantics | 22 | Yes | Binding execution and parameter override semantics. |
| backendRenderer | 14 | No, if explicitly deferred | Future backend/compositor boundary. |
| sceneSemantics | 12 | Yes | Scene/source-local pipeline semantics. |
| unknown | 5 | Yes | Style owner audit. |
| fieldCoverage | 4 | Yes | Primitive descriptor/field-coverage closure. |
| valueSourceSemantics | 3 | Yes | Value-source/sampled-surface semantics. |
| duplicateOrVariant | 3 | No, after owner signoff | Duplicate/variant disposition. |
| lifecycleSemantics | 1 | Yes | Lifecycle/timing semantics. |

## Grouped blocker ledger

| Family | Blocker kind | Records | Blocking? | Representative paths |
|---|---|---:|---|---|
| bindable_rates | bindingSemantics | 8 | Yes | `bindable_rates/glitch_shift_window_bindable.json`; `bindable_rates/marquee_speed_bindable.json`; `bindable_rates/scramble_glitch_shift_bindable.json` |
| complex | oracleOnly | 10 | No | `complex/_DEPRECATED_complex_cellular_faultline.json`; `complex/_DEPRECATED_complex_cinematic_reveal.json`; `complex/_DEPRECATED_complex_content_shader_combo.json` |
| complex | ownerAudit | 73 | Yes | `complex/command_capture_chain.json`; `complex/complex_bounce_filter_native_mix.json`; `complex/complex_cell_motion_shader_pipeline.json` |
| content | oracleOnly | 45 | No | `content/_DEPRECATED_content_dissolve.json`; `content/_DEPRECATED_content_glitch_shift.json`; `content/_DEPRECATED_content_glyph_cascade_braille.json` |
| content | sourceDescriptor | 66 | Yes | `content/content_cell_motion_middle_out.json`; `content/content_cell_motion_root_border_fixed.json`; `content/content_cell_motion_slice.json` |
| easings | motionTimingSemantics | 29 | Yes | `easings/ease_back_in.json`; `easings/ease_back_in_out.json`; `easings/ease_back_out.json` |
| event_driven_dwell | bindingSemantics | 3 | Yes | `event_driven_dwell/bool_binding_truthy_loopback.json`; `event_driven_dwell/integer_binding_demo.json`; `event_driven_dwell/text_binding_demo.json` |
| filters | descriptorPack | 46 | No | `filters/filter_animated_glyph_ramp.json`; `filters/filter_animated_glyph_ramp_gradient.json`; `filters/filter_authoring_ladder_matrix_rain_binding.json` |
| filters | oracleOnly | 45 | No | `filters/_DEPRECATED_filter_bracket_emphasis.json`; `filters/_DEPRECATED_filter_bracket_emphasis_progress_binding.json`; `filters/_DEPRECATED_filter_braille_dust.json` |
| filters | valueSourceSemantics | 3 | Yes | `filters/filter_dim_sample_surface_angle_from.json`; `filters/filter_dim_sample_surface_radius.json`; `filters/filter_dim_sample_surface_radius_from.json` |
| fixtures | sourceDescriptor | 1 | Yes | `fixtures/command_capture_chain.capture.json` |
| loopback | oracleOnly | 3 | No | `loopback/loopback_pill_button_progress_ramp.json`; `loopback/loopback_rigid_shake_severity_ramp.json`; `loopback/loopback_underline_wipe_progress_ramp.json` |
| masks | descriptorPack | 15 | No | `masks/mask_cellular.json`; `masks/mask_center_wipe_fade_modal.json`; `masks/mask_materialize_center.json` |
| masks | duplicateOrVariant | 3 | No | `masks/mask_diamond_square.json`; `masks/mask_iris_square.json`; `masks/mask_radial_square.json` |
| masks | oracleOnly | 15 | No | `masks/_DEPRECATED_mask_blinds.json`; `masks/_DEPRECATED_mask_cellular.json`; `masks/_DEPRECATED_mask_center_wipe_fade_modal.json` |
| motion_routes | motionTimingSemantics | 5 | Yes | `motion_routes/motion_carrier_orbit_helix.json`; `motion_routes/motion_figure_eight_infinity.json`; `motion_routes/scene_layer_follow_lag.json` |
| other | oracleOnly | 1 | No | `_DEPRECATED_baseline.json` |
| samplers | descriptorPack | 5 | No | `samplers/sampler_crt.json`; `samplers/sampler_crt_jitter.json`; `samplers/sampler_faultline.json` |
| samplers | oracleOnly | 6 | No | `samplers/_DEPRECATED_sampler_crt.json`; `samplers/_DEPRECATED_sampler_crt_jitter.json`; `samplers/_DEPRECATED_sampler_faultline.json` |
| scene | bindingSemantics | 6 | Yes | `scene/scene_authoring_ladder_flag_asset_binding.json`; `scene/scene_authoring_ladder_procedural_spinner_binding.json`; `scene/scene_braille_flag_runtime_wave.json` |
| scene | lifecycleSemantics | 1 | Yes | `scene/scene_layer_io_filter_shader.json` |
| scene | sceneSemantics | 12 | Yes | `scene/ansi_source_chain.json`; `scene/scene_authoring_ladder_toast_basic.json`; `scene/scene_braille_flag_asset_token.json` |
| shaders | fieldCoverage | 4 | Yes | `shaders/compositions/shader_border_sweep_position_binding.json`; `shaders/primitives/shader_linear_gradient_apply_to_both.json`; `shaders/primitives/shader_linear_gradient_background_channel.json` |
| shaders | descriptorPack | 72 | No | `shaders/compositions/shader_affordance_wake.json`; `shaders/compositions/shader_affordance_wake_progress_binding.json`; `shaders/compositions/shader_authoring_ladder_focus_row_binding.json` |
| shaders | oracleOnly | 54 | No | `shaders/_DEPRECATED_shader_affordance_wake.json`; `shaders/_DEPRECATED_shader_affordance_wake_progress_binding.json`; `shaders/_DEPRECATED_shader_ambient_occlusion.json` |
| shadows | backendRenderer | 9 | No | `shadows/shadow_bottom_centered_inset.json`; `shadows/shadow_braille_density_texture.json`; `shadows/shadow_full_cell_transparent_offset.json` |
| signals | bindingSemantics | 5 | Yes | `signals/bounded_chaos_noise_signal.json`; `signals/layered_keyframes_drift_signal.json`; `signals/morph_between_two_signals_signal.json` |
| styles | descriptorPack | 13 | No | `styles/style_cell_position_binding.json`; `styles/style_color_shift.json`; `styles/style_fade_in.json` |
| styles | unknown | 5 | Yes | `styles/style_modulo_horizontal_every_third_row.json`; `styles/style_modulo_vertical_every_fourth_column_offset.json`; `styles/style_non_empty_scope.json` |
| styles | oracleOnly | 14 | No | `styles/_DEPRECATED_style_cell_position_binding.json`; `styles/_DEPRECATED_style_color_fade.json`; `styles/_DEPRECATED_style_color_shift.json` |
| subcell_shapes | backendRenderer | 5 | No | `subcell_shapes/braille_rounded_rect_v3.json`; `subcell_shapes/fractional_inset_rect_v3.json`; `subcell_shapes/quadrant_corner_sculpt_v3.json` |

## Minimum next-resolution sequence

1. **Source/content descriptor tranche** — resolves or sharply narrows the 67 `sourceDescriptor` blockers. This should implement/fixture `source.text`, decide descriptor-only versus adapter-backed `source.ansi`, and explicitly classify image, procedural, and offline command-capture artifacts.
2. **Lifecycle/signal/binding/value-source schema packet** — resolves 60 records across motion timing, binding, lifecycle, and sampled-surface value sources. This is the largest true schema/model unblocker.
3. **Complex owner-audit normalization** — splits 73 complex records into descriptor composition, source-local pipeline, command-capture oracle, GUI/human-review, backend, or schema buckets.
4. **Primitive field-coverage closure** — resolves the 4 shader field blockers (`gradient`, `applyTo`, `position`) without pretending unsupported adapter inputs are handled.
5. **Descriptor-pack expansion** — burns down 151 descriptor-only blockers after schema/source semantics are settled enough to avoid adding the wrong vocabulary.

<!-- <FILE>docs/new_kernel/K2_11_SCHEMA_READINESS_LEDGER.md</FILE> - <DESC>K2.11 schema-readiness blocker ledger</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
