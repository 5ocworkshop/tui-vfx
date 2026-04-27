<!-- <FILE>docs/design/tui-vfx-configschema-audit-gate-plan.md</FILE> - <DESC>Plan for the `1.9.A.followup` packet — fix the audit scanner's qualified-path coverage gap, bring four invisible impls into the gate's vision, lift the 15 baseline entries (8 permanent-reason lifts + up to 5 derive-migration candidates + 2 generic-bound lifts), wire `audit-all` into the `check-all` and `ci` umbrella recipes, and decide the warn-only → hard-fail promotion timing relative to public release. Grounded in full end-to-end reads of every load-bearing file at 2026-04-27 and a live `cargo xtask audit configschema` run. The 1.9.A gate itself is shipped (v1.0.0 of this plan got that wrong).</DESC> -->
<!-- <VERS>VERSION: 3.1.0</VERS> -->
<!-- <WCTX>2026-04-28: MINOR update to mark "Out of scope §1 — Macro crate cleanup packet" as DONE. The macro cleanup landed under commit 5401178; its plan has moved to docs/design/completed/. Per the user directive: complete and thorough is ALWAYS the bar.</WCTX> -->
<!-- <CLOG>3.1.0: MINOR — mark "Out of scope §1 — Macro crate cleanup packet" as DONE 2026-04-28 (commit 5401178); update its plan path to docs/design/completed/. 3.0.0: full third revision after end-to-end reads of every load-bearing file. KEY CORRECTIONS over v2.0.0: (a) the live derive macro at `tui-vfx-core-macros/src/lib.rs` (659 lines) is much more capable than the dead OFPF siblings v2.0.0 read — it emits per-variant descriptions from `///` comments, json_value from `#[serde(rename)]`/`rename_all`, tag_field from `#[serde(tag)]`, and field default/min/max from `#[config(...)]`; (b) consequently PathType/RoleTag/Color/StyleRegion/Never are real derive candidates, not "investigate" candidates; (c) the audit scanner has a fully-qualified-path coverage gap missing 4 impls in `cls_cursor_shader.rs` (×2) and `enum_vfx_cursor_behavior.rs` (×2); (d) StyleRegion's baseline `note` is doubly wrong; (e) RoleTag derivability is gated on InternedRoleName needing a ConfigSchema impl. Process note: v1.0.0 prescribed reinventing shipped infrastructure; v2.0.0 corrected that but read dead code; v3.0.0 read live code end-to-end. Each iteration's CLOG names the failure mode the next pass corrected.</CLOG> -->

# Plan — 1.9.A.followup: scanner bug, baseline lift, derive migrations, umbrella wiring, promotion decision

> **Status correction series.**
>
> - **v1.0.0** prescribed building infrastructure that already existed (xtask path, marker, vocabulary, baseline mechanism, format-spec doc — all wrong). Authored without verifying.
> - **v2.0.0** corrected the shipped-vs-pending status but read the dead OFPF-prefixed `fnc_*.rs` files in `tui-vfx-core-macros/src/` rather than the live `lib.rs`. Got the derive macro's capabilities wrong, called PathType / RoleTag "probably reclassify" without grounding, and missed the scanner bug entirely.
> - **v3.0.0** reads end-to-end. Confirms the live macro's behavior. Discovers four impls invisible to the gate today, and a doubly-wrong baseline note. Renames Phase 2 from "investigate via attempted derive" to a per-entry decision matrix grounded in evidence.

---

## Verified state (end-to-end reads at 2026-04-27)

Every claim below is sourced from a `Read` of the cited file at the cited line, or a `cargo`/`ofpf-*` invocation logged here. No claim is sourced from the buy-once sweep doc or from a partial read.

### Files read end-to-end during this verification pass

**Audit gate infrastructure (already shipped):**

- `xtask/src/main.rs` (174 lines) — `cargo xtask audit configschema` subcommand at `:50`, dispatched at `:142`.
- `xtask/src/lib.rs` (15 lines) — re-exports `audit::audit_configschema` only; the lib target is named `xtask_audit_configschema` per `xtask/Cargo.toml:17–19` so the integration tests can `use xtask_audit_configschema::audit_configschema`.
- `xtask/Cargo.toml` (54 lines) — confirmed deps: clap, toml, serde, serde_json, walkdir, anyhow, owo-colors; dev-dep tempfile = "3"; bin + lib targets.
- `xtask/src/audit/mod.rs`, `fnc_audit_configschema.rs`, `fnc_find_justification.rs`, `fnc_load_baseline.rs`, `fnc_scan_file_for_impls.rs` (5 files, ~600 lines combined).
- `xtask/tests/test_audit_configschema.rs` (262 lines) — 7 integration tests covering: missing-justification (warns in warn-only), unrecognised-kind (hard error), justified-impl passes, baselined impl passes without justification, stale baseline entry doesn't panic, macro-body matches skipped, `Other("...")` justification passes with warning. Fixtures use `tempfile::TempDir`.
- `justfile` (331 lines) — `audit-configschema` recipe at `:313`, `audit-all` aggregator at `:318`, `check-all` umbrella at `:178` (`fmt-check lint test docs-all-check`), `ci` simulation at `:326` (same deps as check-all). **Neither umbrella invokes `audit-all` today** — confirmed verified gap.
- `xtask/data/configschema_baseline.toml` (98 lines, schema_version = 1) — 15 grandfathered entries.
- `docs/CONFIGSCHEMA_JUSTIFICATION.md` (162 lines) — full policy: rule, marker (`CONFIGSCHEMA-JUSTIFICATION:`), 8 canonical kinds, baseline policy, promotion schedule (`2026-07-01`).

**Macro crate (the live macro and its dead siblings):**

- `crates/tui-vfx-core-macros/src/lib.rs` (659 lines, full read across 3 windows: 1–100, 100–352, 350–660). **THIS IS THE LIVE MACRO.** Defines `derive_config_schema` proc-macro at `:13–20` and 14 helper functions inline. No `mod` declarations.
- `crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs` (46 lines), `fnc_derive_enum_schema.rs` (107), `fnc_derive_struct_schema.rs` (86), `fnc_field_meta_tokens.rs` (55), `fnc_parse_config_attrs.rs` (57), `types.rs` (26) — **DEAD CODE.** `lib.rs` has no `mod` declaration importing them; `grep -E "^mod |^use crate::"` against `lib.rs` returns zero matches in the relevant ranges. The OFPF files are versioned 0.1.1 / 0.2.0 dated 2025-12-17 / 2025-12-31, predating `lib.rs` v0.4.2. They are an abandoned earlier refactor.
- The dead OFPF code lacks features the live macro has: `parse_serde_attrs` (tag/rename/rename_all/skip/default), `extract_doc_comments` (description from `///`), `apply_rename_all` (snake_case/camelCase/PascalCase/SCREAMING_SNAKE_CASE), `is_option_type`, json_key emission, optional detection.

**Each baseline entry's source impl, read at the cited line:**

- `crates/tui-vfx-core/src/schema/mod.rs` (109 lines, full) — confirms 5 hand-written impls: `String` (`:69`), `&str` (`:77`), `Option<T>` (`:85`), `Vec<T>` (`:92`), `Box<T>` (`:99`); plus 2 macro_rules definitions (`:26-37, :38-52`) that the scanner correctly skips via the `$t` heuristic.
- `crates/tui-vfx-core/src/mixed_signals_schema.rs` (606 lines, read in 3 windows: 1–130, 450–530, plus partial spans) — confirms `SignalOrFloat` (`:32`), `SignalSpec` (`:82–465`, ~35 variants self-recursive via `signal_spec_ref`), `EasingType` (`:467–603`, 25 unit variants).
- `crates/tui-vfx-core/src/bindable/cls_bindable.rs` (478 lines, full) — confirms `Never` enum at `:24` (no variants); `Never` ConfigSchema at `:85`; generic `VfxBindable<T, S>` ConfigSchema at `:277` (gates on `T: ConfigSchema + Clone + PartialEq + 'static, S: BindableSignal`); 3 specialized inherent impls per `(T, S)` carrying the three legacy `evaluate` signatures (`:375, :404, :442`).
- `crates/tui-vfx-types/src/color.rs` (full impl body 27–55) — `Color` impl with explicit per-channel `Range::new(Some(0)..=Some(255))` ranges.
- `crates/tui-vfx-types/src/role_tag.rs` (303 lines, full) — confirms 12 first-class variants + `Custom(InternedRoleName)`. **All variants have `///` doc comments.** No `#[serde(rename_all)]` on the enum. Hand-written impl at `:124–208` flattens `Custom` field to `String::schema()`. `InternedRoleName` is defined at `:58` as a newtype around `InternedString`. **`ofpf-defs InternedRoleName` returns ONLY the struct definition — no ConfigSchema impl exists for it.**
- `crates/tui-vfx-geometry/src/types/path_type.rs` (435 lines, full) — confirms enum has `#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]` at `:13`, `///` doc comments on every variant, `#[serde(alias = "...")]` on `CarrierOrbit` (`:85`) and `FigureEight` (`:102`). Hand-written impl at `:201–432` uses local helpers `f32_field` and `u8_field` to attach per-field `default`/`min`/`max` metadata to `Bounce.bounces` (default 3, range 0–12; `:307`) and `Step.steps` (default 5; `:393`). No other variant fields carry default/range metadata. Field-level descriptions exist only via these helpers — there are no `///` comments on the individual variant fields (only on the variant constructors).
- `crates/tui-vfx-style/src/models/cls_style_region.rs` (226 lines, full) — confirms enum has `#[serde(rename_all = "PascalCase")]` at `:80`, derives only `Serialize` (Deserialize is hand-written in a sibling file). All variants have `///` doc comments. `RowRange`, `Cell`, `ColumnRange`, `Modulo` use `BindableU16` for several fields (which has its own ConfigSchema). The current hand-written ConfigSchema impl at `crates/tui-vfx-style/src/models/fnc_style_region_schema.rs:21` (full read, 135 lines) emits `tag_field: None` and `json_value: None` on every variant, contradicting the live derive's behavior under `rename_all = "PascalCase"` (which would emit `Some("All")`, `Some("Role")`, etc.).
- `crates/tui-vfx-types/Cargo.toml` (32 lines, full) — confirms `tui-vfx-core = { path = "../tui-vfx-core" }` at `:21`. **The dependency exists.** This refutes StyleRegion's baseline note claim that "RoleTag in tui-vfx-types which does not depend on tui-vfx-core" (`xtask/data/configschema_baseline.toml:53`).
- `crates/tui-vfx-style/src/models/fnc_style_region_schema.rs` (135 lines, full).
- `crates/tui-vfx-style/src/models/cls_cursor_shader.rs` (lines 25–115 — both impls covered) — confirms `CursorShaderPrimary` (`:43`) and `CursorShaderTrail` (`:81`) hand-written impls. The struct definitions at `:33` and `:75` carry tuple field `(u16, u16)` which has no ConfigSchema impl, so derive cannot reach these without an opaque-marker on the field. **Both impls use the fully-qualified `impl tui_vfx_core::ConfigSchema for X` form** — invisible to the scanner.
- `crates/tui-vfx-content/src/pool/cls_pool.rs` (153 lines, full) — confirms `Pool<T>` impl at `:68` gating on `T: ConfigSchema`.

**Cargo run captured (live behavior):**

```
$ cargo xtask audit configschema
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.47s
     Running `target/debug/xtask audit configschema`
Auditing hand-written impl ConfigSchema for X blocks...
✓ All impl ConfigSchema for X blocks are justified or baselined.
```

Reports clean. Misleadingly clean — see scanner bug §3.

---

## What the live derive macro emits

Sourced from end-to-end reads of `crates/tui-vfx-core-macros/src/lib.rs`. The dead OFPF files differ; ignore them.

**Top-level (struct or enum) attributes the live macro reads:**

| Attribute | Effect | Lib.rs line |
|---|---|---|
| `///` doc comment | Becomes `description: Some("...")` on the schema node | `:475` (struct), `:631` (enum) |
| `#[serde(rename = "X")]` | Becomes `json_name: Some("X")` | `:481`, `:637` |
| `#[serde(rename_all = "snake_case")]` (and friends) | Drives per-field/variant rename via `apply_rename_all` (snake_case, camelCase, PascalCase, SCREAMING_SNAKE_CASE supported) | `:235–275` |
| `#[serde(tag = "type")]` | Becomes `tag_field: Some("type")` on enums | `:643–647` |

**Per-variant attributes the live macro reads:**

| Attribute | Effect |
|---|---|
| `///` doc comment | `description: Some("...")` on `SchemaVariant::{Unit, Tuple, Struct}` (`:511–515`) |
| `#[serde(rename = "X")]` | `json_value: Some("X")` (`:518–520`) |
| Inherited `rename_all` via `apply_rename_all` | Auto-derives `json_value` from variant name when no explicit rename (`:521–527`) |
| `#[config(hidden)]` | Skip the variant entirely |

**Per-field attributes the live macro reads:**

| Attribute | Effect | Notes |
|---|---|---|
| `///` doc comment | `FieldMeta.description = Some("...")` (`:306–309`) | Live; missing on most field-within-variant declarations |
| `#[config(help = "...")]` | `FieldMeta.help = Some("...")` (`:302–305`) | |
| `#[config(default = X)]` | `FieldMeta.default = Some(ScalarValue::...)` (`:310–316`) | Bool/Char/Number/String literal |
| `#[config(min = X, max = Y)]` | `FieldMeta.range = Some(Range::new(Some(...), Some(...)))` (`:317–336`) | Either alone is allowed |
| `#[serde(rename = "X")]` | Field `json_key = Some("X")` (`:411`, `:593`) | |
| Inherited `rename_all` | Field `json_key` auto-derives (`:413–419`) | |
| `#[config(opaque)]` | Field schema becomes `SchemaNode::Opaque { type_name: stringify!(ty) }` (`:422–426`) | |
| `#[config(hidden)]` or `#[serde(skip)]` | Field omitted entirely (`:399–404`) | |
| `Option<T>` type detection | `FieldMeta.optional = true` (`:338`, via `is_option_type` `:278–285`) | Heuristic, segment-name match |
| `#[serde(default)]` | `FieldMeta.optional = true` (`:338`) | |

**Critical gaps the live macro does NOT cover** (load-bearing for migration decisions):

1. **No `where` clause augmentation.** `:366` forwards `where_clause` verbatim. Generic types whose body calls `T::schema()` need an explicit `where T: ConfigSchema` bound on the type definition for the derive to compile. This is the queued macro-extension packet referenced in `cls_bindable.rs:269–275`.
2. **No `#[serde(alias = "X")]` propagation.** `parse_serde_attrs` at `:186–189` consumes and discards `alias` to avoid parse errors. Variants with aliases (PathType's `helix`, `infinity`, `lemniscate`) won't expose them in the schema. Today's hand-written PathType impl also doesn't expose aliases — so this is neutral.
3. **No untagged-walk schema generation** for `#[serde(untagged)]` enums. The macro consumes `untagged` (`:201`) but emits the enum with `tag_field: None` regardless. Functionally a discard.

---

## Reclassified inventory — 19 hand-written sites total

The shipped baseline has 15 entries. The audit scanner is missing 4 more (qualified-path bug, §3). Total true-positive hand-written impls in scope: **19**.

| # | File:line | Type | Visible to scanner? | True classification | Live-macro derive feasibility |
|---|---|---|---|---|---|
| 1 | `schema/mod.rs:26-37, 38-52` | macro_rules `impl_*_schema!` | yes (skipped via `$t` heuristic) | macro-body — not a real impl | n/a |
| 2 | `schema/mod.rs:69` | `String` | yes | foreign std type | **Permanent.** Orphan rules forbid `#[derive]` on foreign types regardless of macro features. |
| 3 | `schema/mod.rs:77` | `&str` | yes | foreign std type | **Permanent.** Same. |
| 4 | `schema/mod.rs:85` | `Option<T>` | yes | foreign std type, generic | **Permanent.** Foreign type plus needs ConfigSchema bounds. |
| 5 | `schema/mod.rs:92` | `Vec<T>` | yes | foreign std type, generic | **Permanent.** Same. |
| 6 | `schema/mod.rs:99` | `Box<T>` | yes | foreign std type, generic | **Permanent.** Same. |
| 7 | `mixed_signals_schema.rs:32` | `SignalOrFloat` | yes | foreign type (mixed-signals) | **Permanent.** Orphan rules. |
| 8 | `mixed_signals_schema.rs:82` | `SignalSpec` | yes | foreign type (mixed-signals) | **Permanent.** Orphan rules. |
| 9 | `mixed_signals_schema.rs:467` | `EasingType` | yes | foreign type (mixed-signals) | **Permanent.** Orphan rules. (Note: would otherwise be derive-equivalent — 25 unit variants, no metadata — but cannot land here.) |
| 10 | `bindable/cls_bindable.rs:85` | `Never` | yes | local uninhabited enum | **Real candidate** if `tui-vfx-core` already depends on `tui-vfx-core-macros`. Empty enum derives to `SchemaNode::Enum { variants: vec![], … }` — equivalent to the hand-written impl. Verify the dependency edge before migrating. |
| 11 | `bindable/cls_bindable.rs:277` | `VfxBindable<T, S>` | yes | local generic, conditional schema | **Permanent until macro extension.** Two blockers: (a) live macro doesn't synthesize `T: ConfigSchema, S: BindableSignal` bounds on the `where` clause; (b) the hand-written impl uses `S::signal_variant_schema()` to conditionally include a `Signal` variant — runtime trait dispatch, not expressible in derive. Even with the bound-synthesis macro extension, blocker (b) remains. |
| 12 | `pool/cls_pool.rs:68` | `Pool<T>` | yes | local generic | **Conditional.** Live macro doesn't synthesize `T: ConfigSchema`. Migration requires either (i) the queued macro extension, or (ii) adding the bound explicitly to the type and reaching for derive — but the type today is `pub struct Pool<T> { … }` with no bound, so changing it adds a hard requirement on every `Pool<T>` instantiation that may break callers. **Stay hand-written until macro extension.** |
| 13 | `types/role_tag.rs:124` | `RoleTag` | yes | local enum | **Conditional candidate.** Has `///` doc comments on every variant. Adding `#[serde(rename_all = "snake_case")]` would make derive emit `json_value: Some("background")` etc. The blocker is the `Custom(InternedRoleName)` arm — `InternedRoleName` has NO ConfigSchema impl. Three sub-options: (a) add `impl ConfigSchema for InternedRoleName` (5-line impl emitting `SchemaNode::Primitive { type_name: "String", … }` to match current behavior — but creates a NEW hand-written impl that itself needs justification); (b) mark the field `#[config(opaque)]` (emits `SchemaNode::Opaque { type_name: "InternedRoleName" }` — different from String, breaking schema consumers); (c) keep hand-written and lift. **Recommendation:** (c) for now — the InternedRoleName ConfigSchema impl bundles into a separate question of whether tui-vfx-types should grow more schema impls. |
| 14 | `geometry/.../path_type.rs:201` | `PathType` | yes | local enum | **Real candidate with required source-edits.** Has `#[serde(tag = "type", rename_all = "snake_case")]` and `///` doc comments on all variant constructors, so `tag_field`, `json_value`, and per-variant `description` would derive correctly. Two metadata losses: (i) `Bounce.bounces` (default 3, range 0–12) and `Step.steps` (default 5) need `#[config(default = X, min = Y, max = Z)]` annotations on the field declarations to preserve the current schema; (ii) per-field descriptions like "Arc bulge factor", "Bezier control X", etc. are emitted by the hand-written impl via the `f32_field` helper but NOT present as `///` comments on the individual variant fields — migration requires adding the field-level doc comments source-side. Cost: ~30 small annotations. Benefit: the schema becomes derive-driven and stays in sync with attribute additions. **Decision deferred to leader.** |
| 15 | `style/.../fnc_style_region_schema.rs:21` (impl), `cls_style_region.rs` (enum def) | `StyleRegion` | yes | local enum | **Real candidate with schema-shape change.** Baseline note is doubly stale: (a) claims "tui-vfx-types does not depend on tui-vfx-core" — REFUTED by `tui-vfx-types/Cargo.toml:21`; (b) claims cross-crate-trait-dep — REFUTED because `RoleTag` has its own ConfigSchema impl. Real migration outcome: `#[serde(rename_all = "PascalCase")]` is on the type already, so derive would emit `json_value: Some("All")`, `Some("Role")`, etc. — different from the hand-written impl's `json_value: None` everywhere. Per-variant `description` would shift from the hand-written terse text to the (more informative) `///` doc comment text. Top-level `description` would gain the type's full doc-comment block (currently `None`). **The schema improves but the diff is non-trivial.** Decision: migrate (gain) vs. lift with `intentional-divergence` justification freezing the current schema (no diff for downstream consumers). |
| 16 | `types/color.rs:27` | `Color` | yes | local struct | **Trivial migration.** Add `#[config(min = 0, max = 255)]` on each of the four `u8` fields. The fields already have `///` comments ("/// Red channel (0-255)"). The hand-written impl's per-field `range` becomes derive-emitted; `description` already comes from the doc comments. **Recommend migrate.** |
| 17 | `style/.../cls_cursor_shader.rs:43` | `CursorShaderPrimary` | **NO — qualified path** | local struct, has `(u16, u16)` field | **Permanent.** The struct has `pub position: (u16, u16)`. Tuple types like `(u16, u16)` have no ConfigSchema impl in the workspace (verified by absence of any tuple impl in `schema/mod.rs`). Even if the scanner saw it, derive can't handle it without a tuple-type ConfigSchema impl or `#[config(opaque)]` on the field. **Lift with `intentional-divergence-from-derive-output`** and bring into the audit's vision via Phase 2. |
| 18 | `style/.../cls_cursor_shader.rs:81` | `CursorShaderTrail` | **NO — qualified path** | local struct, has `(u16, u16)` field | Same as 17. |
| 19 | `style/.../v3/enum_vfx_cursor_behavior.rs:37` | `VfxCursorPrimary` | **NO — qualified path** | local | **Unverified at this pass — file not read.** Likely similar to CursorShaderPrimary because the symmetry of the file naming (`enum_vfx_cursor_behavior` vs `cls_cursor_shader`) suggests parallel structure. Phase 0.1 verifies. |
| 20 | `style/.../v3/enum_vfx_cursor_behavior.rs:73` | `VfxCursorTrail` | **NO — qualified path** | local | Same — verify in Phase 0.1. |

**Summary by recommended action:**

- **Permanent — lift with foreign-type or generic-bound justification (8 entries):** String, &str, Option, Vec, Box, SignalOrFloat, SignalSpec, EasingType.
- **Permanent — lift with generic-bound justification (2 entries):** VfxBindable<T, S>, Pool<T>. Macro extension queued separately.
- **Conditional — lift today, revisit if InternedRoleName grows ConfigSchema (1 entry):** RoleTag.
- **Migrate to derive (1 entry, trivial):** Color.
- **Migrate to derive with source-edit cost (1 entry, ~30 annotations):** PathType.
- **Migrate to derive with schema-shape change (1 entry):** StyleRegion. Decision: migrate vs. freeze-and-lift.
- **Lift after scanner fix brings them into vision (4 entries, all `intentional-divergence`):** CursorShaderPrimary, CursorShaderTrail, VfxCursorPrimary, VfxCursorTrail (last two pending Phase 0.1 verification).
- **Migrate to derive (Never, 1 entry):** if the dependency edge from tui-vfx-core to tui-vfx-core-macros exists. Phase 0.2 verifies.

**Total post-migration baseline:** 0 entries (all lifted or migrated).

---

## Phasing

### Phase 0 — Plan-write-time verification was completed inline

Each baseline entry's source impl was read at plan-write time. Two small verification steps remain that were not done at plan-write time and are deferred to phase-execution start:

- **Phase 0.1.** Read `crates/tui-vfx-style/src/models/v3/enum_vfx_cursor_behavior.rs` end-to-end to confirm the two `VfxCursor*` impls' shape. Justifies their classification.
- **Phase 0.2.** Read `crates/tui-vfx-core/Cargo.toml` to confirm whether `tui-vfx-core` depends on `tui-vfx-core-macros`. If yes, `Never` is migration-eligible. If no, lift with `uninhabited-type` justification.

**Effort.** ~10 minutes total.

### Phase 1 — Fix the audit scanner's qualified-path bug

**Goal.** Make `cargo xtask audit configschema` see `impl tui_vfx_core::ConfigSchema for X` lines, not just `impl ConfigSchema for X`.

**Steps.**

1. Edit `xtask/src/audit/fnc_scan_file_for_impls.rs:102` (`strip_configschema`). Replace:
   ```rust
   fn strip_configschema(s: &str) -> Option<&str> {
       let rest = s.strip_prefix("ConfigSchema")?;
       Some(rest.trim_start())
   }
   ```
   with logic that accepts either:
   - The bare token `ConfigSchema`, OR
   - A qualified path ending in `::ConfigSchema` (e.g. `tui_vfx_core::ConfigSchema`, `crate::ConfigSchema`, `::tui_vfx_core::ConfigSchema`).
   
   Implementation sketch (15 lines):
   ```rust
   fn strip_configschema(s: &str) -> Option<&str> {
       // Accept either bare `ConfigSchema` or a path ending in `::ConfigSchema`.
       let rest = if let Some(r) = s.strip_prefix("ConfigSchema") {
           r
       } else {
           // Walk up to the last `::` and check the segment after it.
           let trait_name = "ConfigSchema";
           let token_end = s.find(' ').unwrap_or(s.len());
           let token = &s[..token_end];
           if token.ends_with(trait_name)
               && token.len() > trait_name.len()
               && token[..token.len() - trait_name.len()].ends_with("::")
           {
               &s[token_end..]
           } else {
               return None;
           }
       };
       Some(rest.trim_start())
   }
   ```

2. **Update `xtask/tests/test_audit_configschema.rs`** to add a fixture for the qualified-path case. Add `QUALIFIED_PATH_IMPL` constant and a test that confirms an impl written as `impl tui_vfx_core::ConfigSchema for NewType` is flagged when unjustified.

3. Run `cargo test -p xtask --test test_audit_configschema` — all 7 existing tests plus the new one should pass.

4. Run `cargo xtask audit configschema` against the workspace. **Expect a status flip:** the audit will now find the 4 invisible impls (CursorShaderPrimary, CursorShaderTrail, VfxCursorPrimary, VfxCursorTrail). In warn-only mode they appear as warnings; the build still passes (`Result::Ok`). Capture the warning output as evidence.

5. Bump `<VERS>` and update `<CLOG>` on `fnc_scan_file_for_impls.rs` and the test file.

**Deliverable.** PR titled `fix(xtask): audit-configschema scanner now matches qualified-path impls`. Includes the test fixture and the captured warning output as proof.

**Effort.** ~1 hour. The scanner edit is small; the test addition and verification are the bulk.

**Why this is Phase 1.** The scanner bug means today's "✓ All impl ConfigSchema for X blocks are justified or baselined" is silently false. Every later phase assumes the audit's scope is what we think it is. Fixing the scanner first is the precondition for trustworthy lift work.

### Phase 2 — Bring the 4 newly-visible impls into the audit's vision

**Goal.** After Phase 1, the four cursor impls become warnings. Resolve each so the warning count is zero before any later phase acts on the warning count.

**Steps.**

1. Phase 0.1 confirmed the shape of `enum_vfx_cursor_behavior.rs:37, :73`. If they look like the cursor-shader impls (have `(u16, u16)` tuple fields or similar derive-blockers), lift with `intentional-divergence-from-derive-output`. Otherwise classify per evidence.
2. For each of the 4 impls, add a `// CONFIGSCHEMA-JUSTIFICATION:` comment immediately above. Suggested for the cursor-shader pair (already verified):
   ```rust
   // CONFIGSCHEMA-JUSTIFICATION: intentional-divergence-from-derive-output:
   // struct has a (u16, u16) tuple field which has no ConfigSchema impl;
   // hand-written impl emits SchemaNode::Primitive { type_name: "(u16, u16)" }
   // to keep the SpatialShaderType enum derive non-fatal.
   impl tui_vfx_core::ConfigSchema for CursorShaderPrimary { … }
   ```
3. Run `cargo xtask audit configschema` — warning count drops to zero (or to whatever the baseline residual is at this point).

**Deliverable.** PR titled `chore: justify the 4 cursor ConfigSchema impls`. 4 small file edits + version bumps.

**Effort.** ~30 minutes after Phase 1 lands.

### Phase 3 — Lift the 8 foreign-type permanent entries

**Goal.** Each foreign-type entry becomes a source-side `CONFIGSCHEMA-JUSTIFICATION:` comment and is removed from the baseline.

**Entries to lift in this phase:**

| File | Type | Kind | Comment text |
|---|---|---|---|
| `tui-vfx-core/src/schema/mod.rs` | `String` | `primitive-bridge` | "foreign std primitive — orphan rules forbid #[derive]" |
| `tui-vfx-core/src/schema/mod.rs` | `&str` | `primitive-bridge` | "foreign std primitive — thin str wrapper alongside String" |
| `tui-vfx-core/src/schema/mod.rs` | `Option<T>` | `derive-cannot-handle-generic-T` | "foreign std generic — orphan rules + macro doesn't synthesize T: ConfigSchema bound" |
| `tui-vfx-core/src/schema/mod.rs` | `Vec<T>` | `derive-cannot-handle-generic-T` | (same) |
| `tui-vfx-core/src/schema/mod.rs` | `Box<T>` | `derive-cannot-handle-generic-T` | (same) |
| `tui-vfx-core/src/mixed_signals_schema.rs` | `SignalOrFloat` | `derive-cannot-handle-foreign-type` | "lives in mixed-signals; orphan rules forbid #[derive]" |
| `tui-vfx-core/src/mixed_signals_schema.rs` | `SignalSpec` | `derive-cannot-handle-foreign-type` | (same) |
| `tui-vfx-core/src/mixed_signals_schema.rs` | `EasingType` | `derive-cannot-handle-foreign-type` | (same) |

**Steps per entry.** Same as v2.0.0 prescribed — insert the `// CONFIGSCHEMA-JUSTIFICATION:` line immediately above the impl, delete the baseline TOML row, run audit, bump versions.

**Deliverable.** PR titled `chore: lift 8 foreign-type ConfigSchema baseline entries to source comments`.

**Effort.** ~2 hours.

### Phase 4 — Decide each derive-candidate entry

**Goal.** For each candidate, either migrate to `#[derive(ConfigSchema)]` or lift with `intentional-divergence-from-derive-output`. The decision is the leader's; the plan presents the data.

**Per-candidate analysis (from §Reclassified inventory):**

- **Color (entry 16):** **Recommend migrate.** Add `#[config(min = 0, max = 255)]` on each of the four fields. Replace the impl with `#[derive(ConfigSchema)]`. Schema diff: equivalent (assuming the live macro's `range` synthesis matches the hand-written `Range::new(...)` exactly — verify by capturing `to_json_schema(Color::schema())` before/after).
- **Never (entry 10):** **Recommend migrate iff Phase 0.2 confirms the dependency edge.** Empty enum derives to identical schema. If the dependency edge is missing, lift with `uninhabited-type` justification.
- **PathType (entry 14):** **Decision required.** Migration costs ~30 source-side `///` doc comments on individual variant fields plus 4 `#[config(default = X, min = Y, max = Z)]` annotations on `Bounce.bounces` (default 3, min 0, max 12), `Bounce.decay` (none — no metadata), `Step.steps` (default 5). Migration benefit: schema stays in sync with future attribute changes. **Cost-benefit favors migration if PathType is expected to grow more variants;** favors lift if it's stable.
- **RoleTag (entry 13):** **Recommend lift today.** Migration requires either a new `impl ConfigSchema for InternedRoleName` (just moves the hand-written count, does not reduce it) or a schema-shape change (Custom field becoming Opaque rather than String). Neither is a clean win. Revisit when InternedRoleName's role in the wider schema surface is decided.
- **StyleRegion (entry 15):** **Decision required.** Migration changes the schema's `json_value` fields from `None` to PascalCase strings (`Some("All")`, etc.) and changes `description` text from terse hand-written strings to fuller `///` doc-comment text. Both changes are arguably improvements (more accurate to the actual JSON wire shape; richer descriptions). The downstream blast on schema consumers must be assessed. `ofpf-blast crates/tui-vfx-style/src/models/fnc_style_region_schema.rs` returned 179 dependents (preview-truncated). Most are likely `should_style` consumers, not `schema()` consumers, but verify.

**Steps per migration (when chosen).**

1. Make the source-edit changes (annotations for Color/PathType, schema-acceptance for StyleRegion).
2. Run `cargo test -p <owning-crate>` — schema-shape tests catch unintended divergences.
3. Replace the hand-written impl with `#[derive(ConfigSchema)]` on the type definition.
4. Re-run tests; capture the schema-output diff if one of the dependent crates has a fixture that pins a JSON snapshot (search `cargo test --workspace 2>&1 | grep -i schema` to surface them).
5. Delete the baseline entry; bump versions.

**Steps per lift (when chosen).**

Same as Phase 3.

**Deliverable.** Per-entry PRs (or a combined PR if all decisions are "lift"). Each PR body carries the schema-output diff for migrations.

**Effort.** Per migration: 0.5 day for Color, 1 day for PathType (annotation work + dependent test runs), 1 day for StyleRegion (schema-diff assessment). Per lift: ~30 minutes.

### Phase 5 — Lift the 2 generic-bound permanent entries

**Goal.** Lift `VfxBindable<T, S>` and `Pool<T>` with `derive-cannot-handle-generic-T` justifications, even though they're locally-defined. The macro extension that would unblock them is its own packet (out of scope here).

**Steps.** Same as Phase 3.

**Deliverable.** Small PR, ~30 minutes.

### Phase 6 — Wire `audit-all` into umbrella recipes

**Goal.** Close the verified gap that `just check-all` and `just ci` do not invoke the gate.

**Steps.**

1. `justfile:178` — add `audit-all` to the `check-all` dependency list.
2. `justfile:326` — add `audit-all` to the `ci` dependency list.
3. Run both locally; both should still pass (warn-only mode → audit returns Ok).

**Deliverable.** Two-line PR.

**Effort.** ~10 minutes.

### Phase 7 — Decide warn-only → hard-fail promotion timing

Carries forward unchanged from v2.0.0. The shipped default at `fnc_audit_configschema.rs:69` is `WARN_ONLY = true` until 2026-07-01. Three options (hold default, move to release day, move earlier) — decision is the leader's.

**Strong recommendation if all of Phases 1–6 land before public release:** flip to hard-fail at release. With every entry justified or migrated and the umbrella wiring in place, hard-fail provides full protection from drift starting at v1.0. Hold the date if any phase slips.

### Phase 8 — Close

1. Update `docs/CONFIGSCHEMA_JUSTIFICATION.md` if any policy changed (e.g. promotion date moved, baseline drained to empty — execute the doc's own "remove baseline file when empty" instruction in that case).
2. Update `steering/INTENTIONS.md` Intention 12A pointer if needed.
3. Update the buy-once sweep (`docs/design/tui-vfx-buy-once-architecture-sweep.md`) §4 row for Finding 1.9.A. Mark DONE with closing-commit cite.

**Effort.** ~30 minutes.

---

## Pre-public-release recommendations

In priority order, with defense.

**Strong yes — Phase 1 (scanner bug fix).** Ships a real bug fix. Today's gate is silently incomplete (4 invisible impls). Until this fix lands, every claim about the gate's coverage is questionable. ~1 hour, no significant risk.

**Strong yes — Phase 2 (cursor-impl justifications).** Pure mechanical follow-on to Phase 1. ~30 minutes.

**Strong yes — Phase 3 (foreign-type lifts).** Drains 8 baseline entries with permanent reasons. The lift moves rationale from a TOML to source comments, where readers actually see them. ~2 hours.

**Strong yes — Phase 5 (generic-bound lifts).** Drains 2 more entries. ~30 minutes.

**Strong yes — Phase 6 (umbrella wiring).** Closes the verified gap that the gate doesn't run on `just ci`. ~10 minutes.

**Conditional — Phase 4 (derive candidates).** Color is trivial (do it). Never depends on Phase 0.2. PathType, RoleTag, StyleRegion are real decisions:
- **PathType:** 1-day cost; defensible to defer if the schedule is tight.
- **StyleRegion:** schema-shape diff is real; needs leader sign-off; migrate-or-freeze is genuinely a judgment call.
- **RoleTag:** recommend lift today regardless.

A reasonable pre-release scope is "Color migrate, Never migrate iff edge exists, the rest lift" — drains the baseline to zero in ~5 hours total.

**Hold default — Phase 7 (promotion date).** Keep 2026-07-01 unless Phases 1–6 all land cleanly before release. If they do, promoting on release day is defensible.

**Total estimated effort, all phases, pre-release scope:** ~1 day (incl. PathType migration if elected) or ~6 hours (if PathType lifts instead).

---

## Out of scope (named separately so they don't get bundled)

1. **Macro crate cleanup packet — DONE 2026-04-28** (commit `5401178`). `tui-vfx-core-macros/src/lib.rs` was 659 lines of inline logic + 9 dead OFPF-prefixed siblings; it is now a 37-line re-export hub plus 14 OFPF-prefixed siblings + types.rs. Pure refactor, byte-equivalent macro behavior verified via tests + clippy + docs check + audit. Cleanup plan: `docs/design/completed/tui-vfx-core-macros-lib-rs-cleanup-plan.md` (moved at close-out).
2. **Macro extension to synthesize `ConfigSchema` bounds.** The live macro at `lib.rs:352, 366` does not augment the `where` clause. Until that ships, `VfxBindable<T, S>` and `Pool<T>` cannot be derive-migrated. This is a `tui-vfx-core-macros` feature packet, separate from this followup. The hand-written impls in this packet's Phase 5 reference the macro extension explicitly via the `derive-cannot-handle-generic-T` justification kind.
3. **CI workflow creation.** No `.github/workflows/` exists in the repo today. `just ci` is a local-simulation recipe. CI infrastructure is its own concern and out of this packet.
4. **InternedRoleName ConfigSchema impl.** Bundled into RoleTag's "lift today" recommendation; revisited only if the wider schema surface decisions warrant it.

---

## Verification done at plan-write time

Per the project's complete-and-thorough planning rule (memory: `feedback_complete_thorough_planning.md`):

- Every load-bearing claim in this plan was sourced from a `Read` end-to-end of the cited file at plan-write time, not from the buy-once sweep doc, not from the prior plan revisions, not from grep alone.
- The full live macro at `tui-vfx-core-macros/src/lib.rs` was read across three windows totaling 659 lines.
- Each of the 15 baseline entries' source impls was read at the cited line. Most files were read end-to-end; a few were read in offset+limit windows that fully covered the impl body.
- The baseline TOML, the format-spec doc, and the peer test file were each read in full.
- `cargo xtask audit configschema` was run; output captured in §Verified state.
- `ofpf-defs InternedRoleName` was run to confirm the type has no ConfigSchema impl.
- `rg -n "impl\s+\w+::\w*ConfigSchema\s+for"` was run to enumerate all fully-qualified-path impls; 4 returned, all in the cursor cohort.
- `tui-vfx-types/Cargo.toml` was read to refute StyleRegion's stale baseline `note`.
- The dead OFPF-prefixed files in `tui-vfx-core-macros/src/` were read in full to confirm they are dead (no `mod` declarations import them; lib.rs has its own copies of every function with additional features).
- The live derive's behavior was traced from the proc-macro entry (`derive_config_schema` at `lib.rs:13`) through `impl_config_schema` (`:350`) into `derive_struct_schema` (`:381`) and `derive_enum_schema` (`:495`); attribute parsing through `parse_serde_attrs` (`:158`), `parse_config_attrs` (`:49`), `extract_doc_comments` (`:135`).

**Two small verification items that remain** (deferred to Phase 0.1 / 0.2 because they're cheap, mechanical, and only affect entry 10 (Never) and entries 19–20 (the V3 cursor pair)):

- Read `crates/tui-vfx-style/src/models/v3/enum_vfx_cursor_behavior.rs` end-to-end.
- Read `crates/tui-vfx-core/Cargo.toml` to confirm or refute the dependency edge to `tui-vfx-core-macros`.

These should be done before Phase 1 begins so Phase 4's per-entry decision matrix is finalized.

---

## Estimated effort

| Phase | Effort |
|---|---|
| 0.1 + 0.2 — final verification gaps | ~10 minutes |
| 1 — scanner bug fix + new test | ~1 hour |
| 2 — cursor justification comments | ~30 minutes |
| 3 — lift 8 foreign-type entries | ~2 hours |
| 4 — decide and execute derive candidates | 0.5–1 day depending on PathType / StyleRegion decisions |
| 5 — lift 2 generic-bound entries | ~30 minutes |
| 6 — wire audit-all into umbrellas | ~10 minutes |
| 7 — promotion-date one-line edit | ~5 minutes |
| 8 — close (sweep update, doc refresh) | ~30 minutes |
| **Total (lift-everything path)** | **~5 hours** |
| **Total (full-migration path)** | **~1.5 days** |

Single contributor; no cross-repo coordination.

---

## Why this earns its place (Intention 24 step-back test)

- **Real value, current scale.** 19 hand-written impls live in the codebase today. 15 are in a TOML allowlist where their rationale is invisible at the impl site. 4 are silently invisible to the gate entirely. The lift moves rationale to where readers see it; the scanner fix stops silent under-coverage. Both are concrete current pains.
- **Smallest viable mechanization** (Intention 25 rule 5). The gate already exists; this packet just drains the grandfather list, fixes a real coverage bug, and wires the gate into umbrella recipes. No new infrastructure.
- **Step-back test.** "Would I rather maintain a 15-row TOML + 4 invisible impls + an unwired umbrella for three years, or have every rationale at the impl site, every impl visible to the gate, and the gate running on every CI invocation?" The latter, evidently. The TOML and the scanner gap were the right starting state at packet 1.9.A; they are not the right resting state.
- **Reversion plan.** Each lift is a one-line source comment plus one TOML deletion — atomic. Each migration is ~5–30 source-line additions plus an impl deletion — small. The scanner fix is a 15-line function plus a peer test — bounded.

---

## Process note (the v1 → v2 → v3 progression)

This plan has gone through three substantial revisions. Each previous revision was **not** complete-and-thorough, and the next pass discovered claims the previous had not verified.

- **v1.0.0** prescribed reinventing infrastructure that was already shipped. Authored without reading the codebase. Wrong xtask path, wrong marker name, wrong vocabulary, no awareness of the baseline mechanism, no awareness of the format-spec doc, no awareness of warn-only mode and the promotion date.
- **v2.0.0** corrected the shipped-vs-pending status by reading the audit module, the baseline file, and the format-spec doc. But it read the **dead OFPF-prefixed files** in `tui-vfx-core-macros/src/` rather than the live `lib.rs`, and consequently got the derive macro's capabilities wrong: said PathType / RoleTag should be "investigated via attempted derive" with no grounding for the recommendation. Also missed the qualified-path scanner bug entirely (the four cursor impls were silently invisible to v2.0.0's analysis just as they're silently invisible to the gate).
- **v3.0.0** reads end-to-end. Confirms the live macro's behavior. Surfaces the scanner bug. Refutes a stale baseline `note` for StyleRegion. Identifies four invisible impls. Reclassifies every entry against the live macro rather than the dead OFPF code.

The discipline that should have produced v3.0.0 on the first pass: read every load-bearing file end-to-end at plan-write time. The user's directive captures this as a permanent rule — "complete and thorough is ALWAYS the bar." This plan honors it now. Future plans must honor it from the start.

If a future revision flips a central recommendation in v3.0.0 after one more verification round, that revision's CLOG will document v3.0.0's failure mode the same way this CLOG documents v1.0.0's and v2.0.0's. The pattern continues until verification at plan-write time matches the bar.

<!-- <FILE>docs/design/tui-vfx-configschema-audit-gate-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 3.1.0</VERS> -->
