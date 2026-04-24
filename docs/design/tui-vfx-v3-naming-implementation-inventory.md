<!-- <FILE>docs/design/tui-vfx-v3-naming-implementation-inventory.md</FILE> - <DESC>Implementation inventory for the accepted V3 naming slate covering Ra to Vfx, Preview to Playback, frame snapshots, render helpers, and player seams.</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>V3-NAME01/PREVIEW01 needs an exact inventory before any broad rename. This document records current symbols, files, buckets, order, and risks so the later cutover can be staged rather than performed by blind search/replace.</WCTX> -->
<!-- <CLOG>0.1.1: clarify the next actionable rename bucket and keep Bucket C provisional until seam names are final.</CLOG> -->

# V3 naming implementation inventory

This is the implementation inventory for `V3-NAME01` and `V3-PREVIEW01`.
It follows the accepted slate in
[`tui-vfx-v3-naming-normalization-decisions.md`](tui-vfx-v3-naming-normalization-decisions.md):

- `Ra*` public/wire-format types become `Vfx*`.
- canonical `Preview*` engine seams become `Playback*`.
- `DirectV3PreviewSnapshot` becomes `V3FrameSnapshot`.
- `render_direct_v3_snapshot` becomes `render_v3_frame_to_buffer`.
- the thin CLI/tooling surface is `tui-vfx-player`.

No broad rename was performed while producing this inventory.

## Scope buckets

### Bucket A — `Ra*` public/wire-format schema surface in `tui-vfx-recipes`

Primary file: `tui-vfx-recipes/src/recipe_schema/config.rs`.

Current public definitions:

- `RaLayoutMode` at `src/recipe_schema/config.rs:41`
- `RaAnimationType` at `src/recipe_schema/config.rs:64`
- `RaLayoutConfig` at `src/recipe_schema/config.rs:99`
- `RaLifecycleConfig` at `src/recipe_schema/config.rs:176`
- `RaBorderType` at `src/recipe_schema/config.rs:225`
- `RaBorderTrim` at `src/recipe_schema/config.rs:262`
- `RaTitlePosition` at `src/recipe_schema/config.rs:287`
- `RaTitleAlignment` at `src/recipe_schema/config.rs:318`
- `RaPaddingConfig` at `src/recipe_schema/config.rs:340`
- `RaCustomBorderChars` at `src/recipe_schema/config.rs:377`
- `RaFrameContent` at `src/recipe_schema/config.rs:447`
- `RaBorderConfig` at `src/recipe_schema/config.rs:518`
- `RaContentMode` at `src/recipe_schema/config.rs:623`
- `RaContentConfig` at `src/recipe_schema/config.rs:643`
- `RaTimeConfig` at `src/recipe_schema/config.rs:769`
- `RaTransitionConfig` at `src/recipe_schema/config.rs:803`
- `RaMaskConfig` at `src/recipe_schema/config.rs:1013`
- `RaSamplerConfig` at `src/recipe_schema/config.rs:1078`
- `RaFilterConfig` at `src/recipe_schema/config.rs:1121`
- `RaApplyTo` at `src/recipe_schema/config.rs:1167`
- `RaStyleEffect` at `src/recipe_schema/config.rs:1196`
- `RaBaseStyle` at `src/recipe_schema/config.rs:1583`
- `RaStylePipelineConfig` at `src/recipe_schema/config.rs:1631`
- `RaPipelineConfig` at `src/recipe_schema/config.rs:1721`
- `RaRecipeConfig` at `src/recipe_schema/config.rs:1988`

Additional public definitions / aliases under `src/recipe_schema`:

- `RaClock` at `src/recipe_schema/enum_ra_clock.rs:13`
- `RaContinuousConfig` at `src/recipe_schema/cls_ra_continuous_config.rs:15`
- `RaJsonRecipeDefinition` at `src/recipe_schema/parser.rs:18`
- `RaJsonRecipeDyn` at `src/recipe_schema/parser.rs:39`
- `RaImageAspect` at `src/recipe_schema/scene/cls_ra_image_source.rs:13`
- `RaImageSource` at `src/recipe_schema/scene/cls_ra_image_source.rs:22`
- `RaCardSource` compatibility alias at `src/recipe_schema/scene/cls_ra_card_source.rs:39`
- `RaContentSource` compatibility alias at `src/recipe_schema/scene/enum_ra_content_source.rs:23`
- `RaLayerOverflow` at `src/recipe_schema/scene/enum_ra_layer_overflow.rs:12`
- `RaAnsiSource` compatibility alias at `src/recipe_schema/scene/cls_ra_ansi_source.rs:32`
- `RaSceneConfig` at `src/recipe_schema/scene/cls_ra_scene_config.rs:16`
- `RaLayerVisibility` at `src/recipe_schema/scene/enum_ra_layer_visibility.rs:16`
- `RaSceneLayer` at `src/recipe_schema/scene/cls_ra_scene_layer.rs:17`
- `RaSceneFitPolicy` at `src/recipe_schema/scene/enum_ra_scene_fit_policy.rs:12`
- `RaAnchoredPlacement` at `src/recipe_schema/scene/enum_ra_layer_placement.rs:17`
- `RaAbsolutePlacement` at `src/recipe_schema/scene/enum_ra_layer_placement.rs:49`
- `RaLayerPlacement` at `src/recipe_schema/scene/enum_ra_layer_placement.rs:62`
- `RaLayerSurface` at `src/recipe_schema/scene/cls_ra_layer_surface.rs:18`
- `RaProceduralSource` at `src/recipe_schema/scene/cls_ra_procedural_source.rs:11`
- `RaTextAlignment` compatibility alias at `src/recipe_schema/scene/cls_ra_text_source.rs:42`
- `RaTextSource` compatibility alias at `src/recipe_schema/scene/cls_ra_text_source.rs:45`

Recommended target shape:

- Rename real definitions to `Vfx*`.
- Keep `pub use Vfx* as Ra*` compatibility aliases where external users or V2 cutover tooling still need old names.
- Keep serde field names unchanged unless a separate schema decision explicitly changes JSON shape.
- Treat existing `Vfx* as Ra*` aliases as already partially migrated; do not invert them back to concrete `Ra*` definitions.

Risk bucket: **high**. This is the main public/wire-format surface and must be paired with rustdocs, generated docs, and deprecation guidance.

### Bucket B — `Ra*` consumers that must move with Bucket A

Files currently importing or naming `Ra*` outside direct definitions include:

- `src/preview/cls_direct_v3_preview_state.rs` — `RaLayoutConfig`, `RaLayoutMode`
- `src/preview/fnc_resolve_content_text.rs` — `RaContentMode`
- `src/preview/fnc_preview_from_config.rs` — `RaRecipeConfig`, `RaApplyTo`, `RaStyleEffect`, layout/border/title enums, `RaBaseStyle`
- `src/preview/cls_preview_item.rs` — `RaContentMode`
- `src/recipe/fnc_from_value.rs` — `RaJsonRecipeDefinition`
- `src/recipe/types.rs` — `RaRecipeConfig`, `RaJsonRecipeDefinition`
- `src/manager/mod.rs` — `RaSceneConfig`
- `src/scene/fnc_layer_cache_key.rs` — `RaSceneLayer`, `RaContentSource`
- `src/v3/compile/cls_compiled_recipe_plan.rs` — scene/content/placement/surface/overflow/visibility/fit-policy `Ra*` types
- `src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs` — `RaBaseStyle`
- `src/v3/compile/fnc_execute_compiled_step_tree_to_scene.rs` — `RaBaseStyle`
- `src/v3/compile/fnc_render_compiled_plan_deterministically.rs` — `RaLayoutConfig`, `RaLayoutMode`

Recommended target shape:

- Update internal imports/usages to canonical `Vfx*` names after definitions exist.
- Leave compatibility aliases only for external migration and historical V2 docs, not for newly-written internal code.

Risk bucket: **medium-high** because these consumers span legacy preview, V3 compile/render, scene, probe, and manager paths.

### Bucket C — canonical playback seam currently named preview

Current public definitions / exports:

- `src/preview/cls_preview_item.rs:29` — `PreviewItem`
- `src/preview/cls_preview_manager.rs:24` — `PreviewManager`
- `src/preview/cls_preview_recipe_bridge.rs:14` — `PreviewRecipeBridge`
- `src/preview/cls_direct_v3_preview_snapshot.rs:17` — `DirectV3PreviewSnapshot`
- `src/preview/cls_direct_v3_preview_state.rs:27` — `DirectV3PreviewState`
- `src/preview/fnc_preview_from_config.rs:104` — `preview_from_recipe_config`
- `src/preview/fnc_preview_from_config.rs:110` — `preview_from_recipe_config_with_resolution_log`
- `src/preview/fnc_preview_from_config.rs:461` — `preview_for_recipe_id`
- `src/preview/fnc_preview_from_recipe_path.rs:19` — `preview_from_recipe_path_with_cutover_fallback`
- `src/preview/fnc_render_preview_item.rs:169` — `render_preview_item`
- `src/preview/fnc_render_preview_item.rs:286` — `render_preview_item_inspected`
- `src/preview/fnc_render_direct_v3_snapshot.rs:37` — `render_direct_v3_snapshot`
- `src/preview/mod.rs:55-66` — public preview exports
- `src/prelude.rs:57-58` — prelude re-exports
- `src/lib.rs` — top-level preview module docs and export surface

Accepted target names:

- `PreviewItem` -> `PlaybackPlan`
- `PreviewManager` -> `PlaybackController`
- `PreviewRecipeBridge` -> likely `PlaybackRecipeBridge` or `RecipePlaybackBridge`; use `PlaybackRecipeBridge` unless owner picks a shorter bridge name during cutover.
- `DirectV3PreviewState` -> likely `V3PlaybackState` or `V3PlaybackControllerState`; use a compatibility alias until the owner confirms whether stateful direct V3 scrubbing belongs under the broader `PlaybackController` name.
- `DirectV3PreviewSnapshot` -> `V3FrameSnapshot`
- `render_direct_v3_snapshot` -> `render_v3_frame_to_buffer`
- `src/preview/` -> `src/playback/` for canonical engine seams.

Risk bucket: **high**. The old names are used by examples, tests, probe, trace, validator, prelude, and generated docs. Stage this separately from Bucket A if possible.

### Bucket D — V3 compile/render functions that contain `preview`

Current public functions in `src/v3/compile/fnc_render_compiled_plan_deterministically.rs`:

- `render_compiled_plan_for_preview` at line 40
- `render_compiled_plan_for_preview_sampled` at line 54
- `render_compiled_plan_for_preview_timed` at line 75
- `render_compiled_plan_for_preview_timed_with_overrides` at line 98
- `render_compiled_plan_for_preview_area_timed` at line 241
- `render_compiled_plan_for_preview_area_timed_with_overrides` at line 268

Related private helpers:

- `render_preview_snapshot` at line 481
- `render_ordered_preview_if_supported` at line 557

Current return type: `DirectV3PreviewSnapshot`.

Recommended target names:

- `render_compiled_plan_for_playback*` for canonical playback-space functions.
- Return `V3FrameSnapshot` after Bucket C snapshot rename.
- Keep deprecated `render_compiled_plan_for_preview*` aliases/re-exports for a cutover window if downstream tools still import them.

Risk bucket: **medium-high**. These functions are public V3 execution seams and appear in generated API docs and direct V3 tests.

### Bucket E — currently absent target names

Exact searches found no live definitions/usages yet for these accepted target names:

- `CompositionPlan`
- `PlaybackPlan`
- `V3FrameSnapshot`
- `render_v3_frame_to_buffer`
- `tui-vfx-player`

Implication: the rename will introduce these names rather than reconcile existing duplicate definitions. `CompositionPlan` is also not part of the accepted final slate except as a searched planning term; avoid introducing it unless a separate architecture decision chooses it.

Risk bucket: **low for collision**, **medium for docs** because generated and hand-maintained docs must be updated after code introduces the new names.

### Bucket F — tooling, examples, tests, and generated docs consumers

Representative current consumers of `PreviewItem`, `PreviewManager`, direct snapshot rendering, and compiled plan preview functions:

- Examples: `examples/demo.rs`, `examples/play_recipe.rs`, `examples/diag_render_dump.rs`, `examples/diag_timeline_dump.rs`
- Tests: `tests/manager/*`, `tests/integration/*`, `tests/scene/*`, `tests/test_canvas_compositing.rs`, `tests/test_motion_rect_*`, `tests/test_matrix_rain_recipes.rs`
- Probe/runtime code: `src/probe/*`, `src/rendering/*`, `src/manager/mod.rs`
- Tools: `tools/pipeline-validator/src/**`, `tools/tui-vfx-trace/src/orc_run_trace.rs`, `tools/recipe-probe/src/main.rs`
- Generated docs: `docs/generated/V3_API.md`, `docs/generated/v3_api.json`

Recommended handling:

- Update source and tests in the same bucket as the symbol they depend on.
- Regenerate generated docs after the Rust rename; do not hand-edit generated artifacts except as a last-resort temporary note.
- Keep examples named `demo` or `play_recipe` where those are human preview/player tools; only rename engine seam imports and types.

Risk bucket: **medium**. Wide fan-out, but mostly mechanical once compatibility aliases exist.

## Recommended rename order

1. **Introduce canonical aliases first, no behavior change.**
   - Add `Vfx*` aliases or renamed definitions with `Ra*` compatibility aliases for schema-bearing types.
   - Add `V3FrameSnapshot` / `render_v3_frame_to_buffer` aliases around the current direct snapshot type/helper.
   - Add `PlaybackPlan` / `PlaybackController` aliases around current preview plan/controller if a zero-behavior compatibility pass is needed.

2. **Move internal code to canonical imports.**
   - Update V3 compile/render and playback/preview internals to use `Vfx*`, `V3FrameSnapshot`, and playback names.
   - Keep old names available only as compatibility exports.

3. **Rename module path last within the seam.**
   - Move `src/preview/` to `src/playback/` only after symbol aliases are stable.
   - Keep `pub mod preview` or re-export shim temporarily if downstream code still imports `tui_vfx_recipes::preview`.

4. **Update tools/examples/tests.**
   - Pipeline validator, trace, recipe-probe, examples, and tests should import canonical names.
   - Human-facing preview/demo terminology may remain when it describes the UX, not the engine seam.

5. **Regenerate docs and add migration notes.**
   - Refresh generated API/schema docs.
   - Add deprecation tables from old names to new names.
   - Remove stale future-facing `Ra*`/`Preview*` guidance, but leave historical V2/archive references accurate.

6. **Remove compatibility aliases only at a later V3 cutover gate.**
   - Do not remove V2 support or old aliases as part of the initial rename unless the owner explicitly approves the breaking cleanup.

## Next bucket to work

If the next bucket needs to be picked now, start with **Bucket A**.

Reason:

- its target `Vfx*` names are fully enumerated here
- the change is still a schema surface, so the rename can be staged behind compatibility aliases
- the follow-on `Ra*` consumer updates in Bucket B can move in lockstep once Bucket A exists

Do **not** treat Bucket C as the next bucket unless the seam-name decision is narrowed further. `PreviewRecipeBridge` and `DirectV3PreviewState` still have provisional replacement names, so that bucket carries more naming ambiguity than the schema surface.

## Blockers and cautions

- Owner decision is already accepted for the main naming slate, but `DirectV3PreviewState` and `PreviewRecipeBridge` do not have exact accepted replacement names. Treat recommended replacements as provisional.
- `PlaybackPlan` collides conceptually with existing `CompiledRecipePlan`; the cutover should document whether `PlaybackPlan` is the public loaded/renderable wrapper and `CompiledRecipePlan` remains an internal V3 compile artifact.
- `CompositionPlan` was searched but is not currently present and is not the accepted seam name. Do not introduce it by accident.
- Archive paths such as `docs/v2-spec-archive/**` and historical decision docs may correctly retain old names.
- Keep V2 support intact. Naming cutover is not V2 retirement.

## Commands used

```sh
/usr/local/bin/ofpf-orientation --root /usr/projects/tui-vfx
/usr/local/bin/ofpf-orientation --root /usr/projects/tui-vfx-recipes
cat /usr/projects/tui-vfx/steering/INTENTIONS.md \
  /usr/projects/tui-vfx/steering/ORCHESTRATION.md \
  /usr/projects/tui-vfx/docs/design/tui-vfx-v3-naming-normalization-decisions.md \
  /usr/projects/tui-vfx/docs/design/tui-vfx-v3-outstanding-master-list.md \
  /usr/projects/tui-vfx/docs/design/tui-vfx-v3-execution-dag.md
rg -n --hidden -g '!target' -g '!*.lock' '<target symbol patterns>' \
  /usr/projects/tui-vfx /usr/projects/tui-vfx-recipes
rg -n '^pub (struct|enum|type|trait) Ra[A-Z]|^pub use .* as Ra[A-Z]' \
  src/recipe_schema src/v3 src/preview src/recipe src/manager src/scene
rg -n '^pub (struct|enum|type|trait) .*Preview|^pub fn .*preview|^pub use .*Preview|^pub use .*preview' \
  src tools examples tests
rg -n '\b(CompiledRecipePlan|RecipePlan|CompositionPlan|PlaybackPlan|DirectV3PreviewSnapshot|render_direct_v3_snapshot|render_compiled_plan_for_preview|PreviewRecipeBridge|PreviewItem|PreviewManager)\b' \
  src tools examples tests
```

<!-- <FILE>docs/design/tui-vfx-v3-naming-implementation-inventory.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
