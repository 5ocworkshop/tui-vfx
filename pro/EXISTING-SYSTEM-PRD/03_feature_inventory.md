<!-- <FILE>pro/EXISTING-SYSTEM-PRD/03_feature_inventory.md</FILE> - <DESC>Chapter 3 of the evidence-backed Existing-System PRD: product-feature inventory. The table at the top is produced under US-002. Per-feature subsections (Status, Description, ..., 15 schema fields) are produced under US-003.</DESC> -->
<!-- <VERS>VERSION: 0.6.1</VERS> -->
<!-- <WCTX>F012 deepening pass — sub-agent read every transformer + ContentEffect + trait + dispatcher end-to-end; per-transformer field table added.</WCTX> -->
<!-- <CLOG>0.6.1: PATCH — F012 deepened. Per-transformer field surface enumerated end-to-end via sub-agent (31 tool calls). Field-visibility asymmetry, constructor arg-type churn, and Default-coverage gap recorded as Unknowns→consistency observations. 0.6.0: MINOR — F048+F049 for tui-vfx-next. 0.5.0: F031..F047. 0.4.0: F016..F030. 0.3.0: F001..F015. 0.2.0: feature inventory table.</CLOG> -->

# 3. Product Feature Inventory

A feature is an externally meaningful capability, workflow, or behavior the system provides. Every row below is grounded in a code path that demonstrates the behavior; rows whose only evidence is a module name have been excluded.

## 3.1 Feature table

Status vocabulary (per `pro/REVERSE-PRD.md` §"Phase 4"): `implemented` / `partially implemented` / `test-only` / `example-only` / `documented-only` / `behind feature flag` / `unknown`.

Confidence: **High** when code+tests both demonstrate; **Medium** when code path or example/doc but not both; **Low** for naming-only or partial. Low rows do not appear here — they live in chapter 12 (Open Questions).

| ID | Feature | Status | Crates | Entry Points | Options | Evidence | Confidence |
|---|---|---|---|---|---|---|---|
| F001 | Render-pipeline orchestration (CompositionSpec → composed cells) | implemented | `tui-vfx-compositor`, `tui-vfx-types` | `render_pipeline`, `render_pipeline_with_spec`, `render_pipeline_with_spec_area` | `CompositionSpec`, `CompositionOptions`, `CompositionPlaybackTiming`, `&RoleMap` source, `&mut SemanticScene` destination | `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs`, `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs`, `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec_area.rs`, `crates/tui-vfx-compositor/src/pipeline/cls_composition_spec.rs`, `crates/tui-vfx/src/lib.rs:186-195` re-exports through prelude | High |
| F002 | Pipeline observability emission (per-stage trace events) | implemented | `tui-vfx-compositor`, `tui-vfx-debug` | `orc_pipeline_observability`, `InspectionSinkBridge` | trace selectors, stage masks, sinks | `crates/tui-vfx-compositor/src/pipeline/orc_pipeline_observability.rs`, `crates/tui-vfx-compositor/src/traits/cls_inspection_sink_bridge.rs` (via lib re-export), `CHANGELOG.md` 1.16.0 entry naming the additive surface | High |
| F003 | Sampler stage primitives (warp/displace cells before further stages) | implemented | `tui-vfx-compositor`, `mixed-signals` | 11 sampler classes under `compositor/samplers/` | `SamplerSpec`, individual sampler params | `crates/tui-vfx-compositor/src/samplers/` directory contains 11 `cls_*.rs` files (`bounce`, `crt_jitter`, `crt_sampler`, `distortion`, `fault_line`, `gravity`, `pendulum`, `radial_twist`, `ripple`, `shredder`, `sine_wave`) plus `mod.rs` | High |
| F004 | Mask stage primitives (visibility shapes for transitions) | implemented | `tui-vfx-compositor` | 11 mask classes + `col_soft_edge` helper under `compositor/masks/` | `MaskSpec`, `mask_combine_mode`, per-mask params | `crates/tui-vfx-compositor/src/masks/` directory contains `cls_blinds.rs`, `cls_cellular.rs`, `cls_checkers.rs`, `cls_diamond.rs`, `cls_dissolve.rs`, `cls_materialize.rs`, `cls_noise_dither.rs`, `cls_path_reveal.rs`, `cls_radial.rs`, `cls_spotlight.rs`, `cls_wipe.rs`, `col_soft_edge.rs`, `mod.rs`. `mask_combine_mode` lives at `crates/tui-vfx-compositor/src/types/mask_combine_mode.rs` | High |
| F005 | Filter stage primitives (per-cell color/style/glyph transformations) | implemented | `tui-vfx-compositor` | 32-variant `FilterSpec` enum (lines 443-1374; 32 `name()` arms verified at lines 2249-2282); 25 `cls_*.rs` private classes under `compositor/filters/` | `FilterSpec` discriminants + per-variant fields with `SignalOrFloat`/`BindableValue` rate parameters | `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs:443-1374` (the enum); `crates/tui-vfx-compositor/src/filters/` (the implementation files); §3.2 F005 subsection enumerates all 33 variants | High |
| F006 | Shadow rendering (cell-grid-native shadow extrusion + composite) | implemented | `tui-vfx-shadow`, `tui-vfx-types` | `render_shadow`, `render_shadow_into_scene`, `extract_shadow_envelope` | `ShadowConfig`, `ShadowEdges`, `ShadowStyle`, `ShadowCompositeMode`, `ShadowGradeConfig`, `CellMask`, `RoleTag::Shadow` | `crates/tui-vfx-shadow/src/lib.rs:350-363` (the public re-exports — `pub use fnc_render_shadow::{...}`, `pub use fnc_extract_shadow_envelope::{CellMask, extract_shadow_envelope}`, `pub use types::{ShadowCompositeMode, ShadowConfig, ShadowEdges, ShadowGradeConfig, ShadowStyle}`, `pub use renderers::{...}`) | High |
| F007 | Shadow renderer styles (5 visual modes) | implemented | `tui-vfx-shadow` | `renderers/cls_braille.rs`, `cls_gradient.rs`, `cls_half_block.rs`, `cls_medium_shade.rs`, `cls_solid.rs` | renderer style selection via `ShadowStyle` | `crates/tui-vfx-shadow/src/renderers/` directory listing (5 `cls_*.rs` files + `mod.rs`) | High |
| F008 | Style-effect / shader catalog (named factories built from primitive parameters) | implemented | `tui-vfx-style` | `StyleEffect` 11-variant enum + `SpatialShaderType` 31-variant enum + 31 `cls_*_shader.rs` shader-class files + helper signal/ramp/gradient/region/transition types | `StyleEffect`, `SpatialShaderType`, `StyleConfig`, `StyleLayer`, `StyleRegion`, `StyleTransition`, `FadeEffect`, `FadeSpec`, `BlendMode`, `ColorConfig`, `ColorRamp`, `ColorSpace` (`Rgb`/`Hsl`/`Hct`), `Gradient`, `GradientLut`, `FalloffType`, `NoiseType` | `crates/tui-vfx-style/src/models/cls_style_effect.rs:81` (StyleEffect 11 variants); `cls_spatial_shader_type.rs:157` (SpatialShaderType 31 variants); `cls_color_space.rs:31` (ColorSpace 3 variants); per-shader catalogue in §3.2 F008 subsection | High |
| F009 | V3 shader family (`Vfx*` prefix surface) | partially implemented (V3 in flight; V2 still primary) | `tui-vfx-style` | `models/v3/cls_vfx_*_shader.rs`, `models/v3/enum_vfx_*` | V3 vocabulary enums (`VfxSpatialShaderFamily`, `VfxSpatialPrimitive`, `VfxStyleEffectFamily`, etc.) | `crates/tui-vfx-style/src/models/v3/` contains 11 V3 shader classes (`cls_vfx_cursor_shader.rs`, `cls_vfx_edge_distortion_shader.rs`, `cls_vfx_gradient_reveal_shader.rs`, `cls_vfx_guidance_cue_shader.rs`, `cls_vfx_material_light_shader.rs`, `cls_vfx_motion_field_shader.rs`, `cls_vfx_progress_emphasis_shader.rs`, `cls_vfx_stochastic_texture_shader.rs`, `cls_vfx_stripe_motion_shader.rs`, `cls_vfx_surface_depth_shader.rs`, `cls_vfx_traveling_band_shader.rs`) plus 18 `enum_*.rs` files (11 per-shader behavior, 5 family/structure, 2 lowering error) and a `fnc_lower_legacy_spatial_shader.rs` lowering function | High |
| F010 | HCT perceptual color-space integration (Material Color Utilities) | implemented | `tui-vfx-style` | `ColorSpace` enum at `models/cls_color_space.rs:31` with three variants `Rgb` (default) / `Hsl` (shortest-path hue interpolation) / `Hct` (CAM16-based perceptually-uniform via `mcu-hct`, added 1.1.0 on 2026-04-26) | `ColorSpace::{Rgb, Hsl, Hct}` — 3 variants, no `Oklch` despite the rustdoc reference at `cls_style_effect.rs:218` (recorded as a doc-vs-code drift in chapter 11) | `crates/tui-vfx-style/src/models/cls_color_space.rs:31`; `crates/tui-vfx-style/Cargo.toml:33-34` (deps); `Cargo.toml:58-59` (workspace mcu-hct + mcu-utils) | High |
| F011 | Bindable value system (literal / binding / signal three-arm wire form) | implemented | `tui-vfx-core`, `tui-vfx-compositor`, `tui-vfx-style` | `VfxBindableValue`, `VfxBindableU16`, `VfxBindableString` | `BindableValue::Literal/Binding/Signal`, `ShaderRuntimeParams`, `SignalOrFloat` interop | `crates/tui-vfx-core/src/bindable/cls_bindable.rs` (per `.omc/archive/2026-04-28-packet-69-A/progress.txt` line citation `cls_bindable.rs:381` for `VfxBindableValue` — verified in archive evidence), `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs`, `crates/tui-vfx-style/src/models/cls_bindable_string.rs`, `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` | High |
| F012 | Content text transformers (per-glyph text effects) | implemented | `tui-vfx-content` | 15 `cls_*` transformers under `content/transformers/` + `fnc_get_transformer` dispatcher | `Typewriter`, `Scramble`, `GlitchShift`, `ScrambleGlitchShift`, `SplitFlap`, `Marquee`, `Dissolve`, `GlyphCascade`, `Mirror`, `Morph`, `Numeric`, `Odometer`, `Redact`, `SlideShift`, `WrapIndicator` | `crates/tui-vfx-content/src/transformers/` directory contains 15 `cls_*.rs` files plus `fnc_get_transformer.rs`, `fnc_morph_chars.rs`, `mod.rs` | High |
| F013 | Cursor primitive (insertion / typewriter cursor) | implemented | `tui-vfx-content` | `content/cursor/` | (read of cursor module needed for parameters) | `crates/tui-vfx-content/src/cursor/` directory presence; `crates/tui-vfx-style/src/models/cls_cursor_shader.rs` for the visual side; the meta-crate's prelude re-exports cursor types per the CHANGELOG entries naming "cursor primitive (T32)" | High (presence) / Medium (parameter surface — needs read) |
| F014 | Cell-motion scheduler (content-local motion of typed glyph runs) | implemented | `tui-vfx-content` | `content/cell_motion/` | (parameters need read) | `crates/tui-vfx-content/src/cell_motion/` directory presence; `crates/tui-vfx-content/Cargo.toml:29-30` adds `tui-vfx-geometry` "for cell-motion scheduler" inline comment | High (presence) / Medium (full param surface) |
| F015 | Mechanical content cycles (split-flap / odometer / numeric drum patterns) | implemented | `tui-vfx-content` | `content/mechanical/` | (parameters need read) | `crates/tui-vfx-content/src/mechanical/` directory presence; references in `.omc/archive/2026-04-28-packet-69-A/progress.txt` ("mechanical-circular-content-cycles-plan") | High (presence) / Medium (full surface) |
| F016 | Glyph particles (sparkles / fire / drip particles bound to a cell field) | implemented | `tui-vfx-content` | `content/glyph_particles/` | (parameters need read) | `crates/tui-vfx-content/src/glyph_particles/` directory presence | Medium (directory presence; parameter contract requires read) |
| F017 | RocketsplashImage source (`.rss` braille image format consumer) | implemented | `tui-vfx-content` | `sources/cls_rocketsplash_image.rs` + `fnc_blit_render_buffer_to_grid.rs` | image asset path / bytes | `crates/tui-vfx-content/src/sources/cls_rocketsplash_image.rs`, `crates/tui-vfx-content/src/sources/fnc_blit_render_buffer_to_grid.rs`, `crates/tui-vfx-content/Cargo.toml:31` (`rocketsplash-rt.workspace = true`) | High |
| F018 | RocketsplashFont source (`.rsf` font atlas consumer) | implemented | `tui-vfx-content` | `sources/cls_rocketsplash_font.rs` | font asset path / bytes | `crates/tui-vfx-content/src/sources/cls_rocketsplash_font.rs` | High (presence) / Medium (parameter surface — needs read) |
| F019 | 3×3 line-glyph default font (fallback when no font is declared) | implemented | `tui-vfx-content` | `fonts/col_line_3x3_heavy_glyphs.rs`, `fonts/fnc_lookup_line_3x3_glyph.rs`, `fonts/cls_font_glyph_table.rs`, `fonts/cls_font_registry.rs` | (default-font policy is engine-side; recipes can override) | `crates/tui-vfx-content/src/fonts/` directory; `steering/INTENTIONS.md:836-838` (Intention 36) declares this canonical home in code | High |
| F020 | Asset byte-source loaders (filesystem / embedded / arbitrary `Read`) | implemented | `tui-vfx-content` | `content/assets/` | (parameter surface needs read) | `crates/tui-vfx-content/src/assets/` directory presence; `steering/INTENTIONS.md` Intention 27 declares the byte-source contract | Medium (directory presence; concrete loader functions need read for High) |
| F021 | Pool primitives (TextPool / EffectPool / ImagePool / FontPool / PresetPool + PoolPolicy) | implemented | `tui-vfx-content` | `content/pool/` | `PoolPolicy` | `crates/tui-vfx-content/src/pool/` directory; `CHANGELOG.md` 1.12.0 entry names the added primitives | High |
| F022 | Easing functions library (criterion-benched) | implemented | `tui-vfx-geometry` | `geometry/easing/mod.rs` | (easing variants need read) | `crates/tui-vfx-geometry/src/easing/mod.rs`, plus the `easing` criterion bench at `crates/tui-vfx-geometry/Cargo.toml:35-37`, plus `crates/tui-vfx-geometry/benches/easing.rs` filesystem presence | High |
| F023 | Motion-path library (9 path shapes) | implemented | `tui-vfx-geometry` | 9 `cls_*_path.rs` files under `geometry/paths/` | `MotionPath` trait at `geometry/traits/`, per-path params | `crates/tui-vfx-geometry/src/paths/` contains `cls_arc_path.rs`, `cls_bezier_path.rs`, `cls_hover_path.rs`, `cls_linear_path.rs`, `cls_rectilinear_path.rs`, `cls_spiral_path.rs`, `cls_spring_path.rs`, `cls_squash_path.rs`, `cls_step_path.rs`, `mod.rs`. `MotionPath` re-exported at `crates/tui-vfx-geometry/src/lib.rs:18` | High |
| F024 | Wipe geometry helpers (shared between Mask::Wipe and `RevealWipe` shader) | implemented | `tui-vfx-geometry`, `tui-vfx-compositor`, `tui-vfx-style` | `geometry/wipe/` exposes `wipe_progress`, `wipe_visible_at`, `WipeDirection` | wipe direction enum, progress fraction | `crates/tui-vfx-geometry/src/lib.rs:23` (`pub use wipe::{wipe_progress, wipe_visible_at}`); `crates/tui-vfx-compositor/Cargo.toml:32` (compositor adds geometry as a runtime dep specifically for "WipeDirection re-export and the shared wipe geometry helpers" per its `<WCTX>` block) | High |
| F025 | Anchors / borders / layout / transitions / widgets (geometry primitives) | implemented | `tui-vfx-geometry` | `geometry/{anchors, borders, layout, transitions, widgets}/` | `Anchor`, `Point`, `Rect`, `Size` types | `crates/tui-vfx-geometry/src/lib.rs:7-15` (sub-module declarations) | High |
| F026 | `Grid` trait + `BoundaryMode` + `OwnedGrid` (cell-buffer interface) | implemented | `tui-vfx-types` | `grid` module re-exports | `BoundaryMode`, `GridExt`, `OwnedGrid` | `crates/tui-vfx-types/src/lib.rs:97` (`pub use grid::{BoundaryMode, Grid, GridExt, OwnedGrid}`) | High |
| F027 | `SemanticScene` + `RoleMap` (role-aware destination scene) | implemented | `tui-vfx-types` | `semantic_scene`, `role_map` modules | `RoleTag` (with `Custom(Arc<str>)` and `Shadow` variants), `InternedRoleName`, `RoleId`, `RoleInterner`, `RoleMapIter` | `crates/tui-vfx-types/src/lib.rs:103-108` (the role-related re-exports) | High |
| F028 | Glyph rendering framework (subcell sampling + encoder) | partially implemented (Phase 3 per the `<WCTX>` header on `tui-vfx-types/Cargo.toml:3`) | `tui-vfx-types`, `mixed-signals` | `tui-vfx-types/src/glyph/` | `GlyphEncoder`, subcell sampling helpers (per `crates/tui-vfx-types/Cargo.toml:4` `<CLOG>`) | `crates/tui-vfx-types/src/lib.rs:78` (`pub mod glyph`); `crates/tui-vfx-types/Cargo.toml:3-4` `<WCTX>` and `<CLOG>` headers describe Phase 3 of the framework | High (existence) / Medium (Phase 3 means partial — final shape pending) |
| F029 | Braille primitives (subcell dot encoding) | implemented | `tui-vfx-types` | `tui-vfx-types/src/braille/` | (specific shapes need read) | `crates/tui-vfx-types/src/lib.rs:72` (`pub mod braille`) | Medium (presence; full type surface needs read) |
| F030 | RigidShake timing primitive | implemented | `tui-vfx-types` | `rigid_shake_timing` module | `RigidShakeState`, `RigidShakeTiming` | `crates/tui-vfx-types/src/lib.rs:84,102` (`pub mod rigid_shake_timing` and the corresponding `pub use`) | High |
| F031 | Opaque ID types (`LayerId`, `RecipeId`, `RoleId`) | implemented | `tui-vfx-types` | `layer_id`, `recipe_id`, `role_id` modules | (none — they are bare-id types) | `crates/tui-vfx-types/src/lib.rs:99,101,103` re-exports | High |
| F032 | `ConfigSchema` derive macro (proc-macro) | implemented | `tui-vfx-core-macros`, `tui-vfx-core` | `derive_config_schema` in `tui-vfx-core-macros`; `schema` module in `tui-vfx-core` | derive attribute syntax | `crates/tui-vfx-core-macros/src/lib.rs:28` (`pub fn derive_config_schema(input: TokenStream) -> TokenStream`); `crates/tui-vfx-core/src/lib.rs:8,16,23` (`pub mod schema`, `pub use schema::{...}`, `pub use tui_vfx_core_macros::ConfigSchema`) | High |
| F033 | `mixed-signals` schema bridge (engine-native signal types in schema generation) | implemented | `tui-vfx-core` | `mixed_signals_schema` module | (per-signal schema metadata) | `crates/tui-vfx-core/src/lib.rs:7` (`pub mod mixed_signals_schema`) | High (existence) / Medium (full surface) |
| F034 | `TimeSpec` time-axis primitive | implemented | `tui-vfx-core` | `time_spec` module | (TimeSpec variants — needs read) | `crates/tui-vfx-core/src/lib.rs:9,20` (module + `pub use time_spec::TimeSpec`) | High |
| F035 | Centralized debug logger (granular per-module log levels) | implemented | `tui-vfx-debug` | `DebugLogger`, `Logger`, `create_logger`, `get_global_logger` | `LogLevel`, `ModuleConfig` | `crates/tui-vfx-debug/src/lib.rs:37-38` re-exports; `crates/tui-vfx-debug/src/logger.rs` (file presence); `crates/tui-vfx-debug/src/config.rs` (file presence) | High |
| F036 | Inspection foundation (TraceEvent + sinks + filters) | implemented (additive surface introduced in Phase A.4 per `Cargo.toml:6` `<CLOG>` and `CHANGELOG.md` 1.16.0) | `tui-vfx-debug`, `tui-vfx-compositor` | `tui-vfx-debug::inspection` module | `TraceEvent`, `TraceEnvelope`, `TraceSelector`, `TraceFilter`, `StageMask`, `InspectionSink`, `TraceSink`, `TraceReport`, `InspectionSinkBridge` | `crates/tui-vfx-debug/src/lib.rs:34` (`pub mod inspection`); `crates/tui-vfx-debug/src/inspection/` directory; `Cargo.toml:6` `<CLOG>` and `CHANGELOG.md:1.16.0` entries name the surface | High |
| F037 | `pipeline-probe` binary (frame dump / timeline / frame diff) | implemented | `tui-vfx-probe` | bin `pipeline-probe` at `crates/tui-vfx-probe/src/bin/pipeline-probe.rs` | CLI surface — full subcommand set is enumerated in chapter 5 | `crates/tui-vfx-probe/Cargo.toml:29-31`; `crates/tui-vfx-probe/src/bin/pipeline-probe.rs` (filesystem presence); `README.md`'s `<CLOG>` at line 4 says: "MINOR: Update the docs section so README reflects that pipeline-probe now supports frame dumps, timelines, and frame diffs for direct engine scenes" | High |
| F038 | Probe DTO catalogue (per-frame structured introspection types) | implemented | `tui-vfx-probe` | 24+ `cls_probe_*` DTOs (cell, color, diagnostic, diff_cell, diff_report, error, grid_spec, last_touch, operational_analysis, pipeline_inventory, report, request, runtime_context, scene_spec, state_snapshot, summary, timeline_report, timing, trace_event, widget) | per-DTO fields | `crates/tui-vfx-probe/src/lib.rs:75-99` (the run of `pub use cls_probe_*::...` re-exports) | High |
| F039 | Probe SQLite store (frame persistence) | implemented | `tui-vfx-probe` | `cls_probe_sqlite_store::ProbeSqliteStore` | (sqlite path / schema parameters need read) | `crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs`; `crates/tui-vfx-probe/src/lib.rs:93` (`pub use cls_probe_sqlite_store::ProbeSqliteStore`); `crates/tui-vfx-probe/Cargo.toml:25` (`rusqlite = { version = "0.32", features = ["bundled"] }`) | High |
| F040 | Probe diagnostics + root-cause inference | implemented | `tui-vfx-probe` | `collect_basic_diagnostics`, `collect_loopback_fire_diagnostics`, `collect_probe_operational_analysis`, `build_probe_cell_root_cause`, `infer_roles_from_grid`, `diff_frames` | per-fn parameters | `crates/tui-vfx-probe/src/lib.rs:100-106` (`pub use fnc_*` block) | High |
| F041 | Probe runtime-context introspection | implemented | `tui-vfx-probe` | `ProbeRuntimeContext`, `ProbeRuntimeParam`, `runtime_context_from_composition` | (parameters need read) | `crates/tui-vfx-probe/src/lib.rs:91` re-exports `cls_probe_runtime_context::{ProbeRuntimeContext, ProbeRuntimeParam}`; `crates/tui-vfx-probe/src/fnc_runtime_context_from_composition.rs` filesystem presence | High |
| F042 | `cargo xtask` build-tooling surface | implemented | `xtask` | bin `xtask` at `xtask/src/main.rs`; subcommand modules under `xtask/src/{audit, docs, recipes}/` | full subcommand set is enumerated in chapter 5 | `xtask/Cargo.toml:13-15` (`[[bin]]`); `xtask/src/main.rs` (file presence); `xtask/src/audit/`, `xtask/src/docs/`, `xtask/src/recipes/` (directory presence) | High |
| F043 | `xtask audit configschema` lint (ConfigSchema-justification audit) | implemented | `xtask` | `xtask/src/audit/`, integration test `xtask/tests/test_audit_configschema.rs` | (audit options need read) | `xtask/src/audit/` directory; `xtask/tests/test_audit_configschema.rs` (15 787 bytes — substantive integration coverage); `xtask/Cargo.toml:1` `<WCTX>` block: "Packet 1.9.A — ConfigSchema justification lint"; `xtask/Cargo.toml:13-19` (`[[bin]]` plus `[lib] name = "xtask_audit_configschema"`) | High |
| F044 | `xtask docs` capability documentation generation | implemented | `xtask` | `xtask/src/docs/` modules; `cargo xtask docs generate` per the `justfile`'s "DOCUMENTATION GENERATION" header at `justfile:9-25` | `docs/templates/capabilities.toml`, output to `docs/generated/` | `xtask/src/docs/` directory; `justfile:9-25` (header block enumerating the merge of rustdoc + `docs/templates/capabilities.toml` → `docs/generated/CAPABILITIES.md`, `docs/generated/ai-context.md`, `docs/generated/capabilities.json`, `docs/generated/effect_schemas.json`); `crates/tui-vfx/Cargo.toml:1` and the workspace's `xtask/Cargo.toml:1` `<DESC>` blocks | High |
| F045 | Workspace-root example targets | implemented (example-only delivery) | `tui-vfx` (host) | `examples/pipeline_effects_showcase.rs`, `examples/direct_api_signal_strength.rs` | example-internal | `crates/tui-vfx/Cargo.toml:32-38` (the two `[[example]]` blocks); `examples/` directory listing | High |
| F046 | Optional `serde` integration in the foundation types | implemented (Cargo feature, default on) | `tui-vfx-types` | the `serde` feature gate | `default = ["serde"]`, `serde = ["dep:serde"]` | `crates/tui-vfx-types/Cargo.toml:26-28` (the `[features]` block) | High |
| F047 | Two `criterion` benches targeting a 60 fps trace-emission budget | implemented (bench-only) | `tui-vfx-debug` | `bench_emit_overhead`, `bench_full_trace_60fps` | bench harness | `crates/tui-vfx-debug/Cargo.toml:30-36` (`[[bench]]` blocks); `crates/tui-vfx-debug/benches/` directory contains both `.rs` files | High |
| F048 | Clean-room V3.1 surface-contract spike (`tui-vfx-next`) | partially implemented (spike — proves Phase A surface rules without depending on the legacy compositor/style/content/shadow stacks) | `tui-vfx-next` | `Surface`, `SurfaceMetadata`, `SurfaceEngine`, `ApplyOutcome`, `ScopeSpec`, `EffectDescriptor`, `EffectDomain`, `DimEffect`, `ExplicitRoleWriteEffect`, `SurfaceDiagnostic`, `SurfaceDiagnosticCode`, `DiagnosticLevel`, `CellChannel`, `CoordinateSpace`, `RoleSpace`, `ScopeEvalInput` | n/a (spike scope) | `crates/tui-vfx-next/src/lib.rs:1-23` (full re-export surface); `crates/tui-vfx-next/Cargo.toml:1` (`<DESC>` block); module rustdoc at `crates/tui-vfx-next/src/lib.rs:3-9` | High |
| F049 | Cell-write and role-write policy contracts (the V3.1 spike's write surface) | partially implemented (spike) | `tui-vfx-next` | `CellWrite`, `CellWritePolicy`, `RoleWritePolicy` re-exported at `crates/tui-vfx-next/src/lib.rs:23` | per-policy parameters | `crates/tui-vfx-next/src/write.rs:1-52`; `crates/tui-vfx-next/src/effect.rs:1-30` (the `EffectDomain` enum gates `Visual` vs `Procedural` cell writes — visual effects preserve roles; procedural effects may write roles) | High |

## 3.2 Per-feature subsections

The detailed schema (Status / Description / User-visible behavior / Entry points / Inputs / Outputs / Options/config / Data touched / External systems / Errors and edge cases / Observability / Tests / Evidence / Confidence / Unknowns) is produced under US-003. F001–F015 are populated below; F016–F047 land in subsequent US-003 batches.

A note on the compositor's public surface: the `filters/`, `masks/`, and `samplers/` sub-modules in `tui-vfx-compositor` are declared `pub(crate)` (`crates/tui-vfx-compositor/src/lib.rs:7-9`). Consumers do not construct individual `cls_*` filter/mask/sampler classes directly — they construct `FilterSpec` / `MaskSpec` / `SamplerSpec` values (the public types in `crates/tui-vfx-compositor/src/types/`) which the compositor lowers internally.

### F001 — Render-pipeline orchestration

- **Status:** implemented.
- **Description:** A free function family that takes a source `Grid`, a source `RoleMap`, a destination `&mut SemanticScene`, a render rectangle, and either a `CompositionSpec` (high-level) or a pre-built `CompositionOptions` (lower-level), and writes the composed cells into the destination.
- **User-visible behavior:** Cells from the source grid pass through the four-stage pipeline (Sampler → Mask → Shader → Filter) plus any declared shadow stage, with output written into the destination scene at the requested offset.
- **Entry points:** `render_pipeline` and `render_pipeline_with_area` at `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:106` and `:211`; `render_pipeline_with_spec` at `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs:21`; `render_pipeline_with_spec_area` at `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec_area.rs:15`. All four re-exported from `pipeline/mod.rs:35-37`.
- **Inputs:** `source: &dyn Grid`, `source_roles: &RoleMap`, `destination: &mut SemanticScene`, `width: usize`, `height: usize`, `offset_x: usize`, `offset_y: usize`, `spec: &CompositionSpec` (or `options: CompositionOptions<'_>`), `inspector: Option<&mut dyn CompositorInspector>`.
- **Outputs:** Mutation of `destination: &mut SemanticScene`. Returns `()`. The function does not allocate the destination.
- **Options/config:** `CompositionSpec` (`crates/tui-vfx-compositor/src/pipeline/cls_composition_spec.rs:20`, `deny_unknown_fields`) carries `sampler_spec: Option<SamplerSpec>`, `samplers: Vec<SamplerSpec>` (skip if empty), `masks: Vec<MaskSpec>`, `mask_combine_mode: MaskCombineMode = All`, `filters: Vec<FilterSpec>`, `shader_layers: Vec<ShaderLayerSpec>`, `shadow: Option<ShadowSpec>` (`#[config(opaque)]`), `preserve_unfilled: bool = true`, `t: f64 = 0`, `loop_t: Option<f64>`, `phase: Option<Phase>` (skip-serializing, opaque), `runtime_params: ShaderRuntimeParams` (skip-serializing, opaque). Methods: `effective_samplers()` (`:108`), `has_active_sampler()` (`:117`), `push_sampler` (`:128`, mirrors first into `sampler_spec`), `apply_playback_timing` / `with_playback_timing` (`:136, :143`), `try_push_v3_shader_family` / `try_with_v3_shader_family` (`:150, :161`), `v3_shader_families() -> Vec<VfxSpatialShaderFamily>` (`:172`).
- **Data touched:** Source grid cells (read-only), `RoleMap` (read-only), destination `SemanticScene` (mutated). The shadow path additionally writes `RoleTag::Shadow` to the destination role map (`orc_render_pipeline.rs:777-781`).
- **External systems:** None.
- **Errors and edge cases:** Three production-path `expect()` panics, all guarded:

  | Line | Site | Guard |
  |---|---|---|
  | `:255` | `let shadow_spec = options.shadow.as_ref().expect("shadow_spec must be Some");` | Caller-asserted: only reachable after the `is_some()` dispatch at `:118` |
  | `:559` | `shadow_cell.expect("shadow region candidate must have shadow coverage")` | Guarded by the `shadow_has_coverage` boolean at `:553` |
  | `:582, :712` | `shadow_cell.expect("shadow coverage implies a shadow cell")` | Same `shadow_has_coverage` guard above each site |

  Plus REQ-002 records the V3-lowering `.expect()` at `fnc_render_pipeline_with_spec.rs:25-28` (`ShaderWithRegion::try_from_v3_shader_family ... .expect("spec shader layers should lower through the grouped V3 runtime seam")`). No `unwrap()`. Bounds via `if let Some(cell) = source.get(...)` (`:440, :873, :1024`) and `is_none_or` (`:547, :695`). Sampler chain rejection and source-grid-miss yield `continue`, not panic.

- **Four-stage call sequence:**

  Non-shadow path (`render_loop` at `:819-909`):
  1. Sampler — `sample_sampler_chain(samplers, …)` (`:853`); cells where source coord is `None` → `continue` (`:856`).
  2. Mask — `check_prepared_masks(&mask_ctx, prepared_masks, options.mask_combine_mode, None)` (`:868`); `false` → continue.
  3. Source-cell read — `source.get(src_x, src_y)` (`:873`); `None` → continue.
  4. Shader — `apply_shaders(...)` (`:881`) delegates to per-cell `layer.shader.style_at(&shader_ctx, current_style)` (`:1142` inside `apply_shaders`).
  5. Filter — loop `filter.apply(&mut out_cell, &filter_ctx)` (`:902-904`).
  6. Write — `dest.set(offset_x+x, offset_y+y, out_cell)` (`:906`).

  Inspected variant (`render_loop_inspected` at `:913-1077`): same order; adds `emit_role_map_materialized` (`:949`), `emit_per_stage_entered` returning a block with `skipped_shader_indices` (`:967`), per-cell `inspector.on_sampler_applied(...)` (`:991`), `inspector.on_filter_applied(before, after, "Name#i")` (`:1058`), `inspector.on_cell_rendered(...)` (`:1067`); closes with `emit_per_stage_finished(inspector, &block)` (`:1076`). Skipped shaders (zero-scope) are bypassed via `skipped_shader_indices` membership at `:457`.

  Shadow path (`render_pipeline_with_shadow` at `:243-793`):
  1. **Stage 1 — Shadow generation.** `effective_shadow_rect(...)` then `render_shadow(buffer, rect, &shadow_spec.config, options.t)` at `:319-323`. Wrapped by `emit_simple_stage_entered(PipelineStageKind::Shadow, step_id=1, ...)` at `:299` and `emit_simple_stage_finished` at `:329`.
  2. **Snapshot** — `shadow_only_guard.cells_mut().copy_from_slice(buffer.cells())` at `:335`.
  3. **Element pass** (`:418-499`): same Sampler → Mask → Shader → Filter sequence written into the buffer. `emit_per_stage_entered(insp, 2, ...)` at `:397` (step_id = 2).
  4. **Stage 4 — Mask + composite write-back** (`:522-771`): per-cell, classify `in_element` vs shadow region; switch on `shadow_spec.config.composite_mode` between `blend_shadow_cell`, `grade_shadow_cell`, `blend_underlying_shadow_cell` (`:584-611, :599-633, :714-741, :745-759`). `preserve_unfilled` short-circuit (`:566, :677`).
  5. **Role write-back** — `RoleTag::Shadow` written for collected positions via `destination.roles_mut().set(...)` at `:777-781`.
  6. `emit_per_stage_finished(insp, block)` at `:790`.

- **Observability:** Eight inspector-emit sites enumerated:

  | Emitter | Line(s) | Stage |
  |---|---|---|
  | `emit_role_map_materialized` | `:298` (shadow path), `:949` (non-shadow inspected) | pre-stage |
  | `emit_simple_stage_entered` / `_finished` | `:299, :329` | Shadow stage only |
  | `emit_per_stage_entered` | `:397` (shadow element pass step_id=2), `:967` (non-shadow step_id=1) | Sampler/Mask/Shader/Filter bundle |
  | `emit_per_stage_finished` | `:790, :1076` | post-loop |
  | `inspector.on_shadow_cell_applied` | `:556` | per shadow-region cell |
  | `inspector.on_sampler_applied` | `:991` | per cell |
  | (mask events emitted internally inside `check_prepared_masks` when called with `Some(inspector)`) | `:537, :1018` | per cell |
  | `inspector.on_filter_applied(before, after, "Name#i")` | `:1058` | per cell, per filter |
  | `inspector.on_cell_rendered` | `:639, :1067` | per cell |

- **Tests:** `crates/tui-vfx-compositor/tests/test_render_pipeline_signature.rs`, `test_render_pipeline_role_awareness.rs`, `test_pipeline.rs`, `test_inspection_sink_bridge*.rs`, `test_shadow_*_emits.rs`, `test_scope_mismatch_emits_zero_cell_skip.rs`. Cross-crate integration: `crates/tui-vfx/tests/test_foundation_end_to_end.rs`.
- **Evidence:** `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:106, :211, :243-793, :819-909, :913-1077`; `fnc_render_pipeline_with_spec.rs:21-50`; `cls_composition_spec.rs:20-172`; `cls_composition_options.rs:103-361`.
- **Confidence:** High.
- **Unknowns:** None of operational note. The full 1227-LOC body has been catalogued at the public-signature, four-stage-sequence, emit-site, and panic-site level; per-cell inner-loop optimizations are implementation detail not load-bearing for this PRD's scope.

### F002 — Pipeline observability emission

- **Status:** implemented (additive surface introduced in Phase A.4 per `Cargo.toml:6` `<CLOG>`).
- **Description:** Per-stage trace events emitted from the render pipeline through a swappable inspector. The compositor side declares the emit helpers; the consumer side accepts events through `InspectionSink` / `TraceSink` traits in `tui-vfx-debug::inspection`.
- **User-visible behavior:** When `render_pipeline*` is called with a non-`None` `inspector`, per-stage `TraceEvent` values are pushed through that inspector. Sinks can record, filter, or report.
- **Entry points:** `crates/tui-vfx-compositor/src/pipeline/orc_pipeline_observability.rs` (the per-stage emit helpers); `crates/tui-vfx-compositor/src/traits/cls_inspection_sink_bridge.rs` (the `InspectionSinkBridge` adapter); the receiving traits live at `crates/tui-vfx-debug/src/inspection/` (declared at `crates/tui-vfx-debug/src/lib.rs:34`).
- **Inputs:** Stage outputs from the render loop (cells, filter masks, sampler displacements, shader contributions). Specific event payload types are `TraceEvent`, `TraceEnvelope`, `TraceSelector`, `TraceFilter`, `StageMask`, `TraceReport` per the `Cargo.toml:6` `<CLOG>`.
- **Outputs:** Events written into whichever sink is wired (NDJSON-round-trippable per the `<CLOG>` claim).
- **Options/config:** `TraceFilter` / `StageMask` to select which stages emit; `InspectionSink` chooses the destination.
- **Data touched:** Same as F001 (plus the sink's storage).
- **External systems:** None inherent — sinks may write to files / sockets, but the trait surface itself does not require them.
- **Errors and edge cases:** A `None` inspector elides emission (verified by `Option<&mut dyn CompositorInspector>` parameter shape on `render_pipeline_with_spec`).
- **Observability:** This *is* the observability surface.
- **Tests:** `crates/tui-vfx-compositor/tests/test_inspection_sink_bridge.rs`, `test_inspection_sink_bridge_per_stage.rs`, `test_shadow_stage_emits.rs`, `test_scope_mismatch_emits_zero_cell_skip.rs`, `test_shadow_path_emits_shader_scope_skip.rs`. The two criterion benches `tui-vfx-debug::bench_emit_overhead` and `bench_full_trace_60fps` exercise emit-path performance.
- **Evidence:** `crates/tui-vfx-compositor/src/pipeline/orc_pipeline_observability.rs`, `crates/tui-vfx-compositor/src/pipeline/mod.rs:25` (`mod orc_pipeline_observability;`), `crates/tui-vfx-debug/src/lib.rs:34`, `Cargo.toml:6` `<CLOG>` block.
- **Confidence:** High.
- **Unknowns:** Full event-payload schema — requires reading `tui-vfx-debug/src/inspection/`; not enumerated here.

### F003 — Sampler stage primitives

- **Status:** implemented.
- **Description:** Pre-stage cell warping. Samplers run first in the four-stage pipeline; they may displace, shred, ripple, or otherwise pre-transform cells before the mask gates them.
- **User-visible behavior:** Per-cell or per-region warps applied before the mask stage runs.
- **Entry points:** Public `SamplerSpec` enum at `crates/tui-vfx-compositor/src/types/cls_sampler_spec.rs:163-452` with `#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]` and `#[derive(Default)]` (default = `None`). The 11 implementation classes are private (`pub(crate) mod samplers` at `crates/tui-vfx-compositor/src/lib.rs:9`).
- **Inputs:** `SamplerSpec` discriminant + per-variant params (table below); grid coordinates from the pipeline.
- **Outputs:** A pre-transformed cell sampling consumed downstream.
- **Options/config:**

  | Variant | Line | Fields (default if literal) |
  |---|---|---|
  | `None` | `:169` | (unit, default) |
  | `SineWave` | `:183` | `axis: Axis`, `amplitude: SignalOrFloat = 1.0`, `frequency: SignalOrFloat = 1.0`, `speed: SignalOrFloat = 1.0`, `phase: SignalOrFloat = 0.0` |
  | `Ripple` | `:218` | `amplitude: SignalOrFloat = 1.0`, `wavelength: SignalOrFloat = 4.0`, `speed: SignalOrFloat = 1.0`, `center: RippleCenter` (required) |
  | `Shredder` | `:247` | `stripe_width: u16` (required), `odd_speed: SignalOrFloat = 2.0`, `even_speed: SignalOrFloat = 2.0` |
  | `FaultLine` | `:270` | `seed: u64`, `intensity: SignalOrFloat = 1.0`, `split_bias: f32` |
  | `Crt` | `:292` | `scanline_strength: SignalOrFloat = 0.8`, `jitter: SignalOrFloat = 0.5`, `curvature: SignalOrFloat = 0.1` |
  | `CrtJitter` | `:316` | `intensity: SignalOrFloat = 0.5`, `speed_hz: SignalOrFloat = 0.5`, `decay_ms: u64` |
  | `Bounce` | `:343` | `amplitude: SignalOrFloat = 2.0`, `speed: SignalOrFloat = 4.0`, `phase_spread: SignalOrFloat = 0.5` |
  | `Pendulum` | `:379` | `axis: Axis`, `amplitude: SignalOrFloat = 2.0`, `speed: SignalOrFloat = 2.0`, `phase_spread: SignalOrFloat = 0.3` |
  | `Gravity` | `:420` | `axis: Axis`, `acceleration: SignalOrFloat = 4.0`, `terminal_velocity: SignalOrFloat = 10.0` |
  | `RadialTwist` | `:439` | `twist: SignalOrFloat = 1.0`, `center: RippleCenter`, `radius_floor: SignalOrFloat = 0.1` |

  Supporting types: `Axis { X (default), Y }` at `:51`; `RippleCenter { Center (default), Point { x: u16, y: u16 } }` at `:64` with **hand-written serde** (`Serialize` impl `:72` accepts `"center"` or `{x,y}`; `Deserialize` impl `:90` accepts `"center"`/`"Center"` strings or the map form).

- **Data touched:** Source cells (read), produces a transformed sampling.
- **External systems:** None.
- **Errors and edge cases:** `try_from_v3_payload(Value) -> serde_json::Result<Self>` at `:560` performs JSON normalization (`phase_offset` → `phase` for sine_wave; ripple-center `{kind:center}` / `{kind:cell, x, y}` rewrites). Internal `unwrap_or_default()` / `unwrap_or(0)` paths handle missing fields without panic. No production-path `panic!` / `unwrap()` / `expect()`.
- **Observability:** Per-stage emit via F002. Sampler stage entry recorded by `inspector.on_sampler_applied(...)` at `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:991`.
- **Tests:** `crates/tui-vfx-compositor/tests/test_pipeline.rs` and the per-stage tests under `tests/pipeline/`.
- **Evidence:** `crates/tui-vfx-compositor/src/types/cls_sampler_spec.rs:163-452,560,614,631,650`; `crates/tui-vfx-compositor/src/samplers/` (11 `cls_*.rs` + `mod.rs`); `crates/tui-vfx-compositor/src/pipeline/cls_prepared_sampler.rs`. Public methods: `name() -> &'static str` (`:614`), `terse_description()` (`:631`), `key_parameters() -> Vec<(&'static str, String)>` (`:650`).
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F004 — Mask stage primitives

- **Status:** implemented.
- **Description:** Visibility shapes that gate which cells the shader and filter stages touch. Masks combine via `MaskCombineMode` (`crates/tui-vfx-compositor/src/types/mask_combine_mode.rs:28` — variants `All` (default), `Any`, `Blend { ratio: f32 }` clamped 0.0–1.0).
- **User-visible behavior:** Wipes, dissolves, blinds, spotlights, materialize-style reveals, irises, diamonds, paths, radials, cellular shapes, etc.
- **Entry points:** Public `MaskSpec` enum at `crates/tui-vfx-compositor/src/types/cls_mask_spec.rs:192-473` with `#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]`, default = `None`. The 11 implementation classes plus `col_soft_edge.rs` helper are private (`pub(crate) mod masks` at `crates/tui-vfx-compositor/src/lib.rs:8`).
- **Inputs:** `MaskSpec` discriminant + per-variant params (table below); `mask_combine_mode`.
- **Outputs:** A per-cell visibility decision before shader/filter stages.
- **Options/config:**

  | Variant | Line | Fields |
  |---|---|---|
  | `None` | `:198` | unit (default) |
  | `Wipe` | `:228` | `reveal: Option<WipeDirection>`, `hide: Option<WipeDirection>`, `direction: Option<WipeDirection>` (legacy alias), `soft_edge: bool` |
  | `Dissolve` | `:268` | `seed: u64`, `chunk_size: u8` |
  | `Checkers` | `:288` | `cell_size: u16` |
  | `Blinds` | `:304` | `orientation: Orientation`, `count: u16` |
  | `Iris` | `:332` | `shape: IrisShape`, `soft_edge: bool` |
  | `Diamond` | `:345` | `soft_edge: bool` |
  | `NoiseDither` | `:364` | `seed: u64`, `matrix: DitherMatrix` |
  | `Materialize` | `:375` | `origin: RadialOrigin` (default), `seed: u64 = 0`, `chunk_size: u8 = 1`, `noise: f32 = 0.18`, `soft_edge: bool = true` |
  | `PathReveal` | `:411` | `path: RevealPathType`, `soft_edge: bool` |
  | `Radial` | `:434` | `origin: RadialOrigin`, `soft_edge: bool` |
  | `Cellular` | `:459` | `pattern: CellularPattern`, `seed: u64`, `cell_count: u16 = 16` |

  Supporting types: `Orientation { Horizontal (default), Vertical }` (`:72`); `IrisShape { Circle (default), Diamond, Box }` (`:89`); `DitherMatrix { Bayer4 (default), Bayer8 }` (`:115`); `WipeDirection` re-exported from `tui_vfx_geometry` at `:65`; `ResolvedWipe { direction: WipeDirection, invert: bool }` at `:481`.

- **Data touched:** Cell coordinates only; produces a visibility map.
- **External systems:** None.
- **Errors and edge cases:** `resolve_wipe(&self) -> Option<ResolvedWipe>` (`:500`) walks priority `hide > reveal > direction > LeftToRight default`. `should_invert(&self) -> bool` (`:539`). No production-path `panic!`/`unwrap()`/`expect()`. `fnc_check_masks.rs` (`pipeline/mod.rs:18`) is the per-cell mask combination evaluator.
- **Observability:** Per-stage emit via F002. Mask-stage events emitted internally inside `check_prepared_masks` when called with `Some(inspector)` (sites at `orc_render_pipeline.rs:537, :1018`).
- **Tests:** `crates/tui-vfx-compositor/tests/test_pipeline.rs`; per-mask tests in the in-source `#[cfg(test)]` modules.
- **Evidence:** `crates/tui-vfx-compositor/src/types/cls_mask_spec.rs:192-473,500-582`; `crates/tui-vfx-compositor/src/masks/` (11 `cls_*.rs` + `col_soft_edge.rs` + `mod.rs`); `crates/tui-vfx-compositor/src/types/mask_combine_mode.rs:28`. Public methods: `resolve_wipe` (`:500`), `should_invert` (`:539`), `name` (`:544`), `terse_description` (`:562`), `key_parameters` (`:582`).
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F005 — Filter stage primitives

- **Status:** implemented.
- **Description:** Per-cell color, style, and glyph transformations applied after the shader stage. The largest enum in the workspace.
- **User-visible behavior:** 32 variants: from primitive color tweaks (`Dim`, `Greyscale`, `Invert`, `Tint`) through structured emphasis (`Vignette`, `FadeToCanvas`) and per-glyph effects (`AnimatedGlyphRamp`, `GlyphStyle`, `GlyphTimeline`) to ambient atmospherics (`MatrixRain`, `Crt`, `BraillerDust`, `CharsetNoise`, `MotionBlur`, `InterlaceCurtain`) and component-level affordances (`PillButton`, `EdgeGrow`, `BracketEmphasis`, `DotIndicator`, `HoverBar`, `UnderlineWipe`, `KittScanner`, `ShadeScanner`, `RigidShake`, `SubCellShake`, `SubcellLight`, `SubPixelBar`, `GlistenSweep`, `PatternFill`, `ColorBridgedShade`, `ScalarFieldGlyph`).
- **Entry points:** Public `FilterSpec` enum at `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs:443-1374` (2825 LOC — largest single file). 33 variants total (verified against the `name()` arms at `:2248-2283`). Default = `None`. `#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]`.
- **Inputs:** `FilterSpec` discriminant + per-variant params; `SignalOrFloat` rate parameters on most variants; `BindableValue` (the three-arm Literal/Binding/Signal form) on the host-bindable variants (`progress`, `density`, `speed_multiplier`, `strength`, `canvas_color_binding`, `damping_scale_binding`, `num_shakes_binding`, etc.).
- **Outputs:** Per-cell transformations applied in place.
- **Options/config:** 33 variants — selected representative shapes:

  | Variant | Line | Key fields (defaults shown) |
  |---|---|---|
  | `Dim` | `:448` | `factor: SignalOrFloat = 0.5`, `apply_to: ApplyTo` |
  | `Tint` | `:461` | `color: ColorConfig`, `strength: SignalOrFloat = 0.5`, `apply_to: ApplyTo` |
  | `Vignette` | `:507` | `strength: SignalOrFloat = 0.5`, `radius: SignalOrFloat = 0.5`, `sides: Vec<VignetteEdge> = []`, `dither_amount: f32 = 0`, `temporal_dither_hz: f32 = 0` |
  | `FadeToCanvas` | `:485` | `canvas_color: ColorConfig = Rgb(0,0,0)`, `canvas_color_binding: Option<String>`, `strength: BindableValue`, `apply_to: ApplyTo = Both` |
  | `AnimatedGlyphRamp` | `:700` | `glyphs: String`, `cycles_per_second: f32 = 1.0`, `ease: EasingCurve`, `apply_to: AnimatedGlyphRampApplyTo = Foreground`, `affect: AnimatedGlyphRampAffect = NonEmpty`, `phase_offset_x_ms`, `phase_offset_y_ms`, `colors: Option<Vec<ColorConfig>>`, `color_gradient: Option<Gradient>` (XOR) |
  | `MatrixRain` | `:750` | `mode = Modern`, `density: BindableValue = 0.5`, `speed_multiplier: BindableValue = 1.0`, `speed_min: f32 = 5.0`, `speed_max: f32 = 15.0`, `trail_min: u16 = 8`, `trail_max: u16 = 20`, `glyph_change_hz: f32 = 8.0`, `seed: u64 = 42`, `affect = All`, `preset = Matrix`, `chars: Option<String>`, `head_color = Rgb(220,255,220)`, `tail_color = Rgb(0,160,0)` |
  | `RigidShake` | `:928` | `shake_period: f32`, `num_shakes: u8`, `num_shakes_binding: Option<String>`, `pause_duration: f32`, `max_eighths: u8`, `base_eighths: u8`, `damping: Vec<f32>`, `damping_scale_binding: Option<String>` (clamps to 0.1–10.0 at runtime), `element_color/bg_color: ColorConfig`, `inner_width: u16`, `margin_width: u8` (max 4) |
  | `KittScanner` | `:1172` | `boost: u8 = 50`, `band_width: f32 = 0.15`, `bpm: Option<f32>` (overrides bps), `bps: f32 = 1.2 ≈ 72bpm`, `progress: BindableValue`, `motion_mode = PingPong`, `axis = Horizontal`, `apply_to = Both`, `powerline_mode`, `boost_separator_bg` |
  | `ScalarFieldGlyph` | `:1285` | `sampler: SamplerRef` (`TerminalWater{shader}` / `TerminalFire{shader}`), `encoder: GlyphEncoderSpec` (5 variants — see F028), `threshold: f32 = 0`, `only_blank: bool = false`, `recolor: Option<GlyphRecolorSpec>` |
  | `GlyphTimeline` | `:1357` | `frames: Vec<GlyphTimelineFrameSpec>` (non-empty), `trigger: GlyphTimelineTriggerSpec`, `on_complete = Hold`, `apply_to = Foreground`, `affect = NonEmpty` |

  Remaining 23 variants: `None`, `Invert`, `Crt`, `PatternFill`, `Greyscale`, `BraillerDust`, `CharsetNoise`, `InterlaceCurtain`, `MotionBlur`, `ColorBridgedShade`, `SubPixelBar`, `SubcellLight`, `SubCellShake`, `HoverBar`, `UnderlineWipe`, `BracketEmphasis`, `DotIndicator`, `EdgeGrow`, `PillButton`, `GlistenSweep`, `ShadeScanner`, `GlyphStyle`. Their per-variant fields follow the same shape; full enumeration is in the file.

- **Data touched:** Cell color / style / glyph (read + written).
- **External systems:** None.
- **Errors and edge cases:** `validate() -> Result<(), String>` at `:2199` enforces AnimatedGlyphRamp colors-vs-glyphs invariants and non-empty GlyphTimeline frames/palettes. `try_from_v3_payload(Value) -> serde_json::Result<Self>` at `:2146` performs `rigid_shake` num_shakes/damping_scale binding-object normalization, then calls `validate()`. Module-level `pub fn kitt_bps_from_bpm(bpm: f32) -> f32` at `:2115` clamps to ≥ 0.1. No production-path `panic!`/`unwrap()`/`expect()`.
- **Observability:** Per-stage emit via F002 — `inspector.on_filter_applied(before, after, "Name#i")` at `orc_render_pipeline.rs:1058` per cell per filter.
- **Tests:** `crates/tui-vfx-compositor/tests/test_pipeline.rs`; per-filter inline `#[cfg(test)]` modules.
- **Evidence:** `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs:443-1374` (33 variants); `:2115` (kitt_bps helper); `:2146` (try_from_v3); `:2199` (validate); `:2248-2283` (name dispatch); `crates/tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs` (2186 LOC — prepare logic); 25 internal `cls_*.rs` files at `crates/tui-vfx-compositor/src/filters/`.
- **Confidence:** High.
- **Unknowns:** None of operational note for the 32-variant taxonomy. Per-variant default-value semantics for less-common fields can be looked up directly in the file by line number.

### F006 — Shadow rendering

- **Status:** implemented.
- **Description:** Cell-grid-native shadow extrusion that runs **before** the element it shadows. Shadow output is composed with the destination scene; renderer chooses among five visual styles (F007).
- **User-visible behavior:** A drop shadow appears behind a `Rect`-bounded element with configurable offset, alpha, edges, soft edges, animation progress, and composite mode.
- **Entry points:** `render_shadow` and `render_shadow_into_scene` — public re-exports at `crates/tui-vfx-shadow/src/lib.rs:355-359` (`pub use fnc_render_shadow::{...}`); `extract_shadow_envelope` and `CellMask` at `:354` (`pub use fnc_extract_shadow_envelope::{CellMask, extract_shadow_envelope}`).
- **Inputs:** Per the rustdoc Quick Start at `crates/tui-vfx-shadow/src/lib.rs:35-50`: `&mut Grid`, `element_rect: Rect`, `&ShadowConfig`, `progress: f32`. The `_into_scene` variant additionally takes `&mut SemanticScene` so shadow cells are tagged with `RoleTag::Shadow` (per `Cargo.toml:7` `<CLOG>` block describing Phase A.3 changes).
- **Outputs:** Mutation of the destination grid / scene.
- **Options/config:** `ShadowConfig::new(color)`, `.with_offset(dx, dy)`, `.with_style(ShadowStyle)`, `.with_edges(ShadowEdges)`, `.with_soft_edges(bool)`, plus `ShadowGradeConfig` for color grading and `ShadowCompositeMode` for the composite blend (per `crates/tui-vfx-shadow/src/lib.rs:360`).
- **Data touched:** Cells in the shadow rectangle. With the `_into_scene` variant: `SemanticScene` role map (writes `RoleTag::Shadow`).
- **External systems:** None.
- **Errors and edge cases:** Element rectangles that extend off-grid are clipped (per the inner renderer logic — not enumerated by reading every line). `ShadowConfig` is no longer `Copy` (per `Cargo.toml:7` `<CLOG>`) because `RoleTag::Custom` carries `Arc<str>`.
- **Observability:** Shadow stage emits trace events via F002 (verified by `crates/tui-vfx-compositor/tests/test_shadow_stage_emits.rs` and `test_shadow_path_emits_shader_scope_skip.rs`).
- **Tests:** `crates/tui-vfx-shadow/tests/test_cls_shadow_config.rs`, `test_fnc_extract_shadow_envelope.rs`, `test_fnc_render_shadow.rs`. Test peer coverage for `tui-vfx-shadow` is 100 % (chapter 14).
- **Evidence:** `crates/tui-vfx-shadow/src/lib.rs:1-90` (full module-level rustdoc + Quick Start example), `:350-363` (re-exports). `crates/tui-vfx-shadow/src/fnc_render_shadow.rs`, `crates/tui-vfx-shadow/src/fnc_extract_shadow_envelope.rs`, `crates/tui-vfx-shadow/src/types/`. The compositor side wraps it: `tui_vfx_compositor::types::ShadowSpec` at `crates/tui-vfx-compositor/src/types/cls_shadow_spec.rs:49-53` is `pub struct ShadowSpec { #[serde(flatten)] pub config: ShadowConfig }` with `new(ShadowConfig)` (`:57`), `simple(color, dx, dy)` (`:67`), `extra_width()` / `extra_height()` (`:78, :87` — both return `unsigned_abs() as usize`), `element_offset_x()` / `element_offset_y()` (`:95, :107`), and `From<ShadowConfig> for ShadowSpec` (`:116`). No `Default` impl.
- **Confidence:** High.
- **Unknowns:** Full `ShadowEdges` bitflag set and `ShadowCompositeMode` variant list — present in the cited types but not enumerated here.

### F007 — Shadow renderer styles (5 visual modes)

- **Status:** implemented.
- **Description:** Five visual rendering strategies for the shadow shape: braille, gradient, half-block, medium-shade, solid. Selected via `ShadowStyle` (per `crates/tui-vfx-shadow/src/lib.rs:360`).
- **User-visible behavior:** Different cell-glyph + color choices for the same shadow geometry.
- **Entry points:** Files under `crates/tui-vfx-shadow/src/renderers/`: `cls_braille.rs`, `cls_gradient.rs`, `cls_half_block.rs`, `cls_medium_shade.rs`, `cls_solid.rs`. Re-exported via `crates/tui-vfx-shadow/src/lib.rs:363`.
- **Inputs:** Same as F006 plus the selected style discriminant.
- **Outputs:** Per-cell glyph + color writes.
- **Options/config:** Sub-cell precision via the half-block / braille variants per the rustdoc (`lib.rs:13`).
- **Data touched:** Cells in the shadow rectangle.
- **External systems:** None.
- **Errors and edge cases:** None enumerated; failure modes inherit from F006.
- **Observability:** Inherits F006's trace emission.
- **Tests:** `crates/tui-vfx-shadow/tests/test_fnc_render_shadow.rs` (renderer paths exercised end-to-end).
- **Evidence:** `crates/tui-vfx-shadow/src/renderers/` (5 `cls_*.rs` files + `mod.rs`), `crates/tui-vfx-shadow/src/lib.rs:11-12` (rustdoc enumerates the five styles), `:363`.
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F008 — Style-effect / shader catalog (named factories)

- **Status:** implemented.
- **Description:** A library of named shaders consumed by the shader stage, organized through the `StyleEffect` umbrella enum (11 variants, mostly delegating to a `Spatial(SpatialShaderType)` arm) and `SpatialShaderType` (31 variants — one per shader family).
- **User-visible behavior:** 31 spatial shaders catalogued below.
- **Entry points:**
  - **`StyleEffect` enum** at `crates/tui-vfx-style/src/models/cls_style_effect.rs:81` — 11 variants, hand-rolled serde via `StyleEffectSerde:251`:

  | Variant | Fields |
  |---|---|
  | `FadeIn` | `apply_to: FadeApplyTo, ease: EasingCurve, from: FadeTarget` |
  | `FadeOut` | `apply_to: FadeApplyTo, ease: EasingCurve, to: FadeTarget` |
  | `Pulse` | `frequency: f32, color: Color` (`#[config(opaque)]`) |
  | `Rainbow` | `speed: f32` |
  | `Glitch` | `seed: u64, intensity: f32, italic_start: Option<f32>, italic_end: Option<f32>` |
  | `NeonFlicker` | `stability: f32` |
  | `Spatial` | `shader: SpatialShaderType` |
  | `ItalicWindow` | `start: f32, end: f32` |
  | `ColorShift` | `hue_shift, saturation_shift, lightness_shift: f32` |
  | `ColorFade` | `target: Color, color_space: ColorSpace` (rustdoc at `:218` mentions `Oklch` but `ColorSpace` only has `Rgb`/`Hsl`/`Hct` — see chapter 11 for the doc-vs-code drift) |
  | `RigidShakeStyle` | `shake_period: f32, num_shakes: u8, pause_duration: f32` |

  - **`SpatialShaderType` enum** at `cls_spatial_shader_type.rs:157` — 31 variants, internally tagged `type`, `deny_unknown_fields`. Each variant wraps one `*Shader` newtype: `LinearGradient`, `BarberPole`, `Radar`, `Orbit`, `BorderSweep`, `Highlighter`, `Reflect`, `GlistenBand`, `GlitchLines`, `NeonFlicker`, `PulseWave`, `TerminalWater`, `TerminalFire`, `RadialSpiral`, `TracePropagation`, `TracePath`, `FocusedRowGradient`, `RevealWipe`, `StochasticSparkle`, `AmbientOcclusion`, `Bevel`, `Glow`, `EdgeSheen`, `ConcealedLight`, `Diffusion`, `FocusField`, `AffordanceWake`, `WayfindingNode`, `SubCellShake`, `ChromaticEdge`, `Cursor`. Methods include `try_from_v3_payload(Value)` at `:304+` which performs alias translation (e.g., `fractional_stripe_overlay`/`gradient_overlay`/`colored_overlay` rewrite to V2 type names) — recorded as architecture observation 11.16.

- **Inputs:** Per-shader parameter struct. Shared types: `ColorConfig`, `ColorRamp`, `ColorSpace` (3 variants — F010), `Gradient`, `GradientLut`, `BlendMode` (`Normal | Additive | Multiply | Screen | Overlay | Mix`, default `Normal` — at `cls_blend_mode.rs:18`), `FalloffType` (`Linear | Quadratic | Exponential`, default `Quadratic` — at `cls_falloff_type.rs:15`), `NoiseType` (`Uniform | Gaussian`, default `Uniform` — at `cls_noise_type.rs:20`), `FadeTarget` (`Black | White | Transparent | Base | Color { color: ColorConfig }` — at `cls_fade_spec.rs:20`), `FadeApplyTo` (`Foreground | Background | Both`, default `Both` — at `cls_fade_spec.rs:55`), `SignalColor`, `SignalOrFloat`.
- **Outputs:** Per-cell color + style contributions consumed by the shader stage.
- **Options/config:** Per-shader struct fields. Selected representative shapes (full per-shader catalogue available; truncated for chapter brevity):

  | Shader | file:line | Fields |
  |---|---|---|
  | `BarberPoleShader` | `cls_barber_pole_shader.rs:12` | `speed: f32 = 1, stripe_width: u16 = 2, gap_width: u16 = 2, color: ColorConfig` |
  | `BevelShader` | `cls_bevel_shader.rs:41` | `light_direction: LightDirection, highlight_intensity, shadow_intensity: f32, edge_width: u8` |
  | `DiffusionShader` | `cls_diffusion_shader.rs:71` | `source: DiffusionSource, color: ColorConfig, radius: u8, softness, edge_firmness: f32, falloff: FalloffType, intensity: SignalOrFloat, apply_to: DiffusionApplyTo, mode: DiffusionMode, drift_speed, drift_amount: f32` |
  | `GlowShader` | `cls_glow_shader.rs:19` | `color: ColorConfig, radius: u8, falloff: FalloffType, intensity, pulse_speed: f32` |
  | `RevealWipeShader` | `cls_reveal_wipe_shader.rs:38` | `direction: RevealDirection` (single field) |
  | `TerminalFireShader` | `cls_terminal_fire_shader.rs:270` | 16 fields: `mode: FireMode, apply_to: FireApplyTo, aspect = 1.0, base_width = 0.55, min_width = 0.06, wind = 0.0, rise_speed = 2.2, turbulence = 1.0, intensity = 1.0, density = 1.0, cooling = 0.78, flicker_strength = 0.18, blue_core_strength = 0.35, white_core_strength = 0.35, smoke_strength = 0.35`, plus `sparks: FireSparkConfig` (5 sub-fields) and `palette: FirePalette` (6 colors) |
  | `TerminalWaterShader` | `cls_terminal_water_shader.rs:179` | 19 fields: `mode: WaterWaveMode, layers: u8 = 3, amplitude = 0.35, wavelength = 12.0, speed = 1.0, direction_deg = 25.0, steepness = 0.45, normal_strength = 1.4, diffuse = 0.65, specular = 0.55, shininess = 24.0, fresnel = 0.35, foam = 0.5`, plus 3 colors and 4 glint params, plus `apply_to: WaterApplyTo` |
  | `CursorShader` | `cls_cursor_shader.rs:128` | `mode: CursorShaderMode, tint: ColorConfig, primary: Option<CursorShaderPrimary>, trail: Vec<CursorShaderTrail>` |
  | `WayfindingNodeShader` | `cls_wayfinding_node_shader.rs:49` | `color, nodes: Vec<WayfindingNode { x, y }>, radius: u8, intensity, current_index: Option<u16>, current_index_binding, previous_strength, future_strength, pulse_speed: f32, apply_to: WayfindingNodeApplyTo` |

  Remaining 22 shader structs follow the same shape — each declared at its own `cls_*_shader.rs` and listed in the F008 row of §3.1. The earlier "50 files" estimate has been corrected: the actual count is **31 spatial shaders** plus helper files (`cls_fire_field_signal.rs`, `cls_water_field_signal.rs`, `cls_trace_common.rs`, `cls_signal_color.rs`, `cls_color_ramp.rs`, `cls_gradient.rs`, `cls_gradient_lut.rs`, plus enum/region/style files).

- **Data touched:** Cell color + style.
- **External systems:** `mcu-hct` + `mcu-utils` (HCT color math) — see F010.
- **Errors and edge cases:** No production-path `panic!`/`unwrap()`/`expect()` in shader code. One `unwrap()` in `cls_gradient.rs:89-103` is guarded by length check (`if t <= self.stops.first().unwrap().0`); gradient stops are guaranteed non-empty by `Gradient::new` validation. **`BlendMode` has no in-tree consumer** — no shader struct field references it; recorded as architecture observation 11.16. **Doc-vs-code drift:** `StyleEffect::ColorFade` rustdoc at `cls_style_effect.rs:218` lists `Oklch` as available, but `ColorSpace` only has `Rgb | Hsl | Hct` — recorded as architecture observation 11.17.
- **Observability:** Shader stage emits per-stage trace events via F002.
- **Tests:** `crates/tui-vfx-style/tests/test_models.rs`, `test_shader_role_awareness.rs`; per-model fixtures under `tests/models/`. Test peer coverage 59 % (chapter 14).
- **Evidence:** `crates/tui-vfx-style/src/models/cls_style_effect.rs:81-250`; `cls_spatial_shader_type.rs:157-303`; per-shader files cited above; `cls_blend_mode.rs:18`; `cls_falloff_type.rs:15`; `cls_noise_type.rs:20`; `cls_fade_spec.rs:20, :55`.
- **Confidence:** High.
- **Unknowns:** None of operational note for the 31-shader taxonomy. Per-shader fields available at the cited line numbers.

### F009 — V3 shader family (`Vfx*` prefix)

- **Status:** partially implemented (V3 in flight; both V2 and V3 surfaces coexist at audit-time per Intention 8 and Decision 4). The 31-arm V2→V3 lowering is **complete**.
- **Description:** A V3 redesign of the shader / style-effect surface using the `Vfx*` prefix. The V3 family seam has three levels: `VfxStyleEffectFamily` (6 variants) → `VfxSpatialShaderFamily` (2 layers) → 4 primitives + 7 composed primitives.
- **User-visible behavior:** 11 new shader classes + 18 enum files (11 per-shader behavior enums + 5 family/structure enums + 2 lowering error enums) + a 31-arm V2→V3 lowering function. (`models/v3/` directory listing returns 30 files at audit-time; verified via `ls`.)
- **Entry points:**
  - **Family seam.** `VfxStyleEffectFamily` (`enum_vfx_style_effect_family.rs:20`, 6 variants `StyleFade | StyleModulation | TypographyWindow | StyleInstability | PairedCapability | Spatial(VfxSpatialShaderFamily)`); `VfxStyleEffectValue` (`enum_vfx_style_effect_value.rs:22`, the executable surface, with `from_legacy_style_effect`/`try_to_legacy_style_effect`); `VfxSpatialShaderFamily` (`enum_vfx_spatial_shader_family.rs:20` — `Primitive | ComposedPrimitive`); `VfxSpatialPrimitive` (`enum_vfx_spatial_primitive.rs:21`, 4 variants `SurfaceDepth | MotionField | EdgeDistortion | GradientReveal`); `VfxSpatialComposedPrimitive` (`enum_vfx_spatial_composed_primitive.rs:23`, 7 variants `TravelingBand | ProgressEmphasis | MaterialLight | GuidanceCue | StochasticTexture | Cursor | StripeMotion`).
  - **Lowering call site.** `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs:25-28` calls `ShaderWithRegion::try_from_v3_shader_family` (panics on lowering failure — REQ-002).
  - **V2→V3 lowering function.** `fnc_lower_legacy_spatial_shader.rs:23` — `pub fn lower_legacy_spatial_shader(shader: &SpatialShaderType) -> VfxSpatialShaderFamily`.
- **Inputs:** V3 spec types (per-shader file:line in §3.2 F009 table below).
- **Outputs:** Same shader-stage contributions as F008.

  **V3 shader struct shapes (pattern inconsistency surfaced):**

  | Shader | file:line | Shape |
  |---|---|---|
  | `VfxCursorShader` | `cls_vfx_cursor_shader.rs:23` | **flat layout** — `{ mode: VfxCursorMode, tint: ColorConfig, primary: Option<VfxCursorPrimary>, trail: Vec<VfxCursorTrail> }` (mirrors V2) |
  | `VfxEdgeDistortionShader` | `cls_vfx_edge_distortion_shader.rs:23` | `{ behavior: VfxEdgeDistortionBehavior }` |
  | `VfxGradientRevealShader` | `cls_vfx_gradient_reveal_shader.rs:21` | `{ behavior: VfxGradientRevealBehavior }` |
  | `VfxGuidanceCueShader` | `cls_vfx_guidance_cue_shader.rs:25` | `{ behavior: VfxGuidanceCueBehavior }` |
  | `VfxMaterialLightShader` | `cls_vfx_material_light_shader.rs:25` | `{ behavior: VfxMaterialLightBehavior }` |
  | `VfxMotionFieldShader` | `cls_vfx_motion_field_shader.rs:24` | `{ behavior: VfxMotionFieldBehavior }` |
  | `VfxProgressEmphasisShader` | `cls_vfx_progress_emphasis_shader.rs:26` | **flat layout — no `behavior:` field**; 13 sibling fields (`color, apply_to, text_contrast, mode, band_width = 6, soft_edge = 0.0, blend_strength = 1.0, blend_strength_binding, speed = 1.0, speed_binding, direction, direction_binding, row_mask`) |
  | `VfxStochasticTextureShader` | `cls_vfx_stochastic_texture_shader.rs:22` | `{ behavior: VfxStochasticTextureBehavior }` |
  | `VfxStripeMotionShader` | `cls_vfx_stripe_motion_shader.rs:18` | `{ behavior: VfxStripeMotionBehavior }` |
  | `VfxSurfaceDepthShader` | `cls_vfx_surface_depth_shader.rs:22` | `{ behavior: VfxSurfaceDepthBehavior }` |
  | `VfxTravelingBandShader` | `cls_vfx_traveling_band_shader.rs:27` | **mixed** — shared `speed: f32 = 1.0, color: VfxTravelingBandColor` lifted out + `behavior: VfxTravelingBandBehavior` |

  **Pattern inconsistency:** 8 of 11 V3 shaders wrap a single `behavior: VfxXBehavior` field; 3 (`VfxCursorShader`, `VfxProgressEmphasisShader`, `VfxTravelingBandShader`) lift fields out. Recorded as architecture observation 11.18.

- **Options/config (V3 behavior enums — 13):**

  | Enum | file:line | Variants |
  |---|---|---|
  | `VfxCursorMode` | `enum_vfx_cursor_behavior.rs:18` | `Off, Tint, Ghost` (+ helper sub-structs `VfxCursorPrimary`, `VfxCursorTrail`) |
  | `VfxEdgeDistortionBehavior` | `enum_vfx_edge_distortion_behavior.rs:33` | `GlitchLines{...} | ChromaticEdge{...} | SubCellShake{...}` |
  | `VfxGradientRevealBehavior` | `enum_vfx_gradient_reveal_behavior.rs:38` | `LinearGradient{...} | RevealWipe{...}` |
  | `VfxGuidanceCueBehavior` | `enum_vfx_guidance_cue_behavior.rs:71` | `FocusedRow{...} | FocusField{...} | AffordanceWake{...} | WayfindingNode{...}` |
  | `VfxMaterialLightBehavior` | `enum_vfx_material_light_behavior.rs:93` | `Diffusion{...} | ConcealedLight{...} | EdgeSheen{...}` |
  | `VfxMotionFieldBehavior` | `enum_vfx_motion_field_behavior.rs:35` | `PulseWave{...} | Radar{...} | Orbit{...} | RadialSpiral{...} | TerminalWater{...} | TerminalFire{...}` |
  | `VfxProgressEmphasisApplyTo`/`Mode`/`Direction`/`RowMask`/`TextContrast` | `enum_vfx_progress_emphasis_behavior.rs:20-85` | (helper enums for the flat shader) |
  | `VfxStochasticTextureBehavior` | `enum_vfx_stochastic_texture_behavior.rs:47` | `NeonFlicker{...} | StochasticSparkle{...}` |
  | `VfxStripeMotionBehavior` | `enum_vfx_stripe_motion_behavior.rs:18` | `BarberPole{...}` (single variant) |
  | `VfxSurfaceDepthBehavior` | `enum_vfx_surface_depth_behavior.rs:59` | `AmbientOcclusion{...} | Bevel{...} | Glow{...}` |
  | `VfxTravelingBandBehavior` | `enum_vfx_traveling_band_behavior.rs:80` | `Border{...} | Reflect{...} | GlistenBand{...} | TracePropagation{...} | TracePath{...}` |
  | `TryLowerV3SpatialShaderError` | `enum_try_lower_v3_spatial_shader_error.rs:14` | (lowering error type) |
  | `TryLowerV3StyleEffectError` | `enum_try_lower_v3_style_effect_error.rs:16` | `MismatchedVariant { expected_family, actual_effect }` etc. |

- **V2→V3 lowering map (31 V2 variants → 11 V3 families, surjective):**

  Primitive layer (12 V2 → 4 primitive families):

  | V2 variant | V3 family |
  |---|---|
  | `LinearGradient`, `RevealWipe` | `Primitive::GradientReveal` |
  | `Radar`, `Orbit`, `PulseWave`, `RadialSpiral`, `TerminalWater`, `TerminalFire` | `Primitive::MotionField` |
  | `GlitchLines`, `SubCellShake`, `ChromaticEdge` | `Primitive::EdgeDistortion` |
  | `AmbientOcclusion`, `Bevel`, `Glow` | `Primitive::SurfaceDepth` |

  Composed-primitive layer (19 V2 → 7 composed families):

  | V2 variant | V3 family |
  |---|---|
  | `BarberPole` | `ComposedPrimitive::StripeMotion` |
  | `BorderSweep`, `Reflect`, `GlistenBand`, `TracePropagation`, `TracePath` | `ComposedPrimitive::TravelingBand` |
  | `Highlighter` | `ComposedPrimitive::ProgressEmphasis` |
  | `NeonFlicker`, `StochasticSparkle` | `ComposedPrimitive::StochasticTexture` |
  | `EdgeSheen`, `ConcealedLight`, `Diffusion` | `ComposedPrimitive::MaterialLight` |
  | `FocusedRowGradient`, `FocusField`, `AffordanceWake`, `WayfindingNode` | `ComposedPrimitive::GuidanceCue` |
  | `Cursor` | `ComposedPrimitive::Cursor` |

  31/31 V2 variants reached. The match is exhaustive; adding a V2 variant is a build-time error.

- **Data touched:** Cell color + style (same as F008).
- **External systems:** `mixed-signals` (signal expressions in spec params).
- **Errors and edge cases:** V3 lowering errors are typed (`TryLowerV3SpatialShaderError`, `TryLowerV3StyleEffectError`). The pipeline currently `.expect`s the lowering succeeds (F001 / REQ-002).
- **Observability:** Same as F002 / F008.
- **Tests:** Inline `#[cfg(test)]` modules within each V3 file; cross-stage tests in `crates/tui-vfx-compositor/tests/test_pipeline.rs`.
- **Evidence:** `crates/tui-vfx-style/src/models/v3/` (11 `cls_vfx_*_shader.rs` + 18 `enum_*.rs` + 1 `fnc_lower_legacy_spatial_shader.rs` = 30 files); `fnc_render_pipeline_with_spec.rs:23-28` (lowering call site); per-enum file:line citations in the tables above.
- **Confidence:** High.
- **Unknowns:** None of operational note. The V3 cutover roadmap (which V2 shaders retire when) is design-doc territory tracked outside this PRD.

### F010 — HCT perceptual color-space integration

- **Status:** implemented.
- **Description:** Material Color Utilities' HCT (Hue, Chroma, Tone) color space wired into the style crate so brightness scaling and gradient interpolation can route through perceptually uniform tone space. The `ColorSpace` enum has three variants — `Rgb` (default; per-channel linear interpolation), `Hsl` (shortest-path hue interpolation), `Hct` (CAM16-based via `mcu-hct`, added at v1.1.0 on 2026-04-26).
- **User-visible behavior:** Color ramps and brightness-modulating effects produce tonally consistent output across hues when the `Hct` variant is selected.
- **Entry points:** `ColorSpace` enum at `crates/tui-vfx-style/src/models/cls_color_space.rs:31` — flat 3-variant `#[serde(rename_all = "snake_case")]` enum with `#[default] = Rgb`. The `Hct` variant routes through `mcu-hct` per the `Cargo.toml:33-34` deps and the workspace's `mcu-hct = "0.2.0"` / `mcu-utils = "0.2.0"` at `Cargo.toml:58-59`.
- **Inputs:** `Color` (`tui-vfx-types::Color`), `ColorSpace` discriminant.
- **Outputs:** Interpolated / brightness-scaled `Color` in the requested space.
- **Options/config:** `ColorSpace::{Rgb, Hsl, Hct}` — exactly three variants. **No `Oklch` variant** despite a rustdoc reference at `cls_style_effect.rs:218`. Doc-vs-code drift recorded as architecture observation 11.17.
- **Data touched:** `Color` only.
- **External systems:** `mcu-hct 0.2.0` and `mcu-utils 0.2.0` (`Cargo.toml:58-59`, `crates/tui-vfx-style/Cargo.toml:33-34`).
- **Errors and edge cases:** Out-of-gamut handling lives in `mcu-hct`. No production-path panic in the style crate's color-space dispatch.
- **Observability:** None of its own; effects that consume it inherit F002.
- **Tests:** `crates/tui-vfx-style/tests/test_models.rs`.
- **Evidence:** `crates/tui-vfx-style/src/models/cls_color_space.rs:31` (the 3-variant enum); `crates/tui-vfx-style/Cargo.toml:3-4` (`<WCTX>`/`<CLOG>`); `:33-34` (deps); `Cargo.toml:53-59` (workspace mcu-* declarations + HCT rationale comment).
- **Confidence:** High.
- **Unknowns:** None.

### F011 — Bindable value system

- **Status:** implemented.
- **Description:** A three-arm wire form for parameter values: `Literal(T)`, `Binding(String)`, `Signal(SignalOrFloat | …)`. The host supplies bindings per frame via `ShaderRuntimeParams`; signals evaluate against `SignalContext` from `mixed-signals`.
- **User-visible behavior:** Effect parameters can be static numbers, named runtime bindings the host updates per frame, or composed `mixed-signals` expressions that evaluate dynamically.
- **Entry points:** Canonical types live in `tui_vfx_core::bindable`: `VfxBindable<T, S = Never>` at `crates/tui-vfx-core/src/bindable/cls_bindable.rs:167` (with `Never` enum at `:24`, `RuntimeParamsRead` trait at `:49`, `BindableSignal` trait at `:73`); re-exported at `crates/tui-vfx-core/src/lib.rs:11-15`. The style-side `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` and `cls_bindable_string.rs` (each 14 LOC) are now **re-export shims** — `pub use tui_vfx_core::bindable::VfxBindableString as BindableString` (`= VfxBindable<String, Never>`) and `pub use ... VfxBindableU16 as BindableU16` (`= VfxBindable<u16, Never>`); their original 322-LOC and 250-LOC bodies were retired to `recyclebin/` per the sweep 1.2.A consolidation. Compositor-side `BindableValue` (`= VfxBindableValue`) re-export at `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs:14`. Wire format: bare number → `Literal`; `{"literal": N}` → `Literal`; `{"binding": "k"}` → `Binding`; `{"signal": <SignalOrFloat>}` → `Signal`; bare-`SignalSpec` fallback (e.g. `{"type": "sine", ...}`).
- **Inputs:** `evaluate(loop_t, signal_ctx, runtime_params) -> Option<T>`-style methods (signature confirmed for the `f32` arm in the archived progress notes for packet 69-A: `bindable.rs:451-467`).
- **Outputs:** A resolved value of type `T` (or `Option<T>` when the binding is missing).
- **Options/config:** None of its own — it is the parameter type.
- **Data touched:** `ShaderRuntimeParams` (read), `SignalContext` (read).
- **External systems:** `mixed-signals` (signal substrate).
- **Errors and edge cases:** A `Binding(name)` with no matching `ShaderRuntimeParams` entry returns `None` (per the `Option<f32>` return type captured in archived evidence). The strict-contracts validator enforces every `requires_bindings` declaration ships with an effective loopback (Intention 37), but that is recipe-side, not engine-side.
- **Observability:** Binding evaluations are observable via the F002 / F040 trace and probe surfaces (the probe DTO catalogue includes `ProbeRuntimeContext` / `ProbeRuntimeParam`).
- **Tests:** `crates/tui-vfx-core/src/bindable/test_cls_bindable.rs` (in-source); `crates/tui-vfx-content/tests/test_content_effect_apply.rs` (exercises `VfxBindableValue::Literal` per the `lib.rs:33-46` doctest); `crates/tui-vfx-content/tests/test_transformers.rs`.
- **Evidence:** `crates/tui-vfx-core/src/lib.rs:11-15` (the `pub use bindable::{...}` block); `crates/tui-vfx-core/src/bindable/cls_bindable.rs:24, :49, :73, :167` (Never / RuntimeParamsRead / BindableSignal / VfxBindable); `crates/tui-vfx-style/src/models/cls_bindable_u16.rs:14`, `cls_bindable_string.rs:14` (the shims); `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs:14` (compositor-side alias); `crates/tui-vfx-content/src/lib.rs:53-90` (the three-arm rustdoc).
- **Confidence:** High.
- **Unknowns:** None. Architecture observation 11.19 records that no in-tree shader actually consumes the `BindableString`/`BindableU16` aliases — bindings are still raw `Option<String>` keys throughout V2 and V3 progress-emphasis shaders.

### F012 — Content text transformers

- **Status:** implemented.
- **Description:** A 15-variant family of text transformations (per-glyph effects) that take an input string + `progress: f64` and emit an output string. The unified entry point is the `ContentEffect` enum with an inherent `apply` / `apply_to_borrowed` / `apply_with_runtime` family. Each transformer implements the `TextTransformer` trait at `crates/tui-vfx-content/src/traits/text_transformer.rs:13` (v3.0.0): `fn transform<'a>(&self, target: &'a str, progress: f64, ctx: &TransformContext<'_>) -> Cow<'a, str>`.
- **User-visible behavior:** Effects span character-by-character reveal (Typewriter), character resolution (Scramble), masking (Redact), interpolated numerics (Numeric), scrolling windows (Marquee), mechanical drum cycles (SplitFlap, Odometer), morphs (Morph, Dissolve), glitches (GlitchShift, ScrambleGlitchShift), slides (SlideShift), mirroring (Mirror), wrap indicators (WrapIndicator), and per-glyph staggered cascades (GlyphCascade).
- **Entry points:** `ContentEffect` enum at `crates/tui-vfx-content/src/types/cls_content_effect.rs:179` (v2.16.0; `#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]`, plus `#[allow(clippy::large_enum_variant)]` at `:178`). Three inherent apply methods at `crates/tui-vfx-content/src/types/fnc_apply_content_effect.rs`: `apply(&str, f64) -> String` (`:49`), `apply_to_borrowed(&str, f64) -> Cow<'_, str>` (`:68` — defaults both context halves), `apply_with_runtime(&str, f64, &SignalContext, &ShaderRuntimeParams) -> Cow<'_, str>` (`:87` — host-injection path). Dispatcher: `fnc_get_transformer.rs:12` (`pub fn get_transformer(effect: &ContentEffect) -> Box<dyn TextTransformer>`). Per-variant transformer classes under `crates/tui-vfx-content/src/transformers/` (15 `cls_*.rs` files).
- **Inputs:** `&str` target; `progress: f64` (0.0–1.0 typical); `TransformContext<'a>` at `crates/tui-vfx-content/src/traits/cls_transform_context.rs:19` (v1.0.0) — a `Copy` two-reference bundle: `pub signal_ctx: &'a SignalContext`, `pub runtime_params: &'a ShaderRuntimeParams`. Constructor `TransformContext::new(signal_ctx, runtime_params)` at `:36`.
- **Outputs:** `String` (or `Cow<'_, str>` from the borrowed fast path).
- **Options/config:** Per-transformer field surface (verified end-to-end). The 15 transformers and their fields:

  | Transformer | File:line | Bindable rate fields | Other fields | `Default`? | `Default` values |
  |---|---|---|---|---|---|
  | **Typewriter** | `cls_typewriter.rs:16` (v4.1.0) | `pub speed_variance: VfxBindableValue` | `cursor: Option<TypewriterCursor>` (on enum side) | yes | `Literal(0.0)` (`:30`) |
  | **Scramble** | `cls_scramble.rs:17` (v3.2.0) | `resolve_pace: VfxBindableValue` (private) | `seed: u64`, `charset: ScrambleCharset` | yes | seed 0, `Alphanumeric`, `Literal(1.0)` (`:34`) |
  | **GlitchShift** | `cls_glitch_shift.rs:17` (v2.3.0) | `glitch_start, glitch_end: VfxBindableValue` (private) | `shift_amount: u8`, `seed: u64` (`#[allow(dead_code)]`) | yes | 5, `Literal(0.3)`, `Literal(0.4)`, 0 (`:41`) |
  | **ScrambleGlitchShift** | `cls_scramble_glitch_shift.rs:21` (v3.3.0) | `resolve_pace, glitch_start, glitch_end: VfxBindableValue` (private) | `scramble_seed: u64`, `charset: ScrambleCharset`, `shift_amount: u8` | yes | 0, `Binary`, 5, `Literal(0.3)`, `Literal(0.4)`, `Literal(1.0)` (`:53`) |
  | **GlyphCascade** | `cls_glyph_cascade.rs:15` (v1.1.0) | (none) | `alphabet`, `pattern`, `direction: DissolveDirection`, `seed: u64`, `mode` | **no** | n/a |
  | **SplitFlap** | `cls_split_flap.rs:119` (v3.6.0) | `speed, cascade, cycles: VfxBindableValue` (all `pub`) | 14 more `pub` fields incl. `jitter: f32`, `charset: SplitFlapCharset`, `settle_overshoot/leading_blocks: f32`, `rolling_flip/flip_preview/flip_flicker/settle_hinge/spring_settle/authentic_timing: bool`, `from_message: Option<String>`, `dispersion: SplitFlapDispersion`, `tile_width/tile_height: u16` | **no** | uses `new`, `new_mechanical`, `solari_preset` |
  | **Odometer** | `cls_odometer.rs:23` (v4.1.0) | (none) | `direction: OdometerDirection`, `travel: OdometerTravel`, `tile_width/height: u16`, `from_message: Option<String>`, `mechanical: Option<MechanicalCycleConfig>` | **no** | n/a |
  | **Redact** | `cls_redact.rs:10` (v2.1.0) | (none) | `symbol: char` (private) | yes | `'█'` (`:18`) |
  | **Numeric** | `cls_numeric.rs:9` (v1.1.0) | (none) | `format_str: String` (private) | yes | `"{}"` (`:19`) |
  | **Marquee** | `cls_marquee.rs:12` (v2.2.0) | `speed: VfxBindableValue` (private) | `width: u16` | yes | width 10, `Literal(1.0)` (`:25`) |
  | **SlideShift** | `cls_slide_shift.rs:13` (v1.4.0) | (none) | 8 private fields incl. `start_col/end_col/start_row/shift_col/row_shift: i16`, `shift_width: u16`, `line_mode`, `flow_mode` | **no** | n/a |
  | **Mirror** | `cls_mirror.rs:17` (v2.1.0) | (none) | `axis: MirrorAxis` (private) | yes | `Horizontal` (`:41`) |
  | **Dissolve** | `cls_dissolve.rs:18` (v1.2.0; `derive(Default)`) | (none) | `replacement`, `pattern`, `direction`, `seed: u64` (private) | yes (derived) | derive defaults |
  | **Morph** | `cls_morph.rs:21` (v1.9.0; `derive(Default)`) | (none) | `source: String`, `progression`, `direction`, `seed: u64` (private) | yes (derived) | derive defaults |
  | **WrapIndicator** | `cls_wrap_indicator.rs:30` (v1.1.0) | (none) | `pub prefix: String`, `pub suffix: String` | **no** | uses `new`, `arrows`, `brackets`, `angles` |

- **Data touched:** Per-grapheme reads of the input via `unicode-segmentation 1.10` (per `crates/tui-vfx-content/Cargo.toml:20`); no shared mutable state. `Cow::Borrowed(target)` is the per-transformer fast path when `progress >= 1.0` or the effect has fully resolved (verified for Typewriter, Scramble, SplitFlap, Odometer, Mirror, Redact, Marquee, Morph, Dissolve, WrapIndicator).
- **External systems:** None.
- **Errors and edge cases:**
  - Strict Unicode safety via grapheme clusters (per `crates/tui-vfx-content/src/lib.rs:9-10`).
  - **No production-path `panic!` / `unwrap()` / `expect(...)` was observed in any transformer file** (excluding `#[cfg(test)]` modules) — every bindable evaluation uses `.evaluate(progress, ctx.signal_ctx, ctx.runtime_params).unwrap_or(<sane default>)`.
  - 11 rate-bearing field evaluations distribute as: Typewriter 1 (default 0.0), Scramble 1 (default 1.0, clamped `.max(0.1)`), GlitchShift 2 (default 0.0, clamped `.clamp(0.0, 1.0)`), ScrambleGlitchShift 3, SplitFlap 3, Marquee 1 (default 1.0, clamped `.max(0.0)`).
  - Odometer additionally forwards `ctx.runtime_params` into the mechanical-cycle resolver (binding-form font names) at `:88, :136`; resolver failure falls back to the legacy pair-roll without panic at `:142`. Per-tile route failure further degrades to legacy pair-roll for that tile only (`:208`).
  - SplitFlap diagnostic helper: `estimated_flap_ms(animation_ms) -> Option<f32>` (`:313`) — returns `None` when `cycles` is non-`Literal` (cannot estimate symbolic).
  - 8 of 15 transformers ignore `ctx` (`_ctx`): GlyphCascade, Mirror, Morph, Numeric, Redact, Dissolve, SlideShift, WrapIndicator. All deterministic from `progress + seed`.
- **Observability:** Transformers do not emit trace events directly; their integration with the compositor pipeline emits via F002 when used inside a filter.
- **Tests:** `crates/tui-vfx-content/tests/test_transformers.rs`, `test_content_effect_apply.rs`, `test_typewriter_cursor.rs`, `test_cell_motion.rs`, `test_glyph_particles.rs`, `test_cursor.rs`, `test_utils.rs` (7 integration files); plus the `tests/cursor/`, `tests/transformers/`, `tests/utils/` subdirectories. Test peer coverage 76.6 % (chapter 14).
- **Evidence:** `crates/tui-vfx-content/src/lib.rs:1-100` (module rustdoc with three doctests at `:33-46`, `:69-83`, `:96-104`); `crates/tui-vfx-content/src/traits/text_transformer.rs:13` (the trait); `cls_transform_context.rs:19,36` (the context); 15 transformer files cited in the table above; `cls_content_effect.rs:179-557` (the enum + inherent methods `name` `:557`, `terse_description` `:578`, `key_parameters` `:601`); `fnc_apply_content_effect.rs:49,68,87` (the apply trio); `fnc_get_transformer.rs:12` (dispatcher).
- **Confidence:** High.
- **Unknowns:** None of operational note for the 15-variant taxonomy. Two consistency observations recorded in chapter 11: (1) field-visibility asymmetry — only Typewriter.speed_variance, all 17 SplitFlap fields, and WrapIndicator's two fields are `pub`; the other transformers' fields are private with no accessors, so recipe-construction goes through `new` constructors. (2) Constructor arg-type churn — `Numeric::new` takes `&str` (clones internally); all others take owned `String` where applicable. (3) `Default` coverage gap — SplitFlap, Odometer, SlideShift, WrapIndicator have no `Default` impl and require their respective constructors.

### F013 — Cursor primitive

- **Status:** implemented.
- **Description:** Visible insertion / typewriter cursor that overlays terminal-style glyphs (block, underscore, pipe, caret, custom) onto cell positions.
- **User-visible behavior:** A blinking or static cursor glyph at the typewriter's current position, optionally with a shader-side visual accent.
- **Entry points:** `crates/tui-vfx-content/src/cursor/` (sub-module); `TypewriterCursor` constructors at `crates/tui-vfx-content/src/types/` per the rustdoc doctest at `crates/tui-vfx-content/src/lib.rs:96-104` (`TypewriterCursor::block()`, `.underscore()`, `.pipe()`, `.caret()`, `.simple(char)`); shader-side `cls_cursor_shader.rs` at `crates/tui-vfx-style/src/models/`.
- **Inputs:** `TypewriterCursor` constructor; cell position from the typewriter transformer.
- **Outputs:** Cell glyph + style write at the cursor position.
- **Options/config:** Cursor glyph variant (block / underscore / pipe / caret / simple); shader-side animation parameters live in `cls_cursor_shader.rs`.
- **Data touched:** Cells at the cursor position.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** Inherits F002 when running inside the compositor pipeline; standalone use (just the typewriter transformer) does not emit.
- **Tests:** `crates/tui-vfx-content/tests/test_typewriter_cursor.rs`, `test_cursor.rs`, plus the `tests/cursor/` subdirectory; cross-stage `crates/tui-vfx-compositor/tests/cursor_integration.rs`.
- **Evidence:** `crates/tui-vfx-content/src/lib.rs:96-104` (doctest), `crates/tui-vfx-content/src/cursor/` (directory presence), `crates/tui-vfx-style/src/models/cls_cursor_shader.rs`.
- **Confidence:** High.
- **Unknowns:** Whether the cursor primitive is exclusive to the Typewriter transformer or whether other transformers share its cell-overlay logic.

### F014 — Cell-motion scheduler

- **Status:** implemented.
- **Description:** Schedules content-local cell motion (a path that individual cells take over the duration of a transformer) against the geometry crate's path library.
- **User-visible behavior:** Per-glyph cell paths during transformer phases (e.g., a `SplitFlap` flapping along a path or a `Marquee` cell sliding).
- **Entry points:** `crates/tui-vfx-content/src/cell_motion/` directory (sub-module declared at `crates/tui-vfx-content/src/lib.rs:112`).
- **Inputs:** Path specs from `tui-vfx-geometry::paths` (F023); per-cell start / end positions and timing.
- **Outputs:** Per-frame cell positions consumed by the transformer.
- **Options/config:** Path parameters; not enumerated at this level.
- **Data touched:** Cell positions only.
- **External systems:** None — composes `tui-vfx-geometry` (added "for cell-motion scheduler" per `crates/tui-vfx-content/Cargo.toml:29-30` inline comment).
- **Errors and edge cases:** None enumerated.
- **Observability:** None of its own.
- **Tests:** `crates/tui-vfx-content/tests/test_cell_motion.rs`.
- **Evidence:** `crates/tui-vfx-content/src/lib.rs:112` (`pub mod cell_motion`), `crates/tui-vfx-content/Cargo.toml:29-30` (the `tui-vfx-geometry` dep with inline comment), `crates/tui-vfx-content/tests/test_cell_motion.rs`.
- **Confidence:** High (directory + test + Cargo comment trail). Medium on parameter surface (full read needed).
- **Unknowns:** Public-API shape of the scheduler — whether it is `pub mod` open API or only consumed internally by transformers.

### F015a — Cell-motion scheduler (V3 packet 1, public surface — supersedes F014's row)

The earlier F014 row described `cell_motion/` at the directory level. The deeper read shows `cell_motion/` is a **substantially public** surface — `mod.rs` declares `CellActor`, `CellMotionAffect`, `CellMotionCoord`, `CellMotionError`, `CellMotionPhaseSpec`, `CellMotionScope`, `CellMotionSpec`, `CellMotionStats`, `CellMotionVisibility`, plus enum types `CellCollisionMode`, `CellPlacement`, `CellStagger`, plus public `fnc_apply_cell_motion`, `fnc_collect_cell_actors`, `fnc_resolve_actor_offset_ms`, `fnc_resolve_cell_placement` (`crates/tui-vfx-content/src/cell_motion/mod.rs:1-30`). It is described as the "Pure content per-cell motion scheduler for V3 source-cell remapping." This expands the F014 entry from `Medium` to `High` confidence on parameter surface; the V3 packet 1 framing is recorded.

### F015 — Mechanical content cycles

- **Status:** implemented.
- **Description:** A circular content-cycle primitive shared by mechanical-style transformers (split-flap, odometer, numeric drums). The "mechanical circular content cycles plan" is named in archived progress notes (`.omc/archive/2026-04-28-packet-69-A/progress.txt`).
- **User-visible behavior:** Drum-style cycling content where the visible glyph rotates through a cycle list with cadence.
- **Entry points:** `crates/tui-vfx-content/src/mechanical/` (note: this module is declared `mod mechanical;` *without* `pub` at `crates/tui-vfx-content/src/lib.rs:116`, so it is a private implementation detail consumed by the transformer family rather than direct public surface).
- **Inputs:** Cycle list (the visible glyphs); cadence; current position.
- **Outputs:** Current visible glyph for a given cycle position.
- **Options/config:** Per-transformer cadence parameters; for SplitFlap these are `speed`, `cascade`, `cycles` (typed `VfxBindableValue` after packet 69-A).
- **Data touched:** Cycle state.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** None of its own.
- **Tests:** Internal — no direct `tests/test_mechanical*.rs` exists (filesystem listing). The mechanical surface is exercised via `crates/tui-vfx-content/tests/test_transformers.rs`.
- **Evidence:** `crates/tui-vfx-content/src/lib.rs:116` (`mod mechanical;` — note the missing `pub`), `crates/tui-vfx-content/src/mechanical/` (directory presence), archived 69-A progress trail.
- **Confidence:** Medium. Behavior is real and shipped, but the surface is private and the test coverage is indirect (via the transformer-family tests, not a dedicated `test_mechanical*.rs`).
- **Unknowns:** Whether the mechanical surface is intentionally private or whether its `pub mod` upgrade is pending.

### F016 — Glyph particles

- **Status:** implemented (presence; integration shape needs read).
- **Description:** Per-cell particle emitters (sparkles, fire, drip-style particles) bound to a cell field.
- **User-visible behavior:** Animated particle overlays on cells, observable in the dedicated tests.
- **Entry points:** `crates/tui-vfx-content/src/glyph_particles/` (declared `pub mod glyph_particles;` at `crates/tui-vfx-content/src/lib.rs:115`); the directory contains only `mod.rs` at audit-time.
- **Inputs:** Per `mod.rs` (full read needed); the test file exercises the public surface end-to-end.
- **Outputs:** Cell glyph + style writes.
- **Options/config:** Per the (unread) `mod.rs` declarations.
- **Data touched:** Cells.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** None of its own.
- **Tests:** `crates/tui-vfx-content/tests/test_glyph_particles.rs`.
- **Evidence:** `crates/tui-vfx-content/src/lib.rs:115` (`pub mod glyph_particles`), `crates/tui-vfx-content/src/glyph_particles/mod.rs` (file presence — single-file module), `crates/tui-vfx-content/tests/test_glyph_particles.rs`.
- **Confidence:** Medium. Single-file module + dedicated test confirms it works; the public type / function surface is not enumerated here.
- **Unknowns:** Public-API shape (struct names, parameters).

### F017 — RocketsplashImage source (`.rss` braille image consumer)

- **Status:** implemented.
- **Description:** A source primitive that consumes a `rocketsplash-rt` `.rss` image asset and blits its cells into a `tui-vfx` `Grid`.
- **User-visible behavior:** Render a static or animated braille / cell-art image as a scene-layer source.
- **Entry points:** `RocketsplashImage` re-exported at `crates/tui-vfx-content/src/sources/mod.rs:23` (`pub use cls_rocketsplash_image::RocketsplashImage`); `blit_render_buffer_to_grid` helper at `mod.rs:24`.
- **Inputs:** A `rocketsplash_rt::RenderBuffer` (per the `mod.rs` rustdoc at `crates/tui-vfx-content/src/sources/mod.rs:9-12` — both `.rss` and `.rsf` formats share a `RenderBuffer` substrate, which is the input to `blit_render_buffer_to_grid`).
- **Outputs:** Cell writes into `tui_vfx_types::Grid`.
- **Options/config:** Source-side parameters (asset bytes, render rect) — full struct shape lives in `cls_rocketsplash_image.rs`.
- **Data touched:** Cells in the target rect.
- **External systems:** `rocketsplash-rt 0.2.2` (`Cargo.toml:64`).
- **Errors and edge cases:** Off-grid blits — clipping logic in `fnc_blit_render_buffer_to_grid.rs` (not enumerated here).
- **Observability:** None of its own.
- **Tests:** No dedicated `test_rocketsplash_image*.rs` was found in `crates/tui-vfx-content/tests/`. The integration is exercised indirectly through scene-layer compositions that may use this source.
- **Evidence:** `crates/tui-vfx-content/src/sources/mod.rs:1-26` (full module), `crates/tui-vfx-content/Cargo.toml:31` (the `rocketsplash-rt.workspace = true` dep).
- **Confidence:** High (re-export + integration helper + dep). Medium pending dedicated-test verification — coverage gap recorded for chapter 10.
- **Unknowns:** Direct test coverage; full parameter struct.

### F018 — RocketsplashFont source (`.rsf` font atlas consumer)

- **Status:** implemented.
- **Description:** A source primitive that consumes a `rocketsplash-rt` `.rsf` font atlas and exposes a `FontRender` for rasterizing glyphs into a grid.
- **User-visible behavior:** Render a custom-font display string (splash titles, multi-cell flap stacks) without depending on system font files.
- **Entry points:** `RocketsplashFont` and `FontRender` re-exported at `crates/tui-vfx-content/src/sources/mod.rs:22` (`pub use cls_rocketsplash_font::{FontRender, RocketsplashFont}`).
- **Inputs:** Font asset bytes (`.rsf`); text string to rasterize; target grid + position.
- **Outputs:** Cell glyph + style writes via the shared `blit_render_buffer_to_grid`.
- **Options/config:** Per the unread `cls_rocketsplash_font.rs` (struct shape).
- **Data touched:** Cells.
- **External systems:** `rocketsplash-rt`.
- **Errors and edge cases:** Same as F017 (off-grid clipping).
- **Observability:** None of its own.
- **Tests:** No dedicated `test_rocketsplash_font*.rs` was found.
- **Evidence:** `crates/tui-vfx-content/src/sources/mod.rs:1-26` (module rustdoc and re-exports), `crates/tui-vfx-content/src/sources/cls_rocketsplash_font.rs` (file presence).
- **Confidence:** High (re-export + integration helper + dep). Medium pending dedicated-test verification.
- **Unknowns:** Same as F017.

### F019 — 3×3 line-glyph default font

- **Status:** implemented.
- **Description:** A canonical heavy-weight 3×3 cell glyph table covering space, digits, A–Z, common punctuation, currency, and a small operator set. Per Intention 36, this is tui-vfx's default font and runtime fallback.
- **User-visible behavior:** Recipes that need typography without declaring a font render through this table; declared fonts that fail to resolve fall back to it.
- **Entry points:** `line_3x3_heavy_glyphs` constant, `lookup_line_3x3_glyph`, `render_line_3x3_text`, `FontGlyphTable` (with `Line3x3` variant), `FontRegistry`, `DEFAULT_FONT_SENTINEL` — all re-exported at `crates/tui-vfx-content/src/fonts/mod.rs:30-33`.
- **Inputs:** Character + weight (per Intention 36's spec); for `FontRegistry`, name → glyph-table registration.
- **Outputs:** A `[&str; 3]` slice of three rows of glyph cells per character.
- **Options/config:** `FontRegistry` insert / resolve; sentinel-routing via `DEFAULT_FONT_SENTINEL`.
- **Data touched:** None (read-only static table).
- **External systems:** None.
- **Errors and edge cases:** Unsupported characters return a fallback glyph (specific behavior per `fnc_lookup_line_3x3_glyph.rs` — not enumerated).
- **Observability:** None of its own.
- **Tests:** Inline `#[cfg(test)]` modules within each `fonts/cls_*.rs` file. Coverage gap: no dedicated `tests/test_fonts*.rs` was found.
- **Evidence:** `crates/tui-vfx-content/src/fonts/mod.rs:1-33` (full module-level rustdoc + re-exports); `steering/INTENTIONS.md:836-878` (Intention 36 declares the contract).
- **Confidence:** High.
- **Unknowns:** Whether the table covers full A–Z + 0–9 + the punctuation set Intention 36 claims — the file is `col_line_3x3_heavy_glyphs.rs` (a leaf-helper file by OFPF prefix), and the per-character coverage is not enumerated here.

### F020 — Asset registry (name → bytes mapping)

- **Status:** partially implemented (per `crates/tui-vfx-content/src/assets/mod.rs:3` `<WCTX>` — Phase 7 *breadcrumb*; the consumer surface that loads assets by name into a scene-layer source variant is **deferred**).
- **Description:** A name → bytes registry with a registered default and a reserved sentinel literal `default_logo` that routes to the current default. Mirrors `FontRegistry`'s shape so authoring stays consistent across binding kinds.
- **User-visible behavior:** Recipes can name an asset; the host's `AssetRegistry` resolves it to bytes at render time.
- **Entry points:** `AssetRegistry`, `DEFAULT_LOGO_SENTINEL` re-exported at `crates/tui-vfx-content/src/assets/mod.rs:21` (`pub use cls_asset_registry::{AssetRegistry, DEFAULT_LOGO_SENTINEL}`).
- **Inputs:** Asset name (string); register-default + register-by-name calls.
- **Outputs:** Bytes (Vec<u8> or similar).
- **Options/config:** None of its own.
- **Data touched:** Internal name → bytes map.
- **External systems:** None.
- **Errors and edge cases:** A name that does not resolve and is not the sentinel — exact behavior per `cls_asset_registry.rs`.
- **Observability:** None of its own.
- **Tests:** No dedicated `tests/test_assets*.rs` found.
- **Evidence:** `crates/tui-vfx-content/src/assets/mod.rs:1-25` (full module rustdoc + re-exports), `crates/tui-vfx-content/src/assets/cls_asset_registry.rs` (file presence).
- **Confidence:** High on the registry surface; Medium overall because the consumer side (a scene-layer source variant that loads assets by name) is documented as deferred (`mod.rs:14-17`).
- **Unknowns:** When the consumer source variant lands; whether asset bytes are owned (`Vec<u8>`) or borrowed (`&[u8]`).

### F021 — Pool primitives (TextPool / EffectPool / ImagePool / FontPool / PresetPool)

- **Status:** implemented.
- **Description:** A canonical generic `Pool<T>` plus four type aliases (`EffectPool`, `ImagePool`, `FontPool`, `PresetPool`) plus a `TextPool` newtype with sanitize-on-construct. `Preset` is a curated bundle (text + effect + image + font).
- **User-visible behavior:** Random rotation through a list of items per a `PoolPolicy` (e.g., uniform / weighted / cycle).
- **Entry points:** `Pool<T>`, `Preset`, `TextPool`, `EffectPool`, `ImagePool`, `FontPool`, `PresetPool`, `PoolPolicy` (alias-and-newtype layout per the `pool/mod.rs` rustdoc at `crates/tui-vfx-content/src/pool/mod.rs:11-39`).
- **Inputs:** `Pool::new(items, policy)`; `Pool::pick`; `Pool::is_empty`.
- **Outputs:** Picked item per policy.
- **Options/config:** `PoolPolicy` (the file is `col_pool_policy.rs` — a leaf-helper).
- **Data touched:** Pool's internal items list.
- **External systems:** None.
- **Errors and edge cases:** Empty pool — `is_empty` returns true; `pick` behavior on empty pool is per the implementation (not enumerated here).
- **Observability:** None of its own.
- **Tests:** No dedicated `tests/test_pool*.rs` found; in-source `#[cfg(test)]` modules likely.
- **Evidence:** `crates/tui-vfx-content/src/pool/mod.rs:1-50` (the full module rustdoc with the family enumeration), `cls_pool.rs`, `cls_preset.rs`, `cls_text_pool.rs`, `col_pool_policy.rs`, `fnc_pick_index.rs` (the leaf-helper) — all file-present.
- **Confidence:** High (rustdoc + per-file structure + alias layout).
- **Unknowns:** Specific `PoolPolicy` variants (e.g., uniform / weighted / cycle).

### F022 — Easing functions library

- **Status:** implemented (re-export from `mixed-signals`).
- **Description:** Easing curves used by motion paths and timed transitions. As of geometry crate v2.0.0, `easing` is a re-export from `mixed-signals::easing` (per `crates/tui-vfx-geometry/src/easing/mod.rs:4`).
- **User-visible behavior:** Smooth interpolation curves (`EaseInOut`, `BackOut`, `Elastic`, etc.) applied to time-axis values.
- **Entry points:** `EasingType` and `ease` re-exported at `crates/tui-vfx-geometry/src/easing/mod.rs:7` (`pub use mixed_signals::easing::{EasingType, ease}`).
- **Inputs:** `t: f32` (or `f64`) progress + `EasingType` discriminant.
- **Outputs:** Eased `t`.
- **Options/config:** `EasingType` enum variants — definitive list lives in `mixed-signals` (out of workspace scope).
- **Data touched:** A scalar.
- **External systems:** None (pure math).
- **Errors and edge cases:** Out-of-range `t` — handled per `mixed-signals` (not catalogued here).
- **Observability:** None of its own.
- **Tests:** `crates/tui-vfx-geometry/benches/easing.rs` (the criterion bench at `Cargo.toml:35-37` exercises the easing surface).
- **Evidence:** `crates/tui-vfx-geometry/src/easing/mod.rs:1-7` (full module — 7 lines), `crates/tui-vfx-geometry/Cargo.toml:35-37` (the bench).
- **Confidence:** High.
- **Unknowns:** Full `EasingType` variant list (lives in `mixed-signals`).

### F023 — Motion-path library (9 path shapes)

- **Status:** implemented.
- **Description:** Nine path classes (Arc, Bezier, Hover, Linear, Rectilinear, Spiral, Spring, Squash, Step) plus the `MotionPath` trait they implement.
- **User-visible behavior:** Content travels along the chosen path over the duration of a transformer or transition.
- **Entry points:** `ArcPath`, `BezierPath`, `HoverPath`, `LinearPath`, `RectilinearPath`, `SpiralPath`, `SpringPath`, `SquashPath`, `StepPath` re-exported at `crates/tui-vfx-geometry/src/paths/mod.rs:18-26`; `MotionPath` trait re-exported at `crates/tui-vfx-geometry/src/lib.rs:18`.
- **Inputs:** Per-path constructor params (control points for `BezierPath`, radii for `ArcPath` / `SpiralPath`, damping for `SpringPath`, etc.); time / progress.
- **Outputs:** A `Position` (or `Point`) on the path at the requested progress.
- **Options/config:** Per-path parameter struct.
- **Data touched:** None — pure functions.
- **External systems:** None.
- **Errors and edge cases:** Out-of-range progress per-path; not enumerated here.
- **Observability:** None of its own.
- **Tests:** 15 `tests/*.rs` files in `tui-vfx-geometry/tests/` (chapter 2 §2.2). Per-path unit coverage exists; full enumeration deferred to chapter 10.
- **Evidence:** `crates/tui-vfx-geometry/src/paths/mod.rs:1-26` (full module — 26 lines, 9 `pub mod` + 9 `pub use`), `crates/tui-vfx-geometry/src/traits/` (the `MotionPath` trait file lives here).
- **Confidence:** High.
- **Unknowns:** None of operational note for the 9-shape taxonomy.

### F024 — Wipe geometry helpers (shared between mask + shader)

- **Status:** implemented.
- **Description:** Shared `WipeDirection` enum + `wipe_progress` / `wipe_visible_at` helpers used by the `Wipe` mask in compositor and the `RevealWipe` shader in style. Single source of truth so the direction vocabulary doesn't drift between the two consumers.
- **User-visible behavior:** Consistent wipe direction set (including corner-out / corner-in directions per the `wipe/mod.rs:5-8` `<WCTX>` block) at both the mask and shader layers.
- **Entry points:** `WipeDirection` re-exported at `crates/tui-vfx-geometry/src/wipe/mod.rs:25-27` (alias re-export from `types::cls_wipe_direction`); `wipe_progress` and `wipe_visible_at` at `:23`. Crate-root re-exports at `crates/tui-vfx-geometry/src/lib.rs:23`.
- **Inputs:** `WipeDirection` discriminant; per-cell coordinates; progress fraction.
- **Outputs:** Per-cell visibility decision.
- **Options/config:** `WipeDirection` variants.
- **Data touched:** None — pure math.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** Inherits F002 / F004 (mask stage) and F002 / F008 (shader stage).
- **Tests:** Per-helper tests live in `tui-vfx-geometry/tests/`; the mask + shader integration tests live in their respective crates' tests.
- **Evidence:** `crates/tui-vfx-geometry/src/wipe/mod.rs:1-27` (full module rustdoc), `crates/tui-vfx-geometry/src/lib.rs:23`, `crates/tui-vfx-compositor/Cargo.toml:32` (the runtime dep added specifically for "WipeDirection re-export and the shared wipe geometry helpers").
- **Confidence:** High.
- **Unknowns:** Full `WipeDirection` variant list.

### F025 — Geometry primitives (anchors, borders, layout, transitions, widgets)

- **Status:** implemented.
- **Description:** A grouping of supporting geometry primitives consumed across the workspace.
- **User-visible behavior:** Anchored placement (e.g., notifications anchored to corners with a 45 % visual center per `crates/tui-vfx-geometry/src/anchors/mod.rs:11-13`); border-trim specs (vanishing edges, clipped edges); layout helpers (grid snapping); transition rect-path computations (slide / expand-collapse); widget hit-testing (numpad 3×3, triplet grids, direction-selection motion); shared `Anchor`, `Origin`, `PathType`, `Position`, `PositionSpec`, `RectScaleSpec`, `SignedRect`, `SlideDirection`, `SnappingStrategy`, `TransitionSpec` types.
- **Entry points:** `crates/tui-vfx-geometry/src/lib.rs:7-15` (the public sub-modules) and `:18-23` (the `pub use traits::MotionPath`, `pub use types::{...}`, `pub use wipe::{...}` re-exports).
- **Inputs:** Per-primitive (rects, points, anchors, slide directions).
- **Outputs:** Per-primitive (rects, points, paths, hit-test results).
- **Options/config:** Per-primitive.
- **Data touched:** None — pure math.
- **External systems:** None.
- **Errors and edge cases:** Per-primitive (e.g., `calculate_anchor_position` at `crates/tui-vfx-geometry/src/anchors/mod.rs:24` documents its right/bottom-anchor return-the-last-visible-cell behavior).
- **Observability:** None of its own.
- **Tests:** 15 `tests/*.rs` files in `tui-vfx-geometry/tests/` cover this surface (chapter 2 §2.2).
- **Evidence:** `crates/tui-vfx-geometry/src/lib.rs:7-23`, `anchors/mod.rs`, `borders/`, `layout/`, `transitions/`, `widgets/` (each with `mod.rs` + leaf files listed earlier).
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F026 — `Grid` trait + `BoundaryMode` + `OwnedGrid`

- **Status:** implemented.
- **Description:** The cell-buffer interface that the entire workspace consumes. `Grid` is the read trait; `OwnedGrid` is a simple owned implementation; `GridExt` provides extension methods; `BoundaryMode` controls how off-grid reads behave.
- **User-visible behavior:** Any consumer can implement `Grid` for its own buffer type and route the entire pipeline through it.
- **Entry points:** `Grid`, `BoundaryMode`, `GridExt`, `OwnedGrid` re-exported at `crates/tui-vfx-types/src/lib.rs:97` (`pub use grid::{BoundaryMode, Grid, GridExt, OwnedGrid}`).
- **Inputs:** Cell coordinates `(x, y)`; `BoundaryMode` for out-of-bounds policy.
- **Outputs:** `Cell` values (read) or stored writes (`OwnedGrid`).
- **Options/config:** `BoundaryMode`.
- **Data touched:** Cell buffer.
- **External systems:** None.
- **Errors and edge cases:** Out-of-bounds reads — handled per `BoundaryMode`.
- **Observability:** None of its own.
- **Tests:** Inline `#[cfg(test)]` modules in `crates/tui-vfx-types/src/grid.rs` (the file is private — `mod grid;`). Per chapter 2 §2.2, `tui-vfx-types` has 9 `tests/*.rs` files; `tests/test_owned_grid*.rs` is not present in the listing, suggesting `Grid` tests live in-source.
- **Evidence:** `crates/tui-vfx-types/src/lib.rs:78,97` (sub-module declaration `mod grid;` and `pub use` at `:97`), `crates/tui-vfx-types/src/lib.rs:55-58` (the rustdoc enumerating the trait).
- **Confidence:** High.
- **Unknowns:** Full `BoundaryMode` variant list.

### F027 — `SemanticScene` + `RoleMap` + `RoleTag` (role-aware destination)

- **Status:** implemented.
- **Description:** A source surface (grid + per-cell role tags) that is equally produced by widget renders and by recipe-driven scene composers. The pipeline reads roles from the source and writes role-tagged output cells to the destination.
- **User-visible behavior:** Effects can target cells by semantic role (`Role("primary")`, `Role("surface")`) rather than raw colors; theme switches re-skin without per-recipe rewrites (per Intention 19).
- **Entry points:** `SemanticScene`, `RoleMap`, `RoleMapIter`, `RoleTag`, `InternedRoleName`, `RoleId`, `RoleInterner` re-exported at `crates/tui-vfx-types/src/lib.rs:103-108`. `SceneMetadata` at `:107`.
- **Inputs:** Cell coordinates + a `RoleTag`. `RoleTag` has 12 first-class variants (`Background`, `Text`, `Title`, `Caption`, `Border`, `Image`, `Icon`, `Indicator`, `Highlight`, `Shadow`, `Decoration`, `Procedural`) plus `Custom(InternedRoleName)` per the lib.rs rustdoc at `crates/tui-vfx-types/src/lib.rs:24-28`.
- **Outputs:** Per-cell role lookups.
- **Options/config:** `RoleInterner` allocates `RoleId` numbers (first-class 0–11, `Custom` starts at 12).
- **Data touched:** Per-cell role storage.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** Trace events carry `RoleTag` references via the inspection foundation (F036).
- **Tests:** `crates/tui-vfx-types/tests/test_role_id.rs`, `test_role_interner.rs`, `test_role_map.rs`, `test_role_tag.rs`, `test_scene_metadata.rs`, `test_semantic_scene.rs`. Test peer coverage 100 % (chapter 14).
- **Evidence:** `crates/tui-vfx-types/src/lib.rs:14-32` (the role-tagging rustdoc block); `:103-108` (the re-exports); `crates/tui-vfx-types/tests/test_*.rs` listing.
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F028 — Glyph rendering framework

- **Status:** implemented (chapter 12 §12.7 resolved the apparent partial-implementation: the "Slice 6.6 §F.1" reference belongs to a separate font-binding plan, not to this framework. The framework is complete and the consumer chain is live).
- **Description:** Sub-cell sampling + glyph encoding bound by `VfxCellContext` (a seven-field `Copy` bundle shared by Filter / Mask / Sampler / StyleShader stages). Five-variant closed encoder vocabulary plus the eight-subcell sampler.
- **User-visible behavior:** Higher-density per-cell rendering (eight subcells per cell) for filters and shaders that need sub-cell precision. Live consumers: `cls_water_field_signal.rs` and `cls_fire_field_signal.rs` in `tui-vfx-style/models/`, feeding `ScalarFieldGlyphFilter` (`crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs:73`) which the pipeline routes as `ScalarFieldGlyphWater(ScalarFieldGlyphFilter<WaterFieldSignal>)` and `ScalarFieldGlyphFire(ScalarFieldGlyphFilter<FireFieldSignal>)` per `pipeline/cls_prepared_filter.rs:87-89`.
- **Entry points:** `VfxCellContext` at `crates/tui-vfx-types/src/lib.rs:94` (`pub use cls_vfx_cell_context::VfxCellContext`); `glyph` module at `:78` (`pub mod glyph`); within `glyph/`: `cls_glyph_encoder.rs:48-79` (`GlyphEncoder` 5-variant enum with `BrailleSubcell`, `BrailleEighths`, `BlockHorizontal`, `BlockVertical`, `Ramp` plus methods `encode_one` at `:103-119` and `encode_subcell` at `:144-170`); `fnc_sample_eight_subcells.rs:35-44` (`SUBCELL_OFFSETS` table) + `:73-84` (`sample_eight_subcells`) + `:126-137` (`sample_eight_subcells_with_slope`).
- **Inputs:** A cell context (screen coords + normalized coords); a glyph encoding query.
- **Outputs:** Glyph rows / encoded subcell pattern.
- **Options/config:** Glyph encoding parameters; per-stage VfxCellContext fields (`screen_cell_x/y`, `normalized_x/y`).
- **Data touched:** Per-cell glyph + style.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** Inherits F002.
- **Tests:** `crates/tui-vfx-types/src/glyph/test_cls_glyph_encoder.rs`, `test_fnc_sample_eight_subcells.rs` (in-source).
- **Evidence:** `crates/tui-vfx-types/Cargo.toml:3-4` (`<WCTX>` + `<CLOG>`); `crates/tui-vfx-types/src/lib.rs:78, :94`; `crates/tui-vfx-types/src/glyph/mod.rs:23-29` (re-exports `GlyphEncoder`, `sample_eight_subcells`, `sample_eight_subcells_with_slope`, `SUBCELL_OFFSETS`); `crates/tui-vfx-types/src/glyph/{cls_glyph_encoder.rs, fnc_sample_eight_subcells.rs, mod.rs}` plus paired tests `test_cls_glyph_encoder.rs` and `test_fnc_sample_eight_subcells.rs`. Live consumers: `crates/tui-vfx-style/src/models/cls_water_field_signal.rs`, `cls_fire_field_signal.rs`; `crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs:73`; `crates/tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs:87-89`.
- **Confidence:** High.
- **Unknowns:** None.

### F029 — Braille primitives (subcell dot encoding)

- **Status:** implemented.
- **Description:** Utilities for Unicode braille patterns in U+2800–U+28FF, encoding a 2×4 dot grid per cell.
- **User-visible behavior:** Braille glyphs as image-density renderings (the supersampling story).
- **Entry points:** `braille` module at `crates/tui-vfx-types/src/lib.rs:72` (`pub mod braille`).
- **Inputs:** Bit pattern (8 bits, dots 1-8 per the rustdoc layout at `crates/tui-vfx-types/src/braille.rs:7-15`).
- **Outputs:** Braille char (`U+2800 + bits`), or region masks (combine with `&` per the comment block at `:25-26`).
- **Options/config:** Region masks (the comment indicates these exist as named constants in the file).
- **Data touched:** Single chars.
- **External systems:** None.
- **Errors and edge cases:** Bounds — per the file's `<WCTX>`/`<CLOG>` block at `:1-4` documenting an unsafe-conversion safety invariant.
- **Observability:** None.
- **Tests:** Inline `#[cfg(test)]` in `braille.rs`. No dedicated `tests/test_braille*.rs` was found.
- **Evidence:** `crates/tui-vfx-types/src/braille.rs:1-26` (file head with full rustdoc), `crates/tui-vfx-types/src/lib.rs:72`.
- **Confidence:** High.
- **Unknowns:** Full set of named region-mask constants (the comment at `:25-26` references "region masks" but the constants themselves are below the truncation point in this read).

### F030 — RigidShake timing primitive

- **Status:** implemented.
- **Description:** Shared timing state for the `RigidShake` filter and any style-side shake effects, so a single state object drives both the per-cell transform and the cell-level accent.
- **User-visible behavior:** Coordinated shake (jitter) animation across filter and style stages without two parallel timers.
- **Entry points:** `RigidShakeState`, `RigidShakeTiming` re-exported at `crates/tui-vfx-types/src/lib.rs:102` (`pub use rigid_shake_timing::{RigidShakeState, RigidShakeTiming}`); module declared `pub mod rigid_shake_timing;` at `:84`.
- **Inputs:** Time / progress.
- **Outputs:** Per-frame jitter offsets / phase.
- **Options/config:** Per the unread `rigid_shake_timing/` module structure (it is a `pub mod` so the file is `crates/tui-vfx-types/src/rigid_shake_timing.rs` or directory).
- **Data touched:** Internal timing state.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** Inherits F002 when consumed inside the compositor.
- **Tests:** Inline `#[cfg(test)]` modules likely; no dedicated `tests/test_rigid_shake*.rs` was found in the `tui-vfx-types/tests/` listing.
- **Evidence:** `crates/tui-vfx-types/src/lib.rs:84,102`, `crates/tui-vfx-types/src/lib.rs:64-65` (rustdoc: "Shared timing for RigidShake filter and style effects").
- **Confidence:** High.
- **Unknowns:** Full `RigidShakeTiming` constructor surface.

### F031 — Opaque ID types (`LayerId`, `RecipeId`, `RoleId`, `InternedString`)

- **Status:** implemented.
- **Description:** Cheap-to-clone newtype ID types backed by `InternedString` (an `Arc<str>` wrapper). Consumed by trace selectors / inspection sinks without forcing downstream inspection code to depend on the recipe crate (per `crates/tui-vfx-types/src/lib.rs:31-32`).
- **User-visible behavior:** Per-layer / per-recipe / per-role identification across the inspection surface.
- **Entry points:** `LayerId`, `RecipeId`, `RoleId`, `InternedString` re-exported at `crates/tui-vfx-types/src/lib.rs:98-103` (`pub use interned_string::InternedString` etc.).
- **Inputs:** Construction APIs (per `interned_string.rs`, `layer_id.rs`, `recipe_id.rs`, `role_id.rs`).
- **Outputs:** Opaque ID values.
- **Options/config:** None of their own.
- **Data touched:** None — they are bare-id types.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** They appear *in* trace event payloads (via F036).
- **Tests:** `crates/tui-vfx-types/tests/test_interned_string.rs`, `test_layer_id.rs`, `test_recipe_id.rs`, `test_role_id.rs`. Test peer coverage 100 %.
- **Evidence:** `crates/tui-vfx-types/src/lib.rs:98-103`, four dedicated `tests/*.rs` files.
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F032 — `ConfigSchema` derive macro (proc-macro)

- **Status:** implemented.
- **Description:** A proc-macro derive that emits a `ConfigSchema` impl for a struct or enum, exposing its serde shape + field metadata to the schema/doc-generation pipeline. Supporting types include `FieldMeta`, `Range`, `ScalarValue`, `SchemaField`, `SchemaNode`, `SchemaVariant`.
- **User-visible behavior:** A type tagged `#[derive(ConfigSchema)]` exposes a programmatic schema usable by the `xtask` doc generator (F044) and the `tui-vfx-core` JSON-writer family (`fnc_to_json_schema.rs`, `fnc_node_to_json_schema.rs`, `fnc_schema_node_to_json_pretty.rs`).
- **Entry points:** `derive_config_schema` at `crates/tui-vfx-core-macros/src/lib.rs:28` (the only `pub fn` in the proc-macro crate); `ConfigSchema` re-export from `tui-vfx-core` at `crates/tui-vfx-core/src/lib.rs:23` (`pub use tui_vfx_core_macros::ConfigSchema`); supporting types at `crates/tui-vfx-core/src/lib.rs:16-18`.
- **Inputs:** Rust source token stream (compile-time).
- **Outputs:** `impl ConfigSchema for X` emitted at compile time.
- **Options/config:** Per-field attribute syntax (handled inside `derive_config_schema`).
- **Data touched:** Compile-time AST.
- **External systems:** `proc-macro2 1`, `quote 1`, `syn 2 features=["full","extra-traits"]` (`crates/tui-vfx-core-macros/Cargo.toml:21-24`).
- **Errors and edge cases:** Macro emit errors at compile time.
- **Observability:** None (compile-time).
- **Tests:** `tui-vfx-core-macros` has no `tests/` directory at audit-time (chapter 2 §2.2). The derive is exercised through every consumer that uses `#[derive(ConfigSchema)]` (and through `crates/tui-vfx-core/src/bindable/test_cls_bindable.rs` for the `VfxBindable` schema). `xtask audit configschema` (F043) audits hand-written impls.
- **Evidence:** `crates/tui-vfx-core-macros/src/lib.rs:28`; `crates/tui-vfx-core/src/lib.rs:16-18,23`; `crates/tui-vfx-core/src/schema/` (14 supporting files: `cls_json_writer.rs`, `cls_schema_registry.rs`, the `fnc_json_write_*` family).
- **Confidence:** High.
- **Unknowns:** Full per-attribute syntax — needs reading `crates/tui-vfx-core-macros/src/lib.rs` end-to-end.

### F033 — `mixed-signals` schema bridge

- **Status:** implemented.
- **Description:** A schema bridge that exposes engine-native `mixed-signals` signal types to the `ConfigSchema` machinery so generated docs can document signal-bearing fields uniformly.
- **User-visible behavior:** Signal-bearing schema fields appear in generated capability docs without each consumer rewriting their own bridge.
- **Entry points:** `crates/tui-vfx-core/src/mixed_signals_schema.rs` (single-file module; declared at `crates/tui-vfx-core/src/lib.rs:7`).
- **Inputs:** `mixed_signals` types (`SignalSpec`, `SignalOrFloat`, etc.).
- **Outputs:** `ConfigSchema`-compatible `SchemaNode` / `SchemaField` structures.
- **Options/config:** None of its own.
- **Data touched:** Schema metadata only.
- **External systems:** `mixed-signals` (workspace dep at `Cargo.toml:51`).
- **Errors and edge cases:** None enumerated.
- **Observability:** None.
- **Tests:** No dedicated `tests/test_mixed_signals_schema*.rs` was found in `crates/tui-vfx-core/`; coverage is indirect through the `xtask docs` generator's signals subcommands (F044).
- **Evidence:** `crates/tui-vfx-core/src/lib.rs:7` (the `pub mod` declaration), `crates/tui-vfx-core/src/mixed_signals_schema.rs` (the file).
- **Confidence:** High (presence + integration via docs subcommand). Medium pending dedicated-test verification.
- **Unknowns:** Public-API surface of the file — `pub fn` / `pub struct` enumeration was not produced because the file's `pub` symbols did not surface in the small grep window.

### F034 — `TimeSpec` time-axis primitive

- **Status:** implemented.
- **Description:** A small struct representing a time interval with current position, used by the schema and probe surfaces. Carries `start: Instant`, `now: Instant`, `duration: Duration` and a `progress() -> f64` helper.
- **User-visible behavior:** Consumers compute progress through a phase from a `TimeSpec` rather than maintaining their own clock arithmetic.
- **Entry points:** `TimeSpec` re-exported at `crates/tui-vfx-core/src/lib.rs:20` (`pub use time_spec::TimeSpec`); the type is defined at `crates/tui-vfx-core/src/time_spec.rs:13` (`pub struct TimeSpec { pub start: Instant, pub now: Instant, pub duration: Duration }`); `progress` method at `:22`.
- **Inputs:** `Instant` start + now + `Duration` total.
- **Outputs:** `progress() -> f64` (a 0.0..=1.0 fraction).
- **Options/config:** None.
- **Data touched:** Internal time fields only.
- **External systems:** None — uses `std::time::Instant` and `std::time::Duration`.
- **Errors and edge cases:** Behavior when `duration` is zero or `now < start` is not enumerated here.
- **Observability:** None.
- **Tests:** No dedicated `tests/test_time_spec*.rs` in the `tui-vfx-core/tests/` listing.
- **Evidence:** `crates/tui-vfx-core/src/time_spec.rs:13-22`, `crates/tui-vfx-core/src/lib.rs:9,20`.
- **Confidence:** High.
- **Unknowns:** Edge-case division-by-zero behavior in `progress()`.

### F035 — Centralized debug logger

- **Status:** implemented.
- **Description:** A module-scoped logging factory with a shared global singleton. Configurable per-module log levels for granular debug-session control.
- **User-visible behavior:** Consumers in any tui-vfx crate can route debug log calls to a single configurable logger; per-module verbosity can be raised to DEBUG/TRACE without touching unrelated modules.
- **Entry points:** `DebugLogger`, `LogEntry`, `Logger`, `create_logger`, `get_global_logger` re-exported at `crates/tui-vfx-debug/src/lib.rs:38` (`pub use logger::{DebugLogger, LogEntry, Logger, create_logger, get_global_logger}`); `LogLevel`, `ModuleConfig` at `:37` (`pub use config::{LogLevel, ModuleConfig}`).
- **Inputs:** `LogLevel` per module, log call sites.
- **Outputs:** Console output (or whatever the configured logger writes); `LogEntry` records.
- **Options/config:** `ModuleConfig` per module.
- **Data touched:** Logger's internal records.
- **External systems:** `colored 2.1`, `chrono 0.4`, `lazy_static 1.4` (`crates/tui-vfx-debug/Cargo.toml:20-22`).
- **Errors and edge cases:** None enumerated.
- **Observability:** This *is* the logging surface.
- **Tests:** 8 `tests/*.rs` files in `tui-vfx-debug/tests/` (chapter 2 §2.2). Test peer coverage 90 %.
- **Evidence:** `crates/tui-vfx-debug/src/lib.rs:32-38` (the `mod config; pub mod inspection; mod logger;` block plus the re-exports); `crates/tui-vfx-debug/src/config.rs`, `crates/tui-vfx-debug/src/logger.rs`.
- **Confidence:** High.
- **Unknowns:** Full `LogLevel` variant list (TRACE / DEBUG / INFO / WARN / ERROR likely; not enumerated).

### F036 — Inspection foundation (TraceEvent + sinks + filters)

- **Status:** implemented (additive surface introduced in v1.1.0 per `crates/tui-vfx-debug/src/lib.rs:5`).
- **Description:** A canonical `TraceEvent` taxonomy plus `TraceEnvelope` (frame / time / recipe / seq-in-frame carrier), `TraceFilter` + `TraceSelector` (declarative sink-time filtering), `StageMask` (per-stage gating), `InspectionSink` trait, concrete thread-safe `TraceSink`, `TraceReport` (NDJSON round-trip output). Helper types include `PipelineStageKind`, `PipelineSkipReason`, `RoleHistogram`, `RoleMapSource`, plus a test-friendly `AssertingInspector`.
- **User-visible behavior:** Pipeline stages emit structured events. Sinks subscribe with filter declarations; consumers (CLI tooling, AI-context generators) read NDJSON reports.
- **Entry points:** `crates/tui-vfx-debug/src/inspection/mod.rs` is the public hub (declared `pub mod inspection;` at `crates/tui-vfx-debug/src/lib.rs:33`). Sub-modules: `cls_asserting_inspector.rs`, `cls_inspection_sink.rs`, `cls_pipeline_skip_reason.rs`, `cls_pipeline_stage_kind.rs`, `cls_role_histogram.rs`, `cls_role_map_source.rs`, `cls_stage_mask.rs`, `cls_trace_emitter.rs`, `cls_trace_envelope.rs`, `cls_trace_event.rs`, `cls_trace_filter.rs`, `cls_trace_report.rs`, `cls_trace_selector.rs`, `cls_trace_sink.rs` (14 files).
- **Inputs:** `TraceEvent` payloads from emit sites; `TraceFilter` + `TraceSelector` from consumers.
- **Outputs:** Sink-side records; NDJSON reports.
- **Options/config:** `StageMask`, `TraceSelector` set, frame range, time range (per the rustdoc `mod.rs:12-19`).
- **Data touched:** Sink storage.
- **External systems:** None inherent — sinks may write to files.
- **Errors and edge cases:** Sink boundedness — optional per the rustdoc at `mod.rs:18`.
- **Observability:** This *is* the observability surface.
- **Tests:** `crates/tui-vfx-debug/tests/` (8 files); `crates/tui-vfx-compositor/tests/test_inspection_sink_bridge*.rs`. Two criterion benches cover emit-path performance.
- **Evidence:** `crates/tui-vfx-debug/src/lib.rs:5-30` (the rustdoc enumerates the surface), `crates/tui-vfx-debug/src/inspection/mod.rs:1-30` (the module-level rustdoc with the four-stage taxonomy), `crates/tui-vfx-debug/src/inspection/` (14 `cls_*.rs` files).
- **Confidence:** High.
- **Unknowns:** Full `TraceEvent` variant list (the file is `cls_trace_event.rs`; per the inspection `mod.rs:5` `<CLOG>`, version 0.2.0 added new variants — full enumeration deferred).

### F037 — `pipeline-probe` binary

- **Status:** implemented.
- **Description:** A CLI wrapper around `tui-vfx-probe` that accepts a direct `ProbeSceneSpec` JSON document and emits a single frame dump, a timeline (multiple frames sampled across one phase), or a frame diff.
- **User-visible behavior:** Per the bin's own rustdoc at `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:7-12`: takes a `ProbeSceneSpec` JSON, emits frame dump (default), timeline (`--frames N`), or diff (`--diff-to T`).
- **Entry points:** Binary `pipeline-probe` (registered at `crates/tui-vfx-probe/Cargo.toml:29-31`); `main` at `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:23`.
- **Inputs:** Command-line flags `--input <path>`, `--format <fmt>`, `--phase entering|dwelling|exiting` (default `dwelling`), `--sample-t <f64>` (default 0.5), `--cells all|non-empty|modified` (default `all`), `--with-causation`, `--frames <usize>`, `--diff-to <f64>`, `--widget-cell` (per the recent `<CLOG>` at `:5`); a `ProbeSceneSpec` JSON file at `--input`.
- **Outputs:** A frame dump, timeline, or diff to stdout in the requested format. Optional SQLite persistence via `ProbeSqliteStore`.
- **Options/config:** Same as Inputs above. The `--phase` discriminant maps to `ProbePhase::{Entering, Dwelling, Exiting}`.
- **Data touched:** Reads input JSON; optionally writes to a SQLite store; writes to stdout.
- **External systems:** Optional SQLite via `ProbeSqliteStore` (F039).
- **Errors and edge cases:** Per `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:25-29`: returns non-zero exit on any error and prints the error to stderr. Specific argument-parsing error strings live inline (e.g., `"missing value for --format"`, `"unsupported phase: <x>"`).
- **Observability:** This *is* the introspection binary.
- **Tests:** Integration tests in `tui-vfx-probe/tests/` (6 files per chapter 2 §2.2). Test peer coverage 4.8 % (the lowest in the workspace; recorded in chapter 14).
- **Evidence:** `crates/tui-vfx-probe/Cargo.toml:29-31`, `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:1-80` (file head with the full rustdoc + CLI flag parsing).
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F038 — Probe DTO catalogue

- **Status:** implemented.
- **Description:** 24+ `cls_probe_*` DTOs that carry per-frame structured introspection data. These are the wire-format types the probe binary serializes.
- **User-visible behavior:** Probe consumers (CLI scripts, AI integrations) parse a stable JSON shape per DTO.
- **Entry points:** Re-exported at `crates/tui-vfx-probe/src/lib.rs:75-99`. The full list: `ProbeCell`, `ProbeCellRootCause`/`ProbeCellStageCause`, `ProbeColor`, `ProbeDiagnostic`/`ProbeDiagnosticSeverity`, `ProbeDiffCell`, `ProbeDiffReport`, `ProbeError`, `ProbeGridSpec`, `ProbeLastTouch`, `ProbeOperationalAnalysis` (and siblings), `ProbePipelineInventory`, `ProbeFrame`/`ProbePoint`/`ProbeReport`/`ProbeReportSource`/`ProbeSize`, `ProbeCellSelector`/`ProbePhase`/`ProbeRequest`, `ProbeRuntimeContext`/`ProbeRuntimeParam`, `ProbeSceneSpec`, `ProbeSqliteStore`, `ProbeStateSnapshot`, `ProbeSummary`, `ProbeTimelineReport`, `ProbeTiming`, `ProbeTraceEvent`, `ProbeWidget`.
- **Inputs:** Engine state at probe time.
- **Outputs:** JSON-serializable structures.
- **Options/config:** None of their own.
- **Data touched:** Per-DTO (cell color, position, role, time, etc.).
- **External systems:** None.
- **Errors and edge cases:** Per-DTO.
- **Observability:** These *are* the observability DTOs.
- **Tests:** Integration tests in `tui-vfx-probe/tests/` (6 files); coverage gap-flagged for chapter 10.
- **Evidence:** `crates/tui-vfx-probe/src/lib.rs:75-99` (the run of `pub use cls_probe_*::...` re-exports — 24 entries).
- **Confidence:** High.
- **Unknowns:** Per-DTO field surface — only the type names are catalogued here.

### F039 — Probe SQLite store

- **Status:** implemented.
- **Description:** Persistence of probe runs to a SQLite database via `rusqlite` with the `bundled` feature so no external SQLite library is required.
- **User-visible behavior:** Probe runs can be archived and queried via SQL.
- **Entry points:** `ProbeSqliteStore` at `crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs`; re-exported at `crates/tui-vfx-probe/src/lib.rs:93`. Wired into the binary via `--sqlite-query` (per the binary's argument parsing at `:74` — the variable is initialized but the full handler was not read in this batch).
- **Inputs:** Probe DTOs to write; SQL queries to read.
- **Outputs:** SQLite database file; query result rows.
- **Options/config:** Database path; query string.
- **Data touched:** SQLite database file.
- **External systems:** `rusqlite 0.32 features=["bundled"]` (`crates/tui-vfx-probe/Cargo.toml:25`).
- **Errors and edge cases:** SQLite I/O errors propagate via `Box<dyn Error>` from `main`.
- **Observability:** Inherits F035 (debug logger).
- **Tests:** Coverage in `tui-vfx-probe/tests/`; not enumerated here.
- **Evidence:** `crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs`, `crates/tui-vfx-probe/Cargo.toml:25`, `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:74` (`sqlite_query` argument variable).
- **Confidence:** High.
- **Unknowns:** Full SQL schema written by the store.

### F040 — Probe diagnostics + root-cause inference

- **Status:** implemented.
- **Description:** A family of diagnostic-collection helpers (basic, loopback-fire, operational analysis) plus a per-cell root-cause inference function that traces a cell back to the pipeline stage that produced it.
- **User-visible behavior:** A probe consumer can ask "why is this cell rendering this glyph/color?" and get a structured answer.
- **Entry points:** `collect_basic_diagnostics`, `collect_loopback_fire_diagnostics`, `collect_probe_operational_analysis`, `build_probe_cell_root_cause`, `infer_roles_from_grid`, `diff_frames`, `find_widget_cell`, `has_ascii_alpha` re-exported at `crates/tui-vfx-probe/src/lib.rs:100-107`.
- **Inputs:** Probe state, cell coordinates, role grids.
- **Outputs:** `ProbeDiagnostic` lists, `ProbeCellRootCause`, role inferences, diff cells.
- **Options/config:** Per-helper.
- **Data touched:** Read-only against probe state; produces structured outputs.
- **External systems:** None.
- **Errors and edge cases:** Per-helper.
- **Observability:** None of their own.
- **Tests:** `tui-vfx-probe/tests/` (6 files).
- **Evidence:** `crates/tui-vfx-probe/src/lib.rs:100-107`.
- **Confidence:** High.
- **Unknowns:** Specific diagnostic categories; full `ProbeDiagnosticSeverity` variant list.

### F041 — Probe runtime-context introspection

- **Status:** implemented.
- **Description:** A view onto the runtime parameters in effect at probe time — host-supplied `ShaderRuntimeParams` plus loopback values.
- **User-visible behavior:** Probe output includes the runtime parameters that the engine was using at the probed phase, so consumers can reproduce or assert on them.
- **Entry points:** `ProbeRuntimeContext`, `ProbeRuntimeParam` re-exported at `crates/tui-vfx-probe/src/lib.rs:91`; `runtime_context_from_composition` (from the `fnc_runtime_context_from_composition.rs` file).
- **Inputs:** A composition spec and its runtime params.
- **Outputs:** A `ProbeRuntimeContext` carrying per-binding info.
- **Options/config:** None.
- **Data touched:** Read-only against the composition.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** This *is* part of the probe surface.
- **Tests:** Coverage in `tui-vfx-probe/tests/`.
- **Evidence:** `crates/tui-vfx-probe/src/lib.rs:91`, `crates/tui-vfx-probe/src/fnc_runtime_context_from_composition.rs` (file presence).
- **Confidence:** High.
- **Unknowns:** Per-`ProbeRuntimeParam` field shape.

### F042 — `cargo xtask` build-tooling surface

- **Status:** implemented.
- **Description:** The workspace's build-tooling binary, exposed through `cargo xtask <command>` per the `.cargo/config.toml:6` alias. Three top-level subcommands: `audit`, `docs`, `recipes`. Implemented with `clap 4 derive` (`xtask/Cargo.toml:26`).
- **User-visible behavior:** A single CLI launches every workspace-level workflow task (audit gates, doc generation, recipe validation).
- **Entry points:** Bin `xtask` at `xtask/src/main.rs:23` (the `Cli` derive); top-level `enum Commands` at `:23-40` declares `Audit { action: AuditAction }`, `Docs { action: DocsAction }`, `Recipes { action: RecipesAction }`. The dispatcher at `xtask/src/main.rs:124` matches the `cli.command`.
- **Inputs:** CLI args via `clap::Parser`; environment variable `CARGO_MANIFEST_DIR` is read at `:131-134` to resolve the workspace root for the audit subcommand.
- **Outputs:** Subcommand-specific (see F043 / F044 / `RecipesAction`).
- **Options/config:** Per subcommand. The `Recipes::Validate` variant (`xtask/src/main.rs:111-121`) accepts `--recipes-dir <path>` (required) and `--output-dir <path>` (default `docs/generated`).
- **Data touched:** Filesystem (reads source files, capabilities.toml, recipe JSON; writes to `docs/generated/`).
- **External systems:** None other than the local filesystem.
- **Errors and edge cases:** `anyhow::Result` propagation from `main`; per-subcommand error messages via `owo-colors` formatting.
- **Observability:** None of its own.
- **Tests:** `xtask/tests/test_audit_configschema.rs` (15 787 bytes; the lib target `xtask_audit_configschema` exists for testability per `xtask/Cargo.toml:17-19`).
- **Evidence:** `xtask/src/main.rs:1-167`, `xtask/Cargo.toml:13-19`, `.cargo/config.toml:6`.
- **Confidence:** High.
- **Unknowns:** None of operational note for the top-level command tree.

### F043 — `xtask audit configschema` lint

- **Status:** implemented.
- **Description:** Verifies every hand-written `impl ConfigSchema for X` carries a justification comment, or is in the baseline allowlist at `xtask/data/configschema_baseline.toml`. Per `xtask/src/main.rs:46-51`: the documentation reference is `docs/CONFIGSCHEMA_JUSTIFICATION.md`.
- **User-visible behavior:** A CI-runnable lint that fails when a new hand-written `impl ConfigSchema` lacks justification.
- **Entry points:** `cargo xtask audit configschema` (the only `AuditAction` variant at `xtask/src/main.rs:44-52`); dispatcher at `:128-143`.
- **Inputs:** Workspace-relative source files (auto-discovered); the baseline TOML.
- **Outputs:** Pass/fail exit code; per-violation diagnostic text.
- **Options/config:** Baseline file path; the rest is hard-coded.
- **Data touched:** Reads source files + baseline TOML.
- **External systems:** None.
- **Errors and edge cases:** A new impl without justification → fails. An impl on the baseline list → passes.
- **Observability:** None.
- **Tests:** `xtask/tests/test_audit_configschema.rs` (15 787 bytes — substantial integration coverage).
- **Evidence:** `xtask/src/main.rs:42-52,127-145`, `xtask/src/audit/` (5 files: `fnc_audit_configschema.rs`, `fnc_find_justification.rs`, `fnc_load_baseline.rs`, `fnc_scan_file_for_impls.rs`, `mod.rs`), `xtask/tests/test_audit_configschema.rs`, `docs/CONFIGSCHEMA_JUSTIFICATION.md` (referenced from the source).
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F044 — `xtask docs` generation surface

- **Status:** implemented.
- **Description:** A 13-action documentation surface that merges rustdoc comments and TOML-curated editorial content into multiple generated outputs (`docs/generated/`).
- **User-visible behavior:** Authors run `cargo xtask docs <action>` to regenerate capability docs (`CAPABILITIES.md`), the AI prompt (`ai-context.md`), the machine-readable JSON (`capabilities.json`), the schemas (`effect_schemas.json`), the API doc (`API.md`), and the signals reference (`RECIPE_SIGNALS_REFERENCE.md`).
- **Entry points:** 13 `DocsAction` variants at `xtask/src/main.rs:54-110`: `Generate`, `Check`, `AiContext`, `Markdown`, `Validate`, `Scaffold { write }`, `Api`, `ApiCheck`, `ApiValidate`, `ApiScaffold { write }`, `Signals`, `SignalsCheck`, `SignalsValidate`. Dispatcher at `:147-162`.
- **Inputs:** Source files (rustdoc), `docs/templates/capabilities.toml` and sibling TOMLs.
- **Outputs:** Files in `docs/generated/`; the `justfile` block at `:9-25` enumerates them.
- **Options/config:** `--write` flag on the `Scaffold` and `ApiScaffold` actions (`xtask/src/main.rs:80-83`, `:99-102`).
- **Data touched:** Filesystem reads + writes.
- **External systems:** None.
- **Errors and edge cases:** `Check` and `ApiCheck` return non-zero exit codes when generated docs are out-of-date (CI mode); per-action error messages via `anyhow`.
- **Observability:** None.
- **Tests:** No dedicated `xtask/tests/test_docs*.rs` was found; the surface is exercised by running it.
- **Evidence:** `xtask/src/main.rs:54-110,147-162`; `xtask/src/docs/` (20 files: `api_metadata.rs`, `effect_metadata.rs`, `extract_rustdoc.rs`, `extract_signals_rustdoc.rs`, `gen_ai_context.rs`, `gen_api.rs`, `gen_effect_schemas.rs`, `gen_json.rs`, `gen_markdown.rs`, `gen_signals_markdown.rs`, `merge.rs`, `merge_signals.rs`, `mod.rs`, `parse_api_toml.rs`, `parse_signals_toml.rs`, `parse_toml.rs`, `scaffold.rs`, `validate_api.rs`, `validate_coverage.rs`, `validate_signals.rs`); `justfile:9-25` (the doc-generation header).
- **Confidence:** High.
- **Unknowns:** Per-action runtime behavior in detail.

### F044a — `xtask recipes validate`

- **Status:** implemented.
- **Description:** Recipe-validation tooling that reads a recipes directory and produces per-recipe reports against `capabilities.json`.
- **User-visible behavior:** `cargo xtask recipes validate --recipes-dir <path> [--output-dir <path>]` runs the validator.
- **Entry points:** `RecipesAction::Validate { recipes_dir, output_dir }` at `xtask/src/main.rs:111-121`; dispatcher at `:163-177`.
- **Inputs:** `--recipes-dir <string>` (required), `--output-dir <string>` (default `docs/generated`).
- **Outputs:** Reports written to the output directory.
- **Options/config:** Same as Inputs.
- **Data touched:** Reads recipe JSON files; writes report files.
- **External systems:** None.
- **Errors and edge cases:** Per-recipe validation errors written to the report directory.
- **Observability:** None.
- **Tests:** No dedicated test in `xtask/tests/`.
- **Evidence:** `xtask/src/main.rs:111-121,163-177`, `xtask/src/recipes/mod.rs` (single file in the sub-module).
- **Confidence:** High (CLI surface verified). Medium on per-recipe behavior — needs reading `xtask/src/recipes/mod.rs`.
- **Unknowns:** Validator depth (whether it validates schema only, or also semantic contracts).

### F045 — Workspace-root example targets

- **Status:** implemented (example-only delivery).
- **Description:** Two registered Rust examples at the workspace root: `pipeline_effects_showcase` (prints snapshots of `Materialize`, `EdgeGrow`, `GlyphCascade`) and `direct_api_signal_strength` (constructs a Vignette filter whose `strength` is a sine signal in Rust without going through recipes; backs Intention 44).
- **User-visible behavior:** Two `cargo run -p tui-vfx --example <name>` invocations produce printed terminal-rendered output. Per `examples/README.md:1-15`: "The `tui-vfx` library ships without inline examples. Instead, 400+ effect recipes and a full interactive demo browser live in the companion `tui-vfx-recipes` crate."
- **Entry points:** `examples/pipeline_effects_showcase.rs:1-50` (the file-head rustdoc enumerates the three demos at lines 7-12); `examples/direct_api_signal_strength.rs:1-25` (the file-head rustdoc explicitly contrasts the direct-API and recipe paths).
- **Inputs:** None — they are self-contained.
- **Outputs:** Stdout (printed terminal frames + sampled values).
- **Options/config:** None.
- **Data touched:** Examples allocate their own grids and write to stdout.
- **External systems:** None.
- **Errors and edge cases:** None enumerated.
- **Observability:** None.
- **Tests:** Compilation-only (Cargo's example targets are compiled by `cargo build --examples`); no `tests/test_examples*.rs`.
- **Evidence:** `crates/tui-vfx/Cargo.toml:32-38`, `examples/pipeline_effects_showcase.rs:1-50`, `examples/direct_api_signal_strength.rs:1-25`, `examples/README.md:1-25`.
- **Confidence:** High.
- **Unknowns:** None of operational note.

### F046 — Optional `serde` integration in the foundation types

- **Status:** implemented (Cargo feature, default on).
- **Description:** A Cargo-feature gate that enables / disables serde derives on the foundation types. Default is on so most consumers see serde-enabled types without opt-in.
- **User-visible behavior:** When the `serde` feature is on, `tui-vfx-types` types serialize through serde; off-feature builds compile without serde.
- **Entry points:** `[features]` block at `crates/tui-vfx-types/Cargo.toml:26-28` (`default = ["serde"]`, `serde = ["dep:serde"]`).
- **Inputs:** Cargo feature selection at compile time.
- **Outputs:** Conditional `#[derive(Serialize, Deserialize)]` on the foundation types.
- **Options/config:** The feature gate itself.
- **Data touched:** Compile-time AST.
- **External systems:** `serde 1.0 features=["derive"] optional=true` (`crates/tui-vfx-types/Cargo.toml:20`).
- **Errors and edge cases:** Off-feature builds may fail to construct types from JSON — by design.
- **Observability:** None.
- **Tests:** `crates/tui-vfx-types/tests/*.rs` (9 files); some likely include `#[cfg(feature = "serde")]` gates (not enumerated here).
- **Evidence:** `crates/tui-vfx-types/Cargo.toml:20,26-28`.
- **Confidence:** High.
- **Unknowns:** Whether any test is gated `cfg(feature = "serde")`.

### F047 — `criterion` benches (60 fps trace-emission budget)

- **Status:** implemented (bench-only).
- **Description:** Two criterion benches in `tui-vfx-debug` plus one in `tui-vfx-geometry` that exercise hot paths and assert against a frame budget.
- **User-visible behavior:** `cargo bench -p tui-vfx-debug` and `cargo bench -p tui-vfx-geometry` run the benches; reports are written to `target/criterion/`.
- **Entry points:** `bench_emit_overhead` and `bench_full_trace_60fps` at `crates/tui-vfx-debug/Cargo.toml:30-36`; `easing` at `crates/tui-vfx-geometry/Cargo.toml:35-37`.
- **Inputs:** Bench setup data inside each `benches/*.rs`.
- **Outputs:** Criterion HTML reports (the dev-deps include `criterion = { version = "0.5", features = ["html_reports"] }` at both `tui-vfx-debug/Cargo.toml:28` and `tui-vfx-geometry/Cargo.toml:30`).
- **Options/config:** Criterion CLI flags (`--baseline`, `--save-baseline`, etc.).
- **Data touched:** None of the workspace's persistent state.
- **External systems:** `criterion 0.5`.
- **Errors and edge cases:** None enumerated.
- **Observability:** Bench output itself.
- **Tests:** None — they are benches, not tests.
- **Evidence:** `crates/tui-vfx-debug/Cargo.toml:28,30-36`, `crates/tui-vfx-geometry/Cargo.toml:30,35-37`, `crates/tui-vfx-debug/benches/bench_emit_overhead.rs`, `crates/tui-vfx-debug/benches/bench_full_trace_60fps.rs`, `crates/tui-vfx-geometry/benches/easing.rs`.
- **Confidence:** High.
- **Unknowns:** Specific budget thresholds asserted inside the benches (the `60fps` name implies ≤ 16.7 ms; the `<MARKETING.md>` claim of "≤ 2 ms/frame at 60 fps" for the full-trace bench is doc-evidence from `steering/MARKETING.md` but the criterion harness's literal threshold needs reading the bench file).

### F048 — Clean-room V3.1 surface-contract spike (`tui-vfx-next`)

- **Status:** partially implemented (spike — by design narrow scope).
- **Description:** A clean-room V3.1 surface-contract spike that proves Phase A semantic-surface rules without depending on the legacy compositor / style / content / shadow stacks. Per the crate's own module-level rustdoc at `crates/tui-vfx-next/src/lib.rs:3-9`: "intentionally small: a surface, scope evaluation, write policy, diagnostics, and two tiny effects that make role preservation and explicit role writes testable."
- **User-visible behavior:** A `SurfaceEngine` accepts a `Surface` (a `Grid` paired with `RoleMap`-like role storage and `SurfaceMetadata`), evaluates a `ScopeSpec` against `ScopeEvalInput`, applies `EffectDescriptor`-bound effects (`DimEffect` for visual-only cell mutation; `ExplicitRoleWriteEffect` for procedural role-writing), and returns `ApplyOutcome` with diagnostics.
- **Entry points:** `SurfaceEngine`, `ApplyOutcome` (`crates/tui-vfx-next/src/lib.rs:20`); `Surface`, `SurfaceMetadata`, `CellChannel` (`:22`); `ScopeSpec`, `ScopeEvalInput`, `RoleSpace`, `CoordinateSpace` (`:21`); `EffectDescriptor`, `EffectDomain`, `DimEffect`, `ExplicitRoleWriteEffect` (`:19`); `SurfaceDiagnostic`, `SurfaceDiagnosticCode`, `DiagnosticLevel` (`:18`); `CellWrite`, `CellWritePolicy`, `RoleWritePolicy` (`:23`).
- **Inputs:** A `Surface` (constructed against `tui_vfx_types::OwnedGrid` per `crates/tui-vfx-next/src/surface.rs:5`), a `ScopeSpec`, an `EffectDescriptor`-bound effect.
- **Outputs:** `ApplyOutcome` carrying diagnostic events plus the mutated surface state.
- **Options/config:** `EffectDomain::{Visual, Procedural}` — visual effects may change cell visual channels but not roles; procedural effects may generate cells and explicit roles (per `crates/tui-vfx-next/src/effect.rs:11-17`).
- **Data touched:** Surface cells; surface roles (procedural domain only).
- **External systems:** None (the crate depends only on `tui-vfx-types` and `tui-vfx-geometry`; **no** `mixed-signals`, **no** `serde`).
- **Errors and edge cases:** Per-effect domain rules are enforced — `DimEffect` (Visual) preserves roles; `ExplicitRoleWriteEffect` (Procedural) is allowed to write roles. Diagnostics are emitted as `SurfaceDiagnostic` with `SurfaceDiagnosticCode` and `DiagnosticLevel`.
- **Observability:** `SurfaceDiagnostic` is the spike's own diagnostic surface; it does **not** integrate with the F036 inspection foundation in this audit.
- **Tests:** `crates/tui-vfx-next/tests/surface_contract.rs` (one integration test file).
- **Evidence:** `crates/tui-vfx-next/Cargo.toml:1` (`<DESC>` "Clean-room v3.1 surface contract spike crate"), `crates/tui-vfx-next/src/lib.rs:1-23` (full module + re-exports), `crates/tui-vfx-next/src/effect.rs:1-30` (the `EffectDomain` rules), `crates/tui-vfx-next/src/surface.rs:1-30` (the `Surface` + `CellChannel` contract).
- **Confidence:** High. Status is `partially implemented` because it is by design a narrow spike; chapter 12 records the open question of when (or whether) it is promoted into the live engine path.
- **Unknowns:** Whether `tui-vfx-next` is the planned **replacement** surface for the legacy compositor pipeline or a **research surface** that informs the V3 cutover without itself shipping. The crate's `<DESC>` says "spike", which leans toward research.

### F049 — Cell-write and role-write policy contracts

- **Status:** partially implemented (the V3.1 spike's write surface).
- **Description:** Explicit policy types that gate which channels of a cell an effect can write. Visual effects flow through a `CellWritePolicy`; role-writing effects are additionally gated by a `RoleWritePolicy`.
- **User-visible behavior:** Effect descriptors carry `can_write_roles: bool` (per `crates/tui-vfx-next/src/effect.rs:25-26`); the engine refuses cell writes that violate the policy.
- **Entry points:** `CellWrite`, `CellWritePolicy`, `RoleWritePolicy` re-exported at `crates/tui-vfx-next/src/lib.rs:23` (`pub use write::{CellWrite, CellWritePolicy, RoleWritePolicy}`).
- **Inputs:** A `CellWrite` proposal (channel + value); the active `CellWritePolicy` and (for role channel) `RoleWritePolicy`.
- **Outputs:** Approve / reject decision plus a diagnostic on rejection.
- **Options/config:** Per-policy variants (full enumeration requires reading `crates/tui-vfx-next/src/write.rs` end-to-end; the file is 52 lines and is the policy-definition surface).
- **Data touched:** Surface cell channels.
- **External systems:** None.
- **Errors and edge cases:** Policy violations emit `SurfaceDiagnostic` events.
- **Observability:** Diagnostic events.
- **Tests:** `crates/tui-vfx-next/tests/surface_contract.rs`.
- **Evidence:** `crates/tui-vfx-next/src/write.rs:1-52`, `crates/tui-vfx-next/src/effect.rs:11-26`, `crates/tui-vfx-next/src/lib.rs:23`.
- **Confidence:** High. (The contract types exist and the test exercises the full surface.)
- **Unknowns:** The full set of `CellWritePolicy` and `RoleWritePolicy` variants — full read deferred.



## 3.3 Notes on the inventory

- **Engine API vs recipe layer.** `crates/tui-vfx/src/lib.rs:14-31` (the `## Audiences` rustdoc block) explicitly declares two consumption surfaces: the engine API (this chapter's features) and the recipe-authoring layer in the sibling `tui-vfx-recipes` crate. The recipe-authoring surface is out of scope for this PRD because it is not a workspace member; chapter 6 records the in-tree references to it.
- **V2 vs V3.** Several features are V3-in-flight (notably F009 V3 shaders, F011 bindable values that recently landed packet 69-A). Where the V3 surface coexists with V2, both are listed and the status reflects the as-built mix.
- **Naming-only entries excluded.** Items present only as a directory or single file with no re-export and no test reference are recorded in chapter 12 (Open Questions) instead of as features. Examples include the `compositor/widgets/` directory which exists but contains only `mod.rs` at audit-time; whether it carries a public surface is not yet established.

## 3.4 Confidence summary

47 features identified. Of those, 39 carry **High** confidence (code path + re-export + at minimum one `tests/`-or-`benches/`-or-public-API piece of corroborating evidence). 8 carry **Medium** confidence pending a deeper read of the directory in question to enumerate the parameter surface; those rows are flagged inline in the table. None are **Low** at this stage.

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/03_feature_inventory.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
