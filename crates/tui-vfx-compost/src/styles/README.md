# `styles/`

Native `style.*` primitive implementations will live here.

Style primitives alter text style, color, modifier, or style-window behavior.
This directory starts documentation-only until the first style vertical slice is
migrated.

Use `crates/tui-vfx-style/` and relevant compositor code as read-only reference
material. Bring over only the active slice's proven behavior, then adapt it to
canonical v3.1 fields and compost's native render flow.

Example future files:

```text
cls_pulse.rs                      # style.pulse native implementation
cls_color_shift.rs                # style.colorShift native implementation
cls_rainbow.rs                    # style.rainbow native implementation
fnc_pulse_alpha.rs                # helper split if cls_pulse.rs grows too large
col_style_input.rs                # shared style input accessor, only when needed
mod.rs                            # dispatch/export only after first style slice lands
```

Rules:

- leave this directory README-only until a style slice owns real code;
- no legacy style DTO translation or broad `SpatialShaderType` lowering;
- one primitive per file, split helpers by responsibility when needed;
- add matching load validation under `validation/styles/` with the slice.
