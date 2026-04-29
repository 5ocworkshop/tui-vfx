<!-- <FILE>docs/new_kernel/K2_13_RUNTIME_DYNAMISM_DECISION_REPORT.md</FILE> - <DESC>K2.13 runtime dynamism decision report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: settle parameter/signal/graph-value/binding/value-source distinctions.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record accepted runtime dynamism model and sampled-field value source.</CLOG> -->

# K2.13 Runtime Dynamism Decision Report

## Decision

The runtime model keeps four separate concepts:

```text
Parameter: user/studio-adjustable recipe knob.
Signal: host/event/time-varying input stream.
GraphValue: value emitted by one graph node and consumed by another.
Binding: declarative wiring from a parameter/signal/graph value into an input.
```

Binding is not a value class.

## Implementation

`ValueSource` now accepts `sampledField`:

```json
{
  "kind": "sampledField",
  "field": "surfaceAngleFrom",
  "x": { "kind": "literal", "value": { "kind": "number", "value": 4.0 } },
  "y": { "kind": "literal", "value": { "kind": "number", "value": 2.0 } },
  "fallback": { "kind": "number", "value": 0.0 }
}
```

`sampledField` is a deterministic per-cell spatial field and infers `number`. It is allowed as descriptor/node input data and rejected as a hidden lifecycle graph-value shortcut when nested graph values are present.

Effect and source inputs now have an `optional` flag so descriptor fields such as `shader.linearGradient.gradient` and `shader.borderSweep.position` can be schema-accepted without forcing every existing canonical fixture to author them.

## Current holdbacks

Signal-generator loopbacks and preview-only signal emitters are not unresolved schema blockers after K2.13. They remain descriptor/compiler/player evidence backlog unless a future packet chooses to make them first-class canonical signal-producing nodes.

<!-- <FILE>docs/new_kernel/K2_13_RUNTIME_DYNAMISM_DECISION_REPORT.md</FILE> - <DESC>K2.13 runtime dynamism decision report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
