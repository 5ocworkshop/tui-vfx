<!-- <FILE>docs/design/tui-vfx-core-macros-lib-rs-cleanup-plan.md</FILE> - <DESC>Plan for the macro-crate hygiene packet — `tui-vfx-core-macros/src/lib.rs` carries 659 lines of inline logic that violates project OFPF discipline (every other lib.rs in the workspace is a clean re-export hub). Move the live functions into OFPF-prefixed files; delete the abandoned earlier OFPF-refactor siblings that share filenames but contain stale skeletons. Workspace audit confirmed the regression is localized to this single crate.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>2026-04-27: discovered during configschema gate v3.0.0 verification pass. The live derive macro lives entirely in lib.rs; nine `fnc_*.rs` siblings + `types.rs` carry dead, never-compiled stubs from an earlier OFPF refactor that wasn't completed. lib.rs has no `mod` declarations.</WCTX>
<!-- <CLOG>1.0.0: initial plan. Authored after a workspace-wide audit (`ofpf-sql` query for lib.rs sizes + per-file logic-vs-re-export grep) confirmed the regression is localized to `tui-vfx-core-macros/src/lib.rs` only. All 11 other lib.rs files are clean re-export hubs.</CLOG> -->

# Plan — `tui-vfx-core-macros` lib.rs hygiene cleanup

> **Scope.** Restore the OFPF discipline that the rest of the workspace honors. `lib.rs` becomes a thin re-export hub; live macro logic moves into per-function OFPF-prefixed files; the abandoned earlier-refactor stubs get deleted.

---

## Verified state

A workspace-wide audit (executed during the configschema gate v3.0.0 verification pass) confirmed:

| File | LOC | logic decls | role |
|---|---|---|---|
| `crates/tui-vfx-core-macros/src/lib.rs` | **659** | **18 inline (1 pub-fn + 14 fn + 3 struct/enum)** | **REGRESSED — has inline logic that should be in siblings** |
| `crates/tui-vfx-shadow/src/lib.rs` | 368 | 0 | clean (large-due-to-doc-comments re-export hub) |
| `crates/tui-vfx/src/lib.rs` | 238 | 0 | clean umbrella re-export |
| Every other lib.rs (10 files) | <125 each | 0 | clean re-export hubs |

**The regression is localized to one file.** Confirmed by `for f in <each>; do grep -cE "^(pub )?fn \|^(pub )?struct \|^(pub )?enum \|^impl " $f; done`.

**The dead earlier-refactor siblings** in `crates/tui-vfx-core-macros/src/`:

- `fnc_impl_config_schema.rs` (46 lines, v0.1.1, dated 2025-12-17)
- `fnc_derive_enum_schema.rs` (107 lines, v0.2.0, dated 2025-12-31)
- `fnc_derive_struct_schema.rs` (86 lines, v0.2.0, dated 2025-12-31)
- `fnc_field_meta_tokens.rs` (55 lines, v0.2.0, dated 2025-12-31)
- `fnc_parse_config_attrs.rs` (57 lines, v0.1.1, dated 2025-12-17)
- `fnc_parse_scalar_lit.rs` (37 lines)
- `fnc_scalar_lit_from_lit.rs` (23 lines)
- `fnc_scalar_lit_to_scalar_value.rs` (21 lines)
- `col_clean_number.rs` (12 lines)
- `types.rs` (26 lines)

These files are NOT compiled. `lib.rs` has zero `mod` declarations importing them (verified by `grep -nE "^mod " lib.rs` returning empty). They sit in the source tree as residue from an OFPF refactor that was started, then abandoned when functionality was added back into `lib.rs` without being re-split.

The dead files lack features the live macro has: `parse_serde_attrs`, `extract_doc_comments`, `apply_rename_all`, `is_option_type`, json_key emission, optional detection, the `SerdeAttr` struct.

---

## Goal

Restore OFPF discipline: `lib.rs` becomes a re-export hub (~30–50 lines of `mod` + `pub use`); each top-level helper function lives in its own OFPF-prefixed file; types live in `types.rs`. Behavior is preserved exactly; this is a pure refactor.

**Non-goal.** Do not extend the macro's functionality during the cleanup. The macro extension to synthesize `T: ConfigSchema` bounds is its own packet (referenced by the configschema gate plan's Phase 5).

---

## Verified inventory of `lib.rs` contents (from end-to-end read of all 659 lines)

**Procedural-macro entry (1 fn, must stay reachable from lib.rs):**

- `derive_config_schema` (`:13–20`) — `#[proc_macro_derive(ConfigSchema, attributes(config))]`. Procedural macros must be declared in `lib.rs` of a `proc-macro = true` crate; this entry stays here as the public surface, but its body delegates to `impl_config_schema`.

**Helper functions to relocate (14):**

| Function | Live `lib.rs` line | Suggested target file |
|---|---|---|
| `parse_config_attrs` | `:49–91` | `fnc_parse_config_attrs.rs` (replace dead body) |
| `parse_scalar_lit` | `:93–114` | `fnc_parse_scalar_lit.rs` (replace dead body) |
| `scalar_lit_from_lit` | `:116–128` | `fnc_scalar_lit_from_lit.rs` (replace dead body) |
| `clean_number` | `:130–132` | `col_clean_number.rs` (replace dead body) — already correctly classified as `col_` (pure leaf helper) |
| `extract_doc_comments` | `:135–155` | `fnc_extract_doc_comments.rs` (NEW — not in dead set) |
| `parse_serde_attrs` | `:158–216` | `fnc_parse_serde_attrs.rs` (NEW) |
| `to_snake_case` | `:219–232` | `col_to_snake_case.rs` (NEW; pure leaf helper, < 25 LOC) |
| `apply_rename_all` | `:235–275` | `fnc_apply_rename_all.rs` (NEW) |
| `is_option_type` | `:278–285` | `col_is_option_type.rs` (NEW; pure leaf helper, < 10 LOC) |
| `scalar_lit_to_scalar_value` | `:287–294` | `fnc_scalar_lit_to_scalar_value.rs` (replace dead body) |
| `field_meta_tokens` | `:296–348` | `fnc_field_meta_tokens.rs` (replace dead body) |
| `impl_config_schema` | `:350–379` | `fnc_impl_config_schema.rs` (replace dead body) |
| `derive_struct_schema` | `:381–493` | `fnc_derive_struct_schema.rs` (replace dead body) |
| `derive_enum_schema` | `:495–656` | `fnc_derive_enum_schema.rs` (replace dead body) |

**Types to relocate (3):**

| Type | Live `lib.rs` line | Suggested target file |
|---|---|---|
| `ConfigAttr` (struct) | `:22–30` | `types.rs` (replace dead body — already exists with stale ConfigAttr stub) |
| `ScalarLit` (enum) | `:32–38` | `types.rs` (same — already exists with stale stub) |
| `SerdeAttr` (struct) | `:40–47` | `types.rs` (NEW field — does not exist in dead types.rs) |

---

## Phasing

### Phase 0 — Take a snapshot of current behavior

**Goal.** Capture an exact baseline of the macro's current output so the cleanup can be verified byte-equivalent.

**Steps.**

1. Run `cargo test --workspace` — capture pass/fail count. Must stay green through the cleanup.
2. For each crate that uses `#[derive(ConfigSchema)]`, capture the output of `cargo doc --workspace --no-deps` and `cargo xtask docs generate`. These produce schema artifacts that should be byte-identical pre- and post-cleanup.
3. If `tui-vfx-core` has a schema-snapshot test, run it and pin the output.

**Effort.** ~15 minutes.

### Phase 1 — Replace the dead OFPF file bodies with the live `lib.rs` versions

**Goal.** Each existing dead file gets its body replaced with the live function. File names are kept where the dead file's name is correct; new files are added for functions that don't have a dead counterpart.

**Steps.**

1. For each function in the relocate list, copy the live body from `lib.rs` to the target file. Preserve all behavior; do not refactor in this packet.
2. Update each file's metadata envelope: bump `<VERS>` (the existing 0.1.1 / 0.2.0 versions are stale; treat this as a major rewrite to 0.3.0), update `<WCTX>` to "macro crate hygiene cleanup — restore OFPF discipline; replace abandoned-refactor stub bodies with live implementations from lib.rs", update `<CLOG>` to name what changed.
3. New files (those without a dead counterpart) get fresh metadata envelopes at v1.0.0.
4. Update `types.rs` to carry `ConfigAttr`, `ScalarLit`, `SerdeAttr` with the live shapes.
5. **Do not yet** add `mod` declarations to `lib.rs`. Phase 1 is purely about getting the files into shape; Phase 2 wires them.

**Verification.** `cargo build -p tui-vfx-core-macros` should still succeed (the OFPF files compile in isolation as private modules even when not yet `mod`-declared, but only if they don't have inter-file `use crate::` references; if they do, Phase 1's compile may surface ordering issues that Phase 2 resolves). If the build breaks, that's expected — proceed to Phase 2.

**Effort.** ~1 hour. Mechanical lift work; the bulk is metadata envelopes.

### Phase 2 — Wire `lib.rs` to the OFPF files and remove the inline logic

**Goal.** `lib.rs` shrinks from 659 lines to ~30–50 lines. Becomes:

```rust
// header envelope
mod col_clean_number;
mod col_is_option_type;
mod col_to_snake_case;
mod fnc_apply_rename_all;
mod fnc_derive_enum_schema;
mod fnc_derive_struct_schema;
mod fnc_extract_doc_comments;
mod fnc_field_meta_tokens;
mod fnc_impl_config_schema;
mod fnc_parse_config_attrs;
mod fnc_parse_scalar_lit;
mod fnc_parse_serde_attrs;
mod fnc_scalar_lit_from_lit;
mod fnc_scalar_lit_to_scalar_value;
mod types;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use fnc_impl_config_schema::impl_config_schema;

#[proc_macro_derive(ConfigSchema, attributes(config))]
pub fn derive_config_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_config_schema(&input) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

// footer envelope
```

**Steps.**

1. Add the `mod` declarations to `lib.rs` for every Phase 1 file.
2. Add the `use` for `impl_config_schema`.
3. Delete every inline function and type definition from `lib.rs`. Keep only the proc-macro entry.
4. Resolve any `use crate::*` references in the OFPF files — each function needs to import its dependencies (e.g. `fnc_derive_enum_schema` needs `use crate::fnc_field_meta_tokens::field_meta_tokens;`, etc.).
5. Bump `lib.rs` version envelope to 1.0.0 (semantic re-baseline; the mode of operation has changed).

**Verification.**

- `cargo build -p tui-vfx-core-macros` succeeds.
- `cargo test --workspace` passes with the same count as Phase 0's snapshot.
- `cargo doc --workspace --no-deps` produces byte-identical output to Phase 0's snapshot.
- `cargo xtask docs generate` is a no-op (no diff against the captured outputs).

**Effort.** ~2 hours. The wiring is mechanical; the verification is what takes time.

### Phase 3 — Confirm there are no orphaned files

**Goal.** Nothing in `crates/tui-vfx-core-macros/src/` should be uncalled.

**Steps.**

1. `cargo build -p tui-vfx-core-macros 2>&1 | grep -i "unused"` — should report nothing.
2. `cargo clippy -p tui-vfx-core-macros --no-deps -- -D warnings` — should pass clean.
3. Verify each OFPF file is reachable from `lib.rs`'s `mod` graph. `grep -E "^mod " src/lib.rs | wc -l` should equal the file count minus 1 (the `lib.rs` itself).

**Effort.** ~15 minutes.

### Phase 4 — Run the configschema audit and the full workspace tests

**Goal.** Confirm nothing the macro emits has changed. The configschema gate is the load-bearing verification — every `#[derive(ConfigSchema)]` site in the workspace produces schema output via the macro.

**Steps.**

1. `cargo xtask audit configschema` — should report the same status as before the cleanup (warn-only mode → Ok).
2. `cargo test --workspace` — same pass count as Phase 0.
3. Compare `cargo doc --workspace --no-deps` output to the Phase 0 snapshot — should be byte-identical except for line numbers in source links (which is fine).
4. Compare `cargo xtask docs generate` output to the Phase 0 snapshot — should be byte-identical.

**If any output changed**, the cleanup introduced a regression. Phases 1–3 must have changed function behavior somewhere. Bisect by re-introducing the inline lib.rs versions one function at a time until the diff disappears.

**Effort.** ~30 minutes assuming the cleanup is byte-equivalent. Up to a half-day if a regression surfaces.

### Phase 5 — Close

1. Update the macro crate's CLOG with a one-line summary of the cleanup.
2. The configschema gate plan v3.0.0 references this packet at "Out of scope" §1; that reference can be replaced with a "completed" marker when this packet lands.
3. No INDEX.md updates needed (this packet doesn't affect public surface).

**Effort.** ~10 minutes.

---

## Verification done at plan-write time

- All 9 dead OFPF files in `tui-vfx-core-macros/src/` were read end-to-end (or in their entirety where < 200 lines) during the configschema gate v3.0.0 verification pass.
- The live `lib.rs` was read across three windows totaling all 659 lines.
- `grep -nE "^mod |^use crate::"` against `lib.rs` confirmed zero `mod` declarations — the OFPF files are not compiled.
- The version-stamp comparison (`fnc_*.rs` at 0.1.1/0.2.0 dated 2025-12-17/12-31; `lib.rs` at 0.4.2 with no date but later) supports the abandoned-refactor reading: someone started splitting in late December, the inline version evolved past it, the OFPF skeletons never caught up.
- Workspace-wide audit (`ofpf-sql` query for lib.rs sizes + per-file logic-vs-re-export grep) confirmed the regression is localized to this single crate.

**Verified-in-passing claims that should be confirmed at packet-execution start:**

- The procedural-macro target requirement that `#[proc_macro_derive]` must be in `lib.rs` of a `proc-macro = true` crate — verify against the latest `cargo-rustc` documentation. (This is well-established Rust convention; not expected to change.)
- The exact list of `use` statements required at the top of each new OFPF file. Phase 1's mechanical lift will surface these by compile errors; a clean lift fixes them as it goes.

---

## Estimated effort

| Phase | Effort |
|---|---|
| 0 — snapshot current behavior | ~15 minutes |
| 1 — replace dead OFPF bodies | ~1 hour |
| 2 — wire lib.rs to OFPF files | ~2 hours |
| 3 — confirm no orphaned files | ~15 minutes |
| 4 — run audits + tests | ~30 minutes (best case) |
| 5 — close | ~10 minutes |
| **Total (best case)** | **~4 hours** |
| **Total (regression path)** | **~1 day if Phase 4 surfaces a behavioral diff** |

Single contributor; no cross-repo coordination.

---

## Why this earns its place

- **Real value, current scale.** The codebase has a documented OFPF discipline (`steering/INTENTIONS.md`, `~/.claude/rules/ofpf.md`); 11 of 12 lib.rs files honor it. This crate is the outlier. Outliers are the failure mode the discipline exists to prevent — they accumulate, look normal after a while, and erode the property that "where do I go to read X" has a single canonical answer per the conventions.
- **Smallest viable intervention.** No refactor of the macro's logic; pure relocation. No new tests required (the existing macro tests suffice as the byte-equivalence check).
- **Step-back test.** "Would I rather maintain a 659-line lib.rs plus 9 dead siblings for three years, or a 30-line lib.rs plus 14 OFPF files matching the conventions?" The latter is what every other crate already shows works.
- **Reversion plan.** Each lifted function is a single move-and-paste; reverts are atomic. If Phase 4 surfaces a regression, a per-function bisect localizes it within a few minutes.
- **Surfaces a separate question.** The cleanup makes it visible whether `tui-vfx-core-macros` should also gain the `T: ConfigSchema` bound-synthesis feature (the queued macro extension). That packet remains out of scope here, but the cleaned-up `fnc_derive_struct_schema.rs` and `fnc_derive_enum_schema.rs` are exactly where that future extension would land — and it'll be a much smaller diff against an OFPF file than against a 659-line monolith.

---

## Process note

This packet exists because the configschema gate v3.0.0 verification pass discovered the regression. The user surfaced the concern explicitly: "It sounds like we have a side project to clean up lib and move that back into files where I believe it belongs, it is strange to have it in lib for our project, is that right?" The answer is yes; this is the side project.

The workspace-wide audit confirmed the regression is bounded to one file. Per the project's "complete and thorough" planning rule, this plan was authored against full reads of every file in scope, with the dead-vs-live distinction confirmed before any prescription was written.

<!-- <FILE>docs/design/tui-vfx-core-macros-lib-rs-cleanup-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
