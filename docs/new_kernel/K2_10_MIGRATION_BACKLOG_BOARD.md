<!-- <FILE>docs/new_kernel/K2_10_MIGRATION_BACKLOG_BOARD.md</FILE> - <DESC>K2.10 migration backlog board</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>K2.10 corpus-wide migration mapping and backlog board.</WCTX> -->
<!-- <CLOG>0.2.0: PATCH — move sampled-surface filter records behind a value-source decision.
0.1.0: INIT — record corpus-wide migration mapping evidence and next-packet backlog.</CLOG> -->

# K2.10 Migration Backlog Board

## Ready now

No legacy records are ready for canonical fixture creation after value-source-shaped filter inputs were conservatively downgraded.


## Descriptor pack expansion candidates

| Item | Representative legacy paths | Current canonical coverage | Recommended next packet | Blocking decision | Confidence |
|---|---|---|---|---|---|
| Wipe corner/path mask directions | `masks/mask_wipe_corner_out_from_top_left.json`; `masks/mask_path_reveal.json` | Basic `mask.wipe` exists | K2.11 mask descriptor decision | Direction/path vocabulary | High |
| Rich filters | `filters/filter_crt.json`; `filters/filter_vignette_dithered.json`; `filters/filter_matrix_rain.json` | Basic dim/invert/greyscale/tint only | K2.11 filter descriptor expansion | Exact visual semantics and inputs | High |
| Additional samplers | `samplers/sampler_crt.json`; `samplers/sampler_faultline.json`; `samplers/sampler_shredder.json`; `samplers/sampler_radial_twist_v3.json` | Sine/ripple only | K2.11 sampler descriptor decision | Geometry/time semantics | High |
| Style effects and scope variants | `styles/style_rainbow.json`; `styles/style_modulo_horizontal_every_third_row.json`; `styles/style_outer_scope_band.json` | Color fade and role-scope style fixture exist | K2.11 style/scope decision | Scope predicate/modulo/content vocabulary | High |
| Shader compositions | `shaders/compositions/shader_highlighter_center_out.json`; `shaders/compositions/shader_focus_field_ellipse.json`; `shaders/compositions/shader_glisten_band.json` | Border sweep only | K2.11 shader descriptor decision | Composition semantics and runtime bindings | High |

## Player adapter candidates

| Item | Representative legacy paths | Current canonical coverage | Recommended next packet | Blocking decision | Confidence |
|---|---|---|---|---|---|
| Linear-gradient legacy gradient payload handling | `shaders/primitives/shader_linear_gradient_diagonal.json`; `shaders/primitives/shader_linear_gradient_background_channel.json` | `shader.linearGradient` adapter exists | K2.11 field-coverage packet | Whether legacy `gradient` maps to current stops model | Medium |
| Border-sweep position binding | `shaders/compositions/shader_border_sweep_position_binding.json` | `shader.borderSweep` adapter exists | K2.11 binding/field handling | Runtime position binding semantics | Medium |

## Source descriptor candidates

| Item | Representative legacy paths | Current canonical coverage | Recommended next packet | Blocking decision | Confidence |
|---|---|---|---|---|---|
| `source.text` | `scene/scene_layer_surface_base_style.json`; `content/content_cell_motion_slice.json` | `source.card` only | K2.11 source/content descriptor pilot | Text source shape and styling ownership | High |
| `source.ansi` | `scene/ansi_source_chain.json` | Schema has ANSI source kind, no primitive descriptor | K2.11 source/content descriptor pilot | ANSI artifact validation shape | High |
| `source.image` | `scene/scene_image_source_bindable.json` | Schema has image source kind, no primitive descriptor | K2.11 source/content descriptor pilot | Asset resolver seam | High |
| Procedural source candidates | `scene/scene_braille_flag_asset_token.json`; `scene/scene_authoring_ladder_procedural_spinner_binding.json` | None | K2.11 source/content descriptor pilot | Procedural source naming and asset slots | High |
| Offline command capture artifact | `fixtures/command_capture_chain.capture.json` | Schema has command-capture source kind | Keep oracle-only until explicit authoring tool packet | No runtime command execution | High |

## Schema/model decision candidates

| Item | Representative legacy paths | Recommended next packet | Blocking decision | Confidence |
|---|---|---|---|---|
| Filter dim sampled-surface value sources | `filters/filter_dim_sample_surface_angle_from.json`; `filters/filter_dim_sample_surface_radius.json`; `filters/filter_dim_sample_surface_radius_from.json` | K2.11 lifecycle/signal/binding schema decision packet | Value-source/signal semantics for numeric descriptor inputs | High |
| Event dwell remaining demos | `event_driven_dwell/integer_binding_demo.json`; `event_driven_dwell/text_binding_demo.json` | K2.11 lifecycle predicate packet | Trigger predicate and binding source semantics | High |
| Easing and host motion | `easings/*.json`; `motion_routes/motion_figure_eight_infinity.json` | K2.11 lifecycle/signal/binding schema decision packet | Motion descriptor vs effect-local schedule | High |
| Signals and bindable rates | `signals/*_signal.json`; `bindable_rates/marquee_speed_bindable.json` | K2.11 lifecycle/signal/binding schema decision packet | Signal generator/value source/binding execution | High |
| Loopback demos | `loopback/*.json` | Keep oracle-only until demo-layer decision | Loopback must not become canonical runtime data | High |

## GUI/human-review candidates

| Item | Representative legacy paths | Recommended next packet | Blocking decision | Confidence |
|---|---|---|---|---|
| Rich visual oracle inspection | Fire/water shaders, CRT/vignette filters, highlighter/focus shaders | Later GUI manifest/status surface | Need fixture-QC/mapping status display, not compositor wiring | Medium |

## Compositor-backend candidates

| Item | Representative legacy paths | Recommended next packet | Blocking decision | Confidence |
|---|---|---|---|---|
| Shadows | `shadows/shadow_full_cell_transparent_offset.json`; `shadows/shadow_gradient_soft_layers.json` | Later backend adapter decision | Shadow descriptor and backend renderer | High |
| Subcell shapes | `subcell_shapes/fractional_inset_rect_v3.json`; `subcell_shapes/braille_rounded_rect_v3.json` | Later backend adapter decision | Subcell descriptor and renderer | High |

## Owner-audit / oracle-only

- Deprecated legacy recipes.
- Complex full-pipeline and native-only visual recipes.
- Loopback demos until a demo-layer policy exists.
- Command-capture artifacts until offline authoring import/export is designed.

## Duplicate variants

- `masks/mask_diamond_square.json`
- `masks/mask_iris_square.json`
- `masks/mask_radial_square.json`

<!-- <FILE>docs/new_kernel/K2_10_MIGRATION_BACKLOG_BOARD.md</FILE> - <DESC>K2.10 migration backlog board</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
