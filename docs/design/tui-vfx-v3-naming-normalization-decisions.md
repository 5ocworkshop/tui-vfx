<!-- <FILE>docs/design/tui-vfx-v3-naming-normalization-decisions.md</FILE> - <DESC>Accepted V3 naming normalization decisions for public schema, playback seams, timing, motion, and routing vocabulary.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Record accepted project-owner decisions that normalize V3 names around Vfx, Playback, frame snapshots, duration/offset timing, route/dynamics motion, intent vocabulary, and recipe metadata naming.</WCTX> -->
<!-- <CLOG>0.2.0: align recipe metadata naming with the accepted Q#21 policy: use intent_hints, visual_tags, and expected_visual rather than required use_cases/aesthetic_tags. 0.1.0: initial accepted naming slate, closing the active naming-normalization decisions for V3 planning and giving implementation lanes canonical target names.</CLOG> -->

# V3 naming normalization decisions

This document records accepted V3 naming decisions. Use it as the canonical
naming target when updating schema docs, rustdocs, generated docs, examples,
recipe authoring guides, and migration notes.

## Principles

- Use names that describe the architectural role, not the historical demo use.
- Keep `Vfx*` for public/wire-format V3 types.
- Reserve preview/demo language for human preview tools, not the canonical recipe
  playback seam.
- Prefer timing words that distinguish duration from timeline placement.
- Keep motion geometry separate from per-cell pipeline effects.
- Use `intent` for routing/hosting hints to avoid collision with render roles.

## Accepted canonical names

| Area | Old / question | Accepted canonical name | Decision |
|---|---|---|---|
| Public/wire types | `Ra*` | `Vfx*` | Use `Vfx*` for V3 public/wire-format names. Keep `Ra*` only as hidden/deprecated aliases during cutover where needed. |
| Canonical loaded/renderable unit | `PreviewItem`, `RecipeItem`, `PlaybackItem` candidates | `PlaybackPlan` | The seam object is a load-ready/renderable plan, not merely a preview item. |
| Stateful playback owner | `PreviewManager` | `PlaybackController` | Use when the type owns time, state, scrubbing, or frame advancement. If a future type only stores plans, use `PlaybackRegistry`. |
| Module path | `src/preview/` for canonical seam | `src/playback/` | Keep demo/preview examples named preview/demo where accurate; rename the engine seam. |
| Rendered frame | `DirectV3PreviewSnapshot` | `V3FrameSnapshot` | A frame/grid snapshot can serve preview, probe, movie, CI, and static export surfaces. |
| Buffer adapter helper | `render_direct_v3_snapshot` | `render_v3_frame_to_buffer` | Adapter-boundary function name should state that it renders one frame snapshot into a buffer. |
| Thin recipe player | `thin player`, possible movie naming | `tui-vfx-player` | The small CLI/tooling layer is a player, not the deferred movie composer. |
| Future scripted movie layer | `gtd-movie` / `tui-vfx-movie` | `gtd-movie` | Reserve for higher-order scripted multi-scene movies/timelines above recipes. |
| Lifecycle duration | `auto_dismiss_ms` | `duration_ms` | Neutral across toast, splash, ambient, modal, and movie-beat contexts. |
| Timeline stagger | `enter_delay_ms`, `exit_delay_ms` | `enter_offset_ms`, `exit_offset_ms` | Offset means placement on a timeline. Delay sounds like blocking duration. |
| Persistent execution | `continuous` block | `phase: "all"` plus explicit `clock` / `timing` | Avoid a separate continuous mode. Multi-phase execution is phase membership plus clock policy. |
| Placement | `anchor` as top-level concept | `placement` object with `anchor` strategy | Keep `anchor` where it names the placement strategy. The larger concept is placement. |
| Motion path | `motion_path` | `motion.{enter,exit}.route` | Route is the carrier path. It is distinct from dynamics/treatments. |
| Motion treatments | mixed `PathType`/effect wording | `motion.{enter,exit}.dynamics[]` | Dynamics are treatments layered over a route: spring, bounce, pendulum, friction, attractor, etc. |
| Offscreen endpoints | ad hoc `from` / `to` fields | `motion.enter.from`, `motion.exit.to` | Keep endpoint names simple and attach them to the phase-specific motion object. |
| Edge behavior | vanishing/offscreen edge behavior | `edge_crossing` | Covers border and shadow behavior when the moving host crosses a viewport edge. |
| Step routing hint | `RoutingRole` | `StepIntent` | Avoids collision with `RoleTag` and `ThemeRole`; describes why the step exists. |
| Recipe hosting hint | `SurfaceIntent` | `SurfaceIntent` | Keep. It names host/container policy clearly. |
| Recipe metadata discovery hints | `use_cases` | `intent_hints` | Non-authoritative discovery hints only; hosts/manifests own routing and binding. Optional at parse level. |
| Recipe visual tags | `aesthetic_tags` | `visual_tags` | Broader than aesthetics: covers visual family, motion character, and technique tags for search/reference. |
| Recipe visual expectation | — | `expected_visual` | Plain-language visual QA expectation; strongly recommended for debug/reference fixtures. |

## Names to avoid

| Avoid | Use instead | Reason |
|---|---|---|
| `RecipeItem` | `PlaybackPlan` | Too vague; does not convey load-ready playback semantics. |
| `PlaybackItem` | `PlaybackPlan` | `Item` is generic; `Plan` better conveys compiled/load-ready structure. |
| `movie` for the small CLI | `player` | Movie implies scripted multi-scene timeline, which is a deferred higher layer. |
| `delay_ms` for inter-element timeline offsets | `offset_ms` | Delay sounds like duration or blocking behavior; offset is timeline placement. |
| `role` for routing or hosting hints | `intent` | Prevents confusion with per-cell `RoleTag` and theme targeting roles. |

## Canonical V3 vocabulary set

Use this vocabulary when adding or renaming public/schema-bearing V3 surfaces:

- `VfxRecipe` — authoring input recipe shape when a concrete type name is needed.
- `NormalizedVfxRecipe` — canonical normalized form.
- `CompiledVfxRecipe` or `CompiledPlaybackPlan` — compiled execution-ready form.
- `PlaybackPlan` — canonical loaded/renderable unit.
- `PlaybackController` — stateful playback/time/scrubbing owner.
- `PlaybackRegistry` — future plan storage/indexing owner, if needed.
- `V3FrameSnapshot` — one rendered frame/grid snapshot.
- `FrameDiff` — comparison of two rendered frame snapshots.
- `tui-vfx-player` — small recipe playback CLI/tool surface.
- `gtd-movie` — future scripted movie/timeline layer.

## Cutover rules

Use this transition map while cutting over:
[V3 naming implementation inventory](tui-vfx-v3-naming-implementation-inventory.md)

1. Prefer one deliberate V3 naming cutover over repeated piecemeal renames.
2. Use compatibility re-exports where needed during the cutover, but do not teach
   stale names in new docs or examples.
3. Generated docs must use canonical names once the Rust/schema surfaces are
   renamed.
4. Hand-maintained docs may mention old names only in migration notes or
   deprecation tables.
5. V2 and legacy playback stay available until the final V2-retirement gate; the
   naming cutover does not imply V2 removal.

## Plan impact

This closes the active naming-decision portion of:

- Open Q #15 — vocabulary refresh scope, for the names above.
- Open Q #19 — canonical preview/playback seam naming.
- Open Q #18 — routing/hosting hint naming, with `StepIntent` and
  `SurfaceIntent` as canonical terms.
- Chapter 90 movie-composer naming split: `tui-vfx-player` now,
  `gtd-movie` later.

Implementation remains a separate execution lane: rename code, rustdocs,
schema docs, generated docs, recipes, examples, and migration notes in small,
verified stages.

<!-- <FILE>docs/design/tui-vfx-v3-naming-normalization-decisions.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
