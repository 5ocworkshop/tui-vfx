<!-- <FILE>docs/new_kernel/PHASE_K2_28_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>Formal review and AI de-slop report for one-off content/filter native blockers</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Review and cleanup evidence for the v3.1 one-off strict-native content/filter blocker tranche.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record briefing-first review, scoped de-slop, review-finding closure, and post-cleanup verification evidence.</CLOG> -->

# One-off content/filter native blocker review and de-slop report

## Result

The v3.1 one-off content/filter native blocker closure passed formal review after the required local Clippy-allow finding was fixed.

Final coverage evidence remains:

```text
recipes=144 nativePasses=117 fallbacks=27 hardErrors=0
```

No must-fix review findings remain.

## Formal review

Briefed review agent read `.omx/context/k228-subagent-briefing-latest.md` first and did not read `steering/ORCHESTRATION.md`.

Review result after fix: **APPROVED — target coverage, semantics, diagnostics, tests, docs, and audit movement are sound.**

High-level results:

- Verified all 8 target recipes moved to strict native.
- Verified native coverage audit reports `117` native passes, `27` fallbacks, and `0` hard errors.
- Verified native output is parity-locked against `irResolved` rows and styled cells for all 8 target recipes at `phase_t=0.35`.
- Verified `content.slideShift` native stage matches player row-shift primitive semantics.
- Verified target filter style stages mirror player primitive formulas for bracket emphasis, dot indicator, edge grow, hover bar, matrix rain, sub-pixel bar, and underline wipe.
- Verified strict unsupported diagnostics cover unknown inputs, graph outputs, unsupported content scopes, and unsupported filter style-stage scopes.
- Requested one must-fix issue: a local `#[allow(clippy::too_many_arguments)]` on the matrix-rain helper. The implementation now groups matrix-rain render inputs in a private parameter struct and removes the allow.

## AI de-slop pass

Briefed cleanup agent read `.omx/context/k228-subagent-briefing-latest.md` first and did not read `steering/ORCHESTRATION.md`.

Cleanup result: scoped edits applied.

Simplifications made:

- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs`
  - added small private helpers for repeated enum-label and color-label lowering,
  - reduced repeated one-off filter default/color boilerplate while preserving native output.
- `crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs`
  - reduced unnecessary string allocation in filter-stage helpers,
  - hoisted stable per-stage calculations outside inner loops,
  - reused stable default/transparent label constants,
  - consolidated modifier conversion,
  - replaced the local Clippy allow with a private matrix-rain input struct.
- `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`
  - left as a behavior lock; no simplification was worth weakening explicit parity coverage.

## Post-cleanup verification evidence

```text
git diff --check
# PASS
```

```text
cargo fmt --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli -- --check
cargo check -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli
cargo clippy -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --all-targets -- -D warnings
# PASS
```

```text
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_renders_compositor_backend_native_one_off_content_filter_blockers_json \
  test_fnc_cli_rejects_native_one_off_content_filter_blocker_unsupported_shapes_json \
  --no-fail-fast
# PASS: 2 tests run, 2 passed
```

```text
cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-fail-fast
# PASS: 70 tests run, 70 passed, 0 skipped
```

```text
./scripts/k222_native_coverage_audit.sh
# PASS: recipes=144 nativePasses=117 fallbacks=27 hardErrors=0
```

```text
cargo xtask docs check
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema
cargo doc -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-deps
# PASS: generated docs/API/configschema/rustdoc gates passed.
# Existing unrelated docs warnings remain for GlyphStyle, ScalarFieldGlyph, and Highlighter.
```

## Remaining risks

- The source-style stage route intentionally preserves player-visible fixture parity rather than claiming broader compositor filter contract coverage for every possible historical filter field combination.
- Remaining audit blockers are outside this packet's target set and should be handled in the next work packet.
