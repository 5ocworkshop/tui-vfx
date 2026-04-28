<!-- <FILE>docs/new_kernel/PHASE_F2_STATUS.md</FILE> - <DESC>Concise Phase F2 value source and binding status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase F2 wrap: record declarative value sources, parameters, signals, bindings, schemas, and verification evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record ValueSource, ParameterSpec, SignalSpec, BindingSpec, validation, and runtime deferrals.</CLOG> -->

# Phase F2 Status — ValueSource / ParameterSpec / SignalSpec / BindingSpec

## Status

Phase F2 is complete and ready for commit.

## What changed

Phase F2 adds the declarative source and binding layer to:

```text
crates/tui-vfx-contract
```

The contract can now describe where values come from without implementing runtime stores or recipe compilation.

## Decisions locked

- `ValueSource` represents `literal`, `parameter`, `signal`, and simple numeric `map` sources.
- `ParameterId` / `ParameterSpec` define public recipe controls separately from effect inputs.
- `SignalId` / `SignalSpec` define host/runtime-provided values separately from parameters.
- `BindingSpec` is declarative and parameter-target only for F2.
- `BindingMode` is `replace` only.
- Validation resolves parameter/signal references, checks source/target kind compatibility, enforces parameter bindability, validates fallbacks, rejects non-numeric map sources, requires complete map ranges, requires complete map ranges, and rejects unknown parameter binding targets.

## Schema impact

New checked stable contract schemas:

```text
schemas/v3.1/contract/value-source.schema.json
schemas/v3.1/contract/parameter.schema.json
schemas/v3.1/contract/signal.schema.json
schemas/v3.1/contract/binding.schema.json
```

Existing contract schemas remain current.

## Deliberately not added

Phase F2 does not add:

```text
runtime ParameterStore / SignalStore
live override precedence execution
preset/profile persistence
NodeId or node graph
direct node/effect-input bindings
recipe compiler or recipe schema
template expansion
studio controls or studio manifest
expression language / scripting / formulas
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
cargo test -p tui-vfx-contract --test test_schema_generation            PASS
cargo test -p tui-vfx-contract                                          PASS
cargo test -p tui-vfx-next                                              PASS
cargo test --workspace                                                  PASS
cargo tree -p tui-vfx-contract                                          PASS / inspected; no reverse or forbidden direct dependency
cargo tree -p tui-vfx-next                                              PASS / inspected; depends on tui-vfx-contract
forbidden dependency grep over crates/tui-vfx-contract crates/tui-vfx-next  PASS / no matches
git diff --check                                                        PASS
```

## Next recommended phase

Phase G should introduce node graph identity only after the architect approves the F2 declarative source/binding foundation.

<!-- <FILE>docs/new_kernel/PHASE_F2_STATUS.md</FILE> - <DESC>Concise Phase F2 value source and binding status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
