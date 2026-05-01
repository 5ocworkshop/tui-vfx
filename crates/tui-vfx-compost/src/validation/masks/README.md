# `validation/masks/`

Load-time validators for native `mask.*` slices live here.

Create one validation file per migrated mask primitive. The validator should
accept only canonical v3.1 fields that the runtime implementation in
`src/masks/` actually supports.

Example future files:

```text
fnc_validate_wipe_inputs.rs
fnc_validate_radial_inputs.rs
fnc_validate_checkers_inputs.rs
mod.rs                            # dispatch only after first mask validator lands
```

Rules:

- reject runtime-sourced inputs until runtime binding support is deliberately
  added;
- reject unsupported geometry, combine, or edge semantics at load time;
- do not normalize into `MaskSpec` or any legacy DTO.
