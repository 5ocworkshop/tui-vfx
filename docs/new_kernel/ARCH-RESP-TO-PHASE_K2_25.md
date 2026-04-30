<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_25.md</FILE> - <DESC>Next packet for remaining native content transforms</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Post-K2.24 continuation: remove the next content transform blockers from source-only native playback.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define remaining content-stage tranche for cell motion, marquee, morph, scramble, and wrap indicator.</CLOG> -->

# K2.25 remaining native content transforms packet

## Intent

Continue direct blocker resolution by implementing honest compositor-native content stages for the remaining representable `content.*` blockers after split-flap and odometer.

## Required results

1. Audit current player semantics and debug recipe fields for `content.cellMotion`, `content.marquee`, `content.morph`, `content.scramble`, and `content.wrapIndicator`.
2. Implement strict native content-stage support for every field that can be represented without dropping authored semantics.
3. Keep unsupported fields, graph outputs, and non-`all` scopes unsupported with clear diagnostics.
4. Add positive strict-native regressions for supported current debug recipes and negative regressions for unsupported shapes.
5. Re-run the full native coverage audit and update result docs with current counts.
6. Preserve source-only evidence: `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, and `nativeSourceIsolated=true`.

## Constraints

- v3.1 only; do not bump schema version.
- Non-deprecated `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes` only.
- Do not use transient packet shorthand in public field names or code names.
- Use `/usr/projects/tui-vfx-recipes/examples/demo.rs` as behavior oracle only, not as schema source.
- Use `cargo nextest` for tests.
- Subagents must read `.omx/context/k225-subagent-briefing-latest.md` first and must not read or receive `steering/ORCHESTRATION.md`.

## Acceptance gates

- Native audit improves from the K2.24 baseline of 70 native passes / 74 fallbacks / 0 hard errors, or each non-improvement is tied to an exact representability blocker.
- Positive and negative tests prove strict native behavior.
- K2.21 source-isolated native harness still passes.
- Formal review and AI de-slop are documented before closure.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_25.md</FILE> - <DESC>Next packet for remaining native content transforms</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
