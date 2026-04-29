<!-- <FILE>docs/new_kernel/PHASE_K2_2_VISUAL_FRAME_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.2 visual frame substrate status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>New kernel Phase K2.2: summarize stable visual-frame evidence substrate.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — include review/de-slop report and phase-neutral recommendation cleanup.
0.1.0: INIT — document render-frame command, schema, counts, and verification.</CLOG> -->

# Phase K2.2 Visual Frame Substrate Status Memo to the v3.1 Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Packet: `docs/new_kernel/ARCH-RESP-TO-PHASE_K2_1.md` — Phase K2.2

## Executive summary

K2.2 adds a report-only visual-frame evidence substrate to `tui-vfx-player` and `tui-vfx-player-cli`.

The new command is additive:

```text
render-frame
```

It reuses the existing player `render_recipe_file` path and converts `PlayerFrameReport` values into a richer visual-frame report. Existing `render-recipe`, `inventory-recipes`, and `migration-gap` outputs remain preserved.

K2.2 does not wire the compositor, does not modify K1, does not modify recipes, and does not claim visual parity.

## New command shape

Single fixture:

```bash
cargo run -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
```

Recursive corpus:

```bash
cargo run -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

## New schema label

```text
v3.1.player.visualFrameReport.1
```

## Report shape

Top-level:

```text
schemaVersion
root
descriptorPacks
summary
frames[]
```

Frame entry:

```text
recipePath
status
phase
sampleT
loopT
absoluteTimeMs
substrate
cellSource
styleKnown
width
height
renderHash
nonEmptyCells
rows[]
cells[]
unsupportedEffectIds[]
errors[]
warnings[]
```

Sparse cell:

```text
x
y
glyph
foreground
background
modifiers[]
role
```

## Current counts

Recursive `render-frame` over canonical v3.1 debug recipes reports:

```text
schemaVersion=v3.1.player.visualFrameReport.1
total=16
rendered=10
unsupported=6
errors=0
```

The existing player/reporting commands remain consistent:

```text
render-recipe:     total=16 rendered=10 unsupported=6 errors=0
inventory-recipes: totalRecipes=16 rendered=10 unsupported=6 errors=0 descriptorEffectIds=14 representedEffectIds=14 unrepresentedEffectIds=0 unsupportedEffectIds=6
migration-gap:     legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7
```

## Files touched

Player library:

```text
crates/tui-vfx-player/src/lib.rs
crates/tui-vfx-player/src/cls_player_visual_cell.rs
crates/tui-vfx-player/src/cls_player_visual_frame.rs
crates/tui-vfx-player/src/cls_player_visual_frame_report.rs
crates/tui-vfx-player/src/fnc_build_visual_frame.rs
crates/tui-vfx-player/src/fnc_collect_unsupported_effect_ids.rs
crates/tui-vfx-player/src/fnc_collect_visual_cells.rs
crates/tui-vfx-player/src/fnc_extract_recipe_inventory_ids.rs
crates/tui-vfx-player/src/fnc_recommend_migration_queue.rs
crates/tui-vfx-player/src/fnc_render_visual_frame_paths.rs
crates/tui-vfx-player/src/fnc_summarize_visual_frames.rs
```

Player CLI/tests:

```text
crates/tui-vfx-player-cli/src/fnc_print_usage.rs
crates/tui-vfx-player-cli/src/fnc_run.rs
crates/tui-vfx-player-cli/src/fnc_run_render_frame.rs
crates/tui-vfx-player-cli/src/main.rs
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
```

Docs:

```text
docs/VOCABULARY.md
docs/new_kernel/K2_2_VISUAL_FRAME_EVIDENCE.md
docs/new_kernel/PHASE_K2_2_REVIEW_AND_DESLOP_REPORT.md
docs/new_kernel/PHASE_K2_2_VISUAL_FRAME_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md
```

## Captured JSON outputs

```text
/tmp/tui-vfx-k22-render-report.json
/tmp/tui-vfx-k22-inventory-report.json
/tmp/tui-vfx-k22-migration-gap-report.json
/tmp/tui-vfx-k22-baseline-frame.json
/tmp/tui-vfx-k22-visual-frame-report.json
```

## Verification results

```text
cargo fmt --package tui-vfx-player -- --check                         PASS
cargo fmt --package tui-vfx-player-cli -- --check                     PASS
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings  PASS
cargo test -p tui-vfx-player                                         PASS, 4 tests
cargo test -p tui-vfx-player-cli                                     PASS, 12 tests
cargo run -q -p tui-vfx-player-cli -- render-recipe --recursive ...   PASS, total=16 rendered=10 unsupported=6 errors=0
cargo run -q -p tui-vfx-player-cli -- inventory-recipes --recursive ... PASS, totalRecipes=16 rendered=10 unsupported=6 errors=0
cargo run -q -p tui-vfx-player-cli -- migration-gap ...               PASS, legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7
cargo run -q -p tui-vfx-player-cli -- render-frame baseline ...       PASS, schema=v3.1.player.visualFrameReport.1 rendered=1 substrate=textGrid cellSource=rows styleKnown=false cells=22
cargo run -q -p tui-vfx-player-cli -- render-frame --recursive ...    PASS, total=16 rendered=10 unsupported=6 errors=0
cargo test --workspace                                                PASS
git diff --check                                                      PASS
git -C /usr/projects/tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes  PASS, no output
```

## Notes and risks

- Sparse cells are derived from text-grid rows and are explicitly marked with `substrate: "textGrid"`, `cellSource: "rows"`, and `styleKnown: false`, so color/modifier/role fields are stable placeholders until a real visual surface exists.
- `absoluteTimeMs` is currently `0` because the current player samples normalized phase progress rather than wall-clock timelines.
- This is frame evidence, not visual parity.

<!-- <FILE>docs/new_kernel/PHASE_K2_2_VISUAL_FRAME_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.2 visual frame substrate status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
