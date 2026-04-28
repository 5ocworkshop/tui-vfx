<!-- <FILE>docs/new_kernel/PHASE_G2_STATUS.md</FILE> - <DESC>Phase G2 canonical graph execution proof implementation status</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase G2 wrap: summarize GraphSpec proof execution, value resolution, adapters, docs, and verification.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase G2 status for architect handoff.
0.2.0: FINAL — record full verification, deslop review, and architect-review sidecar approval.</CLOG> -->

# Phase G2 Status — Canonical Graph Execution Proof

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: G2 — Canonical Graph Execution Proof

## Summary

Phase G2 adds a proof execution bridge in `tui-vfx-next` for the canonical `GraphSpec` created in G1.

Current answer: **`tui-vfx-next` can now execute a validated `GraphSpec` over semantic `Surface`s using toy proof adapters. ValueSource resolution works for literals, parameters, signals, and numeric maps; graph order controls execution; later nodes observe prior node cell/role writes; scope and write-policy semantics reuse the existing surface engine; missing adapters and value-resolution failures are explicit errors; and F2 bindings remain validation-only.**

This remains a proof layer only. It does not add source recipe authoring, recipe compilation, runtime stores, live override precedence, direct node/effect-input binding targets, phase/trigger semantics, studio metadata, migration, or real effect ports.

## Implemented proof APIs

```text
GraphExecutionContext
GraphExecutionError
GraphExecutionOutcome
GraphExecutor
ProofEffectAdapter
```

Supporting helpers:

```text
fnc_resolve_value_source
fnc_apply_proof_node
fnc_annotate_node_diagnostics
```

## Proof adapters

G2 registers toy proof adapters for:

```text
proof.copy
proof.replaceGlyph
proof.dim
proof.explicitRoleWrite
```

These are proof adapters only, not production effect ports.

## Validation/execution behavior locked

`GraphExecutor::execute()` now:

- runs `GraphSpec::validate()` before execution
- preflights proof adapter availability before execution
- resolves all node inputs before execution, preventing partial mutation from value-resolution failures
- resolves literals, parameter snapshot/default values, signal snapshot/fallback/default values, and numeric maps
- executes nodes in `GraphSpec.order`
- applies node scope, cell write policy, and role write policy through the existing `SurfaceEngine`
- annotates surface diagnostics with graph/node identity paths
- returns final surface, executed node ids, matched/written counts, and diagnostics
- validates but does not apply `GraphSpec.bindings`

## Tests added

```text
crates/tui-vfx-next/tests/test_graph_execution_values.rs
crates/tui-vfx-next/tests/test_graph_execution_order.rs
crates/tui-vfx-next/tests/support/mod.rs
```

Coverage includes:

- literal node input execution
- parameter override execution
- parameter default execution
- signal input execution
- signal fallback behavior
- missing required signal failure
- numeric map source execution
- node order changes output
- later node sees prior node role write
- scope diagnostics reuse existing semantics
- write policy reuse for `SkipTransparentEmpty`
- unknown proof adapter failure
- invalid graph fails before execution
- F2 bindings are not applied in G2

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
cargo test --workspace
cargo tree -p tui-vfx-contract
cargo tree -p tui-vfx-next
forbidden legacy crate grep over tui-vfx-contract and tui-vfx-next
git diff --cached --check
```

The architect-review sidecar returned **APPROVED** for staged G2. The OFPF/deslop gate also passed: new files keep OFPF `cls_`/`fnc_` naming, preserve small-file boundaries, and contain no code references to runtime stores, recipe compilers, trigger engines, studio manifests, or legacy runtime crates.

## Worktree note

The following pre-existing unrelated files remain outside Phase G2 scope and should not be staged into the G2 commit:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

<!-- <FILE>docs/new_kernel/PHASE_G2_STATUS.md</FILE> - <DESC>Phase G2 canonical graph execution proof implementation status</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
