<!-- <FILE>docs/new_kernel/INDEX.md</FILE> - <DESC>Index for clean-room new-kernel planning and status documents</DESC> -->
<!-- <VERS>VERSION: 0.8.0</VERS> -->
<!-- <WCTX>New kernel Phase E0: add physical contract split artifacts.</WCTX> -->
<!-- <CLOG>0.8.0: MINOR — add Phase D3 architect response, E0 status artifacts, and contract/proof schema path split.
0.7.0: MINOR — add Phase D2 architect response, D3 boundary doc, and D3 status artifacts.
0.6.0: MINOR — add preserved contract-add decision tree supporting artifact.
0.5.0: MINOR — add Phase D1 architect response plus Phase D2 template composition artifacts.
0.4.0: MINOR — add Phase D1 architect status memo.
0.3.1: PATCH — include public SceneOutcome schema root.
0.3.0: MINOR — add Phase D0 architect response, Phase D1 status, and scene/element/outcome schema roots.
0.2.1: PATCH — add Phase D0 architect memo to status artifacts.
0.2.0: MINOR — add architecture overview, Phase C/D0 status artifacts, schemas, and Phase D0 plans.
0.1.0: INIT — list briefing, phase inputs/responses/status memos, and parent docs/v3.1 contract references.</CLOG> -->

# New Kernel Docs Index

This directory tracks the clean-room kernel effort for proving the v3.1 contract model before migration or effect porting.

## Start here

- [`AGENT_BRIEFING.md`](AGENT_BRIEFING.md) — reusable briefing for agents working on clean-room kernel phases.
- [`PROJECT_KICKOFF.md`](PROJECT_KICKOFF.md) — original bounded Phase A kickoff prompt/spec.
- [`DRAFT_CONTRACTS.md`](DRAFT_CONTRACTS.md) — broader directional contract discussion; context, not a single-phase implementation plan.

## v3.1 contract docs one level up

- [`../v3.1-architecture-overview.md`](../v3.1-architecture-overview.md) — contract-first overview, progressive phase stack, schema/reference path, and D3 boundary.
- [`../v3.1-contract-boundary.md`](../v3.1-contract-boundary.md) — D3/E0 classification of contract vocabulary, proof implementation, crate ownership, and schema roots.
- [`../v3.1-surface-contract.md`](../v3.1-surface-contract.md) — current clean-room surface/sampling/pipeline/scene contract owned by `tui-vfx-contract` and proven by `tui-vfx-next`.
- [`../v3.1-feature-contract-checklist.md`](../v3.1-feature-contract-checklist.md) — reusable decision checklist for future contract-affecting features.
- [`../v3.1-template-composition.md`](../v3.1-template-composition.md) — Phase D2 design for compile-time template composition into canonical recipes.

## Architect responses

- [`ARCH-RESP-TO-PHASE_A.md`](ARCH-RESP-TO-PHASE_A.md) — Phase A approval and Phase B recommendation.
- [`ARCH-RESP-TO-PHASE_B.md`](ARCH-RESP-TO-PHASE_B.md) — Phase B approval and Phase C recommendation.
- [`ARCH-RESP-TO-PHASE_C.md`](ARCH-RESP-TO-PHASE_C.md) — Phase C approval and Phase D0 schema/reference backfill recommendation.
- [`ARCH-RESP-TO-PHASE_D0.md`](ARCH-RESP-TO-PHASE_D0.md) — Phase D0 approval and Phase D1 scene/element/layer recommendation.
- [`ARCH-RESP-TO-PHASE_D1.md`](ARCH-RESP-TO-PHASE_D1.md) — Phase D1 approval and Phase D2 template composition recommendation.
- [`ARCH-RESP-TO-PHASE_D2.md`](ARCH-RESP-TO-PHASE_D2.md) — Phase D2 approval and Phase D3 contract/engine boundary recommendation.
- [`ARCH-RESP-TO-PHASE_D3.md`](ARCH-RESP-TO-PHASE_D3.md) — Phase D3 approval and Phase E0 physical contract split recommendation.

## Status and memo artifacts

- [`PHASE_A_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_A_STATUS_MEMO_TO_ARCHITECT.md) — Phase A status memo.
- [`PHASE_B_STATUS.md`](PHASE_B_STATUS.md) — concise Phase B status.
- [`PHASE_B_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_B_STATUS_MEMO_TO_ARCHITECT.md) — Phase B status memo.
- [`PHASE_C_STATUS.md`](PHASE_C_STATUS.md) — concise Phase C status.
- [`PHASE_C_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_C_STATUS_MEMO_TO_ARCHITECT.md) — Phase C architect memo.
- [`PHASE_D0_STATUS.md`](PHASE_D0_STATUS.md) — concise Phase D0 status.
- [`PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md) — Phase D0 architect memo requesting next assignment.
- [`PHASE_D1_STATUS.md`](PHASE_D1_STATUS.md) — concise Phase D1 scene/element/layer composition status.
- [`PHASE_D1_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_D1_STATUS_MEMO_TO_ARCHITECT.md) — Phase D1 architect memo requesting next assignment.
- [`PHASE_D2_STATUS.md`](PHASE_D2_STATUS.md) — concise Phase D2 template composition status.
- [`PHASE_D2_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_D2_STATUS_MEMO_TO_ARCHITECT.md) — Phase D2 architect memo requesting next assignment.
- [`PHASE_D3_STATUS.md`](PHASE_D3_STATUS.md) — concise Phase D3 contract/engine boundary status.
- [`PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md) — Phase D3 architect memo requesting next assignment.
- [`PHASE_E0_STATUS.md`](PHASE_E0_STATUS.md) — concise Phase E0 physical contract split status.
- [`PHASE_E0_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_E0_STATUS_MEMO_TO_ARCHITECT.md) — Phase E0 architect memo requesting next assignment.

## Supporting artifacts

- [`CONTRACT_ADD_DECISION_TREE.md`](CONTRACT_ADD_DECISION_TREE.md) — preserved source decision-tree draft for classifying contract-visible feature additions.
- [`PROCESS_OVERVIEW_DIAGRAM.md`](PROCESS_OVERVIEW_DIAGRAM.md) — process overview diagram/notes.
- [`TEMPLATE_INHERITANCE.md`](TEMPLATE_INHERITANCE.md) — scene/template composition guidance from architect discussion.

## Schema artifacts

- [`../../schemas/v3.1/contract/`](../../schemas/v3.1/contract/) — checked generated stable contract schemas for surface, scope, write, diagnostics, scene, element, and outcome.
- [`../../schemas/v3.1/next/`](../../schemas/v3.1/next/) — checked generated proof-pipeline schemas for sampler and pipeline artifacts.

## Ralph planning artifacts

- `.omx/plans/prd-new-kernel-phase-d0.md` — Phase D0 PRD.
- `.omx/plans/test-spec-new-kernel-phase-d0.md` — Phase D0 verification spec.
