<!-- <FILE>docs/tooling/INDEX.md</FILE> - <DESC>Tooling documentation index for tui-vfx and tui-vfx-recipes.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Seed a first-class tooling documentation area for V3 preview, probe, validator, edge-ingestion, and adapter workflows.</WCTX> -->
<!-- <CLOG>0.1.0: initial tooling docs hub with ownership map and links to probe/database/diff, preview/player, resize, and edge-ingestion guidance.</CLOG> -->

# Tooling documentation

This directory is the stable entry point for tui-vfx tooling. It explains which
command or adapter to use before sending readers to the deeper command guides in
`tui-vfx` and `tui-vfx-recipes`.

## Ownership map

| Need | Use | Notes |
|---|---|---|
| Human visual sign-off | `tui-vfx-recipes/examples/demo.rs` or `examples/play_recipe.rs` | Interactive ratatui preview surfaces. |
| V3 recipe inspection | `tui-vfx-recipes/examples/v3_play_recipe.rs` | Minimal V3 loader/normalizer/render-hash example. Good thin-player starting point. |
| Recipe validation | `tui-vfx-recipes/tools/pipeline-validator` | Parse, rules, strict contracts, normalized IR, lowering reports, migration equivalence. |
| Recipe probe reports | `tui-vfx-recipes/tools/recipe-probe` or `pipeline-validator --probe` | Recipe-aware reports, timelines, frame diffs, SQLite xray. |
| Direct engine scene probes | `tui-vfx-probe --bin pipeline-probe` | Engine-level `ProbeSceneSpec` reports. Not recipe-aware. |
| Unified trace capture | `tui-vfx-recipes/tools/tui-vfx-trace` | Lifecycle/resolution/composition/pipeline trace evidence. |
| Edge ingestion/adapters | Planned Chapter 63 lane | ANSI capture, command-output capture, and thin runtime adapters normalize into grid/source contracts. |

## Guides

- [Probe database and frame diffs](probe-database-and-frame-diffs.md)
- [V3 preview and thin player surface](v3-preview-and-thin-player.md)
- [Grid resize adapter contract](grid-resize-adapter-contract.md)
- [Edge ingestion and capture tooling](edge-ingestion-and-capture.md)

## Existing detailed references

- `../PIPELINE_VALIDATOR_LLM_GUIDE.md`
- `../PIPELINE_PROBE_LLM_GUIDE.md`
- `../PIPELINE_TRACE_LLM_GUIDE.md`
- `../RECIPE_AUTHORING_WORKFLOW.md`
- `../../tui-vfx-recipes/docs/RECIPE_PROBE_GUIDE.md`
- `../../tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md`

<!-- <FILE>docs/tooling/INDEX.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
