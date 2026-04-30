<!-- <FILE>docs/new_kernel/PHASE_K2_19_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.19 review and de-slop report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.19 visible playback: compositor backend and studio-control pilot evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.19 results, commands, artifacts, limits, and verification evidence.</CLOG> -->

# K2.19 review and de-slop report

## Review status

Initial implementation review is complete for changed files. The design keeps the intended boundary:

- player core owns `PlayerRenderIrReport`, `PlayerRenderBackend`, and backend-neutral output;
- `tui-vfx-player-backend-compositor` owns compositor/types lowering;
- CLI/UI select a backend and consume backend output;
- UI does not construct compositor DTOs.

## De-slop changes made

- Kept backend lowering functions small and named around durable concepts (`lower_player_ir_to_semantic_scene`, `render_compositor_backend`).
- Avoided transient packet shorthand in public variable or field names.
- Added explicit `playerIrAlreadyResolved` diagnostic rather than pretending direct effect-to-CompositionSpec lowering is complete.
- Used a script harness instead of hand-written artifact claims.
- Replaced the static-frame gap with a durable `play-backend` command and a `demo.rs`-style interactive UI frame loop.
- Kept CI playback deterministic by supporting JSON playback evidence and `--sample-ms 0 --no-clear` ANSI captures without weakening the human playback command.

## Verification evidence

- `cargo nextest run -p tui-vfx-player-cli test_fnc_cli_play_backend_json_finishes_before_ci_timeout test_fnc_cli_play_backend_ansi_emits_compositor_color_without_clear_when_no_clear test_fnc_cli_play_backend_rejects_zero_fps test_fnc_cli_play_backend_rejects_zero_duration --no-fail-fast`: PASS, 4/4.
- `cargo nextest run -p tui-vfx-player-cli test_fnc_cli_render_backend_timeline_preserves_sample_ms_json test_fnc_cli_play_backend_json_finishes_before_ci_timeout test_fnc_cli_play_backend_ansi_emits_compositor_color_without_clear_when_no_clear --no-fail-fast`: PASS, 3/3 after timeline `sampleMs` fix.
- `cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast`: PASS, 59/59.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets --all-features -- -D warnings`: PASS.
- `cargo nextest run --workspace --no-fail-fast`: PASS, 2872/2872.
- `./scripts/k219_visual_demo.sh`: PASS table, 20 artifacts generated, including `live_color_motion.play.ansi`.
- `/tmp/k219-visual-results/live_color_motion.play.ansi`: 5 unique backend hashes and truecolor ANSI evidence, proving color plus motion in one playback artifact.
- De-slop/code-review follow-up fixed the one evidence slop item: `render-backend-timeline --format json --sample-ms 250` now preserves `sampleMs: 250`, and the harness asserts timeline sample phases plus backend-hash variation.

## Remaining risks

- Direct `CompositionSpec` lowering is the next architecture blocker.
- Visual parity requires a comparator/oracle in a later packet.
