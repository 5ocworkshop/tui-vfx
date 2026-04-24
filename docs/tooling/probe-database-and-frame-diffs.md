<!-- <FILE>docs/tooling/probe-database-and-frame-diffs.md</FILE> - <DESC>As-built guide for probe reports, frame diffs, and SQLite xray tooling.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Consolidate the existing probe database and frame-diff surfaces so new tooling work reuses them instead of duplicating them.</WCTX> -->
<!-- <CLOG>0.1.0: initial as-built map for recipe-probe, pipeline-validator --probe, pipeline-probe, frame diffs, and SQLite query tables.</CLOG> -->

# Probe database and frame diffs

The project already has frame-diff and database-backed inspection tooling. New
preview/player work should reuse these surfaces instead of creating another diff
format.

## Recipe-aware path

Use `recipe-probe` directly, or `pipeline-validator --probe` when validation and
probe evidence should be part of one command flow.

```bash
cd /usr/projects/tui-vfx-recipes
cargo run -q -p recipe-probe -- \
  recipes/debug_recipes/shaders/_DEPRECATED_shader_orbit.json \
  --phase dwelling \
  --sample-t 0.0 \
  --diff-to 0.5 \
  --with-causation \
  --sqlite-query "select count(*) as changed_cells from probe_diff_cells"
```

Use this path when the input is a recipe and the question is about recipe
semantics, recipe stages, configured effect instances, bindings, or debug recipe
quality.

## Direct engine scene path

Use `pipeline-probe` for direct `ProbeSceneSpec` inputs. This is engine-level
inspection, not recipe loading.

```bash
cd /usr/projects/tui-vfx
cargo run -q -p tui-vfx-probe --bin pipeline-probe -- \
  --input scene.json \
  --phase dwelling \
  --sample-t 0.0 \
  --diff-to 0.5 \
  --format json
```

## Database xray

Probe reports can be materialized into an in-memory SQLite database with
`--sqlite-query`. The useful tables include:

- `probe_diff_cells`
- `probe_trace_events`
- `probe_analysis_stages`
- `probe_analysis_effects`
- `probe_analysis_combined`
- `probe_diagnostics`
- `probe_motion_effects`
- `probe_runtime_params`
- `probe_binding_resolutions`
- `probe_cell_root_causes`

Prefer SQL when answering questions about large reports, repeated effect
instances, or frame-to-frame motion. It keeps the evidence machine-readable and
avoids grepping dense JSON.

## Contract for future tooling

A thin player, CI visual regression command, or movie-layer prototype should:

1. render frames through the canonical recipe/build path,
2. emit or hand off frame snapshots in the existing probe shape when possible,
3. reuse `--diff-to`, `--frames`, and SQLite materialization for analysis,
4. avoid adding a second diff schema unless the existing report cannot represent
   the evidence.

<!-- <FILE>docs/tooling/probe-database-and-frame-diffs.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
