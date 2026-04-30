<!-- <FILE>docs/new_kernel/PHASE_K2_21_SOURCE_ISOLATED_NATIVE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.21 source-isolated native status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Source-isolated native playback and descriptor studio controls: report successful results first and identify blockers.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.21 source-isolated native playback, studio control, review, and verification results.</CLOG> -->

# Successful source-isolated native results

## 1. SUCCESSFUL RESULTS

The K2.21 source-isolated native compositor harness passed:

```bash
cd /usr/projects/tui-vfx && ./scripts/k221_source_isolated_native_demo.sh
```

Evidence root: `/tmp/k221-source-native-results/`.

Results:

- 5 native compositor renders proved `sourceRenderMode=sourceOnly` and `nativeSourceIsolated=true`.
- 0 native fallbacks occurred under `--fail-on-fallback`.
- `irResolved` compatibility still reports `sourceRenderMode=postEffectIr` and `playerIrAlreadyResolved`.
- 3 native timelines changed backend hashes: `mask.wipe`, `sampler.sineWave`, and `shader.borderSweep`.
- Studio mutation evidence covers number, color, integer, boolean, and enum controls.
- Unknown descriptor/runtime studio assignments now fail fast instead of being silently ignored.

## 2. USER-RUNNABLE COMMANDS

Full harness:

```bash
cd /usr/projects/tui-vfx && ./scripts/k221_source_isolated_native_demo.sh
```

Animated native ANSI timeline with color:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend-timeline --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format ansi --samples 3 --no-clear
```

Descriptor-driven studio color control:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-ui -- --descriptor-pack descriptors/v3.1/packs/primitive.json --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_pill_button_progress_binding.json --backend compositor --composition-mode native --fail-on-fallback --studio --script 'set effect:filter.pillButton:effectNode:activeColor=#ff0000; render; quit' --no-clear
```

Source-input studio text control:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-ui -- --descriptor-pack descriptors/v3.1/packs/primitive.json --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json --backend compositor --composition-mode native --fail-on-fallback --studio --script 'set source:source.card:mainCard:message=SOURCE OVERRIDE; render; quit' --no-clear
```

## 3. WHAT CHANGED FUNCTIONALLY

The player request now carries both post-effect player IR and source-only IR. Native compositor mode consumes the source-only IR before applying native compositor effects; compatibility mode keeps using post-effect IR. This makes native mode honest: it no longer gets credit for player-resolved graph effects before the compositor applies its own native pass.

Generated studio controls now come from recipe/catalog/descriptor inputs, not only signal aliases. Controls can target signal-backed values or runtime input overrides for effect and source descriptors, with enough metadata for dynamic widgets.

## 4. REVIEW AND DE-SLOP RESULTS

Formal review originally requested changes. Those findings were converted into regressions and fixed:

- Bogus runtime override rejection: fixed and tested.
- Descriptor/source/current/default/range/allowed/mutability metadata: fixed and tested.
- Number/color/integer/boolean/enum evidence: fixed in the harness.
- Row-only `changedCells` evidence: fixed and tested.

Final re-review result: PASS with zero blocking issues.

Detailed review/de-slop evidence is recorded in `docs/new_kernel/PHASE_K2_21_REVIEW_AND_DESLOP_REPORT.md`.

## 5. VERIFICATION MATRIX

Fresh evidence collected:

- `cargo fmt --package tui-vfx-player --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli --package tui-vfx-player-ui -- --check` — PASS.
- `cargo check -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui` — PASS.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets -- -D warnings` — PASS.
- `cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast` — PASS, 122/122.
- `cargo nextest run --workspace --no-fail-fast` — PASS, 2883/2883; run ID `de1c3569-355c-450c-bf1a-7b76af6d91d9`.
- `./scripts/k221_source_isolated_native_demo.sh` — PASS.
- `git diff --check` — PASS.
- `cargo xtask docs check` — PASS with pre-existing warnings.
- `cargo xtask docs api-check` — PASS.
- `cargo xtask docs api-validate` — PASS.
- `cargo xtask audit configschema` — PASS.

## 6. HIGHER-LEVEL BLOCKERS TO FORWARD MOMENTUM

The blockers are now more concrete than “fields and decisions remain”:

1. **Effect-lane coverage is still bounded.** The next forward-progress unlock is not more meta-discussion; it is implementing native lowerers for the remaining debug recipe effect families, with a pass/fail table and no silent fallback.
2. **Source-local pipeline fidelity needs an exhaustive audit.** Native mode now starts from source-only IR, but each source/effect interaction must prove that source-local element pipelines, placement, and style semantics survive lowering.
3. **Studio widgets need richer value editors.** The dynamic control model exists for scalar/text/color/boolean/enum/integer controls. Gradient editors and structured compound controls remain the next visible UI gap.
4. **Public-demo completeness requires broad animated playback evidence.** The player now animates native timelines for the bounded set. The next packet should expand recipe-by-recipe until every non-deprecated v3.1 debug recipe has either a native animated pass or a precise implementation task.

## 7. RECOMMENDED NEXT PACKET

Continue with a source-local pipeline and effect-lane expansion packet:

1. Audit every non-deprecated v3.1 `debug_recipes/` recipe under native/source-only mode.
2. Classify failures by missing native effect, source-local pipeline mismatch, studio-control gap, or fixture data issue.
3. Implement the highest-impact missing native lowerers first.
4. Add dynamic studio widget coverage for the first structured control family, preferably gradients.
5. Keep the harness result-first: pass/fail table, animated ANSI artifacts, studio mutation artifacts, and fresh nextest evidence.

<!-- <FILE>docs/new_kernel/PHASE_K2_21_SOURCE_ISOLATED_NATIVE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.21 source-isolated native status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
