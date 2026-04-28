<!-- <FILE>steering/work-packets/68-v3-extends-resolution-and-template-inheritance-lift.md</FILE> - <DESC>Lift template-inheritance helpers to src/recipe/inheritance/ and wire V3 loader through them — fixes 112 wargames recipes that silently drop their `extends` reference today, generalises the V2 helpers as schema-agnostic primitives, and surfaces unresolved extends as a typed error at every V3 in-memory entry point.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Initial packet — discovered during ralph session 2026-04-27 that V3 loader silently drops the `extends: Option<String>` field; cross-repo audit shows V2 has a complete extends/template-inheritance implementation at src/recipe_schema/functions/ (5 helpers) that is byte-for-byte applicable to V3.</WCTX> -->
<!-- <CLOG>0.1.0: initial packet capturing the four-phase plan (Phase 1 fail-loud parse error, Phase 2 helper lift to src/recipe/inheritance/, Phase 3 V3 loader wire-up, Phase 4 audit gate).</CLOG> -->

# Packet 68 — V3 extends resolution + template-inheritance lift

## Goal

Two coupled changes shipped as one packet:

1. **Lift the V2 template-inheritance helpers** out of the V2-named `src/recipe_schema/functions/` directory into a schema-agnostic `src/recipe/inheritance/` home. Five files (`fnc_deep_merge_json`, `fnc_resolve_recipe_template`, `fnc_resolve_template_path`, `fnc_validate_template_refs`, `fnc_expand_variants`) move via `git mv`. No behaviour change at the V2 callsites; only import paths shift.
2. **Wire the lifted abstraction into the V3 loader** so V3 recipes with `"extends": "..."` resolve their parent templates the same way V2 does today. The 112 wargames V3 recipes that currently silently drop their `extends` reference (and fail later with a misleading `missing field 'pipeline'`) start working through the production loader.

A fail-loud guard at the V3 in-memory entry points (`parse_v3_document`, `from_value_v3`) ships first as Phase 1 so the error story stops being silent before the abstraction lift lands.

## Problem (verified by ofpf-* + end-to-end file reads)

- **`V3RecipeDocument.extends: Option<String>`** is declared at `src/v3/authoring/cls_v3_recipe_document.rs:48` with `#[serde(default)]`. The field deserialises but no production V3 code path consumes it: `normalize_recipe` at `src/v3/normalize/orc_normalize_recipe.rs:29` never reads `doc.extends`, and `NormalizedRecipe.identity` at `src/v3/normalized/cls_normalized_recipe.rs:43` has no extends slot. Intention 25 violation: a parsed-and-inert schema field.
- **The pipeline-validator surfaces a misleading error.** Running `pipeline-validator --rules --strict-contracts` on `recipes/wargames/sequence_02_phone_connect.json` (a V3 child that extends `themes/enhanced_crt_computer.json`) currently fails with `Recipe loading failed: invalid V3 recipe JSON: missing field 'pipeline' at line 29 column 3`. The real cause is the unresolved `extends` reference — the parent template, which exists at the right path and is itself a valid V3 file with a `pipeline` block, is never loaded.
- **Blast radius:** 112 wargames recipes use `"extends"` references today. All of them are silently broken on the V3 production load path.
- **V2 already solves this.** `src/recipe_schema/functions/` hosts a complete, security-validated, depth-limited, cycle-detected extends resolver: `resolve_recipe_with_template` (depth 10, `HashSet<PathBuf>` cycle detector, security-bounded path resolution, deep-merge with `extends` field stripped from the result). Three V2 callsites use it today (`recipe::load`, `recipe::load_all`, `recipe_schema::parser::json_recipe_dyn_from_file`).
- **Cross-repo audit (Intention 41) is empty.** Zero hits for any of `resolve_recipe_with_template`, `deep_merge_json`, `resolve_template_path`, `validate_no_circular_ref`, `TemplateResolutionError`, `TemplatePathError`, `CircularReferenceError` across `/usr/projects/tui-vfx`, `/usr/projects/mixed-signals`, `/usr/projects/gt-design`. Lift is internal to `tui-vfx-recipes`; no downstream coordination needed.
- **Existing test already proves the resolver works on V3 JSON.** `tests/recipe_schema/test_scene_back_compat.rs:52-71` calls `resolve_recipe_with_template` then `parse_v3_document` for any V3-versioned recipe in the corpus walk. Test passes today (2/2 green, ≥695 fixtures). The integration pattern is proven; only the production V3 loader needs to call it.

## Locked-in design decisions

1. **New home is `src/recipe/inheritance/`.** Both V2 and V3 already converge through `src/recipe/` (`fnc_load_recipe_document` is the centralised version-aware dispatch). Co-locating the version-agnostic inheritance primitives there matches Intention 26 (single source of truth) and keeps `src/recipe_schema/` properly V2-scoped after the move.
2. **All five helpers move together.** `fnc_expand_variants` depends on `deep_merge_json` (variant-expansion uses the same merge primitive). Splitting would either strand `expand_variants` with a back-reference or force a cross-module import. The five-file family is coherent.
3. **Phase 1 error lives at `parse_v3_document` + `from_value_v3`, not `load_v3_document`.** The disk loader will pre-resolve extends in Phase 3, so the resolved JSON has no `extends` by the time it reaches parse (deep-merge strips the field at `fnc_deep_merge_json.rs:55-58`). Putting the error at parse covers all six in-memory entry points uniformly and is forward-compatible.
4. **Phase 3 wires extends into `v3::load_v3_document` only.** All three V3 disk loaders (`load_v3_document`, `load_v3_normalized`, `load_v3_compiled`) chain through it; the recipe-layer wrappers and the central dispatch all flow through here too. One wire-site, eight reachable entry points.
5. **Project-root resolution via the existing `Cargo.toml`-ancestor pattern.** `pipeline-validator/src/main.rs:292-295` and `tools/pipeline-validator/src/fnc_validate_v3_compiled_recipe.rs:71-75` already use `path.ancestors().find(|p| p.join("Cargo.toml").exists())`. Reuse it inside `load_v3_document` so the public signature stays `(path: &Path)` — no API break.
6. **Mixed-version chains are explicitly rejected.** A V3 child extending a V2 parent will produce a merged document with `schema_version: 3` (overlay wins) but inherit V2-shaped fields that don't fit the V3 schema. Add a check inside the resolver that errors with `TemplateResolutionError::SchemaVersionMismatch { parent_v: u32, child_v: u32 }` rather than letting authors get serde failures from the resulting shape mismatch. Catches author mistakes early per Intention 25.
7. **`recipe_schema::mod.rs:48-52` re-exports are removed, not redirected.** Cross-repo audit shows zero external consumers of the `tui_vfx_recipes::recipe_schema::resolve_recipe_with_template` re-export path. Maintaining a backward-compat redirect would be unmotivated complexity (Intention 24).
8. **Stale metadata headers fixed in the same packet.** All five lift candidates currently say `<FILE>src/v2/functions/...</FILE>` despite living at `src/recipe_schema/functions/` — pre-existing drift from an earlier rename. Per Intention 34 (pipeline-touch obligations), fix in the same change that touches the file.

## Phase 1 — V3 unresolved-`extends` parse-time error

**Goal:** stop dropping `extends` silently. Fail loud at every V3 entry point that cannot resolve template inheritance.

### Files modified (3)

1. **`src/v3/fnc_parse_v3_document.rs`** (126 → ~145 lines)
   - Add variant `ParseV3DocumentError::UnresolvedExtends { template_ref: String }` with message `"V3 recipe declares extends=\"{template_ref}\" but this entry point cannot resolve template inheritance — use load_v3_document(&path) instead"`.
   - In `parse_v3_document(input: &str)` body after the `schema_version != 3` check, add an `if let Some(ref t) = doc.extends && !t.is_empty()` guard returning the new variant.
   - Add inline test `parse_v3_document_rejects_unresolved_extends`.
   - Bump `<VERS>` 0.1.0 → 0.2.0; update `<WCTX>` and `<CLOG>` (one-line each).

2. **`src/recipe/fnc_from_value_v3.rs`** (47 → ~60 lines)
   - Add the same `extends.is_some()` guard returning `ParseV3DocumentError::UnresolvedExtends` (re-using the variant declared in `fnc_parse_v3_document.rs`).
   - Add inline test `from_value_v3_rejects_unresolved_extends`.
   - Bump `<VERS>` 0.1.0 → 0.2.0.

3. **`src/v3/fnc_load_v3_document.rs`** — no changes in Phase 1. The disk loader's call to `parse_v3_document` propagates the new error variant through `LoadV3DocumentError::Parse(ParseV3DocumentError::UnresolvedExtends{...})`, which is strictly better than today's misleading "missing field 'pipeline'" trail. Phase 3 replaces this transitional behaviour with native resolution.

### New test file (1)

- **`tests/test_v3_extends_parse_errors.rs`** (~80 lines, new)
  - Covers `parse_v3_document`, `from_value_v3`, `from_value_v3_normalized`, `parse_v3`, `parse_recipe_document`, `from_value_recipe_document` — every entry surfaces `UnresolvedExtends` for a recipe with non-empty `extends`.
  - Regression guard: same six entries with `extends: ""` or no `extends` field parse cleanly (no over-triggering).

### Phase 1 cargo gates

- `cargo check --workspace --all-targets` — must pass
- `cargo test --workspace` — Phase 1's new tests pass; previously-green tests stay green except any V3 recipe with `extends` that was being silently mishandled (those fail visibly now with a clear error; expected and intentional)
- Pipeline-validator on `wargames/sequence_02_phone_connect.json` — errors with `UnresolvedExtends("themes/enhanced_crt_computer.json")` instead of `missing field 'pipeline'`

## Phase 2 — Lift to `src/recipe/inheritance/`

**Goal:** move the five schema-agnostic helpers out of the V2-named directory; update every import; fix every metadata header; generalise the docstrings.

### File moves (5)

```
git mv src/recipe_schema/functions/fnc_deep_merge_json.rs        src/recipe/inheritance/fnc_deep_merge_json.rs
git mv src/recipe_schema/functions/fnc_resolve_recipe_template.rs src/recipe/inheritance/fnc_resolve_recipe_template.rs
git mv src/recipe_schema/functions/fnc_resolve_template_path.rs   src/recipe/inheritance/fnc_resolve_template_path.rs
git mv src/recipe_schema/functions/fnc_validate_template_refs.rs  src/recipe/inheritance/fnc_validate_template_refs.rs
git mv src/recipe_schema/functions/fnc_expand_variants.rs         src/recipe/inheritance/fnc_expand_variants.rs
```

### New file (1)

- **`src/recipe/inheritance/mod.rs`** (~50 lines, new)
  - Mirror the structure of the deleted `src/recipe_schema/functions/mod.rs`.
  - Re-export every public symbol from each submodule.
  - Top-level rustdoc: schema-agnostic framing — "Recipe inheritance and variant-expansion primitives. Used by V2 loaders today and by V3 loaders after Phase 3 wire-up."

### Per-file edits during the move (5 lift candidates)

For each moved file:
- `<FILE>src/v2/functions/...</FILE>` → `<FILE>src/recipe/inheritance/...</FILE>` in both header and footer
- `<VERS>` MINOR bump (relocation is non-breaking for behaviour)
- `<WCTX>` (one line): `"Lift to src/recipe/inheritance/ — schema-agnostic primitive home shared by V2 loaders and V3 loaders (Phase 3)."`
- `<CLOG>` (one line): `"x.y.0: relocate to src/recipe/inheritance/; generalize docstrings; no behavior change."`

### Docstring generalisations

- `fnc_deep_merge_json.rs:18-21` — change `"the V2 schema uses #[serde(default)]"` → `"recipe schemas use #[serde(default)]"`. Examples stay valid for both V2 and V3.
- `fnc_resolve_recipe_template.rs:1` — drop the "for V2 recipes" qualifier from the rustdoc summary.
- `fnc_validate_template_refs.rs`, `fnc_resolve_template_path.rs` — already version-agnostic in body; only header/footer fixes.
- `fnc_expand_variants.rs:18-29` — example uses `schema_version: 1` but it's an illustrative example, not a constraint; leave as-is.

### Module/import updates (8 files)

| File | Change |
|---|---|
| `src/recipe/mod.rs` | Add `pub mod inheritance;` after `pub mod fnc_load_recipe_document;`. Bump VERS PATCH. |
| `src/recipe_schema/mod.rs` | Remove `pub mod functions;` (line 30) and the `pub use functions::{...}` re-export block (lines 48-52). Update module-level rustdoc to drop the Template Inheritance section; add a one-line pointer: `"Template inheritance lives at `crate::recipe::inheritance::*`."` Bump VERS 1.3.0 → 1.4.0. |
| `src/recipe/fnc_load.rs` | Line 7 + lines 46-67: `crate::recipe_schema::functions::` → `crate::recipe::inheritance::`. Bump VERS 1.1.0 → 1.1.1. |
| `src/recipe/fnc_load_all.rs` | Line 7 + line 39: same path fix. Bump VERS 1.0.0 → 1.0.1. |
| `src/recipe/fnc_from_value_all.rs` | Line 7: same path fix. Bump VERS 1.0.0 → 1.0.1. |
| `src/recipe_schema/parser.rs` | Line 7: same path fix. Bump VERS 2.3.0 → 2.3.1. |
| `src/prelude.rs` | Line 36: `recipe_schema::functions::ExpandVariantsError` → `recipe::inheritance::ExpandVariantsError`. Bump VERS 0.8.0 → 0.8.1. |
| `tests/recipe_schema/test_scene_back_compat.rs` | Line 13: same path fix. Bump VERS 0.3.0 → 0.3.1. |

### Directory cleanup

After the moves, `src/recipe_schema/functions/` is empty (only `mod.rs` remains, which gets removed). Delete the directory.

### Phase 2 cargo gates

- `cargo check --workspace --all-targets` — clean
- `cargo test --workspace` — every previously-green test stays green (lift is byte-equivalent; only paths changed)
- Specifically: `test_scene_back_compat::additive_schema_parses_existing_recipe_corpus` (≥695 fixtures) and `test_parametric_variants_expansion::easing_family_*` (3 tests) must pass

## Phase 3 — Wire V3 loader through the abstraction

**Goal:** `load_v3_document` follows `extends` natively; the wargames recipes start working through the production loader.

### New file (1)

- **`src/recipe/inheritance/fnc_resolve_recipe_v3_text.rs`** (~80 lines, new)
  - Single function: `resolve_recipe_v3_text(path: &Path, project_root: &Path) -> Result<String, TemplateResolutionError>`.
  - Body: build `HashSet<PathBuf>`, call `resolve_recipe_with_template`, serialise the merged `Value` back to a string (matching the pattern at `tests/recipe_schema/test_scene_back_compat.rs:62`).
  - Inline tests:
    - Recipe with no `extends` round-trips through the merger producing parse-equivalent text.
    - Recipe with valid `extends` produces JSON with parent's fields + child overrides + `extends` stripped.
    - Recipe with bad `extends` (path traversal, missing template, cycle) returns the corresponding `TemplateResolutionError` variant.
    - **Mixed schema-version chain returns `SchemaVersionMismatch`** (per Decision 6).
  - `src/recipe/inheritance/mod.rs` adds `pub mod fnc_resolve_recipe_v3_text;` and `pub use fnc_resolve_recipe_v3_text::resolve_recipe_v3_text;`.

### Files modified (2)

1. **`src/recipe/inheritance/fnc_resolve_recipe_template.rs`**
   - Add `TemplateResolutionError::SchemaVersionMismatch { parent_v: u32, child_v: u32 }` variant with `#[error("template chain mixes schema versions: parent={parent_v}, child={child_v}")]`.
   - Inside `resolve_recipe_with_template`, after the recursive parent-load and before the merge, compare `recipe_json["schema_version"]` against `template_json["schema_version"]`. Error if they differ.
   - VERS MINOR bump.

2. **`src/v3/fnc_load_v3_document.rs`** (328 → ~365 lines)
   - Add error variant `LoadV3DocumentError::Extends { source: TemplateResolutionError }` wrapping `crate::recipe::inheritance::TemplateResolutionError`.
   - Replace the file-read at the top of `load_v3_document`:
     ```rust
     let raw = std::fs::read_to_string(path)?;
     let mut doc = parse_v3_document(&raw)?;
     ```
     with:
     ```rust
     let project_root = path.ancestors()
         .find(|p| p.join("Cargo.toml").exists())
         .unwrap_or_else(|| Path::new("."));
     let resolved = crate::recipe::inheritance::resolve_recipe_v3_text(path, project_root)
         .map_err(|source| LoadV3DocumentError::Extends { source })?;
     let mut doc = parse_v3_document(&resolved)?;
     ```
   - Existing tests unchanged (no fixtures use `extends`).
   - Add three new tests:
     - `load_v3_document_resolves_extends_from_v3_template` — fixture child + V3 parent → fully merged document; child overrides win; `extends` field absent from the loaded doc.
     - `load_v3_document_errors_on_missing_template` — `extends: "nonexistent.json"` → `LoadV3DocumentError::Extends(TemplateNotFound)`.
     - `load_v3_document_errors_on_v2_template_in_v3_chain` — V3 child extending V2 parent → `LoadV3DocumentError::Extends(SchemaVersionMismatch{...})`.
   - Bump `<VERS>` 0.4.0 → 0.5.0.

### Phase 3 cargo gates

- `cargo check --workspace --all-targets`
- `cargo test --workspace` — Phase 1's `UnresolvedExtends` tests still pass for in-memory entries (which still fail loud); Phase 3's new tests pass; back-compat test still passes
- Pipeline-validator on `wargames/sequence_02_phone_connect.json` — must now PASS through `--rules --strict-contracts`
- Pipeline-validator on a representative wargames sample (10 random) — record results

## Phase 4 — Audit gate (Intention 15)

Mandatory before declaring the packet done.

### Workspace gates

- `cargo check --workspace --all-targets` (clean)
- `cargo clippy --workspace --all-targets -- -D warnings` (clean — no new `#[allow]` per Intention 40)
- `cargo test --workspace` (every test passes)
- `cargo fmt --check` (clean)

### Recipe-corpus gates

- `cargo run --release -p pipeline-validator -- --rules --strict-contracts recipes/wargames/*.json` — capture pass/fail tally; expect 112 wargames recipes that previously failed on extends-resolution to pass
- Re-run a 10-random V3 sample (matching the diagnostic pattern from earlier in this session) — expect 10/10 PASS
- Sample 10 random non-wargames V3 recipes that don't use `extends` to confirm no regressions

### Cross-repo audit (Intention 41)

- `for root in /usr/projects/{tui-vfx,mixed-signals,gt-design}; do grep -rln 'recipe_schema::functions\|recipe::inheritance' --include='*.rs' "$root/src" "$root/crates" "$root/tools" 2>/dev/null; done`
- Expect zero hits (verified pre-flight; re-verify post-flight as regression guard)

### Documentation

- Update `docs/INDEX.md` if any entries point to `src/recipe_schema/functions/`
- File this packet under `steering/work-packets/completed/` after Phase 4 passes

## Open questions (deferred to execution)

1. **Test naming convention.** Top-level `tests/test_v3_extends_parse_errors.rs` vs inline `#[cfg(test)]`. Recommendation: top-level for cross-cutting integration test (covers six entry points) plus inline for `parse_v3_document` itself.
2. **`expand_variants` V3 wire-up.** Today only V2 supports `template + variants` per Intention 26A. The lift makes the helper available to V3, but wiring it is a separate packet. Out of scope here.

## Why now

The Phase 2 signal-facade work (packets 64-66) added the recipe-side `VfxRecipeSignalSpec` facade and the strict-contracts validator. Validating the V3 corpus surfaced the wargames recipes' silent extends failure. Per Intention 25 (hunt for infrastructure wins), the right response is to fix the silent failure mode mechanically — fail-loud first, then add the missing capability — rather than living with 112 silently-broken recipes.

## Provenance

- ofpf-* tooling used for blast-radius and cross-repo audit
- Every implicated file (46 total) read end-to-end before this plan was written
- Pre-existing test `tests/recipe_schema/test_scene_back_compat.rs:52-71` proves the V2 resolver works on V3 JSON — Phase 3 is moving that integration into the production loader

<!-- <FILE>steering/work-packets/68-v3-extends-resolution-and-template-inheritance-lift.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
