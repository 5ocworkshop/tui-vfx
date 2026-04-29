<!-- <FILE>docs/new_kernel/K2_13_MOTION_SCOPE_DECISION_REPORT.md</FILE> - <DESC>K2.13 motion and scope decision report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: settle motion/easing and built-in style scope vocabulary.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record accepted easing/motion/scope dispositions and implemented scope variants.</CLOG> -->

# K2.13 Motion and Scope Decision Report

## Motion and easing decision

Easing and motion routes are lifecycle/runtime vocabulary, not effect descriptors.

Accepted easing and motion-route semantics include existing `EasingSpec` vocabulary and `figureEight` for legacy infinity-route evidence. Shadow/subcell rendering remains a `backendHoldback`, not a schema blocker.

## Style scope decision

Accepted built-in scope vocabulary:

```text
moduloRows { modulus, remainder }
moduloColumns { modulus, remainder }
nonEmpty
outerBand
inner
```

Generic predicate registries are not accepted in this packet. `style_predicate_interior.json` maps to the built-in `inner` scope.

## Implementation

`ScopeSpec` and `ScopeKind` now include the accepted built-ins. The player styled-grid scope collector evaluates modulo, non-empty glyph, outer-band, and inner-cell scopes for adapter evidence.

## Disposition

Motion timing, lifecycle, scene scope, and style scope offenders are `acceptedSchema` after K2.13. Remaining work is canonical fixture migration and adapter evidence.

<!-- <FILE>docs/new_kernel/K2_13_MOTION_SCOPE_DECISION_REPORT.md</FILE> - <DESC>K2.13 motion and scope decision report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
