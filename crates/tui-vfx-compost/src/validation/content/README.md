# `validation/content/`

Load-time validators for native `content.*` slices live here.

Create one validation file per migrated content primitive. The validator should
accept only canonical v3.1 fields that the runtime implementation in
`src/content/` actually supports.

Example future files:

```text
fnc_validate_typewriter_inputs.rs
fnc_validate_scramble_inputs.rs
fnc_validate_marquee_inputs.rs
mod.rs                            # dispatch only after first content validator lands
```

Rules:

- reject runtime-sourced inputs until runtime binding support is deliberately
  added;
- reject unsupported glyph/text timing semantics at load time;
- do not normalize into a legacy player/content adapter DTO.
