<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md</FILE> - <DESC>Navigation hub and pinned Schema V3.0 reference for the V3 upgrade plan chapter directory. Defines the reading order, summarizes each chapter, and lists the pinned schema reference the rest of the plan depends on. This file's <VERS> tracks its own revisions; individual chapter files version independently.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Initial creation of the chapter directory index. The monolithic tui-vfx-v3-upgrade-plan.md (v0.16.0) is preserved at its original location and will be deprecated later; this directory is the new canonical home going forward. All chapter content is extracted-copied from the monolith, not moved.</WCTX> -->
<!-- <CLOG>1.0.0: initial index. Establishes chapter layout (10/20/.../110 with 10-unit spacing for flexible insertion), pins Schema V3.0 reference in the body, cross-maps each chapter to its monolith source, provides the ANSI chapter-dependency diagram.</CLOG> -->

# TUI-VFX V3 Upgrade Plan

> **Status: draft — direction only.** No implementation schedule, no migration tooling spec, no committed V3 schema grammar yet. This plan captures *what we've decided* and *what still needs to resolve* before a real implementation plan can be written.

## Pinned schema reference

This plan describes **Schema V3.0** as its target. All chapters in this directory refer back to this pinned reference when they cite "V3" semantics. The canonical draft schema shape lives at `docs/design/tui-vfx-v3-schema-draft.json` (the specification-by-example document).

```
 ┌──────────────────────────────────────────────────────────────────┐
 │  Schema V3.0 pinned reference                                    │
 │                                                                  │
 │  Recipe envelope               Pipeline tree (Decision 3)        │
 │  ────────────────              ──────────────────────────        │
 │                                                                  │
 │  schema_version: 3             Pipeline ::= Step                 │
 │  id / title / desc                       | Parallel([Step...])   │
 │  extends?                                | Sequence([Step...])   │
 │  metadata                                                        │
 │  requires_* contracts          Step ::= { kind, scope, phase,    │
 │                                           payload }              │
 │  config:                                                         │
 │    message                     kind ::= Mask | Sampler           │
 │    layout                           | Filter | StyleEffect       │
 │    lifecycle                        | Shader                     │
 │    border                      phase ::= Enter | Dwell | Exit    │
 │    content? (root)                    | All                      │
 │    clock?                                                        │
 │    base_style                  Scope (Decision 1, closed enum)   │
 │    scene.layers? (Decision 5)  StepInput<T> = ParamValue<T>      │
 │    pipeline                      | HintRef<T>  (Decisions 6, 7)  │
 │      timing                                                      │
 │      step? (optional)                                            │
 └──────────────────────────────────────────────────────────────────┘
```

Individual chapters may deepen specific parts of this reference; none redefine it.

## How to read this plan

The plan is organized to make *why* we're making each decision as visible as *what* the decision is. Future readers (humans and AI alike) will not have access to the conversation that produced this plan; the rationale must travel with the text. Each Decision explains: what it is, why this shape was chosen over alternatives, how it composes with other V3 decisions, and how it's envisioned at the authoring/consumer surface. Each Open Question names both the question and what's at stake in choosing different answers.

Reading order is the file number order. Chapters 10–30 set up philosophy, architecture, and the case for V3 before you hit the substantive decision work in 40. Chapters 50–60 cover the *migration workflow* (how the work happens) and *testing/release gates* (how we know it's done) — these are release-blocking and intentionally precede the shape/open-question chapters so readers understand the cutover commitment before surveying unsettled detail. Chapters 70–90 are the substance you'll most often revisit. Chapter 100 enumerates the tooling/CI work the schema cutover forces. Chapter 110 is the audit workflow appendix.

## Chapter map

```
   10_philosophy              30_why_now
          │                        │
          └──────────┬─────────────┘
                     │
                     ▼
          20_architectural_framing
                     │
                     ▼
          ┌────────────────────────┐
          │  40_decisions  (1-8)   │
          └────────────────────────┘
               │           │
               ▼           ▼
    50_migration_     60_testing_
     workflow         release_gates
          │                │
          └───────┬────────┘
                  ▼
     ┌────────────────────────────┐
     │ 70_shape_sketches          │
     │ 80_open_questions  (1-21)  │
     │ 90_deferred_design         │
     └────────────────────────────┘
                  │
                  ▼
       100_tooling_ci_migration
                  │
                  ▼
        110_appendix_audits
```

### Chapter summaries

| # | File | Summary | Status |
|---|---|---|---|
| 00 | [00_INDEX.md](00_INDEX.md) | This file. Pinned schema reference + reading order + chapter map. | — |
| 10 | [10_philosophy.md](10_philosophy.md) | Five principles (Morris, pipe-culture chain-ability, widgets-and-the-grid, authoring-affordance preservation, meaning-low-policy-high) + the constraint-vs-permissiveness design discipline. The durable framings that outlast any single decision. | Stable |
| 20 | [20_architectural_framing.md](20_architectural_framing.md) | Layer model (L1→L5), ecosystem-agnostic seam (tui-vfx renders to grid; ratatui is *a* consumer, not *the* consumer), mixed-signals as upstream home for signal primitives, two-level chaining (signal-graph composition vs pipeline-step chaining). | Stable |
| 30 | [30_why_now.md](30_why_now.md) | The clean-break framing + the feedback cross-reference mapping external weak-seams concerns to specific V3 coverage + the concrete migration drivers that surfaced during recent work. | Stable |
| 40 | [40_decisions.md](40_decisions.md) | The eight structural decisions with adopted direction: (1) Unified Scope, (2) Pattern-as-separable-axis, (3) Tree authoring schema, (4) Ra→Vfx rename, (5) Scene layers carry pipelines, (6) Signal-driven parameters, (7) Step output hints, (8) Canonical upstream semantic seam. | Stable |
| 50 | [50_migration_workflow.md](50_migration_workflow.md) | Three-phase Curate→Re-author→Validate workflow for mainline corpus, plus critical-set fixture-equivalence carve-out. Sequencing with tooling rollout. Concern B's resolution made first-class. | Stable |
| 60 | [60_testing_release_gates.md](60_testing_release_gates.md) | Six release-gate criteria (shadow / offscreen / probe / trace / GT-Design integration / role-aware lowering), validator redesign scope, fixture/golden-artifact strategy, whitelist discipline, recapture cadence, escalation paths. Concern F's resolution + Open Q #9. | Stable |
| 70 | [70_shape_sketches.md](70_shape_sketches.md) | Flat-vs-tree JSON comparisons for three representative cases (simple fade-in toast, ember-felt three-layered dwell, ambient-halo four-per-edge). | Stable |
| 80 | [80_open_questions.md](80_open_questions.md) | 23 open questions with reviewer-opinion annotations. Ordered roughly by impact on plan shape. Open Q #22 (motion_path + offscreen trajectory migration) newly promoted from migration-log major gap to plan-level status. Open Q #23 (timer story — tachyonfx-style first-class Timer primitive vs distributed timing) added during competitive-analysis pass. | Evolving |
| 90 | [90_deferred_design.md](90_deferred_design.md) | Movie-composer territory, recipe migration workflow (non-blocking long-form), distribution and packaging story, recipe metadata fields, retrospective corrections, StaggeredLines PRD primitive, dynamic recipe formalization. | Stable |
| 100 | [100_tooling_ci_migration.md](100_tooling_ci_migration.md) | ~36 components touching V2 schema — 4 trivial / 10 moderate / 22 substantial. Tooling cutover is a first-class release track, not an implied follow-on. Release checklist for the tooling slice. | Stable |
| 110 | [110_appendix_audits.md](110_appendix_audits.md) | Three audit workflows (A shader catalog, B corpus curation, C structural translation sample). Referenced from Decisions 2, 7, and migration-workflow chapter. | Stable |

### Cross-reference conventions

- **Decision 1–8** — reference by number; their home is `40_decisions.md`.
- **Open Q #1–#21** — reference by number; their home is `80_open_questions.md`.
- **Principle 1–5** — reference by number; their home is `10_philosophy.md`.
- **Concern A–F** — references the 2026-04-21 GT-Design lead review memo; resolutions are woven into the decisions and open questions they most affect. Concerns A, B, C, D, E, F pair respectively with Decisions 6+7 (ParamValue/HintRef split), Open Q #2 / Chapter 50 (migration workflow), Open Q #18 (role-domain split), Open Q #14 (tokenization/bindings), Decision 5 (scene-layer implementation track), and Open Q #12 / Chapter 60 (release gates).

### Relation to the monolithic plan file

The monolithic `docs/design/tui-vfx-v3-upgrade-plan.md` (v0.16.0) is the source from which the chapters in this directory were extracted. **It is preserved intact** so reviewers can compare the chaptered form against the pre-chapter source. The monolith will be deprecated later via the `_DEPRECATED_` prefix convention once the chaptered form is reviewed and accepted.

### Companion documents

- `docs/design/tui-vfx-v3-upgrade-audit-workflow.md` — Workflows A / B / C referenced from `110_appendix_audits.md`
- `docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md` — The debug-recipes migration exercise that pressure-tested the draft schema. Contains the evolving schema-question journal (Q1–Q34), the running drift table (D1–D7), and the final V2↔V3 coverage audit.
- `docs/design/tui-vfx-v3-schema-draft.json` — Specification-by-example of the draft V3 recipe schema with inline `#` comments. Stripping comment lines yields valid JSON.
- `feedback/2026-04-21-gtd-tui-vfx-weak-seams-feedback.md` (in gt-design repo) — The weak-seams review cited in Chapter 30's feedback cross-reference.

### V2 schema — historical reference

**Frozen archive:** `docs/v2-spec-archive/` — a read-only snapshot of the V2 spec surface (generated doc artifacts, editorial masters, authoring guides from both tui-vfx and tui-vfx-recipes, the full `recipe_schema` Rust module tree) captured 2026-04-21 immediately before V3 implementation work begins. 53 files total. Do not edit. See `docs/v2-spec-archive/README.md` for archive organization and lifecycle.

The table below names the **live** V2 locations the archive was copied from — these paths continue to exist and V3 work will edit them in place. Use the archive for "what V2 was"; use the live paths for "what V3 becomes":

| Artifact | Location | What it is |
|---|---|---|
| Rust ground-truth types | `tui-vfx-recipes/src/recipe_schema/config.rs` | The `RaRecipeConfig` wire-format + nested types (`RaPipelineConfig`, `RaStylePipelineConfig`, `RaMaskConfig`, `RaFilterConfig`, `RaSamplerConfig`, `RaStyleEffect`, `RaBaseStyle`, `RaClock`, `RaContinuousConfig`, `RaSceneConfig`, `RaLifecycleConfig`, `RaContentConfig`, ...). Serde-deserializable from V2 recipe JSON. |
| Shader type catalog | `tui-vfx-style/src/models/` | 50+ `SpatialShaderType` enum variants + per-shader config structs. Decision 2's restructure source. |
| Parser + template machinery | `tui-vfx-recipes/src/recipe_schema/{parser.rs, functions/}` | `json_recipe_dyn` entry points, `fnc_expand_variants.rs`, `fnc_resolve_recipe_template.rs`, `fnc_deep_merge_json.rs`, `fnc_resolve_template_path.rs`, `fnc_validate_template_refs.rs`. |
| Compositor types | `tui-vfx-compositor/src/types/` | `FilterSpec`, `MaskSpec`, `SamplerSpec`, `MaskCombineMode`. |
| Content-effect pool | `tui-vfx-content/src/pool/` | `ContentEffect`, `EffectPool`, `TextPool`, `ImagePool` (typewriter, scramble, marquee, mirror, split_flap, glyph_cascade, etc.). |
| Geometry + motion types | `tui-vfx-geometry/src/types/` | `EasingCurve`, `MotionSpec`, `TransitionSpec`, `PlacementSpec`, `AnchorSpec`, and `PathType` (Intention 38 authority for motion-path variants: linear, arc, bezier, spring, bounce, projectile, pendulum, …). |
| Generated schema artifacts | `tui-vfx/docs/generated/{API.md, CAPABILITIES.md, capabilities.json, effect_schemas.json, ai-context.md}` | Produced by `cargo xtask docs generate`. The V2-shape fact artifacts the V3 doc-generator cutover must replace. |
| Editorial master | `tui-vfx/docs/templates/capabilities.toml` | Human-curated editorial entries merged with extracted rustdoc at doc-generation time. |
| Monolithic pre-chapter plan | `docs/design/tui-vfx-v3-upgrade-plan.md` (v0.16.0) | The source document this chapter directory was extracted from. Preserved intact; deprecation deferred until the chaptered form is reviewed. |

Chapter 100 (`100_tooling_ci_migration.md`) enumerates every one of these artifacts as a release-blocking migration target.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
