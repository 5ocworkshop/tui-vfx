# <FILE>docs/RECIPE_PROBE_GUIDE.md</FILE> - <DESC>How to use recipe-probe to debug recipe JSON through tui-vfx-probe</DESC>
# <VERS>VERSION: 0.7.0</VERS>
# <WCTX>Unified recipe probe trace, focused-cell, and motion-analysis documentation</WCTX>
# <CLOG>MINOR: Document the focused widget-cell workflow and the new motion_analysis / probe_motion_effects surfaces so recipe authors can discover the fastest root-cause and motion-debugging paths from the guide</CLOG>

# Recipe Probe Guide

`recipe-probe` is the recipe-side adapter CLI in `tui-vfx-recipes`.
It loads a recipe JSON, builds a `ProbeSceneSpec` using the real preview/render planning path, and then delegates into `tui-vfx-probe`.

Use it when you want:
- recipe JSON as the input surface
- structured JSON output instead of prose dumps
- timeline and diff reports without manually building probe scene files
- an explicit stage-by-stage success/failure summary for the configured recipe pipeline
- a focused one-cell root-cause answer via `--widget-cell x,y`

## When to use which tool

| Need | Tool |
| --- | --- |
| Parse/rules/profile validation | `pipeline-validator` |
| Structured frame/timeline/diff output from a recipe file with unified per-cell traces | `recipe-probe` or `pipeline-validator --probe` |
| Direct engine scene debugging without recipe parsing | `pipeline-probe` |

## Basic frame dump

The standalone adapter binary is the cleanest way to go from recipe JSON to probe output.

```bash
cargo run -q -p recipe-probe --   recipes/debug_recipes/shaders/shader_orbit.json   --format json   --phase dwelling   --sample-t 0.5   --cells modified   --with-causation
```

## Timeline

```bash
cargo run -q -p recipe-probe --   recipes/debug_recipes/shaders/shader_orbit.json   --format json   --phase dwelling   --frames 5   --cells modified
```

## Diff

```bash
cargo run -q -p recipe-probe --   recipes/debug_recipes/shaders/shader_orbit.json   --format json   --phase dwelling   --sample-t 0.0   --diff-to 0.5   --with-causation
```

## Validator probe mode

If you prefer to stay in `pipeline-validator`, it now has an explicit delegation mode:

```bash
cargo run -q -p pipeline-validator -- \
  --probe \
  --format json \
  --phase dwelling \
  --sample-t 0.5 \
  --probe-causation \
  recipes/debug_recipes/shaders/shader_orbit.json
```

Additional validator probe flags:
- `--probe-cells all|non-empty|modified`
- `--probe-frames N`
- `--probe-diff-to T`
- `--probe-widget-cell X,Y`

Use this when you want recipe parsing/rules/profile tooling nearby, but still want structured probe output instead of prose.

## Canvas simulation

Like `pipeline-validator`, the adapter can simulate a colored/textual underlay:

```bash
cargo run -q -p recipe-probe --   recipes/debug_recipes/shaders/shader_orbit.json   --canvas 0c1220   --canvas-content lorem   --phase dwelling   --sample-t 0.5   --cells all
```

## What it currently preserves

- recipe loading + template resolution
- preview item construction via `preview_from_recipe_config`
- real render-plan timing/placement
- phase-specific masks/samplers/filters/spatial shader extraction
- content effects and non-spatial style effects in the captured source grid

## Unified per-cell traces

`recipe-probe` and `pipeline-validator --probe` now emit a single unified probe report for recipe playback.

That report merges:
- content-stage cell changes
- non-spatial style-stage cell changes
- compositor-stage sampler/mask/shader/filter traces

So a cell trace can now show the full recipe-side chain in one place instead of splitting content/style into side metadata and compositor changes into the probe report.

The unified report also now carries:
- `runtime` — supplied runtime params plus shader binding requests/resolutions
- `cells[].root_cause` — a synthesized cell-centric explainer for “why this cell ended up here”
- `trace[].params` — parameter payloads for content/style/compositor events when available

## Operational analysis

The JSON output now also includes:
- `analysis` — operational status for the currently requested frame/timeline/diff
- `lifecycle_analysis` — operational status sampled across entering, stable dwelling, and exiting
- `motion_analysis` — timeline-only movement diagnostics for motion-candidate effects
- `focus_cell` — one widget-local cell plus its trace/root-cause when `--widget-cell x,y` is used

That analysis answers:
- which pipeline stages were configured
- which stages actually produced trace activity
- which effects were observed for each stage
- whether the combined result is operationally healthy or already failing diagnostics

At the moment it summarizes:
- `content`
- `style`
- `sampler`
- `mask`
- `shader`
- `filter`

and a combined result with:
- `status`
- error/warning diagnostic counts
- failing stages
- diagnostic codes seen in the analyzed report(s)

This is the fastest structured way to answer:
> “Did every important part of this recipe actually fire, and did the combined result stay healthy?”

Use `lifecycle_analysis` when the question is closer to:
> “Did the whole recipe lifecycle behave, or did something only work in one phase?”

Use `motion_analysis` when the question is:
> “Did this moving effect actually travel meaningfully across the timeline, or did it stall/jitter in place?”

Use `focus_cell` when the question is:
> “Why is widget-local cell `(x,y)` wrong?”

## SQLite xray over unified traces

Add `--sqlite-query` (or `--probe-sqlite-query` inside `pipeline-validator --probe`) to load the unified report into the embedded SQLite backend and query it directly.

Example:

```bash
cargo run -q -p recipe-probe -- \
  recipes/debug_recipes/content/content_typewriter.json \
  --phase dwelling \
  --frames 3 \
  --with-causation \
  --sqlite-query "select stage, count(*) as events from probe_trace_events group by stage order by stage"
```

The SQLite store now also includes operational-analysis tables:
- `probe_analysis_stages`
- `probe_analysis_effects`
- `probe_analysis_combined`
- `probe_diagnostics`
- `probe_motion_effects`
- `probe_runtime_params`
- `probe_binding_resolutions`
- `probe_cell_root_causes`

So you can ask questions like:

```bash
cargo run -q -p recipe-probe -- \
  recipes/vfx-probe-validation/alarm_lighthouse.json \
  --phase dwelling \
  --sample-t 1.0 \
  --with-causation \
  --sqlite-query "select scope, stage, status, observed_event_count from probe_analysis_stages order by scope, stage"
```

To inspect concrete configured effects instead of only stage rollups:

```bash
cargo run -q -p recipe-probe -- \
  recipes/vfx-probe-validation/alarm_lighthouse.json \
  --phase dwelling \
  --sample-t 1.0 \
  --with-causation \
  --sqlite-query "select scope, stage, effect, status from probe_analysis_effects order by scope, stage, effect"
```

`configured_instances` on those rows tells you whether the effect row represents:
- `1` → one unique configured instance
- `>1` → multiple same-name configured instances aggregated together
- `0` → trace activity without a matching configured effect name (usually worth investigating)

Compositor-backed rows now use stable ordinals in the `effect` value
(`KittScanner#1`, `Dim#2`, `BorderSweep#1`) so duplicate same-name elements in
the same stage can be distinguished directly.

Recipe-side non-spatial style layers now do the same:
- `Pulse#1`
- `Pulse#2`
- `Rainbow#1`

That means style-layer identity no longer collapses to the first legacy
`phase_effect` name in probe traces or analysis.

Or inspect diagnostics directly:

```bash
cargo run -q -p recipe-probe -- \
  recipes/vfx-probe-validation/velvet_faultline.json \
  --phase dwelling \
  --sample-t 0.0 \
  --with-causation \
  --sqlite-query "select code, severity, widget_y from probe_diagnostics order by code"
```

For timeline motion diagnostics:

```bash
cargo run -q -p recipe-probe -- \
  recipes/vfx-probe-validation/alarm_lighthouse.json \
  --phase dwelling \
  --frames 5 \
  --with-causation \
  --sqlite-query "select effect, span_x, status from probe_motion_effects order by effect"
```

For focused root-cause debugging:

```bash
cargo run -q -p recipe-probe -- \
  recipes/vfx-probe-validation/alarm_lighthouse.json \
  --phase dwelling \
  --sample-t 1.0 \
  --with-causation \
  --widget-cell 0,0
```

## Current limitations

- direct engine `pipeline-probe` remains compositor-scoped because it has no recipe context
- recipe-side unified traces currently model non-spatial style as a cell-diff stage, while spatial style continues to appear in the compositor shader stage
- validator probe mode is still additive; the legacy prose/debug surfaces remain available alongside the unified structured path

## See also

- `../../tui-vfx/docs/PIPELINE_PROBE_LLM_GUIDE.md`
- `../../tui-vfx/docs/PIPELINE_VALIDATOR_LLM_GUIDE.md`
- `../../tui-vfx/docs/PIPELINE_PROBE_WISHLIST.md`
- `../../tui-vfx/docs/design/pipeline-probe-design.md`

# <FILE>docs/RECIPE_PROBE_GUIDE.md</FILE> - <DESC>How to use recipe-probe to debug recipe JSON through tui-vfx-probe</DESC>
# <VERS>END OF VERSION: 0.6.0</VERS>
