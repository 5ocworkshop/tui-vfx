<!-- <FILE>docs/new_kernel/PHASE_F1_STATUS.md</FILE> - <DESC>Concise Phase F1 typed value/input status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase F1 wrap: record typed value/input DTOs, descriptor inputs, schema roots, and verification evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record ValueKind, Value, ValueSpec, EffectInputSpec, descriptor input validation, and F2 deferral.</CLOG> -->

# Phase F1 Status — ValueKind / Value / EffectInputSpec

## Status

Phase F1 is complete and ready for commit.

## What changed

Phase F1 adds the first typed effect input contract to:

```text
crates/tui-vfx-contract
```

`EffectDescriptor` can now declare descriptor-local typed inputs keyed by `EffectInputId`.

## Decisions locked

- `ValueKind` is a closed initial vocabulary: `null`, `boolean`, `integer`, `number`, `string`, `text`, `color`, `duration`, `enum`, `role`, `scope`, and `rect`.
- `Value` is a strict tagged typed literal, not raw JSON.
- `ValueSpec` declares expected kind, optional typed default, numeric range, enum allowed values, and documentation-only `unit` / `semantic` strings.
- `EffectInputSpec` declares documentation-only `displayName` / `description`, nested `value`, boolean `bindable`, and `RuntimeMutability`.
- `RuntimeMutability` vocabulary is `compileTime`, `phaseStart`, `resetOnly`, and `runtime`.
- `EffectDescriptor.inputs` is a descriptor-local map from `EffectInputId` to `EffectInputSpec`.
- `DescriptorValidationError` remains narrow and now covers invalid input ids, value-kind mismatch, non-finite numeric values, invalid ranges, range failures, and enum-domain failures.

## Schema impact

New checked stable contract schemas:

```text
schemas/v3.1/contract/value.schema.json
schemas/v3.1/contract/effect-input.schema.json
```

Updated checked stable contract schema:

```text
schemas/v3.1/contract/effect-descriptor.schema.json
```

## Deliberately not added

Phase F1 does not add:

```text
ValueSource
ParameterSpec / SignalSpec / BindingSpec
ParamRef / SignalRef / binding refs
expression language or runtime override precedence
presets / recipe compiler / recipe schema
studio controls or studio manifest
node graph / full descriptor registry
phase graph / trigger engine
legacy migration / legacy aliases
real effect ports
```

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

## Next recommended phase

Phase F2 should add `ValueSource`, `ParameterSpec`, `SignalSpec`, and `BindingSpec` only after the architect approves this F1 input/value foundation.

<!-- <FILE>docs/new_kernel/PHASE_F1_STATUS.md</FILE> - <DESC>Concise Phase F1 typed value/input status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
