<!-- <FILE>docs/new_kernel/K2_24_LIFECYCLE_CLOCK_RESULTS.md</FILE> - <DESC>K2.24 lifecycle clock increment results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Public-demo playback increment: use recipe lifecycle durations for UI ticking.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture lifecycle-duration-aware UI ticking evidence.</CLOG> -->

# K2.24 lifecycle clock results

## Successful results

The player UI ticking path now uses the active recipe lifecycle phase duration instead of a hard-coded one-second loop.

- Fixed enter/exit phases use their `PhaseTiming::Fixed` duration.
- Fixed dwell phases use their `DwellPolicy::Fixed` duration.
- Trigger-terminated dwell phases use `maxDuration` when present.
- Recipes without usable timing keep the safe one-second fallback.
- Pause and motion-disabled behavior still returns a stable sample without ticking.

The first regression uses `baseline.json`, whose dwell duration is 5000ms. One script `tick` advances 100ms and now yields `sample_t: 0.02`, proving the UI clock is reading recipe lifecycle timing rather than looping every 1000ms.

## Verification evidence

- `cargo nextest run -p tui-vfx-player-ui --test test_fnc_player_ui test_fnc_ui_tick_uses_recipe_lifecycle_phase_duration` — PASS, 1/1.
- `cargo check -p tui-vfx-player-ui` — PASS.
- `cargo clippy -p tui-vfx-player-ui --all-targets -- -D warnings` — PASS.
- `cargo nextest run -p tui-vfx-player-ui --no-fail-fast` — PASS, 18/18.

<!-- <FILE>docs/new_kernel/K2_24_LIFECYCLE_CLOCK_RESULTS.md</FILE> - <DESC>K2.24 lifecycle clock increment results</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
