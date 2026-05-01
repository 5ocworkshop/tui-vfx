# `filters/`

Native `filter.*` primitive implementations will live here.

Filters transform already-sampled cells or styles after source materialization
and earlier primitive stages. This directory starts documentation-only until the
first filter vertical slice is migrated.

Use `crates/tui-vfx-compositor/src/filters/` as read-only reference material.
Bring over only the proven behavior needed for the active slice, then adapt it to
canonical v3.1 fields. Do not copy the whole legacy directory.

Example future files:

```text
cls_tint.rs                       # filter.tint migrated in place as a native filter
cls_dim.rs                        # filter.dim migrated in place as a native filter
fnc_tint_apply.rs                 # helper split if cls_tint.rs grows too large
col_filter_input.rs               # shared filter input accessor, only when needed
mod.rs                            # dispatch/export only after first filter lands
```

Rules:

- leave this directory README-only until a filter slice owns real code;
- no `FilterSpec` bridge layer or legacy DTO translation;
- one primitive per file, split helpers by responsibility when needed;
- add matching load validation under `validation/filters/` with the slice.
