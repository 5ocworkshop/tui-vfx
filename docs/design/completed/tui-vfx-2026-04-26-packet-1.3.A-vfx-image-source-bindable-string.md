<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-1.3.A-vfx-image-source-bindable-string.md</FILE> - <DESC>Implementation packet for buy-once sweep finding 1.3.A (VfxImageSource.image_name → BindableString). Self-contained execution brief with current-state audit, step-by-step plan, code sketches, test plan, acceptance criteria, verification commands. Captures that the schema lift + compile bridge already shipped at tui-vfx-recipes commit e64cf56 — packet acts as verification + follow-on workbook covering the still-deferred Phase 7.bytes runtime-end-to-end work.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Convert sweep finding 1.3.A into a runnable implementation packet. The handoff doc lists 1.3.A as queued; my OFPF audit shows the schema lift, compile-bridge resolver, runtime expect(), error variant, debug recipe, and 7-test coverage all landed at tui-vfx-recipes commit e64cf56. Packet documents the verification path, captures Phase 7.schema's exact wording from the cycle plan, and surfaces the still-deferred Phase 7.bytes runtime-binding-during-render work.</WCTX> -->
<!-- <CLOG>1.0.0: initial packet — pre-flight, current-state audit (work landed; verify), step-by-step plan written as if work were re-executable, code sketches matching the in-tree shape, test plan, acceptance criteria, verification commands. Captures Phase 7.schema's exact cycle-plan wording and the deferred Phase 7.bytes scope.</CLOG> -->

# Packet 1.3.A — VfxImageSource.image_name → BindableString

> **Source finding.** `docs/design/tui-vfx-buy-once-architecture-sweep.md` §1.3.A (lines 273–294), §7.3 diagram (lines 933–1027).
>
> **Cycle plan reference.** `docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md:22` — Phase 7.schema. Exact wording quoted in §Goal & motivation below.
>
> **Status note (2026-04-26).** OFPF audit at packet-write time confirms the schema lift, compile-bridge resolver, runtime contract, error variant, and the canonical `_bindable` debug recipe all shipped on `tui-vfx-recipes` at commit `e64cf56` ("Lift VfxImageSource.image_name to BindableString (1.3.A)"). The handoff doc `tui-vfx-2026-04-26-handoff-outstanding.md:20` predates the commit and lists 1.3.A as queued — it is stale. This packet is therefore a **verification + follow-on** workbook: the junior dev confirms the in-tree shape matches this packet, runs the verification commands, and notes the still-deferred Phase 7.bytes scope.
>
> **Risk tier (per sweep).** S — one-line schema change plus thin runtime adjustment.

---

## Goal & motivation

Lift the `image_name: String` field on `VfxImageSource` to `BindableString` so recipe authors can declare `{"binding": "selected_logo"}` (or `{"literal": "logo_light"}`, or the bare-string lenient form `"logo_light"`) and have the compile bridge resolve the binding before the scene runtime sees the value. Phase 7.schema of the cycle plan reads:

> One-line schema change in `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs` plus a thin runtime adjustment to evaluate the `BindableString` in the scene-layer composer. **Independent of L2** (the `{"binding": ...}` reference shape is V3 canon) and **independent of host-resolver consolidation**. Lands the *authoring* surface for asset binding without committing to the runtime end-to-end.

The work is independently actionable per the cycle plan and pays back authoring symmetry: `MechanicalContentSource::Preset.font` already uses `BindableString`. After the lift, the same binding surface covers fonts and assets, not a parallel "asset name vs. font name" seam.

## Scope

**In scope.**

- `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs` — change the `image_name` field type from `String` to `BindableString`.
- `tui-vfx-recipes/src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs` — add `resolve_image_source_bindings` helper that lowers a `Binding` arm to `Literal` using the existing `runtime_params + binding_defaults` chain.
- `tui-vfx-recipes/src/v3/compile/enum_build_composition_spec_error.rs` — add `MissingImageBinding` error variant for the unreachable-post-validate case.
- `tui-vfx-recipes/src/scene/layers/cls_image_layer.rs` — runtime contract: `expect()` that the compile bridge has already lowered the binding (the layer continues to consume a literal `&str`, no SceneCtx churn).
- `tui-vfx-recipes/recipes/debug_recipes/scene/scene_image_source_bindable.json` — new debug recipe per Intention 38's `_bindable` marker convention.

**Out of scope.**

- **Phase 7.bytes** — the cycle plan separates Phase 7.schema (this packet) from Phase 7.bytes (the *full* runtime end-to-end where a binding can resolve to bytes per-frame, not just at compile time). Phase 7.bytes stays deferred per the cycle plan and is the natural follow-on if/when an in-tree recipe asks for "swap the image asset every frame from runtime_params."
- `AssetRegistry` / `ImagePool` consolidation — sweep finding 1.2.C, separate packet.
- `aspect: VfxImageAspect` lift — sweep finding 1.3.B, explicitly speculative per the sweep's "leave alone" tier.
- Any change to `tui-vfx-style::models::BindableString` itself (already shipped at v0.1.0; this packet is its first real consumer outside the font-binding work).

**Crates / repos touched.** Only `tui-vfx-recipes`. The `tui-vfx-style::BindableString` import is the only cross-crate touchpoint and that crate already exists.

## Pre-work checklist

```bash
# Daemon health.
ofpf-status
ofpf-stats

# Load the recipes graph (separate repo from tui-vfx).
ofpf-load --root /usr/projects/tui-vfx-recipes
ofpf-status   # confirm source_root flips to tui-vfx-recipes

# Read the source finding and the cycle-plan Phase 7.schema entry.
sed -n '273,294p' /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md
sed -n '933,1027p' /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md
grep -A4 "Phase 7.schema" /usr/projects/tui-vfx/docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md

# Read the BindableString surface this packet plugs into.
ofpf-load --root /usr/projects/tui-vfx
ofpf-inspect crates/tui-vfx-style/src/models/cls_bindable_string.rs

# Re-load recipes; inspect the file the lift edits.
ofpf-load --root /usr/projects/tui-vfx-recipes
ofpf-inspect src/recipe_schema/scene/cls_ra_image_source.rs
ofpf-inspect src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs
ofpf-inspect src/v3/compile/enum_build_composition_spec_error.rs
ofpf-inspect src/scene/layers/cls_image_layer.rs

# Confirm the call sites for image_name across the repo.
grep -rn "image_name" /usr/projects/tui-vfx-recipes/src --include="*.rs"
```

## Current-state audit

Captured 2026-04-26 from the librarian after `ofpf-load --root /usr/projects/tui-vfx-recipes`.

| Path (relative to `tui-vfx-recipes/`) | Role | Current LOC | Fan-in | Fan-out | Notes |
|---|---|---|---|---|---|
| `src/recipe_schema/scene/cls_ra_image_source.rs` | unit | 106 | several recipe-schema sites | `tui-vfx-style::models::BindableString`, `tui-vfx-types::Color` | The lift target. v0.2.0 in tree. Field already typed `BindableString` post-`e64cf56`. |
| `src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs` | core | ~1100 | many | many | Hosts `resolve_image_source_bindings` at line 322. v0.9.0 in tree. |
| `src/v3/compile/enum_build_composition_spec_error.rs` | unit | ~80 | callers of the build helper | (none) | `MissingImageBinding` variant present at lines 73–80. v0.11.0 in tree. |
| `src/scene/layers/cls_image_layer.rs` | unit | ~80 | scene composition path | the asset resolver | Uses `image.image_name.literal().expect("compile bridge guarantees …")` at line 28. v0.2.0. |
| `recipes/debug_recipes/scene/scene_image_source_bindable.json` | debug recipe | n/a | preview / probe tooling | the schema | Canonical `_bindable` marker recipe per Intention 38. |

**Symbol-level call counts.**

- `grep -rn "image_name" /usr/projects/tui-vfx-recipes/src --include="*.rs"` returns 24 hits across 5 files (verified 2026-04-26): the schema (declaration + 4 inline tests), the compile bridge (resolver + 4 inline tests), the error variant (rustdoc + Display message), the runtime layer (`literal().expect(...)`), and `recipe_schema/config.rs:708` (an unrelated `image_name: Option<String>` on `RaPreset` — different field, different type, untouched by this packet).
- `BindableString::evaluate(&ShaderRuntimeParams) -> Option<&str>` at `crates/tui-vfx-style/src/models/cls_bindable_string.rs:63` is the existing call shape the lift inherits. The runtime layer does **not** call `evaluate` — the compile bridge resolves the binding before the layer sees the value, so the runtime calls `literal()` (synchronous accessor at `cls_bindable_string.rs:74`).
- `ofpf-blast crates/tui-vfx-style/src/models/cls_bindable_string.rs` reports 175 dependents (guarded — narrow with `--filter` if needed). Most are downstream font-binding sites; this packet adds one more (the image lift) without changing the type.

**Cycle-plan alignment confirmed.** The sweep doc cites `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs` as the file to edit. That path is current — no V3 Ra→Vfx rename has touched the file path itself. The struct inside has been renamed `RaImageSource` → `VfxImageSource` with a `pub use VfxImageSource as RaImageSource;` `#[doc(hidden)]` alias for the cutover (per `cls_ra_image_source.rs:55–61`).

## Open architectural questions

| # | Question | Recommended default | Source |
|---|---|---|---|
| (sweep §5) | None of the five §5 questions block this finding directly. The lift is independent of the §6.1 (`VfxCellContext`) and §6.2 (`Bindable<T>` consolidation) decisions. | n/a | sweep §5 |
| Phase 7.bytes deferral | Should this packet land Phase 7.bytes too (binding resolution at render time, not just compile time)? | **No.** Per the cycle plan, Phase 7.schema is independently actionable. Phase 7.bytes requires resolving against the asset bytes per frame, which means either (a) the scene composer carries `&ShaderRuntimeParams` (architectural ripple into SceneCtx) or (b) the compile bridge re-runs per frame (re-pays compile cost). Neither pays back without an in-tree recipe asking for it. Surface as a follow-on if such a recipe lands. | Cycle plan v0.7.0 line 22 |
| Default value | Is `BindableString::Literal(String::new())` the right default for `image_name`? | **Yes.** The struct's `Default` derives an empty literal, matching the previous `String::new()` default. Recipes that omit `image_name` continue to deserialize identically. | `cls_ra_image_source.rs:48` |
| Compile-bridge invariant scope | Should `MissingImageBinding` be a compile-bridge error or a validate-time error? | **Compile-bridge error with validate-time prevention.** Intention 37 mandates that every `requires_bindings` entry is checked at validate time; the compile-bridge error is the unreachable-post-validate sentinel that catches drift. | `enum_build_composition_spec_error.rs:73`, Intention 37 |

**Stop-and-ask trigger.** None for this packet. If the in-tree state diverges from §Current-state audit (e.g. `cls_ra_image_source.rs:35` no longer reads `pub image_name: BindableString`), surface to the user before continuing.

## Step-by-step implementation plan

Written as if the lift were being re-executed from a clean tree (i.e. with `image_name: String`). If the in-tree state already matches this packet, run §Verification commands and stop.

### Step 1 — Schema lift (`cls_ra_image_source.rs`)

1. **Pre-edit.** `ofpf-inspect src/recipe_schema/scene/cls_ra_image_source.rs`. Confirm `image_name: String` and that `tui-vfx-style::models::BindableString` is not yet imported.
2. **Write the failing test first** (TDD red). Add `deserializes_tagged_binding_image_name` in the inline tests module — assert that `{"image_name":{"binding":"selected_logo"}}` parses to `BindableString::Binding("selected_logo".to_string())`. Run `cargo test -p tui-vfx-recipes cls_ra_image_source::tests` — fails (the field is still `String`).
3. **Edit the field.** Change `pub image_name: String` to `pub image_name: BindableString`. Add `use tui_vfx_style::models::BindableString;`. Update `Default` to `BindableString::default()` (which is `Literal(String::new())`).
4. **Add the other inline tests** (per the in-tree shape at `cls_ra_image_source.rs:67–101`): `deserializes_bare_string_image_name_as_literal`, `deserializes_tagged_literal_image_name`, `default_image_name_is_empty_literal`. The bare-string test confirms backward-compatibility for existing recipes.
5. **Bump VERS** to 0.2.0; update `<WCTX>` and `<CLOG>` per the in-tree shape (lines 1–4).
6. **Verify.** `cargo test -p tui-vfx-recipes cls_ra_image_source::tests` — green.
7. **Build.** `cargo build -p tui-vfx-recipes` — fails because the compile bridge still treats `image_name` as `String`. Move to Step 2.

### Step 2 — Error variant (`enum_build_composition_spec_error.rs`)

1. Add the `MissingImageBinding { binding_key: String }` variant to `BuildCompositionSpecError`.
2. Display message: `"image_name binding `{binding_key}` did not resolve through runtime_params or loopback"`.
3. Rustdoc: explain that this variant is unreachable post-validate (Intention 37 enforces that every `requires_bindings` entry is declared at validate time) and surfaces a compile-bridge invariant violation.
4. Bump VERS, update `<WCTX>` and `<CLOG>`.
5. **Verify.** `cargo build -p tui-vfx-recipes` — still fails because the compile bridge does not yet emit the variant. Move to Step 3.

### Step 3 — Compile bridge (`fnc_build_scene_source_from_compiled_plan.rs`)

1. **Pre-edit.** `ofpf-around src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs resolve_layer_visibility` to find the existing pattern this helper mirrors.
2. **Write failing tests first.** Add a `tests_image_binding_resolution` module with seven tests:
   - literal pass-through (no resolver call needed).
   - runtime_params resolution (binding key present in `overrides.runtime_params`).
   - loopback fallback (binding key absent from `runtime_params`, present in `binding_defaults`).
   - missing binding (key absent from both → `MissingImageBinding`).
   - undeclared binding (key not in `binding_keys` set → `MissingImageBinding`).
   - non-image source (text / ansi → `Ok(())` no-op).
   - bare-literal source (`BindableString::Literal` arm → `Ok(())` no-op).
3. Run `cargo test -p tui-vfx-recipes tests_image_binding_resolution` — fails (function does not exist).
4. **Add the helper.** `fn resolve_image_source_bindings(source: &mut RaContentSource, overrides: &CompiledV3RuntimeOverrides, binding_defaults: &HashMap<String, serde_json::Value>, binding_keys: &HashSet<String>) -> Result<(), BuildCompositionSpecError>`. Body matches the canonical implementation at `fnc_build_scene_source_from_compiled_plan.rs:322` (see §Code snippets).
5. **Wire it in.** Call `resolve_image_source_bindings(&mut source, overrides, binding_defaults, binding_keys)?;` from `lower_scene_layer` after the existing `resolve_layer_visibility` call.
6. **Bump VERS** to 0.9.0; update CLOG.
7. **Verify.** `cargo test -p tui-vfx-recipes tests_image_binding_resolution` — green.

### Step 4 — Runtime contract (`cls_image_layer.rs`)

1. The layer's `paint` already reads `image.image_name` as a `String`. After the lift the field is `BindableString`. Change the read to `image.image_name.literal().expect("compile bridge guarantees BindableString::Literal for VfxImageSource.image_name")`.
2. The `expect()` encodes the runtime invariant — Step 3 guarantees a `Literal` arm. If the assertion fires in production, the bug is in the compile bridge, not the runtime.
3. Bump VERS to 0.2.0; update CLOG to reference Finding 1.3.A.
4. **Verify.** `cargo build -p tui-vfx-recipes` — green.

### Step 5 — Debug recipe (`scene_image_source_bindable.json`)

1. Per Intention 38, every Bindable surface gets an `_bindable` marker recipe. Write `recipes/debug_recipes/scene/scene_image_source_bindable.json`:
   - One scene layer carrying a `VfxImageSource` with `image_name: { "binding": "selected_logo" }`.
   - `requires_bindings: { "selected_logo": "logo_light" }` declaring the loopback default.
   - Minimal aspect / tint to keep the recipe focused on the binding surface.
2. **Verify.** `cargo run -p tui-vfx-recipes --bin pipeline-validator -- --recipe recipes/debug_recipes/scene/scene_image_source_bindable.json` (or whichever validator entry-point the repo currently exposes).

### Step 6 — Workspace verification

`cargo test --workspace`, `cargo clippy --workspace -- -D warnings`. See §Verification commands.

## Code snippets

**Schema lift** (`cls_ra_image_source.rs` — the load-bearing field declaration):

```rust
use serde::{Deserialize, Serialize};
use tui_vfx_style::models::BindableString;
use tui_vfx_types::Color;

/// Authored image source for a scene layer.
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct VfxImageSource {
    /// Image-pool key to render. Authors may write a bare string
    /// (`"splash_logo"`), an explicit literal (`{"literal":
    /// "splash_logo"}`), or a runtime binding (`{"binding":
    /// "selected_logo"}`) declared in `requires_bindings`. The V3 compile
    /// bridge resolves any `Binding` arm to a `Literal` before the scene
    /// runtime sees this value.
    pub image_name: BindableString,
    /// Optional tint applied to the image cells.
    #[config(opaque)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    /// Aspect policy used to fit the image into the layer rect.
    #[serde(default)]
    pub aspect: VfxImageAspect,
}

impl Default for VfxImageSource {
    fn default() -> Self {
        Self {
            image_name: BindableString::default(),  // Literal(String::new())
            tint: None,
            aspect: VfxImageAspect::Fit,
        }
    }
}
```

**Compile bridge resolver** (the canonical implementation at `fnc_build_scene_source_from_compiled_plan.rs:322`):

```rust
/// Resolve `VfxImageSource.image_name`'s `BindableString::Binding` arm to a
/// `Literal` so the scene runtime never observes a binding. Mirrors
/// [`resolve_layer_visibility`]'s lookup chain: host-supplied
/// `overrides.runtime_params` first, then the contract `binding_defaults`
/// (loopback, per Intention 37 enforced at validate time). If neither
/// resolves, returns `MissingImageBinding` — this should be unreachable
/// post-validate and surfaces a compile-bridge invariant violation.
fn resolve_image_source_bindings(
    source: &mut RaContentSource,
    overrides: &CompiledV3RuntimeOverrides,
    binding_defaults: &HashMap<String, serde_json::Value>,
    binding_keys: &HashSet<String>,
) -> Result<(), BuildCompositionSpecError> {
    let RaContentSource::Image(image) = source else {
        return Ok(());
    };
    let Some(binding_key) = image.image_name.binding_key() else {
        return Ok(());
    };
    if !binding_keys.contains(binding_key) {
        return Err(BuildCompositionSpecError::MissingImageBinding {
            binding_key: binding_key.to_string(),
        });
    }
    let resolved = overrides
        .runtime_params
        .get_text(binding_key)
        .map(str::to_string)
        .or_else(|| {
            binding_defaults
                .get(binding_key)
                .and_then(|value| value.as_str().map(str::to_string))
        });
    match resolved {
        Some(value) => {
            image.image_name = tui_vfx_style::models::BindableString::Literal(value);
            Ok(())
        }
        None => Err(BuildCompositionSpecError::MissingImageBinding {
            binding_key: binding_key.to_string(),
        }),
    }
}
```

**Error variant** (`enum_build_composition_spec_error.rs`):

```rust
#[derive(Debug, thiserror::Error)]
pub enum BuildCompositionSpecError {
    // ... existing variants ...

    /// `VfxImageSource.image_name` carried a `Binding` whose key did not
    /// resolve through `runtime_params` or `binding_defaults`. Unreachable
    /// post-validate (Intention 37 mandates every `requires_bindings`
    /// entry is checked at validate time); fires only on compile-bridge
    /// invariant violation.
    #[error("image_name binding `{binding_key}` did not resolve through runtime_params or loopback")]
    MissingImageBinding { binding_key: String },
}
```

**Runtime contract** (`cls_image_layer.rs`):

```rust
let image_name = source.image_name.literal().expect(
    "compile bridge guarantees BindableString::Literal for VfxImageSource.image_name",
);
let resolved = ctx.assets.resolve(image_name);
```

## Test plan

### Existing tests that must keep passing unchanged

- `cargo test -p tui-vfx-style cls_bindable_string` — the BindableString surface itself is unchanged; this packet only adds a new caller. All 14 inline tests in `cls_bindable_string.rs:194–319` pass.
- `cargo test -p tui-vfx-recipes` — every existing recipe-schema test, including the `MechanicalContentSource::Preset.font` font-binding tests that established the BindableString pattern.
- `cargo test -p tui-vfx-recipes tests::test_cls_ra_image_source` — the `tests/recipe_schema/test_cls_ra_image_source.rs` integration tests.

### New tests added during the lift

- `cls_ra_image_source.rs` inline tests (4): bare-string, tagged-literal, tagged-binding, default-empty-literal. These cover the schema-deserialization surface.
- `fnc_build_scene_source_from_compiled_plan.rs` `tests_image_binding_resolution` module (7): literal pass-through, runtime_params resolution, loopback fallback, missing binding, undeclared binding, non-image source, bare-literal source. These cover the compile-bridge resolver invariant.
- (Optional) `tests/recipe_schema/test_cls_ra_image_source.rs` — extend with end-to-end recipe-load tests if the repo's existing `test_cls_ra_image_source.rs` covers cross-module wiring. Per OFPF every new `fnc_` gets a paired `test_*` file; the inline `#[cfg(test)] mod tests` blocks satisfy this for `cls_ra_image_source.rs` (a `cls_` file), but the compile-bridge resolver helper `resolve_image_source_bindings` is a private `fn` inside a module — its inline tests are the paired surface.

### TDD red→green sequence

1. Red: `cargo test -p tui-vfx-recipes deserializes_tagged_binding_image_name` — fails (field is still `String`).
2. Edit `cls_ra_image_source.rs` per §Step 1.
3. Green: same command.
4. Red: `cargo build -p tui-vfx-recipes` — fails (compile bridge can't compile against `BindableString`).
5. Add `MissingImageBinding` variant per §Step 2.
6. Add `resolve_image_source_bindings` helper per §Step 3 (TDD red on the seven new tests, then green).
7. Edit `cls_image_layer.rs` per §Step 4.
8. Green: `cargo build -p tui-vfx-recipes`, `cargo test -p tui-vfx-recipes`.

### Integration test

`cargo test --workspace` is the final integration check. The debug recipe `scene_image_source_bindable.json` is exercised by the validator/probe pipeline — confirm via the validator binary or the probe-snapshot test corpus.

## Acceptance criteria

- [ ] `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs:35` reads `pub image_name: BindableString`.
- [ ] `tui-vfx-style::models::BindableString` is the import (not a local Bindable-like type).
- [ ] `Default` implementation produces `BindableString::Literal(String::new())`.
- [ ] All four inline tests in `cls_ra_image_source.rs` pass: bare-string, tagged-literal, tagged-binding, default.
- [ ] `BuildCompositionSpecError::MissingImageBinding { binding_key: String }` exists with the canonical Display message.
- [ ] `resolve_image_source_bindings` helper exists in `fnc_build_scene_source_from_compiled_plan.rs` and is called from `lower_scene_layer` after `resolve_layer_visibility`.
- [ ] All seven `tests_image_binding_resolution` tests pass.
- [ ] `cls_image_layer.rs` reads `image.image_name.literal().expect(...)` with the canonical "compile bridge guarantees" message.
- [ ] `recipes/debug_recipes/scene/scene_image_source_bindable.json` exists; filename carries the `_bindable` Intention 38 marker.
- [ ] **Serde shape preserved:** existing recipes with `"image_name": "logo_light"` continue to deserialize as `BindableString::Literal("logo_light".to_string())` via the lenient bare-string path.
- [ ] `cargo build --workspace` succeeds with zero new warnings.
- [ ] `cargo test --workspace` green.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [ ] No `#[allow]` suppressions added.
- [ ] No inert schema fields introduced (the `image_name` lift is a fully wired binding, not parse-and-inert).
- [ ] Rustdoc improved on `VfxImageSource.image_name`, `MissingImageBinding`, and `resolve_image_source_bindings` per `feedback_rustdoc_when_editing`.
- [ ] `cargo doc --no-deps` succeeds.
- [ ] If `VfxImageSource` appears in `docs/templates/capabilities.toml`, `cargo xtask docs generate` updates `docs/CAPABILITIES_REFERENCE.md`.
- [ ] Phase 7.bytes scope is **not** delivered — the runtime layer reads `literal()`, not `evaluate(&runtime_params)`. Phase 7.schema only.

## Verification commands

```bash
# Build clean across the workspace.
cd /usr/projects/tui-vfx && cargo build --workspace
cd /usr/projects/tui-vfx-recipes && cargo build

# Per-crate tests.
cd /usr/projects/tui-vfx-recipes && cargo test
cd /usr/projects/tui-vfx && cargo test -p tui-vfx-style cls_bindable_string

# Clippy.
cd /usr/projects/tui-vfx-recipes && cargo clippy --all-targets -- -D warnings
cd /usr/projects/tui-vfx && cargo clippy --workspace --all-targets -- -D warnings

# Rustdoc.
cd /usr/projects/tui-vfx-recipes && cargo doc --no-deps
cd /usr/projects/tui-vfx && cargo doc --no-deps

# Capability manifest regen (run from tui-vfx; only needed if VfxImageSource is in the manifest).
cd /usr/projects/tui-vfx && cargo xtask docs generate

# Validate the debug recipe against the schema.
cd /usr/projects/tui-vfx-recipes && cargo test scene_image_source_bindable
```

## Rollback plan

If the lift reveals a deal-breaker mid-execution (e.g. a downstream consumer in `gt-design` pattern-matches on `image_name: String` and the alias name doesn't catch it), recover via git:

1. Stop. Do not commit.
2. `cd /usr/projects/tui-vfx-recipes && git restore src/recipe_schema/scene/cls_ra_image_source.rs src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs src/v3/compile/enum_build_composition_spec_error.rs src/scene/layers/cls_image_layer.rs`.
3. If the debug recipe was committed, move it to `recyclebin/recipes/debug_recipes/scene/` per the recyclebin protocol.
4. `cargo build --workspace` to confirm the restored state compiles.
5. File a finding in the sweep doc capturing what blocked the lift, then surface to the user.

The lift touches only four files plus one new debug recipe; rollback is mechanical.

## Risks & gotchas

- **The `expect()` in `cls_image_layer.rs` is a runtime invariant.** If a downstream code path skips the compile bridge and feeds an unresolved `BindableString::Binding` to the layer, the assertion fires. The compile-bridge resolver in `fnc_build_scene_source_from_compiled_plan.rs` is the only legal lowering path; new code paths into the scene runtime must call the resolver before painting.
- **Phase 7.bytes is deliberately deferred.** Recipes can declare `{"binding": "selected_logo"}` today, but the binding resolves *once* (at compile/build time), not per frame. A recipe that wants per-frame asset switching needs Phase 7.bytes — the cycle plan defers this until an in-tree recipe asks for it.
- **The lenient bare-string path is load-bearing.** Existing recipes write `"image_name": "logo_light"`. The `BindableStringRepr::Bare` arm at `cls_bindable_string.rs:125` keeps these parsing as `Literal`. If a future schema change reorders the `BindableStringRepr` variants, the bare-string fallback must remain *last* (per the comment at `cls_bindable_string.rs:113–116`) — `serde(untagged)` walks variants in order, and tagged forms must win.
- **`MissingImageBinding` is unreachable post-validate.** Intention 37 mandates that every `requires_bindings` entry is checked at validate time. The error variant exists to catch compile-bridge invariant violations (e.g. a code path that builds a scene source without going through `lower_scene_layer`). If this fires in production, the bug is upstream of the compile bridge.
- **`Vfx*` prefix already applied.** `VfxImageSource` and `VfxImageAspect` were renamed from `RaImageSource` / `RaImageAspect` in the V3 cutover. The `#[doc(hidden)] pub use VfxImageSource as RaImageSource;` aliases at `cls_ra_image_source.rs:55–61` keep legacy imports compiling. Per Intention 8, the `Vfx` prefix is correct here (wire-format type crossing crate boundaries). Do not revert.
- **`config(opaque)` on `tint` — do not propagate to `image_name`.** The `tint: Option<Color>` field carries `#[config(opaque)]` because `Color` is a wire-format primitive whose schema is hand-written. `image_name: BindableString` does **not** need `opaque` because `BindableString` derives a real schema (hand-written impl at `cls_bindable_string.rs:138`).

## Sequencing note

- This packet stands alone. No queued sweep finding depends on the image-name lift landing first.
- The follow-on Phase 7.bytes (per-frame binding resolution) needs an in-tree recipe asking for it before it earns its place per Intention 24.
- Sweep finding 1.3.B (`BindableEnum<T>` for `aspect` and similar fields) is explicitly speculative and "leave alone" per the sweep — do not bundle.
- Sweep finding 1.2.A (`Bindable<T>` generalization) will eventually rename `BindableString` to `VfxBindable<String>`. Per the in-tree alias-migration mechanic, the lift here will inherit the new name with a `pub use VfxBindableString as BindableString;` shim. No re-edit needed in this file.
- The handoff doc `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md:20` should be updated to mark 1.3.A done once a junior dev confirms the audit matches this packet.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-1.3.A-vfx-image-source-bindable-string.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
