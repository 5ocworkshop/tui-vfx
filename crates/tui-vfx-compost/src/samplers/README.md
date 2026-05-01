# `samplers/`

Native `sampler.*` primitive implementations will live here.

Samplers transform sampling coordinates before cell/style evaluation. This
directory starts documentation-only until the first sampler vertical slice is
migrated.

Use `crates/tui-vfx-compositor/src/samplers/` as read-only reference material.
Bring over only the active slice's proven coordinate behavior, then adapt it to
canonical v3.1 fields.

Example future files:

```text
cls_sine_wave.rs                  # sampler.sineWave native implementation
cls_ripple.rs                     # sampler.ripple native implementation
cls_radial_twist.rs               # sampler.radialTwist native implementation
fnc_sine_wave_offset.rs           # helper split if cls_sine_wave.rs grows too large
col_sampler_input.rs              # shared sampler input accessor, only when needed
mod.rs                            # dispatch/export only after first sampler lands
```

Rules:

- leave this directory README-only until a sampler slice owns real code;
- no `SamplerSpec` bridge layer or legacy DTO translation;
- one primitive per file, helpers only when they reduce size/cohesion pressure;
- add matching load validation under `validation/samplers/` with the slice.
