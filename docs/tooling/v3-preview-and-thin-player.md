<!-- <FILE>docs/tooling/v3-preview-and-thin-player.md</FILE> - <DESC>As-built V3 preview surfaces and thin-player direction.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Document the existing V3 preview/player surfaces and keep the snapshot renderer boundary aligned with the grid-first architecture.</WCTX> -->
<!-- <CLOG>0.3.0: record render_v3_snapshot_onto_grid/render_v3_snapshot_onto_scene as the semantic snapshot renderers and ratatui as an adapter.</CLOG> -->

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
| Grid-first snapshot composer | `/usr/projects/tui-vfx-recipes/src/preview/fnc_render_direct_v3_snapshot.rs` | `render_v3_snapshot_onto_grid` / `render_v3_snapshot_onto_scene` compose a V3 frame snapshot onto framework-agnostic surfaces. |
| Ratatui snapshot adapter | `/usr/projects/tui-vfx-recipes/src/preview/fnc_render_direct_v3_snapshot.rs` | `render_v3_frame_to_buffer` converts the ratatui buffer region to an owned grid, delegates to the grid renderer, and writes back. |
| Diagnostic dump | `/usr/projects/tui-vfx-recipes/examples/diag_render_dump.rs` | Machine-friendly rendered cell dump for focused debugging. |
| Packaged thin player | `/usr/projects/tui-vfx-recipes/tools/tui-vfx-horseman` | Stable `tui-vfx-horseman` package with text and `--json` summary modes over the existing preview/cutover APIs. |

## Snapshot renderer boundary

The semantic snapshot renderer is `render_v3_snapshot_onto_grid` (or
`render_v3_snapshot_onto_scene` when role write-back matters). It accepts
`tui-vfx-types` grids/scenes and dispatches shadow composition by
`ShadowCompositeMode`:

- `GlyphOverlay` uses the compositor's shared glyph-overlay blend helper.
- `GradeUnderlying` preserves the destination glyph and grades foreground /
  background through the compositor's shared grade helper.

`render_v3_frame_to_buffer` is only the ratatui adapter. Python bindings,
crossterm adapters, headless validators, exporters, and test harnesses should
call the grid/scene renderer rather than reimplementing ratatui buffer logic.

## D2 packaged thin-player slice

The first packaged thin-player slice now exists in `tui-vfx-recipes` as the
`tui-vfx-horseman` workspace package. It intentionally stays small: it loads a
recipe through `preview_from_recipe_path_with_cutover_fallback` and emits a text
or JSON summary of the selected direct-V3 or legacy-preview playback seam.

```text
cargo run -q -p tui-vfx-horseman -- recipes/debug_recipes/content/content_slide_shift.json
cargo run -q -p tui-vfx-horseman -- recipes/debug_recipes/content/content_slide_shift.json --json
```

The broader tooling family remains split by responsibility:

```text
tui-vfx-horseman --json        -> stable packaged recipe playback summary
v3_play_recipe --json        -> direct V3 IR + render summary example
pipeline-validator --dump    -> machine-readable grid/frame dump
recipe-probe / --probe       -> probe, diff, and SQLite evidence
diag_render_dump             -> cell-level debug dump for render-path comparison
```

Future player work should stay narrow:

1. keep loading through the canonical V3 recipe/preview/probe helpers,
2. extend the packaged wrapper only when the underlying command already supports
   the requested text / JSON / grid / probe behavior,
3. leave terminal lifecycle and event handling in the host/player layer,
4. do not add `gtd-movie` timeline/composer semantics to `tui-vfx-horseman`,
5. preserve legacy/V2 fallback until the final V2 retirement gate.

## Boundary rules

- Do not make recipes own their clock.
- Do not make the compositor depend on ratatui, crossterm, a command runner, or a
  movie script parser.
- Keep terminal lifecycle and event handling in the host/player layer.
- Route recipe semantics through the canonical builder and preview/direct V3
  snapshot path. Do not create a parallel recipe interpreter.

<!-- <FILE>docs/tooling/v3-preview-and-thin-player.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
