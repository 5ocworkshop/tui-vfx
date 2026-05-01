# `shaders/`

Native `shader.*` primitive implementations live here.

This directory contains runtime execution code for migrated shader primitives that
read canonical v3.1 `NodeSpec` fields directly. Do not add a bridge, lowering
layer, `SpatialShaderType` adapter, or legacy compositor DTO here.

Use `crates/tui-vfx-compositor/src/` and `tui-vfx-style` as read-only reference
material, then write the native compost implementation in this directory.

Example files:

```text
cls_linear_gradient_node.rs       # typed wrapper for shader.linearGradient
col_shader_input.rs               # shared literal input accessors for shaders
fnc_linear_gradient_style.rs      # per-cell style logic for shader.linearGradient
fnc_highlighter_style.rs          # future shader.highlighter slice
fnc_glisten_band_style.rs         # future shader.glistenBand slice
fnc_focus_field_style.rs          # future shader.focusField slice
```

Rules:

- one primitive implementation per file unless a split is needed for OFPF size;
- split oversized/mixed logic into `fnc_<primitive>_<helper>.rs` helpers;
- keep `mod.rs` as dispatch/export only;
- runtime code assumes `loader/` and `validation/` already rejected unsupported
  inputs.
