# `validation/styles/`

Load-time validators for native `style.*` slices live here.

Create one validation file per migrated style primitive. The validator should
accept only canonical v3.1 fields that the runtime implementation in
`src/styles/` actually supports.

Example future files:

```text
fnc_validate_pulse_inputs.rs
fnc_validate_color_shift_inputs.rs
fnc_validate_rainbow_inputs.rs
mod.rs                            # dispatch only after first style validator lands
```

Rules:

- reject runtime-sourced inputs until runtime binding support is deliberately
  added;
- reject unsupported style-window or modifier semantics at load time;
- do not normalize into legacy style/shader DTOs.
