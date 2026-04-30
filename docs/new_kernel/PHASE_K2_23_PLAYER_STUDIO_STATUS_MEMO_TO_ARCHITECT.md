<!-- <FILE>docs/new_kernel/PHASE_K2_23_PLAYER_STUDIO_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.23 status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Public-demo player/studio status: results, current blockers, and next work sequence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — report K2.23 player/studio and native content-stage closure with explicit blockers.</CLOG> -->

# K2.23 status memo to architect

## Results delivered

- Player UI now has demo-oracle-aligned help behavior: help intercepts input, any non-quit key dismisses it, and hidden player state is not mutated while help is visible.
- Reset/reload now re-reads the active recipe JSON from disk and clears volatile session/runtime state.
- UI status and snapshots expose backend, composition mode, fallback status, source render mode, native source isolation, native lowering status, lowered/unlowered counts, backend hash, and styled-cell counts.
- Studio mode now renders a visible controls panel and supports keyboard mutation in Studio focus, including effective-current-value boolean toggling.
- `content.typewriter` now runs in compositor-native source-only mode through a dedicated native content stage, with strict unsupported handling and no silent fallback.
- Current native coverage audit: 144 non-deprecated v3.1 debug recipes, 61 native passes, 83 fallbacks, 0 hard errors.

## Blockers that now need direct resolution

We should stop treating the remaining blockers as an abstract list and address them in execution order:

1. **Content transform native stages are the largest practical unlock.**
   - Current top blockers are `content.splitFlap` and `content.odometer`, followed by broader content transform families such as `content.cellMotion`, `content.marquee`, `content.morph`, `content.scramble`, and `content.wrapIndicator`.
   - Decision already made: content transforms belong in dedicated compositor backend content stages, not in pre-source IR mutation and not as filter/shader approximations.
   - Next action: implement `content.splitFlap` and `content.odometer` native content stages with strict no-silent-fallback rules.

2. **Lifecycle playback is still below public-demo standard.**
   - Current UI ticking proves animation works, but it is not a complete recipe lifecycle clock.
   - The player needs a clearer phase clock that respects recipe timing/dwell semantics, status stability, pause, scrub, and trigger behavior consistently across CLI and UI.
   - Next action: promote the timing work from simple elapsed-loop behavior into a documented lifecycle clock increment with tests and user-runnable demo commands.

3. **Studio controls are operational but not yet rich editors.**
   - Studio now renders controls and mutates them, including effective current values, but value-kind-specific editing remains basic.
   - Numeric, enum, color, gradient, duration, and text controls need ergonomic per-kind mutation/display rules and ideally scripted parity tests.
   - Next action: extend Studio editing one value family at a time, preserving generated descriptor metadata and avoiding schema changes.

4. **Remaining shader/native lanes need explicit lowering decisions.**
   - `shader.revealWipe` is now one of the top non-content blockers.
   - We should decide whether each remaining shader maps to existing compositor primitives, needs a new compositor capability, or should stay an honest fallback.
   - Next action: audit representative shader blockers after the next content-stage tranche.

## Verification summary

- Package and integration Rust gates passed: fmt, check, clippy, and `cargo nextest` over the player/backend/CLI/UI lane.
- Integrated nextest result: 130/130 passed.
- Harnesses passed: source-isolated native demo and full native coverage audit.
- Docs/API/configschema gates passed; docs check retains existing unrelated warnings for `filters.ScalarFieldGlyph`, `filters.GlyphStyle`, and `shaders.Highlighter` ai-hint/TOML gaps.
- Formal review and AI de-slop are documented in `PHASE_K2_23_REVIEW_AND_DESLOP_REPORT.md`.

## Recommended next packet

Create the next work packet around content-native unblock and lifecycle polish:

- Implement native stages for `content.splitFlap` and `content.odometer`.
- Add strict positive/negative tests for each content family.
- Re-run the full native coverage audit after each stage.
- Add one lifecycle clock increment that improves real playback and proves animated user-runnable commands still show color and frame changes.
- Keep Studio controls synced with effective runtime values as new content controls become editable.

<!-- <FILE>docs/new_kernel/PHASE_K2_23_PLAYER_STUDIO_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.23 status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
