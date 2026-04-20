<!-- <FILE>CHANGELOG.md</FILE> - <DESC>Release history for tui-vfx</DESC> -->
<!-- <VERS>VERSION: 1.17.0</VERS> -->
<!-- <WCTX>Sub-plan A Phase A.5 — documentation consistency pass + end-to-end integration test; consolidates A.1-A.4 as one coherent Unreleased section and adds the cross-phase regression net.</WCTX> -->
<!-- <CLOG>1.17.0: PATCH — Phase A.5 consolidation: add "Sub-plan A (recipe scene composer foundation) — final composition check" summary at the head of the Unreleased block; point at the end-to-end integration test and the Sub-plan A handoff doc. No new public API introduced by A.5; existing A.1-A.4 entries preserved in order.
1.16.0: add Unreleased entry for Phase A.4 — tui-vfx-debug inspection module (TraceEvent / TraceEnvelope / TraceSelector / TraceFilter / StageMask / InspectionSink / TraceSink / TraceReport with NDJSON round-trip); additive InspectionSinkBridge in tui-vfx-compositor so CompositorInspector callbacks forward into any InspectionSink; criterion dev-dep + two benches; workspace bumped 0.8.0 → 0.9.0. CompositorInspector stays at crates/tui-vfx-compositor/src/traits/pipeline_inspector.rs.
1.15.0: add Unreleased entry for Phase A.3 — role-aware ShaderContext, ShadowConfig.source_region, fnc_extract_shadow_envelope + CellMask, render_shadow_into_scene, shadow-stage RoleTag::Shadow write-back; workspace bumped 0.7.0 → 0.8.0. ShadowConfig is no longer Copy (RoleTag::Custom carries Arc<str>).
1.14.0: add Unreleased entry for the Phase A.2 hard cutover: render_pipeline* signature change (adds &RoleMap source and &mut SemanticScene destination); StyleRegion legacy bare variants removed from the Rust enum (serde back-compat preserved via custom Deserialize); workspace version bumped 0.6.0 → 0.7.0.
1.13.0: add Unreleased entry for the recipe scene composer foundation primitives shipped in tui-vfx-types 0.6.0.
1.12.0: add Unreleased section covering tui-vfx-content sources (RocketsplashImage/Font + blit helper) and pool primitives (TextPool/EffectPool/ImagePool/FontPool/PresetPool + PoolPolicy). Default-on rocketsplash-rt workspace dep.
1.11.0: add 0.5.0 section covering GlistenBand.speed reconnect + speed_binding, FadeToCanvas.canvas_color_binding and new ShaderRuntimeParamValue::Rgb variant, plus RigidShake.damping_scale_binding.</CLOG> -->

# Changelog

All notable changes to this project will be documented in this file.

This project follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added — shared TraceEmitter stamping authority (Sub-plan B Phase B.1) — workspace 0.11.0

`tui-vfx-debug::inspection` now ships **`TraceFrameContext`** and
**`TraceEmitter`** so frame/time/recipe stamping and `seq_in_frame`
authority live in one shared helper instead of being duplicated inside
individual emit sites. `TraceEmitter` owns an `Arc<dyn InspectionSink>`,
a `RwLock<TraceFrameContext>`, and an atomic `seq_in_frame` counter;
`begin_frame(ctx)` swaps frame context + resets sequence, and `emit(event)`
stamps the current context before forwarding to the sink.

`tui-vfx-compositor::traits::cls_inspection_sink_bridge::InspectionSinkBridge`
now delegates envelope construction to `TraceEmitter` while preserving its
existing public API (`new`, `from_trace_sink`, `begin_frame`, `sink`,
`context`). This centralizes sequence authority for the planned multi-stage
shared-emitter model used by the recipe scene composer.

### Sub-plan A (recipe scene composer foundation) — final composition check (Phase A.5)

All four preceding phase sections (A.1 → A.4) in this Unreleased block
compose as one coherent release. An end-to-end integration test
(`crates/tui-vfx/tests/test_foundation_end_to_end.rs`) exercises every
phase in one scenario: `SemanticScene` with non-trivial role tags
(A.1) → `render_pipeline` with a `Role(Border)` shader (A.2) → shadow
stage with `source_region: Border` writing `RoleTag::Shadow`
write-back (A.3) → `InspectionSinkBridge` capturing pipeline events
into a `TraceSink` with NDJSON round-trip (A.4). No new public API is
introduced by A.5; it is a documentation-consistency pass and a
cross-phase regression net. See the Sub-plan A handoff at
`gt-design/docs/superpowers/handoffs/2026-04-20-sub-plan-A-handoff.md`
for the full public-API surface Sub-plan B can rely on.

### Added — `tui-vfx-debug`, `tui-vfx-compositor` — unified inspection foundation (Sub-plan A Phase A.4) — workspace 0.9.0

`tui-vfx-debug` gains a new **`inspection`** module alongside the existing
logger. It ships the canonical inspection surface defined in the
recipe-scene-composer design spec §9: one sink hook every stage reports
into, one `TraceEvent` taxonomy, and an NDJSON-round-trippable
`TraceReport`.

- **`tui-vfx-debug::inspection::TraceEvent`** — 18 variants across four
  stages (lifecycle / resolution / composition / pipeline);
  `#[non_exhaustive]` so additive variants do not break clients; full
  serde round-trip; `stage()` accessor classifies each variant against
  `StageMask`.
- **`tui-vfx-debug::inspection::TraceEnvelope`** — event plus `frame_no`,
  `t_ms`, optional `recipe_id`, and monotonic `seq_in_frame` counter
  for deterministic replay ordering.
- **`tui-vfx-debug::inspection::TraceSelector`** — `Cell { x, y }`,
  `Rect(Rect)`, `Role(RoleTag)`, `Layer(LayerId)`, `Recipe(RecipeId)`,
  `All`. `#[non_exhaustive]`. Opaque newtypes from `tui-vfx-types` so
  inspection code stays independent of the recipe crate.
- **`tui-vfx-debug::inspection::TraceFilter`** — selectors (OR) +
  `StageMask` (AND) + `Range<u64>` frame range + `Range<u64>` time
  range. `accept_all()` sentinel and `accepts_any_stage()` fast-path
  probe.
- **`tui-vfx-debug::inspection::StageMask`** — bitmask
  (`LIFECYCLE | RESOLUTION | COMPOSITION | PIPELINE`) with `NONE` /
  `ALL` sentinels and `is_empty` for emit-site short-circuit.
- **`tui-vfx-debug::inspection::InspectionSink`** — object-safe
  `Send + Sync` trait with a single `report(&self, TraceEnvelope)`
  method.
- **`tui-vfx-debug::inspection::TraceSink`** — thread-safe, filtered,
  optionally-bounded `InspectionSink` impl. `new(filter)` is
  unbounded; `with_capacity(filter, n)` drops oldest with an explicit
  `dropped` counter.
- **`tui-vfx-debug::inspection::TraceReport`** — envelope list +
  per-stage `TraceReportSummary` + `dropped` counter;
  `to_ndjson(writer)` / `from_ndjson(reader)` methods (placement per
  peer-review S12).

The compositor ships a **bridge**, not a trait move:

- **`tui-vfx-compositor::traits::cls_inspection_sink_bridge::InspectionSinkBridge`**
  implements `CompositorInspector` and forwards every pipeline-stage
  callback into a registered `InspectionSink` as a full
  `TraceEvent`. `TraceFrameContext` supplies frame/time/recipe
  context; `begin_frame(ctx)` resets `seq_in_frame` at each frame
  boundary.
- **`CompositorInspector` stays in `tui-vfx-compositor`** at
  `crates/tui-vfx-compositor/src/traits/pipeline_inspector.rs`.
  Existing direct implementors (`ProbeInspector`, `StageInspector`,
  `TraceInspector`) continue to work unchanged. The bridge is the
  additive forwarding path, not a replacement.

Benches (criterion, `cargo bench -p tui-vfx-debug`):

- `bench_emit_overhead` — contrasts `StageMask::NONE` short-circuit
  against fully-accepted emit. Target: < 1µs per emit at NONE.
- `bench_full_trace_60fps` — ~11k events per representative frame
  (80×24, 4 layers, full pipeline). Target: ≤ 2ms/frame at 60fps.

### Added / Changed — `tui-vfx-style`, `tui-vfx-shadow`, `tui-vfx-compositor` — style + shadow role-awareness (Sub-plan A Phase A.3) — workspace 0.8.0

Shaders and the shadow stage now consume per-cell role information, and
the shadow stage tags its output cells with `RoleTag::Shadow` in the
destination role map so downstream trace consumers and subsequent
pipeline passes can address shadow cells by role.

- **`tui-vfx-style::ShaderContext`** gains a `roles: Arc<RoleMap>` field
  (additive). Struct-literal construction sites must add `roles: <arc>`;
  `ShaderContext::new(...)` callers are source-compatible (the field
  defaults to `Arc::default()` i.e. a 0×0 empty `RoleMap`). New builder
  `with_roles(self, roles)` and accessor `role_at((x, y))` round out the
  ergonomic surface.
- **`tui-vfx-shadow::ShadowConfig`** gains `source_region: Option<RoleTag>`
  plus `with_source_region(role)` builder and `source_region()` accessor.
  When `None` (default), shadow extrusion is rectangular as before.
  When `Some(role)`, the shadow stage extrudes from the **tight bounding
  rectangle of role-matched source cells** — fixing the "shadow on text
  rect" splash bug where a text-only card would otherwise cast shadow
  from its text cells. Serde round-trips both shapes; legacy recipes
  without the field continue to parse unchanged.
- **`ShadowConfig` is no longer `Copy`.** `RoleTag::Custom` carries an
  `Arc<str>` (via `InternedRoleName`), making `ShadowConfig` non-Copy
  when it carries a custom role. Call-sites that relied on the implicit
  copy (`let b = a;`) must now `.clone()` explicitly. All internal call
  sites have been migrated.
- **New: `tui-vfx-shadow::extract_shadow_envelope`** (pure function) +
  `CellMask` primitive. Returns the set of source cells that should
  extrude shadow, honouring the `source_region` filter. Provides
  `bounding_rect()` for callers that want a single rectangle derived
  from the mask.
- **New: `tui-vfx-shadow::render_shadow_into_scene`** entrypoint that
  renders a shadow into a `&mut SemanticScene` and tags every produced
  cell with `RoleTag::Shadow` in `destination.roles_mut()`. Wraps
  `render_shadow` with role-filtered extrusion + destination tag
  write-back.
- **`tui-vfx-compositor::render_pipeline_with_shadow`** promotes its
  destination from `&mut dyn Grid` to `&mut SemanticScene` so the
  compositor-internal shadow path can write `RoleTag::Shadow` into the
  destination role map. Shadow extrusion now honours
  `config.source_region` via `extract_shadow_envelope`.
- **Stage ordering clarification:** the per-cell pipeline stages run
  Sampler → Mask → Shader → Filter → Shadow → Final. Shadow runs AFTER
  filters (unchanged from A.2). The `RoleTag::Shadow` tags the shadow
  stage writes into the destination role map are observable
  immediately via `destination.roles()` and in `tui-vfx-trace`;
  downstream filters in the same frame do NOT see them (their stage
  already ran). Spec §8.3 "downstream filter can target shadow cells
  specifically" refers to a subsequent frame, a subsequent pipeline
  pass, or future re-ordering.
- **Tiny-rectangle safety:** 1×1, 2×1, and 0×0 source cases handled
  without panic at every layer (envelope extraction, shadow render,
  compositor integration).

### Migration notes

- **Consumers building `ShaderContext` via struct literal** must add
  `roles: <arc>` to the literal. The simplest stand-in is
  `Arc::<RoleMap>::default()` (empty 0×0 map). Callers using
  `ShaderContext::new(...)` are source-compatible — the new field
  defaults internally.
- **Consumers of `ShadowConfig`** relying on implicit copy must add
  `.clone()`. Lint hint: `cargo build` will report move errors with
  exact locations.
- **Shadow cells in the destination role map:** consumers that read
  destination roles after a shadow pipeline pass will observe
  `RoleTag::Shadow` in place of the previous default (typically
  `Background`) wherever the shadow stage wrote a cell. This is the
  intended observable change; back-compat is preserved for the
  pixel content itself (bit-for-bit match against pre-A.3 output when
  `source_region` is `None`).

### Changed — BREAKING — `tui-vfx-compositor` — role-aware pipeline (Sub-plan A Phase A.2) — workspace 0.7.0

The compositor's public render entrypoints now consume role-aware inputs
so downstream stages (shaders, shadow extrusion) can target cells by
semantic role rather than guessing from position or glyph content.

- **`render_pipeline`**, **`render_pipeline_with_area`**,
  **`render_pipeline_with_spec`**, and **`render_pipeline_with_spec_area`**
  all now take `source_roles: &RoleMap` immediately after `source`, and
  promote the destination from `&mut dyn Grid` to `&mut SemanticScene`.
  The internal per-cell hot loop evaluates
  `StyleRegion::Role(RoleTag)` predicates by reading
  `source_roles.get((x, y))`.
- **Migration helper:** call-sites without semantic information should
  construct `RoleMap::all_background(width, height)` for the source
  roles and `SemanticScene::from_grid_with_default_role(grid, RoleTag::Background)`
  for the destination scene. Both are one-liner, zero-extra-allocation
  in the common case (the `RoleMap` stores a single interned ID per cell).
- **`StyleRegion`** gains a `Role(RoleTag)` variant for role-based
  targeting. The legacy bare variants `BorderOnly`, `TextOnly`, and
  `BackgroundOnly` have been **removed from the Rust enum**. Existing
  recipe JSON fixtures that still write the legacy strings continue to
  parse correctly — a custom `Deserialize` impl maps each legacy
  string to its canonical `Role(RoleTag::…)` form. Serialization always
  emits the canonical form; the schema converges on round-trip.
- **Extractions** (OFPF size-budget pre-work):
  `StyleRegion::should_style` and `StyleRegion::bounding_rect` are now
  thin delegators to `fnc_style_region_should_style::should_style` and
  `fnc_style_region_bounding_rect::bounding_rect` respectively. The
  method signatures have changed to carry the role context:
  `should_style(x, y, role: Option<RoleTag>, area: Rect) -> bool` and
  `bounding_rect(area: Rect) -> Option<Rect>`. Two legacy shims
  (`should_style_legacy(x, y, w, h)` and `bounding_rect_legacy()`)
  lift the old call-shape onto the new API for call-sites that don't
  yet have role information — they pass `None` for role and a
  zero-origin area.
- **`SemanticScene::grid_mut()`** added so pipeline stages can hand a
  `&mut dyn Grid` (the concrete `OwnedGrid`) to internal helpers that
  still speak `Grid`.

Workspace-level version bumped 0.6.0 → 0.7.0. Downstream crates
(tui-vfx-recipes, gt-design) must migrate their render_pipeline
call-sites in the same release — A.2 includes the gt-design mechanical
compile-fix so the workspace continues to build.

### Added — `tui-vfx-types` — recipe scene composer foundation (Sub-plan A Phase A.1)

Additive foundation primitives that Sub-plans B and C build on for the
unified recipe scene composer (see gt-design
`docs/superpowers/specs/2026-04-20-recipe-scene-composer-design.md`):

- **`SemanticScene`** — source surface combining `OwnedGrid`, `RoleMap`,
  and `SceneMetadata`. Accessor parity with `ratatui::Buffer`: `area()`,
  `cell((x, y))`. Plus `role((x, y))`, `roles()`, `roles_mut()`.
  Constructors: `new(grid, roles)` (panics on dimension mismatch — a
  documented library-misuse panic), `with_metadata(metadata)` builder,
  and `from_grid_with_default_role(grid, default)` migration helper.
- **`RoleMap`** — dense per-cell `RoleTag` storage. Row-major
  `Vec<RoleId>` layout, bounds-checked `get` / `set` (silent no-op out
  of bounds; no panic), row-major `iter()` yielding `(x, y, RoleTag)`,
  serde round-trip via `cfg_attr`. Constructors: `empty(w, h)`,
  `all_background(w, h)` (alias), `new_with_default(w, h, default)`.
- **`RoleTag`** — `#[non_exhaustive]` enum with 12 first-class variants
  (`Background`, `Text`, `Title`, `Caption`, `Border`, `Image`, `Icon`,
  `Indicator`, `Highlight`, `Shadow`, `Decoration`, `Procedural`) and
  `Custom(InternedRoleName)`. `from_shorthand("border")`-style parsing;
  `shorthand_name()` round-tripper; `RoleTag::FIRST_CLASS` const array
  for iteration.
- **`RoleInterner` / `RoleId`** — compact numeric IDs with stable
  assignment: first-class variants reserve IDs 0–11 in the declaration
  order of `RoleTag::FIRST_CLASS`, Custom IDs start at 12 and grow
  monotonically. Every `RoleMap` owns its interner.
- **`LayerId` / `RecipeId`** — opaque interned newtypes used by future
  trace selectors / inspection sinks without forcing downstream
  inspection code (`tui-vfx-debug`) to depend on `tui-vfx-recipes`.
- **`InternedString`** — cheap-to-clone `Arc<str>` wrapper; equality by
  content; shared `empty()` sentinel to minimize allocation on defaults.

No behavior change yet — these are foundation types. Pipeline
signatures are migrated in Phase A.2; shader and shadow stages in
Phase A.3.

### Added — `tui-vfx-content`

- **Rocketsplash source primitives** (`sources/` submodule): `RocketsplashImage` loads `.rss` byte payloads via the co-designed `rocketsplash-rt` sister project; `RocketsplashFont` loads `.rsf` font atlases with a chainable `FontRender` builder (color / gradient / shadow / style / spacing / align / fallback). Both terminate in `blit_into_grid()` so rocketsplash cells compose with every downstream VFX primitive (shadows, wipes, glisten, filters). Shared `blit_render_buffer_to_grid()` helper maps `RenderCell` → `tui_vfx_types::Cell` honoring opacity and clipping. `rocketsplash-rt` is a default workspace dep — no feature flag.
- **General-purpose content-randomization pools** (`pool/` submodule):
  - `PoolPolicy`: `Random` (time-seeded, no `rand` crate dep) or `FirstOnly` (deterministic, test-friendly).
  - `TextPool`: strings, sanitized on construction (control bytes stripped).
  - `EffectPool`: `ContentEffect` values rotated per launch.
  - `ImagePool` / `FontPool`: rocketsplash asset-map keys (name references, not inline bytes — caller owns distribution via `AssetMap`).
  - `PresetPool` of `Preset`: curated `(text, effect, image_name, font_name)` bundles; `Preset` fields are all `Option` so Phase 2 extensions (speaker, shader / shadow overrides) are additive.
  - All pool types derive `ConfigSchema` + serde for recipe-schema introspection and JSON round-trip.

### Notes

- Splash taglines are one use case for the pool primitives; dialog systems, game NPC variety, and error-message rotation are others. Placed in `tui-vfx-content` (not any specific downstream crate) so every tui-vfx consumer benefits without pulling the whole GT stack.

## 0.5.0 — 2026-04-14

This release closes out the **three outstanding shader/filter bindings**
that were called out inside Phase 0 P0.3, P0.4, and P0.5 but partially
shipped in 0.4.0. The gt-design prompt #157 "Outstanding P0 shader
bindings" subsection tracks these as O-P0.A, O-P0.B, and O-P0.C. After
this release Phase 0 engine prerequisites are complete — every field
the runtime-binding story needs is wired, documented, fixture-covered,
and released. Additive minor version bump: every recipe that parsed
against 0.4.0 parses against 0.5.0 unchanged, and the three new
`*_binding` fields are all `Option<String>` with `skip_serializing_if`.

### Added

- **`GlistenBandShader.speed_binding: Option<String>`** (O-P0.A).
  Drives the band sweep rate from a runtime f32 parameter, clamped
  `0.1..=10.0` to prevent degenerate sweep rates (below 0.1 the band
  stalls visibly, above 10.0 it flickers). Companion to the
  existing `direction_binding` and `blend_strength_binding` so apps
  can modulate sweep rate, direction, and intensity from the same
  runtime params map (e.g. accelerate on hover, slow during dwell).
- **Reconnected `GlistenBandShader.speed` field** (O-P0.A
  prerequisite). The `speed` field has been a no-op since v2.0.1
  when the sweep was refactored to be driven purely by `ctx.t`; this
  release reconnects it as a scalar multiplier on the `t`-based
  sweep (`scaled_t = t * effective_speed`) so recipes that declared
  `speed != 1.0` before 0.5.0 now render at the rates their authors
  originally intended. **Visual-behavior change for six existing
  recipes** that used non-default speed values but rendered
  identically regardless: `skeleton_shimmer.json` (0.7, now slower),
  `update_shimmer_ribbon.json` (0.8, now slower), `glisten_band_directions.json`
  (1.5, now faster), `shader_glisten_band*.json` fixtures (1.5), and
  `glisten_white_band_directions.json` (3.0, much faster). This is a
  fix, not a regression — the declared values now mean what they
  say.
- **`FilterSpec::FadeToCanvas.canvas_color_binding: Option<String>`**
  (O-P0.B). Drives the exit-fade target color from a runtime
  parameter at prepare time instead of baking a static
  `ColorConfig` into the recipe. Missing bindings and non-Rgb kinds
  fall through to the declared `canvas_color`. Typical use: an app
  that toggles dark/light mode at runtime reports the current
  terminal background via `current_terminal_bg` runtime params so
  every toast's exit fade tracks the new backdrop without needing
  per-theme recipe variants.
- **`ShaderRuntimeParamValue::Rgb { r: u8, g: u8, b: u8 }` variant**
  (O-P0.B prerequisite). Runtime params previously carried only
  `Integer`, `Float`, `Boolean`, and `Text` — binding a `Color`
  required widening the enum. The new trailing `Rgb` variant
  deserializes from `{"r": <u8>, "g": <u8>, "b": <u8>}` JSON objects
  and does not collide with the existing scalar variants under
  `serde(untagged)`. New `ShaderRuntimeParamValue::as_color()` and
  `ShaderRuntimeParams::get_color(key)` accessors lower the binding
  resolution path for any filter that needs a runtime Color. New
  `From<tui_vfx_types::Color> for ShaderRuntimeParamValue` impl so
  apps can `params.insert("bg", my_color)` without naming the
  variant directly.
- **`FilterSpec::RigidShake.damping_scale_binding: Option<String>`**
  (O-P0.C). Drives a single scalar multiplier over the 8-entry
  `damping` curve so apps can tighten or loosen the shake decay at
  runtime without authoring N recipe variants. Resolved as `f32`,
  clamped `0.1..=10.0`, and multiplied element-wise into every
  element of the curve at prepare time. Recommended pairing with
  the existing `num_shakes_binding` to drive both shake count AND
  decay rate from the same severity source: warning → (1 shake,
  scale 0.5), error → (4 shakes, scale 1.0), critical → (8 shakes,
  scale 2.0). Missing bindings leave the declared damping curve
  unchanged.

### Changed

- **workspace: crate version `0.4.0` → `0.5.0`.**
- **Internal workspace dep floors bumped from `0.4.0` to `0.5.0`**
  across `tui-vfx-types`, `tui-vfx-core`, `tui-vfx-core-macros`,
  `tui-vfx-geometry`, `tui-vfx-compositor`, `tui-vfx-style`,
  `tui-vfx-content`, `tui-vfx-shadow`, and `tui-vfx-debug`.
- **Generated docs regenerated** via `cargo xtask docs generate` to
  pick up the new `canvas_color_binding` and `damping_scale_binding`
  fields. `cargo xtask docs check` is clean with zero warnings.
  `GlistenBand.speed_binding` and the reconnected `speed` field did
  not need doc regeneration because they are struct-level additions
  that the capabilities.toml already listed in the key-parameters
  inventory.

### Deferred

Nothing. After this release every Phase 0 engine prerequisite in
prompt #157 is fully landed. The `btop_focused_row_dynamic_list.json`
reproducibility acceptance bullet remains deferred to Phase 5 per
the prompt — that is a recipe-authoring item, not an engine gap.

### Migration notes

Consumers compiling against the `ShaderRuntimeParamValue` enum must
handle the new `Rgb` variant in match arms that use the non-exhaustive
`match` form (or add `_ => ...` fallbacks). The existing
`kind_name()`, `as_f32()`, `as_u16()`, `serde_json::Value::from`,
and `ShaderRuntimeParams::get_f32`/`get_u16` accessors all degrade
gracefully — `as_f32` and `as_u16` return `None` for `Rgb`, which
matches their existing semantics for `Boolean`/`Text`.

## 0.4.0 — 2026-04-14

This release lands the **Phase 0 binding generalization** for gt-design's
dynamic recipe story: filter parameters, cell-region coordinates, shader
parameters, and select integer counters can now resolve from a
`ShaderRuntimeParams` map at render time, letting apps drive effect
parameters from live widget state (scroll progress, hover index, error
severity, etc.) instead of static literals.

Every addition is **backwards-compatible**: existing recipe JSON that uses
raw number literals continues to parse and render unchanged. The binding
surface is opt-in per field via a new `{"binding": "key"}` tagged form.

### Added

- **tui-vfx-compositor: `BindableValue` wrapper type** for filter-spec f32
  fields. Lives in `tui_vfx_compositor::types::BindableValue` and wraps
  `mixed_signals::SignalOrFloat` with a new `Binding(String)` variant. The
  lenient deserialize accepts four JSON shapes — raw number, tagged
  `{"signal": ...}`, tagged `{"binding": "key"}`, or bare `SignalOrFloat`
  — so existing recipes that emit `"progress": 0.5` keep working. Serialize
  normalizes to the tagged form.
- **tui-vfx-compositor: `PrepareContext` bundle** (`pipeline::cls_prepare_context`)
  carrying `loop_t`, `SignalContext`, and a borrowed `&ShaderRuntimeParams`
  per frame. Passed into `prepare_filter` / `prepare_filters` so filter
  spec arms can resolve runtime bindings without adding new parameters to
  the filter call signature.
- **tui-vfx-compositor: `progress_binding` support on 9 filters.** The
  `progress` field on `KittScanner`, `ShadeScanner`, `GlistenSweep`,
  `HoverBar`, `SubPixelBar`, `UnderlineWipe`, `PillButton`,
  `BracketEmphasis`, and `DotIndicator` is now a `BindableValue`. Apps can
  drive progress from a live runtime param via
  `"progress": {"binding": "scroll_progress"}`. Missing bindings fall back
  to 0.0 (inactive state).
- **tui-vfx-compositor: `FadeToCanvas` filter** (`filters::cls_fade_to_canvas`)
  — the sanctioned replacement for the `tint(black, 0.7+)` exit hack. Blends
  cells toward a declared `canvas_color` at caller-controlled strength,
  avoiding the dark-flash artifact on light canvases. Strength is
  `BindableValue`-typed so apps can drive it from exit animation progress.
  5 unit tests including a regression guard that fails if mid-fade values
  are darker than both widget and canvas.
- **tui-vfx-compositor: `RigidShake.num_shakes_binding`** lets apps drive
  shake count from severity/error-level state (warning → 1 shake, error →
  4, critical → 8). Resolved u16 is saturating-cast to u8 and further
  clamped to the filter's 0–8 hard cap.
- **tui-vfx-style: `BindableU16` wrapper type** for u16 cell-coordinate
  values (`models::cls_bindable_u16`). Parallel to `BindableValue` but
  scoped to integer positions. Lives in tui-vfx-style rather than the
  compositor because `StyleRegion` lives there and the style crate is the
  lowest layer that already owns `ShaderRuntimeParams`.
- **tui-vfx-style: `StyleRegion::Cell.x` / `.y` lifted to `BindableU16`.**
  A single-cell style region's coordinates can now be a runtime binding —
  the primitive powering the HLL modal hover-bar slide-between-buttons
  pattern and any future "indicator slides between N positions" effect.
  `StyleRegion::resolved(&runtime_params) -> Cow<'_, Self>` lowers a
  binding-bearing `Cell` to concrete literals once per layer per frame
  (zero clone for every bindless region via `Cow::Borrowed`). The render
  pipeline in `orc_render_pipeline::apply_shaders` resolves before calling
  `should_style`.
- **tui-vfx-style: shader `*_binding` generalization.** Mirroring the
  existing `FocusedRowGradient::selected_row_binding` pattern, adds:
  - `BorderSweepShader.position_binding` — normalized 0.0–1.0 override
    for the bead's perimeter position.
  - `GlistenBandShader.direction_binding` — u16-coded direction override
    (`0=Forward`, `1=Reverse`, `2=PingPong`).
  - `GlistenBandShader.blend_strength_binding` — f32 override clamped to
    0.0–1.0.
  - `PulseWaveShader.frequency_binding` — f32 override; `blend_at`
    signature gained an explicit `frequency` parameter so `style_at`
    resolves once per frame rather than per cell.
- **docs/templates/capabilities.toml: `FadeToCanvas` capability entry**
  with `calm` energy, `simple` complexity, and
  `exit_transitions` / `dismissal` / `modal_close` use cases. Generated
  API docs pick this up through `cargo xtask docs generate`.

### Changed

- **workspace: version 0.3.0 → 0.4.0.** All workspace members pick up the
  bump via `version.workspace = true`. Workspace.dependencies internal
  crate specs were updated to `version = "0.4.0"` alongside.
- **tui-vfx-compositor: `prepare_filter` / `prepare_filters` signature.**
  Replaced the loose `(loop_t: f64, signal_ctx: &SignalContext)` parameter
  pair with a single `&PrepareContext` argument. Zero behavior change —
  filter arms see the same `loop_t` and `signal_ctx` via shadow bindings.
- **tui-vfx-style: `GlistenBandShader.speed` field.** Deferred adding a
  `speed_binding` counterpart because the underlying `speed` field has
  been a no-op since v2.0.1 (removed from positional computation in favor
  of `loop_t`-driven sweep). A binding over a vestigial field would be
  misleading. Noted on the struct field and will revisit when the speed
  field is reconnected.

### Fixed

- **tui-vfx-compositor: dark-flash exit bug on light canvases.** Recipes
  that previously used `tint(black, 0.7+)` as their exit filter can
  migrate to `FadeToCanvas` with their app's actual terminal background.
  The new filter's inline test suite pins the regression: mid-fade values
  are never darker than both the widget color and the canvas color.

### Notes

- Damping array binding for `RigidShake` is intentionally deferred pending
  a scalar-multiplier design (scale the whole `[f32; 8]` curve uniformly).
  Not needed for the severity-driven shake count use case the P0.5 prompt
  calls out; flagged for a follow-up.
- Canvas-color runtime binding (so themes can update
  `FadeToCanvas.canvas_color` live without recompiling) is deferred to a
  future P0 pass. Today's fix lands the filter and the flash-fix with a
  static canvas color.

## 0.3.0 — 2026-04-11

### Added
- **tui-vfx-content:** Added `ContentEffect::apply(target, progress) -> String` — a one-call ergonomic entry point that hides the `get_transformer` dispatcher, the `SignalContext`, and the `Cow` unwrap for the common static-progress case. Collapses three lines of boilerplate into one.
- **tui-vfx-content:** Added `ContentEffect::apply_to_borrowed(target, progress) -> Cow<'_, str>` preserving the zero-allocation fast path that the underlying `TextTransformer::transform` method provides. Use this when you care about avoiding `Cow::into_owned` in the no-op case (e.g. Typewriter at progress `1.0` returning the full target as a borrowed slice).
- **tui-vfx-content:** Added `ContentEffect::apply_with_context(target, progress, &ctx) -> Cow<'_, str>` — the advanced-use entry point for signal-driven pacing with a caller-supplied `SignalContext`.
- **tui-vfx-content:** Added `TypewriterCursor::simple(glyph)` and the convenience presets `block()` (█), `underscore()` (_), `pipe()` (|), and `caret()` (▌). Each preset is a thin wrapper over `Self::default()` that swaps the character field, so the rest of the `SignalOrFloat` shape is preserved exactly.
- **tui-vfx-content:** Crate-level rustdoc gained Quick start, Static vs signal-driven parameters, and Cursor presets sections explaining when to reach for `SignalOrFloat::Static(n)` vs the dynamic variants.

### Changed
- **workspace:** Bumped workspace version from `0.2.6` to `0.3.0` to reflect the additive `tui-vfx-content` public API additions. All other crates are republished at the new version unchanged.

### Notes
- Every change in this release is **additive** to the public API. The existing `get_transformer` + `TextTransformer::transform` + `SignalContext` path is unchanged and remains the canonical advanced API. No call sites need to migrate.

## 0.2.6 — 2026-03-17

### Added
- **tui-vfx-compositor:** Added `CharsetNoise` filter — non-converging time-varying character replacement operating at the compositor level. Replaces cell characters from a position-aware charset gradient that changes over time. Lives in the filter pipeline alongside braille_dust, tint, dim, etc., so every consumer (factory, recipe preview, any future consumer) gets it automatically. Supports vertical gradient of charsets, per-cell jitter, and desynchronized timing. JSON: `{ "type": "charset_noise", "hz": 8.0, "seed": 42, "jitter": 0.15, "gradient": [...] }`.

### Changed
- **tui-vfx-content:** **BREAKING:** Removed `CharsetNoise` variant from `ContentEffect` enum. CharsetNoise is now a compositor filter (`FilterSpec::CharsetNoise`), not a content transformer. Recipes should use `pipeline.filter.dwell` instead of `content.effect`. The old `cls_charset_noise.rs` and `cls_charset_noise_config.rs` moved to recyclebin.

### Fixed
- **tui-vfx-compositor:** Fixed `CharsetNoise` filter time_step calculation — was dividing by 1000 (leftover from content transformer which received milliseconds). Compositor filters receive normalized t (0–1). Matches braille_dust's pattern.

## 0.2.5 — 2026-03-17

### Added
- **tui-vfx-content:** Added `CharsetNoise` content transformer — non-converging, time-varying character replacement with vertical gradient support. Unlike Scramble (which resolves toward target text), CharsetNoise cycles indefinitely, replacing characters from a configurable charset at a given hz rate. Supports position-aware charset gradients (sparse characters at top, dense at bottom) and per-cell jitter for organic variation. Designed for living textures: fire, rain, smoke, static noise. Including empty characters (like `⠀`) in sparse pools creates flickering shape boundaries.
- **tui-vfx-content:** Added `GradientStop` type for charset gradient configuration and `AffectMode` enum (`all` / `non_empty`) controlling which cells are replaced.
- **tui-vfx-content:** Added `CharsetNoise` variant to `ContentEffect` enum with full serde support (`"type": "charset_noise"` in JSON).
- **tui-vfx-compositor:** Added `Gravity` sampler — parabolic acceleration displacement (`0.5 * a * t²`) with terminal velocity cap. Positive acceleration = fall down/right, negative = rise up/left. Useful for falling text, rising smoke, drop-in entrances, and debris effects. JSON: `{ "type": "gravity", "axis": "y", "acceleration": 6.0, "terminal_velocity": 12.0 }`.
- **tui-vfx-compositor:** Added `drift` field to `BrailleDust` filter — shifts particle hash query position over each lifecycle step, faking gravity (positive = fall) or buoyancy (negative = rise) without per-particle state. JSON: `"drift": 2.0`.

### Fixed
- **tui-vfx-compositor:** `BrailleDust` filter now recognizes empty braille `⠀` (U+2800) as empty alongside whitespace, so dust particles correctly appear in braille art content that uses `⠀` for empty space.
- **tui-vfx-compositor:** `BrailleDust` particles now appear at staggered times — per-cell time offset desynchronizes step transitions so particles don't all flash in unison.
- **tui-vfx-compositor:** `BrailleDust` particles now fade in/out smoothly — foreground color is dimmed by a `sin(π * progress)` bell curve envelope over each lifecycle step, replacing the previous binary snap on/off.

## 0.2.4 — 2026-03-13

### Added
- **tui-vfx-shadow:** Added `ShadowCompositeMode` enum with `GlyphOverlay` (default, backward-compatible) and `GradeUnderlying` (destination-preserving color grading) variants.
- **tui-vfx-shadow:** Added `ShadowGradeConfig` struct with per-channel dim, desaturate, and tint strength controls for fine-tuned grade-underlying shadows.
- **tui-vfx-shadow:** Added `ShadowConfig::with_composite_mode()`, `.with_grade()`, and `.with_dramatic_grade()` builder methods.
- **tui-vfx-compositor:** Added `fnc_grade_shadow_cell` — implements the grade-underlying algorithm (desaturate → dim → tint) that preserves destination glyphs and modifiers while applying color grading scaled by shadow coverage.
- **tui-vfx-compositor:** Pipeline branches on `ShadowCompositeMode`: `GlyphOverlay` uses the existing `blend_shadow_cell`, `GradeUnderlying` uses the new `grade_shadow_cell`.
- **tui-vfx (prelude):** Re-exported `ShadowCompositeMode` and `ShadowGradeConfig` from the prelude.
- **tui-vfx-types:** Added `color_inert` module with `is_color_inert_glyph()` — detects emoji, PUA/nerd-font icons, variation selectors, and ZWJ that ignore ANSI fg color in terminal emulators.
- **tui-vfx-shadow:** Added `ShadowGradeConfig::replacement_char` field (`Option<char>`) — when set, color-inert glyphs are replaced with the given character during grade-underlying compositing. `Default` is `None` (backward compatible); `dramatic()` sets `Some('·')`.
- **tui-vfx-compositor:** `grade_shadow_cell` now conditionally replaces color-inert glyphs with the configured placeholder, preventing bright bitmap artifacts in dimmed shadow regions.

### Fixed
- **tui-vfx-compositor:** Fixed `test_shadow_extends_render_area` assertion that checked `bg` instead of `fg` for half-block soft-edge shadow cells (shadow color is carried in `fg` for `RIGHT_HALF` characters).

### Changed
- **tui-vfx-shadow:** All five renderers (HalfBlock, Solid, MediumShade, Braille, Gradient) now apply a +1 inset on right-edge `start_y` and bottom-edge `start_x`, so shadows start 1 cell further from the element corner. This improves grade-underlying visual weight by preventing the shadow from crowding the element boundary.
- **tui-vfx-shadow:** `ShadowGradeConfig::dramatic()` preset bumped `fg_dim_strength` from 0.28 → 0.40 and `fg_desaturate_strength` from 0.22 → 0.30, making bright underlying text (e.g. white) more visibly subdued in shadow regions.
- **docs:** Updated `HOWTO_SHADOWS.md` with Shadow Compositing Modes section, dramatic example, and custom grade parameters example.
- **docs:** Updated `API_HAND.md` and `api_docs.toml` to document new types and builder methods.
- **xtask/docs:** Updated `api_metadata.rs` and `gen_api.rs` so generated `API.md` includes `ShadowCompositeMode` and `ShadowGradeConfig` sections and the updated `ShadowConfig` struct.

## 0.2.3 — 2026-03-13

### Added
- **tui-vfx-shadow:** Added `ShadowStyle::MediumShade`, a textured full-cell shadow style that renders with the Unicode medium shade character (`▒`).
- **tui-vfx-shadow:** Added `MediumShadeRenderer` and wired it through `render_shadow(...)` style dispatch.

### Changed
- **tui-vfx-shadow:** Updated crate docs and renderer exports to include the new medium-shade style and renderer.
- **xtask/docs:** Included `MediumShade` in extracted shadow effect metadata so generated docs/schema output reflects the new style.

## 0.2.2 — 2026-03-08

### Fixed
- **tui-vfx-style:** All positional-sweep shaders (`GlistenBandShader`, `BorderSweepShader`, `RadarShader`, `ReflectShader`, `OrbitShader`) multiplied `t * self.speed` internally, but the compositor clamps `t` (via `shader_t`) to `[0, 1]`. With `speed < 1.0`, the sweep was truncated to `speed%` of the full range (e.g. `speed: 0.3` → band only reached 30% of widget width). Fix: removed `self.speed` from the positional computation in all 5 shaders. Sweep rate is now controlled exclusively by the caller via `loop_t`, which is the correct architectural boundary — the compositor owns timing, shaders own spatial mapping. The `speed` field remains on each struct for serde compatibility but is no longer used in rendering. Upstream consumers (e.g. `normalise_shader_timing` in gooey-ratatui) that worked around this bug can now be simplified.

## 0.2.1 — 2026-03-01

### Fixed
- **tui-vfx-shadow:** Normalized HalfBlock right-edge shadow to use `fg=shadow, bg=surface` convention, consistent with all other edges. Previously, the right-edge first column and corner used `fg=surface, bg=shadow` (inverted), which caused the compositor's transparent-portion resolution to land in `fg` instead of `bg`. Downstream `apply_vfx_cell_to_rat` would then preserve the destination cell's existing `fg` (often `Color::Reset` = white) rather than the intended background, producing visible white artifacts on right-edge shadows. Replaced `LEFT_THREE_QUARTERS` (▊) with `RIGHT_HALF` (▐) for the right-edge soft gradient.

## 0.2.0 — 2026-02-18

Initial public release.

### Added
- Compositing pipeline with configurable effect chains via `CompositionOptions`
- 10 mask/transition types: Dissolve, Wipe, Iris, Blinds, Checkers, Diamond, Cellular, Radial, PathReveal, NoiseDither
- 8 filter types: Dim, Brighten, Tint, Invert, Vignette, PatternFill, Greyscale, RigidShake
- 6 sampler types: Ripple, SineWave, CRT, CRTJitter, FaultLine, Shredder
- 14 style shader types for procedural color and style generation
- 12 content transformer types: Typewriter, Scramble, Morph, and more
- Shadow rendering with Braille, HalfBlock, and Solid styles
- Framework-agnostic `Grid` trait for integration with any terminal rendering backend
- Data-driven configuration via serde-compatible effect specs (JSON/TOML)
- `ConfigSchema` derive macro for runtime introspection of effect parameters
- `xtask` documentation pipeline: auto-generated API reference, capabilities inventory, effect schemas, and AI context prompt
- Recipe validation tooling for JSON effect configurations

<!-- <FILE>CHANGELOG.md</FILE> - <DESC>Release history for tui-vfx</DESC> -->
<!-- <VERS>END OF VERSION: 1.5.0</VERS> -->
