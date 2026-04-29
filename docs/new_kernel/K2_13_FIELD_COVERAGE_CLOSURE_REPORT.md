<!-- <FILE>docs/new_kernel/K2_13_FIELD_COVERAGE_CLOSURE_REPORT.md</FILE> - <DESC>K2.13 field coverage closure report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: close shader gradient/applyTo/position field blockers.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document field coverage closure and descriptor/player adapter changes.</CLOG> -->

# K2.13 Field Coverage Closure Report

## Decision

The four K2.12 field-coverage blockers are closed.

Accepted fields:

- `shader.linearGradient.gradient`
- `shader.linearGradient.applyTo`
- `shader.borderSweep.position`

## Implementation

Contract:

- `ValueKind::Gradient`
- `Value::Gradient`
- `GradientSpec` / `GradientStop`
- `EffectInputSpec.optional`
- `SourceInputSpec.optional`

Descriptor pack:

- `shader.linearGradient.inputs.gradient` is optional and typed as `gradient`.
- `shader.linearGradient.inputs.applyTo` accepts `foreground`, `background`, and `both`.
- `shader.borderSweep.inputs.position` is optional `number` with range `0..1`.

Player:

- linear-gradient shader resolves canonical `gradient` stops when authored, while preserving `startColor` / `endColor` fallback behavior.
- linear-gradient shader applies to foreground, background, or both channels.
- border-sweep shader accepts explicit `position` and otherwise preserves speed/time-derived progress.
- handled-input coverage includes the newly accepted fields.

## Evidence

```text
primitive-field-coverage usedInputFields == handledInputFields
fieldCoverageBlockedRecords: 0
fieldCoverage offenders: 0
candidateReady legacy gradient fixtures: 3
```

<!-- <FILE>docs/new_kernel/K2_13_FIELD_COVERAGE_CLOSURE_REPORT.md</FILE> - <DESC>K2.13 field coverage closure report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
