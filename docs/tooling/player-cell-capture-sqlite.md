<!-- <FILE>docs/tooling/player-cell-capture-sqlite.md</FILE> - <DESC>v3.1 player cell-capture SQLite guide.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Document the v3.1-owned dense frame/cell capture command for deterministic playback validation.</WCTX> -->
<!-- <CLOG>0.2.0: add recipe-clock sample fields to frame evidence.
0.1.0: initial as-built command, schema, and validation-flow notes for capture-cells.</CLOG> -->

# Player cell-capture SQLite

`capture-cells` is the v3.1-owned dense playback capture command. It samples one
canonical recipe through the contract-native player render-IR path and writes
every cell of every sampled frame to an on-disk SQLite database. Use it when a
playback difference needs exact frame/row/column localization instead of sparse
JSON inspection.

This command complements the older probe SQLite xray. The older probe path is
still the source for direct engine/probe causation tables; `capture-cells` is the
player-side surface for canonical v3.1 recipe playback evidence.

## Command

```bash
cd /usr/projects/tui-vfx
cargo run -q -p tui-vfx-player-cli -- capture-cells \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --sqlite-output /tmp/tui-vfx-player-capture.sqlite \
  --frames 3 \
  --sample-ms 250
```

The command prints a JSON summary with schema
`v3.1.player.cellCapture.1`. The SQLite file is replaced on each run so repeated
validation commands do not mix old rows with new playback evidence.

## SQLite schema

The database schema version is stored in `player_capture_runs.schema_version` as
`v3.1.player.cellCapture.sqlite.1`.

Primary tables:

- `player_capture_runs` — one row for the invocation: recipe id/path, frame
  count, total cell count, descriptor pack arguments, and SQLite output path.
- `player_capture_frames` — one row per sampled frame: frame index, phase,
  phase progress, optional loop progress, recipe clock mode, optional clock
  period, optional absolute elapsed milliseconds, optional sample duration,
  dimensions, render hash, status, row JSON, and diagnostic counts.
- `player_capture_cells` — dense row-major cells for every frame: recipe id,
  frame index, row, column, glyph, foreground, background, modifiers JSON,
  semantic role when available, reserved scene/source/layer columns (currently
  null unless a future render-IR cell-to-element map is added), and `style_known`.
- `player_capture_diagnostics` — player errors and warnings with severity,
  code, path, message, hint, and structured error details.
- `player_capture_provenance` — scene element/source provenance from render IR.
- `player_capture_layers` — scene element visibility/skip decisions.
- `player_capture_graph_values` — graph value snapshots serialized as JSON.

Example localization queries:

```sql
select frame_index, row, col, glyph, foreground, background, role
from player_capture_cells
where glyph != ' '
order by frame_index, row, col;

select frame_index, render_hash, non_empty_cells
from player_capture_frames
order by frame_index;

select frame_index, sample_ms, phase_t, loop_t, clock_mode, clock_period_ms, absolute_t_ms
from player_capture_frames
order by frame_index;

select severity, code, path, message
from player_capture_diagnostics
order by frame_index, severity, code;
```

## Validation flow fit

A deterministic vertical validation run can now use this order:

1. Validate the recipe contract and descriptor coverage (`fixture-qc` or the
   recipe-side validator gates).
2. Render a single frame or backend sample for quick status (`render-ir`,
   `render-backend`, or `render-backend-timeline`).
3. Run `capture-cells` for dense evidence when a mismatch needs exact frame and
   cell coordinates.
4. Compare SQLite rows by `(recipe_id, frame_index, row, col)` to localize glyph,
   style/color, and role differences; use the provenance/layer tables to inspect
   frame-level scene/source context and diagnostics.

The command intentionally uses v3.1 player vocabulary in command/table names.
Mentions of older probe tooling in this guide are source-context only, not new
product naming.

<!-- <FILE>docs/tooling/player-cell-capture-sqlite.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
