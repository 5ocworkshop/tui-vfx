<!-- <FILE>docs/v2-spec-archive/README.md</FILE> - <DESC>V2 schema spec archive README. Frozen snapshot of the V2 schema master + generated artifacts + authoring guides + Rust ground-truth types captured immediately before V3 implementation work begins, so V3 edits don't overwrite V2 historical reference.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Archive created 2026-04-21 as pre-V3-work insurance. V2 is frozen from today; any edits from V3 work onward go to the live paths, not this archive. Paired with the chapter-ized V3 plan at docs/design/tui-vfx-v3-upgrade-plan/ and the monolithic pre-chapter plan at docs/design/tui-vfx-v3-upgrade-plan.md (v0.16.0).</WCTX> -->
<!-- <CLOG>1.0.0: initial archive. Copies generated doc artifacts, editorial capabilities.toml, authoring guides from both tui-vfx and tui-vfx-recipes repos, and the Rust ground-truth recipe_schema module tree from tui-vfx-recipes.</CLOG> -->

# V2 Schema Spec Archive

**Frozen as of 2026-04-21** — captured immediately before V3 implementation work begins.

This directory is a read-only snapshot of the V2 schema's canonical specification surface. Its purpose is to preserve the V2 authoritative state so that V3 work (which will edit many of the live paths these files are copied from) does not accidentally overwrite V2 history. Reviewers and future maintainers can compare V2↔V3 by reading this archive against the live V3 paths.

**Do not edit anything in this directory.** If V2 behavior needs clarification, that clarification belongs as a note in a V3 planning document, not as an edit to the archive. The archive is inert.

## Contents

```
docs/v2-spec-archive/
├── README.md                        # this file
├── generated/                       # from tui-vfx/docs/generated/
│   ├── ai-context.md                #   LLM-facing V2 schema orientation
│   ├── API.md                       #   auto-generated V2 API surface
│   ├── capabilities.json            #   machine-readable V2 effect inventory
│   ├── CAPABILITIES.md              #   human-readable V2 capabilities
│   ├── effect_schemas.json          #   full ConfigSchema per V2 effect
│   ├── recipes_validation.json      #   V2 recipe validation artifacts
│   ├── recipes_validation.md
│   └── README.md
├── editorial/                       # from tui-vfx/docs/templates/
│   ├── api_docs.toml                #   API editorial master
│   └── capabilities.toml            #   capabilities editorial master (paired
│                                    #   with rustdoc at doc-generation time)
├── authoring-guides/                # from tui-vfx/docs/
│   ├── API_HAND.md                  #   hand-maintained V2 API reference
│   ├── CAPABILITIES_REFERENCE.md    #   V2 primitive-inventory reference
│   ├── COMPOSED_CAPABILITIES.md     #   V2 composed-capabilities catalog
│   ├── PIPELINE_PROBE_LLM_GUIDE.md  #   pipeline-probe CLI guide
│   ├── PIPELINE_TRACE_LLM_GUIDE.md  #   tui-vfx-trace CLI guide
│   ├── PIPELINE_VALIDATOR_LLM_GUIDE.md  # pipeline-validator guide
│   ├── RECIPE_AUTHORING_WORKFLOW.md #   V2 recipe authoring workflow
│   ├── RECIPE_VISUAL_QA.md          #   V2 visual QA checklist
│   └── TRACE_EVENT_SCHEMA.md        #   V2 trace event schema
├── schema-reference/                # from tui-vfx-recipes/docs/
│   ├── SCHEMA_REFERENCE.md          #   V2 RaRecipeConfig field reference
│   ├── AUTHORING_GUIDE.md           #   V2 scene-layer authoring
│   ├── PROCEDURAL_SOURCES.md        #   V2 procedural source catalog
│   ├── LIFECYCLE_POLICY.md          #   V2 lifecycle-policy reference
│   ├── RECIPE_PROBE_GUIDE.md
│   └── INDEX.md
└── recipes-rust-source/
    └── recipe_schema/               # from tui-vfx-recipes/src/recipe_schema/
                                     #   Full V2 wire-format type tree —
                                     #   config.rs, parser.rs, scene/,
                                     #   validator/, functions/, etc.
```

## What this archive does NOT include

- **V2 recipe JSON files.** The debug-recipes migration exercise renamed V2 recipes to `_DEPRECATED_<name>.json` in place (see `tui-vfx-recipes/recipes/debug_recipes/` and `tui-vfx-recipes/recipes/wargames/`). Those files stay where they are; no archive copy needed because the `_DEPRECATED_` prefix convention already preserves them alongside their V3 replacements.
- **The Rust source tree outside `recipe_schema`.** Crates like `tui-vfx-style/src/models/`, `tui-vfx-compositor/src/types/`, `tui-vfx-content/src/pool/`, `tui-vfx-geometry/src/types/`, and `tui-vfx-probe/` contain V2-shape types that will evolve into V3-shape types in place. If historical pinning of those trees is needed later, the git history holds the V2 state as of the last commit before V3 work begins.
- **The monolithic pre-chapter V3 plan.** That lives at `docs/design/tui-vfx-v3-upgrade-plan.md` (v0.16.0) and is preserved intact per separate decision until the chaptered form at `docs/design/tui-vfx-v3-upgrade-plan/` is reviewed.

## Companion documents

- **Chaptered V3 plan** — `docs/design/tui-vfx-v3-upgrade-plan/` (navigate via `00_INDEX.md`)
- **V2↔V3 coverage audit** — `docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md` final audit section
- **Draft V3 schema** — `docs/design/tui-vfx-v3-schema-draft.json`
- **Audit workflows (A/B/C)** — `docs/design/tui-vfx-v3-upgrade-audit-workflow.md`

## When to consult this archive

- **Verifying a V2 shape claim.** When a V3 document says "V2 supports X" or "V2 had field Y," this archive is the authoritative reference for whether that was actually true.
- **Auditing migration losses.** If a V3 migration produces a recipe that seems different from the V2 behavior a reviewer remembers, comparing against `schema-reference/SCHEMA_REFERENCE.md` or `recipes-rust-source/recipe_schema/config.rs` tells you whether the difference is an intended V3 improvement or a regression.
- **Checking generator fidelity.** The `generated/` snapshot lets reviewers confirm that V3's rewritten `gen_effect_schemas` produces the right shape by comparing against what V2's generator produced.
- **Onboarding new contributors.** The authoring guides in this archive are the V2 mental model; V3 guides (written during the tooling cutover per Chapter 100) should read as a coherent upgrade rather than a disconnect.

## Archive lifecycle

This archive stays in place through V3 implementation, V3 release, and the first stability period post-release. It may be deleted later if:

1. V3 has shipped and stabilized,
2. no open migration question refers back to a V2-only fact, and
3. git history provides sufficient historical access for any remaining needs.

Until all three conditions hold, the archive is load-bearing and stays read-only.

<!-- <FILE>docs/v2-spec-archive/README.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
