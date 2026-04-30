<!-- <FILE>docs/new_kernel/K2_20_BACKEND_LIMITATIONS_AND_HOLDBACKS.md</FILE> - <DESC>K2.20 native compositor limitations and holdbacks</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native compositor lowering: record remaining blockers honestly after successful bounded native path.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture limitations and next-packet blockers.</CLOG> -->

# K2.20 backend limitations and holdbacks

## Remaining higher-level blockers

1. **Source isolation is not yet perfect.** The native compositor adapter receives player render IR as the source grid. That means the bounded native path proves real `CompositionSpec` application and native evidence, but the source grid is still produced by the player renderer. The next work packet should split pre-effect source rendering from post-effect fallback IR so native mode never risks double-applying player-resolved effects.
2. **Native lowerers cover a bounded set, not the full debug corpus.** K2.20 covers required filters, masks, samplers, shaders, styles, plus pill-button studio control. Remaining debug effects need deliberate lowerer packets by family.
3. **Style easing is acknowledged but linearly approximated.** `style.fadeIn` and `style.fadeOut` lower natively, but authored easing is reported as `fieldIgnoredWithWarning` until easing curves are shared with compositor filter strength.
4. **Baseline has no graph nodes.** It renders in native mode without fallback, but it does not count as non-empty spec or lowered effect coverage.
5. **Studio controls are signal-derived.** The studio pilot generates useful runtime controls for signal-backed node inputs. Parameter editors, color/gradient editors, and descriptor-driven widgets beyond signal-backed controls remain next-packet work.

## No silent fallback rule

Native mode does not silently fall back. Unsupported nodes emit `unsupportedNativeEffect`. Auto mode reports `fallbackUsed=true` and `requiresIrFallback` when it uses the IR-resolved path.

<!-- <FILE>docs/new_kernel/K2_20_BACKEND_LIMITATIONS_AND_HOLDBACKS.md</FILE> - <DESC>K2.20 native compositor limitations and holdbacks</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
