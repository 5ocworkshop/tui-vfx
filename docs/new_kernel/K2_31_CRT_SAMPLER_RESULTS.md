<!-- <FILE>docs/new_kernel/K2_31_CRT_SAMPLER_RESULTS.md</FILE> - <DESC>CRT sampler native blocker closure results</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>v3.1 strict-native compositor backend coverage for CRT sampler debug recipes.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — record scoped AI de-slop review and replace process-heavy wording.
0.1.0: INIT — record CRT sampler native parity implementation and verification evidence.</CLOG> -->

# CRT sampler native blocker results

## Results first

The v3.1 compositor backend now renders the current CRT sampler debug fixtures in strict native mode without fallback:

- `samplers/sampler_crt.json` / `sampler.crt`
- `samplers/sampler_crt_jitter.json` / `sampler.crtJitter`

Native coverage moved from:

```text
recipes=144 nativePasses=127 fallbacks=17 hardErrors=0
```

to:

```text
recipes=144 nativePasses=129 fallbacks=15 hardErrors=0
```

The remaining top unsupported effects after this closure are shader/style blockers rather than CRT samplers:

```text
topUnsupported=shader.barberPole:1, shader.diffusion:1, shader.focusField:1, shader.glistenBand:1, shader.highlighter:1, shader.radar:1, shader.wayfindingNode:1, style.baseStyleOverride:1
```

## Implementation summary

The native backend uses backend-owned source content stages for both CRT samplers instead of lowering them to direct compositor `SamplerSpec` variants. That preserves the current player-visible semantics exactly:

- `sampler.crt` bows rows, applies time jitter, and applies odd-row scanline glyph dropout with the same thresholds as the player primitive.
- `sampler.crtJitter` shifts rows with the authored `amplitude`, `frequency`, and `seed` formula used by the player primitive.

Direct compositor sampler lowering was intentionally not used here because the available compositor primitives do not currently expose the same semantic contract as the v3.1 player fixtures.

## Guardrails added

CLI regression coverage now requires:

- strict-native success for both CRT sampler recipes,
- native-vs-`irResolved` `rows` parity at `phase_t=0.35`,
- native-vs-`irResolved` `styledCells` parity at `phase_t=0.35`,
- explicit fallback diagnostics for unsupported fields, graph outputs, and non-all scopes,
- player-compatible numeric clamping behavior for negative/out-of-range sampler inputs.

## Verification evidence

RED was observed before implementation:

```text
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli crt --no-fail-fast
4 tests run: 1 passed, 3 failed, 74 skipped
failures: unsupportedNativeEffect for strict-native CRT success/clamp cases
```

GREEN targeted verification after implementation:

```text
cargo fmt --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli -- --check
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli crt --no-fail-fast
4 tests run: 4 passed, 74 skipped
bash ./scripts/k222_native_coverage_audit.sh
recipes=144 nativePasses=129 fallbacks=15 hardErrors=0
```

## Scoped AI de-slop review

The scoped de-slop pass reviewed the CRT sampler lowering, rendering, CLI regressions, and K2.31 docs for duplication, confusing names, transient public vocabulary, and docs/test drift.

Cleanup applied:

- Replaced process-heavy wording in K2.31 durable docs and index entries with closure-plan/result vocabulary.

No Rust or test cleanup was applied: the new backend helper names mirror the source-owned player primitive semantics, and the CRT regression helpers remove more duplication than they add without changing behavior.

Post-cleanup verification stayed green:

```text
git diff --check
cargo fmt --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli -- --check
cargo check -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli
cargo clippy -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --all-targets -- -D warnings
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli crt --no-fail-fast
cargo nextest run -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-fail-fast
bash ./scripts/k222_native_coverage_audit.sh
cargo xtask docs check
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema
cargo doc -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli --no-deps
```

Outcomes: targeted CRT regressions `4 passed / 74 skipped`; package nextest `80 passed / 0 skipped`; audit `recipes=144 nativePasses=129 fallbacks=15 hardErrors=0`; docs/API/configschema/rustdoc gates passed. Existing unrelated docs warnings remain for `filters.ScalarFieldGlyph`, `filters.GlyphStyle`, and `shaders.Highlighter`.

## Remaining blockers

The next native-coverage closure effort should target the now-leading shader/style native blockers. These are serious because the active source corpus is the non-deprecated v3.1 `debug_recipes` hierarchy, so each remaining fallback is a baseline playback gap rather than an optional broad-corpus enhancement.
