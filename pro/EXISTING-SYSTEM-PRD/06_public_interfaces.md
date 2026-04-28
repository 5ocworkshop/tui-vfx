<!-- <FILE>pro/EXISTING-SYSTEM-PRD/06_public_interfaces.md</FILE> - <DESC>Chapter 6 of the evidence-backed Existing-System PRD: every public interface — CLI, public Rust API per crate, FFI/WASM (absent), file formats, schemas, message/event protocols.</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>Mid-audit: add tui-vfx-next public Rust API surface as §6.2.5a.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — add tui-vfx-next §6.2.5a covering the new spike crate's public surface. 0.1.0: initial population.</CLOG> -->

# 6. Public Interfaces

## 6.1 CLI

Two binaries. Their flag surfaces are catalogued in chapter 5 (OPT-001..OPT-016). Summary:

| Binary | Crate | Source | Parser | Subcommand surface |
|---|---|---|---|---|
| `xtask` | `xtask` | `xtask/src/main.rs:13-15` (`[[bin]] name = "xtask"`) | `clap 4 derive` (`xtask/Cargo.toml:26`) | 3 top-level commands × N actions: `Audit::Configschema` (1); `Docs::{Generate, Check, AiContext, Markdown, Validate, Scaffold, Api, ApiCheck, ApiValidate, ApiScaffold, Signals, SignalsCheck, SignalsValidate}` (13); `Recipes::Validate` (1) |
| `pipeline-probe` | `tui-vfx-probe` | `crates/tui-vfx-probe/Cargo.toml:29-31` (`[[bin]]`) | manual `std::env::args` parsing (`crates/tui-vfx-probe/src/bin/pipeline-probe.rs:35-90`) | Single binary; flags select between frame-dump (default), timeline (`--frames N`), or diff (`--diff-to T`) modes |

The `cargo xtask` alias at `.cargo/config.toml:6` (`xtask = "run --package xtask --"`) is the canonical invocation form. `pipeline-probe` is invoked directly via `cargo run -p tui-vfx-probe --bin pipeline-probe -- <flags>` or after `cargo build` as the produced binary.

## 6.2 Public Rust API (per crate)

Each crate's `src/lib.rs` is the public-API root. The crate-level re-export surface (the lines that bound what consumers can `use`) is enumerated below.

### 6.2.1 `tui-vfx` (meta-crate)

`crates/tui-vfx/src/lib.rs:186-195`. Re-exports every effect-bearing sibling as a sub-module:

```
pub use tui_vfx_compositor as compositor;
pub use tui_vfx_content as content;
pub use tui_vfx_core as core;
pub use tui_vfx_geometry as geometry;
pub use tui_vfx_shadow as shadow;
pub use tui_vfx_style as style;
pub use tui_vfx_types as types;

pub mod prelude { ... }
```

The `prelude` module (`:195`) is the curated convenience surface for the common case (chapter 3 F045's example uses `use tui_vfx::prelude::*`).

### 6.2.2 `tui-vfx-types`

`crates/tui-vfx-types/src/lib.rs:72-109`. Public sub-modules: `braille`, `color_inert`, `glyph`, `rigid_shake_timing`. Public re-exports: `Cell`, `VfxCellContext`, `Color`, `Anchor`/`Point`/`Rect`/`Size`, `BoundaryMode`/`Grid`/`GridExt`/`OwnedGrid`, `InternedString`, `LayerId`, `Modifiers`, `RecipeId`, `RigidShakeState`/`RigidShakeTiming`, `RoleId`, `RoleInterner`, `RoleMap`/`RoleMapIter`, `InternedRoleName`/`RoleTag`, `SceneMetadata`, `SemanticScene`, `Style`.

### 6.2.3 `tui-vfx-core`

`crates/tui-vfx-core/src/lib.rs:6-23`. Sub-modules: `bindable`, `mixed_signals_schema`, `schema`, `time_spec`. Re-exports: `BindableSignal`, `RuntimeParamsRead`, `VfxBindable`, `VfxBindableString`, `VfxBindableU16`, `VfxBindableValue`; `ConfigSchema`, `FieldMeta`, `Range`, `ScalarValue`, `SchemaField`, `SchemaNode`, `SchemaVariant`; `TimeSpec`; `ConfigSchema` (as a derive shim from `tui-vfx-core-macros`).

### 6.2.4 `tui-vfx-core-macros`

`crates/tui-vfx-core-macros/src/lib.rs:28`. Single public function: `derive_config_schema`.

### 6.2.5 `tui-vfx-geometry`

`crates/tui-vfx-geometry/src/lib.rs:7-23`. Sub-modules: `anchors`, `borders`, `easing`, `layout`, `paths`, `traits`, `transitions`, `types`, `widgets`, `wipe`. Re-exports: `MotionPath`; `Anchor`, `Origin`, `PathType`, `Position`, `PositionSpec`, `RectScaleSpec`, `SignedRect`, `SlideDirection`, `SnappingStrategy`, `TransitionSpec`, `WipeDirection`; `wipe_progress`, `wipe_visible_at`.

### 6.2.5a `tui-vfx-next` (V3.1 surface-contract spike)

`crates/tui-vfx-next/src/lib.rs:11-23`. Public sub-modules: `diagnostic`, `effect`, `engine`, `scope`, `surface`, `write`. Re-exports: `DiagnosticLevel`, `SurfaceDiagnostic`, `SurfaceDiagnosticCode`; `DimEffect`, `EffectDescriptor`, `EffectDomain`, `ExplicitRoleWriteEffect`; `ApplyOutcome`, `SurfaceEngine`; `CoordinateSpace`, `RoleSpace`, `ScopeEvalInput`, `ScopeSpec`; `CellChannel`, `Surface`, `SurfaceMetadata`; `CellWrite`, `CellWritePolicy`, `RoleWritePolicy`. The crate is a clean-room spike (per its `<DESC>` block at `crates/tui-vfx-next/Cargo.toml:1`); it is not imported by any other workspace member at audit-time.

### 6.2.6 `tui-vfx-compositor`

`crates/tui-vfx-compositor/src/lib.rs:6-14`. Public sub-modules: `context`, `pipeline`, `traits`, `types`, `utils`, `widgets`. **`filters`, `masks`, `samplers` are `pub(crate)`** — consumers reach them through `FilterSpec` / `MaskSpec` / `SamplerSpec` in `types/`. The `pipeline` module re-exports `CompositionOptions`, `CompositionPlaybackTiming`, `CompositionSpec`, `RenderArea`, `ShaderLayerSpec`, `ShaderWithRegion`, `ShadowSpec`, `blend_shadow_cell`, `blend_underlying_shadow_cell`, `check_masks`, `grade_shadow_cell`, `render_pipeline`, `render_pipeline_with_area`, `render_pipeline_with_spec`, `render_pipeline_with_spec_area` (`crates/tui-vfx-compositor/src/pipeline/mod.rs:24-37`).

### 6.2.7 `tui-vfx-style`

`crates/tui-vfx-style/src/lib.rs:6-9`. Sub-modules: `models`, `schedules`, `traits`, `utils`. The full per-shader / per-style-effect class catalog lives under `models/` (50 files) and `models/v3/` (V3 `Vfx*` shader family). Re-exports per the `models/mod.rs` are extensive (the file is the highest-fan-out core in the workspace at 143 incoming logic edges per `ofpf-orientation`); chapter 3 §3.1 row F008 enumerates the named-shader set.

### 6.2.8 `tui-vfx-content`

`crates/tui-vfx-content/src/lib.rs:111-123`. Public sub-modules: `assets`, `cell_motion`, `cursor`, `fonts`, `glyph_particles`, `pool`, `prelude`, `sources`, `traits`, `transformers`, `types`, `utils`. The `mechanical` sub-module is **private** (`mod mechanical;` at `:116`). The `prelude` (`:118`) is the convenience surface.

### 6.2.9 `tui-vfx-shadow`

`crates/tui-vfx-shadow/src/lib.rs:350-363`. Sub-modules: `renderers`, `types`. Re-exports: `CellMask`, `extract_shadow_envelope`; the `fnc_render_shadow` family (full re-export list at `:355-359`); `ShadowCompositeMode`, `ShadowConfig`, `ShadowEdges`, `ShadowGradeConfig`, `ShadowStyle`; the `renderers` sub-modules.

### 6.2.10 `tui-vfx-debug`

`crates/tui-vfx-debug/src/lib.rs:32-38`. Public sub-module: `inspection` (the trace-event surface). Re-exports: `LogLevel`, `ModuleConfig`; `DebugLogger`, `LogEntry`, `Logger`, `create_logger`, `get_global_logger`. The `config` and `logger` sub-modules are private (`mod config;`, `mod logger;` at `:32,34`).

### 6.2.11 `tui-vfx-probe`

`crates/tui-vfx-probe/src/lib.rs:75-107`. Twenty-four `cls_probe_*` DTO re-exports plus a function family (`build_probe_cell_root_cause`, `infer_roles_from_grid`, `collect_basic_diagnostics`, `collect_loopback_fire_diagnostics`, `collect_probe_operational_analysis`, `diff_frames`, `find_widget_cell`, `has_ascii_alpha`). Chapter 3 F038/F040 enumerate the catalogue.

### 6.2.12 `xtask`

`xtask/Cargo.toml:17-19` declares an additional `[lib] name = "xtask_audit_configschema"` library target alongside the binary. The library exists for testability (`xtask/tests/test_audit_configschema.rs` consumes it). The library's public surface is enumerated in `xtask/src/lib.rs` (read at audit-time: 21-line file declaring the audit-callable surface).

## 6.3 FFI / WASM / extern bindings

**No evidence found** after inspecting the workspace.

A workspace-wide search for `extern "C"`, `#[no_mangle]`, `wasm_bindgen`, `napi`, `pyo3`, `jni`, and `cxx` returned **zero matches** in workspace source (`ofpf-content "extern \"C\""` etc., empty result sets). `tui-vfx` does not export an FFI / WASM / cross-language surface at audit-time.

## 6.4 HTTP / RPC / gRPC / GraphQL / WebSocket / Message Queues

**No evidence found** after inspecting the workspace.

A workspace-wide search for `TcpListener`, `UdpSocket`, `hyper::`, `axum::`, `tonic::`, `actix`, `rocket::`, `warp::`, `jsonrpsee`, `tarpc` returned **zero matches** in workspace source. None of these crates appear in any `[dependencies]` block (chapter 2 §2.4.3 enumerates the full per-crate dep matrix). `tui-vfx` does not bind a network port, accept connections, or speak any RPC protocol at audit-time.

## 6.5 File formats

| Format | Role | Read by | Written by | Source |
|---|---|---|---|---|
| Recipe JSON (V2 / V3) | The authoring wire format for compositor scenes | `tui-vfx-recipes` (sibling repo, out of workspace scope) and `xtask recipes validate` (which consumes recipe JSON for validation reports) | Authors (humans / AI) | `xtask/src/main.rs:114-115` (`--recipes-dir <string>` accepts a directory of recipe JSON); the canonical loader is in `tui-vfx-recipes` (out-of-scope) |
| `.rss` (rocketsplash splash image) | Static cell-art splash image | `RocketsplashImage` source (F017) at `crates/tui-vfx-content/src/sources/cls_rocketsplash_image.rs` | Sibling tool (rocketsplash) | `crates/tui-vfx-content/src/sources/mod.rs:9-12` describes the format as carrying through `rocketsplash_rt::RenderBuffer` |
| `.rsf` (rocketsplash font atlas) | Custom font atlas for splash / display use | `RocketsplashFont` source (F018) at `crates/tui-vfx-content/src/sources/cls_rocketsplash_font.rs` | Sibling tool (rocketsplash) | `crates/tui-vfx-content/src/sources/mod.rs:9-12` |
| `.rsi`, `.rsb` | Cell-coarse and braille-supersampled image variants of the rocketsplash family | (consumed via `rocketsplash-rt 0.2.2`; in-tree code does not directly handle the file extensions) | Sibling tool (rocketsplash) | Workspace `Cargo.toml:63-64` cites the integration plan at `docs/internal/plans/splash-library-and-vfx-integration.md`; the in-tree consumer is `RocketsplashImage` |
| `docs/templates/capabilities.toml` | Editorial input to capability-doc generation | `xtask docs generate` (and sibling actions) | Authors | `xtask/src/docs/parse_toml.rs` (file presence) |
| `docs/templates/api_docs.toml` | Editorial input to API-doc generation | `xtask docs api` (and siblings) | Authors | `xtask/src/docs/parse_api_toml.rs` (file presence) |
| `docs/templates/signals.toml` | Editorial input to signals-doc generation | `xtask docs signals` (and siblings) | Authors | `xtask/src/docs/parse_signals_toml.rs` (file presence) |
| `xtask/data/configschema_baseline.toml` | Baseline allowlist for the configschema audit | `xtask audit configschema` | Maintainers (manual) | `xtask/src/audit/fnc_load_baseline.rs` (file presence); `xtask/src/main.rs:46-47` (rustdoc pointer) |
| `docs/generated/CAPABILITIES.md`, `ai-context.md`, `capabilities.json`, `effect_schemas.json`, `API.md`, `RECIPE_SIGNALS_REFERENCE.md` | Generated documentation outputs | (read by humans / AI) | `xtask docs generate` | `justfile:9-25` (the doc-generation header enumerates the output set) |
| Probe SQLite database | Persisted probe-frame store | `pipeline-probe --sqlite-query` | `pipeline-probe` (when the SQLite store is engaged) | `crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs` |
| `tests/criterion/` | criterion bench output | (read by humans / CI) | `cargo bench` | `crates/tui-vfx-debug/Cargo.toml:30-36`, `crates/tui-vfx-geometry/Cargo.toml:35-37` |

The recipe JSON V2 / V3 schemas themselves are derived from the `ConfigSchema` machinery (F032) but the canonical loader code lives in the sibling `tui-vfx-recipes` crate, which is outside the workspace under audit.

## 6.6 Schemas

| Schema | Owner crate | Generator | Output |
|---|---|---|---|
| `ConfigSchema`-driven recipe / effect schema | `tui-vfx-core` | `xtask docs effect-schemas` (via `xtask/src/docs/gen_effect_schemas.rs`) | `docs/generated/effect_schemas.json` |
| Capabilities catalog | `xtask` | `xtask docs generate` | `docs/generated/capabilities.json` |
| Signals reference | `xtask` (signals subcommand family) | `xtask docs signals` | `docs/generated/RECIPE_SIGNALS_REFERENCE.md` |
| Probe scene spec | `tui-vfx-probe::ProbeSceneSpec` | (consumed at runtime — JSON shape is the type itself) | input JSON to `pipeline-probe` |
| Probe report (timeline / diff / single-frame) | `tui-vfx-probe::ProbeReport`, `ProbeTimelineReport`, `ProbeDiffReport` | `pipeline-probe` runtime | stdout / SQLite / file (per `--format`) |

## 6.7 Plugin / extension surfaces

The workspace exposes two extension points for downstream consumers:

1. **Custom `Grid` implementations.** Any consumer can implement `tui_vfx_types::Grid` for a custom buffer type and pass it as `&dyn Grid` to `render_pipeline*`. The `BoundaryMode` enum chooses out-of-bounds behavior. (F026.)
2. **Custom inspection sinks.** `tui_vfx_debug::inspection::InspectionSink` is a trait downstream consumers implement to receive trace events; the compositor's `InspectionSinkBridge` adapter forwards `CompositorInspector` callbacks into any registered `InspectionSink`. (F036.)
3. **Custom roles via `RoleTag::Custom(InternedRoleName)`.** Consumers can register named roles beyond the 12 first-class variants. (F027.)
4. **Custom `ConfigSchema` impls.** Hand-written `impl ConfigSchema for X` is allowed, but each must carry a justification comment or appear in the audit baseline (F043).

No FFI / WASM / dynamic-load plugin loader is present.

## 6.8 Confidence

**High** for every interface surface catalogued. The "no evidence found" sections (FFI / WASM, network protocols) are derived from workspace-wide `ofpf-content` searches with empty result sets — that is valid evidence of absence per the `pro/REVERSE-PRD.md` uncertainty rules.

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/06_public_interfaces.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
