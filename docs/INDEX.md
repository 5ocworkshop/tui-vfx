<!-- <FILE>docs/INDEX.md</FILE> - <DESC>Documentation table of contents</DESC> -->
<!-- <VERS>VERSION: 1.38.0</VERS> -->
<!-- <WCTX>V3.1 docs indexing: add parallel migration briefing.</WCTX> -->
<!-- <CLOG>1.38.0: add reusable v3.1 parallel migration agent briefing.
1.37.0: add v3.1 rendering boundary and lowering rules.
1.36.0: add K2.5 styled primitive evidence and player tooling PRD references.
1.35.0: add v3.1 schema and related directory map.
1.34.0: note I0 lifecycle/time/trigger contract semantics and schema roots.
1.33.0: note H1 vocabulary and canonical recipe document contract semantics.
1.32.0: note H0 source and asset contract semantics.
1.31.0: note G4 node output and graph value bus semantics.
1.30.0: note G3 graph topology and channel-aware parallel merge.
1.29.0: note G2 canonical graph execution proof.
1.28.0: note G1 graph and node contract roots.
1.27.0: note F2 value source, parameter, signal, and binding contract roots.
1.26.0: note F1 typed value/input contract and schema roots.
1.25.0: note E1 effect descriptor contract and schema root.
1.24.0: note tui-vfx-contract ownership and schema path split.
1.23.0: add v3.1 contract/engine boundary reference.
1.22.0: add v3.1 template composition design reference.
1.21.0: refresh clean-room v3.1 entries for Phase D1 scene composition.
1.20.0: add clean-room v3.1 architecture/contract/checklist/new-kernel references.
1.19.0: add API_SIGNALS_REFERENCE.md (renamed from SIGNALS_REFERENCE.md) under Generated with a one-line note pointing recipe authors at the recipe-side reference in tui-vfx-recipes.</CLOG> -->

# Documentation Index

## Foundation primitives (in `tui-vfx-types`, since 0.6.0)

The role-tagging primitives that underpin the unified recipe scene
composer (see gt-design
`docs/superpowers/specs/2026-04-20-recipe-scene-composer-design.md`):

- **`SemanticScene`** — source surface (`OwnedGrid` + `RoleMap` +
  `SceneMetadata`) consumed identically by every per-cell pipeline stage
  (sampler / mask / shader / filter / shadow). Accessor parity with
  `ratatui::Buffer` (`area()`, `cell((x, y))`).
- **`RoleMap`** — dense per-cell `RoleTag` storage; bounds-checked
  `get` / `set`, row-major iteration, serde-round-trippable.
- **`RoleTag`** — 12 first-class roles (`Background`, `Text`, `Title`,
  `Caption`, `Border`, `Image`, `Icon`, `Indicator`, `Highlight`,
  `Shadow`, `Decoration`, `Procedural`) plus `Custom(InternedRoleName)`;
  `#[non_exhaustive]`; shorthand `from_shorthand("border")`-style
  parsing.
- **`RoleInterner` / `RoleId`** — compact numeric IDs; first-class 0–11,
  Custom starts at 12.
- **`LayerId` / `RecipeId`** — opaque interned newtypes consumed by
  trace selectors without forcing dependence on the recipe crate.
- **`InternedString`** — cheap-to-clone `Arc<str>` wrapper backing the
  opaque identifiers.

## Unified inspection foundation (in `tui-vfx-debug::inspection`, since 0.9.0)

Since v0.9.0 (Sub-plan A Phase A.4), `tui-vfx-debug` carries the
canonical inspection surface for the recipe scene composer (see
gt-design
`docs/superpowers/specs/2026-04-20-recipe-scene-composer-design.md`
§9). The logger module is unchanged; a new `inspection` module sits
alongside it.

- **`TraceEvent`** — canonical event taxonomy across lifecycle /
  resolution / composition / pipeline stages (18 variants,
  `#[non_exhaustive]`).
- **`TraceEnvelope`** — event plus `frame_no`, `t_ms`, optional
  `recipe_id`, and monotonic `seq_in_frame` counter for deterministic
  replay ordering.
- **`TraceSelector`** — predicate (`Cell`, `Rect`, `Role`, `Layer`,
  `Recipe`, `All`); opaque `LayerId` / `RecipeId` from
  `tui-vfx-types`.
- **`TraceFilter`** — selectors (OR) + `StageMask` (AND) + frame/time
  half-open ranges.
- **`StageMask`** — bitmask (`LIFECYCLE | RESOLUTION | COMPOSITION |
  PIPELINE`); emit-site short-circuit via `is_empty`.
- **`InspectionSink`** — object-safe `Send + Sync` hook every stage
  reports to.
- **`TraceSink`** — thread-safe, filter-aware, optionally bounded
  `InspectionSink` impl; `snapshot()` / `drain()` materialise a
  `TraceReport`.
- **`TraceReport`** — envelope list + per-stage summary + dropped
  counter; `to_ndjson(writer)` / `from_ndjson(reader)` round-trip.

`CompositorInspector` remains at
`crates/tui-vfx-compositor/src/traits/pipeline_inspector.rs` —
the compositor ships an additive `InspectionSinkBridge` in
`crates/tui-vfx-compositor/src/traits/cls_inspection_sink_bridge.rs`
that forwards compositor callbacks into any `InspectionSink` without
disturbing existing direct implementors (`ProbeInspector`,
`StageInspector`, `TraceInspector`).

### Pipeline observability Unit A — per-stage and scope evidence

Five new pipeline-stage variants extend `TraceEvent` so a
"modified zero cells" diagnosis no longer requires source archaeology
(the focused_row_btop case study, 2026-04-26):

- **`StageEntered { kind, step_id, name, scope_summary }`** — one per
  Sampler / Mask / Shader / Filter / Shadow stage application.
- **`StageFinished { kind, step_id, cells_modified, elapsed_ns }`** —
  matching post-application event.
- **`StageSkipped { kind, step_id, reason }`** — replaces the
  entered/finished pair when the stage skipped iteration. `reason` is a
  `PipelineSkipReason` tagged union; `ScopeMatchedZeroCells` carries the
  predicate string and the role histogram the predicate visited.
- **`ScopeEvaluated { step_id, matched, skipped, role_histogram }`** —
  one per stage application; `matched + skipped` equals area cell count.
- **`RoleMapMaterialized { source, histogram }`** — one per render at
  the moment the role map becomes available; `source` discriminator
  distinguishes `Inferred` / `ExplicitFromProducer { producer }` /
  `Injected` so two renders with different role-map sources can be
  diffed cleanly.

Helper types (`PipelineStageKind`, `PipelineSkipReason`, `RoleHistogram`,
`RoleMapSource`) live in `tui-vfx-debug::inspection` next to
`TraceEvent`. The `CompositorInspector` trait grows four matching
callbacks (`on_stage_entered` / `on_stage_finished` / `on_stage_skipped`
/ `on_scope_evaluated`) plus `on_role_map_materialized`, each with
default empty bodies so existing impls compile unchanged.

The `AssertingInspector` test sink (`tui-vfx-debug::inspection`) wraps
forbidden-event predicates and panics on first match with a clear
assertion message. The convenience constructor
`AssertingInspector::forbid_zero_cell_scope_matches()` is the canonical
guard for the focused_row_btop bug class — install it on a recipe that
*should* fire its shader to mechanically refuse any future
`ScopeMatchedZeroCells` skip.

Design spec: [design/tui-vfx-pipeline-observability.md](design/tui-vfx-pipeline-observability.md) (v0.2.0). Full schema reference: [TRACE_EVENT_SCHEMA.md](TRACE_EVENT_SCHEMA.md).

## V3 orientation

- [../CAPABILITIES.md](../CAPABILITIES.md) — Root V3 capability orientation for human and AI authors; distinguishes hand-maintained guidance from generated inventory.

## V3 planning and tooling

- [VOCABULARY.md](VOCABULARY.md) — Canonical v3.1 human vocabulary, legacy/non-canonical term mapping, naming rules, deferrals, and change policy.
- [v3.1-architecture-overview.md](v3.1-architecture-overview.md) — Clean-room v3.1 contract-first architecture overview, scene composition stack, schema/reference path, D3 boundary, E0 physical split, and E1 descriptor model, F1 typed input model, F2 declarative source/binding model, G1 canonical graph container, G2 graph execution proof, G3 topology/channel-aware merge semantics, G4 graph value-bus semantics, H0 source/asset/procedural source contracts, H1 canonical recipe document packaging, and I0 lifecycle/time/trigger contracts.
- [v3.1-contract-boundary.md](v3.1-contract-boundary.md) — D3/E0/E1/F1/F2/G1/G2/G3/G4/H0/H1/I0 classification of contract vocabulary, proof implementation, crate ownership, schema roots, descriptor/input/value-source/binding/graph/execution proof/topology/value-bus/source/asset/recipe/lifecycle model, and handoff guardrails.
- [v3.1-surface-contract.md](v3.1-surface-contract.md) — Clean-room Phase A/B/C/D0/D1/D3/E0/E1/F1/F2/G1/G2/G3/G4/H0/H1/I0 surface, sampling, pipeline, scene, descriptor/input/source/binding/graph/execution-proof, schema-reference, graph value-bus, source/asset/recipe document/lifecycle contracts, vocabulary, and contract/proof ownership.
- [v3.1-feature-contract-checklist.md](v3.1-feature-contract-checklist.md) — Checklist for future v3.1 contract-affecting feature work.
- [v3.1-template-composition.md](v3.1-template-composition.md) — Phase D2 design for compile-time template/mixin/preset/profile expansion into canonical v3.1 recipes.
- [new_kernel/INDEX.md](new_kernel/INDEX.md) — Clean-room kernel phase docs, architect responses, status memos, and schema artifacts.
- [new_kernel/V31_RENDERING_BOUNDARY_RULES.md](new_kernel/V31_RENDERING_BOUNDARY_RULES.md) — Formal v3.1 recipe-to-playback boundary, lowering ownership, compositor adapter decision rules, and ANSI flow diagrams.
- [new_kernel/V31_PARALLEL_MIGRATION_AGENT_BRIEFING.md](new_kernel/V31_PARALLEL_MIGRATION_AGENT_BRIEFING.md) — Reusable shared briefing for parallel V2 `_DEPRECATED_` to v3.1 debug recipe migration lanes.
- [new_kernel/K2_5_STYLED_PRIMITIVE_ADAPTER_EVIDENCE.md](new_kernel/K2_5_STYLED_PRIMITIVE_ADAPTER_EVIDENCE.md) — K2.5 styled primitive adapter evidence for `shader.borderSweep`, `shader.linearGradient`, `style.baseStyleOverride`, and `style.colorFade`.
- [new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md](new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md) — Clean-room player tooling/validation PRD that classifies legacy tooling as oracle inspiration only.
- [design/tui-vfx-v3-INDEX.md](design/tui-vfx-v3-INDEX.md) — Single landing page for V3 planning, schema, I/O, migration, tooling, and outstanding work.
- [design/tui-vfx-v3-naming-normalization-decisions.md](design/tui-vfx-v3-naming-normalization-decisions.md) — Accepted V3 naming normalization decisions for `Vfx*`, playback seams, timing, motion, and intent vocabulary.
- [design/tui-vfx-v3-phase-scoping-decision.md](design/tui-vfx-v3-phase-scoping-decision.md) — Accepted V3 phase-scoping rule for step phases, container propagation, and normalized `PhaseSet` behavior.
- [design/tui-vfx-v3-migration-outcome-policy.md](design/tui-vfx-v3-migration-outcome-policy.md) — Accepted provisional migration outcome policy for `equivalent`, `replacement`, and `retired` tracks without removing legacy recipes.
- [design/tui-vfx-v3-release-gate-policy.md](design/tui-vfx-v3-release-gate-policy.md) — Accepted V3 release-gate manifest, outcome, whitelist, ownership, and fixture recapture policy.
- [design/tui-vfx-v3-scope-composition-decision.md](design/tui-vfx-v3-scope-composition-decision.md) — Accepted V3 scope inheritance and composition combine defaults.
- [design/tui-vfx-v3-capability-governance-decision.md](design/tui-vfx-v3-capability-governance-decision.md) — Accepted V3 promotion ladder for primitives, variants, earned-name compositions, and factory-internal schema promotion.
- [design/tui-vfx-v3-timing-and-metadata-decision.md](design/tui-vfx-v3-timing-and-metadata-decision.md) — Accepted V3 distributed timing and optional recipe metadata policy.
- [design/tui-vfx-v3-per-cell-motion-plan.md](design/tui-vfx-v3-per-cell-motion-plan.md) — V3 source-cell remapping plan for root and scene-layer `cell_motion`, including schema homes, runtime order, tests, docs, and debug fixtures.
- [design/tui-vfx-terminal-water-shader-plan.md](design/tui-vfx-terminal-water-shader-plan.md) — Terminal water/ocean shader implementation plan covering ripples, rain, flow, wake/trail, glint, 256-braille derivation, recipes, tests, and future weather/wind mapping.
- [design/completed/tui-vfx-terminal-fire-shader-plan.md](design/completed/tui-vfx-terminal-fire-shader-plan.md) — Terminal fire/flame shader implementation plan covering emissive temperature/density fields, smoke, blue core, sparks, glyph-ramp derivations, recipes, tests, and shared scalar/noise helpers with water.
- [design/tui-vfx-v3-schema-draft.json](design/tui-vfx-v3-schema-draft.json) — Annotated draft V3 schema; strip `#` comment lines to get valid canonical example JSON.
- [design/tui-vfx-v3-outstanding-master-list.md](design/tui-vfx-v3-outstanding-master-list.md) — Master outstanding V3 work list, including the final-only V2 retirement gate.
- [tooling/INDEX.md](tooling/INDEX.md) — Tooling hub for V3 preview, probe/database/frame diff, resize adapter, and edge ingestion workflows.

## V3.1 schema and related directories

- [`../schemas/v3.1/contract/`](../schemas/v3.1/contract/) — Checked generated stable v3.1 contract JSON Schemas. This is the canonical schema directory for `tui-vfx-contract` roots such as recipe, scene, element, effect descriptors, descriptor packs/catalogs, values, bindings, graph/node/value-bus types, sources/assets, lifecycle, phase, triggers, predicates, scopes, writes, diagnostics, and outcomes.
- [`../schemas/v3.1/next/`](../schemas/v3.1/next/) — Checked generated proof-pipeline schemas owned by `tui-vfx-next`; currently sampler and pipeline artifacts.
- [`../crates/tui-vfx-contract/`](../crates/tui-vfx-contract/) — Rust source of truth for stable v3.1 contract types and Serde/Schemars wire shape.
- [`../crates/tui-vfx-next/`](../crates/tui-vfx-next/) — Clean-room proof crate used to pressure-test new schema/reference concepts before promotion.
- [`../crates/tui-vfx-contract-cli/`](../crates/tui-vfx-contract-cli/) — Contract-only validator CLI for canonical v3.1 recipe validation reports.
- [`../descriptors/v3.1/`](../descriptors/v3.1/) — Descriptor-pack artifacts for the v3.1 primitive catalog, including `packs/primitive.json`.
- [`new_kernel/`](new_kernel/) — Phase-by-phase clean-room kernel docs, architect responses, status memos, review/de-slop reports, and K-series player evidence.
- [`../../tui-vfx-recipes/recipes/v3.1/`](../../tui-vfx-recipes/recipes/v3.1/) — Sibling-repo canonical v3.1 migrated recipe fixtures when both repos are checked out under `/usr/projects`.

## Hand-Maintained
- Recipe-authoring tool ownership:
  - **Preview / demo browser** (`tui-vfx-recipes` preview surfaces) is the canonical **recipe player** for human visual sign-off.
  - **`tui-vfx-trace`** is the canonical **unified lifecycle/resolution/composition/pipeline trace** surface for recipe playback.
  - **`pipeline-validator`** is the canonical **recipe-authoring validation** surface (parse / rules / stages / upstream-native `--debug-recipes-qc`).
  - **`recipe-probe` / `pipeline-validator --probe`** are the canonical **structured recipe evidence** surfaces.
  - **`pipeline-probe`** is the canonical **direct engine-scene** probe and does not replace recipe-aware validation.
- [TERMINAL_MOTION_HEURISTICS.md](TERMINAL_MOTION_HEURISTICS.md) — Canonical terminal-specific motion, depth, and compositing heuristics for effect and recipe design
- [API_HAND.md](API_HAND.md) — Original hand-maintained API reference
- [CAPABILITIES_REFERENCE.md](CAPABILITIES_REFERENCE.md) — Hand-maintained capabilities reference (primitive inventory)
- [COMPOSED_CAPABILITIES.md](COMPOSED_CAPABILITIES.md) — Curated catalog of composed capabilities organised around the six composition axes (mask / sampler / filter / style / motion path / runtime bindings), with foundational A+B→X patterns, variant-rich option-set treasure, a hero/easter-egg showcase, and designer refinement opportunities — grounded in a full audit of the ~630-recipe `tui-vfx-recipes` corpus
- [HOWTO_SHADOWS.md](HOWTO_SHADOWS.md) — Shadow rendering guide and integration patterns
- [PIPELINE_VALIDATOR_LLM_GUIDE.md](PIPELINE_VALIDATOR_LLM_GUIDE.md) — How an LLM should use the `pipeline-validator` CLI (in the sibling `tui-vfx-recipes` repo) to inspect recipe rendering, diagnose shader bugs, verify per-cell output, and run upstream-native debug-recipes QC
- [PIPELINE_TRACE_LLM_GUIDE.md](PIPELINE_TRACE_LLM_GUIDE.md) — How an LLM or user should use `tui-vfx-trace` (in the sibling `tui-vfx-recipes` repo) to capture unified recipe traces as NDJSON or structured JSON summaries
- [PIPELINE_PROBE_LLM_GUIDE.md](PIPELINE_PROBE_LLM_GUIDE.md) — How an LLM or user should use the engine-side `pipeline-probe` CLI to inspect direct `ProbeSceneSpec` inputs as structured JSON/NDJSON
- [TRACE_EVENT_SCHEMA.md](TRACE_EVENT_SCHEMA.md) — Full `TraceEvent` / `TraceEnvelope` schema shipped in `tui-vfx-debug::inspection` (since 0.9.0); AI-consumption-ready reference
- [PIPELINE_PROBE_WISHLIST.md](PIPELINE_PROBE_WISHLIST.md) — Prioritized wishlist for finishing the dream AI-native recipe debug tool, including the current diminishing-returns assessment
- [RECIPE_AUTHORING_WORKFLOW.md](RECIPE_AUTHORING_WORKFLOW.md) — Canonical staged workflow for building complex recipes one effect at a time, validating each layer, then flattening to a single final file
- [RECIPE_VISUAL_QA.md](RECIPE_VISUAL_QA.md) — Canonical visual checklist for manually previewing and signing off complex probe-validation recipes
- [design/pipeline-probe-design.md](design/pipeline-probe-design.md) — The current phase-1 design and rollout plan for engine-owned AI-native observability
- [RESEARCH_DESIGN_EXCELLENCE.md](RESEARCH_DESIGN_EXCELLENCE.md) — Cross-domain research summary for subtle, premium terminal polish and the current recommendation shortlist
- [DESIGN_EXCELLENCE_USAGE_GUIDE.md](DESIGN_EXCELLENCE_USAGE_GUIDE.md) — Research-backed guidance on when and how often subtle effects should be used, including fatigue management and deterministic-vs-stochastic rules
- [Cursor primitive](CAPABILITIES_REFERENCE.md#cursor-primitive-since-040) — General-purpose cursor primitive with grow-in + wake trail; powers `TypewriterCursor` and standalone caret overlays

## Generated (via `cargo xtask docs`)
- [generated/API.md](generated/API.md) — Auto-generated API reference from code + TOML templates
- [generated/API_SIGNALS_REFERENCE.md](generated/API_SIGNALS_REFERENCE.md) — Engine / direct-API signal reference for `mixed_signals::*` primitives reached by direct construction in engine code (`tui-vfx-content`, `tui-vfx-style`, `gt-design` motion runtime). Recipe authors writing recipe JSON should use the recipe-side reference in `tui-vfx-recipes` instead.
- [generated/CAPABILITIES.md](generated/CAPABILITIES.md) — Auto-generated capabilities inventory
- [generated/ai-context.md](generated/ai-context.md) — Condensed AI context prompt
- [generated/capabilities.json](generated/capabilities.json) — Machine-readable effect inventory
- [generated/effect_schemas.json](generated/effect_schemas.json) — Full ConfigSchema per effect

<!-- <FILE>docs/INDEX.md</FILE> - <DESC>Documentation table of contents</DESC> -->
<!-- <VERS>END OF VERSION: 1.38.0</VERS> -->
