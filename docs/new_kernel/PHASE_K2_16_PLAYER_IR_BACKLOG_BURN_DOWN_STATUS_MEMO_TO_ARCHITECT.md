# Phase K2.16 player IR/backlog burn-down status memo to architect

## Executive summary

K2.16 now has the first player-owned render IR surface and `render-ir` CLI, plus a 21-fixture canonical tranche that moves the corpus from 67 to 88 fixtures while preserving zero-gap fixture gates.

## Before/after counters

| Counter | Before | Current |
|---|---:|---:|
| canonical v3.1 fixtures | 67 | 88 |
| validate-recipe valid | 67 | 88 |
| render-recipe rendered | 67 | 88 |
| render-frame rendered | 67 | 88 |
| fixture-qc player errors | 0 | 0 |
| primitive fields used/handled | 422/422 | 541/541 |
| primitive adapter effects rendered | 43 | 45 |
| canonicalExists | 50 | 55 |
| candidateReady | 5 | 0 |
| descriptorDecisionNeeded | 113 | 113 |
| sourceDecisionNeeded | 61 | 61 |
| explicitOwnerDecisionNeeded | 0 | 0 |

## Lane results

| Lane | Result |
|---|---|
| A backlog normalization | Added disposition-first docs and top-priority path report from current JSON evidence. |
| B player render IR | Implemented `PlayerRenderIrReport` and `render-ir` CLI. |
| C graph hardening | Added missing graph-value diagnostic and IR graph-value snapshots. |
| D scene/layer fidelity | Added stable z/declaration ordering, skip-transparent-empty row policy, and IR provenance. |
| E source fidelity | Preserved bounded sources and surfaced provenance; richer ANSI/image remain holdbacks. |
| F content tranche | Added six content hardening fixtures. |
| G primitive descriptor/adaptor tranche | Added filter/sampler/mask fixtures using existing adapters. |
| H shader/style tranche | Added shader/style fixture evidence. |
| I backend seam | Documented player IR -> future backend adapter seam. |
| J studio/docs gates | Added studio-control catalog preflight report; full CLI/spec remains deferred until the backend/studio lane. |

## Exact fixture additions

See `K2_16_PRIMITIVE_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md`, `K2_16_CONTENT_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md`, and `K2_16_SHADER_STYLE_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md`.

## Render IR status

`v3.1.player.renderIr.1` carries rows, sparse styled cells, scene/source provenance, graph value snapshots, diagnostics, and sample-clock fields. It is additive and does not replace `render-frame`.

## Unresolved risks, acceptance deviations, and next packet

Explicit K2.16 acceptance deviations:

- Scene visibility predicate fixtures remain deferred because the current player scene path has no visibility evaluator yet; this is runtime work, not a schema-readiness blocker.
- Studio control catalog remains a preflight report, not a CLI/spec implementation.

Higher-level blockers to address for significant forward progress:

- The 113 descriptor backlog records need an owner-approved descriptor/adapter migration lane rather than repeated rediscovery.
- The 61 source/content records need a source-fidelity lane that decides which sources receive bounded player adapters versus backend-only holdbacks.
- Backend/compositor integration needs an adapter seam implementation so render IR can become live backend input instead of evidence-only JSON.
- GUI/studio controls need a catalog/spec lane after descriptor/source decisions stabilize enough to avoid false control surfaces.

The next packet should choose one of:

- descriptor/adapter migration tranche for the 113 descriptor backlog records;
- source/content fidelity tranche for the 61 source/content records;
- backend adapter prototype for shadows/subcell holdbacks.

Do not reopen schema readiness; use the backlog disposition report to pick implementation lanes.
