<!-- <FILE>crates/tui-vfx-probe/README.md</FILE> - <DESC>README for the engine-owned pipeline probe crate</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>Embedded SQLite query backend documentation</WCTX> -->
<!-- <CLOG>MINOR: Add the embedded SQLite query workflow to the crate README so callers can xray large playback datasets with SQL</CLOG> -->

# tui-vfx-probe

`tui-vfx-probe` is the engine-owned observability crate for `tui-vfx`.
It lets an LLM or human inspect one rendered frame as structured JSON instead of scraping prose output.

## What phase 1 supports

- direct engine input via `ProbeSceneSpec`
- one-frame execution through `run_probe()`
- multi-frame timelines through `collect_timeline()` / `--frames N`
- frame diffs through `run_probe_diff()` / `--diff-to T`
- in-memory SQLite indexing and ad-hoc SQL queries through `ProbeSqliteStore` / `--sqlite-query`
- JSON / NDJSON output through `pipeline-probe`
- cell selectors: `all`, `non-empty`, `modified`
- widget/frame metadata
- summary counts
- compositor-stage `last_touch` attribution
- richer compositor trace emission for sampler/mask/shader/filter events

## What phase 1 does not support yet

- style/content stage hooks
- full engine-wide causation beyond compositor callbacks
- recipe adapter delegation

## CLI quick start

```bash
cargo run -q -p tui-vfx-probe --bin pipeline-probe -- \
  --input probe-scene.json \
  --format json \
  --phase dwelling \
  --sample-t 0.5 \
  --cells modified \
  --with-causation
```

## Input shape

The CLI accepts a `ProbeSceneSpec` JSON document:

- `source` — widget-local source grid
- `destination` — destination frame before rendering
- `widget_offset` — widget placement in the destination frame
- `composition` — serialized `CompositionSpec`

See `docs/PIPELINE_PROBE_LLM_GUIDE.md` for full debugging workflows, timeline/diff examples, SQLite query examples, and output-reading guidance for LLMs and humans.

<!-- <FILE>crates/tui-vfx-probe/README.md</FILE> - <DESC>README for the engine-owned pipeline probe crate</DESC> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
