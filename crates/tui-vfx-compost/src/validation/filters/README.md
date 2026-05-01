# `validation/filters/`

Load-time validators for native `filter.*` slices live here.

Create one validation file per migrated filter primitive. The validator should
accept only canonical v3.1 fields that the runtime implementation in
`src/filters/` actually supports.

Example future files:

```text
fnc_validate_tint_inputs.rs
fnc_validate_dim_inputs.rs
mod.rs                            # dispatch only after first filter validator lands
```

Rules:

- reject runtime-sourced inputs until runtime binding support is deliberately
  added;
- reject descriptor-valid semantics that the migrated runtime slice does not yet
  support;
- do not normalize into `FilterSpec` or any legacy DTO.
