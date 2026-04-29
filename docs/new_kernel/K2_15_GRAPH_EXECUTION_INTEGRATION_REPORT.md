# K2.15 graph execution integration report

## What changed

`tui-vfx-player` now begins executing graph topology/value-bus semantics instead of using only `graph.order`:

- `graph.topology` is preferred when present.
- `graph.order` remains the fallback for existing/simple fixtures.
- Sequence children share the player graph value bus, so later nodes can consume values emitted by earlier nodes.
- Parallel branches read the same input snapshot, execute independently, and merge at the join.
- Parallel graph-value and surface write conflicts emit deterministic non-fatal warnings under last-writer policy.
- Node outputs can publish graph values by re-emitting resolved input values.

## Files

- `crates/tui-vfx-player/src/fnc_apply_graph_effects.rs`
  - Added topology traversal, sequence execution, parallel branch snapshot/merge, graph-value publication, and warning diagnostics.
- `crates/tui-vfx-player/src/cls_player_sample_request.rs`
  - Added player graph value bus storage.
- `crates/tui-vfx-player/src/fnc_resolve_value_source.rs`
  - Resolved `graphValue` sources from the in-flight bus with fallback support.
- `crates/tui-vfx-player/src/fnc_resolve_effect_input.rs`
  - Routed adapter input resolution through graph-aware value resolution.
- `crates/tui-vfx-player/src/cls_player_warning.rs`
  - Added a constructor for structured non-fatal diagnostics.
- `crates/tui-vfx-player/src/cls_player_frame_report.rs`
  - Added warning-aware frame report construction without bumping the serialized report schema label.

## Tests

`cargo nextest run -p tui-vfx-player --no-fail-fast` passed: 27 passed / 0 failed.

Graph-specific coverage includes:

- sequence output consumed by a later node;
- parallel branch output visible after the join;
- sibling branches do not see each other's unpublished values;
- deterministic graph-value conflict warnings;
- deterministic surface conflict warnings;
- `graph.order` fallback remains green;
- unknown topology nodes remain contract-validation-owned and return a structured `contractValidationFailed` diagnostic.

## Honest limits

- Effect-output publication remains unsupported unless an adapter implements a real effect output; current graph value publication is input re-emission.
- Kind mismatch and missing-input diagnostics are partly contract-validation-owned; runtime graph-value absence currently falls back when an authored fallback exists.
- The implementation is intentionally still player-owned evidence, not backend/compositor lowering.

