<!-- <FILE>docs/new_kernel/PHASE_F1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase F1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase F1 wrap: report typed value/input model and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase F1 architect memo in the established status-memo style.</CLOG> -->

# Phase F1 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: F1 — ValueKind / Value / EffectInputSpec

## Executive summary

Phase F1 implements the typed input layer recommended in `ARCH-RESP-TO-PHASE_E1.md`.

Current answer: **`EffectDescriptor` can now declare schema-backed typed effect inputs with validated defaults, numeric ranges, enum domains, bindability metadata, and runtime mutability using a closed value vocabulary.**

The implementation intentionally stops before `ValueSource`, parameters, signals, bindings, recipe parameters, runtime override precedence, studio controls, node graphs, and real effect ports.

## Current implementation state

Contract crate:

```text
crates/tui-vfx-contract
```

New or expanded contract DTOs:

```text
ValueKind
Value
ValueSpec
NumericRange
EffectInputId
EffectInputSpec
RuntimeMutability
EffectDescriptor.inputs
DescriptorValidationError input/value variants
```

New schema roots:

```text
schemas/v3.1/contract/value.schema.json
schemas/v3.1/contract/effect-input.schema.json
```

Updated schema root:

```text
schemas/v3.1/contract/effect-descriptor.schema.json
```

## Goal-by-goal status against the F1 recommendation

| F1 goal / constraint | Current status |
|---|---|
| Add value/input DTOs to `tui-vfx-contract` | **Done.** `ValueKind`, `Value`, `ValueSpec`, `NumericRange`, `EffectInputId`, `EffectInputSpec`, and `RuntimeMutability` are exported. |
| Keep a closed initial `ValueKind` vocabulary | **Done.** The exact requested F1 set is represented. |
| Use tagged typed `Value` literals | **Done.** `Value` uses a strict Serde tagged shape rather than raw JSON. |
| Validate default kind compatibility | **Done.** Type mismatch returns `DescriptorValidationError::ValueKindMismatch`. |
| Validate numeric ranges | **Done.** Numeric defaults are checked against inclusive min/max and invalid/non-finite ranges are rejected. |
| Validate enum domains | **Done.** Empty enum allowed-values and defaults outside the allowed set are rejected. |
| Include role/scope/color/rect typed values | **Done.** These reuse existing schema-backed `RoleTag`, `ScopeSpec`, `Color`, and `Rect` types. |
| Add descriptor inputs | **Done.** `EffectDescriptor.inputs` maps `EffectInputId` to `EffectInputSpec`. |
| Add runtime mutability vocabulary | **Done.** `compileTime`, `phaseStart`, `resetOnly`, and `runtime` are schema-visible. |
| Add bindability metadata only | **Done.** `bindable` is a boolean hint only; no binding model exists. |
| Keep `DescriptorValidationError` narrow | **Done.** No generic diagnostic type was introduced. |
| Add checked schemas | **Done.** `value.schema.json`, `effect-input.schema.json`, and the updated descriptor schema are checked in. |
| Avoid F2+ scope | **Respected.** No `ValueSource`, parameters, signals, bindings, presets, recipes, studio controls, runtime graph, migration, or real effect ports were added. |

## Key decisions

### Descriptor inputs are keyed maps

`EffectDescriptor.inputs` is represented as a `BTreeMap<EffectInputId, EffectInputSpec>`. This follows the architect’s map requirement and gives deterministic schema/test output. Duplicate JSON keys are therefore handled at JSON-parser/map semantics rather than as a post-deserialization duplicate-id validation problem. Input ids are constrained in the generated schema with `propertyNames` and are also explicitly rejected by descriptor validation if invalid ids reach Rust.

### ValueSpec stays strict without becoming F2

`allowedValues` defaults to an empty list so non-enum specs do not carry noisy empty arrays on the wire. Ranges are accepted only for `integer`, `number`, and `duration`; a range on a non-numeric kind is rejected during validation.

### F1 bindability is intentionally non-operational

`bindable: bool` records whether a future F2 source/binding layer may bind an input. It does not introduce `ValueSource`, binding refs, runtime override precedence, expression languages, or studio controls.

### Human-facing metadata remains lightweight

`displayName`, `description`, `unit`, and `semantic` exist for documentation and catalog clarity only. The implementation does not add studio grouping, control widgets, visibility, ordering, advanced/basic flags, or layout metadata.

## What deliberately was not added

Phase F1 does not add:

```text
ValueSource
ParameterSpec / SignalSpec / BindingSpec
ParamRef / SignalRef / binding refs
map/select/expression language
runtime override precedence
presets / recipe parameters
recipe schema/compiler
studio controls / manifest metadata
input inheritance or template expansion
node graph / descriptor registry
runtime bindings / phase graph / trigger engine
legacy migration / legacy aliases
real effect ports
```

## Unrelated worktree files excluded from F1

The worktree still contains pre-existing uncommitted files outside this phase:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

They are explicitly excluded from the F1 change set and must not be staged or committed with this phase.

## Verification evidence

```text
cargo fmt --package tui-vfx-contract -- --check                         PASS
cargo fmt --package tui-vfx-next -- --check                             PASS
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings           PASS
cargo clippy -p tui-vfx-next --all-targets -- -D warnings               PASS
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current  PASS
cargo test -p tui-vfx-contract                                          PASS
cargo test -p tui-vfx-next                                              PASS
cargo test -p tui-vfx-contract --test test_schema_generation            PASS
cargo test -p tui-vfx-next --test test_schema_generation                PASS
cargo test --workspace                                                  PASS
cargo tree -p tui-vfx-contract                                          PASS / inspected; no reverse or forbidden direct dependency
cargo tree -p tui-vfx-next                                              PASS / inspected; depends on tui-vfx-contract
forbidden dependency grep over crates/tui-vfx-contract crates/tui-vfx-next  PASS / no matches
git diff --check                                                        PASS
```

Architect review initially requested doc/schema/value-spec tightening; those issues were fixed, and the final verification matrix above is post-deslop.

## Open questions for next assignment

1. Should Phase F2 introduce `ValueSource` before `ParameterSpec` / `SignalSpec`, or define all three together so binding rules can reference stable source categories immediately?
2. Should `EffectInputSpec` remain the only descriptor-local input shape, or should F2 add separate public parameter/signal namespaces beside it?
3. Should F2 keep bindings purely declarative, or introduce a minimal runtime override precedence model at the same time?
4. Should studio control metadata remain out-of-band until after the binding model, or should a separate studio manifest layer be drafted in parallel?

## Bottom line

Phase F1 gives descriptors a typed input vocabulary without crossing into source/binding/runtime semantics. The next architectural decision is the F2 source/parameter/signal/binding layer.

Recommended next architect assignment: **Phase F2 — ValueSource / ParameterSpec / SignalSpec / BindingSpec**.

<!-- <FILE>docs/new_kernel/PHASE_F1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase F1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
