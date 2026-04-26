<!-- <FILE>docs/design/tui-vfx-v3-INDEX.md</FILE> - <DESC>Single landing page for V3 planning, schema, implementation, tooling, and migration docs.</DESC> -->
<!-- <VERS>VERSION: 0.6.2</VERS> -->
<!-- <WCTX>Keep the V3 landing page current with active authoring, schema, tooling, lifecycle, water shader planning, and deferred post-release capability docs.</WCTX> -->
<!-- <CLOG>0.6.2: add terminal water/ocean shader plan to active work surfaces.
0.6.1: add token-safe TTE-inspired remaining-work handoff. 0.6.0: add the Task 24 glyph particle emitter plan to active V3 work surfaces.
0.5.9: add the deterministic core-pipeline readiness matrix to the active V3 work surfaces and execution references.</CLOG> -->

# V3 documentation index

Start here when working on tui-vfx V3. This page points to the durable planning
chapters, as-built contracts, schema docs, tooling guides, and live punch list.

## Status legend

- **Canonical core docs** are live guidance: use them as the current contract or
  accepted decision record.
- **Active work surfaces** still drive implementation, validation, migration, or
  release-gate follow-up.
- **Retained history** preserves rationale or closed-out planning context. Read it
  for background, but prefer the canonical docs and active trackers for current
  instructions.
- **Post-release/deferred specs** are useful ideas that must not be treated as V3
  release blockers.

For the complete non-destructive classification map, see the
[V3 docs lifecycle/elevation plan](tui-vfx-v3-docs-lifecycle-plan.md).

## Start here

These are live entry points, not historical notes.

1. [V3 outstanding master punch list](tui-vfx-v3-outstanding-master-list.md)
2. [V3 execution DAG](tui-vfx-v3-execution-dag.md)
3. [V3 upgrade-plan chapter index](tui-vfx-v3-upgrade-plan/00_INDEX.md)
4. [V3 schema overview](tui-vfx-v3-schema-overview.md)
5. [V3 schema draft](tui-vfx-v3-schema-draft.json)
6. [V3 docs lifecycle/elevation plan](tui-vfx-v3-docs-lifecycle-plan.md)
7. [V3 I/O contract](tui-vfx-v3-io-contract.md)
8. [V3 naming normalization decisions](tui-vfx-v3-naming-normalization-decisions.md)
9. [V3 naming implementation inventory](tui-vfx-v3-naming-implementation-inventory.md)
10. [V3 phase-scoping decision](tui-vfx-v3-phase-scoping-decision.md)
11. [V3 migration outcome policy](tui-vfx-v3-migration-outcome-policy.md)
12. [V3 release-gate policy](tui-vfx-v3-release-gate-policy.md)
13. [V3 release-gate manifest seed](tui-vfx-v3-release-gate-manifest.md)
14. [V3 scope and composition decision](tui-vfx-v3-scope-composition-decision.md)
15. [V3 capability governance decision](tui-vfx-v3-capability-governance-decision.md)
16. [V3 timing and metadata decision](tui-vfx-v3-timing-and-metadata-decision.md)
17. [Tooling hub](../tooling/INDEX.md)

## Canonical core docs and accepted decisions

Use these as current V3 guidance.

- [V3 upgrade-plan chapter index](tui-vfx-v3-upgrade-plan/00_INDEX.md) — retained chapter hub with pinned Schema V3.0 reference; current work is tracked by this index, the lifecycle plan, and the master punch list.
- [V3 schema overview](tui-vfx-v3-schema-overview.md)
- [V3 schema draft](tui-vfx-v3-schema-draft.json)
- [V3 I/O contract](tui-vfx-v3-io-contract.md)
- [V3 recipe vocabulary](tui-vfx-v3-recipe-vocabulary.md)
- [V3 naming normalization decisions](tui-vfx-v3-naming-normalization-decisions.md)
- [V3 phase-scoping decision](tui-vfx-v3-phase-scoping-decision.md)
- [V3 migration outcome policy](tui-vfx-v3-migration-outcome-policy.md)
- [V3 release-gate policy](tui-vfx-v3-release-gate-policy.md)
- [V3 scope and composition decision](tui-vfx-v3-scope-composition-decision.md)
- [V3 capability governance decision](tui-vfx-v3-capability-governance-decision.md)
- [V3 timing and metadata decision](tui-vfx-v3-timing-and-metadata-decision.md)

## Active work surfaces

These still shape implementation, validation, migration, or release-gate follow-up.

- [V3 outstanding master punch list](tui-vfx-v3-outstanding-master-list.md)
- [V3 execution DAG](tui-vfx-v3-execution-dag.md)
- [V3 docs lifecycle/elevation plan](tui-vfx-v3-docs-lifecycle-plan.md)
- [V3 capability catalog](tui-vfx-v3-capability-catalog.md)
- [V3 lowering map](tui-vfx-v3-lowering-map.md)
- [V3 normalized IR](tui-vfx-v3-normalized-ir.md)
- [V3 compiled execution plan](tui-vfx-v3-compiled-execution-plan.md)
- [V3 scheduler batching plan](tui-vfx-v3-scheduler-batching-plan.md)
- [V3 validator canonicalization checklist](tui-vfx-v3-validator-canonicalization-checklist.md)
- [V3 cross-family coverage plan](tui-vfx-v3-cross-family-coverage-plan.md)
- [V3 core pipeline readiness matrix](tui-vfx-v3-core-pipeline-readiness-matrix.md)
- [V3 spatial field hint plan](tui-vfx-v3-spatial-field-hint-plan.md)
- [V3 scene content integration plan](tui-vfx-v3-scene-content-integration-plan.md)
- [V3 per-cell motion plan](tui-vfx-v3-per-cell-motion-plan.md)
- [V3 glyph particle emitter plan](tui-vfx-v3-glyph-particle-emitter-plan.md)
- [V3 TTE-inspired remaining-work plan](tui-vfx-v3-tte-inspired-remaining-work-plan.md)
- [Terminal water/ocean shader plan](tui-vfx-terminal-water-shader-plan.md)
- [V3 style model restructure inventory](tui-vfx-v3-style-model-restructure-inventory.md)
- [V3 recipe ingredients reference plan](tui-vfx-v3-recipe-ingredients-reference-plan.md)
- [V3 release-gate manifest seed](tui-vfx-v3-release-gate-manifest.md)
- [V3 first slice checklist](tui-vfx-v3-first-slice-checklist.md)

## Retained historical/planning records

Read these for rationale or closed-out context. Do not treat stale `draft`,
`open question`, or `deferred` language here as current task status without
checking the master punch list and lifecycle plan.

- [V3 upgrade-plan chapter index](tui-vfx-v3-upgrade-plan/00_INDEX.md)
- [V3 motion spec](tui-vfx-v3-motion-spec.md)
- [V3 motion compatibility table](tui-vfx-v3-motion-compatibility-table.md)
- [V3 shadow spec](tui-vfx-v3-shadow-spec.md)
- [V3 vanishing edge spec](tui-vfx-v3-vanishing-edge-spec.md)
- [V3 migration findings memo](tui-vfx-v3-migration-findings-memo-claude.md)
- [V3 upgrade audit workflow](tui-vfx-v3-upgrade-audit-workflow.md)
- [V3 upgrade debug recipes migration log](tui-vfx-v3-upgrade-debug-recipes-migration-log.md)
- [V3 naming implementation inventory](tui-vfx-v3-naming-implementation-inventory.md)
- [V3 legacy Ra-to-Vfx rename inventory](tui-vfx-v3-ra-to-vfx-rename-inventory.md)

## Architecture and decisions

- [V3 execution DAG](tui-vfx-v3-execution-dag.md)
- [Upgrade plan chapter index](tui-vfx-v3-upgrade-plan/00_INDEX.md)
- [V3 normalized IR](tui-vfx-v3-normalized-ir.md)
- [V3 lowering map](tui-vfx-v3-lowering-map.md)
- [V3 compiled execution plan](tui-vfx-v3-compiled-execution-plan.md)
- [V3 scheduler batching plan](tui-vfx-v3-scheduler-batching-plan.md)
- [V3 validator canonicalization checklist](tui-vfx-v3-validator-canonicalization-checklist.md)
- [V3 naming normalization decisions](tui-vfx-v3-naming-normalization-decisions.md)
- [V3 naming implementation inventory](tui-vfx-v3-naming-implementation-inventory.md)
- [V3 phase-scoping decision](tui-vfx-v3-phase-scoping-decision.md)
- [V3 scope and composition decision](tui-vfx-v3-scope-composition-decision.md)
- [V3 timing and metadata decision](tui-vfx-v3-timing-and-metadata-decision.md)

## Authoring vocabulary and schema

Start recipe authoring in the sibling ten-section ladder:
`../../../tui-vfx-recipes/docs/scene/AUTHORING_GUIDE.md`. Then use the links below
for canonical words, exact schema facts, ingredient governance, and specialized
primitive behavior.

- [V3 recipe vocabulary](tui-vfx-v3-recipe-vocabulary.md)
- [V3 recipe ingredients reference plan](tui-vfx-v3-recipe-ingredients-reference-plan.md)
- [V3 schema overview](tui-vfx-v3-schema-overview.md)
- [V3 schema draft](tui-vfx-v3-schema-draft.json)
- Sibling hand-maintained schema companion: `../../../tui-vfx-recipes/docs/schema/SCHEMA_REFERENCE.md`
- Sibling generated V3 API inventory: `../../../tui-vfx-recipes/docs/generated/V3_API.md`
- Sibling procedural source catalog: `../../../tui-vfx-recipes/docs/scene/PROCEDURAL_SOURCES.md`
- [V3 motion spec](tui-vfx-v3-motion-spec.md)
- [V3 motion compatibility table](tui-vfx-v3-motion-compatibility-table.md)
- [V3 shadow spec](tui-vfx-v3-shadow-spec.md)
- [V3 vanishing edge spec](tui-vfx-v3-vanishing-edge-spec.md)

## I/O, composition, and execution

- [V3 I/O contract](tui-vfx-v3-io-contract.md)
- [V3 cross-family coverage plan](tui-vfx-v3-cross-family-coverage-plan.md)
- [V3 core pipeline readiness matrix](tui-vfx-v3-core-pipeline-readiness-matrix.md)
- [V3 spatial field hint plan](tui-vfx-v3-spatial-field-hint-plan.md)
- [V3 scene content integration plan](tui-vfx-v3-scene-content-integration-plan.md)
- [V3 per-cell motion plan](tui-vfx-v3-per-cell-motion-plan.md)
- [V3 glyph particle emitter plan](tui-vfx-v3-glyph-particle-emitter-plan.md)
- [V3 TTE-inspired remaining-work plan](tui-vfx-v3-tte-inspired-remaining-work-plan.md)
- [Terminal water/ocean shader plan](tui-vfx-terminal-water-shader-plan.md)
- [V3 style model restructure inventory](tui-vfx-v3-style-model-restructure-inventory.md)

## Tooling and edge adapters

- [Tooling hub](../tooling/INDEX.md)
- [Probe database and frame diffs](../tooling/probe-database-and-frame-diffs.md)
- [V3 preview and thin player surface](../tooling/v3-preview-and-thin-player.md)
- [Grid resize adapter contract](../tooling/grid-resize-adapter-contract.md)
- [Edge ingestion and capture tooling](../tooling/edge-ingestion-and-capture.md)
- [Chapter 63 — edge ingestion and runtime adapters](tui-vfx-v3-upgrade-plan/63_edge_ingestion_runtime_adapters.md)

## Migration and debug recipes

- [V3 upgrade audit workflow](tui-vfx-v3-upgrade-audit-workflow.md)
- [V3 docs lifecycle/elevation plan](tui-vfx-v3-docs-lifecycle-plan.md)
- [V3 upgrade debug recipes migration log](tui-vfx-v3-upgrade-debug-recipes-migration-log.md)
- [V3 migration findings memo](tui-vfx-v3-migration-findings-memo-claude.md)
- [V3 migration outcome policy](tui-vfx-v3-migration-outcome-policy.md)
- [V3 release-gate policy](tui-vfx-v3-release-gate-policy.md)
- [V3 release-gate manifest seed](tui-vfx-v3-release-gate-manifest.md)
- [V3 first slice checklist](tui-vfx-v3-first-slice-checklist.md)
- [V3 outstanding master punch list](tui-vfx-v3-outstanding-master-list.md)

## Capability orientation

- [Root capabilities guide](../../CAPABILITIES.md) — hand-maintained V3 capability orientation; use generated capability docs for extracted inventory facts.

## Specialized capability plans

- [V3 capability catalog](tui-vfx-v3-capability-catalog.md)
- [V3 capability governance decision](tui-vfx-v3-capability-governance-decision.md)
- [Post-release capability specs](post-release/INDEX.md)
  - [Braille dotfield toolkit plan](post-release/braille-dotfield-toolkit-plan.md)
  - [Glyph actor procedural spec](post-release/glyph-actor-procedural-spec.md)
  - [Weather ambient field spec](post-release/weather-ambient-field-spec.md)

## Rules of thumb

- V3-only implementation work should still keep V2 fallback available until the
  final V2-retirement gate in the master punch list.
- Public/schema-bearing changes need rustdocs plus generated-doc validation.
- Hand-maintained docs describe intent, constraints, and author-facing usage;
  generated docs describe extracted facts.
- Tooling work should reuse existing probe/database/frame-diff surfaces before
  adding new formats.
- Reusable signal/math substrate belongs in `mixed-signals`; effect/render
  semantics belong in `tui-vfx` / `tui-vfx-recipes`.

<!-- <FILE>docs/design/tui-vfx-v3-INDEX.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.6.2</VERS> -->
