<!-- <FILE>docs/new_kernel/K1_PLAYER_UI_STATUS.md</FILE> - <DESC>Phase K1 visual player UI evidence</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase K1: document ratatui shell over K0 player path with demo.rs-style fast-fs navigation.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — replace std-only shell notes with ratatui/crossterm UI plus local fast-fs browser and just commands.
0.1.0: INIT — summarize UI command, controls, K0 reuse, and fixture coverage.</CLOG> -->

# K1 Basic Visual Player UI Status

## Deliverable

Phase K1 adds a ratatui/crossterm terminal inspection UI layered on K0, with recipe navigation powered by the local fast-fs checkout used by `tui-vfx-recipes/examples/demo.rs`:

```bash
just player-ui
```

Equivalent direct command:

```bash
cargo run -p tui-vfx-player-ui -- \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
```

For deterministic tests and non-interactive inspection:

```bash
just player-ui-once
cargo run -p tui-vfx-player-ui -- --once --no-clear <recipe.json>
cargo run -p tui-vfx-player-ui -- --script 'right,tick,t' --no-clear <recipe.json>
```

The descriptor pack is loaded through the K0 default primitive-pack path when no explicit pack is passed.

## Demo.rs reuse boundary

K1 follows the `demo.rs` navigation/file-loading portion, not the old recipe render model:

- `ratatui` for pane rendering
- `crossterm` raw-mode event loop and alternate screen
- local `fast-fs` path dependency at `/usr/projects/fast-libraries/crates/fast-fs`
- `fast_fs::nav::Browser`, `BrowserConfig`, `KeyInput`, and `ActionResult` for browser movement and file selection

K1 still renders only through K0 `tui-vfx-player` text-grid snapshots; it is not wired to `tui-vfx-compositor`, does not produce real ratatui `Buffer` effects, and does not import the old recipe runtime/player/probe.

## K0 reuse

The UI does not implement a second player engine. It calls:

- `tui_vfx_player::load_descriptor_catalog`
- `tui_vfx_player::RecipePlayer::new`
- `tui_vfx_player::PlayerSession::new/reset/render`
- `tui_vfx_player::PlayerSampleRequest`
- K0 `PlayerFrameReport` rows, hash, non-empty count, status, and diagnostics

## Supported controls

Global/browser controls:

- `q` quit
- `?` help
- `Tab` switch browser/preview focus
- browser `j/k` or arrows move
- browser `Enter` / `Right` open directory or JSON recipe
- browser `Left` / `Backspace` parent directory
- browser `R` refresh from disk

Preview controls:

- `space` pause/resume
- `r` reset session/runtime inputs
- `m` motion-disabled stable sample
- `[` / `]` phase cycle
- `left` / `right` sample_t scrub
- `t` fire canonical signal-backed dwell trigger when present

## Evidence shown by UI

The ratatui shell shows:

- browser root/current selection
- recipe path
- phase
- sample_t
- loop_t when present
- paused/motion-disabled state
- render hash
- non-empty cell count
- K0 rows in a bordered preview pane
- unsupported diagnostics and validation/render errors

## Fixture coverage

Manual one-shot/script smoke was run against:

- `baseline.json`
- `filters/filter_dim.json`
- `masks/mask_wipe.json`
- `samplers/sampler_sinewave.json`
- `event_driven_dwell/bool_binding_demo.json`

Unsupported diagnostic coverage is tested with:

- `shaders/primitives/shader_linear_gradient.json`

## Verification

Final K1 verification passed for package formatting, workspace clippy, workspace tests, recursive contract validation, recursive K0 render smoke, justfile one-shot launch, local fast-fs dependency use, forbidden runtime dependency guard, and recipe mutation guard. Full workspace formatting remains a known pre-existing issue outside K1 scope.

## Non-claims

K1 remains a basic visual inspection shell. It does not claim compositor-backed playback, old demo visual parity, rendered-frame parity, frame-diff tooling, legacy fallback loading, or new rendering semantics beyond K0.

<!-- <FILE>docs/new_kernel/K1_PLAYER_UI_STATUS.md</FILE> - <DESC>Phase K1 visual player UI evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
