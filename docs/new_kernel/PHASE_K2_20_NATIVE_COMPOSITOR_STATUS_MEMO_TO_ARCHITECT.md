<!-- <FILE>docs/new_kernel/PHASE_K2_20_NATIVE_COMPOSITOR_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.20 status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Native compositor lowering: report successful native results first and identify blockers.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — record final review, de-slop, and workspace nextest evidence.
0.1.0: INIT — capture K2.20 native compositor status memo.</CLOG> -->

# Successful native compositor results

## 1. SUCCESSFUL NATIVE COMPOSITOR RESULTS

The K2.20 native compositor harness passed:

```bash
cd /usr/projects/tui-vfx && ./scripts/k220_native_compositor_demo.sh
```

Evidence root: `/tmp/k220-native-results/`.

Summary:

- 13 native compositor recipe renders succeeded.
- 12 renders emitted non-empty native `CompositionSpec` content.
- 0 native fallbacks occurred.
- Native effect families covered: filter, mask, sampler, shader, style.
- Studio controls changed output for `shader.borderSweep` and `filter.pillButton`.

## 2. USER-RUNNABLE COMMANDS

Full harness:

```bash
cd /usr/projects/tui-vfx && ./scripts/k220_native_compositor_demo.sh
```

Native ANSI render:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format ansi /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json
```

Studio control pilot:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-ui -- --descriptor-pack descriptors/v3.1/packs/primitive.json --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json --backend compositor --composition-mode native --fail-on-fallback --studio --script "set position=0.75; render; quit" --no-clear
```

## 3. NATIVE EFFECT LOWERING COVERAGE

Passed native effects:

- `filter.tint`
- `filter.dim`
- `filter.pillButton`
- `mask.wipe`
- `mask.checkers`
- `sampler.sineWave`
- `sampler.ripple`
- `shader.linearGradient`
- `shader.borderSweep`
- `style.fadeIn`
- `style.fadeOut`

Pass/fail table: `/tmp/k220-native-results/native_pass_fail_table.txt`.

## 4. LIVE STUDIO CONTROL RESULTS

Border sweep:

- Artifact: `/tmp/k220-native-results/studio_border_sweep_before_after.json`
- Control: `position`
- Signal: `sweepPosition`
- Result: before/after backend hashes changed; `changedCells > 0`; fallback false.

Pill button:

- Artifact: `/tmp/k220-native-results/studio_pill_button_before_after.json`
- Control: `progress`
- Signal: `pillProgress`
- Result: before/after backend hashes changed; `changedCells > 0`; fallback false.

## 5. WHAT WORKS END-TO-END

The end-to-end path now works for the bounded K2.20 set:

```text
RecipeDocument v3.1
  -> descriptor catalog + RecipePlayer sample
  -> PlayerRenderBackendRequest
  -> native CompositionSpec lowering
  -> render_pipeline_with_spec
  -> PlayerRenderBackendOutput native evidence
  -> CLI playback + UI/studio snapshot
```

Compositor backend JSON now includes explicit composition mode, fallback, native lowering, spec summary, lowered/unlowered ids, hashes, styled cell counts, diagnostics, and optional changed-cell evidence.

## 6. WHAT STILL FALLS BACK OR REMAINS HOLD-BACKED

Major blockers that must be addressed to keep forward progress moving:

1. **Pre-effect source isolation.** Native mode still uses the player-rendered IR as the source scene. The next packet must split source-only IR from post-effect fallback IR so native mode never depends on player-resolved effects as its source substrate.
2. **Full effect-lane coverage.** K2.20 proves the first bounded real native lowerers, not every debug recipe effect. Remaining effect lanes should be wired family-by-family using the same no-silent-fallback evidence contract.
3. **Richer generated studio widgets.** Studio controls are currently generated for signal-backed node inputs. Descriptor-driven color pickers, gradient editors, parameter controls, ranges, and allowed-value widgets remain outstanding.
4. **Easing/shared value bus polish.** Style fade easing is acknowledged with warning and linearly approximated. The value bus and easing curves should be shared with native filter strength/sampler/shader parameters.

## 7. VERIFICATION MATRIX

Fresh evidence collected:

- `cargo check -p tui-vfx-player-backend-compositor` — PASS
- `cargo check -p tui-vfx-player-cli` — PASS
- `cargo check -p tui-vfx-player-ui` — PASS
- `cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli test_fnc_cli_renders_compositor_backend_native_metadata_json test_fnc_cli_render_backend_timeline_native_hash_changes test_fnc_cli_studio_snapshot_native_mutation_changes_backend_hash` — PASS, 3/3
- `./scripts/k220_native_compositor_demo.sh` — PASS
- `cargo fmt --package tui-vfx-player --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli --package tui-vfx-player-ui -- --check` — PASS
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets -- -D warnings` — PASS
- `cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast` — PASS, 114/114
- `cargo nextest run --workspace --no-fail-fast` — PASS, 2875/2875; run ID `c9fc83dc-26bb-450c-8d6e-6c837dcd780e`
- `git diff --check` — PASS

Formal review/de-slop verification is recorded in `docs/new_kernel/PHASE_K2_20_REVIEW_AND_DESLOP_REPORT.md`.

## 8. FILES/CRATES TOUCHED

Primary touched areas:

- `crates/tui-vfx-player`
- `crates/tui-vfx-player-backend-compositor`
- `crates/tui-vfx-player-cli`
- `crates/tui-vfx-player-ui`
- `scripts/k220_native_compositor_demo.sh`
- K2.20 docs under `docs/new_kernel/`

## 9. REVIEW AND DE-SLOP RESULTS

Formal third-party review and AI de-slop passes completed.

- Third-party review result: APPROVED; no must-fix issues found inside the K2.20 changed-file lane.
- AI de-slop result: completed; removed the private one-method evidence-count trait, preserved durable public field names, and restored the UI state file footer shape.
- Post-deslop regression result: PASS for fmt, check, clippy, targeted nextest, native compositor demo harness, workspace nextest, and diff whitespace validation.
- Remaining de-slop risk: `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs` is still large and should be split by effect family in a follow-up cleanup once native lowering behavior is stable.

## 10. RECOMMENDED NEXT PACKET

K2.20 verification is closed. The next packet should focus on:

1. Split player source-only IR from fallback post-effect IR.
2. Expand native lowerer coverage across the remaining debug recipe effect families.
3. Replace signal-only studio controls with descriptor-driven widget generation for numbers, booleans, enums, colors, gradients, and ranges.
4. Keep no-silent-fallback and artifact gates from K2.20 as mandatory acceptance criteria.

<!-- <FILE>docs/new_kernel/PHASE_K2_20_NATIVE_COMPOSITOR_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.20 status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
