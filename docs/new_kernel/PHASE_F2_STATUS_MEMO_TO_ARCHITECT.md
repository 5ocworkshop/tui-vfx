<!-- <FILE>docs/new_kernel/PHASE_F2_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase F2 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase F2 wrap: report declarative source/binding model and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase F2 architect memo in the established status-memo style.</CLOG> -->

# Phase F2 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: F2 — ValueSource / ParameterSpec / SignalSpec / BindingSpec

## Executive summary

Phase F2 implements the declarative source layer recommended in `ARCH-RESP-TO-PHASE_F1.md`.

Current answer: **`tui-vfx-contract` can now describe literal, parameter, signal, and simple mapped value sources; public parameters and host signals are separate namespaces; and bindable parameter-target bindings validate declaratively without runtime stores or node graph identity.**

The implementation intentionally stops before runtime parameter/signal stores, live override precedence execution, direct effect-input bindings, recipe nodes, recipe compiler/schema, studio controls, and real effect ports.

## Current implementation state

Contract crate:

```text
crates/tui-vfx-contract
```

New contract DTOs:

```text
ValueSource
ParameterId
ParameterSpec
SignalId
SignalSpec
BindingTarget
BindingMode
BindingSpec
```

Expanded validation vocabulary:

```text
InvalidParameterId
InvalidSignalId
UnknownParameter
UnknownSignal
UnknownBindingParameterTarget
SourceKindMismatch
NonNumericMapSource
```

New schema roots:

```text
schemas/v3.1/contract/value-source.schema.json
schemas/v3.1/contract/parameter.schema.json
schemas/v3.1/contract/signal.schema.json
schemas/v3.1/contract/binding.schema.json
```

## Goal-by-goal status against the F2 recommendation

| F2 goal / constraint | Current status |
|---|---|
| Add `ValueSource` | **Done.** Variants cover literal, parameter, signal, and map. |
| Add `ParameterSpec` | **Done.** Parameters have stable ids, metadata, `ValueSpec`, bindability, and validation. |
| Add `SignalSpec` | **Done.** Signals have stable ids, metadata, `ValueSpec`, required policy, and validation. |
| Keep parameters separate from effect inputs | **Done.** `ParameterSpec`, `SignalSpec`, and `EffectInputSpec` remain distinct DTOs. |
| Add `BindingSpec` | **Done.** Binding is declarative and parameter-target only. |
| Keep binding mode minimal | **Done.** Only `replace` exists in F2. |
| Validate references | **Done.** Unknown parameters/signals and unknown binding targets are rejected. |
| Validate kind compatibility | **Done.** Literals, parameter/signal sources, fallbacks, and binding sources are checked against expected kinds. |
| Validate map sources | **Done.** Map ranges are validated and map input sources must be numeric-compatible; maps produce `number`. |
| Add checked schemas | **Done.** Value-source, parameter, signal, and binding schema fixtures are generated and checked. |
| Avoid runtime/recipe/studio scope | **Respected.** No runtime stores, override execution, node graph, recipe compiler, studio controls, migration, or real effect ports were added. |

## Key decisions

### BindingSpec is parameter-target only

F2 follows your recommendation to avoid inventing node identity before Phase G. Direct effect-input binding remains deferred until node graph identity exists. Binding validation also enforces `ParameterSpec.bindable`, so non-bindable parameters can exist as public controls without accepting declarative bindings.

### ValueSource is declarative only

`ValueSource` can describe literals, parameter refs, signal refs, and simple numeric map with complete input/output boundss. It does not evaluate runtime stores, live overrides, preset/profile layers, arbitrary expressions, scripts, or multi-source formulas.

### Default ownership remains in ValueSpec

`ParameterSpec` and `SignalSpec` both own a `ValueSpec`; defaults continue to live inside `ValueSpec.default`. `SignalSpec.required` makes host-provisioning policy explicit without duplicating default ownership.

## What deliberately was not added

Phase F2 does not add:

```text
ParameterStore / SignalStore
live override precedence execution
preset/profile persistence
NodeId / node graph
direct node/effect-input bindings
recipe compiler/schema
template expansion
studio manifest / controls
expression language / scripting
phase graph / trigger engine
legacy migration / aliases
real effect ports
```

## Unrelated worktree files excluded from F2

The worktree still contains pre-existing uncommitted files outside this phase:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

They are explicitly excluded from the F2 change set and must not be staged or committed with this phase.

## Verification evidence

```text
cargo fmt --package tui-vfx-contract -- --check                         PASS
cargo fmt --package tui-vfx-next -- --check                             PASS
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings           PASS
cargo clippy -p tui-vfx-next --all-targets -- -D warnings               PASS
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current  PASS
cargo test -p tui-vfx-contract --test test_schema_generation            PASS
cargo test -p tui-vfx-contract                                          PASS
cargo test -p tui-vfx-next                                              PASS
cargo test --workspace                                                  PASS
cargo tree -p tui-vfx-contract                                          PASS / inspected; no reverse or forbidden direct dependency
cargo tree -p tui-vfx-next                                              PASS / inspected; depends on tui-vfx-contract
forbidden dependency grep over crates/tui-vfx-contract crates/tui-vfx-next  PASS / no matches
git diff --check                                                        PASS
```

## Open questions for next assignment

1. Should Phase G introduce a minimal `NodeId` / node graph around descriptors and parameter bindings, or should we first add a recipe-level container for parameters/signals/bindings?
2. Should direct effect-input binding be introduced in the same phase as node identity, or after node graph validation exists?
3. Should map transforms remain numeric-only in G, or should curve/select transforms wait until recipe compiler work?
4. Should runtime precedence remain documented only until after canonical recipe compilation lands?

## Bottom line

Phase F2 gives future recipe and node-graph work a declarative source/binding vocabulary without crossing into runtime execution. The next architectural decision is node identity versus a recipe container layer.

Recommended next architect assignment: **Phase G — Node Graph / Recipe Container Shape**.

<!-- <FILE>docs/new_kernel/PHASE_F2_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase F2 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
