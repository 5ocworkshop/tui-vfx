<!-- <FILE>docs/new_kernel/K2_13_SCHEMA_DECISION_CONTROL_SURFACE_REPORT.md</FILE> - <DESC>K2.13 schema decision control surface report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: record the machine-readable readiness disposition surface.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document additive schema-readiness disposition fields and declaration gate.</CLOG> -->

# K2.13 Schema Decision Control Surface Report

## Decision

The `schema-readiness` report remains `v3.1.player.schemaReadiness.1` because v3.1 is pre-release and the new fields are additive. The report now exposes a disposition-based declaration gate instead of treating every raw migration status as a schema blocker.

## Additive report fields

`summary` now includes:

- `unresolvedSchemaBlockers`
- `signedOffHoldbacks`
- `explicitOwnerDecisionNeeded`
- `dispositionCounts`
- `remainingOwnerDecisionCount`
- `remainingOwnerDecisions[]`

`offenders[]` now includes:

- `disposition`
- `schemaBlocking`
- `holdbackSignedOff`
- `exactDecisionRequired`
- `recommendedNextAction`

## Accepted disposition vocabulary

```text
acceptedSchema
descriptorBacklog
adapterBacklog
backendHoldback
guiHumanReviewHoldback
oracleOnly
duplicateVariant
explicitOwnerDecisionNeeded
```

## Current evidence

Fresh K2.13 evidence from `schema-readiness --recursive --include-offenders --json` over `/usr/projects/tui-vfx-recipes/recipes/debug_recipes`:

```text
totalLegacyRecords: 603
offenders: 383
fieldCoverageBlockedRecords: 0
unresolvedSchemaBlockers: 0
explicitOwnerDecisionNeeded: 0
remainingOwnerDecisionCount: 0
canDeclareSchemaReady: true
```

Disposition counts:

| Disposition | Count |
|---|---:|
| `acceptedSchema` | 125 |
| `descriptorBacklog` | 263 |
| `backendHoldback` | 15 |
| `guiHumanReviewHoldback` | 2 |
| `oracleOnly` | 195 |
| `duplicateVariant` | 3 |

## Important interpretation

Raw fields such as `schemaBlockedRecords`, `sourceBlockedRecords`, `descriptorBlockedRecords`, `ownerAuditRecords`, and `unknownRecords` remain historical migration-status counters. They are retained for continuity and backlog sizing. The declaration gate is now disposition-based:

```text
canDeclareSchemaReady = totalLegacyRecords > 0
  && unresolvedSchemaBlockers == 0
  && remainingOwnerDecisionCount == 0
```

<!-- <FILE>docs/new_kernel/K2_13_SCHEMA_DECISION_CONTROL_SURFACE_REPORT.md</FILE> - <DESC>K2.13 schema decision control surface report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
