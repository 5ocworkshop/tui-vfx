<!-- <FILE>pro/EXISTING-SYSTEM-PRD/08_runtime_behavior.md</FILE> - <DESC>Chapter 8 of the evidence-backed Existing-System PRD: startup sequence, async / concurrency model, error handling, logging / tracing / metrics, background tasks.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>US-008 — runtime behavior. Workspace is fully synchronous; no async runtime is present.</WCTX> -->
<!-- <CLOG>0.1.0: initial population.</CLOG> -->

# 8. Runtime Behavior

## 8.1 Binaries and startup

Two binaries are built from this workspace.

| Binary | Crate | `main` location | Startup | Exit-code semantics |
|---|---|---|---|---|
| `xtask` | `xtask` | `xtask/src/main.rs:124` (`fn main() -> Result<()>`) | Parses `Cli` via `clap::Parser::parse()`; matches on `cli.command`; dispatches to `audit::*`, `docs::*`, or `recipes::*`; wraps in `anyhow::Result` | Returns `Result<()>` — propagated by Cargo. Errors bubble up as anyhow errors with `.context(...)` from each call site. |
| `pipeline-probe` | `tui-vfx-probe` | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:23` (`fn main()`) | Calls `run()`; on `Err`, prints to stderr and `std::process::exit(1)` | Per `:25-29`: `if let Err(error) = run() { eprintln!("{error}"); std::process::exit(1); }` |

Library crates do not have `main` functions; they are consumed by the meta-crate `tui-vfx`, by sibling repos (gt-design, tui-vfx-recipes), or by the example targets registered against the meta-crate (`crates/tui-vfx/Cargo.toml:32-38`).

## 8.2 Main execution paths

### 8.2.1 `tui-vfx` library consumers

The canonical render path is `render_pipeline_with_spec` (chapter 3 F001). Per `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs:21-50`:

1. Caller constructs a `CompositionSpec` with shader layers, samplers, masks, filters, shadow, and playback timing.
2. Caller invokes `render_pipeline_with_spec(source, source_roles, dest, w, h, ox, oy, &spec, inspector)`.
3. The function lowers each `ShaderLayerSpec` through `ShaderWithRegion::try_from_v3_shader_family` (panics on lowering failure — REQ-002).
4. It builds a `CompositionOptions` with `CompositionPlaybackTiming::new(spec.t, spec.loop_t, spec.phase)` and forwards into `render_pipeline`.
5. `render_pipeline` (the `orc_` orchestrator at `orc_render_pipeline.rs:106`) drives the four-stage pipeline: Sampler → Mask → Shader → Filter, plus the shadow stage if configured.

### 8.2.2 `pipeline-probe` execution

Per `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:34-100`:

1. Parse CLI flags (manual `std::env::args` walk; OPT-007..OPT-016 in chapter 5).
2. Read `--input` JSON and deserialize into a `ProbeSceneSpec` (`:97`: `serde_json::from_str(&fs::read_to_string(input_path)?)?`).
3. Construct a `ProbeRequest` from flags.
4. Branch on flags: `--frames N` → `collect_timeline`; `--diff-to T` → `run_probe_diff`; otherwise → `run_probe`.
5. Optional: persist via `ProbeSqliteStore` if `--sqlite-query` is set.
6. Print results to stdout in the requested `--format`.

## 8.3 Async / concurrency model

**No async runtime.** A workspace-wide search (`grep -rn "async fn\|tokio::\|async_std\|smol::" crates/*/src/ xtask/src/`) returned **zero matches**. None of `tokio`, `async-std`, `smol`, `futures`, `pollster`, or any async-runtime crate appears in any `[dependencies]` block (chapter 2 §2.4.3 enumerates the full dep matrix; none of those crates are listed).

The workspace is fully synchronous. All public-API entry points are blocking sync functions. The `progress: f64` parameter on `ContentEffect::apply`, `progress: f32` on `render_shadow`, and `t: f64` / `loop_t: f64` / `phase` on `CompositionSpec` are caller-supplied per-frame values; the workspace does not own a clock or an animation loop — that lives at the consumer level.

## 8.4 Background tasks

A single `std::thread::sleep` call site exists in production (`crates/tui-vfx-content/src/pool/fnc_pick_index.rs:83` — `std::thread::sleep(std::time::Duration::from_nanos(1));`). The site is a one-nanosecond yield inside a pool-pick helper; this is the only `thread::*` call site in production code.

**No `thread::spawn`, `tokio::spawn`, or `select!` was observed in production code.** The workspace runs on the caller's thread.

## 8.5 Error handling

### 8.5.1 Error types

| Crate | Error type | Definition |
|---|---|---|
| `tui-vfx-content::cell_motion` | `CellMotionError` | `crates/tui-vfx-content/src/cell_motion/cls_cell_motion_spec.rs:96` (`pub enum CellMotionError`) |
| `tui-vfx-probe` | `ProbeError` | `crates/tui-vfx-probe/src/cls_probe_error.rs:10` (`pub enum ProbeError`); converts from `std::io::Error` at `:30-31` (`impl From<std::io::Error> for ProbeError`) |
| `tui-vfx-debug::inspection` | (uses `std::io::Error` directly for NDJSON round-trip) | `crates/tui-vfx-debug/src/inspection/cls_trace_report.rs:53,75` constructs `std::io::Error::new(std::io::ErrorKind::InvalidData, e)` |
| `tui-vfx-next` | `SurfaceDiagnostic` (event-based, not Result-based) | `crates/tui-vfx-next/src/diagnostic.rs` (file presence; chapter 3 F048) |
| `xtask` | `anyhow::Error` (uses `anyhow::Result` and `.context(...)` throughout) | 10+ files in `xtask/src/` import `anyhow::{Context, Result}` (e.g., `xtask/src/main.rs:10`, `recipes/mod.rs:6`, `audit/fnc_load_baseline.rs`, `docs/parse_toml.rs:6`) |

The two domain-error enums (`CellMotionError`, `ProbeError`) are hand-written `pub enum` types — no `thiserror` derive is in use at audit-time (a workspace-wide search for `use thiserror` returned only `xtask` `anyhow` imports, no `thiserror` in any crate). `eyre` does not appear either.

Library crates other than the two listed do not export error types at the crate level. They use `Option<T>` returns (e.g., `VfxBindable::evaluate` returns `Option<f32>` — chapter 3 F011), `bool` returns, or panic via `unwrap`/`expect` for invariant violations.

### 8.5.2 Panic / unwrap / expect

A workspace-wide grep for `panic!`, `unwrap()`, `expect(` (cumulative count per file, top sites):

| File | Match count | Notes |
|---|---:|---|
| `crates/tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs` | 61 | The 2186-LOC prepared-filter file; many sites are inside `#[cfg(test)]` modules. |
| `crates/tui-vfx-shadow/src/renderers/cls_solid.rs` | 32 | Shadow-renderer panics on invariant violations. |
| `crates/tui-vfx-content/src/mechanical/fnc_route_between.rs` | 27 | Mechanical-cycle routing — a private module. |
| `crates/tui-vfx-content/src/transformers/cls_split_flap.rs` | 20 | Split-flap transformer; many tests. |
| `crates/tui-vfx-core/src/bindable/test_cls_bindable.rs` | 19 | `tests/` file (test-only). |
| `xtask/src/docs/extract_rustdoc.rs` | 16 | `xtask` doc-extraction; expects on rustdoc-JSON shape invariants. |
| `crates/tui-vfx-content/src/mechanical/fnc_resolve_mechanical_cycle.rs` | 14 | Mechanical-cycle routing. |
| `crates/tui-vfx-style/src/models/v3/test_try_lower_v3_spatial_shader_family.rs` | 13 | `tests/` file (test-only). |
| `crates/tui-vfx-content/src/types/cls_content_effect.rs` | 13 | Content-effect dispatcher; some `expect` may live in `apply`-style methods. |
| `crates/tui-vfx-content/src/mechanical/fnc_overshoot_face.rs` | 10 | Mechanical-cycle helper. |

The render-pipeline driver itself uses `.expect()` once (`fnc_render_pipeline_with_spec.rs:25-28`) — this is the V3 lowering panic captured by REQ-002. A full audit of which sites are inside `#[cfg(test)]` modules vs production paths was out of scope for this chapter; chapter 9 records the unsafe-code count (2 sites, both in `test_alloc_budget.rs`).

### 8.5.3 Retry / fallback behavior

No retry loop or backoff was observed in production code. The font fallback (chapter 3 F019 — Intention 36) is the closest behavioral fallback: when a recipe-declared font cannot be resolved, the runtime renders through the Line 3×3 table and (per the intention) emits one warning to the trace surface. That fallback logic lives in the sibling `tui-vfx-recipes` validator and the per-call `FontRegistry::resolve` site; the in-tree resolution is wired through `DEFAULT_FONT_SENTINEL` (OPT-025).

## 8.6 Logging / tracing / metrics

### 8.6.1 The `tui-vfx-debug` logger

The custom `tui-vfx-debug` logger (chapter 3 F035) is the workspace's only logging surface. Per-module `LogLevel` configuration via `ModuleConfig`. Global singleton via `get_global_logger`. Output formatting via `colored 2.1`; timestamps via `chrono 0.4`. (`crates/tui-vfx-debug/Cargo.toml:20-22`.)

A workspace-wide grep for `tracing::` and `log::` returned **zero matches** in production code. The workspace does not use the `tracing` ecosystem or the `log` facade at audit-time.

### 8.6.2 Trace events (the inspection foundation)

`tui-vfx-debug::inspection` (chapter 3 F036) carries structured `TraceEvent` payloads through the pipeline when a `CompositorInspector` is registered. NDJSON round-trip is provided by `cls_trace_report.rs` (`use std::io::{BufRead, BufReader, Read, Result as IoResult, Write}` at `:20`). This is the structured-observability surface; the per-module debug logger above (8.6.1) is the unstructured-debug surface.

### 8.6.3 Metrics

**No metrics surface.** A workspace-wide grep for `metrics::` returned zero matches. The two criterion benches (`bench_emit_overhead`, `bench_full_trace_60fps`, `easing` — chapter 3 F047) produce HTML reports under `target/criterion/`, but those are bench artefacts, not runtime metrics.

## 8.7 Shutdown behavior

The two binaries exit on `Result` propagation (`xtask`) or via `std::process::exit(1)` (`pipeline-probe` — `:27`). Neither installs a `Ctrl-C` handler. The synchronous, single-threaded model means there are no background tasks to drain on shutdown.

## 8.8 External system calls

Production library crates make **no network calls and no shell-out calls**. The only external-system interactions are:

- `pipeline-probe` reads `--input` JSON via `std::fs::read_to_string` and writes to stdout / SQLite (chapter 7 §7.3, §7.5).
- `xtask` reads + writes files in `docs/generated/`, `docs/templates/`, and the workspace source tree (chapter 7 §7.2, §7.3).
- `tui-vfx-debug::inspection::cls_trace_report` reads + writes NDJSON (BufReader / BufWriter; chapter 7 §7.2, §7.3).

No subprocess (`Command::new`, `process::*`) call sites in production code (workspace-wide grep returned zero matches outside test fixtures).

## 8.9 Confidence

**High** for every claim in this chapter. The "no async runtime" / "no tracing-ecosystem usage" / "no metrics" / "no thread::spawn" claims are derived from empty workspace-wide grep result sets — those are valid evidence of absence per `pro/REVERSE-PRD.md`'s uncertainty rules.

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/08_runtime_behavior.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
