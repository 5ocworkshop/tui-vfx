<!-- <FILE>docs/design/pipeline-probe-design.md</FILE> - <DESC>First-pass design for the engine-owned AI-native pipeline observability crate and CLI</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>2026-04-27: align probe inspector strategy and verification plan with the pre/post-pass slot architecture decided in tui-vfx-effect-composition-model.md §11. Probe must capture pre/post-pass entered/finished pairs alongside the four element stages to remain truthful once rollout Phase B+C land.</WCTX> -->
<!-- <CLOG>0.4.0: extend Phase 1 inspector strategy to call out PrePass/PostPass coverage; add §17 alignment section cross-referencing the rollout plan and observability spec; expand verification plan to include slot-occupancy reporting and pass-block fingerprint round-trip.</CLOG> -->

# Pipeline Probe Design (Phase 1 draft)

## Status

This document is the first-pass design for the engine-owned observability layer described in `docs/design/ai-native-pipeline-observability-ideas.md`.
It is intentionally limited to **Phase 1**: an engine-side Rust library plus a thin CLI that accepts direct engine input and emits AI-native structured output.
The recipe adapter remains a later rollout phase.

## Goals

1. Expose full-frame, machine-parseable cell state without prose scraping.
2. Make timing assumptions explicit and self-verifying.
3. Preserve the engine/recipes split: the engine owns observability primitives; recipes own parsing and adapter behavior.
4. Establish a stable JSON contract before implementation expands into timelines, diffs, and adapter delegation.

## Non-goals for Phase 1

- No dependency from `tui-vfx` back into `tui-vfx-recipes`.
- No immediate replacement of `pipeline-validator`.
- No assertion DSL, lint mode, or formula introspection in the first pass.
- No attempt to solve every P1/P2 item before the base schema is stable.

## Evidence that shaped this design

- The ideas doc elevates structured output, full-buffer dumps, causation, timelines, diffs, stage coverage, trustworthy clocks, and loud schema drift to P0: see `docs/design/ai-native-pipeline-observability-ideas.md` ideas #1-5, #12, #19, #21, #22, #24-26.
- The engine already exposes per-cell compositor hooks via `CompositorInspector`, but only for sampler/mask/shader/filter/final-cell events: `crates/tui-vfx-compositor/src/traits/pipeline_inspector.rs`.
- The engine already has a recipe-free serialized input type in `CompositionSpec`, which is a better CLI seam than ad-hoc flags: `crates/tui-vfx-compositor/src/pipeline/cls_composition_spec.rs`.
- The current recipe-side `StageInspector` proves that storing before/after stage records is practical, but it currently lives in the adapter and still lacks content-stage coverage: `tui-vfx-recipes/src/inspector/impls/cls_stage_inspector.rs`.
- The current validator still reproduces the audit friction: prose-heavy output, row-major top-border bias, and no direct causation query.

## Recommended crate placement

Create a new workspace crate:

- `crates/tui-vfx-probe/` — library + binary target

Rationale:

- The crate is about queryable observability data contracts, not logging; it does not fit `tui-vfx-debug`.
- It should depend only on engine crates (`tui-vfx-types`, `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, possibly `tui-vfx-core`), preserving the bottom-of-stack layering.
- It can later be re-exported from the top-level `tui-vfx` crate once stable.

## Public surface

### Library API

Phase 1 library entry points should be narrow and typed:

- `ProbeSceneSpec` — direct engine scene description for one probe run. Contains the source widget grid, the initial destination frame, the widget offset inside the frame, and the compositor `CompositionSpec`.
- `ProbeRequest` — what to simulate and how much to emit.
- `ProbeReport` — top-level structured result.
- `run_probe(scene, request) -> ProbeReport`

### CLI

Recommended binary name:

- `pipeline-probe`

Phase 1 command shape should be smaller than the long-term canonical shape, but compatible with it:

```bash
pipeline-probe   --input probe-scene.json   --format json   --phase dwelling   --sample-t 0.5   --cells all
```

Phase 1 flags:

- `--input <path>`
- `--format json|ndjson`
- `--phase entering|dwelling|exiting`
- `--sample-t <0.0..1.0>`
- `--cells all|non-empty|modified`
- `--rect x,y,w,h` (optional region narrowing)
- `--with-causation` (allowed in schema now; initial implementation may emit empty traces until P0 causation lands)

## Progressive disclosure model

The contract should be layered rather than human-sampled:

1. **Frame metadata** — request, timing, widget placement, stage inventory.
2. **Summary counts** — total/non-empty/modified/effect counts.
3. **Cell records** — full widget-area cells or an explicitly requested subset.
4. **Optional per-cell trace** — stage-by-stage causation chain.
5. **Future timeline/diff layers** — arrays of frame reports or delta reports using the same cell schema.

This lets callers start wide and then ask narrower questions without changing data shape philosophy.

## Smallest useful JSON contract

```json
{
  "schema_version": "0.1.0",
  "kind": "frame_dump",
  "source": {
    "input_kind": "probe_scene_spec"
  },
  "request": {
    "phase": "dwelling",
    "sample_t": 0.5,
    "cells": "all",
    "rect": null,
    "with_causation": false
  },
  "timing": {
    "requested_phase": "dwelling",
    "requested_t": 0.5,
    "effective_phase": "dwelling",
    "effective_t": 0.5,
    "tick_ms": 3600
  },
  "frame": {
    "size": { "width": 80, "height": 24 }
  },
  "widget": {
    "abs_origin": { "x": 28, "y": 6 },
    "size": { "width": 24, "height": 9 }
  },
  "pipeline": {
    "sampler": null,
    "mask_count": 0,
    "filter_count": 0,
    "shader_count": 1,
    "style_count": 0,
    "content_count": 0
  },
  "summary": {
    "total_cells": 216,
    "non_empty_cells": 67,
    "modified_cells": 3
  },
  "cells": [
    {
      "abs": { "x": 41, "y": 6 },
      "widget_local": { "x": 13, "y": 0 },
      "ch": "─",
      "fg": { "space": "rgb", "r": 100, "g": 220, "b": 255, "a": 255 },
      "bg": { "space": "rgb", "r": 12, "g": 18, "b": 32, "a": 255 },
      "modifiers": [],
      "last_touch": {
        "stage": "shader",
        "effect": "Orbit"
      },
      "trace": []
    }
  ]
}
```

### Schema rules

- `cells` is never an implicit top-N sample. It is the exact caller-selected set.
- Colors are canonical structured objects, never prose strings.
- Coordinates always include both absolute and widget-local positions.
- Timing always reports both requested and effective values.
- `trace` is present as a stable field so later causation work extends the data, not the top-level shape.

## Proposed core data types

Suggested internal model names:

- `cls_probe_request.rs`
- `cls_probe_report.rs`
- `cls_probe_frame.rs`
- `cls_probe_cell.rs`
- `cls_probe_color.rs`
- `cls_probe_trace_event.rs`
- `cls_probe_summary.rs`
- `cls_probe_timing.rs`
- `cls_probe_widget.rs`
- `cls_probe_pipeline_inventory.rs`
- `cls_probe_rect.rs`

Suggested orchestrators/helpers:

- `orc_run_probe.rs`
- `orc_collect_frame_dump.rs`
- `fnc_select_cells.rs`
- `fnc_normalize_color.rs`
- `fnc_compute_modified_cells.rs`
- later: `orc_collect_timeline.rs`, `fnc_diff_frames.rs`

## Inspector strategy

### Phase 1

Use a new engine-side inspector implementation in `tui-vfx-probe` built on `CompositorInspector`.
Captured events, listed in pipeline-execution order (top to bottom — pre-pass first, four element stages, post-pass last):

- pre-pass entered / finished (post rollout Phase C; whole-canvas events with declared `CanvasExtent` and `BlendMode`)
- sampler events
- mask events
- shader before/after
- filter before/after
- post-pass entered / finished (post rollout Phase C)
- final rendered cell

This is sufficient for the first full-frame JSON dump plus `last_touch` attribution for compositor-owned stages and pass blocks.

### Phase 1.5 / P0 follow-up

Extend the engine-side inspector contract to add:

- `on_style_effect_applied(...)`
- `on_content_effect_applied(...)`
- `on_pre_pass_entered` / `on_pre_pass_finished` (rollout Phase C — landed in the compositor inspector contract; probe surfaces them in the JSON dump under a new `pass_blocks` field)
- `on_post_pass_entered` / `on_post_pass_finished` (same)

This closes the exact stage-coverage gap called out in idea #19 and extends it to the new pass surfaces.
The current recipe-side trait already has style interpolation support, which is evidence that the pattern is viable, but the goal is to move the canonical hook surface into the engine-owned observability path.

Per `tui-vfx-pipeline-observability.md` §11.2 Q12, pre/post passes do not emit per-cell events at v0.3.0 of the observability surface; the probe records only the stage-level entered/finished pairs plus `cells_modified` on finished. If a future investigation drives per-cell pass evidence, it lands additively.

## Input seam choice

Phase 1 should use a new `ProbeSceneSpec` as the primary CLI input contract.

Recommended shape:

- `source: ProbeGridSpec` — widget-local source grid (row-major `Cell` payloads)
- `destination: ProbeGridSpec` — initial frame grid before rendering (lets the probe observe masks, underlay, and future shadow/canvas interactions)
- `widget_offset: { x, y }` — where the widget renders inside the destination frame
- `composition: CompositionSpec` — existing serializable compositor configuration

Why:

- `render_pipeline` needs more than `CompositionSpec`; it also needs source content, destination frame, and placement.
- `CompositionSpec` remains the right nested config for the effect pipeline itself.
- The wrapper keeps the CLI direct-engine and recipe-free while remaining truthful to the actual runtime seam.
- It keeps the first fixture simple and local to `tui-vfx`.

The library API can still accept lower-level `CompositionOptions` + borrowed grids later for purely programmatic consumers.

## Rollout plan

### Phase 1 — engine-only probe

Deliver in `tui-vfx`:

- new `tui-vfx-probe` crate
- `pipeline-probe` binary
- single-frame JSON dump using direct engine input (`ProbeSceneSpec`)
- timeline and frame-diff helpers built on repeated frame dumps
- explicit timing metadata
- full-widget cell enumeration
- richer compositor trace events
- stable schema types and serialization tests

### Phase 2 — engine parity harness

Deliver in `tui-vfx` + `tui-vfx-recipes`:

- representative recipe-to-engine fixture bridge in tests
- parity checks against current validator behavior for a small corpus
- optional feature-flagged delegation path from `pipeline-validator`

### Phase 3 — adapter migration

Deliver in `tui-vfx-recipes`:

- keep Parse / Rules / Profile in recipes
- replace Render / Shader / Output / Stages with engine-probe delegation
- retain legacy path until the 101-recipe corpus passes in delegated mode

## First test fixture recommendation

Prefer a direct engine fixture first.

Recommended first fixture shape:

- a tiny `ProbeSceneSpec` JSON checked into `tui-vfx`
- source grid small enough to inspect by eye
- destination frame initialized explicitly (even if empty) so before/after deltas are unambiguous
- one visually obvious shader or filter so `last_touch` and modified-cell counts are easy to assert

Why this first:

- It tests the new contract without crossing repo boundaries.
- It keeps failures attributable to the new crate, not the recipe adapter.
- It protects the schema early, before delegation complexity is introduced.

A recipe-loaded helper is useful later for parity tests, not as the very first contract test.

## Verification plan

Phase 1 should ship with evidence at three levels:

1. **Schema tests**
   - serialize/deserialize round-trip of `ProbeReport`
   - snapshot-ish assertions for the first frame dump fixture
2. **Engine behavior tests**
   - modified-cell counts match actual cell deltas
   - `effective_phase` / `effective_t` are reported and consistent with the request
   - `cells=all|non-empty|modified` selectors behave exactly
3. **Integration smoke**
   - `cargo test --workspace` in `/usr/projects/tui-vfx`
   - later, once adapter work starts, full recipe corpus in `tui-vfx-recipes`

## Open decisions

These are the only material decisions still worth explicit confirmation before implementation starts:

1. **Crate name** — default recommendation: `tui-vfx-probe`
2. **Binary name** — default recommendation: `pipeline-probe`
3. **First fixture** — default recommendation: direct `ProbeSceneSpec` + tiny deterministic source/destination grids

If no objection appears, these defaults are the recommended implementation baseline.

## Risks to watch

- Style/content effects are not yet fully engine-observable through the compositor hook alone; that gap must be closed deliberately, not hand-waved.
- The timing contract must be treated as part of the schema, not debug-only metadata.
- The probe now supports timeline/diff helpers, but adapter delegation and style/content-stage observability still remain on the backlog.
- **Pre/post-pass coverage drift.** Once rollout Phase B+C land (`docs/design/tui-vfx-pre-post-pass-rollout-plan.md` §B, §C), the probe must capture pre/post-pass entered/finished events or its JSON dump becomes silently incomplete for any recipe that declares passes. The same fingerprint discipline that catches element-stage drift must extend to pass blocks.

## Alignment with the pre/post-pass rollout

This doc evolves with the rollout plan. The rollout plan owns the *when*; this doc owns *what the probe records about each new surface*. Cross-references:

| Probe concern | Rollout phase that produces it | Observability spec section |
|---|---|---|
| `PipelineStageKind::PrePass` / `::PostPass` variants exist | Phase C | §5.2 |
| Compositor emits per-pass entered/finished pairs | Phase B (driver) + Phase C (callbacks) | §9.1 |
| Probe inspector consumes them | Phase C (probe-side wiring lands as part of the compositor work) | n/a — probe-design owns the integration |
| Slot occupancy queryable from validator | Phase D | §10 C3 |
| Probe report surfaces slot occupancy | follow-up after Phase D | §17.4 (Unit C-post-rollout) |
| Pass-block fingerprint round-trip in `--debug-recipes-qc` | Phase F (corpus migration must fingerprint passes) + Phase G (cutover gate) | §17.3 |

When any of those rollout phases lands, the probe-design doc and the probe crate's rustdoc both move in the same change per Intention 34 rule 8 (tooling artifacts move with the architecture).

<!-- <FILE>docs/design/pipeline-probe-design.md</FILE> - <DESC>First-pass design for the engine-owned AI-native pipeline observability crate and CLI</DESC> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
