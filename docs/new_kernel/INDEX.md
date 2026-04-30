<!-- <FILE>docs/new_kernel/INDEX.md</FILE> - <DESC>Index for clean-room new-kernel planning and status documents</DESC> -->
<!-- <VERS>VERSION: 0.33.0</VERS> -->
<!-- <WCTX>K2.16 player render IR and debug-recipe backlog burn-down indexing.</WCTX> -->
<!-- <CLOG>0.33.0: MINOR — add K2.17 descriptor/source burn-down reports, implementation-readiness, control catalog, holdbacks, docs gate, review, and architect memo artifacts.
0.32.0: MINOR — add K2.16 player render IR, scene/source fidelity, backlog normalization, burn-down, docs-gate, holdback, review, and architect memo artifacts.
0.31.0: MINOR — add K2.15 graph execution, fixture, source, docs-gate, holdback, and review artifacts.
0.30.2: PATCH — add K2.14 descriptor/adapter migration reports and review/de-slop artifact.
0.30.1: PATCH — add K2.13 schema decision burn-down reports and review/de-slop artifact.
0.30.0: MINOR — add K2.12 schema-lock decision sprint reports, status memo, and review/de-slop artifact.
0.29.0: MINOR — add K2.11 briefing-first review and de-slop report.
0.28.0: MINOR — add K2.11 schema-readiness ledger, source/content/scene table, studio preflight deferral, and status memo.
0.27.0: MINOR — add K2.10 architect response and K2.11 schema-readiness recommendation.
0.26.0: MINOR — add K2.9 architect response and K2.10 corpus mapping/backlog artifacts.
0.25.0: MINOR — add K2.5 architect response, evidence, tooling PRD, and status memo.
0.24.0: MINOR — add K2.0-K2.4 docs, review/de-slop artifacts, player reports, and schema directory details.
0.23.0: MINOR — add Phase K0 architect response plus K1 visual player UI artifacts.
0.22.0: MINOR — add Phase J2 architect response plus K0 player artifacts.
0.21.0: MINOR — add Phase J1 architect response plus J2 descriptor pack/catalog artifacts.
0.20.0: MINOR — add Phase J0 architect response plus J1 validator harness artifacts.
0.19.0: MINOR — add Phase I0 architect response, J0 primitive migration artifacts, contract CLI, and canonical recipe root.
0.18.0: MINOR — add Phase H1 architect response, I0 lifecycle/time/trigger artifacts, and schema roots.
0.17.0: MINOR — add Phase H0 architect response, H1 vocabulary/evidence/status artifacts, and recipe schema roots.
0.16.0: MINOR — add Phase G4 architect response, H0 status artifacts, and source/asset schema roots.
0.15.0: MINOR — add Phase G3 architect response and G4 status artifacts.
0.14.0: MINOR — add Phase G2 architect response and G3 status artifacts.
0.13.0: MINOR — add Phase G1 architect response and G2 status artifacts.
0.12.0: MINOR — add Phase F2 architect response, G1 status artifacts, and graph/node schema roots.
0.11.0: MINOR — add Phase F1 architect response, F2 status artifacts, and source/binding/graph schema roots.
0.10.0: MINOR — add Phase E1 architect response, F1 status artifacts, and value/input schema roots.
0.9.0: MINOR — add Phase E0 architect response, E1 status artifacts, and effect descriptor schema root.
0.8.0: MINOR — add Phase D3 architect response, E0 status artifacts, and contract/proof schema path split.
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

- [`../VOCABULARY.md`](../VOCABULARY.md) — canonical v3.1 human vocabulary, non-canonical legacy/source-authoring synonym map, J1 migration/validation/parity terminology, and J2 descriptor pack/catalog terms.
- [`../v3.1-architecture-overview.md`](../v3.1-architecture-overview.md) — contract-first overview, progressive phase stack, schema/reference path, D3 boundary, E0 split, E1 descriptor model, F1 typed inputs, F2 declarative value sources/bindings, G1 canonical graph container, G2 graph execution proof, G3 topology/channel-aware merge semantics, G4 node I/O value-bus semantics, H0 source/asset/procedural source contracts, H1 canonical recipe document packaging, I0 lifecycle/time/trigger contracts, J0 primitive migration pilot validation, J1 validator harness hardening, and J2 descriptor-pack catalog work.
- [`../v3.1-contract-boundary.md`](../v3.1-contract-boundary.md) — D3/E0/E1/F1/F2/G1/G2/G3/G4/H0/H1/I0 classification of contract vocabulary, proof implementation, crate ownership, descriptor/input/value-source/binding/graph/execution-proof/topology/value-bus/source/asset/recipe document/lifecycle model, schema roots, and handoff guardrails.
- [`../v3.1-surface-contract.md`](../v3.1-surface-contract.md) — current clean-room surface/sampling/pipeline/scene/descriptor contract owned by `tui-vfx-contract` and proven by `tui-vfx-next`.
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
- [`ARCH-RESP-TO-PHASE_E0.md`](ARCH-RESP-TO-PHASE_E0.md) — Phase E0 approval and Phase E1 minimal descriptor recommendation.
- [`ARCH-RESP-TO-PHASE_E1.md`](ARCH-RESP-TO-PHASE_E1.md) — Phase E1 approval and Phase F1 typed value/input recommendation.
- [`ARCH-RESP-TO-PHASE_F1.md`](ARCH-RESP-TO-PHASE_F1.md) — Phase F1 approval and Phase F2 value source / parameter / signal / binding recommendation.
- [`ARCH-RESP-TO-PHASE_F2.md`](ARCH-RESP-TO-PHASE_F2.md) — Phase F2 approval and Phase G1 canonical node graph container recommendation.
- [`ARCH-RESP-TO-PHASE_G1.md`](ARCH-RESP-TO-PHASE_G1.md) — Phase G1 approval and Phase G2 canonical graph execution proof recommendation.
- [`ARCH-RESP-TO-PHASE_G2.md`](ARCH-RESP-TO-PHASE_G2.md) — Phase G2 approval and Phase G3 topology / parallel snapshot / channel-aware merge recommendation.
- [`ARCH-RESP-TO-PHASE_G3.md`](ARCH-RESP-TO-PHASE_G3.md) — Phase G3 approval and Phase G4 node I/O / graph-local value bus recommendation.
- [`ARCH-RESP-TO-PHASE_G4.md`](ARCH-RESP-TO-PHASE_G4.md) — Phase G4 approval and Phase H0 source / asset / procedural source recommendation.
- [`ARCH-RESP-TO-PHASE_H0.md`](ARCH-RESP-TO-PHASE_H0.md) — Phase H0 response and Phase H1 canonical recipe document / vocabulary recommendation.
- [`ARCH-RESP-TO-PHASE_H1.md`](ARCH-RESP-TO-PHASE_H1.md) — Phase H1 response and Phase I0 time / lifecycle / trigger contract recommendation.
- [`ARCH-RESP-TO-PHASE_I0.md`](ARCH-RESP-TO-PHASE_I0.md) — Phase I0 response and Phase J0 primitive recipe migration pilot / validator recommendation.
- [`ARCH-RESP-TO-PHASE_J0.md`](ARCH-RESP-TO-PHASE_J0.md) — Phase J0 approval and Phase J1 validator hardening / fixture harness recommendation.
- [`ARCH-RESP-TO-PHASE_J1.md`](ARCH-RESP-TO-PHASE_J1.md) — Phase J1 approval and Phase J2 shared primitive descriptor catalog / second-ring migration recommendation.
- [`ARCH-RESP-TO-PHASE_J2.md`](ARCH-RESP-TO-PHASE_J2.md) — Phase J2 response and Phase K0 contract-native skeleton player recommendation.
- [`ARCH-RESP-TO-PHASE_K0.md`](ARCH-RESP-TO-PHASE_K0.md) — Phase K0 response and Phase K1 basic visual player UI recommendation.
- [`ARCH-RESP-TO-PHASE_K2_0.md`](ARCH-RESP-TO-PHASE_K2_0.md) — Phase K2.0 acceptance and updated K2 sequence.
- [`ARCH-RESP-TO-PHASE_K2_1.md`](ARCH-RESP-TO-PHASE_K2_1.md) — Phase K2.1 response and K2.2 visual-frame substrate recommendation.
- [`ARCH-RESP-TO-PHASE_K2_2.md`](ARCH-RESP-TO-PHASE_K2_2.md) — Phase K2.2 response and K2.3 primitive adapter burn-down recommendation.
- [`ARCH-RESP-TO-PHASE_K2_3.md`](ARCH-RESP-TO-PHASE_K2_3.md) — Phase K2.3 response and K2.4 styled-cell visual substrate recommendation.
- [`ARCH-RESP-TO-PHASE_K2_4.md`](ARCH-RESP-TO-PHASE_K2_4.md) — Phase K2.4 response and K2.5 styled primitive/tooling PRD packet.
- [`ARCH-RESP-TO-PHASE_K2_9.md`](ARCH-RESP-TO-PHASE_K2_9.md) — Phase K2.9 response and K2.10 corpus-wide migration mapping/backlog packet.
- [`ARCH-RESP-TO-PHASE_K2_10.md`](ARCH-RESP-TO-PHASE_K2_10.md) — Phase K2.10 acceptance and K2.11 schema-readiness ledger/source descriptor pilot packet.
- [`ARCH-RESP-TO-PHASE_K2_11.md`](ARCH-RESP-TO-PHASE_K2_11.md) — Phase K2.11 response and K2.12 schema-lock decision sprint / low-friction burn-down packet.
- [`ARCH-RESP-TO-PHASE_K2_12.md`](ARCH-RESP-TO-PHASE_K2_12.md) — Phase K2.12 response and K2.13 schema decision/offender burn-down packet.
- [`ARCH-RESP-TO-PHASE_K2_13.md`](ARCH-RESP-TO-PHASE_K2_13.md) — Phase K2.13 extension memo with additional schema-settlement and high-confidence migration-unblocker steps folded into the K2.13 work packet.
- [`ARCH-RESP-TO-PHASE_K2_16.md`](ARCH-RESP-TO-PHASE_K2_16.md) — Phase K2.16 response and player render IR, scene/source fidelity, and backlog burn-down packet.
- [`ARCH-RESP-TO-PHASE_K2_17.md`](ARCH-RESP-TO-PHASE_K2_17.md) — K2.17 descriptor/source burn-down, implementation-readiness, and control-catalog packet.

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
- [`PHASE_E1_STATUS.md`](PHASE_E1_STATUS.md) — concise Phase E1 minimal descriptor status.
- [`PHASE_E1_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_E1_STATUS_MEMO_TO_ARCHITECT.md) — Phase E1 architect memo requesting next assignment.
- [`PHASE_F1_STATUS.md`](PHASE_F1_STATUS.md) — concise Phase F1 typed value/input status.
- [`PHASE_F1_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_F1_STATUS_MEMO_TO_ARCHITECT.md) — Phase F1 architect memo requesting next assignment.
- [`PHASE_F2_STATUS.md`](PHASE_F2_STATUS.md) — concise Phase F2 value source / binding status.
- [`PHASE_F2_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_F2_STATUS_MEMO_TO_ARCHITECT.md) — Phase F2 architect memo requesting next assignment.
- [`PHASE_G1_STATUS.md`](PHASE_G1_STATUS.md) — concise Phase G1 canonical graph container status.
- [`PHASE_G1_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_G1_STATUS_MEMO_TO_ARCHITECT.md) — Phase G1 architect memo requesting next assignment.
- [`PHASE_G2_STATUS.md`](PHASE_G2_STATUS.md) — concise Phase G2 graph execution proof status.
- [`PHASE_G2_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_G2_STATUS_MEMO_TO_ARCHITECT.md) — Phase G2 architect memo requesting next assignment.
- [`PHASE_G3_STATUS.md`](PHASE_G3_STATUS.md) — concise Phase G3 topology / parallel merge status.
- [`PHASE_G3_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_G3_STATUS_MEMO_TO_ARCHITECT.md) — Phase G3 architect memo requesting next assignment.
- [`PHASE_G4_STATUS.md`](PHASE_G4_STATUS.md) — concise Phase G4 node I/O / graph value bus status.
- [`PHASE_G4_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_G4_STATUS_MEMO_TO_ARCHITECT.md) — Phase G4 architect memo requesting next assignment.
- [`PHASE_H0_STATUS.md`](PHASE_H0_STATUS.md) — concise Phase H0 source / asset / procedural source status.
- [`PHASE_H0_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_H0_STATUS_MEMO_TO_ARCHITECT.md) — Phase H0 architect memo requesting next assignment.
- [`PHASE_H1_STATUS.md`](PHASE_H1_STATUS.md) — concise Phase H1 canonical recipe document status.
- [`PHASE_H1_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_H1_STATUS_MEMO_TO_ARCHITECT.md) — Phase H1 architect memo requesting next assignment.
- [`PHASE_I0_STATUS.md`](PHASE_I0_STATUS.md) — concise Phase I0 time / lifecycle / trigger contract status.
- [`PHASE_I0_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_I0_STATUS_MEMO_TO_ARCHITECT.md) — Phase I0 architect memo requesting next assignment.
- [`PHASE_J0_STATUS.md`](PHASE_J0_STATUS.md) — concise Phase J0 primitive recipe migration pilot status.
- [`PHASE_J0_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_J0_STATUS_MEMO_TO_ARCHITECT.md) — Phase J0 architect memo requesting next assignment.
- [`PHASE_J1_STATUS.md`](PHASE_J1_STATUS.md) — concise Phase J1 validator hardening status.
- [`PHASE_J1_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_J1_STATUS_MEMO_TO_ARCHITECT.md) — Phase J1 architect memo requesting next assignment.
- [`PHASE_J2_STATUS.md`](PHASE_J2_STATUS.md) — concise Phase J2 descriptor pack / second-ring migration status.
- [`PHASE_J2_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_J2_STATUS_MEMO_TO_ARCHITECT.md) — Phase J2 architect memo requesting next assignment.
- [`PHASE_K0_STATUS.md`](PHASE_K0_STATUS.md) — concise Phase K0 contract-native skeleton player status.
- [`PHASE_K0_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K0_STATUS_MEMO_TO_ARCHITECT.md) — Phase K0 architect memo requesting next assignment.
- [`PHASE_K1_STATUS.md`](PHASE_K1_STATUS.md) — concise Phase K1 visual player UI status.
- [`PHASE_K1_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K1_STATUS_MEMO_TO_ARCHITECT.md) — Phase K1 architect memo requesting next assignment.
- [`PHASE_K2_0_INVENTORY_GATE_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_0_INVENTORY_GATE_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.0 inventory gate memo and primitive coverage baseline.
- [`PHASE_K2_1_MIGRATION_GAP_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_1_MIGRATION_GAP_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.1 migration-gap memo and recipe-family roadmap evidence.
- [`PHASE_K2_1_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_1_REVIEW_AND_DESLOP_REPORT.md) — third-party review and AI de-slop report for K2.1.
- [`PHASE_K2_2_VISUAL_FRAME_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_2_VISUAL_FRAME_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.2 visual-frame substrate memo.
- [`PHASE_K2_2_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_2_REVIEW_AND_DESLOP_REPORT.md) — third-party review and AI de-slop report for K2.2.
- [`PHASE_K2_3_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_3_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.3 primitive adapter burn-down memo.
- [`PHASE_K2_3_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_3_REVIEW_AND_DESLOP_REPORT.md) — third-party review and AI de-slop report for K2.3.
- [`PHASE_K2_4_STYLED_CELL_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_4_STYLED_CELL_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.4 styled-cell substrate memo, verification, and de-slop summary.
- [`PHASE_K2_5_STYLED_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_5_STYLED_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.5 styled primitive adapter burn-down memo and tooling PRD summary.
- [`PHASE_K2_10_CORPUS_MAPPING_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_10_CORPUS_MAPPING_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.10 corpus mapping memo, verification, review, and next-packet recommendation.
- [`PHASE_K2_11_SCHEMA_READINESS_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_11_SCHEMA_READINESS_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.11 schema-readiness memo, source descriptor pilot, blocker ledger, and next-packet sequence.
- [`PHASE_K2_11_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_11_REVIEW_AND_DESLOP_REPORT.md) — briefing-first formal review and AI de-slop evidence for K2.11.
- [`PHASE_K2_12_SCHEMA_LOCK_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_12_SCHEMA_LOCK_STATUS_MEMO_TO_ARCHITECT.md) — Phase K2.12 schema-lock memo, offender ledger evidence, source.text fixture result, and blocker board.
- [`PHASE_K2_12_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_12_REVIEW_AND_DESLOP_REPORT.md) — briefing-first formal review and AI de-slop evidence for K2.12.
- [`PHASE_K2_13_SCHEMA_DECISION_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_13_SCHEMA_DECISION_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md) — K2.13 schema decision readiness declaration, blocker memo, and next-packet recommendation.
- [`PHASE_K2_13_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_13_REVIEW_AND_DESLOP_REPORT.md) — formal review, anti-slop closure, and verification evidence for K2.13.
- [`PHASE_K2_14_DESCRIPTOR_ADAPTER_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_14_DESCRIPTOR_ADAPTER_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md) — descriptor/adapter migration tranche status memo, before/after metrics, risks, and next-packet recommendation.
- [`PHASE_K2_14_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_14_REVIEW_AND_DESLOP_REPORT.md) — formal review, anti-slop closure, and verification evidence for K2.14.
- [`PHASE_K2_15_GRAPH_DESCRIPTOR_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_15_GRAPH_DESCRIPTOR_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md) — K2.15 graph execution integration, fixture additions, final counters, and architect blocker memo.
- [`PHASE_K2_15_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_15_REVIEW_AND_DESLOP_REPORT.md) — formal review, anti-slop cleanup, and verification evidence for K2.15.
- [`PHASE_K2_16_PLAYER_IR_BACKLOG_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_16_PLAYER_IR_BACKLOG_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md) — K2.16 player render IR, scene/source fidelity, 21-fixture burn-down, final counters, and architect blocker memo.
- [`PHASE_K2_16_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_16_REVIEW_AND_DESLOP_REPORT.md) — formal review, anti-slop cleanup, and verification evidence for K2.16.
- [`PHASE_K2_17_DESCRIPTOR_SOURCE_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md`](PHASE_K2_17_DESCRIPTOR_SOURCE_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md) — K2.17 descriptor/source burn-down, fixture growth, final counters, and architect blocker memo.
- [`PHASE_K2_17_REVIEW_AND_DESLOP_REPORT.md`](PHASE_K2_17_REVIEW_AND_DESLOP_REPORT.md) — K2.17 formal review, anti-slop cleanup, and verification evidence.

## Supporting artifacts

- [`CONTRACT_ADD_DECISION_TREE.md`](CONTRACT_ADD_DECISION_TREE.md) — preserved source decision-tree draft for classifying contract-visible feature additions.
- [`PROCESS_OVERVIEW_DIAGRAM.md`](PROCESS_OVERVIEW_DIAGRAM.md) — process overview diagram/notes.
- [`H1_RECIPE_EVIDENCE_NOTES.md`](H1_RECIPE_EVIDENCE_NOTES.md) — curated recipe-evidence mapping pressure notes for canonical H1 schema work.
- [`I0_EVENT_DWELL_EVIDENCE_NOTES.md`](I0_EVENT_DWELL_EVIDENCE_NOTES.md) — curated event-dwell/timing evidence notes for canonical I0 lifecycle/trigger work.
- [`J0_PRIMITIVE_MIGRATION_EVIDENCE.md`](J0_PRIMITIVE_MIGRATION_EVIDENCE.md) — primitive recipe migration pilot evidence, descriptor seed catalog, and validation proof.
- [`J1_VALIDATOR_HARNESS_STATUS.md`](J1_VALIDATOR_HARNESS_STATUS.md) — validator hardening evidence, recursive report shape, and negative diagnostics coverage.
- [`J2_DESCRIPTOR_PACK_STATUS.md`](J2_DESCRIPTOR_PACK_STATUS.md) — descriptor pack/catalog evidence, primitive pack contents, validator integration, and second-ring fixture status.
- [`K0_PLAYER_STATUS.md`](K0_PLAYER_STATUS.md) — contract-native skeleton player evidence, rendered/unsupported fixture split, adapters, and schema pressure.
- [`K1_PLAYER_UI_STATUS.md`](K1_PLAYER_UI_STATUS.md) — visual player shell evidence, controls, K0 API reuse, and fixture coverage.
- [`DAY2_PLAN_K2.0_PACKET.md`](DAY2_PLAN_K2.0_PACKET.md) — Day-2 K2.0 inventory gate packet.
- [`day2_briefing_report.md`](day2_briefing_report.md) — Day-2 reconnaissance report for K0/K1 architecture and route to visual evidence.
- [`K2_2_VISUAL_FRAME_EVIDENCE.md`](K2_2_VISUAL_FRAME_EVIDENCE.md) — K2.2 render-frame/visual-frame evidence reference.
- [`K2_3_PRIMITIVE_ADAPTER_GAP_EVIDENCE.md`](K2_3_PRIMITIVE_ADAPTER_GAP_EVIDENCE.md) — K2.3 primitive adapter support and blocker evidence reference.
- [`K2_4_STYLED_CELL_SUBSTRATE_EVIDENCE.md`](K2_4_STYLED_CELL_SUBSTRATE_EVIDENCE.md) — K2.4 styled-cell substrate, `styleKnown`, and controlled styled-grid proof reference.
- [`K2_5_STYLED_PRIMITIVE_ADAPTER_EVIDENCE.md`](K2_5_STYLED_PRIMITIVE_ADAPTER_EVIDENCE.md) — K2.5 styled primitive adapter evidence, visual-frame provenance, and report summaries.
- [`K2_PLAYER_TOOLING_VALIDATION_PRD.md`](K2_PLAYER_TOOLING_VALIDATION_PRD.md) — K2 tooling/validation PRD that classifies legacy tooling capabilities as oracle inspiration only.
- [`K2_10_DEBUG_RECIPE_CORPUS_MAPPING_REPORT.md`](K2_10_DEBUG_RECIPE_CORPUS_MAPPING_REPORT.md) — K2.10 corpus-wide migration mapping report with per-family status counts.
- [`K2_10_MIGRATION_BACKLOG_BOARD.md`](K2_10_MIGRATION_BACKLOG_BOARD.md) — K2.10 backlog board grouped by descriptor, source, schema, GUI, backend, owner-audit, and duplicate-variant buckets.
- [`K2_10_RENDER_BACKEND_BOUNDARY_NOTE.md`](K2_10_RENDER_BACKEND_BOUNDARY_NOTE.md) — K2.10 note preserving the future render-backend adapter boundary.
- [`K2_11_SCHEMA_READINESS_LEDGER.md`](K2_11_SCHEMA_READINESS_LEDGER.md) — K2.11 schema-readiness blocker ledger and resolution sequence.
- [`K2_11_SOURCE_CONTENT_SCENE_DECISION_TABLE.md`](K2_11_SOURCE_CONTENT_SCENE_DECISION_TABLE.md) — K2.11 source/content/scene decision table for v3.1 migration blockers.
- [`K2_11_STUDIO_CONTROL_SURFACE_PREFLIGHT.md`](K2_11_STUDIO_CONTROL_SURFACE_PREFLIGHT.md) — K2.11 control-surface preflight deferral and K3.0 trigger.
- [`K2_12_SCHEMA_LOCK_DECISION_REPORT.md`](K2_12_SCHEMA_LOCK_DECISION_REPORT.md) — K2.12 schema-readiness declaration, blocker decision board, and next-packet recommendation.
- [`K2_12_SOURCE_CONTENT_DECISION_TABLE.md`](K2_12_SOURCE_CONTENT_DECISION_TABLE.md) — K2.12 source/content blocker classification and source.text fixture disposition.
- [`K2_12_RUNTIME_DYNAMISM_DECISION_MATRIX.md`](K2_12_RUNTIME_DYNAMISM_DECISION_MATRIX.md) — K2.12 binding, signal, value-source, lifecycle, easing, and motion-route decision matrix.
- [`K2_12_COMPLEX_STYLE_NORMALIZATION_REPORT.md`](K2_12_COMPLEX_STYLE_NORMALIZATION_REPORT.md) — K2.12 complex owner-audit and unknown-style normalization results.
- [`K2_12_DESCRIPTOR_EXPANSION_QUEUE.md`](K2_12_DESCRIPTOR_EXPANSION_QUEUE.md) — K2.12 descriptor expansion, field-coverage, and holdback queue.
- [`K2_12_SCHEMA_API_DOC_INFRA_REPORT.md`](K2_12_SCHEMA_API_DOC_INFRA_REPORT.md) — K2.12 schema/API/docs verification infrastructure report.
- [`K2_12_LOW_FRICTION_BURN_DOWN_REPORT.md`](K2_12_LOW_FRICTION_BURN_DOWN_REPORT.md) — K2.12 implemented low-friction work and explicit deferrals.
- [`K2_13_SCHEMA_DECISION_CONTROL_SURFACE_REPORT.md`](K2_13_SCHEMA_DECISION_CONTROL_SURFACE_REPORT.md) — K2.13 additive schema-readiness disposition/control-surface report.
- [`K2_13_SOURCE_CONTENT_DECISION_REPORT.md`](K2_13_SOURCE_CONTENT_DECISION_REPORT.md) — K2.13 source/content split, source descriptor placeholders, and command-capture holdback.
- [`K2_13_RUNTIME_DYNAMISM_DECISION_REPORT.md`](K2_13_RUNTIME_DYNAMISM_DECISION_REPORT.md) — K2.13 parameter/signal/graph-value/binding and sampled-field value-source decision.
- [`K2_13_SCENE_ELEMENT_LAYER_DECISION_REPORT.md`](K2_13_SCENE_ELEMENT_LAYER_DECISION_REPORT.md) — K2.13 scene/element/layer schema disposition and composition rules.
- [`K2_17_BASELINE_AND_FINAL_COUNTERS.md`](K2_17_BASELINE_AND_FINAL_COUNTERS.md) — K2.17 baseline/final corpus and backlog counters.
- [`K2_17_IMPLEMENTATION_READINESS_REPORT.md`](K2_17_IMPLEMENTATION_READINESS_REPORT.md) — K2.17 disposition-first implementation-readiness report.
- [`K2_17_BACKLOG_DISPOSITION_REPAIR_REPORT.md`](K2_17_BACKLOG_DISPOSITION_REPAIR_REPORT.md) — K2.17 backlog vocabulary and disposition repair notes.
- [`K2_17_CONTENT_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md`](K2_17_CONTENT_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md) — K2.17 content descriptor/adapter/fixture tranche.
- [`K2_17_SOURCE_FIDELITY_TRANCHE_REPORT.md`](K2_17_SOURCE_FIDELITY_TRANCHE_REPORT.md) — K2.17 ANSI/image/procedural source fidelity tranche.
- [`K2_17_FILTER_MASK_SAMPLER_TRANCHE_REPORT.md`](K2_17_FILTER_MASK_SAMPLER_TRANCHE_REPORT.md) — K2.17 filter, mask, and sampler tranche.
- [`K2_17_SHADER_STYLE_TRANCHE_REPORT.md`](K2_17_SHADER_STYLE_TRANCHE_REPORT.md) — K2.17 shader and style tranche.
- [`K2_17_SCENE_LAYER_RUNTIME_CLOSURE_REPORT.md`](K2_17_SCENE_LAYER_RUNTIME_CLOSURE_REPORT.md) — K2.17 scene/layer visibility runtime closure.
- [`K2_17_RENDER_BACKEND_SEAM_REPORT.md`](K2_17_RENDER_BACKEND_SEAM_REPORT.md) — K2.17 player render-backend seam report.
- [`K2_17_STUDIO_CONTROL_CATALOG_REPORT.md`](K2_17_STUDIO_CONTROL_CATALOG_REPORT.md) — K2.17 descriptor-derived control catalog report.
- [`K2_17_HOLDBACK_REGISTER.md`](K2_17_HOLDBACK_REGISTER.md) — K2.17 explicit holdbacks and missed preferred thresholds.
- [`K2_17_SCHEMA_API_DOCS_GATE.md`](K2_17_SCHEMA_API_DOCS_GATE.md) — K2.17 schema/API/docs gate report.
- [`K2_13_MOTION_SCOPE_DECISION_REPORT.md`](K2_13_MOTION_SCOPE_DECISION_REPORT.md) — K2.13 motion/easing and built-in scope vocabulary decisions.
- [`K2_13_FIELD_COVERAGE_CLOSURE_REPORT.md`](K2_13_FIELD_COVERAGE_CLOSURE_REPORT.md) — K2.13 gradient/applyTo/position field coverage closure report.
- [`K2_13_COMPLEX_GRAPH_IO_DECISION_REPORT.md`](K2_13_COMPLEX_GRAPH_IO_DECISION_REPORT.md) — K2.13 complex graph I/O and sequence/parallel disposition report.
- [`K2_13_SCHEMA_API_DOC_INFRA_REPORT.md`](K2_13_SCHEMA_API_DOC_INFRA_REPORT.md) — K2.13 schema/API/docs synchronization report.
- [`K2_13_TEMPLATE_COMPOSITION_DECISION_NOTE.md`](K2_13_TEMPLATE_COMPOSITION_DECISION_NOTE.md) — K2.13 compile-time template composition boundary.
- [`K2_13_STUDIO_CONTROL_SURFACE_PREFLIGHT.md`](K2_13_STUDIO_CONTROL_SURFACE_PREFLIGHT.md) — K2.13 studio control derivation preflight.
- [`K2_14_DESCRIPTOR_ADAPTER_MIGRATION_REPORT.md`](K2_14_DESCRIPTOR_ADAPTER_MIGRATION_REPORT.md) — descriptor and adapter tranche summary with before/after counters.
- [`K2_14_CANONICAL_FIXTURE_ADDITIONS.md`](K2_14_CANONICAL_FIXTURE_ADDITIONS.md) — canonical v3.1 debug fixture additions from the descriptor/adapter tranche.
- [`K2_14_GRAPH_IO_EVIDENCE_REPORT.md`](K2_14_GRAPH_IO_EVIDENCE_REPORT.md) — graph I/O evidence status and player/proof integration gap.
- [`K2_14_SCENE_SOURCE_EVIDENCE_REPORT.md`](K2_14_SCENE_SOURCE_EVIDENCE_REPORT.md) — source adapter and scene evidence status.
- [`K2_14_HOLDBACK_REGISTER.md`](K2_14_HOLDBACK_REGISTER.md) — backend/gui/oracle/duplicate holdback register.
- [`K2_14_SCHEMA_API_DOCS_GATE.md`](K2_14_SCHEMA_API_DOCS_GATE.md) — reproducible schema/API/docs gate commands.
- [`K2_14_STUDIO_CONTROL_DERIVATION_REPORT.md`](K2_14_STUDIO_CONTROL_DERIVATION_REPORT.md) — future studio-control derivation rules.
- [`K2_15_BASELINE_AND_FINAL_COUNTERS.md`](K2_15_BASELINE_AND_FINAL_COUNTERS.md) — K2.15 before/after migration, render, field coverage, adapter, schema-readiness, and mapping counters.
- [`K2_15_GRAPH_EXECUTION_INTEGRATION_REPORT.md`](K2_15_GRAPH_EXECUTION_INTEGRATION_REPORT.md) — player graph topology/value-bus executor integration evidence.
- [`K2_15_GRAPH_IO_FIXTURE_REPORT.md`](K2_15_GRAPH_IO_FIXTURE_REPORT.md) — canonical graph I/O fixture additions and proof goals.
- [`K2_15_SCENE_LAYER_PLAYER_EVIDENCE_REPORT.md`](K2_15_SCENE_LAYER_PLAYER_EVIDENCE_REPORT.md) — scene/layer-local player evidence and honest limits.
- [`K2_15_DESCRIPTOR_ADAPTER_TRANCHE_2_REPORT.md`](K2_15_DESCRIPTOR_ADAPTER_TRANCHE_2_REPORT.md) — descriptor/adapter tranche status and zero-gap evidence.
- [`K2_15_SOURCE_CONTENT_TRANCHE_2_REPORT.md`](K2_15_SOURCE_CONTENT_TRANCHE_2_REPORT.md) — source/content tranche status for ANSI, image, and procedural fixtures.
- [`K2_15_HOLDBACK_REGISTER.md`](K2_15_HOLDBACK_REGISTER.md) — remaining holdbacks and high-level blockers.
- [`K2_15_SCHEMA_API_DOCS_GATE.md`](K2_15_SCHEMA_API_DOCS_GATE.md) — docs/schema impact assessment and synchronization note.
- [`K2_15_STUDIO_CONTROL_CATALOG_PREFLIGHT.md`](K2_15_STUDIO_CONTROL_CATALOG_PREFLIGHT.md) — studio-control/catalog preflight status after graph execution work.
- [`K2_16_BASELINE_AND_FINAL_COUNTERS.md`](K2_16_BASELINE_AND_FINAL_COUNTERS.md) — K2.16 before/after validation, render, field coverage, adapter, schema-readiness, and mapping counters.
- [`K2_16_PLAYER_RENDER_IR_REPORT.md`](K2_16_PLAYER_RENDER_IR_REPORT.md) — player-owned render IR report shape, CLI evidence path, and API boundary.
- [`K2_16_GRAPH_EXECUTOR_HARDENING_REPORT.md`](K2_16_GRAPH_EXECUTOR_HARDENING_REPORT.md) — graph executor diagnostics and guarded runtime-hardening evidence.
- [`K2_16_SCENE_LAYER_RUNTIME_FIDELITY_REPORT.md`](K2_16_SCENE_LAYER_RUNTIME_FIDELITY_REPORT.md) — scene layer ordering and source write-policy fidelity report.
- [`K2_16_SOURCE_FIDELITY_TRANCHE_REPORT.md`](K2_16_SOURCE_FIDELITY_TRANCHE_REPORT.md) — source fidelity tranche status for ANSI, image, procedural, and transparency semantics.
- [`K2_16_CONTENT_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md`](K2_16_CONTENT_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md) — content descriptor/adapter tranche fixtures and evidence.
- [`K2_16_PRIMITIVE_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md`](K2_16_PRIMITIVE_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md) — primitive adapter tranche fixtures and adapter-gap evidence.
- [`K2_16_SHADER_STYLE_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md`](K2_16_SHADER_STYLE_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md) — shader/style descriptor adapter tranche evidence.
- [`K2_16_BACKLOG_NORMALIZATION_REPORT.md`](K2_16_BACKLOG_NORMALIZATION_REPORT.md) — normalized remaining migration mapping backlog by disposition and path.
- [`K2_16_BACKEND_ADAPTER_SEAM_PREFLIGHT.md`](K2_16_BACKEND_ADAPTER_SEAM_PREFLIGHT.md) — player render IR to compositor/backend seam preflight.
- [`K2_16_HOLDBACK_REGISTER.md`](K2_16_HOLDBACK_REGISTER.md) — remaining holdbacks and high-level blockers after K2.16.
- [`K2_16_STUDIO_CONTROL_CATALOG_REPORT.md`](K2_16_STUDIO_CONTROL_CATALOG_REPORT.md) — studio control catalog readiness and deferred GUI-control evidence.
- [`K2_16_SCHEMA_API_DOCS_GATE.md`](K2_16_SCHEMA_API_DOCS_GATE.md) — schema, API, docs, rustdoc, and OFPF synchronization gate for K2.16.
- [`TEMPLATE_INHERITANCE.md`](TEMPLATE_INHERITANCE.md) — scene/template composition guidance from architect discussion.

## Validator and recipe artifacts

- [`../../crates/tui-vfx-contract-cli/`](../../crates/tui-vfx-contract-cli/) — contract-only canonical v3.1 recipe validator CLI with recursive `validate-recipe` report schema `v3.1.validator.report.1`.
- [`../../crates/tui-vfx-player/`](../../crates/tui-vfx-player/) — contract-native skeleton player API with `RecipePlayer`, `PlayerSession`, sampled frame reports, deterministic render hash, and explicit unsupported adapter diagnostics.
- [`../../crates/tui-vfx-player-cli/`](../../crates/tui-vfx-player-cli/) — player CLI emitting `render-recipe` reports (`v3.1.player.frame.1`, `v3.1.player.run.1`), `render-frame` visual-frame reports (`v3.1.player.visualFrameReport.1`), inventory reports, migration-gap reports, primitive-adapter-gap reports, and schema-readiness reports.
- [`../../crates/tui-vfx-player-ui/`](../../crates/tui-vfx-player-ui/) — K1 ratatui/crossterm visual terminal shell with local fast-fs navigation layered on the K0 player/session APIs.
- [`../../descriptors/v3.1/packs/primitive.json`](../../descriptors/v3.1/packs/primitive.json) — standard v3.1 primitive descriptor pack created by Phase J2.
- `../../../tui-vfx-recipes/recipes/v3.1/debug_recipes/` — canonical v3.1 migrated debug recipe fixtures produced by Phase J0 and extended by Phase J2.

## Schema artifacts

- [`../../schemas/v3.1/contract/`](../../schemas/v3.1/contract/) — checked generated stable contract schemas owned by `tui-vfx-contract`; includes surface, scope, write, diagnostic, scene/element/outcome, descriptor pack/catalog, effect descriptor/input/output, value/value-source/parameter/signal/binding, graph/graph-step/node/node-output/graph-value shapes, source/source-descriptor/source-input/source-output/source-instance-id, asset/asset-requirement/asset-ref, recipe metadata/scene/scene-element/element-pipeline/recipe, duration/clock/dwell-policy/trigger/value-predicate/phase/lifecycle.
- [`../../schemas/v3.1/next/`](../../schemas/v3.1/next/) — checked generated proof-pipeline schemas owned by `tui-vfx-next`; currently `sampler.schema.json` and `pipeline.schema.json`.
- [`../../crates/tui-vfx-contract/`](../../crates/tui-vfx-contract/) — Rust source of truth for the stable v3.1 contract schemas above.
- [`../../crates/tui-vfx-next/`](../../crates/tui-vfx-next/) — proof crate for schema/reference concepts that are not yet promoted to the stable contract crate.

## Ralph planning artifacts

- `.omx/plans/prd-new-kernel-phase-d0.md` — Phase D0 PRD.
- `.omx/plans/test-spec-new-kernel-phase-d0.md` — Phase D0 verification spec.

<!-- <FILE>docs/new_kernel/INDEX.md</FILE> - <DESC>Index for clean-room new-kernel planning and status documents</DESC> -->
<!-- <VERS>END OF VERSION: 0.31.0</VERS> -->
