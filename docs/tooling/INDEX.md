<!-- <FILE>docs/tooling/INDEX.md</FILE> - <DESC>Tooling documentation index for tui-vfx and tui-vfx-recipes.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Make the V3 tooling hub a command-first start page that maps the as-built validator, probe, diff/database, preview/player, resize, edge-ingestion, command-capture, trace, and docs-generation surfaces.</WCTX> -->
<!-- <CLOG>0.2.0: expand the tooling hub with concrete as-built command examples, verified tool boundaries, command-capture and docs-generation entries, and corrected sibling-repo cross-links. 0.1.0: initial tooling docs hub with ownership map and links to probe/database/diff, preview/player, resize, and edge-ingestion guidance.</CLOG> -->

# Tooling documentation

This is the start page for V3 tooling across `tui-vfx` and
`tui-vfx-recipes`. Use it to choose the existing command surface before opening
a deeper guide. Do not add a new tool until the existing validator, probe,
trace, preview/player, capture, or docs-generation surface cannot represent the
needed evidence.

## Quick command map

| Need | Existing command surface | Boundary |
|---|---|---|
| Validate recipe structure/rules | `cargo run -q -p pipeline-validator -- --rules <recipe.json>` from `/usr/projects/tui-vfx-recipes` | Recipe-aware validation. Keeps V2/V3 cutover behavior in the recipe repo. |
| Inspect normalized V3 IR | `pipeline-validator --dump-normalized --format json <recipe.json>` | Canonical normalized authoring shape. Not a renderer. |
| Inspect V3 lowering invariants | `pipeline-validator --lowering-report --format json <recipe.json>` | Machine-readable unresolved-lowering / human-review queue. |
| Check V2↔V3 migration pairs | `pipeline-validator --migration-equivalence-report --format json <manifest.json>` | Manifest-driven migration evidence, not owner audit. |
| Run debug fixture QC | `pipeline-validator --debug-recipes-qc --format json <recipe.json>` | Upstream-native structured QC over debug fixtures/concrete exports. |
| Recipe-to-scene probe | `cargo run -q -p recipe-probe -- <recipe.json> ...` | Recipe-aware adapter into the engine probe shape. |
| Validation plus probe in one flow | `pipeline-validator --probe ... <recipe.json>` | Validator owns recipe parsing/rules; delegated output is structured probe evidence. |
| Direct engine scene probe | `cargo run -q -p tui-vfx-probe --bin pipeline-probe -- --input <scene.json> ...` from `/usr/projects/tui-vfx` | Engine-level `ProbeSceneSpec`; not recipe-aware. |
| Frame diff and SQLite xray | `recipe-probe --diff-to <t> --sqlite-query '<sql>' <recipe.json>` | Reuse existing probe report/database shape. Do not invent a parallel diff format. |
| Unified lifecycle/trace evidence | `cargo run -q -p tui-vfx-trace -- --recipe <recipe.json> --format report` | Recipe trace CLI for lifecycle/resolution/composition/pipeline evidence. |
| Human preview/browser | `cargo run --example demo -- [recipe.json]` | Interactive ratatui browser and visual sign-off. |
| Fullscreen single-recipe preview | `cargo run --example play_recipe -- <recipe.json>` | Human isolation of one recipe; terminal lifecycle stays in the example/player. |
| Minimal V3 inspector/player | `cargo run --example v3_play_recipe -- <v3-recipe.json>` | Prints normalized IR and deterministic render metadata for supported V3 recipes. |
| Demo-path cell dump | `cargo run --example diag_render_dump -- <recipe.json> [dwell_seconds]` | Machine-readable cells through the same render path as the demo. |
| Resize contract evidence | `cargo run --example diag_resize_preserve_phase -- [recipe.json]` | Shows host-owned resize rerendering with preserved phase/time. No core resize loop. |
| Offline command-output capture | `cargo run -q -p recipe-source-capture -- --output <artifact.json> -- <cmd> ...` | Authoring/tooling capture only. Runtime recipe playback must not spawn commands. |
| Generated capabilities/API docs | `just docs-generate`, `just docs-check`, `just docs-validate`, `just docs-api`, `just docs-api-check`, `just docs-api-validate` from `/usr/projects/tui-vfx` | `xtask`/`just` pipeline for generated docs and drift checks. |

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

### Preview or inspect playback

```bash
cd /usr/projects/tui-vfx-recipes
cargo run --example demo -- recipes/debug_recipes/shaders/compositions/shader_barber_pole.json
cargo run --example play_recipe -- recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json
cargo run --example v3_play_recipe -- recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json
cargo run --example diag_render_dump -- recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json 1.0
```

Preview examples are host/player surfaces. They may own terminal lifecycle,
keyboard events, and ratatui buffers. They should not move those responsibilities
into the compositor or recipe schema.

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

## Existing detailed references

- [`../PIPELINE_VALIDATOR_LLM_GUIDE.md`](../PIPELINE_VALIDATOR_LLM_GUIDE.md)
- [`../PIPELINE_PROBE_LLM_GUIDE.md`](../PIPELINE_PROBE_LLM_GUIDE.md)
- [`../PIPELINE_TRACE_LLM_GUIDE.md`](../PIPELINE_TRACE_LLM_GUIDE.md)
- [`../RECIPE_AUTHORING_WORKFLOW.md`](../RECIPE_AUTHORING_WORKFLOW.md)
- [`../../../tui-vfx-recipes/docs/RECIPE_PROBE_GUIDE.md`](../../../tui-vfx-recipes/docs/RECIPE_PROBE_GUIDE.md)
- [`../../../tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md`](../../../tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md)
- [`../../../tui-vfx-recipes/docs/V3_STANDALONE_PREVIEW_SURFACES.md`](../../../tui-vfx-recipes/docs/V3_STANDALONE_PREVIEW_SURFACES.md)

## Known gaps to document or close later

- `pipeline-probe --help` is not an advertised supported surface in this
  snapshot; use the command shape above and the probe guide.
- ANSI source ingestion is still an adapter/tooling lane. The offline command
  capture tool exists; broader ANSI-to-grid source ingestion needs a separate
  as-built guide when implemented.
- The thin V3 player remains example-level (`v3_play_recipe`, direct V3 preview
  state, and diagnostic dumps). A packaged `tui-vfx-player` command should reuse
  those paths rather than inventing a recipe interpreter.
- Frame/database evidence should continue to reuse probe reports and SQLite xray
  tables unless a concrete missing field forces a schema extension.

<!-- <FILE>docs/tooling/INDEX.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
