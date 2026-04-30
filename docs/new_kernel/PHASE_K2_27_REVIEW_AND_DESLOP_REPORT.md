<!-- <FILE>docs/new_kernel/PHASE_K2_27_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>Formal review and AI de-slop report for residual style/content native blockers</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Review and cleanup evidence for the v3.1 residual strict-native style/content blocker tranche.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record briefing-first review, scoped de-slop, parity hardening, and post-cleanup verification evidence.</CLOG> -->

# Residual style/content native blocker review and de-slop report

## Result

The v3.1 residual style/content native blocker closure passed formal review after the required broken-link finding was fixed.

Final coverage evidence remains:

```text
recipes=144 nativePasses=109 fallbacks=35 hardErrors=0
```

No must-fix review findings remain.

## Formal review

Briefed review agent read `.omx/context/k227-subagent-briefing-latest.md` first and did not read `steering/ORCHESTRATION.md`.

Review result after fix: **APPROVED — code path sound; missing report/index link fixed by this document.**

High-level results:

- Verified all 10 target recipes moved to strict native.
- Verified native coverage audit reports `109` native passes, `35` fallbacks, and `0` hard errors.
- Verified native output matched `irResolved` rows and styled cells for all 10 target recipes at `phase_t=0.35`.
- Verified strict unsupported diagnostics for unknown inputs, graph outputs, unsupported content scopes, and unsupported style scopes.
- Verified durable names are descriptive and do not introduce transient packet shorthand.
- Requested one must-fix documentation issue: `docs/new_kernel/INDEX.md` linked this report before the file existed. This report closes that finding.

## AI de-slop pass

Briefed cleanup agent read `.omx/context/k227-subagent-briefing-latest.md` first and did not read `steering/ORCHESTRATION.md`.

Cleanup result: scoped edit applied.

Simplification made:

- `crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs`
  - changed the private style-cell helper to accept borrowed style labels instead of owned strings,
  - removed repeated per-cell `String` construction in modulo-column and neon-flicker style-stage application,
  - preserved emitted cell styles, metadata keys, public names, and behavior.

Reviewed but intentionally unchanged:

- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs`
  - further cleanup would risk diagnostic wording or lowerer behavior.
- `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`
  - table-driven regressions are explicit and useful.

## Post-review hardening

The review recommended making native-vs-IR parity executable instead of relying only on manual review. The target success regression now compares `rows` and `styledCells` between strict native and `irResolved` backend output for all 10 residual style/content recipes at `phase_t=0.35`.

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
  test_fnc_cli_renders_compositor_backend_native_residual_style_content_blockers_json \
  test_fnc_cli_rejects_native_residual_style_content_blocker_unsupported_shapes_json \
  --no-fail-fast
# PASS: 2 tests run, 2 passed
```

```text
cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-fail-fast
# PASS: 68 tests run, 68 passed, 0 skipped
```

```text
./scripts/k222_native_coverage_audit.sh
# PASS: recipes=144 nativePasses=109 fallbacks=35 hardErrors=0
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

- Unknown enum string rejection can be hardened in a later packet where strict-native mappings still intentionally mirror player defaults.
- Remaining audit blockers are one-off content/filter effects outside this packet's target set and should be handled in the next work packet.
