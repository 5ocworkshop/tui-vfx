<!-- <FILE>docs/new_kernel/PHASE_K2_20_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.20 review and de-slop report</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Native compositor lowering: track formal review and de-slop status.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — record formal AI de-slop pass, safe simplifications, and verification evidence.
0.1.0: INIT — seed review/de-slop report; final verification updates pending.</CLOG> -->

# K2.20 review and de-slop report

## Status

Formal AI de-slop pass completed for the K2.20 changed-file lane covering player request/options/output types, compositor native lowering/render adapter, CLI/UI mode and studio wiring, the native demo script, and K2.20 result docs.

## Review findings

- No behavior-changing cleanup was required in public request/options/output fields.
- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs` no longer uses a one-method private trait solely to patch evidence counts after construction; evidence is built directly with the final count values.
- `crates/tui-vfx-player-ui/src/cls_player_ui_state.rs` now keeps its metadata footer at the physical end of the source file.
- The compositor lowerer remains large for an OFPF-prefixed `fnc_` file. It was not split during this pass because doing so would broaden the review-only cleanup scope and risk conflicts with concurrent work.

## De-slop actions

- Removed the private `EvidenceCounts` trait and `.with_counts(...)` post-processing path.
- Preserved backend evidence wire fields (`compositionMode`, `fallbackUsed`, `nativeLoweringAttempted`, `nativeLoweringSucceeded`, `compositionSpecNonEmpty`, `loweredNodeCount`, `loweredEffectIds`) without renaming them to transient packet vocabulary.
- Moved the UI state metadata footer after `normalize_key`, preserving source behavior while restoring file metadata shape.

## Post-deslop regression

- `cargo fmt --package tui-vfx-player --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli --package tui-vfx-player-ui -- --check` — PASS.
- `cargo check -p tui-vfx-player-backend-compositor -p tui-vfx-player-ui -p tui-vfx-player-cli -p tui-vfx-player` — PASS.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets --all-features -- -D warnings` — PASS.
- `cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-ui -p tui-vfx-player-cli --no-fail-fast` — PASS, 62 tests.
- `./scripts/k220_native_compositor_demo.sh` — PASS; results in `/tmp/k220-native-results`.
- `git diff --check` — PASS.
- `git -C /usr/projects/tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes` — clean for recipe roots.

## Remaining risks

- The native composition lowerer should be split by effect-family helpers in a dedicated cleanup packet if the owner wants strict OFPF file-size conformance before merge.

<!-- <FILE>docs/new_kernel/PHASE_K2_20_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.20 review and de-slop report</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
