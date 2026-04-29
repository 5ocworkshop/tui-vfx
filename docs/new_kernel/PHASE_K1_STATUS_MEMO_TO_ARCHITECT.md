<!-- <FILE>docs/new_kernel/PHASE_K1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase K1 status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase K1 architect update and next-assignment request.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — update memo for ratatui/crossterm shell, local fast-fs browser, and just commands.
0.1.0: INIT — report K1 visual UI implementation, fixture coverage, and preserved CLI behavior.</CLOG> -->

# Phase K1 Status Memo to Architect

## Phase

Phase K1 — Basic Visual Player UI on Top of K0

## Executive summary

K1 is implemented as a thin ratatui/crossterm terminal inspection UI over the K0 contract-native player. It does not replace the K0 CLI, does not create a second player engine, and is not compositor-backed playback yet.

The interactive command is now available through the project justfile:

```bash
just player-ui
```

Equivalent direct command:

```bash
cargo run -p tui-vfx-player-ui -- \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
```

For scripted/manual smoke evidence:

```bash
just player-ui-once
cargo run -p tui-vfx-player-ui -- --once --no-clear <recipe.json>
cargo run -p tui-vfx-player-ui -- --script 'right,tick,t' --no-clear <recipe.json>
```

## Demo.rs navigation/file-loading alignment

Per the correction that K1 should feel like `tui-vfx-recipes/examples/demo.rs`, the interactive shell now uses the same style of terminal stack for the browser/file-loading portion:

```text
ratatui
crossterm
fast_fs::nav::{Browser, BrowserConfig, KeyInput, ActionResult}
```

`fast-fs` is a local path dependency from:

```text
/usr/projects/fast-libraries/crates/fast-fs
```

The browser opens at the canonical v3.1 debug recipe root when the initial recipe is under that tree, supports demo-like keyboard navigation, and loads selected JSON files back through the same K0 player state.

## K0 APIs reused

The UI calls the K0 shared library path:

```text
tui_vfx_player::load_descriptor_catalog
tui_vfx_player::RecipePlayer::new
tui_vfx_player::PlayerSession::new/reset/render
tui_vfx_player::PlayerSampleRequest
K0 PlayerFrameReport rows/hash/non_empty_cells/status/errors
```

The UI owns only crossterm input routing, fast-fs browser state, ratatui presentation, and display state. The visible frame is K0 snapshot text displayed inside ratatui panes, not real compositor output.

## UI behavior added

The shell displays:

```text
browser root/current selection
recipe path
phase
sample_t
loop_t when present
paused / motion-disabled state
render_hash
non_empty_cells
K0 text-grid rows in a bordered preview pane
unsupported diagnostics and validation/render errors
```

Supported controls:

```text
q       quit
?       help
Tab     switch browser / preview focus
j/k     browser move
arrows  browser move or preview scrub, depending on focus
Enter   open directory or JSON recipe in browser
R       refresh browser from disk
space   pause / resume
r       reset session/runtime inputs
m       motion-disabled stable sample
[ / ]   previous / next phase
left    sample_t - 0.05 in preview, parent directory in browser
right   sample_t + 0.05 in preview, open in browser
t       fire canonical signal-backed dwell trigger
```

## Fixture coverage

Manual one-shot/script smoke was run against the requested canonical fixtures:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_sinewave.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/event_driven_dwell/bool_binding_demo.json
```

Unsupported diagnostic visibility was tested against:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient.json
```

## Event-trigger input

The UI detects a signal-backed dwell trigger from the canonical I0 lifecycle contract. Pressing/script-command `t` sets that signal to `true`, lets `PlayerSession` latch the trigger, and moves the next UI sample to `Exit` once K0 reports `dwellTerminated=true`.

The regression test covers the `userDismissed` signal in `event_driven_dwell/bool_binding_demo.json`.

## CLI preservation

K0 CLI behavior is preserved. The existing `tui-vfx-player-cli render-recipe` path remains unchanged.

## Verification evidence

Fresh verification on the final K1 tree:

```text
cargo fmt --package tui-vfx-player-ui -- --check      PASS
cargo clippy --workspace --all-targets -- -D warnings PASS
cargo test --workspace                                PASS
git diff --check                                      PASS
contract validate recursive v3.1 fixtures             PASS: total=16 valid=16 invalid=0
player render recursive v3.1 fixtures                 PASS: total=16 rendered=10 unsupported=6 errors=0
just player-ui-once                                   PASS: baseline snapshot renders
local fast-fs dependency guard                         PASS: /usr/projects/fast-libraries/crates/fast-fs
forbidden runtime dependency guard                     PASS: no tui-vfx-recipes/compositor/style/content/shadow/next under tui-vfx-player-ui
recipe repo mutation guard                             PASS: no changes under old or v3.1 recipe roots
```

Note: full `cargo fmt --all -- --check` still reports pre-existing workspace formatting diffs outside K1-owned files. I did not commit those unrelated formatting changes.

Ralph verifier result: APPROVED — no concrete K1 blockers found.

## Old recipe mutation

No old recipes under `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/` were modified. The canonical migrated fixture root also remains unmodified by K1.

## Remaining limitations

K1 is intentionally basic:

- no compositor-backed playback yet
- no old demo render/runtime reuse
- no visual parity claims
- no frame-diff tooling
- no new adapters beyond K0
- no legacy fallback loading

## Request

Please review K1 as the first human-facing ratatui inspection shell over the K0 player path and assign the next phase. The likely next branch is richer semantic frame output, first visual adapter expansion, or tightening the UI around whatever primitive family you want validated next.

<!-- <FILE>docs/new_kernel/PHASE_K1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase K1 status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
