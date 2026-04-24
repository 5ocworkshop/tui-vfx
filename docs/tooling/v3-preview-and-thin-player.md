<!-- <FILE>docs/tooling/v3-preview-and-thin-player.md</FILE> - <DESC>As-built V3 preview surfaces and thin-player direction.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Document the existing V3 preview/player surfaces and keep the future movie/thin-player lane aligned with the grid-first architecture.</WCTX> -->
<!-- <CLOG>0.1.0: initial guide mapping demo/play_recipe/v3_play_recipe/direct snapshots to the future thin-player boundary.</CLOG> -->

# V3 preview and thin player surface

`tui-vfx` is grid-first. `ratatui` is a consumer, not the core runtime. Preview
and player tooling should therefore load recipes, advance time, render grids, and
only then choose an output adapter.

## Existing surfaces

| Surface | Location | Use |
|---|---|---|
| Interactive browser | `/usr/projects/tui-vfx-recipes/examples/demo.rs` | Human browsing and visual sign-off. |
| Interactive single recipe | `/usr/projects/tui-vfx-recipes/examples/play_recipe.rs` | Human playback of one recipe. |
| Minimal V3 inspector | `/usr/projects/tui-vfx-recipes/examples/v3_play_recipe.rs` | Parse, normalize, compile, render deterministically, print render hash. |
| Direct V3 snapshot state | `/usr/projects/tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs` | Maintains sampled timing and rendered snapshot for supported V3 recipes. |
| Direct snapshot composer | `/usr/projects/tui-vfx-recipes/src/preview/fnc_render_direct_v3_snapshot.rs` | Composes a direct V3 snapshot into a ratatui buffer at the adapter boundary. |
| Diagnostic dump | `/usr/projects/tui-vfx-recipes/examples/diag_render_dump.rs` | Machine-friendly rendered cell dump for focused debugging. |

## Thin-player shape

The Chapter 90 movie-composer note names the future player as plausibly
`gtd-movie` or `tui-vfx-movie`. That layer should stay above recipes:

```text
script/scene list -> recipe loader -> shared clock/runtime bindings -> grid render -> output adapter
```

The first useful implementation does not need raw terminal mode. It can be a
small CLI that:

1. loads a recipe through the canonical V3 loader,
2. accepts frame times or a frame count,
3. renders each frame to the existing grid/snapshot type,
4. prints text, JSON summaries, or probe-compatible artifacts,
5. optionally sends pairs of frames to the existing frame-diff tooling.

## Boundary rules

- Do not make recipes own their clock.
- Do not make the compositor depend on ratatui, crossterm, a command runner, or a
  movie script parser.
- Keep terminal lifecycle and event handling in the host/player layer.
- Route recipe semantics through the canonical builder and preview/direct V3
  snapshot path. Do not create a parallel recipe interpreter.

<!-- <FILE>docs/tooling/v3-preview-and-thin-player.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
