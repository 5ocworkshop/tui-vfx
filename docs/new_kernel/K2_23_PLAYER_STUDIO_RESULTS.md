<!-- <FILE>docs/new_kernel/K2_23_PLAYER_STUDIO_RESULTS.md</FILE> - <DESC>K2.23 public-demo player and studio results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Public-demo player/studio: record help, reload, status, and interactive studio evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture UI foundation results and verification evidence.</CLOG> -->

# K2.23 public-demo player and studio results

## Successful results

The player UI now has the first public-demo foundation behaviors inspired by the working demo oracle:

- Help overlay is modal and oracle-aligned: any non-quit key dismisses it without mutating hidden player state; `q` quits.
- `r` / reset now re-reads the active recipe JSON from disk and resets volatile playback/runtime state.
- Snapshot and ratatui status expose compositor evidence: composition mode, fallback, native/source isolation, native lowering, lowered/unlowered counts, and backend hash.
- Studio mode renders a controls panel and supports keyboard mutation through Studio focus (`Enter`, `e`, or space) in addition to existing script/CLI `set` mutations.

## Verification evidence

- UI regressions passed inside the package lane:
  - `test_fnc_ratatui_help_overlay_dismisses_non_quit_input_without_state_mutation`
  - `test_fnc_ui_reset_command_reloads_active_recipe_from_disk`
  - `test_fnc_ui_compositor_status_reports_source_and_native_evidence`
  - `test_fnc_ratatui_studio_keyboard_mutation_changes_source_control`
  - `test_fnc_ratatui_studio_keyboard_toggle_uses_effective_current_value`
- Package lane: `cargo nextest run -p tui-vfx-player-ui` passed during the UI implementation pass.
- Integrated lane: `cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast` — PASS, 130/130.

## Remaining public-demo gaps

- Lifecycle playback is still only partially demo-grade; phase timing needs a fuller recipe lifecycle clock.
- Studio editing supports visible controls, keyboard mutation, and effective-current-value toggling, not yet polished per-widget editing for every value kind.
- Browser search/filter and flat recursive browsing from the oracle remain follow-up work.

<!-- <FILE>docs/new_kernel/K2_23_PLAYER_STUDIO_RESULTS.md</FILE> - <DESC>K2.23 public-demo player and studio results</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
