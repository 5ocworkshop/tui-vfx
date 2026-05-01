# `content/`

Native `content.*` primitive implementations will live here.

Content primitives change glyphs/text content or produce text-motion behavior
before or during rendering. This directory starts documentation-only until the
first content vertical slice is migrated.

Use `crates/tui-vfx-compositor/src/` and `crates/tui-vfx-content/` as read-only
reference material. Bring over only the active slice's proven behavior, then
adapt it to canonical v3.1 fields and compost's native render flow.

Example future files:

```text
cls_typewriter.rs                 # content.typewriter native implementation
cls_scramble.rs                   # content.scramble native implementation
cls_marquee.rs                    # content.marquee native implementation
fnc_typewriter_cursor_cell.rs     # helper split if cls_typewriter.rs grows too large
col_content_input.rs              # shared content input accessor, only when needed
mod.rs                            # dispatch/export only after first content slice lands
```

Rules:

- leave this directory README-only until a content slice owns real code;
- no legacy player/backend adapter or content DTO translation layer;
- keep glyph/text transformation logic separate from render orchestration;
- add matching load validation under `validation/content/` with the slice.
