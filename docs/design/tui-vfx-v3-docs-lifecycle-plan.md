<!-- <FILE>docs/design/tui-vfx-v3-docs-lifecycle-plan.md</FILE> - <DESC>Provisional V3 docs lifecycle/elevation plan: classifies live docs into core reference, active plan, retained record, stale-status cleanup, and later archive/merge buckets without moving or rewriting anything yet.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Classify the V3 docs set so the outstanding V3-DOCS01 lane has a concrete, non-destructive elevation plan.</WCTX> -->
<!-- <CLOG>0.1.0: initial classification plan for V3 docs lifecycle/elevation work.</CLOG> -->

# V3 docs lifecycle / elevation plan

> Status: draft classification plan only. No docs are moved, deleted, archived, or rewritten in this pass.

This file turns the V3-DOCS01 punch-list item into a concrete, non-destructive
classification map. It keeps the focus on docs lifecycle and elevation, not on
broader punch-list execution.

## Scope and guardrails

- Do not move, delete, archive, or rewrite any existing V3 docs in this pass.
- Keep tui-vfx focused on architecture and docs that belong to tui-vfx.
- Do not pull tui-vfx-recipes owner policy into this repo.
- Treat the buckets below as recommendations, not final ownership decisions.

## 1) Durable core docs

These should stay prominently linked from the V3 landing page because they are
canonical reference surfaces or accepted decision records.

- `docs/design/tui-vfx-v3-INDEX.md` — landing page for the V3 doc set.
- `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md` — canonical chapter hub and
  pinned schema reference.
- `docs/design/tui-vfx-v3-schema-draft.json` — specification-by-example for the
  V3 recipe shape.
- `docs/design/tui-vfx-v3-schema-overview.md` — narrative schema guide.
- `docs/design/tui-vfx-v3-io-contract.md` — V3 I/O boundary contract.
- `docs/design/tui-vfx-v3-recipe-vocabulary.md` — canonical authoring language.
- `docs/design/tui-vfx-v3-naming-normalization-decisions.md` — accepted naming
  slate.
- `docs/design/tui-vfx-v3-capability-governance-decision.md` — accepted
  promotion/governance rules.
- `docs/design/tui-vfx-v3-phase-scoping-decision.md` — accepted phase rule.
- `docs/design/tui-vfx-v3-scope-composition-decision.md` — accepted scope and
  combine defaults.
- `docs/design/tui-vfx-v3-timing-and-metadata-decision.md` — accepted metadata
  and distributed timing policy.
- `docs/design/tui-vfx-v3-release-gate-policy.md` — accepted release-gate
  policy.

## 2) Active implementation plans

These are the live work surfaces that still shape implementation sequencing,
proofs, or validator/runtime follow-through.

- `docs/design/tui-vfx-v3-outstanding-master-list.md` — master punch list and
  follow-on tracker.
- `docs/design/tui-vfx-v3-execution-dag.md` — dependency and ownership DAG.
- `docs/design/tui-vfx-v3-capability-catalog.md` — live capability catalog.
- `docs/design/tui-vfx-v3-lowering-map.md` — V2→V3 lowering map.
- `docs/design/tui-vfx-v3-normalized-ir.md` — execution-facing normalized IR
  note.
- `docs/design/tui-vfx-v3-compiled-execution-plan.md` — compiled execution-plan
  note.
- `docs/design/tui-vfx-v3-scheduler-batching-plan.md` — scheduler/batching
  follow-on.
- `docs/design/tui-vfx-v3-cross-family-coverage-plan.md` — cross-family
  coverage follow-on.
- `docs/design/tui-vfx-v3-scene-content-integration-plan.md` — scene/content
  integration follow-on.
- `docs/design/tui-vfx-v3-spatial-field-hint-plan.md` — spatial field and hint
  plan.
- `docs/design/tui-vfx-v3-motion-compatibility-table.md` — motion lowering map
  for route/dynamics/from/to/edge behavior.
- `docs/design/tui-vfx-v3-recipe-ingredients-reference-plan.md` — ingredients
  reference plan.
- `docs/design/tui-vfx-v3-first-slice-checklist.md` — first implementation
  slice tracker.
- `docs/design/tui-vfx-v3-validator-canonicalization-checklist.md` — validator
  and canonicalization tracker.
- `docs/design/tui-vfx-v3-release-gate-manifest.md` — checked-in release-gate
  manifest seed.
- `docs/design/tui-vfx-v3-release-gate-manifest.seed.json` — machine-readable
  manifest companion seed.

## 3) Design records to retain

These are durable historical or rationale records. They preserve why V3 took a
particular shape, even when they are no longer the active work surface.

- `docs/design/tui-vfx-v3-upgrade-plan/10_philosophy.md`
- `docs/design/tui-vfx-v3-upgrade-plan/20_architectural_framing.md`
- `docs/design/tui-vfx-v3-upgrade-plan/30_why_now.md`
- `docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md`
- `docs/design/tui-vfx-v3-upgrade-plan/50_migration_workflow.md`
- `docs/design/tui-vfx-v3-upgrade-plan/55_schema_to_implementation_roadmap.md`
- `docs/design/tui-vfx-v3-upgrade-plan/56_capability_catalog_and_execution.md`
- `docs/design/tui-vfx-v3-upgrade-plan/57_v2_to_v3_lowering_rules.md`
- `docs/design/tui-vfx-v3-upgrade-plan/58_normalized_ir_and_authoring_types.md`
- `docs/design/tui-vfx-v3-upgrade-plan/59_validator_and_canonicalization.md`
- `docs/design/tui-vfx-v3-upgrade-plan/60_testing_release_gates.md`
- `docs/design/tui-vfx-v3-upgrade-plan/61_first_implementation_slice.md`
- `docs/design/tui-vfx-v3-upgrade-plan/62_compiled_execution_plan.md`
- `docs/design/tui-vfx-v3-upgrade-plan/63_edge_ingestion_runtime_adapters.md`
- `docs/design/tui-vfx-v3-upgrade-plan/70_shape_sketches.md`
- `docs/design/tui-vfx-v3-upgrade-plan/80_open_questions.md`
- `docs/design/tui-vfx-v3-upgrade-plan/90_deferred_design.md`
- `docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md`
- `docs/design/tui-vfx-v3-upgrade-plan/110_appendix_audits.md`
- `docs/design/tui-vfx-v3-motion-spec.md` — draft motion design note.
- `docs/design/tui-vfx-v3-shadow-spec.md` — draft shadow design note.
- `docs/design/tui-vfx-v3-vanishing-edge-spec.md` — draft viewport-edge note.
- `docs/design/tui-vfx-v3-braille-dotfield-toolkit-plan.md` — post-release
  strategy note.
- `docs/design/tui-vfx-v3-migration-findings-memo-claude.md` — migration
  findings memo.
- `docs/design/tui-vfx-v3-upgrade-audit-workflow.md` — audit/curation workflows
  reference.
- `docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md` — debug
  recipes migration log and schema journal.
- `docs/design/tui-vfx-v3-naming-implementation-inventory.md` — rename rollout
  inventory.
- `docs/design/tui-vfx-v3-ra-to-vfx-rename-inventory.md` — legacy rename
  inventory.
- `docs/design/tui-vfx-v3-style-model-restructure-inventory.md` — style-model
  restructure inventory.

## 4) Stale status docs to reconcile

These are the highest-risk files for misleading lifecycle language. They should
be rechecked against as-built state before any future cleanup pass.

- `docs/design/tui-vfx-v3-first-slice-checklist.md` — still contains 8 explicit
  `IN_PROGRESS` rows.
- `docs/design/tui-vfx-v3-validator-canonicalization-checklist.md` — still
  contains 2 explicit `IN_PROGRESS` rows.
- `docs/design/tui-vfx-v3-outstanding-master-list.md` already calls this out as
  `V3-QDOC01`.

## 5) Archive / merge candidates

These are not deletion orders. They are candidates for later consolidation once
owners confirm the final cutover shape.

- `docs/design/tui-vfx-v3-naming-implementation-inventory.md` → likely fold into
  `docs/design/tui-vfx-v3-naming-normalization-decisions.md` after rename
  cutover.
- `docs/design/tui-vfx-v3-ra-to-vfx-rename-inventory.md` → likely fold into the
  same naming decision record after cutover.
- `docs/design/tui-vfx-v3-release-gate-manifest.seed.json` → likely merge into
  `docs/design/tui-vfx-v3-release-gate-manifest.md` once the manifest shape is
  stable.
- `docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md` and
  `docs/design/tui-vfx-v3-migration-findings-memo-claude.md` → possible future
  synthesis record if the remaining schema-history detail can be frozen into one
  retained history artifact.
- `docs/design/tui-vfx-v3-style-model-restructure-inventory.md` → may later be
  collapsed into the capability/normalized-IR record set once implementation
  stops needing a separate inventory.

## High-risk takeaways

- The only explicit stale-life-cycle language I found that looks behind the
  as-built state is the pair of checklist files above.
- The master list already has a named cleanup item for this lane, so the new
  plan is aligned with existing punch-list intent.
- Keep downstream consumer policy out of this repo; tui-vfx owns architecture
  and docs for tui-vfx, while tui-vfx-recipes owns recipe-authoring/runtime
  policy.

## Follow-up

If this bucket map is accepted, the V3 landing page should link here so the docs
lifecycle lane is discoverable from the canonical index.
