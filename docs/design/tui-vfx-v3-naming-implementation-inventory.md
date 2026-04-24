<!-- <FILE>docs/design/tui-vfx-v3-naming-implementation-inventory.md</FILE> - <DESC>Working cutover inventory for the accepted V3 naming slate across schema, playback seams, frame snapshots, adapter helpers, and thin-player/tooling surfaces.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>V3-NAME01/PREVIEW01 needs a concrete cross-repo cutover map before any broad rename. This document records the accepted naming slate, the current file/symbol buckets in tui-vfx, tui-vfx-recipes, and gt-design, and the compatibility plan so later work can move in deliberate slices.</WCTX> -->
<!-- <CLOG>0.2.0: expand the inventory into concrete repo/file/symbol buckets and add the compat/re-export plan for the accepted naming slate.</CLOG> -->

# V3 naming cutover inventory

This is the cutover inventory for `V3-NAME01` and `V3-PREVIEW01`.
It follows the accepted slate in
[`tui-vfx-v3-naming-normalization-decisions.md`](tui-vfx-v3-naming-normalization-decisions.md):

- `Ra*` public/wire-format types become `Vfx*`.
- canonical `Preview*` engine seams become `Playback*`.
- `DirectV3PreviewSnapshot` becomes `V3FrameSnapshot`.
- `render_direct_v3_snapshot` becomes `render_v3_frame_to_buffer`.
- the thin CLI/tooling surface is `tui-vfx-player`.

No broad rename was performed while producing this inventory.

## 1. Accepted target slate

| Current / question | Accepted target | Cutover note |
|---|---|---|
| `Ra*` public/wire-format types | `Vfx*` | Keep `Ra*` as compatibility aliases during cutover where needed. |
| `PreviewItem` / `RecipeItem` / `PlaybackItem` candidates | `PlaybackPlan` | The seam object is a load-ready/renderable plan, not a demo item. |
| `PreviewManager` | `PlaybackController` | Use when the type owns time, state, scrubbing, or frame advancement. |
| `src/preview/` canonical seam path | `src/playback/` | Keep a `preview` shim only while downstream imports are still moving. |
| `DirectV3PreviewSnapshot` | `V3FrameSnapshot` | One rendered frame/grid snapshot can serve preview, probe, movie, CI, and static export surfaces. |
| `render_direct_v3_snapshot` | `render_v3_frame_to_buffer` | Adapter-boundary function should state that it renders one frame snapshot into a buffer. |
| thin player / movie-layer naming | `tui-vfx-player` | Use player vocabulary for the small CLI/tool surface; reserve movie for the deferred scripted layer. |
| `render_compiled_plan_for_preview*` | `render_compiled_plan_for_playback*` | Canonical playback-space helper family. |
| `PreviewRecipeBridge` | `PlaybackRecipeBridge` | Bridge/helper name should follow the seam vocabulary unless a later decision narrows it. |
| `DirectV3PreviewState` | `V3PlaybackState` or `V3PlaybackControllerState` | Provisional until the exact ownership split is finalized. |

## 2. Current live buckets by repo

### 2.1 `tui-vfx-recipes`: live schema surface and preview seam

This is the highest-risk bucket. It contains the current public `Ra*` schema
surface, the current `preview` seam, and the V3 deterministic render helpers.

Representative schema files:

```text
tui-vfx-recipes/src/recipe_schema/config.rs
tui-vfx-recipes/src/recipe_schema/mod.rs
tui-vfx-recipes/src/recipe_schema/parser.rs
tui-vfx-recipes/src/recipe_schema/validator/*
tui-vfx-recipes/src/recipe_schema/scene/*
```

Representative schema symbols:

- `RaRecipeConfig`, `RaPipelineConfig`, `RaStylePipelineConfig`, `RaMaskConfig`, `RaFilterConfig`, `RaSamplerConfig`, `RaStyleEffect`, `RaBaseStyle`, `RaClock`, `RaContinuousConfig`, `RaSceneConfig`, `RaLifecycleConfig`, `RaContentConfig`
- `RaJsonRecipeDefinition`, `RaJsonRecipeDyn`
- `RaSceneLayer`, `RaSceneConfig`, `RaSceneFitPolicy`, `RaLayerPlacement`, `RaLayerOverflow`, `RaLayerVisibility`
- `RaImageSource`, `RaProceduralSource`, `RaCardSource`, `RaTextSource`, `RaContentSource`, `RaLayerSurface`

Current preview/playback seam files:

```text
tui-vfx-recipes/src/preview/cls_preview_item.rs
tui-vfx-recipes/src/preview/cls_preview_manager.rs
tui-vfx-recipes/src/preview/cls_preview_recipe_bridge.rs
tui-vfx-recipes/src/preview/cls_direct_v3_preview_snapshot.rs
tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs
tui-vfx-recipes/src/preview/fnc_preview_from_config.rs
tui-vfx-recipes/src/preview/fnc_preview_from_recipe_path.rs
tui-vfx-recipes/src/preview/fnc_render_direct_v3_snapshot.rs
tui-vfx-recipes/src/preview/fnc_render_preview_item.rs
tui-vfx-recipes/src/preview/mod.rs
tui-vfx-recipes/src/v3/compile/fnc_render_compiled_plan_deterministically.rs
```

Current consumer buckets around that seam:

```text
tui-vfx-recipes/examples/demo.rs
tui-vfx-recipes/examples/play_recipe.rs
tui-vfx-recipes/examples/diag_render_dump.rs
tui-vfx-recipes/examples/diag_timeline_dump.rs
tui-vfx-recipes/examples/v3_play_recipe.rs
tui-vfx-recipes/tests/*
tui-vfx-recipes/tools/tui-vfx-trace/src/orc_run_trace.rs
tui-vfx-recipes/src/probe/*
```

Recommended cutover shape:

- rename the real `Ra*` definitions to `Vfx*`
- keep `pub use Vfx* as Ra*` compatibility aliases for the schema surface until the final cutover gate
- add `PlaybackPlan` / `PlaybackController` / `PlaybackRecipeBridge` / `V3FrameSnapshot` / `render_v3_frame_to_buffer` alongside the old names, then move internal imports to the canonical names
- keep `src/preview/` as a temporary compatibility re-export while `src/playback/` becomes the canonical module path
- keep serde field names unchanged unless a separate schema decision explicitly changes JSON shape

Risk bucket: **high**.

### 2.2 `tui-vfx`: docs, plans, and tooling references

This repo mostly holds the accepted naming decisions, the punch list, and the
tooling docs that still talk about the seam as preview/player.

Representative files:

```text
tui-vfx/docs/design/tui-vfx-v3-naming-normalization-decisions.md
tui-vfx/docs/design/tui-vfx-v3-outstanding-master-list.md
tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md
tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/80_open_questions.md
tui-vfx/docs/design/tui-vfx-v3-execution-dag.md
tui-vfx/docs/design/tui-vfx-v3-migration-findings-memo-claude.md
tui-vfx/docs/tooling/INDEX.md
tui-vfx/docs/tooling/v3-preview-and-thin-player.md
tui-vfx/docs/RECIPE_PROBE_GUIDE.md
```

Current role:

- source of the accepted naming slate
- source of the migration narrative
- place to remove stale future-facing `Preview*` language after the code rename lands
- place to keep historical V2 references accurate when the surrounding text is intentionally historical

### 2.3 `gt-design`: downstream consumer vocabulary and integration points

This repo already uses `RecipePlayback*` vocabulary. It is not the same rename
event as tui-vfx's `Preview*` → `Playback*`, but it is relevant because the
compatibility plan must not collide with the downstream names that already
exist.

Representative files:

```text
gt-design/crates/gtd-ratatui/src/recipes/types.rs
gt-design/crates/gtd-ratatui/src/recipes/player.rs
gt-design/crates/gtd-ratatui/src/recipes/planner.rs
gt-design/crates/gtd-ratatui/src/recipes/mod.rs
gt-design/crates/gtd-ratatui/src/prelude.rs
gt-design/crates/gtd-ratatui/src/lib.rs
gt-design/examples/motion_lab/cls_lab_state.rs
gt-design/examples/v2recipes_lab/*
gt-design/docs/api/generated/public/l4/10_ratatui.md
```

Current role:

- downstream UI/runtime code already names its own plan/player abstractions
- any new tui-vfx aliases should avoid creating ambiguous duplicate terms in the downstream consumer surface
- this repo is a compatibility-check bucket, not the primary rename bucket

## 3. Compatibility / re-export plan

- **Schema surface:** define the real `Vfx*` types in `tui-vfx-recipes`, then keep `pub use Vfx* as Ra*` aliases during cutover. Do not change serde field names unless a separate schema decision says to.
- **Playback seam:** define the real `PlaybackPlan`, `PlaybackController`, and `PlaybackRecipeBridge` names, then keep old `Preview*` exports as compatibility shims until examples, tests, tooling, and generated docs move.
- **Snapshot/buffer path:** define `V3FrameSnapshot` as the canonical frame DTO and `render_v3_frame_to_buffer` as the canonical adapter helper. Keep `DirectV3PreviewSnapshot` and `render_direct_v3_snapshot` as wrappers or aliases for the transition window.
- **Module path:** move the canonical implementation toward `src/playback/`, but keep `src/preview/` as a compatibility module until downstream imports are updated.
- **Thin-player surface:** `tui-vfx-player` is currently a documentation target, not a live binary name in these repos. Any early CLI should reuse the canonical playback/frame APIs instead of inventing a second preview interpreter.
- **Downstream consumers:** keep `gt-design` on its existing `RecipePlayback*` names unless a concrete compile or docs issue proves that a follow-on rename is needed.

## 4. Risk order and next execution slices

1. **Bucket A / `tui-vfx-recipes` schema surface** — highest risk, broadest fan-out.
2. **Bucket A / `tui-vfx-recipes` preview/playback seam** — rename the live seam and the direct V3 frame DTO/helper family.
3. **Bucket B / `tui-vfx` docs and tooling refs** — refresh the decision docs, punch list, and thin-player docs after the code names settle.
4. **Bucket C / `gt-design` compatibility check** — adjust only if the canonical names create confusion or a real compile/docs issue.
5. **Thin-player packaging** — decide whether the first live `tui-vfx-player` implementation is a docs-only placeholder or a small binary wrapper after the seam names are stable.

### Next execution slice recommendation

Start with the `tui-vfx-recipes/src/recipe_schema/**` bucket, especially
`src/recipe_schema/config.rs`, `src/recipe_schema/mod.rs`, and the `scene/`
modules. That lands the `Vfx*` public schema foundation first and keeps the
later `Preview*` → `Playback*` rename anchored on the accepted wire-format
vocabulary.

## 5. Current target-name absence

Exact searches found no live definitions in the target repos yet for:

- `PlaybackPlan`
- `V3FrameSnapshot`
- `render_v3_frame_to_buffer`
- `tui-vfx-player`
- `PlaybackController`
- `PlaybackRecipeBridge`

Implication:

- the rename will introduce these names rather than reconcile existing duplicate definitions
- docs already carry the accepted targets, but code still needs the actual definitions
- `PlaybackPlan` is the canonical loaded/renderable unit in the decision slate, not a synonym for the downstream `RecipePlaybackPlan` in gt-design

## 6. Notes on the current inventory read

- Archive paths such as `docs/v2-spec-archive/**` and the retired notes in `recyclebin/**` may keep historical `Ra*` / `Preview*` wording.
- `PlaybackPlan` is accepted in the naming slate, but its eventual relationship to `CompiledRecipePlan` should be documented when the code rename lands.
- The accepted `Preview*` → `Playback*` cutover is still a rename plan, not a V2 retirement plan.

<!-- <FILE>docs/design/tui-vfx-v3-naming-implementation-inventory.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
