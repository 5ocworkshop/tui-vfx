<!-- <FILE>docs/new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md</FILE> - <DESC>K2 player tooling and validation PRD</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>K2.5 companion lane: derive the clean-room player tooling roadmap from reviewed legacy recipe tooling sources.</WCTX> -->
<!-- <CLOG>0.3.0: MINOR — add source review of tui-vfx-probe, tui-vfx-debug, and tui-vfx-horseman internals.
0.2.0: MINOR — ground the roadmap in source-reviewed legacy CLI behavior and prioritize AI recipe authoring/debug loops.
0.1.0: INIT — classify legacy recipe tooling capabilities as clean-room adoption, adaptation, deferral, or rejection.</CLOG> -->

# K2 Player Tooling and Validation PRD

## Purpose

Adopt capability patterns, not source code or legacy validation authority.

The clean-room authority remains:

```text
tui-vfx-contract-cli     structural validation
tui-vfx-player-cli       canonical player/render/inventory/migration evidence
tui-vfx-player           contract-native player library
```

Legacy tools in `/usr/projects/tui-vfx-recipes` are proven AI authoring and debugging surfaces. They show which command shapes make recipe iteration fast, inspectable, scriptable, and safe. They do **not** define v3.1 semantics and must not become dependencies of `tui-vfx-player` or `tui-vfx-player-cli`.

## Deterministic review basis

This PRD is based on a source/doc review of these files rather than prompt-only inventory:

```text
/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/cli.rs
/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/main.rs
/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_debug_recipes_qc.rs
/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_normalized_explorer_mode.rs
/usr/projects/tui-vfx-recipes/tools/recipe-probe/src/main.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/README.md
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/bin/pipeline-probe.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/cls_probe_report.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/cls_probe_diff_report.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/cls_probe_timeline_report.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/cls_probe_cell.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/cls_probe_operational_analysis.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/cls_probe_sqlite_store.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/orc_run_probe.rs
/usr/projects/tui-vfx/crates/tui-vfx-probe/src/orc_collect_timeline.rs
/usr/projects/tui-vfx/crates/tui-vfx-debug/src/inspection/cls_trace_event.rs
/usr/projects/tui-vfx/crates/tui-vfx-debug/src/inspection/cls_trace_selector.rs
/usr/projects/tui-vfx/crates/tui-vfx-debug/src/inspection/cls_stage_mask.rs
/usr/projects/tui-vfx/crates/tui-vfx-debug/src/inspection/cls_trace_filter.rs
/usr/projects/tui-vfx/crates/tui-vfx-debug/src/inspection/cls_trace_sink.rs
/usr/projects/tui-vfx/crates/tui-vfx-debug/src/inspection/cls_trace_report.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-horseman/src/fnc_build_summary.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-horseman/src/fnc_build_corpus_summary.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-horseman/src/fnc_playback_summary.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-horseman/src/fnc_render_text_summary.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-trace/src/cli.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-trace/src/orc_run_trace.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-trace/src/fnc_render_report.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-trace/README.md
/usr/projects/tui-vfx-recipes/tools/tui-vfx-horseman/src/fnc_parse_cli_args.rs
/usr/projects/tui-vfx-recipes/tools/tui-vfx-horseman/README.md
/usr/projects/tui-vfx-recipes/tools/recipe-source-capture/src/main.rs
/usr/projects/tui-vfx-recipes/tools/recipe-signals-doc/src/main.rs
/usr/projects/tui-vfx-recipes/tools/recipe-signals-doc/src/orc_generate_recipe_signals_doc.rs
/usr/projects/tui-vfx-recipes/tools/recipe-validator/src/main.rs
/usr/projects/tui-vfx-recipes/tools/recipe-validator/README.md
/usr/projects/tui-vfx-recipes/tools/fnc_check_release_gate_probe_smoke.py
/usr/projects/tui-vfx-recipes/tools/fnc_generate_v3_docs.py
/usr/projects/tui-vfx-recipes/tools/loopback_fires_sweep.py
/usr/projects/tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md
/usr/projects/tui-vfx-recipes/docs/RECIPE_PROBE_GUIDE.md
/usr/projects/tui-vfx-recipes/docs/V3_STANDALONE_PREVIEW_SURFACES.md
/usr/projects/tui-vfx-recipes/Justfile
```

Review conclusion: the largest value is not any single flag. The value is the **closed AI iteration loop**:

```text
validate structure -> render one frame -> sample timeline -> diff frames -> inspect one cell -> query events -> run corpus QC -> refresh docs -> capture reproducible evidence
```

The clean-room player should grow toward that loop while keeping v3.1 report schemas stable and honest.

## Current clean-room tooling state

Current commands:

| Command | Owner | Purpose |
| --- | --- | --- |
| `validate-recipe` | `tui-vfx-contract-cli` | Contract-only structural validation for canonical v3.1 recipes. |
| `render-recipe` | `tui-vfx-player-cli` | Contract-native player frame/run evidence. |
| `render-frame` | `tui-vfx-player-cli` | Visual-frame report evidence with rows plus sparse cells. |
| `inventory-recipes` | `tui-vfx-player-cli` | Descriptor/source/effect coverage and adapter status. |
| `migration-gap` | `tui-vfx-player-cli` | Legacy/v3.1 family coverage planning evidence. |
| `primitive-adapter-gap` | `tui-vfx-player-cli` | Per-primitive support outcome and blocker classification. |

Current report schemas:

```text
v3.1.validator.report.1
v3.1.player.renderReport.1
v3.1.player.inventoryReport.1
v3.1.player.migrationGap.1
v3.1.player.visualFrameReport.1
v3.1.player.primitiveAdapterGap.1
```

## Product goals

1. Make clean-room player output useful for an AI agent iterating on recipes without opening an interactive TUI.
2. Keep every machine-readable output schema-labeled, corpus-friendly, and diffable.
3. Make unsupported, fallback, bridge, and degraded behavior explicit instead of hidden in prose.
4. Reuse the current grid-first player facts: `rows[]`, visual-frame cells, styled-cell evidence, diagnostics, and render hashes.
5. Avoid importing legacy runtime crates, V2 fallback authority, or recipe-side semantics into the clean-room player.

## Non-goals

- No direct source port from `/usr/projects/tui-vfx-recipes/tools`.
- No legacy recipe parser, preview manager, or V2 fallback dependency in `tui-vfx-player`.
- No visual parity claim from player reports.
- No command execution during recipe playback. Offline command capture may be a future authoring utility only.
- No second structural validator beside `tui-vfx-contract-cli`.

## Reviewed legacy tools and clean-room implications

### `pipeline-validator`

Reviewed source shows this is more than a validator. It is a mode-based evidence router with these durable design patterns:

| Pattern | Evidence from reviewed tool | Clean-room decision |
| --- | --- | --- |
| Mode-gated CLI | `--dump-normalized`, `--explore-normalized`, `--lowering-report`, `--migration-equivalence-report`, `--probe`, `--debug-recipes-qc` conflict with incompatible flags in `cli.rs`. | Adopt the mode-gated shape. Reject ambiguous flag combinations early. |
| Phase/sample scheduling | `--phase`, `--sample-t`, `--frames`, `scheduled_phase_samples()`, and explicit warning when `--frames` overrides `--sample-t`. | Adopt for player timeline. Preserve deterministic sample order. |
| Human-readable graph exploration | `fnc_run_normalized_explorer_mode.rs` prints identity, contracts, scene layers, step tree, scope, merge/combine, payload, and I/O. | Add a clean-room `explain-recipe`/`explain-frame` candidate after player report schemas settle. |
| Delegated probe mode | `--probe-*` flags keep validation nearby while delegating structured probe output. | Do not duplicate legacy probe internals; adopt the ergonomic idea: one command can validate then emit focused player evidence. |
| Debug fixture QC | `fnc_run_debug_recipes_qc.rs` emits summaries with family counts, source surface, validation, frame/timeline/diff probes, operational analysis, and bridge notes. | High-value: build a clean-room `fixture-qc` around contract-native reports. |
| Runtime/source overrides | `--runtime-params-json`, `--compiled-v3-source-text`, and `@file` support including command-capture artifacts. | Adopt later as explicit authoring inputs. Keep runtime recipe execution free of command spawning. |
| Canvas simulation | `--canvas` plus `--canvas-content empty|sentinel|lorem`. | Adopt after composition/canvas semantics exist. Use it to expose transparency/bleed-through bugs. |
| Strict contract checks | `--rules --strict-contracts` as a common authoring gate. | Keep structural authority in `tui-vfx-contract-cli`; player can consume validator output but not redefine contract validity. |

Clean-room takeaways:

- Near-term player commands should accept `--phase`, `--phase-t`, `--loop-t`, `--frames`, `--diff-to`, and `--recursive` consistently.
- Report summaries should include enough aggregate counters for CI and agent loops to make pass/fail decisions without custom `jq` for every run.
- Mode conflicts should be encoded in parser/validation logic, not documented only.

### `recipe-probe`

Reviewed source and guide show the strongest AI-debugging surface. It accepts recipe paths and emits structured JSON for single frames, timelines, diffs, SQLite queries, operational analysis, lifecycle analysis, motion analysis, and focused cell root cause.

High-value concepts to adopt:

| Concept | Why it matters for AI iteration | Clean-room shape |
| --- | --- | --- |
| `--frames N` | Proves an effect changes across time instead of only at one sample. | `render-frame --frames N --json` or `frame-timeline`. |
| `--diff-to T` | Answers “what changed between two samples?” without visual inspection. | `frame-diff --phase-t A --diff-to B` over visual-frame cells and render hashes. |
| `--cells all|non-empty|modified` | Controls output size for agents. | Adopt for visual-frame/timeline/diff reports. |
| `--with-causation` | Links output cells to stages/effects when trace data exists. | Defer until player has event attribution; reserve field names now. |
| `--widget-cell X,Y` | Directly answers one bad-cell question. | Adopt as `--cell X,Y` / `focusCell` once cell reports include enough provenance. |
| `--sqlite-query SQL` | Lets agents ask ad-hoc questions over large reports. | Defer implementation, but design reports so SQLite ingestion is straightforward. |
| `analysis`, `lifecycle_analysis`, `motion_analysis` | Turns raw probe data into health summaries. | Add clean-room summary blocks before adding SQL. |
| `bridge_note` | Makes fallback truth explicit. | Adopt the honesty pattern for any fallback/degraded player behavior. |

Clean-room takeaways:

- The first post-K2.5 tooling feature should be a **frame timeline/diff pair**, not a large tracing system.
- Every future report should support both “full evidence” and “small answer” modes.
- Focus-cell workflows are high leverage for AI because they avoid scanning thousands of cells.

### `tui-vfx-probe` crate

Reviewed source confirms `recipe-probe` is an adapter over a reusable engine-owned probe crate, not just a standalone CLI. The crate already defines the cleanest object model for future player evidence:

| Probe concept | Reviewed source | Clean-room implication |
| --- | --- | --- |
| `ProbeReport` | `schema_version`, `kind`, `source`, `request`, `timing`, `frame`, `widget`, `pipeline`, `runtime`, `summary`, `diagnostics`, `cells`. | Use this as the reference shape for future player frame/timeline/debug reports, translated into v3.1 player vocabulary. |
| `ProbeCell` | Absolute and widget-local coordinates, glyph, fg/bg, modifiers, last touch, trace, root cause. | Add cell provenance incrementally; keep coordinates explicit and avoid making consumers infer local vs frame coordinates. |
| `ProbeTimelineReport` | Evenly spaced samples across one phase using repeated `run_probe`. | K2.6 timeline can be simple: repeated current player samples, no new runtime semantics. |
| `ProbeDiffReport` | `from_t`, `to_t`, changed-cell count, per-cell before/after values. | K2.6 diff should compare visual-frame/styled-cell evidence and row changes. |
| `ProbeOperationalAnalysis` | Stage/effect configured counts, touched cells, event counts, diagnostic counts, combined status. | Add analysis summaries once player has enough stage/effect attribution; do not block timeline/diff on full causation. |
| `ProbeSqliteStore` | In-memory tables for frames, cells, trace events, runtime params, binding resolutions, root causes, diagnostics, diffs, analysis, motion effects. | Design report fields so a future SQLite/xray utility can ingest them without re-shaping. Defer SQL execution until reports are richer. |
| `pipeline-probe` CLI | Rejects `--frames` with `--diff-to`, and rejects `--widget-cell` with timeline/diff/SQL modes. | Copy the deterministic conflict rules into clean-room CLI validation. |

Important boundary: `tui-vfx-probe` is compositor/engine observability. The clean-room player should learn from its report contracts, but K2 player evidence remains contract-native and must not import legacy recipe runtime semantics.

### `tui-vfx-trace`

Reviewed source shows a selector/stage/time-window trace CLI with NDJSON and summary report outputs. It supports selectors (`all`, `cell`, `rect`, `role`, `layer`, `recipe`), stage masks (`lifecycle`, `resolution`, `composition`, `pipeline`, plus pipeline aliases), `--from-ms`, `--to-ms`, and BrokenPipe-tolerant output.

Clean-room decision:

- Defer full trace until player emits real attribution events.
- Adopt the selector grammar and stage-mask vocabulary as the likely future shape.
- Adopt BrokenPipe-safe streaming behavior for any future NDJSON command.
- Preserve report mode and stream mode separately:

```text
trace-frame --format report
trace-frame --format ndjson --output -
```

### `tui-vfx-debug` crate

Reviewed source shows the trace substrate behind `tui-vfx-trace`. Its value is the event taxonomy and filtering model:

| Debug concept | Reviewed source | Clean-room implication |
| --- | --- | --- |
| `TraceEvent` taxonomy | Lifecycle, resolution, composition, and pipeline variants, including stage entry/finish/skip, scope evaluation, and role-map materialization. | Future `trace-frame` should keep this four-stage vocabulary; K2.6 should not try to fake it before events exist. |
| `TraceSelector` | `all`, exact cell, rect, role, layer, recipe selectors. | Adopt selector names later; for near-term player reports start with simpler `--cells` and `--cell`. |
| `StageMask` | Four low-bit stages plus `NONE`/`ALL`, cheap empty-mask checks. | Any future trace command should use explicit masks and fast no-op paths. |
| `TraceFilter` | OR across selectors, AND across stage/frame/time ranges. | Preserve deterministic filtering semantics in docs before implementing trace. |
| `TraceSink` | Thread-safe, optionally bounded, dropped counter. | Streaming/long-running reports must expose truncation/dropped counts. |
| `TraceReport` | Envelope list, per-stage summary, dropped count, NDJSON round-trip. | Future trace reports should expose both full events and summary counters. |

Clean-room decision: treat `tui-vfx-debug` as the canonical vocabulary source for future trace semantics, but do not make K2.5/K2.6 player reports claim trace coverage until the player emits real lifecycle/resolution/composition/pipeline events.

### `tui-vfx-horseman`

Reviewed source/readme show a deliberately thin headless summary surface:

```text
tui-vfx-horseman [--json] [--project-root <path>] (--corpus <dir> | <recipe.json>)
```

It reports direct V3 snapshot vs legacy preview fallback and corpus warning counts without opening raw terminal mode or claiming full-color sign-off.

Clean-room decision:

- Most of this is already covered by `render-recipe`, `inventory-recipes`, and `migration-gap`.
- Adopt the **thin corpus summary** ergonomics: one command should answer “which recipes load/render, which warn, which fail?”
- Reject legacy preview fallback as authority. If a clean-room report ever falls back or degrades, it must say so in a `fallbacks[]` or `warnings[]` field.

Additional source-reviewed details:

- `fnc_build_summary.rs` emits `schema_version`, recipe/source paths, declared schema, bridge flag, warnings, and playback summary. This is a useful compact envelope pattern for player corpus commands.
- `fnc_build_corpus_summary.rs` recursively discovers JSON files, filters schema version 3, counts direct V3 snapshots, legacy preview items, unavailable reports, load failures, and schema-detection errors. This is exactly the level of aggregate accounting a clean-room corpus smoke should provide.
- `fnc_playback_summary.rs` makes direct V3 snapshot details compact: phase, sample time, loop time, absolute time, render hash, non-empty cells, dimensions, offsets, shader families, shadow flag, cell-motion presence, and content-effect fields. Clean-room player reports already own some of these; cell-motion/content-effect summaries are good future inventory fields.
- `fnc_render_text_summary.rs` keeps text output digestible while JSON stays machine-readable. Clean-room commands should preserve this split instead of making humans read large JSON by default.

### `recipe-source-capture`

Reviewed source shows an offline command-capture artifact writer:

```text
schema=tui_vfx.command_capture.v1
stdout_ansi
stdout_text
source_text
stderr_text
success
exit_code
```

It can preserve artifacts for nonzero commands only with `--allow-nonzero`, and pipeline-validator can consume `source_text` / `stdout_text` via `--compiled-v3-source-text @artifact.json`.

Clean-room decision:

- Adopt later as an **authoring fixture capture utility**, not player runtime behavior.
- The valuable pattern is reproducible offline evidence: command, cwd, success, exit code, normalized stdout/stderr, and source text.
- Future clean-room tooling could accept `--source-text TEXT|@FILE` for authoring/debug recipes, but playback must never spawn commands.

### `recipe-signals-doc` and generated V3 docs

Reviewed source shows two drift-check patterns:

1. `recipe-signals-doc --write|--check` derives docs from catalog + upstream rustdoc + overlay, and `--check` fails on drift.
2. `tools/fnc_generate_v3_docs.py --write|--check` generates `docs/generated/V3_API.md`, JSON, and README from Rust docs and rejects missing public item docs.

Clean-room decision:

- Adopt generated report-schema docs after player report schemas stabilize.
- Add a `--check` mode rather than relying on manual doc updates.
- Candidate future commands:

```text
tui-vfx-player-cli report-schema-docs --write
tui-vfx-player-cli report-schema-docs --check
```

### `recipe-validator`

Reviewed source and README confirm the tool is deprecated and warns users to prefer `pipeline-validator`.

Clean-room decision:

- Reject as a new path.
- Keep it only as a migration lesson: old validators become confusing when newer canonical surfaces exist, so clean-room tooling should retire or alias old commands explicitly.

### Release-gate scripts and Justfile composition

Reviewed `Justfile` and scripts show the recipe repo values composed, GUI-free release gates:

```text
just v3-headless-smoke
just v3-release-gate-probe-smoke
just docs-v3-check
```

The release-gate smoke script runs `recipe-probe`, parses JSON, checks combined analysis/lifecycle success, checks non-empty cells, and can write the report artifact.

Clean-room decision:

- Adopt a small justfile vocabulary that composes existing player commands with pipe-safe JSON checks.
- Prefer deterministic scripts for multi-step release gates instead of fragile shell one-liners.
- Keep pipe safety explicit for streaming/large output commands.

### `loopback_fires_sweep.py`

Reviewed source shows a corpus sweep that runs recipes with a SQLite-backed loopback registry, then summarizes which bindings fell back.

Clean-room decision:

- Do not adopt loopback behavior into the player.
- Adopt the **fallback sweep** pattern: corpus reports should count and list fallback/defaulted/degraded behavior so missing runtime support is visible.

## Adoption matrix

| Feature family | Legacy source | Clean-room target | Priority | Decision |
| --- | --- | --- | --- | --- |
| Schema-labeled JSON reports | `pipeline-validator`, `horseman`, `recipe-probe`, `trace` | Existing player reports; standardize field names | Immediate | Adopt |
| Recursive corpus commands | `pipeline-validator`, `horseman` | Existing `--recursive`; keep aggregate summaries | Immediate | Adopt |
| Mode conflict validation | `pipeline-validator` | Parser/option validation per command | Immediate | Adopt |
| Phase/sample/frame controls | `pipeline-validator`, `recipe-probe` | `render-frame --phase --phase-t --loop-t --frames` | High | Adopt next |
| Frame timeline | `recipe-probe` | `frame-timeline` or `render-frame --frames` | High after K2.5 | Adopt |
| Frame diff | `recipe-probe` | `frame-diff` or `render-frame --diff-to` | High after timeline | Adopt |
| Cell selectors | `recipe-probe`, `tui-vfx-probe`, `trace` | `--cells all|non-empty|modified` and later `--select` | High | Adopt in staged form |
| Focus cell/root cause | `recipe-probe`, `tui-vfx-probe` | `--cell X,Y`, `focusCell`, future `rootCause` | High after cell provenance | Adopt |
| Operational/lifecycle/motion summaries | `recipe-probe`, `tui-vfx-probe`, QC runner | `analysis`, `lifecycleAnalysis`, `motionAnalysis` report blocks | Medium/high | Adopt incrementally |
| Debug fixture QC | `pipeline-validator --debug-recipes-qc` | `fixture-qc` over clean-room reports | Medium/high | Adopt after timeline/diff |
| Runtime params/source overrides | `pipeline-validator` | `--runtime-params-json`, `--source-text` | Medium | Adapt with strict authority boundaries |
| Canvas simulation | `pipeline-validator`, `recipe-probe` | `--canvas`, `--canvas-content` | Medium after composition substrate | Defer/adapt |
| Trace selectors/stage masks | `tui-vfx-debug`, `tui-vfx-trace` | `trace-frame --select --stages` | Later | Defer/adopt vocabulary |
| NDJSON streams and dropped counts | `tui-vfx-debug`, `tui-vfx-trace` | `trace-frame --format ndjson` | Later | Defer |
| SQLite xray | `tui-vfx-probe`, `recipe-probe`, `pipeline-validator --probe` | Optional report ingestion/query helper | Later | Defer; design reports to ingest cleanly |
| Headless corpus smoke | `tui-vfx-horseman`, `Justfile`, release-gate script | `just k2-player-smoke` / script around player JSON | Medium | Adopt composed gate |
| Command capture artifacts | `recipe-source-capture` | Offline authoring fixture capture | Later | Adapt; never runtime playback |
| Generated docs checks | `recipe-signals-doc`, `fnc_generate_v3_docs.py` | `report-schema-docs --check` | Medium | Adopt after schema maturity |
| Deprecated validator | `recipe-validator` | None | Immediate | Reject |

## Proposed clean-room command taxonomy

Current commands:

```text
render-recipe
render-frame
inventory-recipes
migration-gap
primitive-adapter-gap
```

Near-term candidates, ordered for maximum AI iteration value:

```text
render-frame --phase <enter|dwell|exit>
render-frame --phase-t <number>
render-frame --loop-t <number>
render-frame --frames <N>
render-frame --diff-to <phaseT>
render-frame --cells <all|non-empty|modified>
render-frame --cell <x,y>
```

If that becomes too dense, split after compatibility is proven:

```text
frame-timeline
frame-diff
explain-frame
fixture-qc
```

Later candidates:

```text
trace-frame
probe-frame
report-schema-docs
capture-source
```

## Future report schemas

Future candidates, not implemented in K2.5:

```text
v3.1.player.frameTimelineReport.1
v3.1.player.frameDiffReport.1
v3.1.player.frameAnalysis.1
v3.1.player.fixtureQcReport.1
v3.1.player.traceReport.1
v3.1.player.traceStream.1
v3.1.player.schemaDocsReport.1
v3.1.player.commandCapture.1
```

## Minimum viable K2.6 recommendation

The next tooling packet should be small and high leverage:

1. Add frame timeline report over `render-frame` evidence.
2. Add frame diff report between two phase samples.
3. Add `--cells all|non-empty|modified` so reports can be agent-sized.
4. Keep output schema-labeled and corpus-summarized.
5. Do not add trace, SQLite, canvas, command capture, or legacy runtime fallback yet.

Acceptance sketch:

```text
render-frame --frames 5 --json recipe.json
  -> schema=v3.1.player.frameTimelineReport.1
  -> frames[0..N].visualFrame
  -> summary.totalFrames=N

render-frame --phase-t 0.0 --diff-to 0.5 --json recipe.json
  -> schema=v3.1.player.frameDiffReport.1
  -> fromFrame, toFrame, changedCells, changedRows, hashChanged
```

Why this first: it directly supports AI recipe debugging with the evidence already present after K2.5, and it creates the substrate for later motion analysis, fixture QC, and trace attribution.

## Scope guard

The tooling PRD must not turn legacy tooling into v3.1 authority.
It classifies legacy tooling as oracle inspiration only.
Any new clean-room CLI feature must report through schema-labeled v3.1 player or contract reports.

<!-- <FILE>docs/new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md</FILE> - <DESC>K2 player tooling and validation PRD</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
