<!-- <FILE>docs/design/tui-vfx-v3-docs-lifecycle-plan.md</FILE> - <DESC>V3 docs lifecycle/elevation plan: classifies docs into core reference, active plan, retained record, completed-work, and archive/merge buckets, with the 2026-04-27 cutover into completed/ and history/ subdirectories recorded.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Record the 2026-04-27 cutover that moved completed implementation plans into docs/design/completed/ and historical/rationale records into docs/design/history/.</WCTX> -->
<!-- <CLOG>0.3.0: record the 2026-04-27 cutover — completed implementation plans moved to completed/, historical/rationale records moved to history/; supersede the prior "do not move" guardrail and update bucket paths.</CLOG> -->

# V3 docs lifecycle / elevation plan

> Status: classification map updated for the 2026-04-27 cutover. Completed
> implementation plans now live under `docs/design/completed/`; historical and
> rationale records now live under `docs/design/history/`. The earlier
> "non-destructive, no-move" stance is superseded.

This file turns the V3-DOCS01 punch-list item into a concrete classification
map. It keeps the focus on docs lifecycle and elevation, not on broader
punch-list execution.

## Scope and guardrails

- The 2026-04-27 cutover moved completed plans into `completed/` and historical
  records into `history/`. Bucket paths below reflect the post-cutover layout.
- Two docs in the high-confidence DONE bucket were intentionally **left in
  place** pending owner review:
  `docs/design/tui-vfx-buy-once-architecture-sweep.md` (multi-finding sweep with
  mixed status) and `docs/design/tui-vfx-terminal-water-shader-plan.md`
  (downstream framework calls it implemented but the plan's own status text is
  ambiguous).
- Keep tui-vfx focused on architecture and docs that belong to tui-vfx.
- Do not pull tui-vfx-recipes owner policy into this repo.
- When a retained record says `draft`, `deferred`, or `open question`, verify
  current status against the master punch list before creating work from it.

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
- `docs/design/tui-vfx-v3-migration-outcome-policy.md` — accepted migration
  outcome classification policy while owner recipe audit remains pending.

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
- `docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md` — active
  release-blocking tooling/CI checklist.
- `docs/design/tui-vfx-v3-upgrade-plan/63_edge_ingestion_runtime_adapters.md`
  and `63a_eira_05_grapheme_storage_review.md` — edge tooling and grapheme
  review status surfaces; most effect/path primitives are complete, while docs,
  tooling, and adapter follow-up can still be active.

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
- `docs/design/tui-vfx-v3-upgrade-plan/70_shape_sketches.md`
- `docs/design/tui-vfx-v3-upgrade-plan/80_open_questions.md`
- `docs/design/tui-vfx-v3-upgrade-plan/90_deferred_design.md`
- `docs/design/tui-vfx-v3-upgrade-plan/110_appendix_audits.md`
- `docs/design/history/tui-vfx-v3-motion-spec.md` — draft motion design note.
- `docs/design/history/tui-vfx-v3-shadow-spec.md` — draft shadow design note.
- `docs/design/history/tui-vfx-v3-vanishing-edge-spec.md` — draft viewport-edge note.
- `docs/design/post-release/braille-dotfield-toolkit-plan.md` — post-release
  strategy note.
- `docs/design/post-release/glyph-actor-procedural-spec.md` — post-release
  procedural spec.
- `docs/design/post-release/weather-ambient-field-spec.md` — post-release
  ambient-field spec.
- `docs/design/history/tui-vfx-v3-migration-findings-memo-claude.md` — migration
  findings memo.
- `docs/design/history/tui-vfx-v3-upgrade-audit-workflow.md` — audit/curation workflows
  reference.
- `docs/design/history/tui-vfx-v3-upgrade-debug-recipes-migration-log.md` — debug
  recipes migration log and schema journal.
- `docs/design/history/tui-vfx-v3-naming-implementation-inventory.md` — rename rollout
  inventory.
- `docs/design/history/tui-vfx-v3-ra-to-vfx-rename-inventory.md` — legacy rename
  inventory.
- `docs/design/tui-vfx-v3-style-model-restructure-inventory.md` — style-model
  restructure inventory.

## 4) Stale status reconciliation

The highest-risk checklist status drift has been reconciled against the master
punch list and implemented evidence.

- `docs/design/tui-vfx-v3-first-slice-checklist.md` — no remaining explicit
  `IN_PROGRESS` rows; FS-01 through FS-08 now point at initial parse, normalize,
  validation, scene placement, and dump evidence.
- `docs/design/tui-vfx-v3-validator-canonicalization-checklist.md` — no
  remaining explicit `IN_PROGRESS` rows; VC-01 through VC-10 now distinguish
  complete, complete-initial, and partial migration-equivalence work.
- `docs/design/tui-vfx-v3-outstanding-master-list.md` remains leader-owned; this
  plan should be used as the evidence note for the V3-DOCS01/QDOC01 status
  recommendation rather than edited directly from this packet.

## 5) Archive / merge candidates

These are not deletion orders. They are candidates for later consolidation once
owners confirm the final cutover shape.

- `docs/design/history/tui-vfx-v3-naming-implementation-inventory.md` → likely fold into
  `docs/design/tui-vfx-v3-naming-normalization-decisions.md` after rename
  cutover.
- `docs/design/history/tui-vfx-v3-ra-to-vfx-rename-inventory.md` → likely fold into the
  same naming decision record after cutover.
- `docs/design/tui-vfx-v3-release-gate-manifest.seed.json` → likely merge into
  `docs/design/tui-vfx-v3-release-gate-manifest.md` once the manifest shape is
  stable.
- `docs/design/history/tui-vfx-v3-upgrade-debug-recipes-migration-log.md` and
  `docs/design/history/tui-vfx-v3-migration-findings-memo-claude.md` → possible future
  synthesis record if the remaining schema-history detail can be frozen into one
  retained history artifact.
- `docs/design/post-release/*.md` → keep separate and deferred unless the owner
  promotes one into an active post-V3 workstream.
- `docs/design/tui-vfx-v3-style-model-restructure-inventory.md` → may later be
  collapsed into the capability/normalized-IR record set once implementation
  stops needing a separate inventory.

## Focused link/status audit outcome

- Current status buckets are reflected in `docs/design/tui-vfx-v3-INDEX.md` so
  agents can distinguish canonical live docs, active follow-up, retained history,
  and post-release/deferred specs from the landing page.
- Local Markdown links under `docs/design/**/*.md` and `docs/tooling/**/*.md`
  pass the focused checker used for this ticket.
- No broken local Markdown targets remained to repair in this pass.
- No missing historical/external Markdown links remained that needed conversion
  to plain text in this pass.
- The V3 landing page links this lifecycle plan from the start-here section, so
  the docs lifecycle lane is discoverable from the canonical index.

## Active follow-up / remaining watch items

- Historical design records still intentionally use words such as `draft`,
  `deferred`, and `open question` where they describe retained history or
  explicitly post-release territory. Do not bulk-normalize those terms without
  owner approval.
- Keep auditing lifecycle status as active V3 docs change; this file is a map,
  not a one-time archive operation.
- `V3-M02`, `V3-CI02`, `V3-M01`, and final V2 retirement statuses remain active
  project work, not docs-hygiene drift.
- Keep downstream consumer policy out of this repo; tui-vfx owns architecture
  and docs for tui-vfx, while tui-vfx-recipes owns recipe-authoring/runtime
  policy.
