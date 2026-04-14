<!-- <FILE>docs/PIPELINE_PROBE_LLM_GUIDE.md</FILE> - <DESC>How an LLM or user should use pipeline-probe to debug direct engine scenes</DESC> -->
<!-- <VERS>VERSION: 0.9.0</VERS> -->
<!-- <WCTX>Probe-side diagnostics documentation</WCTX> -->
<!-- <CLOG>MINOR: Document the new probe-side diagnostics helpers for border/text integrity issues and clarify how they complement the existing report/timeline/diff surfaces</CLOG> -->

# Pipeline Probe: A Direct-Engine Guide for LLMs and Humans

`pipeline-probe` is the engine-owned observability CLI for `tui-vfx`.
Use it when you want to inspect **direct engine scenes** instead of recipe JSON.

It answers the question:

> “Given this source grid, destination frame, placement, and composition config, what does one rendered frame actually contain?”

## When to use `pipeline-probe` vs `pipeline-validator`

| Situation | Tool |
| --- | --- |
| You have a recipe JSON in `tui-vfx-recipes` and want recipe-aware parsing/rules/profile checks | `pipeline-validator` |
| You have a recipe JSON and want structured frame/timeline/diff output | `recipe-probe` or `pipeline-validator --probe` |
| You already have engine-level grids + `CompositionSpec` and want structured output now | `pipeline-probe` |
| You need JSON/NDJSON instead of prose dumps | `pipeline-probe` or `recipe-probe` |
| You need parse/rules/profile checks from the recipe schema | `pipeline-validator` |

The recipe-side adapter CLI `recipe-probe` lives in the sibling `tui-vfx-recipes` repo at `tools/recipe-probe/`. It builds a `ProbeSceneSpec` for you from a recipe file and then delegates into the capabilities documented below. If you are already using `pipeline-validator`, the same sibling repo now also exposes `pipeline-validator --probe` as an in-place delegation path. Those recipe-side tools now emit unified per-cell traces that merge content, style, and compositor stages, while direct `pipeline-probe` remains compositor-scoped.

## What phase 1 supports

- one rendered frame per invocation
- multi-frame timelines via `--frames N`
- frame-to-frame diffs via `--diff-to T`
- `json` and `ndjson` output
- cell selectors: `all`, `non-empty`, `modified`
- widget/frame metadata
- summary counts
- compositor-stage `last_touch`
- optional trace emission with sampler/mask metadata and filter/shader before/after snapshots
- probe-side diagnostics helpers in the library for:
  - alphabetic text leaking onto border rows
  - underline glyphs contaminating the bottom border

## What phase 1 does not support yet

- style/content stage attribution
- full engine-wide causation coverage beyond compositor callbacks
- recipe adapter delegation
- full automatic detection of text truncation, missing-glyph, and motion-continuity defects at the CLI level

## Input document: `ProbeSceneSpec`

The CLI reads a single JSON document with four fields:

- `source` — widget-local source grid
- `destination` — destination frame before rendering
- `widget_offset` — where the widget is rendered inside the destination frame
- `composition` — serialized `CompositionSpec`

### Minimal shape

```json
{
  "source": {
    "width": 1,
    "height": 1,
    "cells": [
      {
        "ch": "A",
        "fg": { "r": 255, "g": 255, "b": 255, "a": 255 },
        "bg": { "r": 0, "g": 0, "b": 0, "a": 255 },
        "mods": {
          "bold": false,
          "italic": false,
          "underline": false,
          "dim": false,
          "reverse": false,
          "strikethrough": false,
          "slow_blink": false,
          "rapid_blink": false,
          "hidden": false
        },
        "mod_alpha": null
      }
    ]
  },
  "destination": {
    "width": 1,
    "height": 1,
    "cells": [
      {
        "ch": " ",
        "fg": { "r": 255, "g": 255, "b": 255, "a": 255 },
        "bg": { "r": 0, "g": 0, "b": 0, "a": 255 },
        "mods": {
          "bold": false,
          "italic": false,
          "underline": false,
          "dim": false,
          "reverse": false,
          "strikethrough": false,
          "slow_blink": false,
          "rapid_blink": false,
          "hidden": false
        },
        "mod_alpha": null
      }
    ]
  },
  "widget_offset": { "x": 0, "y": 0 },
  "composition": {
    "masks": [],
    "mask_combine_mode": "all",
    "filters": [],
    "shader_layers": [],
    "preserve_unfilled": true,
    "t": 0.0
  }
}
```

Important:
- `cells` is row-major
- `source.cells.len()` must equal `source.width * source.height`
- `destination.cells.len()` must equal `destination.width * destination.height`
- the widget rectangle must fit entirely inside the destination frame

## Flags that matter

- `--input <path>` — required scene JSON file
- `--format json|ndjson` — pretty JSON for humans, NDJSON for tools
- `--phase entering|dwelling|exiting` — sets the phase passed into the engine
- `--sample-t <0.0..1.0>` — phase-local progress value
- `--cells all|non-empty|modified` — how much cell detail to emit
- `--with-causation` — emit trace entries with sampler source coords, mask visibility, and shader/filter before/after snapshots
- `--frames N` — sample evenly across the selected phase and emit a timeline report
- `--diff-to T` — compare `--sample-t` against another phase-local time and emit only changed cells
- `--sqlite-query SQL` — materialize the produced report into an in-memory SQLite database and return query results as JSON rows

## SQLite xray mode

For large timelines or dense per-cell traces, use the embedded SQLite query layer instead of grepping JSON.

Example: count all trace events in a timeline run

```bash
cargo run -q -p tui-vfx-probe --bin pipeline-probe -- \
  --input animated-scene.json \
  --phase dwelling \
  --frames 5 \
  --sqlite-query "select stage, count(*) as events from probe_trace_events group by stage order by events desc"
```

Useful tables:
- `probe_runs`
- `probe_frames`
- `probe_cells`
- `probe_trace_events`
- `probe_diff_cells`

This is especially helpful when you need to answer questions like:
- which cells were touched by shader but not filter?
- how many modified cells exist per frame?
- what is the full trace history for widget-local `(x, y)` across a timeline?

## Probe-side diagnostics helpers

For callers embedding `tui-vfx-probe` as a library, the crate now exposes
basic diagnostics helpers on top of `ProbeReport`:

- `row_text(&report, y)` — reconstruct a widget-local text row
- `max_widget_y(&report)` — find the bottommost emitted widget row
- `has_ascii_alpha(text)` — detect semantic text leakage into decorative rows
- `collect_basic_diagnostics(&report)` — emit typed diagnostics for:
  - `alpha_on_top_border`
  - `alpha_on_bottom_border`
  - `underline_on_bottom_border`

These helpers are intended to bridge the gap between:
- **structural truth** — “which cells changed?”
- **visual integrity** — “did the border/text contract survive composition?”

They do **not** replace human visual QA, but they do convert common classes of
visual breakage into repeatable machine checks.

On the recipe side, `tui-vfx-recipes` now layers an additional diagnostics pass
on top of probe output that can reason about the intended message string across
dwell samples. That recipe-aware layer is where checks like
`expected_message_missing` belong, because only the recipe adapter knows the
semantic text contract.

## Typical workflows

### 1. Sanity-check a no-effect scene

```bash
cargo run -q -p tui-vfx-probe --bin pipeline-probe -- \
  --input probe-scene.json \
  --format json \
  --phase dwelling \
  --sample-t 0.5 \
  --cells all
```

Expect:
- `modified_cells = 0`
- no `last_touch` entries
- empty `trace` arrays

### 2. Ask “which cells did this filter change?”

```bash
cargo run -q -p tui-vfx-probe --bin pipeline-probe -- \
  --input dim-scene.json \
  --format ndjson \
  --phase dwelling \
  --sample-t 1.0 \
  --cells modified \
  --with-causation
```

Expect:
- only changed cells in `cells[]`
- `last_touch.stage = "filter"`
- `trace[0].stage = "filter"`

### 3. Ask “did my shader actually touch anything?”

Run with `--cells modified` on a scene containing a spatial shader.
If `cells[]` is empty, either:
- the shader did not visibly modify the selected frame, or
- your `sample_t` / placement / region assumptions are wrong

### 4. Ask “how does this effect evolve across the phase?”

```bash
cargo run -q -p tui-vfx-probe --bin pipeline-probe -- \
  --input animated-scene.json \
  --format json \
  --phase dwelling \
  --frames 5 \
  --cells modified
```

Expect:
- `kind = "timeline"`
- `frame_count = 5`
- `frames[0].timing.requested_t = 0.0`
- `frames[4].timing.requested_t = 1.0`

### 5. Ask “what changed between two times?”

```bash
cargo run -q -p tui-vfx-probe --bin pipeline-probe -- \
  --input animated-scene.json \
  --format json \
  --phase dwelling \
  --sample-t 0.0 \
  --diff-to 0.5 \
  --with-causation
```

Expect:
- `kind = "frame_diff"`
- `changed_cells_count > 0` when the effect is visibly animated
- `cells[].before` / `cells[].after` snapshots for each changed cell

## How to read the output

Key fields:

- `request` — what you asked the probe to do
- `timing` — what the probe says it actually simulated
- `frame.size` — destination frame size
- `widget.abs_origin` + `widget.size` — where the widget lives in the frame
- `summary.total_cells` — full widget-local cell count
- `summary.non_empty_cells` — cells whose final rendered state is not empty
- `summary.modified_cells` — cells whose final rendered state differs from the source grid
- `cells[].abs` — frame-absolute coordinates
- `cells[].widget_local` — widget-local coordinates
- `cells[].last_touch` — most recent compositor stage that touched the cell
- `cells[].trace` — per-cell compositor trace events; shader/filter events carry `before`/`after` snapshots, sampler events carry `sampled_from`, and mask events carry `visible`
- `diagnostics[]` — optional structured probe warnings/errors for visual-integrity findings that can be determined directly from the report

## Recommended LLM workflow

1. Start with `--cells modified`.
2. If that is empty, rerun with `--cells all` to confirm placement and baseline cell state.
3. Check `summary.modified_cells` against your expectation before reasoning about specific cells.
4. Use `widget_local` coords when comparing against source-grid intent.
5. Use `abs` coords when comparing against frame overlays or underlays.
6. Use `--frames` when you need progression across a phase and `--diff-to` when you only care about changed cells between two times.
7. Treat `trace` as compositor-scoped causation: it is richer than before, but it still does not cover recipe-side style/content stages yet.
8. When debugging border/text integrity, pair raw report inspection with the new diagnostics helpers instead of relying only on ad hoc SQL.

## Current limitations to keep in mind

- `modified` means “final widget-local cell differs from the source grid cell”, not “any stage touched this cell at any point”.
- `last_touch` only covers compositor callbacks currently exposed by `CompositorInspector`.
- style/content stages are still outside the current direct-engine probe path.
- `trace` is richer now, but it is still limited to the compositor callbacks currently exposed by `CompositorInspector`.

## See also

- `docs/design/pipeline-probe-design.md`
- `docs/PIPELINE_VALIDATOR_LLM_GUIDE.md`
- `crates/tui-vfx-probe/README.md`

<!-- <FILE>docs/PIPELINE_PROBE_LLM_GUIDE.md</FILE> - <DESC>How an LLM or user should use pipeline-probe to debug direct engine scenes</DESC> -->
<!-- <VERS>END OF VERSION: 0.8.0</VERS> -->
