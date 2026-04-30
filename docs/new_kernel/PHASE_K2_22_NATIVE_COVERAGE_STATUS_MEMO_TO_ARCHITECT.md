<!-- <FILE>docs/new_kernel/PHASE_K2_22_NATIVE_COVERAGE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.22 native coverage status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native coverage and simple filter expansion: report successful results first and identify next blockers.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.22 audit, lowerer expansion, UI oracle findings, and verification evidence.</CLOG> -->

# Successful native coverage results

## 1. SUCCESSFUL RESULTS

K2.22 converted the remaining “native coverage” discussion into a measured full-corpus audit and a concrete lowerer expansion.

Full audit command:

```bash
cd /usr/projects/tui-vfx && ./scripts/k222_native_coverage_audit.sh
```

Current result:

```text
recipes=144 nativePasses=61 fallbacks=83 hardErrors=0
```

Implemented native filter lowerers:

- `filter.invert`
- `filter.greyscale`
- `filter.fadeToCanvas`
- `filter.crt`
- cleanly representable `filter.vignette`

Coverage moved from 47 native passes / 97 fallbacks to 61 native passes / 83 fallbacks.

## 2. USER-RUNNABLE COMMANDS

Native coverage audit:

```bash
cd /usr/projects/tui-vfx && ./scripts/k222_native_coverage_audit.sh
```

Animated native color playback still works through the K2.21 path:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend-timeline --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format ansi --samples 3 --no-clear
```

Strict native simple-filter proof:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_crt.json --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format json
```

## 3. WHAT WORKS NOW

- The audit harness produces full debug recipe pass/fallback/hard-error counts instead of anecdotal spot checks.
- Strict native mode proves added filters with `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, and `nativeSourceIsolated=true`.
- Auto mode remains useful for corpus audit because it records unsupported native blockers without stopping the whole run.
- Non-representable inputs are still honest blockers; for example, vignette recipes with authored color/progress aliases are not silently lowered.

## 4. UI ORACLE FINDINGS

The working `/usr/projects/tui-vfx-recipes/examples/demo.rs` oracle confirms the next public-demo UI blockers:

1. Interactive studio panel is missing; controls are generated and scriptable, but not navigable/editable in the ratatui UI.
2. Playback loop is not lifecycle/timing aware enough for a polished public demo.
3. Help overlay should intercept input instead of allowing hidden mutations.
4. Status should show FPS/frame time, composition mode, fallback/native/source mode, and stable user messages.
5. Reload should re-read active recipe JSON from disk.

## 5. HIGHER-LEVEL BLOCKERS TO FORWARD MOMENTUM

The blockers are concrete now:

1. **Content transform strategy is the biggest unlock.** Content blocks 33 recipes. The next implementation packet should target content rendering/lowering rather than continuing to pick off single-count filters.
2. **Interactive Studio is the visible product gap.** Generated controls exist; the missing work is focus, control navigation, editing, dirty state, and live render feedback.
3. **Lifecycle playback must become demo-grade.** The current loop can animate sampled compositor output, but public player behavior needs lifecycle timing, reload, stable messages, and help/status ergonomics borrowed from the working demo oracle.
4. **Remaining native blockers need no-silent-fallback contracts.** Any family expansion must either map semantics faithfully or leave the recipe unsupported with exact diagnostics.

## 6. VERIFICATION MATRIX

Fresh evidence collected:

- `cargo fmt --package tui-vfx-player --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli --package tui-vfx-player-ui -- --check` — PASS.
- `cargo check -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui` — PASS.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets -- -D warnings` — PASS.
- `cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast` — PASS, 123/123.
- `./scripts/k222_native_coverage_audit.sh` — PASS, 144 recipes / 61 native passes / 83 fallbacks / 0 hard errors.
- `git diff --check` — PASS.
- `cargo xtask docs check` — PASS with pre-existing warnings.
- `cargo xtask docs api-check` — PASS.
- `cargo xtask docs api-validate` — PASS.
- `cargo xtask audit configschema` — PASS.

## 7. RECOMMENDED NEXT PACKET

Create a content-transform native strategy packet:

1. Audit `content.*` player adapters and compositor primitives to decide whether content transforms should lower before source IR, as native compositor filters, or as a dedicated backend stage.
2. Implement the first high-count content family (`content.typewriter`, `content.splitFlap`, or `content.odometer`) with strict source-only native evidence.
3. In parallel, start the interactive Studio UI packet: focus state, panel rendering, control navigation, and editing for boolean/enum/number/color/text controls.

<!-- <FILE>docs/new_kernel/PHASE_K2_22_NATIVE_COVERAGE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.22 native coverage status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
