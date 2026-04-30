# K2.16 player render IR report

## Implemented

K2.16 introduces a player-owned render IR surface with schema label `v3.1.player.renderIr.1`.

The IR carries:

- sampled clock fields (`phase`, `phaseT`, `loopT`);
- final text rows and existing render hash;
- sparse styled-cell evidence;
- scene/source/layer provenance for recipe scene elements;
- graph value snapshots after graph execution;
- player errors and warnings.

## Code surface

```text
crates/tui-vfx-player/src/cls_player_render_ir.rs
crates/tui-vfx-player/src/fnc_build_player_render_ir.rs
crates/tui-vfx-player/src/fnc_render_recipe_file_ir.rs
crates/tui-vfx-player/src/cls_recipe_player.rs
crates/tui-vfx-player-cli/src/fnc_run_render_ir.rs
```

## CLI

```bash
cargo run -q -p tui-vfx-player-cli -- render-ir \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json /path/to/recipe.json
```

The CLI is a thin shim over player-owned APIs and does not import compositor or UI internals.

## Verification

```text
cargo nextest run -p tui-vfx-player test_fnc_player_render_ir_carries_rows_styles_provenance_and_graph_values --no-fail-fast: pass
cargo nextest run -p tui-vfx-player-cli test_fnc_cli_renders_single_recipe_render_ir_json --no-fail-fast: pass
```
