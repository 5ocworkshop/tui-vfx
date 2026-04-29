<!-- <FILE>docs/new_kernel/K2_12_RUNTIME_DYNAMISM_DECISION_MATRIX.md</FILE> - <DESC>K2.12 runtime dynamism decision matrix</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 schema lock: make binding, signal, timing, and dynamic value blockers explicit.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document runtime dynamism clusters and next decisions.</CLOG> -->

# K2.12 Runtime Dynamism Decision Matrix

## Matrix

| Cluster | Records | Current readiness kind | K2.12 disposition | Required decision |
|---|---:|---|---|---|
| Bindable rates | 8 | `bindingSemantics` | Blocking | Define parameter override timing and binding execution for rate-like inputs. |
| Event-driven dwell | 3 | `bindingSemantics` | Blocking | Boolean dwell is mostly shaped; integer/text predicates still need typed predicate semantics. |
| Signals | 5 | `bindingSemantics` | Blocking | Decide signal generator/loopback boundary and whether demos are runtime features or oracle-only. |
| Easings | 29 | `motionTimingSemantics` | Blocking | Accept named easing catalog separately from object/Bezier forms. |
| Motion routes | 5 | `motionTimingSemantics` | Blocking | Decide route vocabulary and route-versus-effect ownership. |
| Value-source filters | 3 | `valueSourceSemantics` | Blocking | Define sampled-surface/value-source descriptors for numeric fields. |
| Scene lifecycle/binding overlap | 7 | `bindingSemantics`, `lifecycleSemantics`, `sceneSemantics` | Blocking | Split visibility gates, source binding, motion binding, and scene pipeline I/O. |
| Loopback | 3 | `oracleOnly` after signoff | Hold back | Treat loopback generator demos as oracle/demo artifacts unless a runtime signal-generator contract is accepted. |
| Complex runtime dynamism | 9 | `valueSourceSemantics` | Blocking | Resolve `signal`, `source`, `binds`, and field-hint semantics before migrating. |

## Recommended sequencing

1. Settle binding execution and parameter override timing first; it affects bindable rates, signals, event-driven dwell, scene records, and complex runtime examples.
2. Split named easing catalog acceptance from Bezier/object-form timing decisions. Named easings can be low-risk descriptor/catalog work; object forms need schema review.
3. Decide sampled-surface value sources before descriptor packs grow fields such as `source`, `signal`, `radius`, or `emitsHint`.
4. Keep loopback demos out of runtime schema readiness unless owner explicitly accepts a signal-generator model.

## Forward-progress blocker

The blocker is not the presence of individual legacy fields. The blocker is the absence of one executable model for when dynamic values are sampled, which owner runs bindings, and which values may reference scene/source state.

<!-- <FILE>docs/new_kernel/K2_12_RUNTIME_DYNAMISM_DECISION_MATRIX.md</FILE> - <DESC>K2.12 runtime dynamism decision matrix</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
