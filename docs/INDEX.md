<!-- <FILE>docs/INDEX.md</FILE> - <DESC>Documentation table of contents</DESC> -->
<!-- <VERS>VERSION: 1.15.0</VERS> -->
<!-- <WCTX>Add the V3 per-cell motion plan and annotated schema draft to the root documentation index.</WCTX> -->
<!-- <CLOG>1.15.0: add V3 per-cell motion plan and annotated schema draft pointers.
1.14.0: add the root CAPABILITIES guide to the main docs navigation.
1.13.0: add the V3 timing and metadata decision pointer for Q21/Q23 closure.
1.12.0: add the V3 outstanding master list and first-class tooling hub pointers.
1.11.0: add PIPELINE_TRACE_LLM_GUIDE.md, add tui-vfx-trace to the recipe-authoring ownership split, and keep the existing inspection-foundation references intact.
1.10.0: add Unified inspection foundation subsection pointing at tui-vfx-debug::inspection and TRACE_EVENT_SCHEMA.md; update pipeline-probe + validator guide entries with cross-reference to the inspection surface.
1.9.0: add Foundation primitives subsection naming the role-tagging primitives shipped in tui-vfx-types 0.6.0 for the recipe scene composer.
1.8.0: add explicit recipe-authoring tool ownership split plus the debug-recipes QC surface.</CLOG> -->

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

Full schema reference: [TRACE_EVENT_SCHEMA.md](TRACE_EVENT_SCHEMA.md).

## V3 orientation

- [../CAPABILITIES.md](../CAPABILITIES.md) — Root V3 capability orientation for human and AI authors; distinguishes hand-maintained guidance from generated inventory.

## V3 planning and tooling

- [design/tui-vfx-v3-INDEX.md](design/tui-vfx-v3-INDEX.md) — Single landing page for V3 planning, schema, I/O, migration, tooling, and outstanding work.
- [design/tui-vfx-v3-naming-normalization-decisions.md](design/tui-vfx-v3-naming-normalization-decisions.md) — Accepted V3 naming normalization decisions for `Vfx*`, playback seams, timing, motion, and intent vocabulary.
- [design/tui-vfx-v3-phase-scoping-decision.md](design/tui-vfx-v3-phase-scoping-decision.md) — Accepted V3 phase-scoping rule for step phases, container propagation, and normalized `PhaseSet` behavior.
- [design/tui-vfx-v3-migration-outcome-policy.md](design/tui-vfx-v3-migration-outcome-policy.md) — Accepted provisional migration outcome policy for `equivalent`, `replacement`, and `retired` tracks without removing legacy recipes.
- [design/tui-vfx-v3-release-gate-policy.md](design/tui-vfx-v3-release-gate-policy.md) — Accepted V3 release-gate manifest, outcome, whitelist, ownership, and fixture recapture policy.
- [design/tui-vfx-v3-scope-composition-decision.md](design/tui-vfx-v3-scope-composition-decision.md) — Accepted V3 scope inheritance and composition combine defaults.
- [design/tui-vfx-v3-capability-governance-decision.md](design/tui-vfx-v3-capability-governance-decision.md) — Accepted V3 promotion ladder for primitives, variants, earned-name compositions, and factory-internal schema promotion.
- [design/tui-vfx-v3-timing-and-metadata-decision.md](design/tui-vfx-v3-timing-and-metadata-decision.md) — Accepted V3 distributed timing and optional recipe metadata policy.
- [design/tui-vfx-v3-per-cell-motion-plan.md](design/tui-vfx-v3-per-cell-motion-plan.md) — V3 source-cell remapping plan for root and scene-layer `cell_motion`, including schema homes, runtime order, tests, docs, and debug fixtures.
- [design/tui-vfx-v3-schema-draft.json](design/tui-vfx-v3-schema-draft.json) — Annotated draft V3 schema; strip `#` comment lines to get valid canonical example JSON.
- [design/tui-vfx-v3-outstanding-master-list.md](design/tui-vfx-v3-outstanding-master-list.md) — Master outstanding V3 work list, including the final-only V2 retirement gate.
- [tooling/INDEX.md](tooling/INDEX.md) — Tooling hub for V3 preview, probe/database/frame diff, resize adapter, and edge ingestion workflows.

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
- [generated/CAPABILITIES.md](generated/CAPABILITIES.md) — Auto-generated capabilities inventory
- [generated/ai-context.md](generated/ai-context.md) — Condensed AI context prompt
- [generated/capabilities.json](generated/capabilities.json) — Machine-readable effect inventory
- [generated/effect_schemas.json](generated/effect_schemas.json) — Full ConfigSchema per effect

<!-- <FILE>docs/INDEX.md</FILE> - <DESC>Documentation table of contents</DESC> -->
<!-- <VERS>END OF VERSION: 1.15.0</VERS> -->
