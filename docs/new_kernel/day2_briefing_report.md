Recon Summary

K0/K1 are already a clean contract-native path, but the rendered artifact is still a `Vec<String>` text-grid smoke surface. The shortest route to real visuals is not to revive the old `tui-vfx-recipes` runtime; it is to add a K0-owned adapter that lowers canonical `RecipeDocument` scene/source/graph data into `tui_vfx_types::{OwnedGrid, RoleMap, SemanticScene}` plus `tui_vfx_compositor::pipeline::CompositionSpec`, then reuse `render_pipeline_with_spec`. K1 should continue to consume K0 reports/state and only add a ratatui cell blit for human inspection.

Evidence commands used:

- `ofpf-load --root /usr/projects/tui-vfx --port 3355`
- `ofpf-load --root /usr/projects/tui-vfx-recipes --port 3355`
- `ofpf-orientation --root /usr/projects/tui-vfx --port 3355 --top 3`
- `cargo run -q -p tui-vfx-player-cli -- render-recipe --recursive --descriptor-pack descriptors/v3.1/packs/primitive.json /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes`
- Narrow `ofpf-read`, `ofpf-inspect`, `ofpf-defs`, `ofpf-content`, and local JSON inventory scripts for the files named below.

## 1. Current K0/K1 architecture

### K0 ownership

| Concern | Owner paths | Symbols |
|---|---|---|
| Descriptor loading | `crates/tui-vfx-player/src/fnc_load_descriptor_catalog.rs` | `load_descriptor_catalog`, `LoadedDescriptorCatalog`, `DescriptorPackReport` |
| Recipe file loading | `crates/tui-vfx-player/src/fnc_render_recipe_file.rs` | `render_recipe_file`, `file_error` |
| Player coordinator | `crates/tui-vfx-player/src/cls_recipe_player.rs` | `RecipePlayer::new`, `RecipePlayer::render_recipe`, `RecipePlayer::error_report` |
| Contract validation | `crates/tui-vfx-player/src/cls_recipe_player.rs`; `crates/tui-vfx-contract/src/cls_recipe_document.rs` | `RecipePlayer::render_recipe` calls `RecipeDocument::validate_with_catalog` |
| Session/lifecycle sample state | `crates/tui-vfx-player/src/cls_player_session.rs` | `PlayerSession::new`, `reset`, `render`, `sample_dwell_trigger` |
| Scene/source text-grid rendering | `crates/tui-vfx-player/src/fnc_render_scene.rs` | `render_scene`, `render_source`, `render_text_source`, `blit_rows` |
| Graph effect application | `crates/tui-vfx-player/src/fnc_apply_graph_effects.rs` | `apply_graph_effects`, `apply_wipe`, `apply_checkers` |
| Frame DTO construction | `crates/tui-vfx-player/src/fnc_build_player_frame.rs`; `crates/tui-vfx-player/src/cls_player_frame.rs`; `crates/tui-vfx-player/src/cls_player_frame_report.rs` | `build_player_frame`, `PlayerFrame`, `PlayerFrameReport::from_frame` |
| Recursive run report | `crates/tui-vfx-player/src/cls_player_run_report.rs`; `crates/tui-vfx-player/src/cls_player_summary.rs` | `PlayerRunReport`, `PlayerSummary` |
| CLI parse/output | `crates/tui-vfx-player-cli/src/fnc_run.rs`; `crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs`; `crates/tui-vfx-player-cli/src/main.rs` | `run`, `print_report`, `parse_cli_options`, `main` |

K0 flow in exact symbols:

```text
fnc_run::run
  -> collect_recipe_paths
  -> load_descriptor_catalog
  -> RecipePlayer::new
  -> render_recipe_file
  -> RecipePlayer::render_recipe
       -> RecipeDocument::validate_with_catalog
       -> render_scene
       -> apply_graph_effects
       -> build_player_frame
       -> PlayerFrameReport::from_frame
  -> print_report
```

### K1 ownership

| Concern | Owner paths | Symbols |
|---|---|---|
| UI state and K0 reuse | `crates/tui-vfx-player-ui/src/cls_player_ui_state.rs` | `PlayerUiState`, `load`, `load_recipe_path`, `apply_command`, private `render` |
| Browser/app state | `crates/tui-vfx-player-ui/src/cls_player_ui_app.rs` | `PlayerUiApp`, `PlayerUiFocus`, `new`, `open_focused_entry`, `refresh_browser`, `browser_root_for` |
| Ratatui rendering | `crates/tui-vfx-player-ui/src/fnc_render_ratatui_ui.rs`; `crates/tui-vfx-player-ui/src/fnc_render_ratatui_help.rs` | `render_ratatui_ui`, `render_status`, `render_body`, `render_browser`, `render_preview`, `render_ratatui_help` |
| Text snapshot mode | `crates/tui-vfx-player-ui/src/fnc_render_ui_snapshot.rs` | `render_ui_snapshot`, `frame_rows`, `help_text` |
| Keyboard/browser navigation | `crates/tui-vfx-player-ui/src/fnc_handle_player_ui_key.rs` | `handle_player_ui_key`, `handle_browser_key`, `handle_preview_key` |
| Scripted testing | `crates/tui-vfx-player-ui/src/fnc_run_script.rs` | `run_script` |
| Terminal loop | `crates/tui-vfx-player-ui/src/fnc_run_interactive.rs`; `crates/tui-vfx-player-ui/src/fnc_run.rs`; `crates/tui-vfx-player-ui/src/main.rs` | `run_interactive`, `run`, `main` |
| Tests | `crates/tui-vfx-player-ui/tests/test_fnc_player_ui.rs` | `test_fnc_ui_binary_renders_baseline_once`, `test_fnc_ui_script_fires_event_dwell_trigger`, `test_fnc_ui_reports_unsupported_effects_visibly`, `test_fnc_ratatui_renderer_draws_without_terminal_io` |

K1 does not implement a second player engine. `PlayerUiState` imports and calls K0 APIs directly:

- `tui_vfx_player::load_descriptor_catalog`
- `tui_vfx_player::RecipePlayer::new`
- `tui_vfx_player::PlayerSession::{new, reset, render}`
- `tui_vfx_player::PlayerSampleRequest`
- `tui_vfx_player::PlayerFrameReport`

The K1-only logic is UI/browser/control rendering. The preview pane currently displays `state.report().rows.join("\n")` in `fnc_render_ratatui_ui.rs::render_preview`.

## 2. Current compositor/probe bridge opportunities

### Already-available compositor output seams

| Path | Symbol | Reuse value |
|---|---|---|
| `crates/tui-vfx-types/src/grid.rs` | `Grid`, `OwnedGrid`, `GridExt` | Framework-agnostic cell grid. `OwnedGrid` is the easiest K0 internal visual frame storage. |
| `crates/tui-vfx-types/src/semantic_scene.rs` | `SemanticScene::{new, from_grid_with_default_role, grid, grid_mut, roles, roles_mut}` | Role-aware destination/source surface expected by compositor APIs. |
| `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs` | `render_pipeline`, `render_pipeline_with_area` | Direct compositor path over `Grid`, `RoleMap`, `SemanticScene`, `CompositionOptions`. |
| `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs` | `render_pipeline_with_spec` | Best adapter target: accepts serializable `CompositionSpec` and calls `render_pipeline`. |
| `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec_area.rs` | `render_pipeline_with_spec_area` | Same as above with `RenderArea`. |
| `crates/tui-vfx-compositor/src/pipeline/cls_composition_spec.rs` | `CompositionSpec` | Existing serializable pipeline DTO with `samplers`, `masks`, `filters`, `shader_layers`, `shadow`, timing. |
| `crates/tui-vfx-probe/src/orc_run_probe.rs` | `run_probe` | Proof of direct-engine path: builds `OwnedGrid`, infers roles, creates `SemanticScene`, calls `render_pipeline_with_spec`, then reads final cells. |
| `crates/tui-vfx-probe/src/cls_probe_scene_spec.rs` | `ProbeSceneSpec` | JSON-friendly wrapper around source/destination/composition for direct-engine probe. |
| `crates/tui-vfx-probe/src/fnc_infer_roles_from_grid.rs` | `infer_roles_from_grid` | Useful role-map bootstrap for K0 source cards/text until producer-supplied roles exist. |

### Ratatui Buffer status

No concrete `impl Grid for ratatui::buffer::Buffer` exists in the repo. The only ratatui adapter reference found is illustrative documentation:

- `README.md:176-197` shows a placeholder `RatatuiAdapter<'a>(&'a mut Buffer)`.
- `crates/tui-vfx/src/lib.rs:142-155` says consumers implement `Grid` for framework buffers.

K1 currently uses ratatui widgets (`Paragraph`) to display K0 string rows, not compositor cells.

### Smallest adapter seam

Smallest seam to replace K0 text-grid rows with compositor-backed cells:

1. Keep `RecipePlayer::render_recipe` and `PlayerSession::render` as K0 authority.
2. Add a K0 visual frame builder that returns `OwnedGrid` or a new serializable cell DTO beside existing `rows`.
3. Convert `source.card`/`source.text` in `fnc_render_scene.rs` into `OwnedGrid + RoleMap` rather than only `Vec<String>`.
4. Add a contract-node-to-`CompositionSpec` adapter for the descriptor ids in `primitive.json`.
5. Call `render_pipeline_with_spec` into `SemanticScene`.
6. Preserve `PlayerFrameReport.rows` for CLI regression; add `cells`/`styledRows` only after K0 tests lock current hashes/statuses.
7. In K1 `render_preview`, blit returned cells into the preview area or render a cell-aware widget instead of `Paragraph::new(rows.join("\n"))`.

Closest existing model: `tui-vfx-probe::run_probe` lines around `build_owned_grid`, `infer_roles_from_grid`, `SemanticScene::from_grid_with_default_role`, `render_pipeline_with_spec`, then `destination_scene.grid().clone()`.

## 3. Current primitive support gap

Descriptor pack: `descriptors/v3.1/packs/primitive.json`.

K0 classification source: `crates/tui-vfx-player/src/fnc_apply_graph_effects.rs::apply_graph_effects`:

- no-op rendered: `filter.dim`, `filter.tint`, `filter.invert`, `filter.greyscale`, `mask.none`, `sampler.sineWave`
- visibly mutates text rows: `mask.wipe`, `mask.checkers`
- unsupported diagnostic: all other effects via `unsupportedEffectAdapter`

| Effect id | K0 status | Current v3.1 fixture coverage |
|---|---|---|
| `filter.dim` | b. no-ops but reports rendered | `filters/filter_dim.json` |
| `filter.greyscale` | b. no-ops but reports rendered | `filters/filter_greyscale.json` |
| `filter.invert` | b. no-ops but reports rendered | `filters/filter_invert.json` |
| `filter.tint` | b. no-ops but reports rendered | `filters/filter_tint.json` |
| `mask.checkers` | a. renders visibly | `masks/mask_checkers.json` |
| `mask.dissolve` | c. reports unsupported | `masks/mask_dissolve.json` |
| `mask.none` | b. no-ops but reports rendered | `masks/mask_none.json` |
| `mask.wipe` | a. renders visibly | `masks/mask_wipe.json` |
| `sampler.ripple` | c. reports unsupported | `samplers/sampler_ripple.json` |
| `sampler.sineWave` | b. no-ops but reports rendered | `samplers/sampler_sinewave.json` |
| `shader.borderSweep` | c. reports unsupported | `shaders/compositions/shader_border_sweep.json` |
| `shader.linearGradient` | c. reports unsupported | `shaders/primitives/shader_linear_gradient.json` |
| `style.baseStyleOverride` | c. reports unsupported | `styles/style_role_scope_border.json` |
| `style.colorFade` | c. reports unsupported | `styles/style_color_fade.json` |

Descriptor-only but absent from current v3.1 fixtures: none for effect ids. The descriptor pack has one source descriptor, `source.card`, and every current v3.1 fixture uses it.

## 4. Current v3.1 debug_recipes coverage

Root: `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`.

K0 recursive smoke result from this pass: `{ total: 16, rendered: 10, unsupported: 6, errors: 0 }`.

| File | Recipe id | Graph node effect ids | Source ids | Current K0/K1 status |
|---|---|---|---|---|
| `baseline.json` | `debugBaseline` | none | `source.card` | rendered |
| `event_driven_dwell/bool_binding_demo.json` | `debugEventDrivenDwellBoolBindingDemo` | none | `source.card` | rendered |
| `filters/filter_dim.json` | `debugFilterDim` | `filter.dim` | `source.card` | rendered; no-op style smoke |
| `filters/filter_greyscale.json` | `debugFilterGreyscale` | `filter.greyscale` | `source.card` | rendered; no-op style smoke |
| `filters/filter_invert.json` | `debugFilterInvert` | `filter.invert` | `source.card` | rendered; no-op style smoke |
| `filters/filter_tint.json` | `debugFilterTint` | `filter.tint` | `source.card` | rendered; no-op style smoke |
| `masks/mask_checkers.json` | `debugMaskCheckers` | `mask.checkers` | `source.card` | rendered; visible text-row mutation |
| `masks/mask_dissolve.json` | `debugMaskDissolve` | `mask.dissolve` | `source.card` | unsupported |
| `masks/mask_none.json` | `debugMaskNone` | `mask.none` | `source.card` | rendered; no-op |
| `masks/mask_wipe.json` | `debugMaskWipe` | `mask.wipe` | `source.card` | rendered; visible text-row mutation |
| `samplers/sampler_ripple.json` | `debugSamplerRipple` | `sampler.ripple` | `source.card` | unsupported |
| `samplers/sampler_sinewave.json` | `debugSamplerSinewave` | `sampler.sineWave` | `source.card` | rendered; no-op smoke |
| `shaders/compositions/shader_border_sweep.json` | `debugShaderBorderSweep` | `shader.borderSweep` | `source.card` | unsupported |
| `shaders/primitives/shader_linear_gradient.json` | `debugShaderLinearGradient` | `shader.linearGradient` | `source.card` | unsupported |
| `styles/style_color_fade.json` | `debugStyleColorFade` | `style.colorFade` | `source.card` | unsupported |
| `styles/style_role_scope_border.json` | `debugStyleRoleScopeBorder` | `style.baseStyleOverride` | `source.card` | unsupported |

No current v3.1 fixture is validation-only under K0: every file either renders or reports explicit unsupported adapters after validation.

## 5. Old debug_recipes migration gap

Old root: `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/`.

New root: `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`.

| Family | Old count | Current v3.1 count | Represented in v3.1? |
|---|---:|---:|---|
| baseline | 2 | 1 | yes |
| filters | 98 | 4 | yes |
| masks | 41 | 4 | yes |
| samplers | 13 | 2 | yes |
| shaders/primitives | 39 | 1 | yes |
| shaders/compositions | 39 | 1 | yes |
| styles | 34 | 2 | yes |
| content | 111 | 0 | no |
| scene | 19 | 0 | no |
| shadows | 9 | 0 | no |
| complex | 83 | 0 | no |
| event_driven_dwell | 4 | 1 | yes |
| signals | 5 | 0 | no |
| easings | 29 | 0 | no |
| subcell_shapes | 5 | 0 | no |
| motion_routes | 5 | 0 | no |
| loopback | 3 | 0 | no |
| other | 64 | 0 | no |

Families not represented yet in v3.1:

- `content`
- `scene`
- `shadows`
- `complex`
- `signals`
- `easings`
- `subcell_shapes`
- `motion_routes`
- `loopback`
- `other` (`bindable_rates`, `fixtures`, and root/uncategorized legacy files)

Existing migration/QC guidance:

| Artifact/tool | Path/symbol | Use |
|---|---|---|
| Inventory manifest seed | `/usr/projects/tui-vfx-recipes/docs/v3_recipe_inventory_manifest.md` | Provisional corpus inventory and migration buckets. It explicitly says owner audit remains blocking for final keep/rewrite/drop decisions. |
| Equivalence manifest example | `/usr/projects/tui-vfx-recipes/docs/v3_migration_equivalence_pairs.example.json` | Manifest shape for V2↔V3 pairs. Includes a `complex_full_pipeline` pair, but paths point inside old `recipes/debug_recipes`, not the new `recipes/v3.1/debug_recipes` root. |
| Migration equivalence mode | `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_migration_equivalence_report_mode.rs::run_migration_equivalence_report_mode` | `pipeline-validator --migration-equivalence-report --format json <manifest>` compares named V2/V3 pairs. |
| Debug recipe QC mode | `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_debug_recipes_qc.rs::run_debug_recipes_qc` | `pipeline-validator --debug-recipes-qc --format json <paths>` gives family counts and validation/probe-ready reports. |
| CLI flags | `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/cli.rs` | Flags include `--debug-recipes-qc`, `--migration-equivalence-report`, `--lowering-report`, `--probe`, `--dump-normalized`, `--explore-normalized`. |

## 6. Schema finalization risk

### Current v3.1 public contract types

All paths below are under `/usr/projects/tui-vfx`.

| Area | Types/symbols | Files |
|---|---|---|
| Recipe root | `RecipeDocument`, `RecipeId`, `RecipeMetadata` | `crates/tui-vfx-contract/src/cls_recipe_document.rs`, `cls_recipe_id.rs`, `cls_recipe_metadata.rs` |
| Graph | `GraphSpec`, `GraphStep`, `GraphId`, `NodeId`, `NodeSpec` | `crates/tui-vfx-contract/src/cls_graph_spec.rs`, `cls_graph_step.rs`, `cls_node_spec.rs` |
| Node outputs/value bus | `NodeOutputSpec`, `NodeOutputSource`, `GraphValueId`, `GraphValueKind`, `GraphValueShape`, `GraphValueMergePolicy` | `crates/tui-vfx-contract/src/cls_node_output_spec.rs`, `cls_node_output_source.rs`, `cls_graph_value_id.rs`, `cls_graph_value_kind.rs`, `cls_graph_value_shape.rs`, `cls_graph_value_merge_policy.rs` |
| Lifecycle | `LifecycleSpec`, `LifecyclePhase`, `PhaseSpec`, `PhaseTiming`, `ClockSpec`, `ClockMode`, `DurationSpec`, `DwellPolicy` | `crates/tui-vfx-contract/src/cls_lifecycle_spec.rs`, `cls_lifecycle_phase.rs`, `cls_phase_spec.rs`, `cls_phase_timing.rs`, `cls_clock_spec.rs`, `cls_clock_mode.rs`, `cls_duration_spec.rs`, `cls_dwell_policy.rs` |
| Trigger | `TriggerSpec`, `TriggerCondition`, `TriggerAction`, `TriggerLatchPolicy`, `TriggerResetBoundary` | `crates/tui-vfx-contract/src/cls_trigger_spec.rs`, `cls_trigger_condition.rs`, `cls_trigger_action.rs`, `cls_trigger_latch_policy.rs`, `cls_trigger_reset_boundary.rs` |
| Scope | `ScopeSpec`, `ScopeKind`, `ScopeSupport`, `ScopeEvalInput`, `CoordinateSpace`, `RoleSpace` | `crates/tui-vfx-contract/src/cls_scope_spec.rs`, `cls_scope_kind.rs`, `cls_scope_support.rs`, `cls_scope_eval_input.rs`, `cls_coordinate_space.rs`, `cls_role_space.rs` |
| Value/input | `Value`, `ValueKind`, `ValueSpec`, `ValueSource`, `ValuePredicate`, `ParameterSpec`, `SignalSpec`, `BindingSpec` | `crates/tui-vfx-contract/src/cls_value.rs`, `cls_value_kind.rs`, `cls_value_spec.rs`, `cls_value_source.rs`, `cls_value_predicate.rs`, `cls_parameter_spec.rs`, `cls_signal_spec.rs`, `cls_binding_spec.rs` |
| Source | `SourceDescriptor`, `SourceSpec`, `SourceInputSpec`, `SourceOutputSpec`, `SourceKind`, `SourceLifecycle`, `SourceRolePolicy` | `crates/tui-vfx-contract/src/cls_source_descriptor.rs`, `cls_source_spec.rs`, `cls_source_input_spec.rs`, `cls_source_output_spec.rs`, `cls_source_kind.rs`, `cls_source_lifecycle.rs`, `cls_source_role_policy.rs` |
| Descriptor pack/catalog | `DescriptorPack`, `DescriptorPackRef`, `DescriptorPackId`, `DescriptorCatalog`, `EffectDescriptor`, `EffectInputSpec`, `EffectOutputSpec` | `crates/tui-vfx-contract/src/cls_descriptor_pack.rs`, `cls_descriptor_pack_ref.rs`, `cls_descriptor_pack_id.rs`, `cls_descriptor_catalog.rs`, `cls_effect_descriptor.rs`, `cls_effect_input_spec.rs`, `cls_effect_output_spec.rs` |
| Scene/semantic surface | `RecipeScene`, `RecipeSceneElement`, `RecipeElementPipeline`, `Scene`, `SceneElement`, `Surface`, `SceneOutcome`, `SurfaceDiagnostic` | `crates/tui-vfx-contract/src/cls_recipe_scene.rs`, `cls_recipe_scene_element.rs`, `cls_recipe_element_pipeline.rs`, `cls_scene.rs`, `cls_scene_element.rs`, `cls_surface.rs`, `cls_scene_outcome.rs`, `cls_surface_diagnostic.rs` |
| Schema generation tests | `generated_contract_schema_contains_rustdoc_descriptions`, `generated_contract_schema_objects_are_strict_and_described` | `crates/tui-vfx-contract/tests/test_schema_generation.rs` |

### Risks / under-specified areas for all primitive + complex effects

| Risk | Exact evidence | Why it matters |
|---|---|---|
| Descriptor catalog is tiny | `descriptors/v3.1/packs/primitive.json` has 14 effect ids and only `source.card`. | Old corpus has content, shadows, scene, signals, easings, subcell shapes, motion routes, loopback, and many primitive variants not representable in the current pack. |
| Source model is card-only in current pack | `primitive.json` source descriptors: `source.card` only; K0 source adapters in `fnc_render_scene.rs::render_source` support `source.card` and `source.text`, but only `source.card` is packed. | Complex demos need content/source families such as typewriter/text effects, procedural/scene sources, assets, and role-rich surfaces. |
| K0 ignores `RecipeSceneElement::pipeline` | `crates/tui-vfx-contract/src/cls_recipe_scene_element.rs` has `pipeline: Option<RecipeElementPipeline>`; `crates/tui-vfx-player/src/fnc_render_scene.rs::render_scene` only blits source rows by placement, then `apply_graph_effects` applies `recipe.graph.order` globally. | Element-local pipelines and graph topology are contract-visible but not player-visible. Complex scene layering will not match semantics until this is used. |
| Topology exists but K0 applies only linear order | `GraphSpec::topology: Option<GraphStep>` in `cls_graph_spec.rs`; `apply_graph_effects` iterates `recipe.graph.order`. | Complex effects need sequence/parallel semantics, merge policy, phase routing, and element-local subsets. |
| Scope is minimal | `ScopeSpec` variants are `All`, `Role`, `Rect`, `RowRange`, `ColumnRange`. | Good for early primitives, but old debug recipes include richer geometry/region concepts. Current scope may need more field hints or adapter rules before complex corpus parity. |
| Value model is scalar/rect/scope/role, not structured effect payloads | `Value` variants in `cls_value.rs`: null, bool, int, number, string, text, color, duration, enum, role, scope, rect. | Many legacy effects carry arrays, points, paths, gradients, color stops, font refs, content payloads, motion routes, and signal graphs. These need descriptor-specific flattening or new value shapes. |
| Compositor lowering does not exist in K0 | K0 file `fnc_apply_graph_effects.rs` switches on string ids and mutates `Vec<String>`; no `CompositionSpec` adapter is present. | Real visual output requires mapping `NodeSpec` ids/inputs to `CompositionSpec::{samplers,masks,filters,shader_layers,shadow}` and/or content/source primitives. |
| Ratatui cell adapter absent | No concrete `Grid` impl for `ratatui::buffer::Buffer`; only README placeholder. | K1 can still render via a custom widget/blit from `OwnedGrid`, but direct buffer adapter is not currently shipped. |
| Descriptor/output semantics are validation-first | `EffectDescriptor` validates declared inputs/outputs/scope/write support, but does not encode runtime lowering target. | Need a stable adapter registry linking descriptor ids to compositor/content/shadow implementations. |

## 7. Complex effect demonstration gap

Old fixture inspected:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_full_pipeline.json`

Old `complex_full_pipeline.json` shape:

| Legacy area | Payloads observed |
|---|---|
| content | `typewriter` content effect, `enter_only` |
| base style | foreground white, RGB background |
| enter mask | `wipe` |
| enter sampler | `sine_wave` |
| enter filter | `tint` |
| exit mask | `dissolve` |
| exit sampler | `shredder` |
| exit filter | `dim` |
| all-phase shader | `glisten_band` |

Current v3.1 equivalent: none found under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`. There is no `complex/` directory and no `*complex*` or `*pipeline*` v3.1 debug fixture.

### Minimal v3.1 complex demo shape

Do not start with a full old parity migration. Use a new v3.1 fixture that exercises all current primitive descriptor families with known ids:

- one `source.card` source with text/message, width, height
- one `RecipeScene` with one `RecipeSceneElement`
- graph nodes:
  - `mask.wipe` for visible enter mask
  - `sampler.sineWave` for sampler plumbing
  - `filter.tint` or `filter.dim` for filter plumbing
  - `shader.linearGradient` for shader plumbing
  - `style.baseStyleOverride` or `style.colorFade` for style/scope plumbing
- graph topology: either current linear `order` first, then `GraphStep::Sequence`/`GraphStep::Parallel` once K0 consumes topology
- lifecycle: `enter`, `dwell`, `exit` with explicit phase samples

### Blocking descriptors/adapters for a real visual demo

| Blocker | Current status |
|---|---|
| `shader.linearGradient` adapter | Descriptor and fixture exist; K0 reports `unsupportedEffectAdapter`. |
| `style.baseStyleOverride` adapter | Descriptor and fixture exist; K0 reports `unsupportedEffectAdapter`. |
| `style.colorFade` adapter | Descriptor and fixture exist; K0 reports `unsupportedEffectAdapter`. |
| `mask.dissolve` adapter | Descriptor and fixture exist; K0 reports `unsupportedEffectAdapter`. |
| `sampler.ripple` adapter | Descriptor and fixture exist; K0 reports `unsupportedEffectAdapter`. |
| `sampler.shredder` descriptor/adapter | Present in old `complex_full_pipeline`; absent from `primitive.json`. |
| `shader.glisten_band` descriptor/adapter | Present in old `complex_full_pipeline`; absent from `primitive.json`. |
| `content.typewriter` descriptor/adapter | Present in old `complex_full_pipeline`; absent from `primitive.json`; K0 currently handles text sources, not content effects. |
| base style/source style lowering | `style.baseStyleOverride` descriptor exists, but no visual adapter; K0 text-grid rows cannot express style changes. |

## 8. Recommended day plan

Keep K0 CLI regression as automation authority. Use K1 only for visual human inspection until K0 reports carry compositor-backed cells.

### Packet 1 — Inventory/reporting gate for v3.1 debug fixtures

| Field | Detail |
|---|---|
| Objective | Add a repeatable K0-side report that lists every v3.1 debug fixture, graph effect ids, source ids, descriptor coverage, and current render status. This becomes the migration gate before broad corpus work. |
| Files likely touched | `crates/tui-vfx-player/src/` new or existing report helper; `crates/tui-vfx-player-cli/src/fnc_run.rs`; `crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs`; tests in `crates/tui-vfx-player-cli/tests/`. |
| Commands | `cargo test -p tui-vfx-player-cli`; `cargo run -q -p tui-vfx-player-cli -- render-recipe --recursive --descriptor-pack descriptors/v3.1/packs/primitive.json /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes`; new inventory command/filter if added. |
| Acceptance | Machine-readable report proves 16 fixtures, 14/14 descriptor effect ids represented, 10 rendered, 6 unsupported, with per-file effects/sources. No renderer behavior changes. |
| Risks | Scope creep into adapters; avoid by making this report-only. Preserve existing `v3.1.player.run.1` output or version any new report schema. |

### Packet 2 — K0 visual frame substrate without adapter expansion

| Field | Detail |
|---|---|
| Objective | Introduce a K0 internal visual frame path that can carry `OwnedGrid`/cell data while preserving current text rows and hashes. Convert `source.card`/`source.text` rows to `OwnedGrid + RoleMap`. |
| Files likely touched | `crates/tui-vfx-player/src/fnc_render_scene.rs`; `crates/tui-vfx-player/src/cls_player_frame.rs`; `crates/tui-vfx-player/src/fnc_build_player_frame.rs`; tests in `crates/tui-vfx-player/tests/test_fnc_recipe_player.rs`. |
| Commands | `cargo test -p tui-vfx-player`; K0 recursive render smoke. |
| Acceptance | Existing rows/statuses remain stable; new cell/visual DTO is available behind K0 API; no compositor effects yet. |
| Risks | Hash churn and serialization compatibility. Prefer additive fields and unchanged row hash input until the visual path is accepted. |

### Packet 3 — Contract graph to `CompositionSpec` adapter for supported primitives

| Field | Detail |
|---|---|
| Objective | Add a small adapter registry that lowers current descriptor ids into `CompositionSpec` for a first compositor-backed frame. Start with `mask.wipe`, `mask.checkers`, `mask.none`, `sampler.sineWave`, and one real filter (`filter.tint` or `filter.dim`). |
| Files likely touched | New helper under `crates/tui-vfx-player/src/`; `fnc_apply_graph_effects.rs` or replacement seam; `crates/tui-vfx-player/src/cls_recipe_player.rs`; tests in `crates/tui-vfx-player/tests/`. |
| Commands | `cargo test -p tui-vfx-player`; recursive K0 smoke; focused render for `masks/mask_wipe.json`, `filters/filter_tint.json`, `samplers/sampler_sinewave.json`. |
| Acceptance | K0 can produce compositor-backed cells for the first supported set while preserving CLI rows. Unsupported ids still report explicitly. |
| Risks | Descriptor input names may not line up with compositor spec constructors; keep one id at a time and include accept/default tests. |

### Packet 4 — K1 cell preview blit

| Field | Detail |
|---|---|
| Objective | Teach K1 preview to render K0 visual cells in a ratatui area, falling back to text rows when visual cells are absent. |
| Files likely touched | `crates/tui-vfx-player-ui/src/fnc_render_ratatui_ui.rs`; possibly new cell-to-ratatui helper; `crates/tui-vfx-player-ui/tests/test_fnc_player_ui.rs`. |
| Commands | `cargo test -p tui-vfx-player-ui`; `just player-ui-once`; manual `just player-ui`. |
| Acceptance | Ratatui `TestBackend` test proves draw succeeds with visual cells; existing one-shot text output remains useful. |
| Risks | Color/modifier mapping from `tui_vfx_types::Cell` to ratatui needs alpha policy. Keep transparent cells non-writing or default-style first. |

### Packet 5 — First complex fixture and descriptor/adapters gap closure

| Field | Detail |
|---|---|
| Objective | Add one canonical v3.1 complex fixture using current descriptor ids, then implement only the missing adapter(s) needed for a visible mask+sampler+filter+shader/style demo. |
| Files likely touched | `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/complex/complex_minimal_pipeline.json`; `descriptors/v3.1/packs/primitive.json` only if descriptor gaps block; K0 adapter files from Packet 3. |
| Commands | K0 recursive smoke; `cargo test -p tui-vfx-player`; recipe-side `pipeline-validator --debug-recipes-qc --format json recipes/v3.1/debug_recipes` if working in recipe repo. |
| Acceptance | New complex fixture validates, K0 renders compositor-backed visual cells, K1 can visually inspect it, unsupported count does not regress unexpectedly. |
| Risks | Temptation to migrate old `complex_full_pipeline` wholesale. Avoid until adapter/reporting gate makes gaps explicit. |

## 9. First work packet recommendation

Recommended first packet: **K0 v3.1 fixture inventory gate**.

Why first:

- It improves migration/debug systematically without changing render semantics.
- It turns today's ad-hoc inventory into an automation gate owned by K0, the agreed automation authority.
- It prevents blind corpus migration by making descriptor coverage, unsupported adapters, and fixture status visible before edits.
- It gives later adapter packets an objective before/after metric: unsupported count should fall for specific ids, rendered count should rise only when real visual support exists.
- It is small enough for an implementation agent: parse existing recursive run data + recipe JSON graph/source ids + descriptor pack ids, add tests, no compositor work.

Suggested first packet: K0 v3.1 fixture inventory gate
