# `validation/samplers/`

Load-time validators for native `sampler.*` slices live here.

Create one validation file per migrated sampler primitive. The validator should
accept only canonical v3.1 fields that the runtime implementation in
`src/samplers/` actually supports.

Example future files:

```text
fnc_validate_sine_wave_inputs.rs
fnc_validate_ripple_inputs.rs
fnc_validate_radial_twist_inputs.rs
mod.rs                            # dispatch only after first sampler validator lands
```

Rules:

- reject runtime-sourced inputs until runtime binding support is deliberately
  added;
- reject unsupported coordinate or timing semantics at load time;
- do not normalize into `SamplerSpec` or any legacy DTO.
