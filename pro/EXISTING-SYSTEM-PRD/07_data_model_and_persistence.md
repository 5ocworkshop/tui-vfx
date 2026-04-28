<!-- <FILE>pro/EXISTING-SYSTEM-PRD/07_data_model_and_persistence.md</FILE> - <DESC>Chapter 7 of the evidence-backed Existing-System PRD: domain types, file formats, file-system reads/writes, and the single SQLite-based persistence surface.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>US-007 — data model and persistence.</WCTX> -->
<!-- <CLOG>0.1.0: initial population.</CLOG> -->

# 7. Data Model and Persistence

## 7.1 Domain types (per crate)

The taxonomy of public types follows from chapter 6's per-crate Rust API enumeration. This section names the load-bearing domain types and the crates that own them. (Per-field schemas live alongside the type definition; this chapter does not duplicate every struct's full field list.)

| Type | Crate | Role | Evidence |
|---|---|---|---|
| `Cell` | `tui-vfx-types` | A single cell (char + style); the workspace's atomic unit of cell-grid output | `crates/tui-vfx-types/src/lib.rs:93` |
| `Color` | `tui-vfx-types` | RGBA color with alpha-compositing semantics | `crates/tui-vfx-types/src/lib.rs:95` |
| `Style` | `tui-vfx-types` | Combined fg + bg + modifiers | `crates/tui-vfx-types/src/lib.rs:109` |
| `Modifiers` | `tui-vfx-types` | Bitmask of text modifiers (bold / italic / underline / etc.) | `crates/tui-vfx-types/src/lib.rs:100` |
| `Anchor` / `Point` / `Rect` / `Size` | `tui-vfx-types` | Geometry primitives | `crates/tui-vfx-types/src/lib.rs:96` |
| `Grid` (trait) / `OwnedGrid` / `GridExt` / `BoundaryMode` | `tui-vfx-types` | The cell-buffer interface | `crates/tui-vfx-types/src/lib.rs:97` |
| `SemanticScene` / `RoleMap` / `RoleMapIter` / `RoleTag` (12 first-class + `Custom(InternedRoleName)`) / `RoleId` / `RoleInterner` / `SceneMetadata` | `tui-vfx-types` | Role-tagged destination scene | `crates/tui-vfx-types/src/lib.rs:14-32, 103-108` |
| `LayerId` / `RecipeId` / `InternedString` | `tui-vfx-types` | Opaque ID newtypes backed by `Arc<str>` | `crates/tui-vfx-types/src/lib.rs:31, 98-103` |
| `RigidShakeState` / `RigidShakeTiming` | `tui-vfx-types` | Shared timing state for the shake filter and shake-style effect | `crates/tui-vfx-types/src/lib.rs:84, 102` |
| `VfxCellContext` | `tui-vfx-types` | Seven-field `Copy` per-cell bundle (Slice 6.6 §F.1) | `crates/tui-vfx-types/src/lib.rs:94` |
| `VfxBindable<T, S>` / `VfxBindableValue` / `VfxBindableU16` / `VfxBindableString` | `tui-vfx-core` | Three-arm bindable wire form (Literal / Binding / Signal) | `crates/tui-vfx-core/src/lib.rs:11-15`; `bindable/cls_bindable.rs:24,49,73,167` |
| `BindableSignal` (trait) / `RuntimeParamsRead` (trait) | `tui-vfx-core` | Trait surface implemented by signal types and runtime-params readers | `crates/tui-vfx-core/src/lib.rs:11-15` |
| `ConfigSchema` (trait) / `FieldMeta` / `Range` / `ScalarValue` / `SchemaField` / `SchemaNode` / `SchemaVariant` | `tui-vfx-core` | Schema-bridge types consumed by xtask doc-generation | `crates/tui-vfx-core/src/lib.rs:16-18` |
| `TimeSpec` | `tui-vfx-core` | `{ start: Instant, now: Instant, duration: Duration }` with `progress() -> f64` | `crates/tui-vfx-core/src/time_spec.rs:13-22` |
| `CompositionSpec` / `CompositionOptions` / `CompositionPlaybackTiming` / `ShaderLayerSpec` / `ShaderWithRegion` / `RenderArea` | `tui-vfx-compositor` | Pipeline driver types — the inputs to `render_pipeline_with_spec` | `crates/tui-vfx-compositor/src/pipeline/mod.rs:25-30` |
| `FilterSpec` / `MaskSpec` / `SamplerSpec` / `ShadowSpec` / `BindableValue` / `mask_combine_mode::MaskCombineMode` | `tui-vfx-compositor` | Public wire-format types for stage selection | `crates/tui-vfx-compositor/src/types/` (filesystem listing); `pipeline/mod.rs:24` (`ShadowSpec` re-export) |
| `ShadowConfig` / `ShadowEdges` / `ShadowStyle` / `ShadowCompositeMode` / `ShadowGradeConfig` / `CellMask` | `tui-vfx-shadow` | Shadow-stage configuration types | `crates/tui-vfx-shadow/src/lib.rs:354-363` |
| `ContentEffect` (the 15-variant text-transformer enum) / `TypewriterCursor` / `ScrambleCharset` / `DissolveDirection` | `tui-vfx-content` | Text-effect domain types | `crates/tui-vfx-content/src/types/` (filesystem listing); `crates/tui-vfx-content/src/lib.rs:33-90` (rustdoc enumerates the variants) |
| `CellActor` / `CellMotionSpec` / `CellMotionStats` / `CellMotionVisibility` / `CellMotionAffect` / `CellMotionCoord` / `CellMotionError` / `CellMotionPhaseSpec` / `CellMotionScope` / `CellCollisionMode` / `CellPlacement` / `CellStagger` | `tui-vfx-content` | V3 packet 1 cell-motion scheduler types | `crates/tui-vfx-content/src/cell_motion/mod.rs:1-30` |
| `RocketsplashImage` / `RocketsplashFont` / `FontRender` | `tui-vfx-content` | External-asset source primitives (consume `rocketsplash_rt::RenderBuffer`) | `crates/tui-vfx-content/src/sources/mod.rs:21-23` |
| `FontGlyphTable` (with `Line3x3` variant) / `FontRegistry` / `Pool<T>` / `EffectPool` / `ImagePool` / `FontPool` / `PresetPool` / `TextPool` / `Preset` / `PoolPolicy` / `AssetRegistry` | `tui-vfx-content` | Asset / pool / font-registry types | `crates/tui-vfx-content/src/{fonts, pool, assets}/mod.rs` |
| Per-shader `cls_*_shader` types + `StyleEffect` / `StyleConfig` / `StyleLayer` / `StyleRegion` / `StyleTransition` / `FadeEffect` / `FadeSpec` / `BlendMode` / `ColorConfig` / `ColorRamp` / `ColorSpace` / `Gradient` / `GradientLut` / `FalloffType` / `NoiseType` / `SignalColor` | `tui-vfx-style` | Style-effect catalog (50 files) and the V3 `Vfx*` family (11 files in `models/v3/`) | `crates/tui-vfx-style/src/models/` directory listing |
| `MotionPath` (trait) + 9 path classes + `WipeDirection` / `Anchor` / `Origin` / `PathType` / `Position` / `PositionSpec` / `RectScaleSpec` / `SignedRect` / `SlideDirection` / `SnappingStrategy` / `TransitionSpec` | `tui-vfx-geometry` | Geometry primitives; chapter 3 F022..F025 enumerate | `crates/tui-vfx-geometry/src/lib.rs:18-23`; `paths/mod.rs:18-26` |
| `LogLevel` / `ModuleConfig` / `DebugLogger` / `Logger` / `LogEntry` | `tui-vfx-debug` | Debug-logger types | `crates/tui-vfx-debug/src/lib.rs:37-38` |
| `TraceEvent` / `TraceEnvelope` / `TraceFilter` / `TraceSelector` / `StageMask` / `InspectionSink` (trait) / `TraceSink` / `TraceReport` / `TraceEmitter` / `AssertingInspector` / `PipelineStageKind` / `PipelineSkipReason` / `RoleHistogram` / `RoleMapSource` | `tui-vfx-debug::inspection` | Inspection-foundation taxonomy | `crates/tui-vfx-debug/src/inspection/` (14 files) |
| 24 `cls_probe_*` DTOs (chapter 3 F038 enumerates) + `ProbeRequest` / `ProbeCellSelector` / `ProbePhase` / `ProbeRuntimeContext` / `ProbeRuntimeParam` / `ProbeSqliteStore` | `tui-vfx-probe` | Pipeline-observability DTOs | `crates/tui-vfx-probe/src/lib.rs:75-99` |
| `Surface` / `SurfaceMetadata` / `SurfaceEngine` / `ApplyOutcome` / `ScopeSpec` / `EffectDescriptor` / `EffectDomain` / `DimEffect` / `ExplicitRoleWriteEffect` / `SurfaceDiagnostic` / `SurfaceDiagnosticCode` / `DiagnosticLevel` / `CellChannel` / `CoordinateSpace` / `RoleSpace` / `ScopeEvalInput` / `CellWrite` / `CellWritePolicy` / `RoleWritePolicy` | `tui-vfx-next` | V3.1 spike surface (chapter 3 F048 / F049) | `crates/tui-vfx-next/src/lib.rs:11-23` |

## 7.2 File formats (read at runtime)

The workspace's runtime libraries do **not** read files. All `fs::*` / `File::*` / `read_to_string` call sites are in **build tooling (xtask)**, **the `pipeline-probe` binary**, **NDJSON round-trip in `tui-vfx-debug::inspection`**, and **a single style-crate test fixture** that loads recipe JSON for testing.

| Format | Read by | Site | Purpose |
|---|---|---|---|
| `ProbeSceneSpec` JSON | `pipeline-probe` binary | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:97` (`serde_json::from_str(&fs::read_to_string(input_path)?)?`) | Input scene spec to probe |
| Recipe JSON (V2) | a style test | `crates/tui-vfx-style/tests/models/test_terminal_fire_recipes.rs:29` (`std::fs::read(&full)`) | Test fixture loading |
| TraceReport NDJSON | `tui-vfx-debug::inspection::cls_trace_report` | `crates/tui-vfx-debug/src/inspection/cls_trace_report.rs:20-75` (uses `std::io::{BufRead, BufReader, Read, Write}`) | NDJSON round-trip for inspection reports |
| Source files (`*.rs`) | `xtask audit configschema` | `xtask/src/audit/fnc_audit_configschema.rs:83`, `fnc_load_baseline.rs:41` (both use `std::fs::read_to_string`) | Workspace lint scan |
| `docs/templates/*.toml` | `xtask docs *` | `xtask/src/docs/parse_toml.rs:195`, `parse_signals_toml.rs:70`, `parse_api_toml.rs` (all use `fs::read_to_string`) | Editorial inputs to doc generation |
| Recipe JSON (V2/V3) directories | `xtask recipes validate` | `xtask/src/recipes/mod.rs:10` (`use std::fs;`) | Recipe-validator input |

## 7.3 File formats (written at runtime)

| Format | Written by | Site | Purpose |
|---|---|---|---|
| `docs/generated/CAPABILITIES.md`, `ai-context.md`, `capabilities.json`, `effect_schemas.json`, `API.md`, `RECIPE_SIGNALS_REFERENCE.md` | `xtask docs *` | `xtask/src/docs/gen_ai_context.rs:29`, `gen_effect_schemas.rs:67`, `gen_json.rs:35`, `gen_api.rs:63`, `gen_signals_markdown.rs:51`, `mod.rs:111+` | Generated documentation outputs |
| TOML stub scaffolds | `xtask docs scaffold[-write]` | `xtask/src/docs/scaffold.rs:11-12, 76` (`fs::OpenOptions::new()`) | Author-side TOML scaffolds |
| TraceReport NDJSON | `tui-vfx-debug::inspection::cls_trace_report` | `crates/tui-vfx-debug/src/inspection/cls_trace_report.rs:20-75` | NDJSON round-trip output |
| Probe SQLite database | `pipeline-probe` (when `--sqlite-query` and the store are engaged) | `crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs` | Persisted probe runs |

## 7.4 `include_bytes!` / `include_str!`

A workspace-wide search (`grep -rn "include_str!\|include_bytes!" crates/*/src/ xtask/src/`) returned **only rustdoc-example mentions**, not production-code uses:

- `crates/tui-vfx-content/src/sources/cls_rocketsplash_image.rs:19,37` — example rustdoc snippets showing `RocketsplashImage::from_bytes(include_bytes!("logo.rss"))?` as a recommended pattern.
- `crates/tui-vfx-content/src/sources/cls_rocketsplash_font.rs:22` — same pattern for fonts.
- `crates/tui-vfx-content/src/pool/mod.rs:39` — rustdoc note: "the caller is expected to maintain an AssetMap that resolves each name to `.rss` or `.rsf` bytes at render time. […] (embed via `include_bytes!`, load from disk, stream from network, whatever)."

No production code path embeds a static asset via `include_bytes!` at audit-time. The library accepts byte sources from the consumer (per Intention 27) — embedding is the consumer's choice.

## 7.5 Database / cache

| System | Engine | Used by | Source | Status |
|---|---|---|---|---|
| Probe SQLite store | `rusqlite 0.32 features=["bundled"]` | `pipeline-probe` binary | `crates/tui-vfx-probe/Cargo.toml:25`; `crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs`; `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:14` (`use std::fs;` plus `rusqlite` indirectly via the store) | implemented |

A workspace-wide search for `sled::`, `rocksdb::`, `sqlx::`, `diesel::`, `sea_orm::` returned **zero matches**. SQLite (via rusqlite) is the only persistent-storage engine present at audit-time. No in-memory cache layer (LRU, etc.) was observed in production code.

## 7.6 State machines / lifecycle enums

The compositor pipeline defines a four-stage canonical order: Sampler → Mask → Shader → Filter (per the rustdoc on `crates/tui-vfx-shadow/src/lib.rs:25-30` plus `crates/tui-vfx-compositor/src/pipeline/mod.rs` re-exports). Lifecycle phases are catalogued by `tui_vfx_probe::ProbePhase::{Entering, Dwelling, Exiting}` (`crates/tui-vfx-probe/src/cls_probe_request.rs`; chapter 5 OPT-009). The shadow stage runs **before** the element it shadows (`crates/tui-vfx-shadow/src/lib.rs:18-30`).

`tui-vfx-debug::inspection`'s `PipelineStageKind` and `PipelineSkipReason` enums (`crates/tui-vfx-debug/src/inspection/cls_pipeline_stage_kind.rs`, `cls_pipeline_skip_reason.rs`) catalogue the per-stage state taxonomy at the trace-event level.

## 7.7 Migrations / versioning

The workspace is pre-1.0 (`Cargo.toml:31`). No migration tooling is present at audit-time:

- No `migrations/` directory in any crate.
- No `version`-tagged enum at the wire-format level (the V2/V3 split lives at the recipe-schema level in `tui-vfx-recipes`, out of audit scope).
- The recipe-schema V2→V3 cutover is the explicit version-bump path (per Intention 10 / Decision 4); the tooling is the `xtask docs` family + the in-tree V3 lowering function `fnc_lower_legacy_spatial_shader.rs` (chapter 3 F009).

## 7.8 Confidence

**High** for every catalogued type and persistence site — each is verified against re-exports, file paths, or direct `grep`/`ofpf-content` evidence. The "no evidence" rows for sled / rocksdb / sqlx / diesel / sea_orm and for migration tooling are valid evidence of absence (empty result sets from workspace-wide queries).

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/07_data_model_and_persistence.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
