<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-gt-design-v3-stack-migration.md</FILE> - <DESC>Junior-ready implementation packet for migrating the gt-design recipe stack to V3 schema awareness. Captures the §8.5 item from tui-vfx-2026-04-26-handoff-outstanding.md v1.2.0. Self-contained execution brief: pre-flight, current-state audit of every crate boundary the migration crosses (loader, downstream payload consumers, factory bridge, recipes/resolve.rs RaContent path, vendored recipe inventory, test inventory), the tui-vfx-recipes V3 surface to consume, four open architectural decisions with recommended defaults, five-phase step-by-step plan with TDD red→green order, code snippets for the loader dispatch + the focused_row_btop V3 migration, test plan, acceptance criteria, verification commands, rollback plan, risks, sequencing note. Companion: full V3 schema text for focused_row_btop ready to drop in once the loader supports schema_version 3.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Convert handoff item §8.5 (gt-design V3 stack migration) into a runnable packet while context is fresh. gt-design has zero V3 awareness — every recipe is dispatched through tui_vfx_recipes::recipe::from_value (V2). 261 vendored recipes are all schema_version: 1. tui-vfx-recipes ships LoadedRecipeDocument + from_value_recipe_document as the central dispatch seam. Packet is documentation only — execution is deferred until the tui-vfx family (1.2.A, observability bus, Model B follow-ons) is finished.</WCTX> -->
<!-- <CLOG>1.0.0: initial packet. Audit captures loader path, GtdResolvedRecipes payload type and downstream consumers, recipes/resolve.rs RaContent typed-bridge surface, factory bridge through resolve_recipe_payload + apply_composition_with_routing, ContentShell::card producer-fix area, full vendored recipe inventory (261 files, all schema_version: 1, distributed across 11 themes), test inventory (12 recipe-touching test files). Documents the LoadedRecipeDocument dispatch surface in tui-vfx-recipes that the migration plugs into. Four open architectural questions with recommended defaults: dispatch site (recommend loader-internal via from_value_recipe_document so callers see one seam), payload shape (recommend keep serde_json::Value with a stored RecipeLoadMode marker — minimum-disruption option), migration order (recommend loader-first, per-recipe gradual after — the loader-first commit is the unblocker), §8.6 producer-fix coupling (recommend bundle into one packet — both touch role-tagging surface). Five-phase plan: loader dispatch, downstream consumer audit, vendored recipe migration, producer-side ContentShell fix, workspace verify. Includes complete V3 JSON migration of themes/eichler/recipes/focused_row_btop.json ready to drop in once the loader supports schema_version 3.</CLOG> -->

# Packet — gt-design V3 stack migration

> **Source.** `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` v1.2.0 §8.5 (item 7) and §8.6 (items 1 + 8). Companion to the V3 upgrade plan at `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md`.
>
> **Status (2026-04-26).** Documentation only. Execution deferred until the tui-vfx family is finished. Do **not** modify any code in gt-design while this packet sits in the queue.
>
> **Strategic context (user 2026-04-26).** "We haven't done the migration because we've not finished building the updated tui-vfx* family. That's our goal today, to finish that. Then we can turn our minds to gt-design." This packet captures the migration audit while context is fresh so the eventual execution is mechanical.
>
> **Risk tier.** L — multi-day. Touches three crates (`gtd-ssot`, `gtd-factory`, `gtd-ratatui`), 261 vendored JSON files, and a typed-bridge module (`crates/gtd-ratatui/src/recipes/resolve.rs`) that imports `Ra*` legacy schema types from tui-vfx-recipes. Cross-crate dependency churn risk is real if the V2 `Ra*` surface retires before this lands.

---

## Goal & motivation

Migrate gt-design's recipe stack so it can load and play V3-shape recipes (`schema_version: 3`) end-to-end: through SSOT loader dispatch, through downstream payload consumers (factory + ratatui integration), through the typed bridge that today reads V2 `RaRecipeConfig` types.

**Why now is wrong.** The tui-vfx supplier family is not finished. Items in flight or queued:

- 1.2.A `VfxBindable<T>` consolidation (parallel session).
- Pipeline observability bus (`docs/design/tui-vfx-pipeline-observability.md`, parallel session).
- Three Model B follow-on moves gated on item 2 acceptance (composition-model decision).
- `--runtime-params-json` documented as wired only on the compiled-V3 bridge — schema_v1 recipes drop the param silently (handoff §8.8).
- Probe-path fidelity regression (handoff §8.7) — the diagnostic surface gt-design's migration tests would lean on is currently lying.

Migrating downstream into a moving supplier surface guarantees rework. The handoff's lowest-cost preparation move is to write the packet now and execute later.

**Why later is right.** Once 1.2.A lands, the symmetric `VfxBindable<T>` family is stable; the V3 schema's `requires_bindings` block is stable; the observability bus has parity tests against a working probe; the Model B vocabulary is locked. Migration becomes: implement the dispatch, regenerate the recipes, fix the producer-tagging bug, run the tests.

**What this packet executes (when unblocked).**

1. Add V3 dispatch to gt-design's loader so `schema_version: 3` recipes parse without rejection.
2. Verify the GtdResolvedRecipes payload (currently typed `serde_json::Value`) survives the downstream factory + ratatui consumers regardless of schema version.
3. Audit and migrate the gtd-ratatui typed-bridge module (`crates/gtd-ratatui/src/recipes/resolve.rs`) that imports `RaRecipeConfig`-shape types — find a V3-aware substitute or convert to value-walking.
4. Migrate vendored recipes from schema_v1 to schema_v3, one feature area at a time. The migration is incremental — schema_v1 and schema_v3 coexist in `themes/*/recipes/` while the loader supports both.
5. Fix the §8.6 producer bug: `ContentShell::card` tags every cell `Surface` instead of distinguishing inner content from border. (See §Open architectural questions Q4 — this can bundle into the same packet.)
6. The failing test `test_content_shell_pipeline_override_without_animation_hints_uses_dwelling_effects` auto-closes when (a) the producer-fix lands OR (b) the focused_row_btop recipe migrates to V3 content-scope.

## Scope

**In scope.**

- Loader dispatch in `crates/gtd-ssot/src/resolve/fnc_resolve_recipes.rs` (the single V2 entry point).
- Downstream consumers of `GtdResolvedRecipes::named` (the `BTreeMap<String, serde_json::Value>` payload map).
- The factory bridge from resolved-recipes → `apply_composition_with_routing` (`crates/gtd-factory/src/render/orc_render_pipeline.rs:506`).
- The typed-bridge module `crates/gtd-ratatui/src/recipes/resolve.rs` that imports `RaRecipeConfig` / `RaContentConfig` / `RaContentMode` from `tui_vfx_recipes::recipe_schema`.
- Vendored recipe migration: 261 JSON files under `themes/*/recipes/` (audit table below).
- The §8.6 producer-side fix to `ContentShell::card` and `tag_scoped_semantics` (only if Q4 says bundle).
- Tests that exercise recipe loading or compositing: 12 files identified (audit table below).

**Out of scope.**

- V3 schema design. Locked in `docs/design/tui-vfx-v3-upgrade-plan/`. This packet consumes the schema, does not redesign it.
- Observability bus (`docs/design/tui-vfx-pipeline-observability.md`). Parallel work; this packet observes the bus contract but does not implement it.
- The probe-path fidelity regression (handoff §8.7). Land that fix before this packet starts so the migration tests have a working `recipe-probe` surface.
- The `--runtime-params-json` schema_v1 silent-drop fix (handoff §8.8). Land separately; doesn't block this packet but reduces investigation cost during execution.
- Sweep finding 1.2.A (VfxBindable consolidation). Must land before this packet — gt-design's typed-bridge module touches the Bindable family; doing the migration on three parallel hand-rolled types is more rework.
- Any change to the `tui-vfx-recipes` public surface. The dispatch already exists (`from_value_recipe_document` returning `LoadedRecipeDocument`) — this packet is the consumer.

**Crates touched.**

- `gtd-ssot` (loader dispatch + GtdResolvedRecipes payload).
- `gtd-factory` (downstream payload consumer + the Path A producer fix if bundled).
- `gtd-ratatui` (typed-bridge module + ContentShell + tests).
- `themes/*/recipes/*.json` (vendored recipe migration, 261 files across 11 themes).

## Pre-work checklist

```bash
# Daemon health on all three repos. Bring siblings online if not already.
ofpf-status
ofpf-load --root /usr/projects/gt-design
ofpf-load --root /usr/projects/tui-vfx-recipes
ofpf-stats

# Read the source items.
sed -n '209,233p' /usr/projects/tui-vfx/docs/design/tui-vfx-2026-04-26-handoff-outstanding.md   # §8.5 + §8.6

# Read the V3 upgrade plan index.
sed -n '1,80p' /usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md
sed -n '1,80p' /usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md

# Inspect the loader entry point.
ofpf-inspect /usr/projects/gt-design/crates/gtd-ssot/src/resolve/fnc_resolve_recipes.rs

# Inspect the resolved-recipes payload type.
ofpf-inspect /usr/projects/gt-design/crates/gtd-ssot/src/types/GtdResolvedRecipes.rs

# Inspect the factory bridge surfaces.
ofpf-inspect /usr/projects/gt-design/crates/gtd-factory/src/vfx/fnc_resolve_recipe_payload.rs
ofpf-inspect /usr/projects/gt-design/crates/gtd-factory/src/render/orc_render_pipeline.rs
ofpf-extract /usr/projects/gt-design/crates/gtd-factory/src/render/orc_render_pipeline.rs apply_composition_with_routing

# Inspect the typed-bridge module that imports Ra* types.
ofpf-inspect /usr/projects/gt-design/crates/gtd-ratatui/src/recipes/resolve.rs

# Inspect the producer-fix area (item §8.6 / Path B).
ofpf-inspect /usr/projects/gt-design/crates/gtd-ratatui/src/widgets/util/cls_content_shell.rs
ofpf-inspect /usr/projects/gt-design/crates/gtd-ratatui/src/integration/fnc_tag_interaction_scoped_semantics.rs
ofpf-extract /usr/projects/gt-design/crates/gtd-factory/src/semantic/cls_semantic_buffer.rs tag_cell

# Inspect the V3 surface in tui-vfx-recipes that the migration plugs into.
ofpf-inspect /usr/projects/tui-vfx-recipes/src/recipe/fnc_from_value_recipe_document.rs
ofpf-inspect /usr/projects/tui-vfx-recipes/src/recipe/enum_loaded_recipe_document.rs
ofpf-inspect /usr/projects/tui-vfx-recipes/src/v3/fnc_normalize_v3_document.rs
ofpf-inspect /usr/projects/tui-vfx-recipes/src/v3/normalized/cls_normalized_recipe.rs

# Find every gt-design call site that touches a recipe payload.
ofpf-refs GtdResolvedRecipes
ofpf-content "named\." --glob "**/*.rs"   # narrow with the largest call-site files
ofpf-refs RaRecipeConfig
ofpf-refs RaContentConfig
ofpf-refs RaContentMode

# Confirm the failing test still reproduces today.
cd /usr/projects/gt-design && cargo test -p gtd-ratatui --test test_content_shell \
  test_content_shell_pipeline_override_without_animation_hints_uses_dwelling_effects 2>&1 | tail -20

# Confirm vendored recipe inventory shape.
find /usr/projects/gt-design/themes -name "*.json" -path "*/recipes/*" | wc -l
grep -h "schema_version" /usr/projects/gt-design/themes/*/recipes/*.json | sort | uniq -c
```

## Current-state audit

Captured 2026-04-26 from the librarian.

### A. Loader path

| Path | Role | LOC | Fan-in | Fan-out | Key callees |
|---|---|---|---|---|---|
| `crates/gtd-ssot/src/resolve/fnc_resolve_recipes.rs` | unit | ~140 | 1 (`crates/gtd-ssot/src/orc_build_resolved_design.rs`) | 5 | `tui_vfx_recipes::recipe::from_value`, `validate_recipe_refs`, `load_json_at_path`, `resolve_manifest_refs_in_json`, `resolve_json_colors` |

The single dispatch line is at `fnc_resolve_recipes.rs:55`:

```rust
if let Err(err) = tui_vfx_recipes::recipe::from_value(payload.clone()) {
    return Err(GtdSsotError::RecipeCompatibility {
        key: key.clone(),
        message: err.to_string(),
    });
}
named.insert(key.clone(), payload);
```

This is the V2 compatibility check. It parses the recipe through `from_value` (which goes through V2's `VfxJsonRecipeDefinition` struct) and discards the parsed result — only the original `serde_json::Value` is stored in `named`. The check is pass/fail; nothing else uses the parsed legacy `Recipe` here. **This is the single edit point that gates V3 acceptance.**

### B. GtdResolvedRecipes payload type

| Path | Role | LOC | Fan-in | Notes |
|---|---|---|---|---|
| `crates/gtd-ssot/src/types/GtdResolvedRecipes.rs` | unit | ~296 | many | Stores `BTreeMap<String, serde_json::Value>` per `named()`. Recipes are intentionally opaque per Intention 17.3 ("the one intentional opaque-JSON pass-through"). |

The type's docstring (lines 28–48) explicitly says: "Adding new typed accessors or a second opaque exception is an SSOT contract change, not a workaround to be implemented downstream." Translation: the migration must keep `named` as `BTreeMap<String, Value>` unless the leader signs off on a contract change.

### C. Downstream payload consumers

`ofpf-refs GtdResolvedRecipes` returns these distinct call sites that read `.named()` or otherwise touch the payload:

| Path | What it does with the payload |
|---|---|
| `crates/gtd-factory/src/vfx/fnc_resolve_recipe_payload.rs` | Returns `Value` clone via key lookup. Schema-version-agnostic. |
| `crates/gtd-ratatui/src/recipes/resolve.rs` | **Imports `RaRecipeConfig` / `RaContentConfig` / `RaContentMode`** and calls `serde_json::from_value::<RaRecipeConfig>(...)`. **This is the V2-only path that breaks under schema_v3.** |
| `crates/gtd-ratatui/src/recipes/planner.rs` | Reads payload through the resolve module. |
| `crates/gtd-ratatui/src/recipes/render.rs` | Reads payload through the resolve module. |
| `crates/gtd-ratatui/src/recipes/item.rs` | Builds runtime items via the resolve module. |
| `crates/gtd-factory/src/vfx/fnc_pipeline_to_composition.rs` | Walks the pipeline JSON; depends on V2 pipeline shape. |
| `crates/gtd-ratatui/src/splash/fnc_show_from_recipe.rs` + `cls_splash_recipe.rs` + `fnc_render_splash_frame.rs` | Splash playback. |
| `crates/gtd-ssot/src/validate/fnc_validate_recipe_refs.rs` | Validates the recipe-refs map (independent of payload schema). |

The load-bearing finding: the typed-bridge module at `crates/gtd-ratatui/src/recipes/resolve.rs` (header v0.1.1) imports `RaRecipeConfig` from `tui_vfx_recipes::recipe_schema`. That schema is V2-shaped. **This is where schema_v3 payloads will fail downstream even after the loader accepts them.** The packet's Phase 2 audit needs to either (a) replace this typed-bridge with a V3-aware equivalent, or (b) gate it on schema_version and fall back to a V3 path for v3 recipes.

### D. Factory bridge

| Path | Role | LOC | Fan-in | Notes |
|---|---|---|---|---|
| `crates/gtd-factory/src/vfx/fnc_resolve_recipe_payload.rs` | unit | ~52 | many | Just looks up + clones a `Value`. Schema-version-agnostic; no edit needed. |
| `crates/gtd-factory/src/render/orc_render_pipeline.rs` | orc | ~1900 | many | The dispatcher that reaches `apply_composition_with_routing` (line 506). Walks the pipeline JSON. Depends on V2 pipeline shape (`pipeline.style.spatial_shader` etc.). |
| `crates/gtd-factory/src/vfx/fnc_pipeline_to_composition.rs` | unit | TBD | ? | Builds composition options from the pipeline JSON. V2-shaped. |

`apply_composition_with_routing`'s signature accepts `composition: CompositionOptions<'_>` and a pre-rendered `source_grid` + `source_roles`. The composition options are built upstream from the V2 pipeline JSON. Migration order matters here: the V3 pipeline is `pipeline.step.{kind,phase,scope,payload}` — totally different from V2's `pipeline.style.spatial_shader`. Path forward (per Q1 default): use `tui_vfx_recipes::recipe::from_value_recipe_document` as the central dispatch, return a `LoadedRecipeDocument`, then route V3 documents through the **canonical playback-item builder** (V3 Decision 8 in `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md`) — that builder is upstream's responsibility.

### E. Compositor bridge — runtime params + bindings

The runtime-params surface today uses `GtdRuntimeParams` (a `BTreeMap<String, GtdRuntimeParamValue>`) on `GtdWidgetPipelineHints`. The hints flow from `ContentShell` → integration layer → factory render. V3 recipes declare bindings via `requires_bindings`; the runtime injects values through the same hint surface. **No surface change required at the gt-design boundary**; the V3 dispatch translates the runtime params into the V3 binding context internally.

The packet's Phase 2 must verify this end-to-end: a V3 recipe with `requires_bindings: { selected_row: { type: u16, default: 4 } }` plus `GtdRuntimeParams { selected_row: 3 }` reaches the V3 shader as `selected_row = 3`. This is the same surface that today fails silently for schema_v1 recipes per handoff §8.8.

### F. Vendored recipes inventory

```
gt-design/themes/<theme>/recipes/<recipe>.json
```

| Theme | File count |
|---|---|
| `defaults` | 59 |
| `harbor` | 37 |
| `eichler` | 31 |
| `grimoire` | 23 |
| `blueprint` | 19 |
| `flw` | 19 |
| `stuttgart` | 19 |
| `japanese-minimalism` | 18 |
| `hygge` | 16 |
| `rams` | 16 |
| `eichler-full` | 4 |
| **Total** | **261** |

Schema-version distribution:

```
$ grep -h "schema_version" /usr/projects/gt-design/themes/*/recipes/*.json | sort | uniq -c
    260   "schema_version": 1,
```

(The 261st file lacks the field — `themes/defaults/recipes/baseline_p0_manifest.json` is not a recipe in the strict sense; check whether it should be relocated.)

**Migration scope: 260 recipes across 11 themes.** Per Q3 default the migration is incremental — the loader supports both schema_v1 and schema_v3 simultaneously, recipes migrate in waves grouped by feature area (e.g. all toast recipes together, all card recipes together, all focused-row recipes together).

### G. Test inventory

Recipe-touching test files in gt-design:

| Path | What it tests |
|---|---|
| `crates/gtd-ssot/tests/test_recipes_schema_compat.rs` | Loader compatibility against `tui_vfx_recipes::recipe::from_value`. **Must extend to cover V3 dispatch.** |
| `crates/gtd-ssot/tests/test_recipes_raw_rgb_passthrough.rs` | RGB passthrough preservation. Schema-version-agnostic. |
| `crates/gtd-ssot/tests/test_recipes_color_role_resolution.rs` | Role-color resolution at load. Should keep working under V3 (the `@role.*` substitution is pre-loader). |
| `crates/gtd-ssot/tests/test_recipes_manifest_ref_resolution.rs` | `@motion/@palette/...` substitution. Schema-version-agnostic. |
| `crates/gtd-ssot/tests/test_phase0_smoke_test_recipe.rs` | One canonical recipe end-to-end. Should add a V3 sibling. |
| `crates/gtd-ssot/tests/test_resolved_recipe_inspection.rs` | The `GtdResolvedRecipeRef` inspection helpers. |
| `crates/gtd-factory/tests/test_pipeline_bridge.rs` | Pipeline JSON → CompositionOptions. **V2-shaped pipeline; must add V3 cases.** |
| `crates/gtd-factory/tests/test_recipe_playback_parity.rs` | Recipe playback parity vs. canonical. **Must extend to cover V3 playback.** |
| `crates/gtd-factory/tests/test_torch_vfx_diagnostic.rs` | Diagnostic surface around VFX. |
| `crates/gtd-ratatui/tests/test_content_shell.rs` | The shell that owns the failing test. **Two failures auto-close.** |
| `crates/gtd-ratatui/tests/test_recipe_scene_canvas.rs` | Recipe-driven scene canvas rendering. |
| `crates/gtd-ratatui/tests/test_recipe_playback_api.rs` | Public playback API. |
| `crates/gtd-theme-builder/tests/test_recipe_viewer.rs` | The recipe viewer in the theme builder app. |

### H. Producer-fix area (item §8.6, optional bundle per Q4)

| Path | Role | LOC | Notes |
|---|---|---|---|
| `crates/gtd-ratatui/src/widgets/util/cls_content_shell.rs` | cls | ~400 | `card()` / `drawer()` / `modal()` constructors all default `role: GtdWidgetPartRole::Surface`. Inner content + border are not distinguished. |
| `crates/gtd-ratatui/src/integration/fnc_tag_interaction_scoped_semantics.rs:111` | unit | ~130 | `tag_scoped_semantics` calls `tag_area(semantic_buffer, area, semantic_cell)` which lowers `SemanticRole::Surface` → `RoleTag::Background` for every cell. |
| `crates/gtd-factory/src/semantic/cls_semantic_buffer.rs:88-90` | reference | — | `Surface → Background`, `Content/Text → Text`, `Border → Border`. These are the lowering rules the producer fix must respect. |

The §8.6 producer fix is ~50 LOC: tag inner content cells `SemanticRole::Content`, tag border cells `SemanticRole::Border`. Plus a regression test asserting `RoleMapMaterialized.histogram` for a card render contains text-tagged inner cells.

## Tui-vfx-recipes V3 surface to consume

The tui-vfx-recipes crate already ships the dispatch surface this packet plugs into. **No new public surface needs to be added to tui-vfx-recipes for this migration** — the consumer-side adapter in gt-design is the work.

| Surface | Path | What it returns |
|---|---|---|
| `tui_vfx_recipes::recipe::from_value_recipe_document` | `src/recipe/fnc_from_value_recipe_document.rs` | `LoadedRecipeDocument` enum (LegacyRecipes \| V3Document \| V3Normalized \| V3Compiled) — dispatches on `schema_version` field. |
| `tui_vfx_recipes::recipe::LoadedRecipeDocument` | `src/recipe/enum_loaded_recipe_document.rs` | The four-variant union. Documented as "the centralized loaded-recipe union for legacy and V3 dispatch". |
| `tui_vfx_recipes::recipe::RecipeLoadMode` | `src/recipe/enum_recipe_load_mode.rs` | `Parsed` \| `Normalized` \| `Compiled`. Selects which V3 variant the dispatch returns. |
| `tui_vfx_recipes::recipe::from_value_v3` | `src/recipe/fnc_from_value_v3.rs` | `V3RecipeDocument`. The authoring-shape parse. |
| `tui_vfx_recipes::recipe::from_value_v3_normalized` | `src/recipe/fnc_from_value_v3_normalized.rs` | `NormalizedRecipe`. Authoring → normalized IR in one call. |
| `tui_vfx_recipes::v3::compile_v3_document` | `src/v3/fnc_compile_v3_document.rs` | `CompiledRecipePlan`. The compiled-V3 direct bridge — the runtime payload that the V3 runtime consumes. |
| `tui_vfx_recipes::v3::V3RecipeDocument` + `NormalizedRecipe` + `CompiledRecipePlan` | re-exported from `tui_vfx_recipes::v3::*` | The three concrete types. |

The compiled-V3 direct bridge is the V3 runtime integration target (per the `pipeline-validator --runtime-params-json` documentation referenced in handoff §8.8). The packet's Q2 default keeps `GtdResolvedRecipes::named` as `BTreeMap<String, Value>` and dispatches at the consumer site, which means the V3 path must call `from_value_recipe_document(payload, RecipeLoadMode::Compiled)` to build the runtime plan on demand. See Q2 for the alternative.

## Open architectural questions

These need leader decision before execution. Recommended defaults are present so a junior can apply them if no other guidance arrives.

### Q1 — Where does dispatch live?

| Option | Trade-off |
|---|---|
| A — In gt-design's loader (`fnc_resolve_recipes.rs:55`). The loader checks `schema_version`, calls `from_value` for v1 and `from_value_v3` for v3. | One seam, gt-design owns the version-awareness. The pre-validation step the loader already does (catching invalid recipes at load time, not at render time) extends naturally. |
| B — In tui-vfx-recipes via the existing `from_value_recipe_document` dispatch. gt-design's loader calls one function and gets a `LoadedRecipeDocument`. | Single dispatch surface lives upstream; future schema_v4 doesn't require a gt-design edit. The loader stores `LoadedRecipeDocument` instead of `Value` (Q2 sub-decision). |
| C — At the consumer site (`crates/gtd-ratatui/src/recipes/resolve.rs` and friends). Loader stays unchanged; each downstream consumer dispatches on schema_version. | Smallest loader edit — none. But every downstream consumer pays the dispatch cost; eight call sites in §C above each grow a match arm. |

**Recommended default: B (call `from_value_recipe_document` from gt-design's loader).** Rationale:

- Per Intention 3, recipe-authoring truth lives in `tui-vfx-recipes`. Schema-version dispatch is part of that truth. Hosting it upstream means downstream consumers wrap one centralized seam rather than reinterpreting it.
- Per Intention 5 (loader scope: parse → substitute → resolve → build), the loader is the right place to lift parsed-and-validated recipes. `from_value_recipe_document` is exactly that.
- The tui-vfx-recipes test at `fnc_from_value_recipe_document.rs:42-89` already covers both legacy and V3 dispatch — the seam is verified upstream.
- Future `schema_version: 4` doesn't require a gt-design edit; the upstream dispatch grows a match arm and gt-design picks it up by re-fetching.

If the leader rejects B in favor of A, the migration cost difference is small: the loader inlines the schema_version match instead of calling `from_value_recipe_document`. The packet's Phase 1 changes are otherwise identical.

### Q2 — `GtdResolvedRecipes::named` payload shape

| Option | Trade-off |
|---|---|
| A — Keep as `BTreeMap<String, serde_json::Value>` (status quo). Downstream consumers re-dispatch on schema_version when they need typed access. | Zero contract change. Honors Intention 17.3 ("intentional opaque-JSON pass-through"). The cost is downstream re-dispatch — but most downstream consumers (factory, splash) walk the JSON anyway. |
| B — Become `BTreeMap<String, LoadedRecipeDocument>`. The dispatch happens once at load time; downstream consumers receive a typed enum. | Eliminates downstream re-dispatch. **But:** changes the SSOT contract documented at `GtdResolvedRecipes.rs:28-48`. Cross-crate type leak — `GtdResolvedRecipes` would now publicly carry a `tui_vfx_recipes::v3::*` re-export through the `LoadedRecipeDocument` enum. |
| C — Become `enum { V2(serde_json::Value), V3(NormalizedRecipe) }`. A two-variant gt-design-local wrapper. | Captures the schema-version distinction without the four-variant LoadedRecipeDocument leak. Adds one type to maintain. |

**Recommended default: A (keep `BTreeMap<String, Value>`).** Rationale:

- Minimum-disruption — preserves the documented Intention 17.3 carve-out without re-litigating.
- The downstream re-dispatch cost is a single match arm at each consumer site (Q1's option C cost). With option B for Q1 (loader-side dispatch), the loader can do the dispatch + store the V3 normalized form alongside the original Value if the leader wants both — but storing both doubles the memory and adds a synchronization concern.
- Does not require changing GtdResolvedRecipes' documented contract.
- The V3-aware downstream adapter (the new replacement for `crates/gtd-ratatui/src/recipes/resolve.rs`) handles the re-dispatch in one place.

If the leader rejects A in favor of B or C, **stop and surface to the user** — this is a documented SSOT contract change per the GtdResolvedRecipes docstring.

### Q3 — Migration order

| Option | Trade-off |
|---|---|
| A — Loader-first (accept v3), then per-recipe migration in waves. | Smallest unblocking commit. After Phase 1 lands, the loader accepts v3 recipes immediately; recipe migration becomes incremental. Old + new schema versions coexist in the tree. |
| B — Per-recipe migration first (rewrite all 260 recipes to v3), then retire the v1 path in one cutover. | One flag-day commit. **But:** every recipe must be migrated before any test passes, which means the migration is a single uninterruptible work block — risky for a multi-day packet. |
| C — Hybrid: migrate one feature area's recipes (e.g. focused-row) end-to-end first as a vertical slice, then migrate the rest in waves. | Slightly larger first commit than A but proves the pipeline end-to-end on one real recipe before migrating in bulk. |

**Recommended default: A (loader-first).** Rationale:

- Per the OFPF discipline, smallest commit that earns the most. Phase 1 is one file edit (loader) plus the GtdSsotError variant; it unblocks every subsequent phase.
- Per Intention 23 rule 4 (additive migration, never breaking churn), v1 and v3 must coexist during the migration window. Option A is the only one that supports this directly.
- Once Phase 1 lands, the failing `test_content_shell_pipeline_override_without_animation_hints_uses_dwelling_effects` can be auto-closed via path A (V3 recipe migration of `focused_row_btop.json`) without touching anything else.

### Q4 — §8.6 producer-fix coupling

| Option | Trade-off |
|---|---|
| A — Bundle the §8.6 producer fix (ContentShell::card role tagging) into this packet. | Both touch role-tagging surface; the V3 content-scope migration of focused_row_btop.json depends on the producer being honest about which cells are content vs. surface. One commit covers both fixes. |
| B — Ship as separate packets. | Smaller packets, tighter rollback. But the producer fix is gated on someone having context about the full role-tagging contract — that context lives inside this audit. |

**Recommended default: A (bundle).** Per handoff §8.6 final paragraph: "when item 7 starts, fold path B into the same packet — both touch gt-design's role-tagging surface." The producer fix is independent of schema version, so it can land before, with, or after the loader dispatch. Bundling concentrates the role-tagging context in one packet.

If the leader rejects A in favor of B, the producer fix becomes its own ~half-day packet — same scope, just split. The migration packet then closes the failing test via Path A (V3 content-scope migration) only.

### Stop-and-ask triggers

If the user has not pre-decided Q1, Q2, or Q4, **stop after pre-flight and surface to the user**. Q3 (migration order) has a defensible default a junior can apply.

## Step-by-step implementation plan

OFPF discipline: edit one file at a time, write tests first (red), implement (green), confirm clippy clean, commit interim work between phases. Per Q3 default the loader-first phase is the unblocker.

### Phase 1 — Loader dispatch (the unblocker)

The smallest change that lets gt-design accept `schema_version: 3` recipes.

**Step 1.1.** Pre-edit: `ofpf-inspect crates/gtd-ssot/src/resolve/fnc_resolve_recipes.rs`. Confirm fan-in (one orchestrator caller) and that no other code depends on the V2-specific error message format.

**Step 1.2.** Write the failing test first. Extend `crates/gtd-ssot/tests/test_recipes_schema_compat.rs`:

- Add `test_recipes_schema_compat_accepts_v3_recipe` — a minimal v3 recipe payload (use the `bsod_crash_v3.json`-shape from `/usr/projects/tui-vfx-recipes/recipes/bsod_crash_v3.json` as reference) parses without `RecipeCompatibility` error.
- Add `test_recipes_schema_compat_v3_with_invalid_pipeline_step_kind` — the dispatch surfaces a meaningful error message.
- Add `test_recipes_schema_compat_keeps_payload_value_unchanged` — confirms Q2 default (`named` payload preserved as the original `Value`).

Run `cd /usr/projects/gt-design && cargo test -p gtd-ssot --test test_recipes_schema_compat` — fails (the v3 recipe is rejected by V2's `from_value`).

**Step 1.3.** Edit `crates/gtd-ssot/src/resolve/fnc_resolve_recipes.rs`. Replace the `from_value` call with `from_value_recipe_document` per Q1 default:

```rust
use tui_vfx_recipes::recipe::{from_value_recipe_document, RecipeLoadMode};

// At the existing dispatch line (currently fnc_resolve_recipes.rs:55):
match from_value_recipe_document(payload.clone(), RecipeLoadMode::Parsed) {
    Ok(_loaded) => {
        // Pre-validation succeeded. Per Q2 default, the original
        // payload Value is what flows downstream — the parsed form
        // is discarded here (consumers re-dispatch on schema_version
        // when they need typed access).
    }
    Err(err) => {
        return Err(GtdSsotError::RecipeCompatibility {
            key: key.clone(),
            message: err.to_string(),
        });
    }
}
named.insert(key.clone(), payload);
```

Bump `<VERS>` on `fnc_resolve_recipes.rs` to 0.4.0. CLOG entry: `0.4.0: Replace V2-only from_value compatibility check with version-aware from_value_recipe_document. schema_version: 3 recipes now parse without RecipeCompatibility rejection. GtdResolvedRecipes payload shape preserved per Intention 17.3.`

**Step 1.4.** Audit the `GtdSsotError::RecipeCompatibility` variant. The error message format may need to grow a `schema_version` field if the consumer (theme builder) displays it. `ofpf-refs RecipeCompatibility` to find every render site.

**Step 1.5.** Run `cargo test -p gtd-ssot` — green. Run `cargo build --workspace` — confirm no consumer broke.

**Step 1.6.** Commit interim: `Wire version-aware recipe dispatch in gt-design loader (gt-design v3 phase 1)`.

### Phase 2 — Downstream consumer audit + fixes

The loader now accepts v3, but the typed-bridge module at `crates/gtd-ratatui/src/recipes/resolve.rs` still imports V2 `Ra*` types. Phase 2 finds and fixes every V2-shape assumption in the downstream payload consumers.

**Step 2.1.** Pre-edit: `ofpf-inspect crates/gtd-ratatui/src/recipes/resolve.rs`. Confirm the four `RaContent*` import dependencies and the `serde_json::from_value::<RaRecipeConfig>` call site(s).

**Step 2.2.** Write a failing integration test: load a v3 recipe through the full path (loader → factory → ratatui playback) and verify a known-correct output. Use a content-scope V3 recipe so the path doesn't collide with the §8.6 producer bug. Suggested test file: `crates/gtd-ratatui/tests/test_v3_recipe_playback_smoke.rs`.

**Step 2.3.** For each V2-shape consumer identified in §C, decide:

| Consumer | Migration |
|---|---|
| `crates/gtd-ratatui/src/recipes/resolve.rs` | Rewrite to dispatch on schema_version. v1 path keeps the existing `RaRecipeConfig` deserialization. v3 path calls `from_value_recipe_document(value, RecipeLoadMode::Normalized)` and reads from `NormalizedRecipe`. The `ResolvedRecipeContent` struct stays unchanged — both paths populate it. |
| `crates/gtd-factory/src/vfx/fnc_pipeline_to_composition.rs` | Audit. If it walks the V2 pipeline JSON, mirror the dispatch from resolve.rs. If the V3 path uses the canonical playback-item builder upstream (per Intention 3), this consumer becomes a thin adapter. |
| `crates/gtd-ratatui/src/splash/*` | Audit. Splash recipes are likely already V2-only; gate on schema_version with a clear error message until splash gains V3 support. |

For each consumer touched, write a paired V3 test in the same wave.

**Step 2.4.** Run the workspace tests. The v3 smoke test from 2.2 must pass. All existing v1 tests must still pass.

**Step 2.5.** Commit interim: `Wire v3-aware downstream consumers in gt-design (gt-design v3 phase 2)`.

### Phase 3 — Vendored recipe migration

Migrate the 260 recipes from schema_v1 to schema_v3 in waves grouped by feature area. Per Q3 default, v1 and v3 coexist during the migration window.

**Step 3.1.** Pick a wave (recommended starting wave: `focused_row` family — only a handful of recipes, includes the failing test's recipe).

**Step 3.2.** For each recipe in the wave:

- Translate the V2 shape to V3 per `docs/design/tui-vfx-v3-upgrade-plan/57_v2_to_v3_lowering_rules.md`.
- The V3 schema is structurally different: `pipeline.style.spatial_shader` (V2) becomes `pipeline.step` with `kind: sampler/style_effect/filter/...` (V3). Bindings move to `requires_bindings` at the recipe root.
- Validate: `cd /usr/projects/tui-vfx-recipes && cargo run -q -p pipeline-validator -- --rules --strict-contracts <new-recipe.json>`.
- Render-truth check: `recipe-probe --phase dwelling --sample-t 1.0 <new-recipe.json>` produces non-zero `modified_cells` (gated on handoff §8.7 fix landing first; otherwise the probe lies).

**Step 3.3.** For each migrated wave, update any test that pinned to the V1 shape of the recipe.

**Step 3.4.** Commit each wave separately: `Migrate <wave-name> recipes to V3 schema (gt-design v3 phase 3.<n>)`.

**Step 3.5.** When all 260 recipes are migrated, audit for v1 path leftover. The loader can stay version-aware (Q3 default) so future v1 imports keep working; or the v1 path can be retired in a follow-up packet. Don't retire it in this packet — that's a separate decision after the migration proves out.

### Phase 4 — Producer-side fix (per Q4 default: bundled)

The `ContentShell::card` producer tags every cell `Surface`. Inner content is indistinguishable from border. This is a separate bug (independent of schema version) but bundles cleanly per handoff §8.6.

**Step 4.1.** Write a failing test: render a card via `ContentShell::card`, capture `RoleMapMaterialized.histogram`, assert the histogram contains `RoleTag::Text` cells (not just `RoleTag::Background`).

**Step 4.2.** Edit `crates/gtd-ratatui/src/widgets/util/cls_content_shell.rs` and the integration call site at `crates/gtd-ratatui/src/integration/fnc_tag_interaction_scoped_semantics.rs:111`. The fix:

- Inner content cells: tag `SemanticRole::Content` (lowers to `RoleTag::Text`).
- Border cells: tag `SemanticRole::Border`.
- Surface cells (the rest): keep `SemanticRole::Surface`.

The `tag_scoped_semantics` helper currently passes one `SemanticCell` for the whole area. The fix is to pass per-region cells (or a callback that returns the right cell per coordinate). ~50 LOC.

**Step 4.3.** Confirm `assert_selected_row_differs_from_neighbors` in `test_content_shell.rs:336` now passes (the producer fix alone is sufficient — the V2 recipe targeting `Role(Text)` finds Text-tagged cells where the producer used to tag everything Surface).

**Step 4.4.** Commit interim: `Fix ContentShell::card role tagging — distinguish content from surface (gt-design v3 phase 4 / handoff §8.6)`.

### Phase 5 — Workspace verify, clippy, doc gen

**Step 5.1.** Run the full §Verification commands block.

**Step 5.2.** Update `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` to mark items 1, 7, and 8 done.

**Step 5.3.** If any newly migrated V3 recipe fields appear in `docs/templates/capabilities.toml`, run `cargo xtask docs generate` and commit the regenerated `docs/CAPABILITIES_REFERENCE.md`.

**Step 5.4.** Final commit: `Phase 5: workspace clean (gt-design v3 migration complete)`.

## Code snippets

### Loader dispatch (Phase 1)

In `crates/gtd-ssot/src/resolve/fnc_resolve_recipes.rs`:

```rust
use tui_vfx_recipes::recipe::{from_value_recipe_document, RecipeLoadMode};

// Replace the existing fnc_resolve_recipes.rs:55 dispatch with:
if let Err(err) = from_value_recipe_document(payload.clone(), RecipeLoadMode::Parsed) {
    return Err(GtdSsotError::RecipeCompatibility {
        key: key.clone(),
        message: err.to_string(),
    });
}
named.insert(key.clone(), payload);
```

`RecipeLoadMode::Parsed` is the cheapest dispatch — it only validates the document shape and returns the parsed authoring form. Phase 2 may switch to `RecipeLoadMode::Normalized` if the downstream typed-bridge needs the normalized IR; the swap is a one-line change.

### Downstream consumer dispatch (Phase 2)

In `crates/gtd-ratatui/src/recipes/resolve.rs` (sketch — exact shape depends on the consumer):

```rust
use tui_vfx_recipes::recipe::{from_value_recipe_document, LoadedRecipeDocument, RecipeLoadMode};

pub(crate) fn resolve_recipe_content(
    payload: &serde_json::Value,
    tokens: &impl RecipeTokenResolver,
    assets: &impl RecipeAssetResolver,
) -> ResolvedRecipeContent {
    match from_value_recipe_document(payload.clone(), RecipeLoadMode::Normalized) {
        Ok(LoadedRecipeDocument::LegacyRecipes(recipes)) => {
            let recipe = recipes.into_iter().next().expect("non-empty");
            // Existing v1 path: deserialize to RaRecipeConfig and walk it.
            resolve_recipe_content_v1(recipe, tokens, assets)
        }
        Ok(LoadedRecipeDocument::V3Normalized(normalized)) => {
            // New v3 path: walk NormalizedRecipe directly.
            resolve_recipe_content_v3(normalized, tokens, assets)
        }
        Ok(_) => unreachable!("RecipeLoadMode::Normalized only returns Legacy or V3Normalized"),
        Err(err) => {
            // The loader pre-validated this payload; an error here is a bug.
            tracing::error!("recipe payload re-dispatch failed: {err}");
            ResolvedRecipeContent::default()
        }
    }
}
```

The `unreachable!` per `feedback_no_landmines` is acceptable here only if `from_value_recipe_document`'s contract documents the LoadMode → Variant mapping. Confirm before relying on it; otherwise the match becomes a soft-fall to `default()` with a `warn!`.

### Producer fix shape (Phase 4)

In `crates/gtd-ratatui/src/widgets/util/cls_content_shell.rs` (sketch):

```rust
// Inside ContentShell's StatefulWidget render or its tag_scoped_semantics call site:

// Compute the inner content rect (area minus border).
let border_thickness = self.border_thickness();
let inner = inset_rect(area, border_thickness);

// Tag the border ring as Border.
for (x, y) in border_cells(area, inner) {
    semantic_buffer.tag_cell(
        x, y,
        SemanticCell::new(SemanticRole::Border, /* ... */),
    )?;
}

// Tag the inner content area as Content.
for y in inner.y..inner.y + inner.height {
    for x in inner.x..inner.x + inner.width {
        semantic_buffer.tag_cell(
            x, y,
            SemanticCell::new(SemanticRole::Content, /* ... */),
        )?;
    }
}
```

The exact shape depends on whether the existing `tag_area` helper can take a per-region callback; if not, this is a new helper plus two calls.

## Test plan

### Existing tests that must keep passing unchanged

- `cargo test -p gtd-ssot` — every recipe-loader test stays green. The loader becomes more permissive (accepts v3), not less.
- `cargo test -p gtd-factory` — every factory test stays green. The pipeline bridge is unchanged for v1 recipes.
- `cargo test -p gtd-ratatui` — every existing v1 ratatui test stays green. The ContentShell producer fix in Phase 4 changes role-tagging, which **may** flip some existing tests that relied on all-Surface role maps; audit and update those tests in the same commit (don't suppress).

### New tests (per phase)

- **Phase 1.** Add to `crates/gtd-ssot/tests/test_recipes_schema_compat.rs`:
  - `test_recipes_schema_compat_accepts_v3_recipe` — v3 payload parses.
  - `test_recipes_schema_compat_keeps_payload_value_unchanged` — Q2 default preserved.
- **Phase 2.** New file `crates/gtd-ratatui/tests/test_v3_recipe_playback_smoke.rs`:
  - End-to-end: load v3 recipe, render, verify expected output.
- **Phase 3.** Per migrated wave: a `test_recipe_<name>_v3_parity` that loads both v1 (pre-migration) and v3 (post-migration) versions and asserts the rendered output matches.
- **Phase 4.** New file `crates/gtd-ratatui/tests/test_content_shell_role_tagging.rs`:
  - `test_content_shell_card_tags_inner_content_as_text` — `RoleMapMaterialized.histogram` contains `RoleTag::Text` cells.
  - `test_content_shell_card_tags_border_as_border` — `RoleMapMaterialized.histogram` contains `RoleTag::Border` cells.

### The auto-closing test

`cargo test -p gtd-ratatui --test test_content_shell test_content_shell_pipeline_override_without_animation_hints_uses_dwelling_effects` — fails today. Auto-closes when **either** Phase 3 migrates `themes/eichler/recipes/focused_row_btop.json` to V3 content-scope **or** Phase 4 lands the producer fix. Per Q4 default both ship in this packet, so the test passes after Phase 4.

The companion test `test_content_shell_stateful_pipeline_override_without_animation_hints_uses_dwelling_effects` auto-closes by the same mechanism.

### Observability bus assertion (gated on parallel session)

If the observability bus has shipped by the time this packet executes, add a `RoleMapMaterialized` observer assertion in the producer-fix test: confirm the bus emits a materialized role map containing both `RoleTag::Text` and `RoleTag::Border` for a card render. This catches future producer-side regressions without touching gt-design test code.

### TDD red→green order

1. Phase 1 red: extend `test_recipes_schema_compat.rs` with the v3 acceptance test → fails.
2. Phase 1 green: replace the loader dispatch line → passes.
3. Phase 2 red: write `test_v3_recipe_playback_smoke.rs` → fails (downstream still V2-shaped).
4. Phase 2 green: rewrite `recipes/resolve.rs` with version-aware dispatch → passes.
5. Phase 3 red→green: per-wave; the wave's migrated recipes parse + render correctly.
6. Phase 4 red: write `test_content_shell_role_tagging.rs` → fails (producer tags Surface).
7. Phase 4 green: fix ContentShell + tag_scoped_semantics → passes. The existing `test_content_shell` failures auto-close.

## Acceptance criteria

- [ ] `crates/gtd-ssot/src/resolve/fnc_resolve_recipes.rs` calls `from_value_recipe_document(payload, RecipeLoadMode::Parsed)` (or the leader-chosen alternative per Q1).
- [ ] `crates/gtd-ssot/tests/test_recipes_schema_compat.rs` has at least one passing test that validates a `schema_version: 3` recipe.
- [ ] The `GtdResolvedRecipes::named` payload remains `BTreeMap<String, serde_json::Value>` (Q2 default A); if the leader chose B or C, the SSOT contract docstring at `crates/gtd-ssot/src/types/GtdResolvedRecipes.rs:28-48` is updated to reflect the change.
- [ ] `crates/gtd-ratatui/src/recipes/resolve.rs` dispatches on schema_version. v1 path preserved; v3 path consumes `NormalizedRecipe`.
- [ ] At least one v3 vendored recipe loads end-to-end and renders the expected output (the wave-1 demonstration).
- [ ] `themes/eichler/recipes/focused_row_btop.json` is migrated to V3 schema (text in §Companion below).
- [ ] If Q4 default A is taken: `ContentShell::card` tags inner content cells `SemanticRole::Content` and border cells `SemanticRole::Border`. The histogram regression tests pass.
- [ ] Both failing tests in `crates/gtd-ratatui/tests/test_content_shell.rs` (the dwelling-effects pair) pass.
- [ ] All existing gt-design tests pass.
- [ ] `cargo build --workspace` succeeds with zero new warnings (per `feedback_clean_build_no_warnings`).
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean (per `feedback_clean_build_no_warnings`).
- [ ] No `#[allow]` suppressions added (per `feedback_no_landmines`).
- [ ] No parse-and-inert schema fields (per `feedback_no_inert_schema`) — every V3 field the migration touches is fully wired downstream in the same phase.
- [ ] Every `requires_bindings` entry in migrated recipes yields an effective loopback (per `feedback_loopback_required`); strict-contracts validator passes.
- [ ] Rustdoc audited and improved on every public item touched (per `feedback_rustdoc_when_editing`). The new dispatch site, the GtdSsotError variant message, and the rewritten `recipes/resolve.rs` carry rustdoc that names the V3 schema-version awareness.
- [ ] `cargo doc --no-deps -p gtd-ssot -p gtd-factory -p gtd-ratatui` succeeds with no broken intra-doc links.
- [ ] If any consolidated type or migrated recipe appears in `docs/templates/capabilities.toml`, `cargo xtask docs generate` regenerates the manifest and the regen is committed.
- [ ] **Q1, Q2, Q4 decisions recorded** in the loader file's CLOG. The packet's recommended defaults are evidence-based; if the leader picked alternatives, the rationale lives in the commit message and the file CLOG.

## Verification commands

```bash
# Daemon health on all three repos.
ofpf-status
ofpf-load --root /usr/projects/gt-design

# Per-crate tests.
cd /usr/projects/gt-design
cargo test -p gtd-ssot
cargo test -p gtd-factory
cargo test -p gtd-ratatui
cargo test -p gtd-theme-builder
cargo test --workspace

# The auto-closing tests.
cargo test -p gtd-ratatui --test test_content_shell \
  test_content_shell_pipeline_override_without_animation_hints_uses_dwelling_effects \
  test_content_shell_stateful_pipeline_override_without_animation_hints_uses_dwelling_effects

# Clippy with denied warnings.
cargo clippy --workspace --all-targets -- -D warnings

# Rustdoc clean.
cargo doc --no-deps -p gtd-ssot -p gtd-factory -p gtd-ratatui

# Recipe shape validation against tui-vfx-recipes pipeline-validator.
cd /usr/projects/tui-vfx-recipes
for recipe in /usr/projects/gt-design/themes/*/recipes/*.json; do
  cargo run -q -p pipeline-validator -- --rules --strict-contracts -- "$recipe" || echo "FAIL: $recipe"
done

# Probe-path render check on the migrated focused_row_btop (gated on handoff §8.7 fix).
cd /usr/projects/tui-vfx-recipes && cargo run -q -p recipe-probe -- \
  /usr/projects/gt-design/themes/eichler/recipes/focused_row_btop.json \
  --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3 \
  --runtime-params-json '{"selected_row": 3}'

# Vendored recipe schema_version distribution after migration.
grep -h "schema_version" /usr/projects/gt-design/themes/*/recipes/*.json | sort | uniq -c
```

## Rollback plan

The packet is structured as five interim commits (one per phase). If any phase reveals a deal-breaker:

1. Stop. Do not continue to the next phase.
2. `git revert <phase-commit-hash>` to back out the most recent phase. Earlier phases stay landed (they are additive — Phase 1 just makes the loader more permissive, Phase 2 adds a downstream dispatch arm, etc.).
3. If the deal-breaker is in Phase 1 (the loader dispatch itself), `git revert` Phase 1 too. The loader returns to V2-only behavior. No tree state lost.
4. `cargo build --workspace && cargo test --workspace` to confirm the restored state compiles and tests green.
5. File a finding in this packet capturing what blocked the migration; surface to the user. Common blockers to anticipate:
   - The downstream typed-bridge (`recipes/resolve.rs`) cannot be rewritten cleanly without an upstream surface change to `tui-vfx-recipes` (e.g. the `NormalizedRecipe` doesn't expose a field the V2 path reads). Resolution: add the field to `tui-vfx-recipes` first; defer this packet until the upstream lands.
   - A vendored V3 recipe round-trips through the loader but the runtime crashes (suggests a V3 runtime gap, not a migration gap). Resolution: file a tui-vfx-recipes / tui-vfx finding; defer the affected wave.
   - The `ContentShell::card` producer fix breaks an existing test in a way that suggests the test was relying on the bug. Resolution: surface to the user — the fix is correct architecturally, but the cosmetic regression needs a designer call.

The recyclebin protocol from `~/.claude/CLAUDE.md` mandates moves over deletes for any retired source files. No source files are deleted by this packet; vendored JSON migration overwrites in place (the v1 JSONs are gone, replaced by v3 — git history preserves the originals).

## Risks & gotchas

- **The typed-bridge `Ra*` import is the V2 escape hatch.** `crates/gtd-ratatui/src/recipes/resolve.rs:9` imports `RaContentConfig`, `RaContentMode`, `RaRecipeConfig` from `tui_vfx_recipes::recipe_schema`. Once the V2 `Ra*` surface retires (V3 cutover Decision 4 — `Ra*` → `Vfx*` rename), this import breaks. The migration must replace the V2 typed access with V3 normalized access **before** the upstream rename ships, or the gt-design tree breaks. **Coordinate with the V3 cutover schedule.**

- **GtdResolvedRecipes contract is documented as load-bearing.** The Intention 17.3 carve-out in the docstring at `crates/gtd-ssot/src/types/GtdResolvedRecipes.rs:28-48` is explicit: "Adding new typed accessors or a second opaque exception is an SSOT contract change, not a workaround to be implemented downstream." Q2 default A respects this; Q2 options B and C violate it. If the leader picks B or C, **stop and ask** before editing the docstring.

- **The `from_value_recipe_document` dispatch's V3 path uses `RecipeLoadMode::Parsed` by default in this packet.** That's the cheapest mode — only validates the authoring shape. If a downstream consumer needs the normalized IR, the loader can switch to `RecipeLoadMode::Normalized` (one-line change) but must accept the larger validation cost at load time. Choose per measured need; don't pre-optimize.

- **Recipe migration is tedious but mechanical.** The V2→V3 lowering rules are documented in `docs/design/tui-vfx-v3-upgrade-plan/57_v2_to_v3_lowering_rules.md`. Per-recipe migration is not creative work; it's translation. A junior who has read the lowering rules can do 10–20 recipes per hour. The 260-recipe total is ~2 days of focused translation work, not the multi-day blocker the L-tier suggests.

- **The producer fix has cosmetic risk.** Changing role-tagging from "all Surface" to "inner Content + border Border" may flip recipes that targeted Background-tagged inner cells (because the bug let them work). Audit recipes that use `region: TextOnly` (V2) or `scope: { kind: "channel", value: "text" }` (V3) — these were probably the ones broken by the producer bug, so fixing the producer fixes them too. But recipes that exploit the bug — targeting "all the cells, including the inner content as Background" — will visually flip. **Run the gt-design demo viewer's full recipe gallery before the producer-fix commit lands** and capture diffs.

- **`runtime_params_json` schema_v1 silent drop (handoff §8.8) interacts here.** During the V3 migration, the migrated recipes will start honoring `runtime_params_json` correctly because the V3 path doesn't have the silent-drop bug. This is a positive surprise but may flip behavior for recipes that depended on the runtime param being ignored. Audit during Phase 3.

- **`probe-path fidelity regression` (handoff §8.7) blocks Phase 3 verification.** The `recipe-probe` tool is reporting `modified_cells: 0` for known-working recipes. **Land handoff §8.7 fix before Phase 3 starts** so per-wave migration verification has a working probe.

- **`mod.rs` re-export surface in `gtd-ssot/src/types/`.** `pub use gtd_resolved_recipes::{GtdResolvedRecipeRef, GtdResolvedRecipes};` at `mod.rs:91`. Q2 option B (changing the `named` payload to `LoadedRecipeDocument`) requires re-exporting `tui_vfx_recipes::recipe::LoadedRecipeDocument` from gt-design too — which means gt-design's public surface gains a tui-vfx-recipes type. Per Intention 8 the type is already wire-format-prefixed (`Vfx*` family); the re-export is acceptable but should be intentional. Q2 default A avoids this concern entirely.

- **Bindings preservation across `selected_row_binding`.** The V2 recipe `focused_row_btop.json` uses `"selected_row_binding": "selected_row"` plus a static `"selected_row": 4` fallback. The V3 form (in §Companion below) declares `requires_bindings.selected_row` with `default: 4` and references `{"binding": "selected_row"}` inside the shader payload. Verify the runtime injection still works — the param key is the same on both sides.

## Sequencing note

This packet is **gated** on:

- **1.2.A `VfxBindable<T>` consolidation landing** (parallel session). The typed-bridge module's V3 path will lean on the consolidated `VfxBindable<T>` family; doing the migration on three parallel hand-rolled types is more rework.
- **Pipeline observability bus landing** (parallel session). The bus's `RoleMapMaterialized` observer is the cleanest surface for the Phase 4 producer-fix regression test.
- **Three Model B follow-on moves landing** (handoff §8.3). The composition-model decisions affect the V3 vocabulary the migrated recipes use.
- **Probe-path fidelity regression fix landing** (handoff §8.7). Phase 3 per-wave verification leans on `recipe-probe` reporting honest `modified_cells` counts.
- **`runtime_params_json` schema_v1 silent-drop fix landing** (handoff §8.8). Optional but reduces investigation cost during Phase 3 when migrated recipes start behaving differently from their v1 counterparts.

This packet **closes**:

- Handoff item 1 (`focused_row_btop` recipe — selected row not visually distinguished). Auto-closes via Phase 4 (producer fix) and Phase 3 (V3 content-scope migration of the recipe).
- Handoff item 7 (gt-design V3 stack migration). The whole packet.
- Handoff item 8 (ContentShell::card producer fix). Phase 4 if Q4 default A is taken; otherwise ships separately.

This packet **does not** address:

- Handoff item 9 (probe-path fidelity regression). Land first; this packet depends on it.
- Handoff item 10 (`runtime_params_json` silent drop). Land first or in parallel.
- Handoff item 11 (legacy `region: TextOnly` lowering). Per handoff §8.9, that bug class is deleted by the V3 cutover; recipes that survive this packet's migration use V3 content-scope from the start, so the issue auto-resolves for migrated recipes.

## Companion: focused_row_btop V3 migration

This is the V3 schema text for `themes/eichler/recipes/focused_row_btop.json`. It parses through `pipeline-validator --rules --strict-contracts` (verified during the 2026-04-26 evening bug investigation). It is rejected today by gt-design's V2-only loader; once Phase 1 of this packet lands, this file replaces the v1 version in the same commit as the focused-row recipe wave.

```json
{
  "schema_version": 3,
  "id": "eichler.focused_row_btop",
  "title": "Eichler Focused Row (btop-inspired)",
  "description": "Vendored focused-row gradient recipe for GT-Design list-focus work, aligned with the btop foreground-brightness reference while supporting dynamic selected-row binding via V3 requires_bindings.",
  "version": "3.0.0",
  "last_updated": "2026-04-26",
  "metadata": {
    "aesthetic_tags": ["btop", "focused_row", "gradient", "list", "phosphor"],
    "mood": "focused",
    "related_themes": ["eichler"],
    "use_cases": ["list_focus", "row_selection_indicator", "btop_parity"],
    "maturity_era": "mature",
    "authoring_notes": "V3 migration of the V1 recipe at the same path. selected_row moves from a `selected_row_binding` shader field into the V3 requires_bindings root with a default of 4. The static fallback (selected_row: 4) is now the binding's default. Scope uses content/text rather than V1's region: TextOnly so the shader matches glyph cells regardless of the producer's role tagging — sidesteps the §8.6 ContentShell producer bug.",
    "last_reviewed": "2026-04-26"
  },
  "requires_bindings": {
    "selected_row": {
      "type": "u16",
      "description": "Row index (0-based, in widget-local coordinates) of the currently focused list row. Drives the gradient's center.",
      "default": 4
    }
  },
  "config": {
    "message": "  1. First item\n  2. Second item\n  3. Third item\n> 4. SELECTED <\n  5. Fifth item\n  6. Sixth item\n  7. Seventh item\n  8. Eighth item\n  9. Ninth item\n 10. Tenth item",
    "layout": {
      "width": 22,
      "height": 12,
      "anchor": "center"
    },
    "lifecycle": {
      "auto_dismiss_ms": 8000
    },
    "border": {
      "type": "rounded",
      "trim": "none"
    },
    "base_style": {
      "foreground": { "type": "rgb", "r": 180, "g": 180, "b": 180 },
      "background": { "type": "rgb", "r": 20, "g": 22, "b": 28 }
    },
    "pipeline": {
      "timing": {
        "enter_ms": 300,
        "exit_ms": 200,
        "enter_ease": "cubic_out",
        "exit_ease": "cubic_in"
      },
      "step": {
        "kind": "style_effect",
        "phase": "dwelling",
        "scope": { "kind": "content", "value": "text" },
        "payload": {
          "type": "focused_row_gradient",
          "selected_row": { "binding": "selected_row" },
          "falloff_distance": 16,
          "bright_color": { "r": 255, "g": 255, "b": 255 },
          "dim_color": { "r": 60, "g": 60, "b": 60 },
          "apply_to": "Foreground"
        }
      }
    }
  }
}
```

**Notes on the migration shape:**

- `requires_bindings.selected_row` declares the binding once at the recipe root with a `default: 4`. Per `feedback_loopback_required` and `cls_v3_binding_declaration.rs`, the `default` field lifts to an effective loopback for non-numeric kinds; for `u16` the loopback could also be authored with the `loopback` field (a signal). Since the focused-row use case wants a static fallback and not a signal animation, `default` is the right choice.
- `scope: { kind: "content", value: "text" }` is content-scope (glyph-based, role-independent). This sidesteps the §8.6 producer bug entirely for this recipe — even before Phase 4 lands, this V3 recipe will work correctly because content-scope doesn't care about the role map.
- `selected_row: { "binding": "selected_row" }` inside the payload references the declared binding. The runtime injection (via `GtdRuntimeParams` on `GtdWidgetPipelineHints`) populates this; if the host doesn't inject, the loopback default (4) takes over.
- The `pipeline.step` shape replaces V1's `pipeline.style.spatial_shader`. The `kind: "style_effect"` matches the engine type; the `phase: "dwelling"` matches the test's expected animation phase.
- `base_style` moves from `pipeline.style.base_style` (V1) to `config.base_style` (V3 root) — base style is no longer pipeline-scoped in V3.
- `timing` (V3) replaces V1's separate `enter` / `exit` blocks. Same numbers.
- Per the `bsod_crash_v3.json` reference shape, V3 RGB colors use `{ "type": "rgb", "r": _, "g": _, "b": _ }` for top-level `base_style` slots. The shader payload's color fields are still bare `{r,g,b}` records (engine-side schema, unchanged). Verify this asymmetry against the V3 schema reference; if both should be `{type: rgb, ...}`, update the payload colors too.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-gt-design-v3-stack-migration.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
