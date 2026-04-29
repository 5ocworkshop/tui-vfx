<!-- <FILE>docs/new_kernel/PHASE_K1_STATUS.md</FILE> - <DESC>Concise Phase K1 visual player UI status</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase K1 wrap status with ratatui and local fast-fs navigation.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — record ratatui/crossterm UI and just recipes.
0.1.0: INIT — summarize K1 UI implementation and verification status.</CLOG> -->

# Phase K1 Status — Basic Visual Player UI on K0

## Status

K1 adds `crates/tui-vfx-player-ui/`, a ratatui/crossterm terminal inspection UI layered on the K0 player library. It is not compositor-backed playback yet.

## Main points

- Interactive UI uses the local `/usr/projects/fast-libraries/crates/fast-fs` browser pattern from `tui-vfx-recipes/examples/demo.rs` for recipe navigation and file selection.
- Rendering remains contract-native through K0 `tui-vfx-player` snapshot reports; there is no old recipe runtime dependency and no compositor output yet.
- `just player-ui` launches the interactive baseline recipe UI.
- `just player-ui-once` renders a deterministic text snapshot for quick smoke checks.
- Script/one-shot modes remain available for regression tests.

## Controls

`q`, `?`, `Tab`, browser arrows/`j`/`k`, `Enter`/`Right`, `Left`/`Backspace`, `R`, preview `space`, `r`, `m`, `[`/`]`, `left`/`right`, and `t` are supported.

## Evidence files

- `docs/new_kernel/K1_PLAYER_UI_STATUS.md`
- `docs/new_kernel/PHASE_K1_STATUS_MEMO_TO_ARCHITECT.md`
- `crates/tui-vfx-player-ui/`

<!-- <FILE>docs/new_kernel/PHASE_K1_STATUS.md</FILE> - <DESC>Concise Phase K1 visual player UI status</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
