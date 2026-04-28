<!-- <FILE>docs/new_kernel/PHASE_G3_STATUS.md</FILE> - <DESC>Phase G3 topology and channel-aware merge implementation status</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase G3 wrap: summarize topology DTOs, parallel snapshot execution, channel deltas, docs, and verification.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase G3 status for architect handoff.
0.2.0: FINAL — record full verification, deslop review, and architect-review approval.</CLOG> -->

# Phase G3 Status — Topology / Parallel Snapshot / Channel-Aware Merge

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: G3 — Topology / Parallel Snapshot / Channel-Aware Merge Semantics

## Summary

Phase G3 adds canonical graph topology and proof execution for node, sequence, and parallel steps.

Current answer: **`GraphSpec` can now carry optional `GraphStep` topology. When topology is absent, existing G2 linear `order` execution remains valid. When topology is present, sequence children execute in order, parallel children read the same pre-parallel snapshot, branch outputs are captured as channel-aware deltas, and joins merge deltas by cell channel under an explicit `ParallelMergePolicy`.**

This remains a clean-room proof layer. It does not add node I/O / hint bus, source recipe schema/compiler, runtime stores, live override precedence, direct node/effect-input bindings, phase/trigger semantics, studio metadata, migration, aliases, or real effect ports.

## Implemented contract APIs

```text
GraphStep
ParallelMergePolicy
GraphSpec.topology: Option<GraphStep>
```

`GraphStep` supports:

```text
node
sequence
parallel
```

`ParallelMergePolicy` supports:

```text
childOrderLastWriterWins
errorOnSameChannelConflict
```

## Implemented proof APIs

```text
CellChannelWrite
CellDelta
SurfaceDelta
```

Supporting proof helpers:

```text
orc_execute_graph_step
orc_apply_proof_node
fnc_surface_delta_between
fnc_apply_surface_delta
fnc_merge_surface_delta
fnc_read_proof_input
```

## Proof adapters

G3 keeps the G2 toy adapters and adds channel-specific proof adapters for merge tests:

```text
proof.copy
proof.replaceGlyph
proof.dim
proof.explicitRoleWrite
proof.setForeground
proof.setBackground
```

These are proof adapters only, not production effect ports.

## Validation/execution behavior locked

- `GraphSpec::validate()` still validates graph identity, declarations, descriptors, nodes, bindings, and `order`.
- Optional topology references must point to declared nodes.
- Duplicate topology node references are rejected by default.
- Topology leaves must cover declared nodes when topology is present.
- Linear `GraphSpec.order` remains the fallback when topology is absent.
- Sequence children execute in order and later children see earlier writes.
- Parallel children all read the same pre-parallel snapshot.
- Parallel sibling branches do not see each other’s writes before join.
- Branch output is represented as per-cell, per-channel proof deltas.
- Different-channel writes compose at merge.
- Same-channel writes use explicit policy: child-order last-writer-wins or error.

## Tests added/updated

```text
crates/tui-vfx-contract/tests/test_graph_policy_contract.rs
crates/tui-vfx-next/tests/test_graph_execution_topology.rs
crates/tui-vfx-next/tests/support/mod.rs
```

Coverage includes:

- explicit sequence topology validation
- explicit parallel topology validation
- unknown topology node rejection
- duplicate topology node rejection
- missing topology coverage rejection
- topology sequence later child sees earlier write
- parallel children read the same snapshot
- parallel sibling does not see prior sibling write
- parallel different-channel writes compose
- parallel same-channel conflict child-order-last-wins
- parallel same-channel conflict errors when policy requires
- nested sequence branch sees its own prior step inside parallel
- linear G2 execution remains passing when topology is absent
- graph and graph-step schemas are current

## Docs updated

```text
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/v3.1-architecture-overview.md
docs/v3.1-contract-boundary.md
docs/v3.1-feature-contract-checklist.md
docs/v3.1-surface-contract.md
docs/INDEX.md
```

## Deliberately not added

```text
node I/O / hint value bus
source recipe authoring schema
canonical recipe compiler
runtime ParameterStore / SignalStore
live override precedence
direct node/effect-input binding targets
phase graph / trigger engine
studio manifest / controls
legacy migration / aliases
real effect ports
```

## Verification status

Final phase verification passed:

```text
cargo fmt --package tui-vfx-contract -- --check
cargo fmt --package tui-vfx-next -- --check
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
cargo clippy -p tui-vfx-next --all-targets -- -D warnings
cargo test -p tui-vfx-contract
cargo test -p tui-vfx-next
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_proof_schemas_are_current
cargo test -p tui-vfx-contract --test test_schema_generation
cargo test -p tui-vfx-next --test test_schema_generation
cargo test --workspace
cargo tree -p tui-vfx-contract
cargo tree -p tui-vfx-next
forbidden legacy crate grep over tui-vfx-contract and tui-vfx-next
git diff --cached --check
```

The first architect-review sidecar requested the `GraphStep::Parallel` wire/schema field be camelCase `mergePolicy`; the fix was applied and schema fixtures regenerated. The second architect-review sidecar returned **APPROVED**. The final OFPF/deslop gate found changed files properly prefixed/sized after splitting graph validation and topology execution into `orc_` files.

## Worktree note

The following pre-existing unrelated files remain outside Phase G3 scope and should not be staged into the G3 commit:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

<!-- <FILE>docs/new_kernel/PHASE_G3_STATUS.md</FILE> - <DESC>Phase G3 topology and channel-aware merge implementation status</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
