<!-- <FILE>docs/new_kernel/PHASE_K2_21_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.21 review and de-slop report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Source-isolated native playback and generated studio controls: track formal review, fixes, de-slop, and regression evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture formal review findings, corrective results, de-slop actions, and verification evidence.</CLOG> -->

# K2.21 review and de-slop report

## Successful corrective results

Formal review initially requested changes. The K2.21 lane now has concrete fixes and passing regression evidence for each reviewed blocker:

- Unknown descriptor/runtime override keys are rejected instead of silently accepted.
- Generated studio controls expose richer public metadata: target kind, source/effect descriptor id, node/source instance context, current/default value, value/control kind, range, allowed values, optionality, mutability, and runtime-input address.
- Studio mutation evidence covers number, color, integer, boolean, and enum control families.
- Row-only source mutations now count as `changedCells` by comparing dense row and styled-cell maps.
- Native compositor mode now proves `sourceRenderMode=sourceOnly` and `nativeSourceIsolated=true`; `irResolved` remains explicitly post-effect with `sourceRenderMode=postEffectIr` and `playerIrAlreadyResolved`.

## Review findings and fixes

| Finding | Result |
| --- | --- |
| Bogus runtime override keys were accepted without effect. | Fixed in `crates/tui-vfx-player-cli/src/fnc_run_studio_snapshot.rs`; runtime assignments must resolve to known effect/source inputs or the studio snapshot returns an error. Regression: `test_fnc_cli_studio_snapshot_rejects_unknown_descriptor_runtime_override`. |
| Generated studio controls did not expose enough descriptor/source metadata. | Fixed in `crates/tui-vfx-player-ui/src/cls_player_ui_control.rs`, `crates/tui-vfx-player-ui/src/cls_player_ui_state.rs`, and `crates/tui-vfx-player-ui/src/fnc_render_ui_snapshot.rs`; controls now carry current/default/range/allowed/mutability/source/effect/runtime metadata. Regressions: UI descriptor/source control tests plus K2.21 harness `studio_descriptor_control_ui.txt`. |
| Harness did not prove all relevant control families. | Fixed in `scripts/k221_source_isolated_native_demo.sh`; evidence now includes signal number, runtime color, source integer, signal boolean, and runtime enum acceptance/no-visual-change diagnostics. |
| `changedCells` missed row-only mutations. | Fixed in `crates/tui-vfx-player-cli/src/fnc_run_studio_snapshot.rs`; diffing now uses dense cell maps from rows and styled cells. Regression: `test_fnc_cli_studio_snapshot_source_runtime_override_counts_row_changes`. |

## Formal re-review result

Sagan performed the final read-only K2.21 re-review after corrective fixes and returned PASS with zero blocking issues. The review re-ran or inspected the K2.21 harness, targeted nextest lane, fmt/check/clippy, diff whitespace validation, source-only native evidence, `irResolved` compatibility evidence, generated control metadata, and the studio mutation artifacts for number/color/integer/boolean/enum controls.

## AI de-slop actions

Scoped to changed K2.21 files only:

- Consolidated repeated compositor metadata insertion into `insert_common_spec_metadata` in `crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs`.
- Isolated studio `--set` parsing into `parse_set_assignment` in `crates/tui-vfx-player-cli/src/fnc_run_studio_snapshot.rs`.
- Centralized UI control target selection through `control_target` in `crates/tui-vfx-player-ui/src/cls_player_ui_state.rs`.
- Preserved durable public names (`sourceRenderMode`, `nativeSourceIsolated`, `runtimeInputOverride`) and avoided transient packet shorthand in runtime fields.

## Fresh verification evidence

- `cargo fmt --package tui-vfx-player --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli --package tui-vfx-player-ui -- --check` — PASS.
- `cargo check -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui` — PASS.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets -- -D warnings` — PASS.
- `cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast` — PASS, 122/122.
- `cargo nextest run --workspace --no-fail-fast` — PASS, 2883/2883; run ID `de1c3569-355c-450c-bf1a-7b76af6d91d9`.
- `./scripts/k221_source_isolated_native_demo.sh` — PASS; results in `/tmp/k221-source-native-results`.
- `git diff --check` — PASS.
- `cargo xtask docs check` — PASS with existing warnings for `filters.GlyphStyle`, `filters.ScalarFieldGlyph`, and `shaders.Highlighter` TOML/ai-hint gaps.
- `cargo xtask docs api-check` — PASS.
- `cargo xtask docs api-validate` — PASS.
- `cargo xtask audit configschema` — PASS.

## Remaining risks

- Native lowerer coverage is still intentionally bounded to the mapped effect families; remaining debug recipe lanes need family-by-family native support before claiming full player completeness.
- Source-local element pipeline semantics are source-isolated but not yet exhaustively audited across every source/effect interaction.
- Gradient editing is not yet a rich studio widget; current studio evidence covers primitive scalar/text/color/boolean/enum/integer controls.

<!-- <FILE>docs/new_kernel/PHASE_K2_21_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.21 review and de-slop report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
