<!-- <FILE>docs/design/tui-vfx-pipeline-observability.md</FILE> - <DESC>Design spec for the inspection event bus, the pipeline-inspector tool, and the debugging-first cultural commitments that hold them together. Motivated by the focused_row_btop case study from 2026-04-26, where prose trace and ad-hoc probes concealed a role-map mismatch for 30+ minutes of investigation.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>2026-04-27: Unit A landed; align Units B and C with the pre/post-pass slot architecture decided in tui-vfx-effect-composition-model.md §11. Pre-pass and post-pass blocks become observable surfaces; PipelineStageKind grows two variants; Unit C explain subcommands learn the six-slot taxonomy.</WCTX> -->
<!-- <CLOG>0.3.0: align observability plan with rollout plan §A–§H. PipelineStageKind grows PrePass/PostPass; Shadow variant deprecates at Phase G. §9.1 emit sites add pre-pass / post-pass driver rows. §10 Unit C scope extends to pass-block investigations + slot occupancy. §11.2 adds Q12–Q14. §13 cultural commitments extend to PrePass/PostPass impls. §16 sequencing line updated. New §17 records the alignment with cross-refs to rollout plan.</CLOG> -->

# tui-vfx pipeline observability and inspector — design spec

## Status

v0.3.0: Unit A landed (see archived PRD/progress at `.omc/archive/2026-04-27-unit-a-pipeline-observability/`). Units B and C remain deferred work loops. This revision aligns Units B and C with the pre/post-pass slot architecture decided in `tui-vfx-effect-composition-model.md` §11 and rolled out per `tui-vfx-pre-post-pass-rollout-plan.md` §A–§H. Both unit scopes grow to cover the new surfaces; the bus design itself is unchanged. See §17 for the alignment summary.

v0.2.0: revised against the codebase. The eight v0.1.0 open questions are resolved (see §11). Three execution units identified in §12; Unit A (tui-vfx event extension) is the load-bearing single-PR work.

**v0.1.0 was written from steering docs without an `ofpf-defs` orientation pass and significantly understated shipped infrastructure.** The InspectionSink trait, TraceEvent enum (18 variants across lifecycle / resolution / composition / pipeline), CompositorInspector trait, InspectionSinkBridge, StageMask / TraceFilter / TraceSelector / TraceSink / TraceEnvelope / TraceEmitter / TraceFrameContext, the `bench_emit_overhead` and `bench_full_trace_60fps` criterion benches, the `tui-vfx-trace` CLI tool with `--select` / `--stages` / `--format ndjson|report`, and three production CompositorInspector impls (`ProbeInspector`, `StageInspector`, `TraceInspector`) all already exist as of 2026-04-26. v0.2.0 acknowledges that surface and narrows the spec to the genuinely-missing pieces: per-stage entry/exit/skip events, scope-evaluation evidence, a role-map-source discriminant, and a binding event pair.

This makes the spec an extension proposal against shipped infrastructure, not a from-scratch design.

## 1. North star

The compositor pipeline emits a typed event stream. Production rendering and every introspection tool subscribe to the same stream. There is no separate diagnostic render path. There is no stringly-typed log line at a stage boundary. There is one bus, several sinks, and one inspector tool that consumes the bus to answer concrete questions about a single render.

The cultural commitment that holds the bus together: every new stage, scope predicate, binding form, or producer-side cell tagger emits an event, and every bugfix whose investigation required reading the trace adds a new event type or field that would have made the bug obvious. The bus grows in lockstep with the rest of the codebase.

This document defines the observability primitive (`VfxObserver`), the event taxonomy, the sinks, the cost model, the inspector tool that consumes the bus, the plumbing changes required to install it, and the cultural commitments that govern its evolution.

## 2. Motivating case study — focused_row_btop, 2026-04-26

The full investigation lives in the conversation transcript. The compressed form:

A gt-design integration test asserted that the focused row in a `ContentShell::card` should differ in foreground color from its neighbors when a `focused_row_gradient` shader recipe with `selected_row_binding` is applied. The test failed with `Rgb(245, 248, 252) == Rgb(245, 248, 252)` for selected and neighbors.

Investigation steps that *should* have been one command each:

1. **"Did the shader run?"** Existing answer: read `pipeline-validator --probe` output. Tool reported `shader_count: 1, shader_effects: ["FocusedRowGradient#1"], modified_cells: 0`. The same `modified_cells: 0` appeared for the canonical-working `btop_focused_row_demo.json` — so "0" carried no signal.
2. **"Did the binding resolve?"** Existing answer: read the same probe output. Tool reported `binding_resolutions: [{status: fallback_static, fallback_value: 4}]`. CLI flag `--runtime-params-json` exists but is silently dropped on schema_v1 recipes; no warning.
3. **"Did the scope match any cells?"** No tool answers this. The probe reports stage presence in metadata; cell-level skip reasons are not exposed.
4. **"Why does the trace's `composition_effect_preview` show 228 fg_changes while the post-apply diff shows the surface unchanged?"** Answered only by reading `factory_trace_composition_preview` in `crates/gtd-factory/src/render/orc_render_pipeline.rs:1440` and noticing it calls `apply_composition` (geometric role inference), while the production path calls `apply_composition_with_roles` (explicit semantic-buffer roles). The diagnostic preview lied, by construction, because the two paths use different role-map sources.

Total time: 30+ minutes. The actual root cause is one sentence: the recipe's `Role(Text)` scope matches zero cells because `ContentShell::card` tags every cell with `Surface` → `RoleTag::Background`, and the shader correctly skips them all.

A bus that emits `ScopeEvaluated { matched: 0, skipped: 320, role_histogram: { Background: 320 } }` followed by `StageSkipped { reason: ScopeMatchedZeroCells { predicate: "Role(Text)", role_histogram: { Background: 320 } } }` answers the entire investigation in one event subscription.

## 3. Gap analysis — what exists today

This table was rewritten in v0.2.0 after `ofpf-defs` orientation. The v0.1.0 version was wrong about both `tui-vfx-trace` (not a crate; it's a CLI tool) and the absence of a typed taxonomy / subscription model.

### 3.1 What ships today (the shared event bus)

| Surface | Lives in | What it provides |
|---|---|---|
| `InspectionSink` trait | `crates/tui-vfx-debug/src/inspection/cls_inspection_sink.rs:20` | Object-safe `Send + Sync` sink: `fn report(&self, envelope: TraceEnvelope)`. Designed for `Arc<dyn InspectionSink>` sharing across the compositor bridge, scene composer, and lifecycle manager. |
| `TraceSink` | `cls_trace_sink.rs` | Concrete sink — bounded or unbounded, thread-safe, filter-aware. `accepts_any_stage()` short-circuit makes the no-op path allocation-free (proved by `bench_emit_overhead`). |
| `TraceEvent` enum | `cls_trace_event.rs:39` | Typed taxonomy with 18 variants across four sections: lifecycle (`LifecyclePhaseEntered`, `LifecyclePhaseTransition`, `LifecycleDismissed`, `LifecycleHeld`); resolution (`AssetResolved`, `ProceduralResolved`, `TokenResolved`, `RecipeBindingResolved`); composition (`LayerStarted`, `LayerCellPainted`, `LayerCompleted`, `LayerSkipped`); pipeline (`SamplerApplied`, `MaskChecked`, `ShaderApplied`, `FilterApplied`, `ShadowCellApplied`, `CellRendered`). |
| `TraceEnvelope`, `TraceEmitter`, `TraceFrameContext` | `cls_trace_envelope.rs`, `cls_trace_emitter.rs` | Envelope carries `event + frame_no + t_ms + recipe_id + seq_in_frame`. The emitter centralises stamping so cross-repo emit sites share one authority. |
| `StageMask` | `cls_stage_mask.rs` | Bitflags over four coarse stages (`LIFECYCLE`, `RESOLUTION`, `COMPOSITION`, `PIPELINE`) with `NONE` / `ALL` sentinels. The fast-path gate emit sites consult before constructing envelopes. |
| `TraceFilter`, `TraceSelector` | `cls_trace_filter.rs`, `cls_trace_selector.rs` | Selector grammar (`All`, `Cell`, `Rect`, `Role`, `Layer`, `Recipe`) + stage mask + frame range + time range. Composable at sink-time. |
| `TraceReport`, `TraceReportSummary` | `cls_trace_report.rs` | Snapshot/drain output with per-stage counts and dropped tally for bounded sinks. NDJSON round-trip. |
| `CompositorInspector` trait | `crates/tui-vfx-compositor/src/traits/pipeline_inspector.rs:20` | Per-stage callbacks plumbed into the production `render_pipeline` via `inspector: Option<&mut dyn CompositorInspector>`. Methods today: `on_sampler_applied`, `on_mask_checked`, `on_shader_applied`, `on_filter_applied`, `on_shadow_cell_applied`, `on_cell_rendered`. All have default empty bodies. |
| `InspectionSinkBridge` | `cls_inspection_sink_bridge.rs` | Adapter from `CompositorInspector` callbacks to `InspectionSink` reports. Lets any `Arc<dyn InspectionSink>` consume the production pipeline events. |
| `ProbeInspector` / `StageInspector` / `TraceInspector` | `tui-vfx-probe`, `tui-vfx-compositor` | Three production `CompositorInspector` impls already in use. |
| `bench_emit_overhead`, `bench_full_trace_60fps` | `crates/tui-vfx-debug/benches/` | Criterion benches that gate the noop and full-trace paths. |
| `tui-vfx-trace` (CLI tool) | `tui-vfx-recipes/tools/tui-vfx-trace` (README v0.2.0) | `cargo run -p tui-vfx-trace -- --recipe X --frames N --select role:border --stages composition,pipeline --format ndjson` — captures the event stream from a recipe render. |

### 3.2 What still falls short

| Tool | Where it falls short for the case-study class of bug |
|---|---|
| `TraceEvent` taxonomy | Per-cell stage events exist (`SamplerApplied`, `MaskChecked`, etc.) but **no per-stage entry/exit/skip events**. A stage that runs against a scope matching zero cells produces zero per-cell events — indistinguishable from "stage didn't run" or "stage ran successfully but had nothing to do." There is no `StageSkipped` with a reason, no `ScopeEvaluated` with a matched/skipped count, no `RoleMapMaterialized` with a source discriminant. |
| `tui-vfx-probe` | Pipeline metadata can report `shader_count: 1, modified_cells: 0` while production produces zero changes — the report is built from spec inspection plus a parallel render rather than from production execution evidence. The `0` carries no signal because canonical-working recipes can also report `0`. |
| `pipeline-validator --probe` | Inherits probe's all-or-nothing shape; `--runtime-params-json` is silently ignored on schema_v1; per-cell `trace: []` is empty when scope filters skip the cell, with no skip reason. |
| `GTD_TRACE_RENDER=1` (gt-design) | Stringly-typed prose lines whose field set drifts; the `composition_effect_preview` line is built from a parallel render that uses a different role-map source than production. |
| `factory_trace_composition_preview` (gt-design `crates/gtd-factory/src/render/orc_render_pipeline.rs:1440`, called from `:399`) | Re-renders via `apply_composition` (geometric role inference) while production calls `apply_composition_with_roles` (semantic-buffer roles). The diagnostic line lies, by construction, when the two role-map sources disagree — which is exactly when a bug like the focused_row_btop case study is happening. |

The bus exists. The taxonomy is too coarse for skip-evidence questions. The diagnostic re-render in gt-design is structurally divergent from production. The tooling under `tui-vfx-recipes/tools/` has not yet been migrated to consume the production event stream.

## 4. Philosophy

Three principles, in priority order. Each principle dictates a concrete commitment.

**4.1 Production and introspection are derived from the same execution.** The render that the user sees and the trace that the developer reads come from the same `render_pipeline` invocation. There is no parallel diagnostic re-render. There is no "preview" path that uses different role-map source, different scope evaluation, or different binding resolution from production. If a tool wants a diff, it diffs two cell snapshots taken from the same render via two subscribed sinks.

*Concrete commitment.* `factory_trace_composition_preview` is retired. Any future "what if I changed X" diagnostic runs the production pipeline twice with X mutated between runs, never with parallel divergent code paths.

**4.2 Skip events are first-class.** When a stage runs but produces no cell changes, the bus emits a structured event explaining why, with the data needed to reproduce the predicate evaluation. "Modified zero cells" is never a silent state. "Scope matched zero cells" is never a fact you have to derive.

*Concrete commitment.* `StageSkipped`, `ScopeEvaluated`, and `CellSkipped` are required event kinds; emitters that fail to produce them when applicable are bugs, audited at code review.

**4.3 Hooks earn their place by enabling the next investigation.** Adding an event kind, a sink, or an inspector subcommand requires a one-line note to the spec stating which past or anticipated investigation it serves. Speculative observability gets the same treatment as speculative features (Intention 24): no driver, no addition.

*Concrete commitment.* Every new event kind cites the bug or use case that introduced it; the appendix in §13 of this spec carries that ledger forward.

These principles are stricter than the current ad-hoc trace habits. The strictness is the point: today's tooling permitted a 30-minute investigation because it was permitted to drift.

## 5. The bus — extend the existing `InspectionSink` / `TraceEvent` surface

### 5.1 The trait — already shipped as `InspectionSink`

The trait this spec proposed at v0.1.0 (`VfxObserver`) ships today as `InspectionSink` in `crates/tui-vfx-debug/src/inspection/cls_inspection_sink.rs:20`. The shape is equivalent and stricter on the threading dimension:

```rust
// SHIPPED — crates/tui-vfx-debug/src/inspection/cls_inspection_sink.rs
pub trait InspectionSink: Send + Sync {
    fn report(&self, envelope: TraceEnvelope);
}
```

`render_pipeline` already takes `inspector: Option<&mut dyn CompositorInspector>` (`fnc_render_pipeline_with_spec.rs:30`). `InspectionSinkBridge` adapts `CompositorInspector` callbacks to `InspectionSink` reports, so any `Arc<dyn InspectionSink>` plugs into the production render path with no new plumbing.

**v0.2.0 decision:** do not introduce a parallel `VfxObserver` trait. The existing `InspectionSink` and `CompositorInspector` are extended in place. Naming stays unprefixed for now; the V3 cutover (Decision 4) bundles any `Vfx*` rename in one event (Intention 10).

### 5.2 Event taxonomy — extend the existing `TraceEvent` enum

The shipped `TraceEvent` (`cls_trace_event.rs:39`) already covers four sections (lifecycle / resolution / composition / pipeline) with 18 variants. The pipeline section today carries per-cell variants only: `SamplerApplied`, `MaskChecked`, `ShaderApplied`, `FilterApplied`, `ShadowCellApplied`, `CellRendered`. The genuinely-missing surface is **per-stage** evidence (entry/exit/skip) plus **scope-evaluation** evidence and a **role-map source** discriminant.

v0.2.0 adds five new variants in the existing Pipeline section, plus four new helper types. Naming follows the existing crate convention (no `Vfx*` prefix per Q2). All new variants and helpers carry `#[serde(default)]` on optional fields per Q5.

```rust
// PROPOSED — extend crates/tui-vfx-debug/src/inspection/cls_trace_event.rs
pub enum TraceEvent {
    // ── existing variants unchanged ──

    // ── new pipeline variants (Unit A) ─────────────────────────────────
    /// A pipeline stage began applying.
    StageEntered {
        kind: PipelineStageKind,
        step_id: u32,
        name: String,
        scope_summary: String,
    },
    /// A pipeline stage finished applying.
    StageFinished {
        kind: PipelineStageKind,
        step_id: u32,
        cells_modified: u32,
        elapsed_ns: u64,
    },
    /// A pipeline stage was skipped without iterating cells.
    StageSkipped {
        kind: PipelineStageKind,
        step_id: u32,
        reason: PipelineSkipReason,
    },
    /// Scope predicate evaluation summary for one stage application.
    ScopeEvaluated {
        step_id: u32,
        matched: u32,
        skipped: u32,
        role_histogram: RoleHistogram,
    },
    /// Role map became available to the pipeline; carries the source
    /// (inferred geometrically vs explicit producer-tagged vs injected).
    RoleMapMaterialized {
        source: RoleMapSource,
        histogram: RoleHistogram,
    },
}

pub enum PipelineStageKind {
    // Pre/post-pass slots — listed in pipeline-execution order, top to bottom.
    PrePass,        // added in rollout Phase C; emitted by the new pre-pass driver in Phase B
    Sampler,
    Mask,
    Shader,
    Filter,
    PostPass,       // added in rollout Phase C; emitted by the new post-pass driver in Phase B
    Shadow,         // deprecation alias during the migration window; removed in rollout Phase G
}

pub enum PipelineSkipReason {
    EmptyArea,
    ScopeMatchedZeroCells { predicate: String, role_histogram: RoleHistogram },
    DisabledByPolicy { policy: String },
    BudgetExceeded { budget_ns: u64 },
}

pub struct RoleHistogram {
    pub background: u32,
    pub text: u32,
    pub border: u32,
    pub indicator: u32,
    pub highlight: u32,
}

pub enum RoleMapSource {
    Inferred,
    ExplicitFromProducer { producer: String },
    Injected,
}
```

`step_id` is a 1-based counter local to one pipeline run; the join key for inspector queries. Existing per-cell variants do not yet carry `step_id` — adding it is a future additive change driven by a real cross-stage join investigation.

The following variants from the v0.1.0 sketch are deferred until an investigation drives them:

- `PipelineStarted` / `PipelineFinished` — partly redundant with the existing `LayerStarted` / `LayerCompleted`; revisit when a multi-layer composition bug needs a top-level wrapper.
- `CellTransformed` / `CellSkipped` per-cell variants with `cause` discriminants — the existing `*Applied` per-cell variants cover the affirmative case; a `cause` discriminant earns its place when a cross-stage causation question requires it.
- `BindingRequested` / `BindingResolved` — partially covered by the existing `RecipeBindingResolved`; the parameter-binding pair lands as part of Unit C when the gt-design `--bindings` investigation needs it.

This conforms to §4.3: hooks earn their place by enabling the next investigation.

### 5.3 Field-naming conventions

Field names are stable across event kinds and across the JSON serialization. The conventions:

- `step_id` — `u32`, never the human name.
- `name` — human-readable label (`"FocusedRowGradient"`, `"FadeIn"`).
- `kind` — `PipelineStageKind` discriminator.
- `cells_modified`, `cells_skipped` — `u32` counts, never `usize`.
- `elapsed_ns` — `u64` nanoseconds, never `Duration` (cheap to serialize).
- `predicate` — `String` summary of the scope predicate (machine-stable, e.g., `"Role(Text)"` or `"And(RowRange,Channel(Background))"`). `String` (not `&'static str`) so it round-trips through serde.
- `histogram` — `RoleHistogram` struct, never an opaque map.

Stable field names make `--probe-sqlite-query` (and its bus-aware successor) tractable.

## 6. Sinks

Most of the v0.1.0 sink list ships today under different names. The home crate is `tui-vfx-debug::inspection`. Consumers compose sinks via `Arc<dyn InspectionSink>`.

| Sink (this spec) | Status | Shipped name |
|---|---|---|
| Noop default | shipped | `TraceFilter::accepts_any_stage()` short-circuit on `TraceSink` makes the no-op path allocation-free without a separate `Noop` sink |
| JSONL output | shipped | `TraceReport::to_ndjson()` plus the `tui-vfx-trace` CLI's `--format ndjson` |
| Filtering wrapper | shipped | `TraceFilter { selectors, stages, frames, time_ms }` applied at sink-time |
| Bounded ring | shipped | `TraceSink::with_capacity(filter, n)` — VecDeque with dropped counter |
| Counting / summary | shipped | `TraceReportSummary::of(envelopes)` — per-stage counts on the report |
| Composite | partial | a sink can wrap an `Arc<dyn InspectionSink>`; an explicit `CompositeSink` could land if a real fan-out use case earns it |
| SQLite query | not shipped | opt-in addition under Unit C; default stays NDJSON per Q6 |
| **Asserting (test-only)** | **load-bearing for Unit A** | new in this spec — `AssertingInspector` in `tui-vfx-debug::inspection`; rejects forbidden events with a clear panic message. The regression test for today's bug class installs it. |

Composite/SQLite sinks earn their place when an investigation drives them. The asserting sink is in Unit A because it is the mechanical gate behind acceptance criterion 3 (skip events are total).

## 7. Cost model — runtime filter, not Cargo feature

**Q4 decision:** runtime filter only via the existing `TraceSink::accepts_any_stage()` short-circuit. No `vfx-cell-events` Cargo feature.

Reasoning: a Cargo feature is friction at the moment of investigation. Forgetting `--features vfx-cell-events` on a `cargo run -p tui-vfx-trace` invocation produces an empty stream, which a reader interprets as "tool is broken" and falls back to source. Runtime filtering avoids that failure mode.

The shipped runtime gate already gives near-zero cost when no sink is installed:

- Default `inspector: None` on `render_pipeline_with_spec` skips every emit site.
- A sink with `TraceFilter::accepts_any_stage() == false` drops events without envelope construction (proved by `bench_emit_overhead`).
- The `bench_full_trace_60fps` criterion bench enforces ≤5% Tier-A regression on the steady-state path. Per-cell costs are measured separately and budget the inspector-mode path.

Per-stage events (the new variants in §5.2) add at most one envelope per stage per render — single-digit nanoseconds with the noop-filter sink. Per-cell events (`SamplerApplied`, `MaskChecked`, etc.) already pay per-cell cost when a non-empty filter is installed; the new `ScopeEvaluated` event amortises one histogram per stage rather than per cell.

If `bench_full_trace_60fps` shows a regression beyond budget after Unit A lands, the response is to push the gate down into the emit site (e.g., elide `ScopeEvaluated` when the inspector has no filter for it), not to introduce a compile-time feature.

## 8. The inspector tool — extend `tui-vfx-trace`, do not fork

**Decision:** the v0.1.0 sketch proposed a new `vfx-inspect` workspace binary. v0.2.0 supersedes that — `tui-vfx-trace` (`tui-vfx-recipes/tools/tui-vfx-trace`, README v0.2.0) already takes `--recipe / --frames / --select / --stages / --format ndjson|report` against the same selector grammar the proposed `vfx-inspect run` would have used. Forking a second binary fragments discoverability (Intention 13) and re-implements the existing CLI surface for no gain.

Unit C extends `tui-vfx-trace` with the canned-investigation subcommands the v0.1.0 spec named, in the same crate. The `cargo run -p tui-vfx-trace -- ...` invocation pattern stays unchanged.

### 8.1 Command surface (Unit C scope)

```
# already shipped — unchanged
tui-vfx-trace --recipe X --frames N --select role:border --stages composition,pipeline --format ndjson

# Unit C adds:
tui-vfx-trace explain <RECIPE>           # Pre-built investigations (Unit C)
  [--empty-changes]                      # Why did this recipe produce zero cell changes?
  [--cell <X,Y>]                         # Show every event that touched cell (X,Y)
  [--bindings]                           # Walk every binding request and its resolution
  [--scope <STEP_ID>]                    # Walk the scope predicate, report match/skip per role

tui-vfx-trace record <RECIPE> --out <TAPE>  # Record full event tape (Unit C)
tui-vfx-trace replay <TAPE>                 # Replay against the same query surface (Unit C)

tui-vfx-trace diff <RECIPE_A> <RECIPE_B>    # Run both, diff event streams (Unit C, optional)
  [--align-by step_id|name|cell]

tui-vfx-trace bisect <RECIPE>               # Permute role maps and find the smallest map that makes the recipe fire (Unit C, optional)
  [--target-step <STEP_ID>]
```

The existing `--bindings` story is covered by the recipe runtime; v0.2.0 does not need a separate CLI flag at the inspector layer until the binding event pair lands (deferred per §5.2).

`--source-roles` and `--apply-only-step` from the v0.1.0 sketch are deferred until an investigation drives them.

### 8.2 Pre-built investigations (Unit C)

The `explain` subcommand will carry a small library of investigations that wrap common observer compositions. Each investigation is one Rust function that builds a sink + filter and pretty-prints the resulting report.

`tui-vfx-trace explain <RECIPE> --empty-changes` is the case study from §2. Sketch (against the shipped `InspectionSink` surface, after Unit A's variants land):

```rust
fn explain_empty_changes(recipe_path: &Path) -> Report {
    use tui_vfx_debug::inspection::{TraceFilter, TraceSink, StageMask};
    let filter = TraceFilter { stages: StageMask::PIPELINE, ..TraceFilter::accept_all() };
    let sink = Arc::new(TraceSink::new(filter));
    render_recipe_with_sink(recipe_path, sink.clone());
    let report = sink.snapshot();
    pretty_print_skip_chain(&report)  // prints StageSkipped + ScopeEvaluated + RoleMapMaterialized
}
```

Output for today's bug:

```
Render summary
  recipe:           focused_row_btop
  pipeline:         shader=1, mask=0, filter=0, sampler=0
  cells modified:   0  (expected: > 0 since pipeline has shader)

Role map
  source:           ExplicitFromProducer { producer: "ContentShell::card" }
  histogram:        background=320 text=0 border=0 indicator=0 highlight=0

Stage skip chain
  step_id=0  Shader[FocusedRowGradient]
    scope:        Role(Text)
    matched:      0
    skipped:      320
    skip reason:  ScopeMatchedZeroCells {
                    predicate: "Role(Text)",
                    role_histogram: { background: 320, text: 0, ... }
                  }

Diagnosis
  Scope `Role(Text)` matched zero cells because the role map produced
  by ContentShell::card has zero `text` cells. Either:
    (a) the recipe scope should be content-based (e.g. Content(Text))
        — see V3 canonical scope `{"kind":"content","value":"text"}`
    (b) the producer should tag inner content cells with
        SemanticRole::Content (lowers to RoleTag::Text)
```

The diagnosis text is hard-coded heuristic for the `ScopeMatchedZeroCells` case. Other reasons (`BudgetExceeded`, `DisabledByPolicy`, `EmptyArea`) get their own pretty-printers. The library grows as new skip reasons are added.

### 8.3 Tape format

Recorded sessions are append-only JSONL where each line is one `VfxEvent` serialization. Tape files are the wire format for `vfx-inspect replay`, the reproducer attached to a bug ticket, and the input for downstream tools (CI golden-event diffs, post-mortem analysis).

Tape format is versioned (`vfx_inspect_tape.v1`). Consumers MUST tolerate unknown event kinds (forward-compatible); producers MUST stamp the schema version in the first event of the tape.

### 8.4 Existing tools as bus consumers (post-Unit-B/C)

After Units B and C land:

- `pipeline-validator --probe` installs an `InspectionSinkBridge` plus the existing `TraceSink`, builds its report shape from the event stream.
- `recipe-probe` installs the same — differs only in CLI ergonomics.
- `tui-vfx-trace` is the canonical inspector entry point with `explain` subcommands for canned investigations.
- GTD-side `GTD_TRACE_RENDER=1` (Unit B) installs an `InspectionSinkBridge` at the production render site and formats events into the legacy prose-line shape for muscle-memory compatibility, with a stderr deprecation notice.

The diagnostic re-render in `factory_trace_composition_preview` is deleted in Unit B (Q8 decision). There is one render, the bus carries the evidence, any diff is produced by subscribing two sinks to the same render — never by re-running the pipeline with divergent role-map sources.

## 9. Plumbing — where new hooks land (Unit A)

Existing per-cell call sites already exist (`on_sampler_applied`, `on_mask_checked`, `on_shader_applied`, `on_filter_applied`, `on_shadow_cell_applied`, `on_cell_rendered` in `render_loop_inspected` and `apply_shaders_inspected`). Unit A adds per-stage and scope-evaluation emit sites at the boundaries of those existing loops.

### 9.1 Compositor (`crates/tui-vfx-compositor`) — Unit A

Sites are listed in pipeline-execution order, top to bottom (pre-pass first, four element stages, post-pass last). Pre/post-pass rows land with rollout Phase C and ship together with the Phase B driver.

| Location | Events emitted |
|---|---|
| Each pre-pass entry in the new pre-pass driver (rollout Phase B) | `on_stage_entered { kind: PrePass }` then `on_stage_finished` (or `on_stage_skipped` if `EmptyArea` / `DisabledByPolicy`) |
| Each Sampler iteration in `render_loop_inspected` (before/after) | `on_stage_entered { kind: Sampler }` then `on_stage_finished` (or `on_stage_skipped` if zero-cell scope) |
| Each Mask iteration | same with `kind: Mask` |
| Each Shader iteration in `apply_shaders_inspected` | same with `kind: Shader` |
| Each Filter iteration in the existing filter loop | same with `kind: Filter` |
| Each post-pass entry in the new post-pass driver (rollout Phase B) | `on_stage_entered { kind: PostPass }` then `on_stage_finished` (or `on_stage_skipped` if the dest-aware blend short-circuits to a no-op) |
| Shadow stage in legacy `render_pipeline_with_shadow` | `on_stage_entered { kind: Shadow }` (alias) until rollout Phase G removes the legacy fork; thereafter shadow emits as `kind: PrePass` from the new driver |
| Scope predicate evaluator (one per stage application — element stages only) | `on_scope_evaluated` summarising matched/skipped counts and role histogram |
| Role-map availability at render entry | `on_role_map_materialized` (initial Unit A emission uses `RoleMapSource::Injected`; gt-design upgrades to `ExplicitFromProducer` in Unit B) |

`step_id` is a 1-based per-render counter assigned at iteration time, walking the pipeline in execution order: pre-passes increment first, then element stages, then post-passes. Pre/post-passes are whole-canvas operations and do not emit `ScopeEvaluated` (no per-cell scope predicate); they declare canvas extent on `StageEntered` and `cells_modified` on `StageFinished`.

### 9.2 Recipe runtime (`crates/tui-vfx-recipes`) — deferred

`BindingRequested` / `BindingResolved` events for parameter bindings (separate from the existing `RecipeBindingResolved` for selector→recipe binding) land when the gt-design `--bindings` investigation in Unit C drives them. The existing `RecipeBindingResolved` covers the selector use case today.

### 9.3 Producer side (`crates/gtd-factory`) — Unit B

| Location | Events emitted (Unit B) |
|---|---|
| `apply_composition_with_roles` entry | `on_role_map_materialized` with `source: ExplicitFromProducer { producer: "<widget_name>" }` |
| `infer_source_roles` (legacy fallback) | `on_role_map_materialized` with `source: Inferred` |
| `factory_trace_composition_preview` | DELETED (Q8). Replace `GTD_TRACE_RENDER=1` with an InspectionSinkBridge installed at the production render site, formatting events to the legacy prose line shape with a deprecation notice. |

### 9.4 Element-stage and pass impls (across crates)

Each implementation gets one `on_stage_entered`/`on_stage_finished` pair from its call site in the compositor — implementations themselves do not emit. Per-cell `*Applied` callbacks already exist for element stages and continue to fire as today. Pre/post-pass impls emit only the stage-level pair (no per-cell pass-cell-applied event in v0.3.0; if a future investigation needs one, it lands additively per §11.2 Q12).

Cultural commitment §13.1 extends to `PrePass` and `PostPass` impls: every new pass primitive ships with one peer test asserting the expected stage-level emit shape (kind / step_id ordering / canvas extent on entered / `cells_modified` on finished).

## 10. Acceptance criteria

Per-unit acceptance. v0.2.0 collapses the v0.1.0 list to what each unit owns.

### Unit A (this spec's load-bearing scope)

A1. **New event variants ship in `TraceEvent`.** `StageEntered`, `StageFinished`, `StageSkipped`, `ScopeEvaluated`, `RoleMapMaterialized` are present with the field shapes from §5.2. Each has a serde JSON round-trip test.

A2. **`CompositorInspector` carries the new callbacks.** `on_stage_entered`, `on_stage_finished`, `on_stage_skipped`, `on_scope_evaluated` exist with default empty bodies; existing impls (`ProbeInspector`, `StageInspector`, `TraceInspector`, `InspectionSinkBridge`) compile unchanged.

A3. **`InspectionSinkBridge` forwards the new callbacks.** A bridge round-trip test confirms each new callback produces a matching `TraceEvent` envelope in the underlying `TraceSink`.

A4. **The pipeline emits at the right sites.** `render_loop_inspected` and `apply_shaders_inspected` emit per-stage entry/exit/skip plus per-application scope evaluation. Sum of `matched + skipped` on `ScopeEvaluated` equals area cell count. `step_id` is stable across one render and starts at 1.

A5. **Skip events are total for the case study scope.** An integration test installs `InspectionSinkBridge` over a Role(Text)-scoped shader applied to an all-Background `RoleMap` and asserts a `StageSkipped { ScopeMatchedZeroCells }` event with `role_histogram.background > 0` and `text == 0` is captured. (Generalising to "every Filter/Mask/Sampler/StyleShader impl" is a Unit A.5 follow-up that requires a per-impl macro; deferred until the new variants are in use.)

A6. **`AssertingInspector` exists and rejects forbidden events.** Unit tests confirm: (a) emitting a non-matching event does not panic; (b) emitting a matching event panics with a clear message that names the variant.

A7. **Bench budget holds.** `bench_emit_overhead` and `bench_full_trace_60fps` show ≤ 5% Tier-A regression with the noop-filter sink. Cell-event regression budget recorded for inspector mode.

A8. **Cross-repo audit clean.** Per Intention 41, additive changes do not break tui-vfx-recipes / mixed-signals / gt-design.

### Unit B (gt-design wiring)

B1. **Single-source rule.** `factory_trace_composition_preview` is deleted; `GTD_TRACE_RENDER=1` reads from the production InspectionSinkBridge stream (same env var, same prose output, no parallel render).

B2. **Producer roles are explicit.** `apply_composition_with_roles` emits `RoleMapMaterialized` with `source: ExplicitFromProducer`. The focused_row_btop scenario, when re-run with the corrected role tagging, produces a clean trace (no `ScopeMatchedZeroCells` for the fixed recipe).

### Unit C (CLI subcommands and observability tooling)

C1. **`tui-vfx-trace explain --empty-changes <recipe>`** produces an output that names the cause without the operator reading any source file. Coverage includes the four element-stage skip reasons (role-map mismatch, scope-matched-zero, disabled-by-policy, budget-exceeded) AND pre/post-pass empty cases (pre-pass `EmptyArea`, post-pass dest-aware blend short-circuit). Output includes the role histogram, the skip reason, and pass occupancy if the recipe declares pre/post-passes.

C2. **Tape format stable within v1.** A recorded tape replayed through `tui-vfx-trace replay` produces the same query results as the live render that recorded it. Round-trip test diffs query outputs. New event kinds and new fields land additively with `#[serde(default)]`. PipelineStageKind growing to include PrePass/PostPass (rollout Phase C) is itself an additive change covered by this rule.

C3. **`pipeline-validator --probe` and `recipe-probe`** build their reports from the production event stream rather than parallel render paths. Reports surface slot occupancy (which of the six slots a recipe populates) using the contract-discovery surface from rollout Phase D.

C4. **New investigation: `tui-vfx-trace explain --pass <name>`.** For a recipe that declares a pre/post-pass, this subcommand reports canvas extent, blend mode, `cells_modified` on the writeback, and (for pre-passes that interact with masks) whether the element-stage mask gated the writeback band. Driver: same diagnosis discipline as §2 case study, applied to the pass surfaces that ship with rollout Phase H and beyond.

### Cultural enforcement (continuous, all units)

Any new `Filter`/`Mask`/`Sampler`/`StyleShader` impl, scope variant, or producer-side semantic-buffer tagger added after Unit A lands ships with at least one event-stream test that asserts the expected `StageEntered`/`ScopeEvaluated`/`StageSkipped` shape. The CI gate fails the PR if the test is missing — the gate itself is a Unit A.5 follow-up that requires the per-impl test macro.

## 11. Decisions captured (v0.2.0) and questions still open

The eight v0.1.0 questions are resolved. Any future open questions land below the decision log.

### 11.1 Decisions

**Q1 — extend the existing `TraceEvent` enum; do not build a parallel `VfxEvent`.** The shipped enum (`cls_trace_event.rs:39`) already has 18 variants covering lifecycle / resolution / composition / pipeline. The five missing per-stage and scope-evidence variants slot into the Pipeline section. Existing tooling (`tui-vfx-trace` CLI, `TraceFilter`, `StageMask`, NDJSON wire format) consumes them for free. Forking would fragment Intention 23 (rule of three) for no investigator gain.

**Q2 — leave new types unprefixed; bundle the `Vfx*` rename into V3's Decision-4 cutover.** The shipped types are unprefixed (`InspectionSink`, `TraceSink`, `TraceEvent`, `StageMask`). A rename today is pure Intention 24 cost (every callsite touched, zero behavior change). Intention 10 explicitly licenses major-version naming resets — V3 is that moment.

**Q3 — `InspectionSink: Send + Sync` is already shipped.** Object-safe, designed for `Arc<dyn>` sharing across the bridge, scene composer, and lifecycle manager. No change.

**Q4 — runtime filter only; no `vfx-cell-events` Cargo feature.** Friction at the moment of investigation is the worst CLI failure mode. The shipped `TraceSink::accepts_any_stage()` short-circuit gives near-zero cost when no sink is installed. `bench_emit_overhead` and `bench_full_trace_60fps` are the enforcement; if they regress past 5% the response is a smaller emit-site gate, not a Cargo feature.

**Q5 — additive-only within v1, `#[serde(default)]` on every new field.** Producer side stays strict (`deny_unknown_fields` where present); consumer/replay (Unit C) tolerates unknown variants. Cheap discipline; high value for tape-as-bug-reproducer.

**Q6 — SQLite opt-in (`--format sqlite`); NDJSON stays default.** The 90% case is `… | jq` or `… | grep`. SQLite earns its place when an investigation needs cross-event joins. Lands in Unit C alongside `explain`.

**Q7 — gt-design only at v0.1.0 of the producer coverage; per-producer checklist for future ones.** No speculative enumeration of hypothetical hooks for movie-player / SVG exporter. Add hooks when the producer lands (Intention 24).

**Q8 — delete `factory_trace_composition_preview` now; keep `GTD_TRACE_RENDER=1` working but swap its guts.** The env var is muscle memory; preserve it. Re-implement to install an `InspectionSinkBridge` at the production render site and format events into the legacy prose line shape with a stderr deprecation notice. No more divergent role-map sources.

### 11.2 Questions still open after v0.2.0

**Q9 (Unit A scope).** Should `step_id` be added to existing per-cell variants (`SamplerApplied`, `MaskChecked`, `ShaderApplied`, `FilterApplied`, `ShadowCellApplied`) so cross-stage joins on a cell can be trivially expressed in `--format sqlite`? Recommended default: **defer** until a cross-stage join investigation needs it. Adding `step_id` retroactively is additive with `#[serde(default = "u32::MAX")]`.

**Q10 (Unit B scope).** Should the gt-design `RoleMapMaterialized` emit at the producer entry point also include the widget identity (e.g., `widget_path: Vec<String>`) so a multi-widget render can be filtered to one widget's role map? Recommended default: **yes, when Unit B lands**, because the focused_row_btop case study had a `ContentShell::card` producing the role map and the widget identity was load-bearing for diagnosis.

**Q11 (Unit C scope).** Should `tui-vfx-trace record` and `tui-vfx-trace replay` round-trip the `RoleHistogram` and `PipelineSkipReason` enums by tagged union JSON, or by an internal binary format? Recommended default: **tagged union JSON** — the existing `TraceReport::to_ndjson()` is JSON; consistency over compactness; tapes are diff-friendly.

**Q12 (rollout Phase C / Unit C scope).** Should pre/post-pass blocks emit a per-cell event analogous to the existing `ShadowCellApplied`, or stay stage-level only? Recommended default: **stage-level only at v0.3.0**, lift if a real investigation needs per-cell pass evidence (e.g. a glow blend mode subtly miscomposing one cell band). Per-cell pre/post events would multiply emit volume on whole-canvas passes; the bench gate (§7) would force a Tier-B feature flag if added speculatively. Per §4.3 (hooks earn their place), defer until a bug class drives it.

**Q13 (rollout Phase C scope).** Where does `CanvasExtent` (the pre/post-pass canvas description: `Element` vs. `Extruded { extra_w, extra_h, offset_x, offset_y }`) surface in the event stream? Recommended default: **on `StageEntered` only, when `kind ∈ {PrePass, PostPass}`**, as an optional field with `#[serde(default)]`. Element-stage events leave it `None`. Authoring tools and `--pass <name>` investigations consume it; per-stage taxonomy stays homogeneous on the wire.

**Q14 (rollout Phase D / Unit C scope).** Slot occupancy reporting: should the contract-discovery surface emit one event per recipe-load (`SlotOccupancyResolved { pre_passes: N, sampler: bool, mask: bool, shader: bool, filter: bool, post_passes: N }`), or surface only on demand via the validator API? Recommended default: **on-demand via the validator API**; recipe load already emits `RecipeBindingResolved` for the binding surface, and slot occupancy is derived from the same parsed structure. An on-load event would double-emit. The `explain --pass` and `--bindings` investigations call the API and pretty-print as part of their report.

## 12. Migration plan — three units

The seven v0.1.0 phases collapse into three units once the shipped surface is acknowledged. Each unit is one PR-sized scope. Units land in order; A is the only one with no cross-repo dependencies.

| Unit | Scope | Repos touched | Gate |
|---|---|---|---|
| **Unit A** (this Ralph run) | Add the five new `TraceEvent` variants (`StageEntered`, `StageFinished`, `StageSkipped`, `ScopeEvaluated`, `RoleMapMaterialized`) and supporting helper types (`PipelineStageKind`, `PipelineSkipReason`, `RoleHistogram`, `RoleMapSource`). Extend `CompositorInspector` with four matching callbacks (default empty bodies). Extend `InspectionSinkBridge` to forward them. Wire emit sites in `render_loop_inspected` / `apply_shaders_inspected`. Add `AssertingInspector` test sink. Land the focused_row_btop regression test that asserts `ScopeMatchedZeroCells` fires. | tui-vfx | A1–A8 of §10 |
| **Unit B** (follow-up Ralph run) | Install `InspectionSinkBridge` at gt-design's production render entry. Emit `RoleMapMaterialized` from `apply_composition_with_roles` and `infer_source_roles`. Delete `factory_trace_composition_preview`. Re-implement `GTD_TRACE_RENDER=1` to read the production stream and format to the legacy prose line shape with a stderr deprecation notice. | gt-design (+ Intention 41 four-repo audit) | B1–B2 of §10 |
| **Unit C** (follow-up Ralph run) | Extend `tui-vfx-trace` with `explain --empty-changes`, `explain --cell`, `explain --bindings`, `record`, `replay`. Migrate `pipeline-validator --probe` and `recipe-probe` to consume the production event stream. Add `--format sqlite` option (per Q6). Add the parameter `BindingRequested`/`BindingResolved` event pair if the bindings investigation needs it. | tui-vfx-recipes | C1–C3 of §10 |

Unit A is load-bearing: after it lands, the new events exist and any consumer can subscribe through the existing bridge. Units B and C compound the value but are independently schedulable.

## 13. Cultural commitments

The bus only stays useful if the codebase grows it. The commitments below are concrete enough to enforce at code review.

**13.1 New stage or pass implementation.** Every new `impl Filter`, `impl Mask`, `impl Sampler`, `impl StyleShader`, `impl PrePass`, or `impl PostPass` ships with at least one peer test in its `test_*.rs` file that:
- installs an `AssertingInspector` configured to expect the right `StageEntered` / `StageFinished` / `ScopeEvaluated` shape (element stages) or `StageEntered` / `StageFinished` with the expected `CanvasExtent` and `cells_modified` (pre/post passes),
- runs the stage or pass against a known input,
- asserts the assertion passes.

The test prevents silent emission drift and documents the expected observer footprint. Pre/post passes additionally assert `step_id` ordering — pre-passes come before element stages, post-passes after — to catch driver-ordering regressions early.

**13.2 New scope variant.** Adding a variant to the `VfxScope` enum requires updating the predicate-summary string used by `ScopeEvaluated` and `ScopeMatchedZeroCells`. The CI step that validates scope-summary coverage fails the PR if the new variant has no summary.

**13.3 New binding form.** New `Bindable*` types (or the consolidated `Bindable<T>` per sweep finding 1.2.A) emit `BindingRequested`/`BindingResolved` events through the same path as the existing `BindableU16`/`BindableString`/`BindableValue`. The peer test asserts the event shape.

**13.4 Bug fix triggered by trace reading.** Every bug whose investigation involved reading the trace (or absence of one) adds at minimum:
- a regression test that installs `VfxAssertObserver` to refuse the bug-shaped event (e.g., today's bug refuses `StageSkipped { ScopeMatchedZeroCells }` for the fixed recipe), and
- an entry in §13.6 of this spec naming the bug, the event that surfaced it, and the prevention rule.

**13.5 Producer-side widgets.** Any new widget in `gt-design` (or any other producer) that builds a `SemanticBuffer` is responsible for tagging cells with their honest `SemanticRole`. The Stage-C contract (per `crates/gtd-factory/src/render/orc_render_pipeline.rs:5` v0.12.0 CLOG) is binding: explicit roles must be at least as rich as geometric inference. The peer test inspects `RoleMapMaterialized.histogram` and asserts at least the expected role kinds are populated.

**13.6 Bug-driven event ledger.** This section is appended to as each new event kind earns its place by an investigation. v0.2.0 entries (carried from v0.1.0; landing in Unit A unless marked):

- `StageEntered` / `StageFinished` — driver: focused_row_btop bug, 2026-04-26. Without per-stage entry/exit, a stage that runs against a zero-cell scope is indistinguishable from a stage that did not run.
- `StageSkipped { ScopeMatchedZeroCells }` — driver: same bug. Without an explicit skip reason, a recipe that produces zero cell changes is indistinguishable from a recipe running correctly with a no-op input.
- `ScopeEvaluated { matched, skipped, role_histogram }` — driver: same bug. The histogram makes the role mismatch (background=320, text=0) visible in one event.
- `RoleMapMaterialized { source }` — driver: same bug. The diagnostic preview lied because production and trace used different role-map sources; the discriminant makes the source explicit. Lands in Unit A with `Injected`; gt-design upgrades to `ExplicitFromProducer` in Unit B.
- `BindingRequested` / `BindingResolved` (parameter bindings) — driver: same bug; the recipe's `--runtime-params-json` was silently dropped on schema_v1. Lands in Unit C with the gt-design `--bindings` investigation.

Future bugs append here; the ledger is the spec's audit trail for §4.3 (hooks earn their place).

## 14. Risks and trade-offs

**Coupling growth.** Every stage now depends on the observer trait. Mitigation: the trait lives in `tui-vfx-types` (already a hub), the dependency edge already exists. `VfxNoopObserver` keeps zero-cost defaults. Per Intention 24, the abstraction earns its place by today's bug + the future debug needs the spec enumerates.

**Hot-path regression.** Tier-A events on every stage entry/exit are not free, even with inlining. Mitigation: bench gate (acceptance criterion 4) catches >5% regression. Tier B is feature-gated.

**Observer ordering coupling.** A `CompositeObserver` runs sinks in registration order. If a sink mutates state another sink reads, ordering matters. Mitigation: at v0.1.0 all sinks are read-only consumers; `VfxAssertObserver` is the only sink with side effects beyond writing output, and its semantics are independent of order. Document the ordering rule in the trait rustdoc.

**Cultural fatigue.** Requiring an event-shape test for every new stage is friction. Mitigation: provide a test helper macro (`vfx_observer_smoke!(MyShader, expected_events)`) that collapses the boilerplate to one line. The cost of writing the test is bounded; the cost of a future bug like today's is not.

**Tape format lock-in.** Once `vfx-inspect record` is in use, tape consumers depend on the format. Mitigation: §11 Q5 enforces additive-only changes within v1, with `#[serde(default)]` on every field. Breaking changes require a v2 namespace.

**Inspector tool sprawl.** `vfx-inspect` could grow into a kitchen-sink CLI. Mitigation: every subcommand needs a one-line note in the spec citing the investigation it serves (per §4.3). Subcommands without a driver are rejected.

**Sink crate placement.** §11 Q1 leaves the sinks in `tui-vfx-trace`, but the JSONL/SQLite sinks pull in serde_json and rusqlite, which are heavy for a crate that consumers may not want. Mitigation: each sink behind a Cargo feature on `tui-vfx-trace`; no consumer pays for unused sinks.

## 15. Companion documents

Bound to this spec; updated together when the architecture evolves:

- `docs/design/tui-vfx-pipeline-observability-producer-coverage.md` (TBD) — enumerates required emit sites for each producer (gtd-factory today; future headless renderer, SVG/SIXEL exporters, etc.).
- `docs/design/tui-vfx-pipeline-observability-event-ledger.md` (TBD) — overflow target for §13.6 once the in-spec ledger exceeds reasonable size.
- `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` — the handoff doc that surfaced this spec via the focused_row_btop bug.

## 16. Sequencing relative to other in-flight work

| Adjacent work | Relationship to this spec |
|---|---|
| V3 schema cutover (`docs/design/tui-vfx-v3-upgrade-plan/`) | Independent. Bus lives below the schema layer; works on V2 and V3 recipes equally. |
| Buy-once sweep finding 1.1.A (Phase F, DONE) | `VfxCellContext` bundle is the right place to thread the observer reference into stage `apply` methods. Phase F enabled this spec. |
| Buy-once sweep finding 1.2.A (`VfxBindable<T, S>`) | Bus's `BindingRequested`/`BindingResolved` events (deferred to Unit C) should be designed against the consolidated `VfxBindable<T, S>` shape, not the current three-sibling form. Sequencing: 1.2.A lands first; Unit C designs the binding event pair against the consolidated type. |
| Effect-composition model decision (`docs/design/tui-vfx-effect-composition-model.md`) | Coupled. Model B is locked (§10 of that doc); §11 added pre/post-pass slots around the four element stages. `PipelineStageKind` grows two variants (`PrePass`, `PostPass`) under rollout Phase C. The `Shadow` variant deprecates at rollout Phase G. The bus design is otherwise unchanged; new variants slot into the existing taxonomy. |
| Pre/post-pass rollout plan (`docs/design/tui-vfx-pre-post-pass-rollout-plan.md`) | Direct producer of new bus events. Phase B adds the pre/post-pass driver in `render_pipeline`; Phase C wires the inspector callbacks and bridge forwarding. Units B and C in this spec consume those events once they ship. See §17 for the alignment summary. |
| Mixed-signals signal-facade proposal (`docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md`) | Independent. Signal-graph evaluation could emit a `SignalEvaluated` event in a future taxonomy revision; not in v0.1.0. |
| ContentShell::card role-tagging fix (today's Phase 2) | Companion. The producer-side fix should land alongside Phase 3 of this migration so the `RoleMapMaterialized.histogram` test captures the corrected state. |

## 17. Pre/post-pass alignment (v0.3.0)

This section consolidates the alignment between this spec and `tui-vfx-pre-post-pass-rollout-plan.md`. None of the spec's design choices change; the surfaces grow.

### 17.1 What grows (and where)

| Surface | Change | Driven by rollout phase |
|---|---|---|
| `PipelineStageKind` | Add `PrePass`, `PostPass`. `Shadow` becomes a deprecation alias during the migration window. | Phase C (variants) + Phase G (alias removal) |
| `CompositorInspector` | Add `on_pre_pass_entered` / `on_pre_pass_finished` / `on_post_pass_entered` / `on_post_pass_finished` callbacks with default empty bodies. Per-cell pass callback intentionally omitted (Q12 defer). | Phase C |
| `InspectionSinkBridge` | Forward the four new callbacks to matching `TraceEvent::Stage*` variants (kind-discriminated). | Phase C |
| Compositor emit sites | Pre/post-pass driver in `render_pipeline` emits the per-pass entered/finished pairs. Shadow's legacy emit folds into `kind: PrePass` once Phase G removes the legacy fork. | Phase B (driver) + Phase C (emit) + Phase G (cutover) |
| Element-stage emit sites | Unchanged. Per-cell `*Applied` callbacks still fire for Sampler/Mask/Shader/Filter. | n/a |
| `step_id` ordering | Pre-passes increment first, then element stages, then post-passes — pipeline-execution order, top to bottom. | Phase B (driver) |
| `StageEntered` payload | Optional `canvas_extent` field populated when `kind ∈ {PrePass, PostPass}`. `#[serde(default)]`. | Phase C (Q13) |
| Validator contract-discovery | Slot occupancy queryable via the validator API (six slots). | Phase D |
| `tui-vfx-trace explain --empty-changes` | Coverage extends to pre/post-pass empty cases (`EmptyArea`, dest-aware blend short-circuit). | Phase C (Unit C consumer) |
| `tui-vfx-trace explain --pass <name>` | New investigation reporting canvas extent, blend mode, writeback `cells_modified`, mask-gate interaction. | Phase H+ (when first PostPass primitive ships) |

### 17.2 What does NOT change

- `InspectionSink` trait signature, `TraceEnvelope` shape, sink composition model, NDJSON wire format, `TraceFilter` / `TraceSelector` / `StageMask` grammar — all stable. Pre/post-pass events flow through the existing surface.
- The four element-stage emit sites (`render_loop_inspected`, `apply_shaders_inspected`) — unchanged.
- The case-study test (`StageSkipped { ScopeMatchedZeroCells }` for Role(Text) on all-Background) — still valid against the element pipeline; pre/post-passes don't emit `ScopeEvaluated`.
- Q1–Q11 decisions and their rationales.
- The cultural commitment in §13.6 — driver-attribution discipline applies to pre/post-pass emit additions in the same way it applied to Unit A.

### 17.3 Sequencing — when each piece of this spec moves

| Spec section | Updated by | Lands when |
|---|---|---|
| §5.2 `PipelineStageKind` taxonomy | rollout Phase C | with the two new variants |
| §9.1 emit sites table | rollout Phase B + C | when the driver and inspector callbacks ship together |
| §9.4 cultural commitment | rollout Phase A | when `PrePass`/`PostPass` traits land (test obligation activates) |
| §10 Unit B acceptance | rollout Phase F | gt-design recipes that adopt pre/post-pass primitives carry the producer-side coverage |
| §10 Unit C acceptance C1, C3, C4 | Unit C work loop | after rollout Phase B+C+D land |
| §11.2 Q12–Q14 | as drivers emerge | per §4.3, hooks earn their place by an investigation |
| §13.1 cultural commitment | rollout Phase A | activated when first `PrePass` impl (Shadow port, Phase B) lands |
| §16 sequencing | done in v0.3.0 | this revision |

### 17.4 What this means for the deferred Unit B and Unit C work loops

Neither unit blocks the rollout plan; the rollout plan blocks them. Concretely:

- **Unit B (gt-design wiring)** can launch any time after rollout Phase C lands (the bridge has the new callbacks). If gt-design widgets adopt pre/post-pass primitives in the meantime, Unit B's producer-side coverage extends to those passes naturally — no extra design work.
- **Unit C (CLI subcommands)** has two phases: the v0.2.0-scoped subcommands (`explain --empty-changes`, `record`, `replay`, `--bindings`, `--format sqlite`) can launch after rollout Phase C; the v0.3.0-scoped `explain --pass <name>` and slot-occupancy reporting wait until rollout Phase D and a real first PostPass primitive (Phase H) exist.

Splitting Unit C into `C-pre-rollout` (today's scope) and `C-post-rollout` (pass-aware additions) is one option for whoever picks up that work loop. Alternative: defer all of Unit C until rollout Phase D lands and ship it as one larger work loop. Decision is the user's; the spec accommodates either.

<!-- <FILE>docs/design/tui-vfx-pipeline-observability.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
