# `masks/`

Native `mask.*` primitive implementations will live here.

Masks decide whether a cell or channel participates in rendering. This directory
starts documentation-only until the first mask vertical slice is migrated.

Use `crates/tui-vfx-compositor/src/masks/` as read-only reference material. Copy
or rewrite only the small amount of proven mask logic needed for the active
slice, then adapt it to canonical v3.1 fields.

Example future files:

```text
cls_wipe.rs                       # mask.wipe native implementation
cls_radial.rs                     # mask.radial native implementation
cls_checkers.rs                   # mask.checkers native implementation
col_soft_edge.rs                  # shared helper if still genuinely shared
fnc_wipe_visibility.rs            # helper split if cls_wipe.rs grows too large
mod.rs                            # dispatch/export only after first mask lands
```

Rules:

- leave this directory README-only until a mask slice owns real code;
- no `MaskSpec` bridge layer or legacy DTO translation;
- keep shared helpers small and named for behavior, not migration phase;
- add matching load validation under `validation/masks/` with the slice.
