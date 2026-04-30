<!-- <FILE>docs/new_kernel/PHASE_K2_23_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.23 review and de-slop report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Public-demo player/studio and native content-stage work: formal review, fix closure, de-slop, and verification evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.23 review findings, fixes, de-slop actions, and post-de-slop evidence.</CLOG> -->

# K2.23 review and de-slop report

## Successful results

- Formal review found no critical or high-severity issue; re-review passed after requested changes were fixed.
- Help overlay behavior is aligned with the working demo oracle: any non-quit key dismisses help without mutating hidden player state; `q` quits.
- UI compositor evidence test now asserts the fallback field explicitly: `fallback_used: false`.
- Studio keyboard edits now use effective current values, not stale generated control values. Boolean controls can toggle forward and back from the actual current value.
- Stale native coverage counts in K2.23 briefing and the K2.22 review report were synchronized with current audit evidence: 61 native passes / 83 fallbacks / 0 hard errors.
- Scoped de-slop pass made only small ownership/readability simplifications and did not widen behavior or schema scope.

## Formal review findings and fixes

1. **Stale audit counts in reviewed docs/context** — fixed.
   - Updated `.omx/context/k223-subagent-briefing-latest.md`.
   - Updated `docs/new_kernel/PHASE_K2_22_REVIEW_AND_DESLOP_REPORT.md`.
   - Current reproducible audit: `recipes=144 nativePasses=61 fallbacks=83 hardErrors=0`.

2. **Studio keyboard edits used stale displayed values** — fixed.
   - `PlayerUiState::set_control_value` now updates the selected control's displayed current value after mutation.
   - `PlayerUiState::mutate_studio_control_interactively` resolves effective values from runtime signal overrides, runtime input overrides, recipe fallbacks, or defaults before selecting the next value.
   - Added regression `test_fnc_ratatui_studio_keyboard_toggle_uses_effective_current_value`.

3. **Missing explicit fallback assertion in UI status test** — fixed.
   - `test_fnc_ui_compositor_status_reports_source_and_native_evidence` now asserts `fallback_used: false`.

## Re-review status

Final re-review verdict: PASS. The reviewer confirmed all requested changes were closed and no new blocking findings remained.

## AI de-slop actions

Scoped cleanup stayed inside K2.23 touched files:

- Removed an unnecessary backend-options clone during UI state initialization.
- Captured backend fallback/native checks before moving backend options into the compositor request, clarifying ownership.
- Imported `LoweredCompositionSpec` directly in the compositor backend to avoid a long fully-qualified type path.
- Avoided broad rewrites of UI rendering, control DTOs, or content-stage lowering after tests were green.

## Verification evidence

Post-review and post-de-slop gates:

- `cargo fmt --package tui-vfx-player --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli --package tui-vfx-player-ui -- --check` — PASS.
- `cargo check -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui` — PASS.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets -- -D warnings` — PASS.
- `cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast` — PASS, 130/130.
- `./scripts/k222_native_coverage_audit.sh` — PASS, 144 recipes, 61 native passes, 83 fallbacks, 0 hard errors.
- `./scripts/k221_source_isolated_native_demo.sh` — PASS.
- `git diff --check` — PASS.
- `cargo xtask docs check` — PASS with existing warnings for `filters.ScalarFieldGlyph`, `filters.GlyphStyle`, and `shaders.Highlighter` TOML/ai-hint gaps.
- `cargo xtask docs api-check` — PASS.
- `cargo xtask docs api-validate` — PASS.
- `cargo xtask audit configschema` — PASS.

## Remaining risks

- Public-demo lifecycle playback still needs a fuller recipe lifecycle clock rather than only the current simple elapsed phase loop.
- Studio editing now has visible controls, keyboard focus, effective-value toggling, and script mutations, but still lacks polished per-widget editors for every value kind.
- Native coverage remains blocked by content-transform and shader families, led by `content.splitFlap`, `content.odometer`, `content.cellMotion`, and `shader.revealWipe`.

<!-- <FILE>docs/new_kernel/PHASE_K2_23_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.23 review and de-slop report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
