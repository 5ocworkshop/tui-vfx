# Graph I/O Evidence Report

K2.14 adds an explicit proof-test aggregate for graph I/O migration evidence in `crates/tui-vfx-next/tests/test_graph_execution_values.rs`. Full player graph I/O execution remains future work: proof-level sequence/parallel/value-bus semantics live in `tui-vfx-next`, while `tui-vfx-player` still applies graph nodes in `graph.order` for canonical player evidence.

## Current evidence

- Canonical fixtures can use signal/value-source inputs into descriptor fields.
- Player adapters resolve bound/fallback signal values for shader/filter fields.
- `tui-vfx-next` remains the stronger graph execution proof surface for sequence propagation, parallel snapshots, and merge diagnostics.
- `graph_io_migration_tranche_covers_sequence_join_and_conflict_evidence` covers sequence graph-value consumption, parallel post-join graph-value visibility, and deterministic conflict errors.

## Remaining blocker for forward progress

The next major graph step is to make player evidence consume graph topology and graph values rather than only ordered primitive nodes. Until then, graph I/O fixtures should be treated as proof/runtime integration backlog, not schema-decision blockers.
