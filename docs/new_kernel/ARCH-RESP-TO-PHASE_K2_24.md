<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_24.md</FILE> - <DESC>Next packet for native content stages and lifecycle playback polish</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Post-K2.23 continuation: turn the top content blockers and lifecycle playback gap into executable work.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define content split-flap/odometer native stages, lifecycle clock increment, and Studio effective-value preservation.</CLOG> -->

# K2.24 native content stages and lifecycle playback packet

## Intent

Continue the public-demo player/studio path by directly resolving the highest-impact blockers left after native `content.typewriter`: native `content.splitFlap`, native `content.odometer`, and a visible lifecycle playback increment that makes animation feel deliberate rather than a single-frame preview.

## Required results

1. Implement strict compositor-native content stages for `content.splitFlap` and `content.odometer` where current v3.1 debug recipes can be represented without dropping authored semantics.
2. Keep no-silent-fallback behavior: unsupported inputs, outputs, or scopes must remain unsupported with clear diagnostics rather than being approximated.
3. Re-run the native debug-recipes audit after each content-stage tranche and update result docs with current counts.
4. Improve the player/UI lifecycle clock by at least one public-demo-visible increment: stable phase timing, pause/scrub/reset semantics, and user-facing status must remain understandable.
5. Preserve Studio effective-current-value behavior while adding or touching controls.
6. Produce user-runnable commands showing animated, colored compositor playback and Studio mutation with current native evidence.

## Constraints

- v3.1 only; do not bump schema version.
- Non-deprecated `debug_recipes` only; exclude deprecated recipe paths.
- Use `/usr/projects/tui-vfx-recipes/examples/demo.rs` as behavior oracle only, not as schema source.
- Do not use transient packet shorthand in public field names or variable names.
- Use `cargo nextest` for tests.
- Subagents must read `.omx/context/k224-subagent-briefing-latest.md` first and must not read or receive `steering/ORCHESTRATION.md`.

## Acceptance gates

- Positive strict-native regressions for `content.splitFlap` and `content.odometer` when supported.
- Negative regressions proving unsupported authored semantics remain unsupported.
- Native coverage audit improves from the K2.23 baseline; current observed result after content stages is 70 native passes / 74 fallbacks / 0 hard errors.
- Lifecycle playback increment has targeted UI/player regression coverage and a user-runnable command.
- K2.21 source-isolated native harness and K2.22 native coverage audit still pass.
- Formal review and AI de-slop are documented before closure.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_24.md</FILE> - <DESC>Next packet for native content stages and lifecycle playback polish</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
