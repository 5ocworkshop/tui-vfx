# Descriptor / Adapter Migration Status Memo to Architect

## Executive summary

K2.14 materially advances the v3.1 debug migration from schema-decision readiness into implementation evidence. The packet adds 30 canonical fixtures, expands the primitive descriptor pack from 18 to 45 effects, and keeps the canonical corpus green.

## Before / after counters

```json
{
  "canonicalFixtures": { "before": 27, "after": 57 },
  "renderRecipe": {"total": 57, "rendered": 57, "unsupported": 0, "errors": 0},
  "renderFrame": {"total": 57, "rendered": 57, "unsupported": 0, "errors": 0},
  "fixtureQc": {"totalRecipes": 57, "validated": 57, "validationErrors": 0, "rendered": 57, "unsupported": 0, "playerErrors": 0, "visualFrames": 57, "fieldCoverageUnhandled": 0, "adapterGapUnresolved": 0, "timelineSmokePassed": true, "diffSmokePassed": true, "overallStatus": "pass"},
  "primitiveFieldCoverage": {"totalRecipes": 57, "totalPrimitiveInstances": 134, "usedInputFields": 361, "handledInputFields": 361, "usedButUnhandledInputFields": 0, "declaredButUnusedInputFields": 44, "missingDescriptorInputFields": 0, "schemaDecisionNeededFields": 0},
  "primitiveAdapterGap": {"totalEffects": 43, "rendered": 43, "stillUnsupported": 0, "blockedByStyledCellSubstrate": 0, "blockedBySemanticDecision": 0, "missingDescriptor": 0},
  "schemaReadiness": {"totalLegacyRecords": 603, "schemaReadyRecords": 249, "schemaBlockedRecords": 93, "sourceBlockedRecords": 61, "descriptorBlockedRecords": 113, "adapterBlockedRecords": 0, "fieldCoverageBlockedRecords": 0, "ownerAuditRecords": 280, "oracleOnlyRecords": 193, "duplicateOrVariantRecords": 3, "unknownRecords": 0, "estimatedSchemaReadinessPercent": 41.3, "canDeclareSchemaReady": true, "unresolvedSchemaBlockers": 0, "signedOffHoldbacks": 238, "explicitOwnerDecisionNeeded": 0, "dispositionCounts": {"acceptedSchema": 169, "backendHoldback": 15, "descriptorBacklog": 219, "duplicateVariant": 3, "guiHumanReviewHoldback": 2, "oracleOnly": 195}, "remainingOwnerDecisionCount": 0, "remainingOwnerDecisions": []},
  "migrationMapping": {"families": 18, "records": 603, "canonicalExists": 48, "candidateReady": 5, "descriptorDecisionNeeded": 113, "schemaDecisionNeeded": 93, "ownerAuditNeeded": 280, "adapterDecisionNeeded": 0, "sourceDecisionNeeded": 61, "blockedByUnsupportedSource": 0, "blockedByUnsupportedEffect": 0, "blockedByFieldCoverage": 0, "blockedByAmbiguousLegacyIntent": 0, "duplicateOrVariant": 3, "notYetClassified": 0}
}
```

## Lane summary

| Lane | Result |
| --- | --- |
| A Control/metrics | Baseline and after counters captured; gates remain green. |
| B Runtime/value evidence | Bound signal fallback fixtures for color, position, focus, and index inputs added; loopback remains offline/oracle-only. |
| C Content | Six content descriptors/adapters/fixtures added. |
| D Sources | ANSI, image fallback, and procedural dots-spinner source adapters/fixtures added. |
| E Graph I/O | Added explicit proof-test aggregate for sequence graph-value consumption, parallel post-join visibility, and deterministic conflict errors; player topology/value-bus execution remains future work. |
| F Scene/layer | Source/scene fixture evidence improved; rich layer-local pipeline/visibility remains future work. |
| G Filters/masks/samplers | Eleven descriptor/adapter targets added or hardened. |
| H Shaders/styles | Shader composition and built-in style scope fixtures added. |
| I Holdbacks | Register created; backend/gui/oracle/duplicate dispositions remain non-schema holdbacks. |
| J Schema/API/docs/studio | Gate and studio derivation reports created. |

## Unresolved risks and recommended next packet

The highest-level blocker is no longer schema indecision; it is deeper player/proof integration. The next significant progress requires: (1) player graph topology/value-bus execution rather than order-only graph walking, (2) richer scene layer-local pipeline and visibility evidence, and (3) second-tranche descriptor/adapters for remaining descriptorPack/sourceDescriptor backlog.

Recommended next packet: **Descriptor / Adapter Migration Tranche 2**, with a dedicated graph/player integration lane.
