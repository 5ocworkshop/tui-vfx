<!-- <FILE>docs/new_kernel/PHASE_G3_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase G3 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase G3 wrap: report topology and channel-aware merge proof and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase G3 architect memo in the established status-memo style.
0.2.0: FINAL — record full verification, deslop review, and architect-review approval.</CLOG> -->

# Phase G3 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: G3 — Topology / Parallel Snapshot / Channel-Aware Merge Semantics

## Executive summary

Phase G3 implements the topology phase recommended in `ARCH-RESP-TO-PHASE_G2.md`.

Current answer: **the canonical graph model can now express node, sequence, and parallel execution topology. The proof executor preserves G2 linear behavior when topology is absent, executes sequences in order, runs parallel children against the same pre-parallel snapshot, captures branch outputs as channel-aware deltas, and merges those deltas by channel under an explicit policy.**

The phase intentionally stops before G4’s node I/O / hint bus and before any source recipe compiler, runtime store, phase engine, studio manifest, migration layer, or real effect ports.

## Current implementation state

Stable contract crate:

```text
crates/tui-vfx-contract
```

New/changed contract vocabulary:

```text
GraphStep
ParallelMergePolicy
GraphSpec.topology
```

Proof crate:

```text
crates/tui-vfx-next
```

New proof vocabulary:

```text
CellChannelWrite
CellDelta
SurfaceDelta
```

New/updated proof execution helpers:

```text
orc_execute_graph_step
orc_apply_proof_node
fnc_surface_delta_between
fnc_apply_surface_delta
fnc_merge_surface_delta
fnc_read_proof_input
```

## Goal-by-goal status against the G3 recommendation

| G3 goal / constraint | Current status |
|---|---|
| Add stable topology DTOs | **Done.** `GraphStep` supports node, sequence, and parallel. |
| Preserve linear fallback | **Done.** `GraphSpec.topology = None` interprets `order` as linear sequence. |
| Validate topology node refs | **Done.** Unknown topology node references fail validation. |
| Reject duplicate node references | **Done.** Duplicate topology leaves fail by default. |
| Require topology coverage | **Done.** Declared nodes must be covered when topology is present. |
| Sequence child order | **Done.** Later sequence children read prior sequence output. |
| Parallel snapshot isolation | **Done.** All parallel children read the same input snapshot. |
| Sibling branches isolated | **Done.** Tests prove a role-scoped sibling cannot see a prior sibling’s role write before join. |
| Branches produce deltas | **Done.** Proof execution captures per-channel `SurfaceDelta` values. |
| Channel-aware merge | **Done.** Merge applies glyph/foreground/background/modifiers/modifier-alpha/role writes independently. |
| Different-channel writes compose | **Done.** Foreground and background branch writes compose in one final cell. |
| Same-channel policy | **Done.** Child-order last-writer-wins and error-on-conflict are implemented. |
| Keep G4 out | **Respected.** No hint value bus or node output state was added. |
| Keep runtime/recipe/studio out | **Respected.** No compiler, runtime store, phase engine, studio manifest, migration, aliases, or real ports were added. |

## Key decisions

### Topology is optional and preserves G2 graphs

`GraphSpec.topology` is optional. Existing G1/G2 graphs that only use `order` keep their behavior. This lets G3 add topology without invalidating the linear proof path.

### Merge uses proof deltas, not whole branch surfaces

Parallel branches execute against a snapshot, but merge does not blindly copy whole final branch surfaces. Instead, each branch reports the exact channels it changed. The join applies those channel writes in child order, so foreground-only and background-only branches compose correctly.

### Error-on-conflict was implemented

The architect marked error-on-conflict optional if small. It was small enough to include. It provides a stronger proof that same-channel conflicts are observable and policy-controlled rather than accidental overwrite behavior.

### Channel-specific proof adapters remain proof-only

`proof.setForeground` and `proof.setBackground` exist only to prove channel-aware merge. They are not production effect ports and should not be copied into source recipe authoring or real descriptor registries.

## What deliberately was not added

Phase G3 does not add:

```text
node I/O / hint value bus
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

Architect review initially requested one correction: `GraphStep::Parallel` needed the stable wire/schema field `mergePolicy` instead of `merge_policy`. That fix was applied, schemas were regenerated, and the second review returned **APPROVED**. The final OFPF/deslop review found changed files properly prefixed/sized after splitting graph validation and topology execution into `orc_` files.

## Request for next assignment

Please review Phase G3 as the topology / parallel snapshot / channel-aware merge lock point and advise the next phase.

Based on your roadmap, the expected next step is:

```text
Phase G4 — Node I/O / Hint Value Bus
```

with source recipe document schema still deferred until after G4.

<!-- <FILE>docs/new_kernel/PHASE_G3_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase G3 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
