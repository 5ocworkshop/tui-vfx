<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md</FILE> - <DESC>Chapter 100 — tooling and CI migration: the release-blocking tooling work V3 forces. Enumerates ~36 V2-schema-touching components, estimates migration impact per component, sequences with Concerns B (migration workflow) and F (release gates), and provides the explicit tooling-slice release checklist.</DESC> -->
<!-- <VERS>VERSION: 1.5.0</VERS> -->
<!-- <WCTX>Extracted from the monolithic plan (v0.16.0) "Tooling and CI migration" section. The sub-agent tooling inventory that informs this chapter lives in the migration log (docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md).</WCTX> -->
<!-- <CLOG>1.5.0: record that the first scene-bearing direct V3 subset now also supports bridgeable layer-local scene pipelines through the shared scene-source builder.
1.4.0: record that the direct recipe-probe V3 bridge now also emits operational and motion analysis for timeline-capable runs within the supported subset.
1.3.0: record that recipe-probe now also accepts the supported compiled V3 bridge subset, producing direct ProbeSceneSpec evidence without forcing every V3 probe request through paired legacy fallback.
1.2.0: record that tui-vfx-trace now also accepts the supported compiled V3 bridge subset, emitting real pipeline TraceReports instead of forcing every V3 trace request through legacy fallback.
1.1.0: record that supported compiled V3 recipes now reach the deterministic compositor bridge in both the dedicated V3 demo and pipeline-validator output stage while broader V3 runtime/probe coverage remains outstanding.
1.0.0: initial extraction from the monolith.</CLOG> -->

# 100 — Tooling and CI migration — release-blocking work

V3 is a clean break at the schema and loader level; it is also a clean break at the surrounding tooling level. The tools and CI that keep the V2 schema honest — validators, preview/demo, doc generators, probe/trace, authoring guides, template-expansion machinery — all read or write V2-shape data and will either continue serving V2 alongside V3 during the cutover or cut over to V3 wholesale. This chapter names that work explicitly so it does not get missed, and flags which pieces are "repoint the deserializer" vs. which need real design effort.

An exhaustive tooling inventory was conducted as part of the debug-recipes migration exercise (full report in `docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`). The V2 schema specs themselves are archived at `docs/v2-spec-archive/`. The list below is the summary plus the plan implications.

## 10 — Inventory map

```
   V2 schema touchpoints (~36 components total)
   ══════════════════════════════════════════════

        trivial (4)      moderate (10)     substantial (22)
         │                  │                    │
         ▼                  ▼                    ▼

   ┌──────────┐      ┌────────────────┐    ┌────────────────────┐
   │ Justfile │      │ compositor     │    │ recipe_schema/     │
   │ CI stubs │      │ content/pool   │    │   config.rs        │
   │ deep_merge      │ geometry types │    │ style/models/      │
   │ template │      │ parser version │    │ pipeline-validator │
   │  path    │      │   dispatch     │    │ debug-recipes-qc   │
   └──────────┘      │ gen_json       │    │ recipe-probe       │
                     │ gen_api        │    │ tui-vfx-trace      │
                     │ validator rules│    │ fnc_preview_       │
                     │ render_preview │    │   from_config      │
                     │ schema tests   │    │ demo binary        │
                     │ motion_blur    │    │ xtask docs/        │
                     │   crate        │    │   gen_effect_      │
                     └────────────────┘    │   schemas          │
                                           │ gen_ai_context     │
                                           │ capabilities.toml  │
                                           │ RECIPE_AUTHORING_  │
                                           │   WORKFLOW.md      │
                                           │ SCHEMA_REFERENCE   │
                                           │ AUTHORING_GUIDE    │
                                           │ PROCEDURAL_SOURCES │
                                           │ PIPELINE_VALIDATOR │
                                           │   _LLM_GUIDE       │
                                           │ ai-context.md      │
                                           │ 18 schema tests    │
                                           │ 10 probe tests     │
                                           └────────────────────┘
```

## 20 — Inventory detail

### 10 — Rust type definitions + loaders (6 components)

1. `tui-vfx-recipes/src/recipe_schema/config.rs` — the wire-format ground truth (`RaRecipeConfig` and every nested type). V3 renames to `VfxRecipeConfig` per Decision 4; the type surface changes substantially because Decision 3 restructures the pipeline as a tree and Decision 1 replaces scattered scoping fields with the unified Scope primitive. **Substantial.**
2. `tui-vfx-style/src/models/` — 50+ shader type definitions. Pattern-as-separable-axis (Decision 2) requires a structural reorganization from "every shader is an enum variant" to "named compositions are Rust factories that produce `ColoredOverlay + Pattern` trees". **Substantial.**
3. `tui-vfx-compositor/src/types/` — `FilterSpec`, `MaskSpec`, `SamplerSpec`, `MaskCombineMode`. Type definitions mostly stable; recipe loading paths need the V3 path. **Moderate.**
4. `tui-vfx-content/src/pool/` — `ContentEffect`, `EffectPool`, `TextPool`, `ImagePool`. Type definitions stable. **Moderate.**
5. `tui-vfx-geometry/src/types/` — `EasingCurve`, `MotionSpec`, `TransitionSpec`. Motion and easing types stable; the `motion_path` gap (Open Q #22 + final audit) is where this crate may gain new variants. **Moderate, pending motion_path resolution.**
6. `tui-vfx-probe/src/` — probe introspection reads the live recipe config. Impacts the Decision F trace taxonomy. **Substantial.**

### 20 — Parser + template machinery (6 functions)

- `fnc_expand_variants.rs`, `fnc_resolve_recipe_template.rs`, `fnc_deep_merge_json.rs`, `fnc_resolve_template_path.rs`, `fnc_validate_template_refs.rs`, `parser.rs` (`json_recipe_dyn_*` entry points). The `extends` + `template + variants` machinery mostly carries forward. The parser entry point needs version dispatch during the cutover window, or wholesale cutover if V3 ships clean-break (Concern B). **Mostly moderate; parser.rs is substantial.**

### 30 — Doc generators (7 subcommands + templates)

- `xtask docs generate` and its sub-generators (`gen_effect_schemas`, `gen_json`, `gen_markdown`, `gen_ai_context`, `gen_api`) produce `docs/generated/{CAPABILITIES.md, capabilities.json, effect_schemas.json, ai-context.md, API.md}`. Every shader variant is enumerated in these artifacts.
- `gen_effect_schemas.rs` in particular enumerates V2's `SpatialShaderType` variants. V3's Pattern-as-axis reorganization requires a full rewrite of this generator: the output must describe primitives (`ColoredOverlay` + `Pattern` enum) and named compositions (Tier 1 Rust factories) as two distinct surfaces rather than a single flat enum.
- `docs/templates/capabilities.toml` is the editorial master paired with rustdoc. Authors will need to re-document V3 effects or update entries. Related to Intention 28 (documentation is a first-class automated engineering contract).
- `gen_ai_context.rs` — the AI orientation doc. V3 rewrites every recipe structure example. **Substantial across the board; `gen_effect_schemas` and `gen_ai_context` are the largest.**

### 40 — Validators + CI gates (8 tools)

- `pipeline-validator` — 6 validation stages (parse, profile, render, shader, output, debug-recipes-qc), 18 test files, custom rule language. Core parsing stage is V2-locked. **Substantial** and gates on Chapter 60's release criteria (Concern F). Chapter 50's critical-set infrastructure is built on top.
- `recipe-probe` — probe diagnostic reports for cell focus, motion, operational analysis. 10 test files exercising scene/continuous/clock features. **Substantial.**
- `tui-vfx-trace` — trace capture. Trace event taxonomy may change with V3's scene-layer work (Decision 5 implementation track). **Moderate-to-substantial.**
- `recipe_schema/validator/` (embedded validator rules) — scene/continuous rule enforcement. **Moderate.**
- `recipe_schema/tests/` (18 test files) — encode V2 semantics. **Substantial** — parallel V3 test suite required.
- `fnc_run_debug_recipes_qc.rs` — 400+ recipe QC pass. Depends on V2 deserialization. Gates Chapter 60's release criteria. **Substantial.**
- `Justfile` recipes (`just check`, gates). **Trivial** as task runners; CI will need new V3 gates.
- `.github/workflows/` — CI wiring. Trivial structurally; content updates follow the validator/gen work above.

### 50 — Preview / demo binaries (3 surfaces)

- `fnc_preview_from_config.rs` (~19KB) — the preview widget generator. Recipe-to-UI rendering. **Substantial.** Customer-facing.
- `cargo run --example demo` — the public demo. Loads `recipes/` and drives the UI. **Substantial.** Must accept both V2 and V3 during cutover, or cut over wholesale under Chapter 50's clean-break discipline.
- `fnc_render_preview_item.rs` (~17KB) — per-frame rendering layer. **Moderate.**

### 60 — Authoring guides (6 documents)

- `docs/RECIPE_AUTHORING_WORKFLOW.md`, `tui-vfx-recipes/docs/schema/SCHEMA_REFERENCE.md`, `tui-vfx-recipes/docs/scene/AUTHORING_GUIDE.md`, `tui-vfx-recipes/docs/scene/PROCEDURAL_SOURCES.md`, `docs/generated/ai-context.md` (generated), `docs/PIPELINE_VALIDATOR_LLM_GUIDE.md`. Every example recipe, every structural reference, every field description must be rewritten for the V3 tree schema + primitives-by-default model. **Substantial across the board.** Related to Intention 28.

**Impact tally:** ~36 components total. Rough split: 4 trivial, 10 moderate, 22 substantial.

## 30 — Plan implications

1. **"Minor changes" understates the work.** The sub-agent inventory surfaced that pipeline-validator + `fnc_run_debug_recipes_qc.rs` alone are a substantial review cycle (6 validation stages, custom rule language, implicit V2 semantics in rule design). Treating them as "repoint the deserializer" will miss the validation rules that encode V2 behavior. Similarly, `fnc_preview_from_config` + demo are customer-facing; every live recipe load depends on them. The tooling cutover is not a trivial follow-on to the schema cutover; it is a first-class release track.

2. **The tooling migration sequences with Chapter 50 (migration workflow) and Chapter 60 (release gates).** Chapter 60 defines release-gate criteria (shadow/offscreen/probe/trace compatibility for the critical set); those criteria are *evaluated by* the validator/probe/trace tooling. Chapter 50's critical-set carve-out is executed *by* that same tooling. The tools must be V3-ready before the Chapter 60 gate can even run. This creates a sequencing: (a) tooling V3 support lands first, (b) critical-set migration runs through the tools under Chapter 50, (c) Chapter 60's gates evaluate green.

3. **Doc generators are authorship infrastructure.** Intention 28 ("documentation is a first-class automated engineering contract") means `gen_effect_schemas`, `gen_ai_context`, and the SKILLS.md / ai-context.md flow are not afterthoughts — they are part of the release surface. V3 does not ship without its doc pipeline producing V3-shape artifacts.

4. **The `extends` + `template+variants` machinery is mostly free.** Deep-merge, template path resolution, circular-ref detection are schema-agnostic. They carry forward with minor adjustments. The Stage 5 wargames migration (66 files) confirmed this — V3 `extends` worked unchanged.

5. **Dual-load vs clean-cutover choice is the tooling lever for Chapter 50.** The plan's clean-break framing (see `30_why_now.md`) suggests the tools also cut over wholesale. The alternative — parser dispatch on `schema_version` to load both — is cheap and preserves Chapter 50's mainline-corpus track. The two are compatible; the parser dispatches, the critical set uses the mechanical translator, the mainline corpus is re-authored. Choosing dispatch keeps the preview/demo able to show both while the mainline transitions. Default lean: **dispatch-on-schema_version in the parser; every other tool cuts over to V3 wholesale once its V3 support lands.**

6. **Temporary mixed-schema shims must be retired explicitly at endgame.** During the cutover, some helper seams may project V3 documents back into legacy compatibility types to keep old validator/test/example surfaces working while deeper runtime ports are still in flight. That is acceptable only as a transitional aid. Before declaring the V3 cutover complete and removing V2 support, audit for these shims (for example mixed-schema config projection helpers) and delete them rather than letting them become permanent compatibility sediment.

## 35 — Current implemented cutover state (`tui-vfx-recipes`)

The tooling migration is no longer purely theoretical. The following cutover seams are already in place in `tui-vfx-recipes`:

- **Centralized version-aware dispatch**
  - path-based dispatch for legacy vs V3 recipe files
  - in-memory dispatch for raw JSON strings / `serde_json::Value`
- **Centralized legacy runtime seam**
  - one helper for “load one playable legacy recipe”
- **Explicit cutover bridge**
  - one helper that can bridge a current V3 recipe path through its paired `_DEPRECATED_` legacy fixture when a legacy runtime surface still needs to operate
- **Customer-facing preview/demo seam**
  - the public demo/player and diagnostic example players now route through an upstream cutover-aware preview helper instead of assembling transitional policy at the call sites
- **Reporting/tool surfaces**
  - `pipeline-validator`
  - `recipe-probe`
  - `tui-vfx-trace`
  - the debug-recipes QC path
  now all use explicit shared seams and, where necessary, surface that they are temporarily bridged through legacy runtime truth rather than pretending full V3 runtime support already exists

This is meaningful progress, but it is **not yet equivalent to native V3 runtime support**.

The current stance is:

- structural V3 loading / normalization / validation / compile seams exist
- transitional legacy-runtime bridges exist where required for active audit and migration workflows
- supported compiled V3 plans can already bridge into `CompositionSpec`, render deterministically through the compositor, and surface grouped shader families plus a stable render hash in:
  - the dedicated `v3_play_recipe` example
  - `pipeline-validator --stage output` for the supported bridge subset
  - that supported subset now explicitly includes rect / rect-exclude spatial scopes by expanding them into concrete cell selections against the compiled layout dimensions
  - that supported subset now also includes channel-scoped filters/shaders/style-effects where the payload can lower into an existing runtime `apply_to` contract
  - that supported subset now also includes content/glyph/boolean selector scopes when they can be evaluated against the compiled envelope `source_text`
  - the dedicated `v3_play_recipe` example can now also render a first scene-bearing subset by lowering simple compiled scene layers through the existing stock scene composer before applying any bridgeable root pipeline
  - that first scene-bearing subset now includes stock procedural sources resolved through the real procedural registry, not only card-backed layers
  - that first scene-bearing subset also includes simple text layers, and those card/procedural/text source families are now covered across the direct tool lanes
  - that first scene-bearing subset also supports a bridgeable root pipeline layered over the composed scene, not only scene-only recipes
  - that first scene-bearing subset now also supports bridgeable layer-local scene pipelines on those simple scene layers through the shared scene-source builder, not only scene-only layers plus a root pipeline
  - `pipeline-validator --stage output` now also exercises that same first scene-bearing subset
  - `pipeline-validator --dump --stage output --format json` now also emits one deterministic structured dump sample for that same supported bridge subset
  - `pipeline-validator --probe` now also accepts that same supported bridge subset directly, producing direct `ProbeSceneSpec` evidence instead of forcing paired legacy runtime fallback
  - `recipe-probe` now also accepts that same first scene-bearing subset through the direct `ProbeSceneSpec` bridge
- supported compiled V3 plans can also reach `tui-vfx-trace` for the same bridge subset, emitting real pipeline trace envelopes without pretending full V3 preview/scene parity has already landed
- `tui-vfx-trace` now also accepts that same first scene-bearing subset through the shared direct bridge path
- supported compiled V3 plans can also reach `recipe-probe` for that same subset, producing direct `ProbeSceneSpec` frame/timeline/diff evidence and operational/motion analysis where the direct probe outputs already support it instead of always deferring to paired legacy runtime fallback
  - stock procedural scene sources are now covered across those first scene-bearing direct tool paths, not only card-backed layers
- those bridges are explicit and temporary

That distinction matters for Chapter 60’s release gates: “current V3 corpus can be inspected and audited” is **not** the same claim as “the runtime path is fully V3-native.”

## 40 — Release checklist (tooling slice)

V3 does not ship until each of the following is green:

- [ ] `tui-vfx-recipes/src/recipe_schema/config.rs` exposes V3 types + V2 types behind a `schema_version` dispatch, or cuts over wholesale.
- [ ] `tui-vfx-style/src/models/` restructured per Decision 2 (primitives + Tier 1 factories).
- [ ] `pipeline-validator` understands V3 and passes Chapter 60's gate criteria for the critical set.  
      _Current state: supported compiled V3 recipes now progress through parse/profile/render/shader/output and exercise the deterministic compositor bridge; rules/probe/critical-set parity remain outstanding._
- [ ] `recipe-probe` and `tui-vfx-trace` produce V3-aware reports; trace event taxonomy finalized (Decision 5 implementation track).  
      _Current state: both `recipe-probe` and `tui-vfx-trace` now accept the supported compiled V3 bridge subset directly; `recipe-probe` also carries operational/motion analysis for that subset where available. Scene parity, lifecycle-analysis parity, and broader trace semantics remain outstanding._
- [ ] `fnc_run_debug_recipes_qc.rs` passes against the V3 debug-recipes corpus.
- [ ] `xtask docs generate` produces V3-shape `effect_schemas.json`, `capabilities.json`, `ai-context.md`, `CAPABILITIES.md`.
- [ ] `tui-vfx-recipes` provides a repo-local `docs-v3-generate` / `docs-v3-check` path for the new `src/v3` spine until the broader generator story is unified.
- [ ] `docs/RECIPE_AUTHORING_WORKFLOW.md`, `SCHEMA_REFERENCE.md`, `AUTHORING_GUIDE.md`, `PROCEDURAL_SOURCES.md`, `PIPELINE_VALIDATOR_LLM_GUIDE.md` rewritten for V3.
- [ ] `docs/templates/capabilities.toml` editorial entries match the V3 effect surface.
- [ ] `cargo run --example demo` loads V3 recipes from the migrated corpus; optional V2 dual-load behind a feature flag per Chapter 50's transition-window semantics.
- [ ] CI gates in `.github/workflows/` and `Justfile` updated to run V3 validation.

Each item is cross-referenced back to the sub-agent's inventory in the migration log for implementation-level detail, and to the frozen V2 spec archive (`docs/v2-spec-archive/`) for historical reference.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.5.0</VERS> -->
