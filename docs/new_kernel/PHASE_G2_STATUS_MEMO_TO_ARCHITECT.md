<!-- <FILE>docs/new_kernel/PHASE_G2_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase G2 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase G2 wrap: report canonical graph execution proof and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase G2 architect memo in the established status-memo style.
0.2.0: FINAL — record full verification, deslop review, and sidecar approval before handoff.</CLOG> -->

# Phase G2 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: G2 — Canonical Graph Execution Proof

## Executive summary

Phase G2 implements the proof execution layer recommended in `ARCH-RESP-TO-PHASE_G1.md`.

Current answer: **`tui-vfx-next` can now execute a validated canonical `GraphSpec` over semantic surfaces with toy proof adapters. The executor resolves `ValueSource` inputs, respects graph order, lets later nodes see prior node writes, reuses existing surface scope/write semantics, and fails clearly for invalid graphs, missing adapters, and unresolved required values.**

The implementation intentionally stops before source recipe authoring, recipe compilation, runtime stores, live override precedence, direct node/effect-input bindings, phase/trigger semantics, studio metadata, migration, and real effect ports.

## Current implementation state

Proof crate:

```text
crates/tui-vfx-next
```

New proof APIs:

```text
GraphExecutionContext
GraphExecutionError
GraphExecutionOutcome
GraphExecutor
ProofEffectAdapter
```

New proof helpers:

```text
fnc_resolve_value_source
fnc_apply_proof_node
fnc_annotate_node_diagnostics
```

G2 proof adapters:

```text
proof.copy
proof.replaceGlyph
proof.dim
proof.explicitRoleWrite
```

## Goal-by-goal status against the G2 recommendation

| G2 goal / constraint | Current status |
|---|---|
| Consume `GraphSpec` in `tui-vfx-next` | **Done.** `GraphExecutor::execute()` accepts a contract `GraphSpec`. |
| Validate before execution | **Done.** `GraphSpec::validate()` runs before adapter preflight or value resolution. |
| Avoid partial mutation on value failures | **Done.** All node inputs are resolved before any node mutates a surface. |
| Add proof effect adapters | **Done.** Four toy `proof.*` adapters are registered by `with_standard_proof_adapters()`. |
| Missing adapter fails clearly | **Done.** Returns `GraphExecutionError::MissingProofAdapter`. |
| Resolve literal sources | **Done.** Literals clone through directly. |
| Resolve parameter sources | **Done.** Snapshot parameter values override `ParameterSpec.value.default`; missing values fail explicitly. |
| Resolve signal sources | **Done.** Snapshot signal values override fallback/default; missing required values fail explicitly. |
| Resolve numeric maps | **Done.** Numeric maps resolve after nested source resolution and support clamp/output mapping. |
| Execute graph order | **Done.** `GraphSpec.order` is the execution loop. |
| Later nodes see prior node writes | **Done.** Tests prove role writes from one node affect a later role-scoped node. |
| Reuse scope/write semantics | **Done.** Execution routes through `SurfaceEngine::apply_from_source`; tests cover scope diagnostics and `SkipTransparentEmpty`. |
| Include graph/node diagnostics | **Done.** Surface diagnostics are annotated with `graph.node[index].nodeId` paths. |
| Keep F2 bindings validation-only | **Done.** `GraphSpec::validate()` validates bindings, but `GraphExecutor` never applies them. |
| Keep runtime/recipe/studio out | **Respected.** No runtime stores, override precedence, compiler, source schema, phase graph, studio, migration, or real ports were added. |

## Key decisions

### Execution context is a snapshot, not a store

`GraphExecutionContext` contains one-shot `parameter_values` and `signal_values` maps. It deliberately avoids runtime store naming and behavior. There is no live precedence stack; resolution is limited to explicit snapshot values, declared defaults, signal fallbacks, and literals.

### Bindings remain validation-only

G2 follows your recommendation not to execute F2 bindings. A dedicated test sets a valid binding that would change a parameter value if applied, then proves the node still uses the parameter default. This preserves binding execution for a later runtime-store phase.

### Proof adapters are toy adapters

The adapter enum is intentionally named `ProofEffectAdapter` and registers `proof.*` ids. It is sufficient to prove GraphSpec execution semantics without becoming a production descriptor registry or real effect port layer.

## What deliberately was not added

Phase G2 does not add:

```text
source recipe authoring schema
canonical recipe compiler
runtime ParameterStore / SignalStore
live override precedence execution
direct node/effect-input bindings
phase graph / trigger engine
studio manifest / controls
legacy migration / aliases
real effect ports
```

## Verification evidence

Final required verification passed:

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

The staged implementation also passed the architect-review sidecar with **APPROVED**. The final OFPF/deslop review found the new files within the expected small-file range, compliant with `cls_`/`fnc_` naming, and free of code references to runtime stores, recipe compilers, trigger engines, studio manifests, or legacy runtime crates.

## Request for next assignment

Please review Phase G2 as the graph-execution proof lock point and advise the next phase.

Based on your roadmap, the expected next step is:

```text
Phase H1 — Canonical Recipe Document Schema
```

with source authoring and template expansion still deferred beyond H1.

<!-- <FILE>docs/new_kernel/PHASE_G2_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase G2 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
