<!-- <FILE>docs/tooling/v3-preview-and-thin-player.md</FILE> - <DESC>As-built V3 preview surfaces and thin-player direction.</DESC> -->
<!-- <VERS>VERSION: 0.1.2</VERS> -->
<!-- <WCTX>Document the existing V3 preview/player surfaces and keep the future movie/thin-player lane aligned with the grid-first architecture.</WCTX> -->
<!-- <CLOG>0.1.2: map current V3 inspector, dump, probe, and thin-player wrapper responsibilities.</CLOG> -->

# V3 preview and thin player surface

`tui-vfx` is grid-first. `ratatui` is a consumer, not the core runtime. Preview
and player tooling should therefore load recipes, advance time, render grids, and
only then choose an output adapter.

## Existing surfaces

| Surface | Location | Use |
|---|---|---|
| Interactive browser | `/usr/projects/tui-vfx-recipes/examples/demo.rs` | Human browsing and visual sign-off. |
| Interactive single recipe | `/usr/projects/tui-vfx-recipes/examples/play_recipe.rs` | Human playback of one recipe. |
| Minimal V3 inspector | `/usr/projects/tui-vfx-recipes/examples/v3_play_recipe.rs` | Parse, normalize, compile, render deterministically, and emit either text or `--json` snapshot summaries from disk. |
| Machine-readable grid dump | `pipeline-validator --dump --stage output --format json` | Digestible frame/grid output from the canonical V3 output-dump path. |
| Probe / diff / SQLite | `recipe-probe` or `pipeline-validator --probe` | Probe-compatible structured evidence, timeline samples, diffs, and queryable probe tables. |
| Direct V3 snapshot state | `/usr/projects/tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs` | Maintains sampled timing and rendered snapshot for supported V3 recipes. |
| Direct snapshot composer | `/usr/projects/tui-vfx-recipes/src/preview/fnc_render_direct_v3_snapshot.rs` | Composes a direct V3 snapshot into a ratatui buffer at the adapter boundary. |
| Diagnostic dump | `/usr/projects/tui-vfx-recipes/examples/diag_render_dump.rs` | Machine-friendly rendered cell dump for focused debugging. |

## D2 next slice

The current as-built answer is split across existing surfaces, not a single
packaged player binary:

```text
v3_play_recipe --json        -> direct V3 IR + render summary
pipeline-validator --dump    -> machine-readable grid/frame dump
recipe-probe / --probe       -> probe, diff, and SQLite evidence
diag_render_dump             -> cell-level debug dump for render-path comparison
```

The next implementation slice should stay narrow:

1. keep loading through the canonical V3 recipe/preview/probe helpers,
2. package a thin wrapper around the existing surfaces instead of adding a new
   semantics path,
3. expose text / JSON / grid / probe output modes only where the underlying
   command already supports them,
4. leave terminal lifecycle and event handling in the host/player layer,
5. do **not** rename binaries yet; `tui-vfx-player` is the eventual packaging
   name, not the current cutover.

For today, the quickest agent-readable direct-V3 command remains:

```text
cargo run -q --example v3_play_recipe -- --json recipes/debug_recipes/shaders/primitives/shader_orbit.json
```

## Boundary rules

- Do not make recipes own their clock.
- Do not make the compositor depend on ratatui, crossterm, a command runner, or a
  movie script parser.
- Keep terminal lifecycle and event handling in the host/player layer.
- Route recipe semantics through the canonical builder and preview/direct V3
  snapshot path. Do not create a parallel recipe interpreter.

<!-- <FILE>docs/tooling/v3-preview-and-thin-player.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.2</VERS> -->
