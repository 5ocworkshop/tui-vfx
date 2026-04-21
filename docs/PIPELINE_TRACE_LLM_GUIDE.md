<!-- <FILE>docs/PIPELINE_TRACE_LLM_GUIDE.md</FILE> - <DESC>How an LLM or user should use tui-vfx-trace to inspect recipe traces</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Sub-plan B Phase B.6 — add the LLM-facing companion guide for tui-vfx-trace so AI agents can choose the unified trace surface and understand the selector/stage/output grammar.</WCTX> -->
<!-- <CLOG>0.1.0: initial tui-vfx-trace guide covering tool selection, selector syntax, stage syntax, output formats, and schema references.</CLOG> -->

# Pipeline Trace: A Unified Recipe Trace Guide for LLMs and Humans

`tui-vfx-trace` is the recipe-side trace CLI in the sibling
`tui-vfx-recipes` repo at `tools/tui-vfx-trace/`.

Use it when you want the **full recipe-driven trace stream**:

- lifecycle events from `AnimationManager`
- resolution events from the scene composer
- composition events from layer painting
- pipeline events from compositor inspection

## When to use which tool

| Situation | Tool |
| --- | --- |
| You need parse/rules/profile validation for recipe JSON | `pipeline-validator` |
| You need structured probe reports or per-cell diffs | `recipe-probe` / `pipeline-validator --probe` |
| You already have engine-level `ProbeSceneSpec` inputs | `pipeline-probe` |
| You need the unified end-to-end event stream across every recipe stage | `tui-vfx-trace` |

## Basic usage

```bash
cargo run -q -p tui-vfx-trace -- \
  --recipe recipes/scenes/example_card.json \
  --frames 10 \
  --format ndjson \
  --output -
```

## Selectors

Repeat `--select` to OR selectors together:

- `all`
- `cell:x,y`
- `rect:x,y,width,height`
- `role:border`
- `layer:card_layer`
- `recipe:scene_example_card`

## Stages

Repeat `--stages` to OR stage groups together. Comma-delimited lists are
accepted:

- `lifecycle`
- `resolution`
- `composition`
- `pipeline`
- `all`
- `none`

Convenience aliases `sampler`, `mask`, `shader`, `filter`, and `shadow`
all map to the pipeline stage.

## Output formats

- `--format ndjson` — one serialized `TraceEnvelope` per line
- `--format report` — structured JSON summary with stage counts, layer
  aggregates, fallback counts, and lifecycle milestones

Use `--output -` for stdout. `BrokenPipe` is treated as success so
shell pipelines like `| head` or `| jq` stay quiet.

## Typical questions

### “Did my recipe emit composition events for the expected layer?”

```bash
cargo run -q -p tui-vfx-trace -- \
  --recipe recipes/scenes/example_card.json \
  --frames 5 \
  --select layer:card_layer \
  --stages composition \
  --format report
```

### “Show me only border-role events in NDJSON”

```bash
cargo run -q -p tui-vfx-trace -- \
  --recipe recipes/scenes/example_card.json \
  --frames 10 \
  --select role:border \
  --format ndjson \
  --output -
```

### “Restrict output to the first 100ms”

```bash
cargo run -q -p tui-vfx-trace -- \
  --recipe recipes/scenes/example_card.json \
  --frames 20 \
  --from-ms 0 \
  --to-ms 100 \
  --format ndjson \
  --output -
```

## Schema reference

The emitted event taxonomy and envelope fields are defined in
[TRACE_EVENT_SCHEMA.md](TRACE_EVENT_SCHEMA.md).

## See also

- [PIPELINE_VALIDATOR_LLM_GUIDE.md](PIPELINE_VALIDATOR_LLM_GUIDE.md)
- [PIPELINE_PROBE_LLM_GUIDE.md](PIPELINE_PROBE_LLM_GUIDE.md)
- `../tui-vfx-recipes/tools/tui-vfx-trace/README.md`

<!-- <FILE>docs/PIPELINE_TRACE_LLM_GUIDE.md</FILE> - <DESC>How an LLM or user should use tui-vfx-trace to inspect recipe traces</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
