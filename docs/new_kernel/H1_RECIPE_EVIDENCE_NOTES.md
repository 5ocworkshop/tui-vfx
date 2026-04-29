<!-- <FILE>docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md</FILE> - <DESC>Phase H1 recipe evidence mapping notes for canonical recipe schema pressure</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase H1: record curated legacy recipe evidence without adopting legacy field names.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — map representative recipe concepts to canonical v3.1 homes and deferred gaps.</CLOG> -->

# H1 Recipe Evidence Notes

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: H1 — Canonical Recipe Document Schema

## Purpose

These notes record mapping pressure from representative recipe examples while designing the canonical v3.1 recipe document.

The examples are **evidence only**:

```text
Do not migrate them here.
Do not preserve their exact JSON shape.
Do not let legacy aliases or old field names define canonical v3.1.
```

A successful H1 mapping means the old evidence concept has a clear canonical home, a migration-only treatment, or a documented deferred phase. It does not mean the old field name becomes valid canonical syntax.

## Recipe evidence read

Curated examples from `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/`:

```text
baseline.json
complex/complex_full_pipeline.json
complex/complex_nested_parallel_sequences.json
complex/complex_parallel_overlap_conflict_snapshot.json
complex/v3_cross_family_sequence_disjoint.json
complex/v3_scheduler_parallel_join_filter_mask.json
complex/v3_io_scalar_filter.json
complex/v3_io_radial_twist_spiral_chain.json
complex/v3_io_parallel_merge_shader.json
complex/v3_io_authoring_ladder_toast_glow_chain.json
scene/scene_layer_full_stack.json
scene/scene_layer_visibility_binding_io.json
scene/scene_layer_io_filter_shader.json
scene/scene_authoring_ladder_flag_asset_binding.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
scene/scene_braille_flag_asset_token.json
scene/scene_braille_flag_runtime_wave.json
content/content_split_flap_solari_authentic.json
filters/filter_kitt_scanner_progress_binding.json
loopback/loopback_rigid_shake_severity_ramp.json
signals/wave_with_envelope_signal.json
event_driven_dwell/bool_binding_demo.json
event_driven_dwell/bool_binding_truthy_loopback.json
motion_routes/motion_figure_eight_infinity.json
motion_routes/scene_layer_follow_lag.json
motion_routes/toast_shadow_edge_crossing.json
complex/resize_preserve_phase_chain.json
scene/ansi_source_chain.json
scene/scene_image_source_bindable.json
complex/command_capture_chain.json
```

## Evidence mapping table

| Old evidence concept | Canonical v3.1 home | Status |
|---|---|---|
| Recipe id/title/description/version | `RecipeDocument.id`, `RecipeDocument.version`, `RecipeMetadata` | Covered. |
| `config.message` text payload | `SourceSpec` using a text `SourceDescriptor` input | Covered; exact field lowers away. |
| `config.layout.width` / `height` | `RecipeScene.width` / `RecipeScene.height` or source input when source-local | Covered. |
| `config.base_style` | Source descriptor/source inputs for initial surface or effect descriptor inputs for transforms | Covered conceptually; styling descriptors still need real ports later. |
| `config.border` | Source descriptor/source inputs or effect descriptor/node depending on whether it produces or transforms surface | Covered conceptually; descriptor porting deferred. |
| `config.pipeline.step.kind` | `GraphStep::Sequence`, `GraphStep::Parallel`, or `GraphStep::Node` | Covered; old names are non-canonical. |
| Pipeline child order | `GraphSpec.order` plus optional `GraphSpec.topology` | Covered. |
| Parallel pipeline conflicts | `ParallelMergePolicy` and `GraphValueMergePolicy` | Covered at topology/value level; richer visual conflict behavior deferred to effect descriptors/runtime. |
| Shader/filter/mask/sampler payloads | `EffectDescriptor`, `NodeSpec.inputs`, `ValueSource` | Covered as descriptor-backed nodes; real effect ports deferred. |
| `io.outputs[].hint` | `EffectOutputSpec`, `NodeOutputSpec`, `GraphValueId` | Covered; `hint` is legacy evidence only. |
| Node producer/consumer chains | `GraphValueId` and `ValueSource::GraphValue` | Covered. |
| Parallel graph value joins | `GraphValueMergePolicy`, graph topology, node inputs | Covered for contract shape; execution proof exists separately. |
| `requires_bindings` | `ParameterSpec`, `SignalSpec`, `BindingSpec`, source/node `ValueSource` inputs | Covered; old field lowers away. |
| Runtime progress/audio/wave values | `SignalSpec` and `ValueSource::Signal` | Covered for declaration/reference; runtime store execution deferred. |
| Loopback/demo signal scaffolding | Demo/player profile outside canonical recipe | Deferred; not a contract root in H1. |
| `requires_assets` | `AssetSpec` plus source `AssetRequirement` | Covered; old field lowers away. |
| `{{ flag_art }}` interpolation token | `AssetRef { id }` | Covered with canonical structural refs; interpolation is rejected/non-canonical. |
| Scene layer entries | `RecipeSceneElement` referencing a source-produced surface | Covered; `scene.layers[]` does not become canonical. |
| Source descriptors | `SourceDescriptor` | Covered. |
| Source instances | `RecipeDocument.sources: BTreeMap<SourceInstanceId, SourceSpec>` | Covered. |
| Typed source inputs | `SourceInputSpec` and `SourceSpec.inputs` | Covered. |
| Asset-backed source slots | `AssetRequirement` and `AssetRef` | Covered. |
| Procedural source params such as wave speed | Dotted `SourceInputId` with `ValueSource` | Covered without adopting old field names. |
| Generated/default roles | `SourceRolePolicy` | Covered. |
| Source-produced surfaces | `SourceOutputSpec` and `RecipeSceneElement.source` | Covered. |
| Source-local pipelines | `RecipeElementPipeline` as a graph/topology reference seam | Covered as a future integration point; no runtime execution in H1. |
| ANSI/image/command capture sources | `SourceKind::{Ansi, Image, CommandCapture}` plus descriptors/specs | Covered at descriptor category level; real adapters deferred. |
| Visibility predicates | Phase/trigger/timing model or future binding target, not H1 root | Deferred. |
| Event-driven dwell/latch/caps | Future phase/timing/trigger model | Deferred. |
| Motion routes / enter / exit | Future motion/phase model over graph/scene elements | Deferred. |
| Resize behavior | `SourceLifecycle.resize_aware`, `ScopeSpec`, future host-grid/timing semantics | Partly covered; runtime resize execution deferred. |
| Studio controls/manifest data | Future manifest/studio phase | Deferred. |

## H1 conclusions

1. The canonical recipe root should package the already-locked concepts rather than inventing source-specific or pipeline-specific legacy branches.
2. Representative recipes map without adding new top-level concepts beyond `RecipeDocument`, `RecipeScene`, source instances, and element-local pipeline references.
3. Remaining unmapped pressure is mostly descriptor porting, migration rules, runtime execution, timing/trigger semantics, or studio/demo/player concerns.
4. H1 should not lock phase/timing, motion, visibility predicates, demo loopback, or studio manifest syntax.

## Schema-lock pressure result

H1 does not prove the full corpus maps perfectly. It proves that failures now classify cleanly:

```text
A. Need an effect/source descriptor or adapter port.
B. Need a migration/lowering rule.
C. Need a deferred runtime/timing/studio/demo phase.
D. Need a genuine schema concept.
```

The curated evidence produced no immediate D-class requirement that should block H1.

<!-- <FILE>docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md</FILE> - <DESC>Phase H1 recipe evidence mapping notes for canonical recipe schema pressure</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
