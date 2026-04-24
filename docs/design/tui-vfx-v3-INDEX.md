<!-- <FILE>docs/design/tui-vfx-v3-INDEX.md</FILE> - <DESC>Single landing page for V3 planning, schema, implementation, tooling, and migration docs.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Add the V3 execution DAG to the V3 landing page so parallel tracks and dependencies are discoverable from the canonical start page.</WCTX> -->
<!-- <CLOG>0.3.0: add V3 execution DAG link. 0.2.0: add timing and metadata decision link. 0.1.0: initial V3 documentation index spanning the upgrade plan, schema, I/O, motion, scene/content, tooling, migration, and outstanding punch list docs.</CLOG> -->

# V3 documentation index

Start here when working on tui-vfx V3. This page points to the durable planning
chapters, as-built contracts, schema docs, tooling guides, and live punch list.

## Start here

1. [V3 outstanding master punch list](tui-vfx-v3-outstanding-master-list.md)
2. [V3 execution DAG](tui-vfx-v3-execution-dag.md)
3. [V3 upgrade-plan chapter index](tui-vfx-v3-upgrade-plan/00_INDEX.md)
4. [V3 schema overview](tui-vfx-v3-schema-overview.md)
5. [V3 schema draft](tui-vfx-v3-schema-draft.json)
6. [V3 I/O contract](tui-vfx-v3-io-contract.md)
7. [V3 naming normalization decisions](tui-vfx-v3-naming-normalization-decisions.md)
8. [V3 phase-scoping decision](tui-vfx-v3-phase-scoping-decision.md)
9. [V3 migration outcome policy](tui-vfx-v3-migration-outcome-policy.md)
10. [V3 release-gate policy](tui-vfx-v3-release-gate-policy.md)
11. [V3 scope and composition decision](tui-vfx-v3-scope-composition-decision.md)
12. [V3 capability governance decision](tui-vfx-v3-capability-governance-decision.md)
13. [V3 timing and metadata decision](tui-vfx-v3-timing-and-metadata-decision.md)
14. [Tooling hub](../tooling/INDEX.md)

## Architecture and decisions

- [V3 execution DAG](tui-vfx-v3-execution-dag.md)
- [Upgrade plan chapter index](tui-vfx-v3-upgrade-plan/00_INDEX.md)
- [V3 normalized IR](tui-vfx-v3-normalized-ir.md)
- [V3 lowering map](tui-vfx-v3-lowering-map.md)
- [V3 compiled execution plan](tui-vfx-v3-compiled-execution-plan.md)
- [V3 scheduler batching plan](tui-vfx-v3-scheduler-batching-plan.md)
- [V3 validator canonicalization checklist](tui-vfx-v3-validator-canonicalization-checklist.md)
- [V3 naming normalization decisions](tui-vfx-v3-naming-normalization-decisions.md)
- [V3 phase-scoping decision](tui-vfx-v3-phase-scoping-decision.md)
- [V3 scope and composition decision](tui-vfx-v3-scope-composition-decision.md)
- [V3 timing and metadata decision](tui-vfx-v3-timing-and-metadata-decision.md)

## Authoring vocabulary and schema

- [V3 recipe vocabulary](tui-vfx-v3-recipe-vocabulary.md)
- [V3 schema overview](tui-vfx-v3-schema-overview.md)
- [V3 schema draft](tui-vfx-v3-schema-draft.json)
- [V3 motion spec](tui-vfx-v3-motion-spec.md)
- [V3 shadow spec](tui-vfx-v3-shadow-spec.md)
- [V3 vanishing edge spec](tui-vfx-v3-vanishing-edge-spec.md)

## I/O, composition, and execution

- [V3 I/O contract](tui-vfx-v3-io-contract.md)
- [V3 cross-family coverage plan](tui-vfx-v3-cross-family-coverage-plan.md)
- [V3 spatial field hint plan](tui-vfx-v3-spatial-field-hint-plan.md)
- [V3 scene content integration plan](tui-vfx-v3-scene-content-integration-plan.md)
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
- [V3 upgrade debug recipes migration log](tui-vfx-v3-upgrade-debug-recipes-migration-log.md)
- [V3 migration findings memo](tui-vfx-v3-migration-findings-memo-claude.md)
- [V3 migration outcome policy](tui-vfx-v3-migration-outcome-policy.md)
- [V3 release-gate policy](tui-vfx-v3-release-gate-policy.md)
- [V3 first slice checklist](tui-vfx-v3-first-slice-checklist.md)
- [V3 outstanding master punch list](tui-vfx-v3-outstanding-master-list.md)

## Specialized capability plans

- [V3 capability catalog](tui-vfx-v3-capability-catalog.md)
- [V3 capability governance decision](tui-vfx-v3-capability-governance-decision.md)
- [V3 braille dotfield toolkit plan](tui-vfx-v3-braille-dotfield-toolkit-plan.md)

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
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
