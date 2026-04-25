<!-- <FILE>docs/tooling/INDEX.md</FILE> - <DESC>Tooling documentation index for tui-vfx and tui-vfx-recipes.</DESC> -->
<!-- <VERS>VERSION: 0.2.3</VERS> -->
<!-- <WCTX>Make the V3 tooling hub a command-first start page that maps the as-built validator, probe, diff/database, preview/player, resize, edge-ingestion, command-capture, trace, docs-generation, and headless smoke surfaces.</WCTX> -->
<!-- <CLOG>0.2.3: add the probe_alarm_lighthouse release-gate smoke command to the headless tooling map.</CLOG> -->

# Tooling documentation

This is the start page for V3 tooling across `tui-vfx` and
`tui-vfx-recipes`. Use it to choose the existing command surface before opening
a deeper guide. Do not add a new tool until the existing validator, probe,
trace, preview/player, capture, or docs-generation surface cannot represent the
needed evidence.

This page is the canonical map for the current as-built surfaces. If a task
fits one of the rows below, reuse that command surface first and only open a
deeper guide when you need more detail.

## Quick command map

| Need | Existing command surface | Status | Boundary |
|---|---|---|---|
| Validate recipe structure/rules | `cargo run -q -p pipeline-validator -- --rules <recipe.json>` from `/usr/projects/tui-vfx-recipes` | as-built | Recipe-aware validation. Keeps V2/V3 cutover behavior in the recipe repo. |
| Inspect normalized V3 IR | `pipeline-validator --dump-normalized --format json <recipe.json>` | as-built | Canonical normalized authoring shape. Not a renderer. |
| Inspect V3 lowering invariants | `pipeline-validator --lowering-report --format json <recipe.json>` | as-built | Machine-readable unresolved-lowering / human-review queue. |
| Check V2↔V3 migration pairs | `pipeline-validator --migration-equivalence-report --format json <manifest.json>` | as-built | Manifest-driven migration evidence, not owner audit. |
| Run debug fixture QC | `pipeline-validator --debug-recipes-qc --format json <recipe.json>` | as-built | Upstream-native structured QC over debug fixtures/concrete exports. |
| Recipe-to-scene probe | `cargo run -q -p recipe-probe -- <recipe.json> ...` | as-built | Recipe-aware adapter into the engine probe shape. |
| Validation plus probe in one flow | `pipeline-validator --probe ... <recipe.json>` | as-built | Validator owns recipe parsing/rules; delegated output is structured probe evidence. |
| Direct engine scene probe | `cargo run -q -p tui-vfx-probe --bin pipeline-probe -- --input <scene.json> ...` from `/usr/projects/tui-vfx` | engine-level | `ProbeSceneSpec`; not recipe-aware. |
| Frame diff and SQLite xray | `recipe-probe --diff-to <t> --sqlite-query '<sql>' <recipe.json>` | diff/database | Reuse existing probe report/database shape. Do not invent a parallel diff format. |
| Unified lifecycle/trace evidence | `cargo run -q -p tui-vfx-trace -- --recipe <recipe.json> --format report` | trace | Headless recipe trace CLI for lifecycle/resolution/composition/pipeline evidence. |
| Lightweight player summary / corpus readiness | `cargo run -q -p tui-vfx-horseman -- (--corpus <dir> | <recipe.json>) [--json]` | headless player | Packaged non-interactive summary surface. The command and package are both `tui-vfx-horseman`; do not document the discarded temporary player name. |
| Human preview/browser | `cargo run --example demo -- [recipe.json]` | preview/player | Interactive ratatui browser and visual sign-off. |
| Fullscreen single-recipe preview | `cargo run --example play_recipe -- <recipe.json>` | preview/player | Human isolation of one recipe; terminal lifecycle stays in the example/player. |
| Minimal V3 inspector/player | `cargo run --example v3_play_recipe -- <v3-recipe.json>` | example-level | Prints normalized IR and deterministic render metadata for supported V3 recipes. |
| Demo-path cell dump | `cargo run --example diag_render_dump -- <recipe.json> [dwell_seconds]` | debug dump | Machine-readable cells through the same render path as the demo. |
| Resize contract evidence | `cargo run --example diag_resize_preserve_phase -- [recipe.json]` | host-owned resize | Shows host-owned resize rerendering with preserved phase/time. No core resize loop. |
| Offline command-output capture | `cargo run -q -p recipe-source-capture -- --output <artifact.json> -- <cmd> ...` | offline-only | Authoring/tooling capture only. Runtime recipe playback must not spawn commands. |
| Generated capabilities/API docs | `just docs-generate`, `just docs-check`, `just docs-validate`, `just docs-api`, `just docs-api-check`, `just docs-api-validate` from `/usr/projects/tui-vfx` | generated docs | `xtask`/`just` pipeline for generated docs and drift checks. |
| Headless Chapter 100 smoke | `cd /usr/projects/tui-vfx-recipes && just v3-headless-smoke` | as-built | Headless release-gate rehearsal for Chapter 100. Composes validator, debug-QC, probe, trace, release-gate probe evidence, and docs freshness checks while keeping a legacy fallback probe in the same run. |
| First release-gate probe smoke | `cd /usr/projects/tui-vfx-recipes && just v3-release-gate-probe-smoke` | as-built | Smallest GUI-free evidence check: runs `recipe-probe --format json` for `probe_alarm_lighthouse` and validates the report status/cells shape. |

Status shorthand used above:

- `as-built` means the surface already exists and should be reused as the first
  stop.
- `engine-level` means the command operates on engine inputs, not recipes.
- `headless player` means non-interactive summary/corpus evidence; it does not enter raw mode or provide full-color playback.
- `preview/player` means the host owns terminal lifecycle and full-color interactive playback.
- `offline-only` means the command captures authoring data but must not run
  during recipe playback.
- `generated docs` means the command checks or emits generated documentation
  artifacts, not hand-edited docs.

## Common workflows

### Validate and inspect a V3 recipe

```bash
cd /usr/projects/tui-vfx-recipes
cargo run -q -p pipeline-validator -- \
  --rules \
  --strict-contracts \
  recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json

cargo run -q -p pipeline-validator -- \
  --dump-normalized \
  --format json \
  recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json

cargo run -q -p pipeline-validator -- \
  --lowering-report \
  --format json \
  recipes/debug_recipes/styles/style_cell_position_binding.json
```

Use this path for authoring/schema questions. Use `--strict-contracts` when the
question is whether V3 runtime binding and template placeholder usage is fully
declared.

### Probe, diff frames, and query the probe database

```bash
cd /usr/projects/tui-vfx-recipes
cargo run -q -p recipe-probe -- \
  recipes/debug_recipes/shaders/_DEPRECATED_shader_orbit.json \
  --phase dwelling \
  --sample-t 0.0 \
  --diff-to 0.5 \
  --with-causation \
  --sqlite-query "select count(*) as changed_cells from probe_diff_cells"

cargo run -q -p pipeline-validator -- \
  --probe \
  --probe-diff-to 0.5 \
  --probe-sqlite-query "select count(*) as changed_cells from probe_diff_cells" \
  recipes/debug_recipes/shaders/_DEPRECATED_shader_orbit.json
```

Use `recipe-probe` when the probe report is the primary output. Use
`pipeline-validator --probe` when validation and probe evidence should remain in
one command. The SQLite materialization is the current xray surface for large
reports; prefer it over ad-hoc JSON greps.

### Probe a direct engine scene

```bash
cd /usr/projects/tui-vfx
cargo run -q -p tui-vfx-probe --bin pipeline-probe -- \
  --input scene.json \
  --phase dwelling \
  --sample-t 0.5 \
  --diff-to 0.75 \
  --format json
```

This expects a `ProbeSceneSpec` input. It does not parse recipes and should not
be used to prove recipe loader behavior.

### Capture trace evidence

```bash
cd /usr/projects/tui-vfx-recipes
cargo run -q -p tui-vfx-trace -- \
  --recipe recipes/debug_recipes/shaders/_DEPRECATED_shader_orbit.json \
  --frames 3 \
  --stages lifecycle,resolution,composition,pipeline \
  --format report
```

Use trace output for lifecycle/stage questions that are broader than a single
frame diff.

### Headless summary or full-color playback

```bash
cd /usr/projects/tui-vfx-recipes
cargo run -q -p tui-vfx-horseman -- \
  recipes/debug_recipes/content/content_slide_shift.json \
  --json

cargo run -q -p tui-vfx-horseman -- \
  --corpus recipes/debug_recipes/content \
  --json
```

Use `tui-vfx-horseman` for lightweight headless summaries and corpus-readiness
evidence. It reuses the demo/player preview seam, but it is not the full-color
interactive browser. The command and package are both `tui-vfx-horseman`; use that name in docs and scripts so the tool is not confused with full-color playback.

### Full-color preview or playback

```bash
cd /usr/projects/tui-vfx-recipes
cargo run --example demo -- recipes/debug_recipes/shaders/compositions/shader_barber_pole.json
cargo run --example play_recipe -- recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json
cargo run --example v3_play_recipe -- recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json
cargo run --example diag_render_dump -- recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json 1.0
```

Preview examples are full-color host/player surfaces. They may own terminal
lifecycle, keyboard events, and ratatui buffers. They should not move those
responsibilities into the compositor or recipe schema. Use the headless
validator/probe/trace/`tui-vfx-horseman` lanes when the evidence only needs
structured summaries rather than interactive visual sign-off.

### Verify resize behavior

```bash
cd /usr/projects/tui-vfx-recipes
cargo run --example diag_resize_preserve_phase -- \
  recipes/debug_recipes/complex/resize_preserve_phase_chain.json
```

The as-built contract is host-owned resize: preserve elapsed/runtime state,
choose a new target grid, and ask V3 to render again. Add core resize machinery
only for a proven bug, not because a player receives a terminal resize event.

### Capture command output for recipe sources

```bash
cd /usr/projects/tui-vfx-recipes
cargo run -q -p recipe-source-capture -- \
  --output /tmp/tui-vfx-command-capture.json \
  -- bash -lc 'printf "\033[32mOK\033[0m\\n"'

cargo run -q -p pipeline-validator -- \
  --compiled-v3-source-text @/tmp/tui-vfx-command-capture.json \
  recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json
```

`recipe-source-capture` writes a `tui_vfx.command_capture.v1` artifact with ANSI
stdout, stripped text, stderr text, command metadata, and success status. Capture
is an offline authoring/tooling step. Runtime recipe playback must remain
deterministic and must not spawn commands.

### Generate and check docs

```bash
cd /usr/projects/tui-vfx
just docs-generate
just docs-check
just docs-validate
just docs-api
just docs-api-check
just docs-api-validate
```

These wrap `cargo xtask docs ...` and are the as-built entry points for generated
capabilities/API docs and drift validation. Recipe-side V3 tooling references
live in `/usr/projects/tui-vfx-recipes/docs/`.

## Focused guides

- [Probe database and frame diffs](probe-database-and-frame-diffs.md)
- [V3 preview and thin player surface](v3-preview-and-thin-player.md)
- [Grid resize adapter contract](grid-resize-adapter-contract.md)
- [Edge ingestion and capture tooling](edge-ingestion-and-capture.md)
- [Release-gate evidence records](release-gate-evidence.md)

## Existing detailed references

- [`../PIPELINE_VALIDATOR_LLM_GUIDE.md`](../PIPELINE_VALIDATOR_LLM_GUIDE.md)
- [`../PIPELINE_PROBE_LLM_GUIDE.md`](../PIPELINE_PROBE_LLM_GUIDE.md)
- [`../PIPELINE_TRACE_LLM_GUIDE.md`](../PIPELINE_TRACE_LLM_GUIDE.md)
- [`../RECIPE_AUTHORING_WORKFLOW.md`](../RECIPE_AUTHORING_WORKFLOW.md)
- [`./release-gate-evidence.md`](release-gate-evidence.md)
- [`../../../tui-vfx-recipes/docs/RECIPE_PROBE_GUIDE.md`](../../../tui-vfx-recipes/docs/RECIPE_PROBE_GUIDE.md)
- [`../../../tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md`](../../../tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md)
- [`../../../tui-vfx-recipes/docs/V3_STANDALONE_PREVIEW_SURFACES.md`](../../../tui-vfx-recipes/docs/V3_STANDALONE_PREVIEW_SURFACES.md)

## Known gaps to document or close later

- `pipeline-probe --help` is not an advertised supported surface in this
  snapshot; use the command shape above and the probe guide.
- ANSI source ingestion is still an adapter/tooling lane. The offline command
  capture tool exists; broader ANSI-to-grid source ingestion needs a separate
  as-built guide when implemented.
- The packaged lightweight player is `tui-vfx-horseman` (package: `tui-vfx-horseman`). It remains a headless summary/corpus
  surface and should keep reusing the existing preview/cutover paths rather than
  becoming a second renderer or recipe interpreter.
- Frame/database evidence should continue to reuse probe reports and SQLite xray
  tables unless a concrete missing field forces a schema extension.

<!-- <FILE>docs/tooling/INDEX.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.3</VERS> -->
