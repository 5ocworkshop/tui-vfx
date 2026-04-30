<!-- <FILE>docs/new_kernel/PHASE_K2_29_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>Formal review and AI de-slop report for vignette/mask native blockers</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Review and cleanup evidence for the v3.1 vignette/mask strict-native blocker tranche.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record briefing-first review, scoped de-slop, review-finding closure, and post-cleanup verification evidence.</CLOG> -->

# Vignette/mask native blocker review and de-slop report

## Result

The v3.1 vignette/mask native blocker closure passed scoped implementation, review iteration, and de-slop verification after fixing review findings.

Final coverage evidence:

```text
recipes=144 nativePasses=125 fallbacks=19 hardErrors=0
```

No known must-fix review findings remain after the final `filter.vignette applyTo` invalid-enum fix and the creation of this report.

## Formal review

Briefed review agents read `.omx/context/k229-subagent-briefing-latest.md` first and did not read `steering/ORCHESTRATION.md`.

Review results and closures:

- Verified all 8 target recipes render in strict native mode with `fallbackUsed=false`.
- Verified native coverage audit reports `125` native passes, `19` fallbacks, and `0` hard errors.
- Requested strict invalid-enum rejection instead of defaulting unsupported authored values.
  - Closure: invalid enum regressions now cover `filter.vignette applyTo`, `mask.blinds orientation`, `mask.iris shape`, and `mask.pathReveal direction`.
- Requested real semantic evidence for direct masks rather than metadata-only assertions.
  - Closure: non-isomorphic masks now use source-owned content stages and the success regression asserts native-vs-`irResolved` `rows` and `styledCells` parity for all K2.29 targets.
- Requested this missing review/de-slop report.
  - Closure: this file records review, cleanup, verification evidence, and remaining risks.

## AI de-slop pass

Briefed cleanup agents read `.omx/context/k229-subagent-briefing-latest.md` first and did not read `steering/ORCHESTRATION.md`.

Simplifications made:

- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs`
  - clarified vignette mixed-field checks,
  - extracted source-style input locals,
  - removed a single-use unsupported-mask wrapper after `mask.none` remained the only direct native mask in this packet,
  - kept strict enum handling explicit at lowering time.
- `crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs`
  - hoisted stable cellular mask threshold calculation,
  - mirrored existing player primitive formulas in source-owned mask helpers to preserve parity.
- `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`
  - removed redundant parity gating because every target now requires parity,
  - added invalid-enum rejection coverage.

## Verification evidence

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
  test_fnc_cli_renders_compositor_backend_native_vignette_mask_blockers_json \
  test_fnc_cli_rejects_native_vignette_mask_blocker_invalid_enum_values_json \
  test_fnc_cli_rejects_native_vignette_mask_blocker_unsupported_shapes_json \
  --no-fail-fast
# PASS: 3 tests run, 3 passed, 68 skipped
```

```text
cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-fail-fast
# PASS: 73 tests run, 73 passed, 0 skipped
```

```text
./scripts/k222_native_coverage_audit.sh
# PASS: recipes=144 nativePasses=125 fallbacks=19 hardErrors=0
```

```text
cargo xtask docs check
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema
cargo doc -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-deps
# PASS: generated docs/API/configschema/rustdoc gates passed.
# Existing unrelated docs warnings remain for filters.ScalarFieldGlyph, filters.GlyphStyle, and shaders.Highlighter.
```

## Remaining risks

- Source-owned mask stages intentionally preserve current player-visible debug fixture semantics; they do not claim that the direct compositor mask contracts are semantically identical for every future authored field combination.
- Remaining native audit blockers are outside this packet: `mask.radial`, `mask.wipeCorner`, `sampler.crt`, `sampler.crtJitter`, `shader.barberPole`, `shader.diffusion`, `shader.focusField`, and `shader.glistenBand`.
