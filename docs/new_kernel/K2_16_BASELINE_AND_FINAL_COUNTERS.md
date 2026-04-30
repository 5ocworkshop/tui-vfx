# K2.16 baseline and current counters

## Baseline before K2.16 edits

```text
canonical v3.1 fixtures: 67
validate-recipe: 67 valid / 0 invalid
fixture-qc: pass, 67 rendered, 0 playerErrors
primitive-field-coverage: 422 used / 422 handled / 0 unhandled
primitive-adapter-gap: 43 rendered / 0 unresolved
schema-readiness: canDeclareSchemaReady=true, explicitOwnerDecisionNeeded=0
migration-mapping: canonicalExists=50, candidateReady=5, descriptorDecisionNeeded=113, sourceDecisionNeeded=61
```

## Current K2.16 counters

```text
canonical v3.1 fixtures: 88
validate-recipe: 88 valid / 0 invalid
render-recipe: 88 rendered / 0 unsupported / 0 errors
render-frame: 88 rendered / 0 unsupported / 0 errors
fixture-qc: pass, 88 rendered, 0 playerErrors
primitive-field-coverage: 541 used / 541 handled / 0 unhandled
primitive-adapter-gap: 45 rendered / 0 unresolved
schema-readiness: canDeclareSchemaReady=true, explicitOwnerDecisionNeeded=0
migration-mapping: canonicalExists=55, candidateReady=0, descriptorDecisionNeeded=113, sourceDecisionNeeded=61
```

## Interpretation

K2.16 has added 21 canonical fixtures so far and converted the five pre-existing `candidateReady` paths into canonical-existing records. The raw descriptor/source backlog counters are unchanged because this tranche deliberately avoided false-green descriptor claims for unsupported legacy-only families.
